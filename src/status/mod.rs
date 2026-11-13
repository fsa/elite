//! Нативный экран статуса пилота — клавиша F9.
//!
//! Оформление — как у карты галактики (`F10`): белая рамка вокруг всего
//! экрана, красная ячейка с белым заголовком сверху и нижняя панель с
//! подсказкой. Данные читаются из живого блока командира (`PilotName`);
//! экран ничего не меняет и служит только для просмотра.
//!
//! Показываются поля оригинала (`Cmd_L_PilotInfo` в версии Кладова):
//! имя командира, текущая система, номер галактики, деньги, топливо,
//! правовой статус (`PilotLegalStatus`: `0` — чист, `< 0x32` — нарушитель,
//! дальше — беглец) и боевой рейтинг (`PilotRating0..2`, пороги
//! `8/16/32/64/128/512/2560/6400` — как в таблицах `byte_F0AE`/`byte_F0B4`).

use crate::display::Frame;
use crate::galaxy::{self, GalaxyLayout};

mod draw;
#[cfg(test)]
mod tests;

/// Длина поля имени в блоке командира (`PilotName` — 10 символов и `NUL`).
const NAME_LEN: usize = 11;

/// Смещения полей внутри блока командира (`PilotName` = `0xD054`).
const OFF_LEGAL: usize = 11;
const OFF_RATING: usize = 12;
const OFF_CASH2: usize = 18;
const OFF_CASH1: usize = 20;
const OFF_FUEL: usize = 46;

/// Раскладка экрана статуса (адреса версии Кладова).
#[derive(Clone, Copy)]
pub struct StatusLayout {
    /// Адрес живого блока командира (`PilotName`).
    pub pilot: u16,
    /// Раскладка галактики — имя текущей системы и её номер.
    pub galaxy: GalaxyLayout,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_STATUS: StatusLayout = StatusLayout {
    pilot: 0xD054,
    galaxy: galaxy::KLADOV_GALAXY,
};

/// Снимок данных командира для отрисовки.
#[derive(Clone, Debug, Default, PartialEq)]
struct Info {
    name: String,
    system: String,
    /// Номер галактики `1..8`.
    galaxy: u8,
    /// Деньги десятыми кредита (`cash2*10000 + cash1`).
    cash: u32,
    /// Топливо десятыми светового года (`70` = `7.0 LY`).
    fuel: u8,
    /// `PilotLegalStatus`.
    legal: u8,
    /// Боевой рейтинг (`PilotRating0..2`, 24 бита).
    rating: u32,
}

/// Состояние и отрисовка экрана статуса.
pub struct Status {
    /// Показывается ли экран сейчас.
    pub active: bool,
    layout: StatusLayout,
    info: Info,
}

impl Status {
    pub fn new() -> Self {
        Status {
            active: false,
            layout: KLADOV_STATUS,
            info: Info::default(),
        }
    }

    /// Открыть экран: снять состояние командира из памяти запущенной игры.
    pub fn open(&mut self, mem: &[u8], layout: StatusLayout) {
        self.layout = layout;
        let p = layout.pilot as usize;
        let gl = layout.galaxy;
        let seed: [u8; 6] = mem[gl.seed..gl.seed + 6].try_into().unwrap();
        let planets = galaxy::planets(seed);
        let (sx, sy) = (mem[gl.sys_x] as i32, mem[gl.sys_y] as i32);
        let system = galaxy::nearest(&planets, gl.metric, sx, sy)
            .map(|planet| galaxy::name(planet.state, &mem[gl.digraphs..gl.digraphs + 64]))
            .unwrap_or_default();
        self.info = Info {
            name: read_name(&mem[p..p + NAME_LEN]),
            system,
            galaxy: (mem[gl.galaxy] & 7) + 1,
            cash: u16::from_le_bytes([mem[p + OFF_CASH2], mem[p + OFF_CASH2 + 1]]) as u32
                * 10000
                + u16::from_le_bytes([mem[p + OFF_CASH1], mem[p + OFF_CASH1 + 1]]) as u32,
            fuel: mem[p + OFF_FUEL],
            legal: mem[p + OFF_LEGAL],
            rating: u32::from_le_bytes([
                mem[p + OFF_RATING],
                mem[p + OFF_RATING + 1],
                mem[p + OFF_RATING + 2],
                0,
            ]),
        };
        self.active = true;
    }

    /// Закрыть экран (возврат к экрану игры).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Нарисовать экран во весь слой.
    pub fn draw(&self, f: &mut Frame, font: &[u8]) {
        draw::draw(self, &self.info, f, font);
    }

    /// Строка денег для экрана (`100.0 CR`).
    fn cash_text(&self) -> String {
        format!("{}.{} CR", self.info.cash / 10, self.info.cash % 10)
    }

    /// Строка топлива для экрана (`7.0 LY`).
    fn fuel_text(&self) -> String {
        format!("{}.{} LY", self.info.fuel / 10, self.info.fuel % 10)
    }

    /// Правовой статус (`Cmd_L_PilotInfo`: `0` — чист, `< 0x32` — нарушитель).
    fn legal_text(&self) -> &'static str {
        match self.info.legal {
            0 => "CLEAN",
            1..=0x31 => "OFFENDER",
            _ => "FUGITIVE",
        }
    }

    /// Название боевого рейтинга по таблицам `byte_F0AE`/`byte_F0B4`.
    fn rating_text(&self) -> &'static str {
        match self.info.rating {
            0..=7 => "HARMLESS",
            8..=15 => "MOSTLY HARMLESS",
            16..=31 => "POOR",
            32..=63 => "AVERAGE",
            64..=127 => "ABOVE AVERAGE",
            128..=511 => "COMPETENT",
            512..=2559 => "DANGEROUS",
            2560..=6399 => "DEADLY",
            _ => "ELITE",
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::new()
    }
}

/// Имя командира: строка до `NUL`, без хвостовых пробелов; непечатаемые
/// байты заменяются на `?`, чтобы повреждённый сейв не ломал отрисовку.
fn read_name(block: &[u8]) -> String {
    let end = block.iter().position(|&b| b == 0).unwrap_or(block.len());
    let name: String = block[..end]
        .iter()
        .map(|&b| if (0x20..0x7F).contains(&b) { b as char } else { '?' })
        .collect();
    name.trim_end().to_string()
}
