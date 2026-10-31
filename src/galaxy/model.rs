//! Данные звезды, версии игры и построение галактики.

use super::*;

/// Одна звезда галактики.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Planet {
    /// Порядковый номер в последовательности галактики (`0..255`).
    pub index: usize,
    /// Экранная горизонталь `0..255` — старший байт `w1`.
    pub x: i32,
    /// Сырая вертикаль `0..255` — старший байт `w0`.
    pub y_raw: i32,
    /// Шесть байт состояния звезды: `w0`, `w1`, `w2` (мл., ст.).
    pub state: [u8; 6],
}

impl Planet {
    /// Экранная строка карты `24..151`.
    pub fn y(&self) -> i32 {
        (self.y_raw >> 1) + FIELD_TOP
    }

    /// Радиус кружка планеты (как на малой карте игры, `PrnPlanetOnSmallMap`):
    /// класс `(hi(w2) & 0x0C) >> 2` отображается в `3..5`.
    pub fn radius(&self) -> i32 {
        let c = ((self.state[5] as i32) & 0x0C) >> 2;
        (if c == 0 { 1 } else { c }) + 2
    }
}

/// Особенности расчёта расстояний в конкретной версии игры.
///
/// Модуль разности в игре считается отдельной рутиной, и в версии Кладова
/// она отличается от оригинальной на единицу. Поэтому расстояния в двух
/// версиях игры различаются — и нативная карта обязана повторять именно тот
/// вариант, который считает запущенная игра.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metric {
    Kladov,
}

impl Metric {
    /// Модуль разности как в игре: `|a - b|`, где `a` — уменьшаемое.
    pub fn abs_diff(self, a: i32, b: i32) -> i32 {
        let d = a - b;
        if d < 0 { -d - 1 } else { d }
    }
}

/// Раскладка адресов галактики в памяти конкретной версии игры.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GalaxyLayout {
    /// Семя галактики (`PilotGalaxyData0`), 6 байт.
    pub seed: usize,
    /// Таблица диграфов имён (`TxtPairOfChr`/`byte_FDB3`), 64 байт.
    pub digraphs: usize,
    /// Номер текущей галактики (`PilotNumGalaxy`, 0..7).
    pub galaxy: usize,
    /// Координаты текущей системы: X и «сырой» Y.
    pub sys_x: usize,
    pub sys_y: usize,
    /// Топливо (`Pilot_Fuel`): задаёт радиус достижимости на карте.
    pub fuel: usize,
    /// Особенности расчёта расстояний в этой версии игры.
    pub metric: Metric,
}

/// Версия Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_GALAXY: GalaxyLayout = GalaxyLayout {
    seed: 0xD090,
    digraphs: 0xFDB3,
    galaxy: 0xD065,
    sys_x: 0xD099,
    sys_y: 0xD097,
    fuel: 0xD082,
    metric: Metric::Kladov,
};

/// Шаг `ModifyPlanetData`.
pub(super) fn twist(s: [u8; 6]) -> [u8; 6] {
    let w0 = u16::from_le_bytes([s[0], s[1]]);
    let w1 = u16::from_le_bytes([s[2], s[3]]);
    let w2 = u16::from_le_bytes([s[4], s[5]]);
    let n0 = w1;
    let n1 = w2;
    let n2 = w0.wrapping_add(w1).wrapping_add(w2);
    [
        n0 as u8,
        (n0 >> 8) as u8,
        n1 as u8,
        (n1 >> 8) as u8,
        n2 as u8,
        (n2 >> 8) as u8,
    ]
}

/// Построить все звёзды галактики из 6-байтного семени.
pub fn planets(seed: [u8; 6]) -> Vec<Planet> {
    let mut s = seed;
    let mut out = Vec::with_capacity(COUNT);
    for index in 0..COUNT {
        out.push(Planet {
            index,
            x: s[3] as i32,
            y_raw: s[1] as i32,
            state: s,
        });
        for _ in 0..4 {
            s = twist(s);
        }
    }
    out
}
