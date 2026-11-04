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

/// Сколько контактов помещается в снимок радара.
///
/// Оригинальный сканер обходит станцию, шесть основных слотов и слот ракеты,
/// поэтому восемь — ровно столько, сколько он может показать за кадр.
pub const RADAR_CONTACTS: usize = 8;

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
    ///
    /// Сейчас не встречается в снимках игры, но остаётся в модели: экран
    /// умеет рисовать пустую клетку, а тесты проверяют этот случай.
    #[allow(dead_code)]
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
    /// Искусственное мигание для заставки; из памяти читается текущая фаза.
    #[allow(dead_code)]
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
/// рисуется полым. `visible` — рисовать ли указатель вообще: на титульном
/// экране ZX `PrnCocpit_Ifn` рисует только кольцо (`DataCompass`), а сам
/// указатель появляется лишь в полёте (`PrintCompass`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Compass {
    pub x: i8,
    pub y: i8,
    pub filled: bool,
    pub visible: bool,
}

impl Compass {
    /// Указатель в центре — цель отсутствует или совпадает с игроком.
    pub const fn centered() -> Compass {
        Compass {
            x: 0,
            y: 0,
            filled: true,
            visible: true,
        }
    }

    /// Только кольцо без указателя — как на титульном экране ZX.
    pub const fn hidden() -> Compass {
        Compass {
            visible: false,
            ..Compass::centered()
        }
    }
}

impl Default for Compass {
    fn default() -> Self {
        Self::centered()
    }
}

/// Что за объект показывает радар.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadarContactKind {
    /// Станция (`objCoriolis`).
    Station,
    /// Корабль из одного из основных слотов.
    Ship,
    /// Ракета (`objMissile`).
    Missile,
}

/// Один контакт радара в системе координат игрока.
///
/// `x` — вправо, `y` — вверх, `z` — вперёд; единицы те же, что в слотах
/// объектов Z80. Компоненты радара сами решают, как отобразить контакт,
/// поэтому модель не зависит от геометрии и от полётной подсистемы.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadarContact {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub kind: RadarContactKind,
}

impl RadarContact {
    const EMPTY: RadarContact = RadarContact {
        x: 0,
        y: 0,
        z: 0,
        kind: RadarContactKind::Ship,
    };
}

/// Небольшой набор контактов радара фиксированного размера.
///
/// Массив внутри, а не `Vec`, чтобы [`CockpitValues`] оставался `Copy`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadarContacts {
    items: [RadarContact; RADAR_CONTACTS],
    len: u8,
}

impl RadarContacts {
    /// Пустой набор — радар без целей.
    pub const fn empty() -> Self {
        RadarContacts {
            items: [RadarContact::EMPTY; RADAR_CONTACTS],
            len: 0,
        }
    }

    /// Добавить контакт; лишние сверх [`RADAR_CONTACTS`] отбрасываются.
    pub fn push(&mut self, contact: RadarContact) {
        if (self.len as usize) < RADAR_CONTACTS {
            self.items[self.len as usize] = contact;
            self.len += 1;
        }
    }

    /// Контакты, поместившиеся в снимок.
    pub fn as_slice(&self) -> &[RadarContact] {
        &self.items[..self.len as usize]
    }
}

impl Default for RadarContacts {
    fn default() -> Self {
        Self::empty()
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
    /// Контакты радара в системе координат игрока.
    pub radar: RadarContacts,
    /// Станция рядом (`CoriolisNear`): на панели виден символ `S` и доступен
    /// стыковочный компьютер.
    pub station_near: bool,
    /// Работает ECM (`TimeWorkECM <> 0`): на панели виден символ `E`.
    pub ecm_on: bool,
    /// Фаза мигания для [`Danger::BlinkingRed`].
    pub blink: bool,
}

impl CockpitValues {
    /// Показания панели на заставке — как на титульном экране ZX.
    ///
    /// Оригинал не пересчитывает приборы на заставке: `PrnCocpit_Ifn`
    /// восстанавливает сохранённый растровый кокпит (`DataCocpit`), поэтому
    /// шкалы показывают то состояние, которое было на экране в момент
    /// снапшота. Значения ниже измерены по нижним 64 строкам ZX-экрана
    /// версии Кладова (`zx_src/kladov.snapshot`): длина белой заливки в
    /// полосе переведена в байты `0..0xA0`, маркер `RL` стоит по центру,
    /// `DC` — на 8 пикселей левее, ракеты не показаны, опасности нет.
    /// Оба символа `E`/`S` на титульном экране горят: их атрибуты сохранены
    /// в `DataCocpit` белыми (`E` — ECM, `S` — станция рядом).
    ///
    /// Набор постоянный, поэтому панель на заставке не мерцает.
    pub fn for_splash() -> Self {
        CockpitValues {
            left: [
                Indicator::Gauge(0x80), // FS — 32 пикселя заливки
                Indicator::Gauge(0x3C), // AS — 15
                Indicator::Gauge(0x9C), // FU — 39
                Indicator::Gauge(0x10), // CT — 4
                Indicator::Gauge(0x04), // LT — 1
                Indicator::Gauge(0x9C), // AL — 39
                Indicator::Missiles(0),
            ],
            right: [
                Indicator::Gauge(0x00),      // SP — пусто
                Indicator::Deflection(0),    // RL — маркер по центру
                Indicator::Deflection(-8),   // DC — маркер левее центра
                Indicator::Gauge(0x88),      // 1 — 34 пикселя
                Indicator::Gauge(0xA0),      // 2 — полная шкала
                Indicator::Gauge(0xA0),      // 3
                Indicator::Gauge(0xA0),      // 4
            ],
            danger: Danger::Off,
            compass: Compass::hidden(),
            radar: RadarContacts::empty(),
            station_near: true,
            ecm_on: true,
            blink: true,
        }
    }
}
