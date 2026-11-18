//! Нативный экран информации о выбранной планете — клавиша F7,
//! альтернатива экрану `DATA ON <PLANET>` (клавиша `P` в версии Кладова).
//!
//! Данные читаются из живых полей планеты, подготовленных оригиналом при
//! показе экрана (`FuelToPlanet`, `PlanetEconomy`, `PlanetGovernment`,
//! `PlanetTechLevel`, `PlanetPopulation`, `PlanetGrossProduct`,
//! `PlanetData1/2`). Имя планеты приходит из централизованного состояния
//! игры. Экран ничего не меняет и служит только для просмотра.

use crate::display::Frame;

mod draw;
#[cfg(test)]
mod tests;

/// Адреса полей планеты в памяти Z80 (версия Кладова).
const ADDR_PLANET_DATA1: usize = 0xDEBC;
const ADDR_PLANET_DATA2: usize = 0xDEBE;
const ADDR_FUEL_TO_PLANET: usize = 0xDEF6;
const ADDR_PLANET_ECONOMY: usize = 0xDEF8;
const ADDR_PLANET_GOVERNMENT: usize = 0xDEF9;
const ADDR_PLANET_TECH_LEVEL: usize = 0xDEFA;
const ADDR_PLANET_POPULATION: usize = 0xDEFB;
const ADDR_PLANET_GROSS_PRODUCT: usize = 0xDEFC;

/// Раскладка экрана информации о планете (версия Кладова).
#[derive(Clone, Copy)]
pub struct SysInfoLayout {
    /// Заглушка: все поля адресуются константами.
    pub _unused: u8,
}

/// Экран информации о планете (F7).
#[derive(Default)]
pub struct SysInfo {
    /// Экран открыт и показывается вместо игры.
    pub active: bool,
    /// Данные текущего экрана.
    pub info: Info,
}

/// Подготовленные данные экрана.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Info {
    /// Название планеты (как печатает оригинал: первая буква заглавная).
    pub name: String,
    /// Расстояние до планеты в десятых светового года; 0 — не показывать.
    pub distance_tenths: u8,
    /// Строка экономики, например `RICH INDUSTRIAL`.
    pub economy: String,
    /// Форма правления.
    pub government: String,
    /// Технологический уровень (1..).
    pub tech_level: u8,
    /// Население в десятых миллиарда.
    pub population_tenths: u8,
    /// Тип жителей (`HUMAN COLONIALS` / `HUMANOIDS`).
    pub species: String,
    /// Валовой продукт, млн. кр.
    pub gross_product: u16,
    /// Средний радиус, км.
    pub radius_km: u16,
}

/// Заготовка для версии Кладова.
pub const KLADOV_SYSINFO: SysInfoLayout = SysInfoLayout { _unused: 0 };

impl SysInfo {
    /// Новый выключенный экран.
    pub fn new() -> Self {
        Self::default()
    }

    /// Открыть экран и прочитать данные из памяти эмулятора. Имя планеты
    /// приходит из централизованного состояния игры, а не из поля `PlanetName`.
    pub fn open(&mut self, mem: &[u8], _layout: SysInfoLayout, name: &str) {
        self.info = read_info(mem, name);
        self.active = true;
    }

    /// Закрыть экран.
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Нарисовать экран в HD-слой.
    pub fn draw(&self, hd: &mut Frame, font: &[u8; crate::font::ZX_FONT_LEN]) {
        if !self.active {
            return;
        }
        draw::draw(hd, font, &self.info);
    }
}

/// Прочитать и разобрать данные экрана из памяти.
pub(crate) fn read_info(mem: &[u8], name: &str) -> Info {
    let distance_tenths = mem.get(ADDR_FUEL_TO_PLANET).copied().unwrap_or(0);
    let economy_code = mem.get(ADDR_PLANET_ECONOMY).copied().unwrap_or(0);
    let gov_code = mem.get(ADDR_PLANET_GOVERNMENT).copied().unwrap_or(0);
    let tech_level = mem
        .get(ADDR_PLANET_TECH_LEVEL)
        .copied()
        .unwrap_or(0)
        .wrapping_add(1);
    let population_tenths = mem.get(ADDR_PLANET_POPULATION).copied().unwrap_or(0);
    let gross_product = read_u16(mem, ADDR_PLANET_GROSS_PRODUCT);
    let data1 = read_u16(mem, ADDR_PLANET_DATA1);
    let data2 = read_u16(mem, ADDR_PLANET_DATA2);

    let radius_km = (((((data2 >> 8) & 0x0F) + 0x0B) as u16) << 8) | ((data1 >> 8) as u16);
    let species = if (data2 & 0x80) != 0 {
        "Humanoids".to_string()
    } else {
        "Human colonials".to_string()
    };

    Info {
        name: name.to_string(),
        distance_tenths,
        economy: economy_name(economy_code),
        government: government_name(gov_code),
        tech_level,
        population_tenths,
        species,
        gross_product,
        radius_km,
    }
}

fn read_u16(mem: &[u8], addr: usize) -> u16 {
    let lo = mem.get(addr).copied().unwrap_or(0) as u16;
    let hi = mem.get(addr + 1).copied().unwrap_or(0) as u16;
    lo | (hi << 8)
}

/// Название экономики по `PlanetEconomy` (см. `sub_EEA8`).
pub(crate) fn economy_name(code: u8) -> String {
    if code == 4 {
        return "Mainly agricultural".to_string();
    }
    let adj_code = if code > 4 { code - 5 } else { code } & 3;
    let adjective = match adj_code {
        0 => "Rich",
        1 => "Average",
        2 => "Poor",
        _ => "Mainly",
    };
    let noun = if (code & 4) != 0 {
        "agricultural"
    } else {
        "industrial"
    };
    format!("{adjective} {noun}")
}

/// Форма правления по `PlanetGovernment` (токены `0x11..0x18`).
pub(crate) fn government_name(code: u8) -> String {
    match code & 7 {
        0 => "Anarchy",
        1 => "Feudal",
        2 => "Multi-government",
        3 => "Dictatorship",
        4 => "Communist",
        5 => "Confederacy",
        6 => "Democracy",
        _ => "Corporate state",
    }
    .to_string()
}

/// Формат расстояния в десятых светового года: `x.x`, либо пусто при 0.
pub(crate) fn format_distance(tenths: u8) -> String {
    if tenths == 0 {
        String::new()
    } else {
        format!("{}.{}", tenths / 10, tenths % 10)
    }
}

/// Формат населения в десятых миллиарда.
pub(crate) fn format_population(tenths: u8) -> String {
    format!("{}.{}", tenths / 10, tenths % 10)
}
