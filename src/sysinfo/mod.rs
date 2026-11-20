//! Нативный экран информации о выбранной планете — клавиша F7,
//! альтернатива экрану `DATA ON <PLANET>` (клавиша `P` в версии Кладова).
//!
//! Данные читаются из живых полей планеты, подготовленных оригиналом при
//! показе экрана (`FuelToPlanet`, `PlanetEconomy`, `PlanetGovernment`,
//! `PlanetTechLevel`, `PlanetPopulation`, `PlanetGrossProduct`,
//! `PlanetData1/2`). Имя планеты приходит из централизованного состояния
//! игры. Экран ничего не меняет и служит только для просмотра.

use crate::display::Frame;
use crate::galaxy;

mod draw;
#[cfg(test)]
mod tests;

/// Адреса подготовленных полей планеты в памяти Z80 (версия Кладова).
#[cfg(test)]
const ADDR_PLANET_DATA1: usize = 0xDEBC;
#[cfg(test)]
const ADDR_PLANET_DATA2: usize = 0xDEBE;
#[cfg(test)]
const ADDR_FUEL_TO_PLANET: usize = 0xDEF6;
#[cfg(test)]
const ADDR_PLANET_ECONOMY: usize = 0xDEF8;
#[cfg(test)]
const ADDR_PLANET_GOVERNMENT: usize = 0xDEF9;
#[cfg(test)]
const ADDR_PLANET_TECH_LEVEL: usize = 0xDEFA;
#[cfg(test)]
const ADDR_PLANET_POPULATION: usize = 0xDEFB;
#[cfg(test)]
const ADDR_PLANET_GROSS_PRODUCT: usize = 0xDEFC;

/// Экран информации о планете (F7): показывает текущую и выбранную системы.
#[derive(Default)]
pub struct SysInfo {
    /// Экран открыт и показывается вместо игры.
    pub active: bool,
    /// Данные текущей системы.
    pub current: Info,
    /// Данные выбранной на карте системы.
    pub selected: Info,
}

/// Подготовленные данные одной системы.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Info {
    /// Название планеты (как печатает оригинал: первая буква заглавная).
    pub name: String,
    /// Расстояние до планеты в десятых светового года от текущей системы;
    /// 0 — не показывать.
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



impl SysInfo {
    /// Новый выключенный экран.
    pub fn new() -> Self {
        Self::default()
    }

    /// Открыть экран и вычислить данные текущей и выбранной систем из памяти.
    pub fn open(&mut self, mem: &[u8], layout: galaxy::GalaxyLayout) {
        let (current, selected) = read_both(mem, layout);
        self.current = current;
        self.selected = selected;
        self.active = true;
    }

    /// Обновить данные экрана из памяти запущенной игры.
    pub fn refresh(&mut self, mem: &[u8], layout: galaxy::GalaxyLayout) {
        let (current, selected) = read_both(mem, layout);
        self.current = current;
        self.selected = selected;
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
        draw::draw(hd, font, &self.current, &self.selected);
    }
}

fn adc(a: u8, b: u8, carry: bool) -> (u8, bool) {
    let r = a as u16 + b as u16 + carry as u16;
    (r as u8, r > 0xFF)
}

/// Вычислить параметры планеты из её 6-байтного состояния по алгоритму
/// `GenerateInfoPlanet` (см. `kladov/10_circles_planets/10_circles_planets.asm`).
///
/// `distance_tenths` — расстояние от текущей системы до этой планеты
/// (для текущей системы 0).
pub(crate) fn info_from_state(state: [u8; 6], name: &str, distance_tenths: u8) -> Info {
    let w0hi = state[1];
    let w1lo = state[2];
    let w1hi = state[3];
    let w2lo = state[4];
    let w2hi = state[5];

    let mut economy = w0hi & 7;
    let government = (w1lo >> 3) & 7;
    // srl a (a=government); jr nz,skip — коррекция только если (government >> 1) == 0
    if (government >> 1) == 0 {
        economy |= 2;
    }

    // Технологический уровень.
    let (tech_level, _) = {
        // a=eco; xor 7; and a (C=0); b=a
        // a=w1hi; and 3; add a,b (C=c1, но затем перезаписывается srl)
        let (sum1, _c1) = adc(w1hi & 3, economy ^ 7, false);
        // a=government; srl a -> C = бит 0 government
        let gov_srl = government >> 1;
        let c_gov = (government & 1) != 0;
        // adc a,b
        adc(gov_srl, sum1, c_gov)
    };

    // Население: eco + tech*4 + gov + 1 (с переносами).
    let (tech_x4, c_mul) = {
        let (t2, _) = adc(tech_level, tech_level, false); // add a,a
        adc(t2, t2, false) // add a,a
    };
    let population_tenths = {
        let (p1, c1) = adc(economy, tech_x4, c_mul);
        let (p2, c2) = adc(p1, government, c1);
        let (pop, _c3) = adc(p2, 1, c2);
        pop
    };

    // Валовой продукт.
    let gross_product = {
        // a=eco; xor 7 (C=0); adc a,3 -> d, C=c_d
        let (d, c_d) = adc(economy ^ 7, 3, false);
        // a=government; adc a,4 с C=c_d (ld a,(nn) флаги не меняет)
        let (mul_a, _) = adc(government, 4, c_d);
        let first = (d as u16).wrapping_mul(mul_a as u16);
        // HL = population * low(first); add hl,hl x3
        let second = (population_tenths as u16).wrapping_mul((first & 0x00FF) as u16);
        second.wrapping_mul(8)
    };

    let radius_km = (((w2hi & 0x0F) as u16 + 0x0B) << 8) | (w1hi as u16);
    let species = if (w2lo & 0x80) != 0 {
        "Humanoids".to_string()
    } else {
        "Human colonials".to_string()
    };

    Info {
        name: name.to_string(),
        distance_tenths,
        economy: economy_name(economy),
        government: government_name(government),
        tech_level: tech_level.wrapping_add(1),
        population_tenths,
        species,
        gross_product,
        radius_km,
    }
}

/// Найти планету по координатам цели/системы и вернуть её индекс.
fn find_planet_index(
    planets: &[galaxy::Planet],
    layout: galaxy::GalaxyLayout,
    x: i32,
    y_raw: i32,
) -> usize {
    galaxy::nearest(planets, layout.metric, x, y_raw)
        .map(|p| p.index)
        .unwrap_or(0)
}

/// Прочитать данные текущей и выбранной систем из памяти игры.
pub(crate) fn read_both(mem: &[u8], layout: galaxy::GalaxyLayout) -> (Info, Info) {
    let seed: [u8; 6] = mem
        .get(layout.seed..layout.seed + 6)
        .and_then(|s| s.try_into().ok())
        .unwrap_or([0; 6]);
    let digraphs = mem
        .get(layout.digraphs..layout.digraphs + 64)
        .map(|d| d.to_vec())
        .unwrap_or_else(|| vec![0; 64]);
    let planets = galaxy::planets(seed);

    let sys_x = mem.get(layout.sys_x).copied().unwrap_or(0) as i32;
    let sys_y = mem.get(layout.sys_y).copied().unwrap_or(0) as i32;
    let tgt_x = mem.get(layout.target_x).copied().unwrap_or(0) as i32;
    let tgt_y = mem.get(layout.target_y).copied().unwrap_or(0) as i32;

    let sys_i = find_planet_index(&planets, layout, sys_x, sys_y);
    let tgt_i = find_planet_index(&planets, layout, tgt_x, tgt_y);

    let current_name = galaxy::name(planets[sys_i].state, &digraphs);
    let selected_name = galaxy::name(planets[tgt_i].state, &digraphs);

    let distance_tenths = galaxy::fuel_to_cursor(layout.metric, sys_x, sys_y, tgt_x, tgt_y)
        .clamp(0, 255) as u8;

    let current = info_from_state(planets[sys_i].state, &current_name, 0);
    let selected = info_from_state(planets[tgt_i].state, &selected_name, distance_tenths);
    (current, selected)
}

/// Прочитать данные выбранной планеты из подготовленных полей памяти Z80.
/// Используется для совместимости и тестов.
#[cfg(test)]
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

#[cfg(test)]
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
