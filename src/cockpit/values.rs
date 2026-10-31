//! Модель показаний панели: строки, индикаторы, опасность и компас.
//!
//! Кокпит не читает память Z80 сам: вызывающий код готовит снимок
//! [`CockpitValues`] на кадр. Здесь же лежат диапазоны значений и
//! детерминированные показания для заставки.

use super::palette::{DANGER_COLOR, DANGER_GREEN, DANGER_YELLOW};

/// Высота нижней части в знакоместах — 7 видимых строк, как в оригинале.
pub const PANEL_ROWS: i32 = 7;

/// Количество позиций запаса ракет — четыре, как в оригинале.
pub const MISSILE_SLOTS: i32 = 4;
/// Номер строки с запасом ракет (слева внизу).
pub const MISSILE_ROW: i32 = PANEL_ROWS - 1;

/// Максимум шкалы индикатора (`0xA0` = 100%, как в оригинале).
pub const GAUGE_MAX: u8 = 0xA0;
/// Максимальное отклонение стрелки влево/вправо (нейтраль — 0).
pub const DEFLECTION_MAX: i8 = 15;

/// Значение одной длинной клетки панели.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indicator {
    /// Шкала `0..=GAUGE_MAX`.
    Gauge(u8),
    /// Отклонение влево/вправо: `-DEFLECTION_MAX..=DEFLECTION_MAX`, 0 — центр.
    Deflection(i8),
    /// Запас ракет `0..=MISSILE_SLOTS`.
    Missiles(u8),
    /// Пустая клетка.
    Empty,
}

/// Цвет индикатора опасности.
///
/// Цвета совпадают с атрибутами, которые `PrintIndDanger` записывает в экран:
/// `0x04` — зелёный, `0x06` — жёлтый, `0x02` — красный. Мигание — это
/// чередование атрибутов `0x02` и `0x00`, поэтому отдельное состояние
/// [`Danger::BlinkingRed`] оставлено только для искусственных значений
/// (например, заставки), а из памяти читается уже текущая фаза.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Danger {
    Off,
    Green,
    Yellow,
    Red,
    BlinkingRed,
}

impl Danger {
    /// Разобрать атрибут знакоместа индикатора опасности.
    ///
    /// В оригинале (`PrintIndDanger`) в атрибут пишется цвет чернил: `0x04`,
    /// `0x06`, `0x02` или `0x00` (погасшая фаза мигания). Старшие биты
    /// (яркость, мигание) на цвет самого кружка не влияют.
    pub fn from_attribute(value: u8) -> Danger {
        match value & 0x07 {
            0x04 => Danger::Green,
            0x06 => Danger::Yellow,
            0x02 => Danger::Red,
            _ => Danger::Off,
        }
    }

    /// Цвет для текущей фазы мигания; `None` — кружок не рисуется.
    pub(super) fn color(self, blink_on: bool) -> Option<u32> {
        match self {
            Danger::Off => None,
            Danger::Green => Some(DANGER_GREEN),
            Danger::Yellow => Some(DANGER_YELLOW),
            Danger::Red => Some(DANGER_COLOR),
            Danger::BlinkingRed => blink_on.then_some(DANGER_COLOR),
        }
    }
}

/// Положение указателя компаса.
///
/// `x` и `y` — смещение от центра в «пикселях» оригинального компаса:
/// `-8..=8`, положительный `y` — вверх (как в `CompasPointerCrdY`).
/// `filled` — цель впереди (`CompasPointerType == 0`), иначе указатель
/// рисуется полым.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Compass {
    pub x: i8,
    pub y: i8,
    pub filled: bool,
}

impl Compass {
    /// Указатель в центре — цель отсутствует или совпадает с игроком.
    pub const fn centered() -> Compass {
        Compass {
            x: 0,
            y: 0,
            filled: true,
        }
    }
}

impl Default for Compass {
    fn default() -> Self {
        Self::centered()
    }
}

/// Снимок показаний панели на один кадр.
///
/// Индексы строк совпадают с [`super::LEFT_LABELS`] и
/// [`super::RIGHT_LABELS`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CockpitValues {
    pub left: [Indicator; PANEL_ROWS as usize],
    pub right: [Indicator; PANEL_ROWS as usize],
    /// Индикатор опасности в верхнем левом кружке.
    pub danger: Danger,
    /// Указатель компаса в верхнем правом кружке.
    pub compass: Compass,
    /// Фаза мигания для [`Danger::BlinkingRed`].
    pub blink: bool,
}

impl CockpitValues {
    /// Псевдослучайные показания для заставки.
    ///
    /// Генератор детерминированный: набор не меняется от кадра к кадру,
    /// поэтому панель на заставке не мерцает.
    pub fn for_splash() -> Self {
        let mut seed = 0x5EED_1234u32;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 16) as u8
        };
        let gauge = |v: u8| Indicator::Gauge(v % (GAUGE_MAX + 1));
        let deflection = |v: u8| Indicator::Deflection((v % 31) as i8 - DEFLECTION_MAX);
        CockpitValues {
            left: [
                gauge(next()),
                gauge(next()),
                gauge(next()),
                gauge(next()),
                gauge(next()),
                gauge(next()),
                Indicator::Missiles(next() % (MISSILE_SLOTS as u8 + 1)),
            ],
            right: [
                gauge(next()),
                deflection(next()),
                deflection(next()),
                gauge(next()),
                gauge(next()),
                gauge(next()),
                gauge(next()),
            ],
            danger: Danger::Green,
            compass: Compass::centered(),
            blink: true,
        }
    }
}
