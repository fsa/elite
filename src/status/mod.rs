//! Нативный экран статуса пилота — клавиша F9, альтернатива экрану `L`.
//!
//! Оформление — просто белая рамка вокруг всего экрана. Данные читаются из
//! живого блока командира (`PilotName`); экран ничего не меняет и служит
//! только для просмотра.
//!
//! Показываются поля оригинала (`Cmd_L_PilotInfo` в версии Кладова):
//! имя командира, текущая и целевая системы гиперпрыжка, деньги, топливо,
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
    /// Система, выбранная для гиперпрыжка.
    target: String,
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
        let digraphs = &mem[gl.digraphs..gl.digraphs + 64];
        let at = |x: usize, y: usize| {
            galaxy::nearest(&planets, gl.metric, mem[x] as i32, mem[y] as i32)
                .map(|planet| title_case(&galaxy::name(planet.state, digraphs)))
                .unwrap_or_default()
        };
        self.info = Info {
            name: read_name(&mem[p..p + NAME_LEN]),
            system: at(gl.sys_x, gl.sys_y),
            target: at(gl.target_x, gl.target_y),
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

    /// Строка денег для экрана (`100.0 Cr`).
    fn cash_text(&self) -> String {
        format!("{}.{} Cr", self.info.cash / 10, self.info.cash % 10)
    }

    /// Строка топлива для экрана (`7.0 Light Years`).
    fn fuel_text(&self) -> String {
        format!("{}.{} Light Years", self.info.fuel / 10, self.info.fuel % 10)
    }

    /// Правовой статус (`Cmd_L_PilotInfo`: `0` — чист, `< 0x32` — нарушитель).
    fn legal_text(&self) -> &'static str {
        match self.info.legal {
            0 => "Clean",
            1..=0x31 => "Offender",
            _ => "Fugitive",
        }
    }

    /// Название боевого рейтинга по таблицам `byte_F0AE`/`byte_F0B4`.
    fn rating_text(&self) -> &'static str {
        match self.info.rating {
            0..=7 => "Harmless",
            8..=15 => "Mostly Harmless",
            16..=31 => "Poor",
            32..=63 => "Average",
            64..=127 => "Above Average",
            128..=511 => "Competent",
            512..=2559 => "Dangerous",
            2560..=6399 => "Deadly",
            _ => "Elite",
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

/// Имя системы в том виде, как его печатает оригинал: первая буква заглавная,
/// остальные — строчные (`LAVE` -> `Lave`).
fn title_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
        None => String::new(),
    }
}
