//! Нативный кокпит (приборная панель) корабля.
//!
//! Кокпит используется на двух экранах:
//!
//! * в полёте (`F1`) — вокруг вида из кабины и под ним;
//! * на заставке — вокруг поля с вращающимся кораблём и под ним.
//!
//! Раньше рамку вокруг вида рисовал [`crate::flight`], а нижнюю часть —
//! `Cockpit::draw`; из-за этого кокпит на заставке выглядел иначе (у неё не
//! было главной рамки). Теперь вся «обвязка» кокпита — главная рамка и рамка
//! нижней части — живёт здесь, а экраны только рисуют своё содержимое.
//!
//! В оригинале панель — статический растр в видеопамяти ZX (`DataCocpit`,
//! оригинал `0xC800`, Кладов `0xC000`), нижние 64 строки экрана; индикаторы
//! уложены в 7 строк по высоте, на которые поверх допечатываются значения:
//! шесть полос (щиты, топливо, температура, лазер, высота), скорость и четыре
//! энергоблока, компас и радиолокатор (`UpdateAllIndicators`, `PrintCompass`,
//! `PrintObjScaner`).
//!
//! Сейчас панель — таблица с рамками: слева и справа по семь строк, в центре
//! одна большая клетка на всю высоту. В каждой боковой строке две клетки:
//! короткая подпись/значок на чёрном фоне и длинная клетка-полоса. Полосы
//! залиты цветом типа корабля игрока: у Cobra Mk III красным, у Cobra Mk IV
//! фиолетовым; на заставке всегда красным. Подписи повторяют обозначения
//! оригинала: слева `FS`, `AS`, `FU`, `CT`, `LT`, `AL` и значок скорости,
//! справа `SP`, `RL`, `DC` и номера энергоблоков `1`..`4`. Они нарисованы
//! тем же шрифтом ПЗУ ZX, что и заголовок `ELITE` на заставке.
//!
//! Длинная клетка-полоса задаётся в процентах от ширины панели
//! ([`BAR_WIDTH_FRAC`]) и растягивается вместе с окном. Клетка подписи
//! фиксирована по высоте строки ([`LABEL_WIDTH_PER_ROW_PCT`]) и при смене
//! соотношения сторон не растягивается.
//!
//! Значения панели приходят одним снимком [`CockpitValues`]: кокпит не
//! читает память Z80 сам. [`CockpitLayout::read`] превращает байты живой
//! памяти в [`Indicator`] для каждой строки. Статическая часть панели
//! (`draw_chrome`) и значения (`draw_indicators`) рисуются раздельно, но
//! вызывающий код делает один вызов [`Cockpit::draw`].
//!
//! Геометрия (`CockpitGeom`) отдаёт прямоугольники вида, панели, клеток и
//! разделителей, чтобы следующие шаги рисовали внутри неё, а не заново
//! считали отступы.

use crate::display::Frame;

/// Цвет линий кокпита — «белый» ZX (ink 7), как в оригинальном растре.
pub const FRAME_COLOR: u32 = 0xFFD7_D7D7;
/// Красный индикатор опасности и цвет рамки при попадании.
pub const DANGER_COLOR: u32 = 0xFFFF_4040;
/// Зелёный индикатор опасности (`IndDanger = 0x04`).
pub const DANGER_GREEN: u32 = 0xFF00_D700;
/// Жёлтый индикатор опасности (`IndDanger = 0x06`).
pub const DANGER_YELLOW: u32 = 0xFFD7_D700;
/// Цвет фона кокпита (paper 0).
pub const BACKGROUND: u32 = 0xFF00_0000;

/// Толщина линий рамки в пикселях окна.
pub const LINE: i32 = 3;
/// Доля высоты окна под нижнюю часть (панель).
///
/// В оригинале было 64/192 ≈ 33%; 25% оставляют место под заголовок
/// заставки, но делают все семь строк панели одинаково высокими.
const PANEL_FRAC: i32 = 25;
/// Высота нижней части в знакоместах — 7 видимых строк, как в оригинале.
pub const PANEL_ROWS: i32 = 7;

/// Ширина длинной клетки-полосы в процентах от внутренней ширины панели.
///
/// Эта клетка растягивается вместе с шириной окна.
pub const BAR_WIDTH_FRAC: i32 = 15;
/// Ширина клетки подписи/значка в процентах от высоты строки.
///
/// Клетка с текстом не зависит от ширины окна: при изменении соотношения
/// сторон растягиваются только длинные клетки-полосы.
pub const LABEL_WIDTH_PER_ROW_PCT: i32 = 225;
/// Цвет длинных клеток у Cobra Mk III — красный (ZX paper 2).
pub const BAR_RED: u32 = 0xFFD7_0000;
/// Цвет длинных клеток у Cobra Mk IV — фиолетовый (ZX paper 3).
pub const BAR_MAGENTA: u32 = 0xFFD7_00D7;

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
/// Значения взяты из оригинала (`IndDanger`): `0x04` — зелёный, `0x06` —
/// жёлтый, `0x02` — красный, `0x63` — мигающий красный. `0` — не рисовать.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Danger {
    Off,
    Green,
    Yellow,
    Red,
    BlinkingRed,
}

impl Danger {
    /// Разобрать байт `IndDanger`.
    pub fn from_byte(value: u8) -> Danger {
        match value {
            0x04 => Danger::Green,
            0x06 => Danger::Yellow,
            0x02 => Danger::Red,
            0x63 => Danger::BlinkingRed,
            _ => Danger::Off,
        }
    }

    /// Цвет для текущей фазы мигания; `None` — кружок не рисуется.
    fn color(self, blink_on: bool) -> Option<u32> {
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
/// Кокпит не читает память Z80 сам: вызывающий код (полёт или заставка)
/// готовит значения заранее. Индексы строк совпадают с [`LEFT_LABELS`] и
/// [`RIGHT_LABELS`].
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

/// Цвета кокпита: линии/индикаторы и фон длинных клеток.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CockpitStyle {
    pub line: u32,
    pub bar: u32,
}

impl CockpitStyle {
    /// Cobra Mk III и заставка: белые линии, красные полосы.
    pub const COBRA_MK3: CockpitStyle = CockpitStyle {
        line: FRAME_COLOR,
        bar: BAR_RED,
    };
    /// Cobra Mk IV: белые линии, фиолетовые полосы.
    pub const COBRA_MK4: CockpitStyle = CockpitStyle {
        line: FRAME_COLOR,
        bar: BAR_MAGENTA,
    };
}

impl Default for CockpitStyle {
    fn default() -> Self {
        Self::COBRA_MK3
    }
}

/// Подписи левой колонки сверху вниз.
///
/// Последняя строка — не текст, а значок скорости ([`SPEED_ICON`]), поэтому
/// для неё указан `None`.
pub const LEFT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("FS"),
    Some("AS"),
    Some("FU"),
    Some("CT"),
    Some("LT"),
    Some("AL"),
    None,
];
/// Подписи правой колонки сверху вниз: `SP`, `RL`, `DC` и номера энергоблоков.
pub const RIGHT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("SP"),
    Some("RL"),
    Some("DC"),
    Some("1"),
    Some("2"),
    Some("3"),
    Some("4"),
];
/// Растр значка скорости из оригинальной панели: 12×5 пикселей.
///
/// Горизонтальная шкала со стрелкой влево; старший бит — левый пиксель.
const SPEED_ICON: [u16; 5] = [
    0b011000000000,
    0b010111111111,
    0b011111111111,
    0b010111111111,
    0b011000000000,
];

/// Адреса переменных панели в памяти игры (текущее значение показания).
///
/// Каждый индикатор-полоса — байт `0..0xA0` (160 т.е. 100%), полоса занимает
/// 5x5 «пикселей» ZX от `AdrScrIndicators` (значение сдвигается вправо на 2).
/// Адреса различаются у версий на постоянный сдвиг (Кладов +0xA2).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
#[derive(Clone, Copy)]
pub struct CockpitLayout {
    /// Передний щит (`ShieldFront`).
    pub shield_front: u16,
    /// Задний щит (`ShieldRear`).
    pub shield_rear: u16,
    /// Топливо (`Fuel`).
    pub fuel: u16,
    /// Температура кабины (`Temperature`).
    pub temperature: u16,
    /// Температура лазера (`TempLaser`).
    pub laser_temp: u16,
    /// Высота/дальность до поверхности (`Altitude`).
    pub altitude: u16,
    /// Скорость (`Speed`).
    pub speed: u16,
    /// Четыре энергоблока (`ShieldUnit1..4`), идут подряд с шагом 2.
    pub shield_unit1: u16,
    /// Отклонение влево/вправо (`RotateLR`), нейтраль `0x15`.
    pub rotate_lr: u16,
    /// Отклонение вверх/вниз (`RotateDU`), нейтраль `0x15`.
    pub rotate_du: u16,
    /// Запас ракет (`PilotMissiles`), 0..4.
    pub missiles: u16,
    /// Индикатор опасности (`IndDanger`) — задаёт цвет рамки.
    pub ind_danger: u16,
    /// Признак межпланетного пространства (`InFreeSpace`).
    pub in_free_space: u16,
    /// Координаты планеты (`PlanetCrd`, 9 байт знак-величина).
    pub planet_crd: u16,
    /// Фиктивные координаты планеты вне системы (`fsPlCrdXl`, 9 байт).
    pub fake_planet_crd: u16,
    /// Слот станции (`objCoriolis`).
    pub coriolis_slot: u16,
    /// Флаг доступности J-драйва у станции (`J_Drive_Coriolis`).
    pub j_drive_coriolis: u16,
}

/// Раскладка переменных панели для оригинальной версии (`elite.bin`).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const ELITE_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x7046,
    shield_rear: 0x7048,
    fuel: 0x704A,
    temperature: 0x704C,
    laser_temp: 0x704E,
    altitude: 0x7050,
    speed: 0x7052,
    shield_unit1: 0x7054,
    rotate_lr: 0x705C,
    rotate_du: 0x705E,
    missiles: 0xA845,
    ind_danger: 0x7062,
    in_free_space: 0x7061,
    planet_crd: 0x706E,
    // Для оригинальной версии адрес фиктивных координат пока не найден.
    fake_planet_crd: 0,
    coriolis_slot: 0x5FA6,
    j_drive_coriolis: 0x707B,
};

/// Раскладка переменных панели для версии Кладова (`kladov.bin`), +0xA2.
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const KLADOV_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x70E8,
    shield_rear: 0x70EA,
    fuel: 0x70EC,
    temperature: 0x70EE,
    laser_temp: 0x70F0,
    altitude: 0x70F2,
    speed: 0x70F4,
    shield_unit1: 0x70F6,
    rotate_lr: 0x70FE,
    rotate_du: 0x7100,
    missiles: 0xD083,
    ind_danger: 0x7104,
    in_free_space: 0x7103,
    planet_crd: 0x7110,
    fake_planet_crd: 0x8AC3,
    coriolis_slot: 0x6048,
    j_drive_coriolis: 0x711D,
};

impl CockpitLayout {
    /// Прочитать показания панели из живой памяти Z80.
    ///
    /// Значения шкал ограничиваются [`GAUGE_MAX`], отклонения приводятся
    /// к нейтрали `0x15`, запас ракет — к [`MISSILE_SLOTS`].
    pub fn read(&self, mem: &[u8]) -> CockpitValues {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        let gauge = |addr: u16| Indicator::Gauge(byte(addr).min(GAUGE_MAX));
        let deflection = |addr: u16| {
            let offset = byte(addr) as i16 - 0x15;
            Indicator::Deflection(
                offset.clamp(-(DEFLECTION_MAX as i16), DEFLECTION_MAX as i16) as i8,
            )
        };
        CockpitValues {
            left: [
                gauge(self.shield_front),
                gauge(self.shield_rear),
                gauge(self.fuel),
                gauge(self.temperature),
                gauge(self.laser_temp),
                gauge(self.altitude),
                Indicator::Missiles(byte(self.missiles).min(MISSILE_SLOTS as u8)),
            ],
            right: [
                gauge(self.speed),
                deflection(self.rotate_lr),
                deflection(self.rotate_du),
                gauge(self.shield_unit1),
                gauge(self.shield_unit1 + 2),
                gauge(self.shield_unit1 + 4),
                gauge(self.shield_unit1 + 6),
            ],
            danger: Danger::from_byte(byte(self.ind_danger)),
            compass: self.read_compass(mem),
            blink: true,
        }
    }

    /// Прочитать указатель компаса.
    ///
    /// Цель выбирается как в `PrintCompass`:
    ///
    /// * вне системы — фиктивные координаты `fsPlCrd`;
    /// * рядом с планетой — станция (`objCoriolis`), если J-драйв у неё
    ///   доступен, иначе планета;
    /// * в остальных случаях — планета.
    fn read_compass(&self, mem: &[u8]) -> Compass {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        let target = if self.in_free_space != 0 && byte(self.in_free_space) != 0 {
            (self.fake_planet_crd != 0).then_some(self.fake_planet_crd)
        } else if self.planet_crd != 0 {
            let high = (byte(self.planet_crd + 2)
                | byte(self.planet_crd + 5)
                | byte(self.planet_crd + 8))
                & 0x7F;
            let station = self.coriolis_slot != 0
                && self.j_drive_coriolis != 0
                && byte(self.j_drive_coriolis) != 0xFF;
            if high < 2 && station {
                Some(self.coriolis_slot)
            } else {
                Some(self.planet_crd)
            }
        } else {
            None
        };
        let Some(target) = target else {
            return Compass::centered();
        };
        let crd = |offset: u16| {
            read_coordinate(
                mem.get(target as usize + offset as usize..)
                    .unwrap_or(&[]),
            )
        };
        let x = crd(0);
        let y = crd(3);
        let z = crd(6);
        let len = (x * x + y * y + z * z).sqrt();
        let offset = |v: f32| {
            if len < 0.5 {
                0
            } else {
                (v * COMPASS_POINTER_MAX as f32 / len)
                    .round()
                    .clamp(-(COMPASS_POINTER_MAX as f32), COMPASS_POINTER_MAX as f32)
                    as i8
            }
        };
        Compass {
            x: offset(x),
            y: offset(y),
            filled: z >= 0.0,
        }
    }
}

/// Максимальное смещение указателя компаса от центра, как в оригинале:
/// `CompasPointerCrdX/Y` лежат в диапазоне `-8..=8`.
const COMPASS_POINTER_MAX: i8 = 8;

/// Прочитать 24-битную координату в формате знак-величина.
fn read_coordinate(b: &[u8]) -> f32 {
    if b.len() < 3 {
        return 0.0;
    }
    let mag = (((b[2] & 0x7F) as u32) << 16) | ((b[1] as u32) << 8) | b[0] as u32;
    let value = mag as f32;
    if b[2] & 0x80 != 0 {
        -value
    } else {
        value
    }
}

/// Прямоугольник в пикселях окна.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    /// Прямоугольник, сжатый на `d` пикселей со всех сторон.
    pub fn inset(&self, d: i32) -> Rect {
        let d = d.max(0);
        Rect {
            x: self.x + d,
            y: self.y + d,
            w: (self.w - 2 * d).max(0),
            h: (self.h - 2 * d).max(0),
        }
    }

    /// Координата правого края (включительно).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn right(&self) -> i32 {
        self.x + self.w - 1
    }

    /// Координата нижнего края (включительно).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn bottom(&self) -> i32 {
        self.y + self.h - 1
    }

    /// Пуст ли прямоугольник (рисовать нечего).
    pub fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }
}

/// Геометрия кокпита для текущего окна.
///
/// Кокпит занимает всю ширину окна. Главная рамка окружает область вида
/// (верхнюю часть окна), нижняя часть (панель) прижата к низу окна.
/// Внутри панели координаты считаются в «единицах знакомест» шириной `cell`:
/// 32 знакоместа по горизонтали, [`PANEL_ROWS`] по вертикали — как в ZX-версии.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CockpitGeom {
    /// Высота окна.
    pub height: i32,
    /// Ширина окна.
    pub width: i32,
    /// Экранная Y-координата нижней части (сверху).
    pub panel_y: i32,
    /// Высота нижней части в пикселях.
    pub panel_h: i32,
    /// Ширина одного знакоместа (`width / 32`).
    pub cell: i32,
    /// Общий отступ: от краёв окна до обеих рамок и между рамками.
    pub margin: i32,
}

impl CockpitGeom {
    /// Экранная X-координата знакоместа `col` (0..32).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn col_x(&self, col: i32) -> i32 {
        self.cell * col
    }

    /// Экранная Y-координата знакоместной строки `row` (0..[`PANEL_ROWS`]).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn row_y(&self, row: i32) -> i32 {
        self.panel_y + self.panel_h * row / PANEL_ROWS
    }

    /// Внешняя граница главной рамки (вокруг области вида).
    ///
    /// Нижняя линия рамки касается верхней границы панели
    /// (`panel_y - 1`), как в оригинале, где панель продолжает кадр.
    /// Боковые поля и поле сверху равны [`CockpitGeom::margin`].
    pub fn main_frame(&self) -> Rect {
        let m = self.margin.max(LINE);
        Rect {
            x: m,
            y: m,
            w: (self.width - 2 * m).max(0),
            h: (self.panel_y - m).max(0),
        }
    }

    /// Внутренняя область вида — сюда рисуется 3D-сцена или заставка.
    pub fn view(&self) -> Rect {
        self.main_frame().inset(LINE)
    }

    /// Внешняя граница белой рамки нижней части.
    ///
    /// Отступ от краёв окна — тот же [`CockpitGeom::margin`], поэтому и
    /// зазор до главной рамки получается таким же.
    pub fn panel_frame(&self) -> Rect {
        let m = self.margin.max(LINE);
        let y = self.panel_y + m;
        Rect {
            x: m,
            y,
            w: (self.width - 2 * m).max(0),
            h: (self.height - m - y).max(0),
        }
    }

    /// Внутренняя область панели — без линий рамки.
    pub fn panel_inner(&self) -> Rect {
        self.panel_frame().inset(LINE)
    }

    /// Высота строки боковой колонки без горизонтального разделителя.
    ///
    /// Все семь строк одинаковой высоты (разница не больше пикселя), а
    /// остаток от деления распределяется по верхним строкам. Последняя
    /// строка упирается в нижнюю линию рамки панели, поэтому разделителя
    /// под ней нет.
    pub fn side_row(&self, row: i32) -> Rect {
        let inner = self.panel_inner();
        let row = row.clamp(0, PANEL_ROWS - 1);
        let available = (inner.h - (PANEL_ROWS - 1) * LINE).max(PANEL_ROWS);
        let base = available / PANEL_ROWS;
        let extra = available % PANEL_ROWS;
        let h = base + if row < extra { 1 } else { 0 };
        let y = inner.y + row * (base + LINE) + row.min(extra);
        Rect {
            x: inner.x,
            y,
            w: inner.w,
            h,
        }
    }

    /// Горизонтальный разделитель под строкой `row` (только боковые колонки).
    pub fn row_separator(&self, row: i32) -> Rect {
        let row = row.clamp(0, PANEL_ROWS - 2);
        let band = self.side_row(row);
        Rect {
            x: band.x,
            y: band.y + band.h,
            w: band.w,
            h: LINE,
        }
    }

    /// Ширина клетки под подпись/значок.
    ///
    /// Считается от высоты строки, а не от ширины панели, поэтому при смене
    /// соотношения сторон клетка с текстом сохраняет размер.
    pub fn label_width(&self) -> i32 {
        (self.side_row(0).h * LABEL_WIDTH_PER_ROW_PCT / 100).max(1)
    }

    /// Ширина длинной клетки-полосы.
    pub fn bar_width(&self) -> i32 {
        self.panel_inner().w * BAR_WIDTH_FRAC / 100
    }

    /// Клетка подписи/значка строки `row` (слева или справа).
    pub fn label_cell(&self, row: i32, right: bool) -> Rect {
        let inner = self.panel_inner();
        let band = self.side_row(row);
        let w = self.label_width();
        Rect {
            x: if right { inner.x + inner.w - w } else { inner.x },
            y: band.y,
            w,
            h: band.h,
        }
    }

    /// Длинная клетка-полоса строки `row` (слева или справа).
    pub fn bar_cell(&self, row: i32, right: bool) -> Rect {
        let inner = self.panel_inner();
        let band = self.side_row(row);
        let lw = self.label_width();
        let bw = self.bar_width();
        Rect {
            x: if right {
                inner.x + inner.w - lw - LINE - bw
            } else {
                inner.x + lw + LINE
            },
            y: band.y,
            w: bw,
            h: band.h,
        }
    }

    /// Ширина одной позиции запаса ракет.
    ///
    /// Позиция — это белый прямоугольник ракеты и промежуток до следующей:
    /// в оригинале иконка шириной в знакоместо чуть шире своей высоты,
    /// а между иконками остаётся цветной зазор. Поэтому шаг позиции равен
    /// высоте строки плюс [`LINE`].
    pub fn missile_slot_width(&self) -> i32 {
        (self.side_row(MISSILE_ROW).h + LINE).max(1)
    }

    /// Клетка запаса ракет — левая часть длинной клетки.
    ///
    /// Содержит до [`MISSILE_SLOTS`] фиксированных позиций; на узком окне
    /// ужимается по ширине полосы.
    pub fn missile_cell(&self, row: i32, right: bool) -> Rect {
        let bar = self.bar_cell(row, right);
        let w = (self.missile_slot_width() * MISSILE_SLOTS).min(bar.w);
        Rect {
            x: bar.x,
            y: bar.y,
            w,
            h: bar.h,
        }
    }

    /// Клетка режима — правая часть длинной клетки после ракет.
    ///
    /// Пока всегда чёрная; позже её цвет будет зависеть от режима.
    pub fn mode_cell(&self, row: i32, right: bool) -> Rect {
        let bar = self.bar_cell(row, right);
        let missiles = self.missile_cell(row, right);
        let x = missiles.x + missiles.w + LINE;
        Rect {
            x,
            y: bar.y,
            w: (bar.x + bar.w - x).max(0),
            h: bar.h,
        }
    }

    /// Большая центральная клетка на всю высоту панели.
    pub fn center_cell(&self) -> Rect {
        let inner = self.panel_inner();
        let side = self.label_width() + LINE + self.bar_width() + LINE;
        Rect {
            x: inner.x + side,
            y: inner.y,
            w: (inner.w - 2 * side).max(0),
            h: inner.h,
        }
    }

    /// Квадрат кружка в верхней части центральной клетки.
    ///
    /// `diameter` — желаемый диаметр; он ужимается, чтобы два кружка
    /// поместились в центральную клетку. Кружки прижаты к верхнему краю и
    /// разнесены к левому и правому краям, как в оригинале.
    fn top_circle(&self, right: bool, diameter: i32) -> Rect {
        let center = self.center_cell();
        if center.is_empty() {
            return Rect {
                x: center.x,
                y: center.y,
                w: 0,
                h: 0,
            };
        }
        let gap = (self.side_row(0).h / 2).max(LINE);
        let d = diameter.min(center.w / 3).min(center.h / 2).max(0);
        Rect {
            x: if right {
                center.x + center.w - gap - d
            } else {
                center.x + gap
            },
            y: center.y + gap,
            w: d,
            h: d,
        }
    }

    /// Кружок индикатора опасности (верхний левый в центральной клетке).
    pub fn danger_cell(&self) -> Rect {
        self.top_circle(false, (self.side_row(0).h * 2).max(1))
    }

    /// Кружок компаса (верхний правый в центральной клетке).
    pub fn compass_cell(&self) -> Rect {
        // В оригинале компас чуть больше индикатора опасности: 20x20 против
        // 16x16 «пикселей» ZX.
        self.top_circle(true, (self.side_row(0).h * 5 / 2).max(1))
    }

    /// X-координаты четырёх вертикальных разделителей таблицы.
    pub fn column_separators(&self) -> [i32; 4] {
        let inner = self.panel_inner();
        let lw = self.label_width();
        let bw = self.bar_width();
        [
            inner.x + lw,
            inner.x + lw + LINE + bw,
            inner.x + inner.w - lw - LINE - bw - LINE,
            inner.x + inner.w - lw - LINE,
        ]
    }
}

/// Разложение виртуальной сетки знакомест по окну `WxH`.
///
/// Панель прижимается к нижнему краю окна и занимает всю его ширину.
pub fn panel_geometry(w: u32, h: u32) -> CockpitGeom {
    let w = w as i32;
    let h = h as i32;
    let panel_h = (h * PANEL_FRAC / 100).clamp(48.min(h), h).max(1);
    let cell = (w / 32).max(1);
    // Один «шаг» отступа — четверть знакоместа: на большом окне обе рамки
    // заметно отодвинуты от краёв, на маленьком — хотя бы две толщины линии.
    let margin = (cell / 4).clamp(LINE * 2, 32);
    CockpitGeom {
        height: h,
        width: w,
        panel_y: h - panel_h,
        panel_h,
        cell,
        margin,
    }
}

/// Нативное оформление кокпита. Состояния нет — рамки статичны, а экраны
/// рисуют своё содержимое до вызова [`Cockpit::draw`].
pub struct Cockpit;

impl Cockpit {
    pub fn new() -> Self {
        Cockpit
    }

    /// Нарисовать кокпит: рамки, таблицу и текущие показания.
    ///
    /// `font` — знакогенератор ПЗУ ZX (коды 0x20..0x7F) для подписей,
    /// `style` — цвета линий и полос, `values` — снимок показаний на кадр.
    pub fn draw(
        &self,
        f: &mut Frame,
        font: &[u8],
        style: CockpitStyle,
        values: &CockpitValues,
    ) {
        let g = panel_geometry(f.width as u32, f.height as u32);
        self.draw_with_geom(f, &g, font, style, values);
    }

    /// Нарисовать кокпит по заранее вычисленной геометрии.
    ///
    /// Содержимое вида (космос, заставка) должно быть уже нарисовано:
    /// кокпит только заливает фон нижней части, рисует две рамки и таблицу
    /// с индикаторами в нижней части.
    pub fn draw_with_geom(
        &self,
        f: &mut Frame,
        g: &CockpitGeom,
        font: &[u8],
        style: CockpitStyle,
        values: &CockpitValues,
    ) {
        // Нижняя часть непрозрачна и закрывает всё, что под ней.
        if g.panel_h > 0 && g.width > 0 {
            f.fill_rect(
                0,
                g.panel_y,
                g.width as usize,
                g.panel_h as usize,
                BACKGROUND,
            );
        }
        // Главная рамка вокруг вида и отдельная рамка нижней части с
        // отступом от неё.
        stroke_rect(f, g.main_frame(), LINE, style.line);
        stroke_rect(f, g.panel_frame(), LINE, style.line);
        draw_chrome(f, g, font, style);
        draw_indicators(f, g, style, values);
        draw_danger(f, g, values);
        draw_compass(f, g, style, values);
    }
}

/// Нарисовать статическую часть панели: разделители и подписи.
fn draw_chrome(f: &mut Frame, g: &CockpitGeom, font: &[u8], style: CockpitStyle) {
    let inner = g.panel_inner();
    if inner.is_empty() {
        return;
    }
    // Горизонтальные разделители только в боковых колонках: центральная
    // клетка одна на все семь строк.
    let center = g.center_cell();
    for row in 0..PANEL_ROWS - 1 {
        let sep = g.row_separator(row);
        let left_w = (center.x - inner.x).max(0);
        if left_w > 0 {
            f.fill_rect(
                inner.x,
                sep.y,
                left_w as usize,
                sep.h as usize,
                style.line,
            );
        }
        let right_x = center.x + center.w;
        let right_w = (inner.x + inner.w - right_x).max(0);
        if right_w > 0 {
            f.fill_rect(
                right_x,
                sep.y,
                right_w as usize,
                sep.h as usize,
                style.line,
            );
        }
    }
    // Вертикальные разделители между всеми клетками таблицы.
    for x in g.column_separators() {
        f.fill_rect(x, inner.y, LINE as usize, inner.h as usize, style.line);
    }
    // Подписи/значки на чёрном фоне коротких клеток.
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            draw_indicator_label(f, g, row, right, style.line, font);
        }
    }
}

/// Нарисовать значения всех длинных клеток.
///
/// Функция самодостаточна: каждая клетка сначала заливается своим фоном,
/// поэтому старую шкалу не нужно стирать отдельно.
fn draw_indicators(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    for right in [false, true] {
        let cells = if right { &values.right } else { &values.left };
        for row in 0..PANEL_ROWS {
            let value = cells[row as usize];
            if !right && row == MISSILE_ROW {
                let missiles = match value {
                    Indicator::Missiles(n) => n,
                    _ => 0,
                };
                draw_missiles(f, g, style, missiles);
                continue;
            }
            draw_indicator(f, g, row, right, value, style);
        }
    }
}

/// Нарисовать индикатор опасности: залитый круг цвета `IndDanger`.
///
/// В оригинале это 2x2 знакоместа с кругом радиусом около 7 пикселей ZX.
/// Мигающий красный (`0x63`) в выключенной фазе не рисуется — фон панели
/// уже залит чёрным.
fn draw_danger(f: &mut Frame, g: &CockpitGeom, values: &CockpitValues) {
    let cell = g.danger_cell();
    if cell.is_empty() {
        return;
    }
    let Some(color) = values.danger.color(values.blink) else {
        return;
    };
    let d = cell.w.min(cell.h);
    let radius = (d / 2 - (d / 16).max(1)).max(0);
    f.disc(cell.x + cell.w / 2, cell.y + cell.h / 2, radius, color);
}

/// Растр указателя компаса: заполненный ромб 4x4.
const COMPASS_POINTER_FILLED: [u8; 4] = [0x18, 0x3C, 0x3C, 0x18];
/// Растр указателя компаса: полый квадрат 4x4.
const COMPASS_POINTER_EMPTY: [u8; 4] = [0x3C, 0x24, 0x24, 0x3C];

/// Нарисовать компас: окружность и указатель на цель.
///
/// Размеры повторяют оригинал: круг 20x20 «пикселей» ZX и указатель 4x4,
/// смещённый от центра на `-8..=8`. При увеличении окна один пиксель ZX
/// превращается в [`COMPASS_POINTER_MAX`]-ю часть диаметра.
fn draw_compass(
    f: &mut Frame,
    g: &CockpitGeom,
    style: CockpitStyle,
    values: &CockpitValues,
) {
    let cell = g.compass_cell();
    if cell.is_empty() {
        return;
    }
    let d = cell.w.min(cell.h);
    if d <= 0 {
        return;
    }
    let unit = (d / 20).max(1);
    let cx = cell.x + cell.w / 2;
    let cy = cell.y + cell.h / 2;
    let radius = (d / 2 - 1).max(1);
    // Толщина кольца — один пиксель оригинала.
    for k in 0..unit {
        f.circle(cx, cy, (radius - k).max(0), style.line);
    }
    // Указатель: положительный `y` — вверх, как `CompasPointerCrdY`.
    let x0 = cx + values.compass.x as i32 * unit - 2 * unit;
    let y0 = cy - values.compass.y as i32 * unit - 2 * unit;
    let sprite = if values.compass.filled {
        &COMPASS_POINTER_FILLED
    } else {
        &COMPASS_POINTER_EMPTY
    };
    for (row, bits) in sprite.iter().enumerate() {
        for col in 0..4 {
            if bits & (0x20 >> col) != 0 {
                f.fill_rect(
                    x0 + col * unit,
                    y0 + row as i32 * unit,
                    unit as usize,
                    unit as usize,
                    style.line,
                );
            }
        }
    }
}

/// Нарисовать значение одной длинной клетки.
fn draw_indicator(
    f: &mut Frame,
    g: &CockpitGeom,
    row: i32,
    right: bool,
    value: Indicator,
    style: CockpitStyle,
) {
    let cell = g.bar_cell(row, right);
    if cell.is_empty() {
        return;
    }
    match value {
        Indicator::Gauge(v) => draw_gauge(f, cell, style, v),
        Indicator::Deflection(v) => draw_deflection(f, cell, style, v),
        Indicator::Missiles(_) => {
            // Запас ракет живёт только в нижней левой клетке; в остальных
            // местах показываем пустую клетку.
            f.fill_rect(
                cell.x,
                cell.y,
                cell.w as usize,
                cell.h as usize,
                BACKGROUND,
            );
        }
        Indicator::Empty => {
            f.fill_rect(
                cell.x,
                cell.y,
                cell.w as usize,
                cell.h as usize,
                BACKGROUND,
            );
        }
    }
}

/// Шкала: цветной фон и белая заливка слева направо, как в оригинале.
///
/// Заливка не касается верхней и нижней границы клетки: сверху и снизу
/// остаётся цветной отступ толщиной [`LINE`] — столько же, сколько у рамки.
fn draw_gauge(f: &mut Frame, cell: Rect, style: CockpitStyle, value: u8) {
    f.fill_rect(
        cell.x,
        cell.y,
        cell.w as usize,
        cell.h as usize,
        style.bar,
    );
    let inner = indicator_inner(cell);
    if inner.is_empty() {
        return;
    }
    let filled = inner.w * value.min(GAUGE_MAX) as i32 / GAUGE_MAX as i32;
    if filled > 0 {
        f.fill_rect(
            inner.x,
            inner.y,
            filled.min(inner.w) as usize,
            inner.h as usize,
            style.line,
        );
    }
}

/// Область значения внутри клетки: без цветного отступа [`LINE`] сверху и
/// снизу. По горизонтали значение занимает всю ширину клетки, как в
/// оригинале.
fn indicator_inner(cell: Rect) -> Rect {
    let gap = LINE.min((cell.h - 1).max(0) / 2);
    Rect {
        x: cell.x,
        y: cell.y + gap,
        w: cell.w,
        h: (cell.h - 2 * gap).max(0),
    }
}

/// Индикатор влево/вправо: маркер на цветном фоне, 0 — по центру.
fn draw_deflection(f: &mut Frame, cell: Rect, style: CockpitStyle, value: i8) {
    f.fill_rect(
        cell.x,
        cell.y,
        cell.w as usize,
        cell.h as usize,
        style.bar,
    );
    let inner = indicator_inner(cell);
    if inner.is_empty() {
        return;
    }
    let marker_w = (LINE * 2).clamp(1, inner.w.max(1));
    let half = inner.w / 2;
    let max_shift = (half - marker_w / 2).max(0);
    let shift = value.clamp(-DEFLECTION_MAX, DEFLECTION_MAX) as i32 * max_shift
        / DEFLECTION_MAX as i32;
    let x = (inner.x + half + shift - marker_w / 2)
        .clamp(inner.x, inner.x + inner.w - marker_w);
    f.fill_rect(
        x,
        inner.y,
        marker_w as usize,
        inner.h as usize,
        style.line,
    );
}

/// Прямоугольник одной ракеты внутри ракетной клетки.
///
/// Высота совпадает с заливкой индикаторов, ширина чуть больше высоты на
/// толщину линии. Левый отступ и промежутки между ракетами остаются
/// цветными — это фон клетки.
fn missile_icon_rect(cell: Rect, pitch: i32, slot: i32) -> Rect {
    let inner = indicator_inner(cell);
    let gap = (inner.y - cell.y).max(0);
    let icon_h = inner.h;
    Rect {
        x: cell.x + gap + slot * pitch,
        y: inner.y,
        w: (icon_h + LINE).max(1),
        h: icon_h,
    }
}

/// Нарисовать запас ракет и клетку режима в нижней левой длинной клетке.
///
/// Фон ракетной части всегда цвет полосы (в оригинале — красный). Ракеты —
/// белые прямоугольники с цветной серединой: максимум четыре, слева
/// направо. Если ракет меньше, правые прямоугольники не рисуются, но фон
/// остаётся цветным. Прямоугольник чуть шире своей высоты, а по высоте
/// совпадает с заливкой индикаторов. Справа после разделителя — клетка
/// режима; пока она всегда чёрная.
fn draw_missiles(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, missiles: u8) {
    let left = g.bar_cell(MISSILE_ROW, false);
    if left.is_empty() {
        return;
    }
    let cell = g.missile_cell(MISSILE_ROW, false);
    if cell.is_empty() {
        return;
    }
    f.fill_rect(
        cell.x,
        cell.y,
        cell.w as usize,
        cell.h as usize,
        style.bar,
    );
    let filled = (missiles as i32).clamp(0, MISSILE_SLOTS);
    let pitch = g.missile_slot_width();
    for slot in 0..filled {
        let icon = missile_icon_rect(cell, pitch, slot);
        if icon.x + icon.w > cell.x + cell.w {
            break;
        }
        stroke_rect(f, icon, LINE, style.line);
    }
    // Разделитель между ракетами и клеткой режима.
    let sep_x = cell.x + cell.w;
    let sep_w = (left.x + left.w - sep_x).clamp(0, LINE);
    if sep_w > 0 {
        f.fill_rect(
            sep_x,
            left.y,
            sep_w as usize,
            left.h as usize,
            style.line,
        );
    }
    // Клетка режима: по умолчанию чёрная.
    let mode = g.mode_cell(MISSILE_ROW, false);
    if !mode.is_empty() {
        f.fill_rect(
            mode.x,
            mode.y,
            mode.w as usize,
            mode.h as usize,
            BACKGROUND,
        );
    }
}

/// Подпись индикатора строки `row` (слева или справа).
///
/// `None` означает, что в клетке вместо текста рисуется значок скорости.
fn indicator_label(row: i32, right: bool) -> Option<&'static str> {
    let table = if right { &RIGHT_LABELS } else { &LEFT_LABELS };
    table.get(row as usize).copied().flatten()
}

/// Нарисовать подпись индикатора (или значок скорости) по центру короткой клетки.
fn draw_indicator_label(
    f: &mut Frame,
    g: &CockpitGeom,
    row: i32,
    right: bool,
    line_color: u32,
    font: &[u8],
) {
    let area = g.label_cell(row, right);
    if area.is_empty() {
        return;
    }
    let Some(label) = indicator_label(row, right) else {
        draw_speed_icon(f, area, line_color);
        return;
    };
    if font.is_empty() {
        return;
    }
    let chars = label.len() as i32;
    let scale = (area.h / 8).min(area.w / (8 * chars).max(1)).max(1);
    let text_w = 8 * scale * chars;
    let text_h = 8 * scale;
    let x = area.x + (area.w - text_w) / 2;
    let y = area.y + (area.h - text_h) / 2;
    // На очень маленьких окнах подпись может не поместиться в клетку —
    // обрезаем её, чтобы не залезть на соседние строки и полосы.
    f.set_clip(area.x, area.y, area.w.max(0) as usize, area.h.max(0) as usize);
    f.draw_text(font, x, y, label, line_color, scale as usize);
    f.clear_clip();
}

/// Нарисовать значок скорости по центру короткой клетки.
///
/// Масштаб тот же, что у подписей: один пиксель растра — [`LINE`] и более
/// пикселей окна, в зависимости от высоты строки.
fn draw_speed_icon(f: &mut Frame, area: Rect, color: u32) {
    const W: i32 = 12;
    const H: i32 = SPEED_ICON.len() as i32;
    let scale = (area.h / 8).min(area.w / W).max(1);
    let w = W * scale;
    let h = H * scale;
    let x = area.x + (area.w - w) / 2;
    let y = area.y + (area.h - h) / 2;
    f.set_clip(area.x, area.y, area.w.max(0) as usize, area.h.max(0) as usize);
    for (row, bits) in SPEED_ICON.iter().enumerate() {
        for col in 0..W {
            if bits & (1 << (W - 1 - col)) != 0 {
                f.fill_rect(
                    x + col * scale,
                    y + row as i32 * scale,
                    scale as usize,
                    scale as usize,
                    color,
                );
            }
        }
    }
    f.clear_clip();
}

impl Default for Cockpit {
    fn default() -> Self {
        Self::new()
    }
}

/// Обвести прямоугольник `r` рамкой толщиной `t`.
fn stroke_rect(f: &mut Frame, r: Rect, t: i32, color: u32) {
    if r.is_empty() {
        return;
    }
    let t = t.min(r.w).min(r.h).max(1);
    // Верхняя и нижняя полосы.
    f.fill_rect(r.x, r.y, r.w as usize, t as usize, color);
    f.fill_rect(r.x, r.y + r.h - t, r.w as usize, t as usize, color);
    // Боковые полосы между ними.
    let mid_h = (r.h - 2 * t).max(0) as usize;
    if mid_h > 0 {
        f.fill_rect(r.x, r.y + t, t as usize, mid_h, color);
        f.fill_rect(r.x + r.w - t, r.y + t, t as usize, mid_h, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Шрифт-заглушка: все знаки — сплошной прямоугольник.
    fn font() -> Vec<u8> {
        vec![0xFFu8; 0x300]
    }

    /// Значения для тестов: пустые шкалы и полный запас ракет.
    fn values(missiles: u8) -> CockpitValues {
        CockpitValues {
            left: [
                Indicator::Gauge(0),
                Indicator::Gauge(0),
                Indicator::Gauge(0),
                Indicator::Gauge(0),
                Indicator::Gauge(0),
                Indicator::Gauge(0),
                Indicator::Missiles(missiles),
            ],
            right: [Indicator::Gauge(0); PANEL_ROWS as usize],
            danger: Danger::Off,
            compass: Compass::centered(),
            blink: true,
        }
    }

    fn draw(w: usize, h: usize) -> (Frame, CockpitGeom) {
        let mut f = Frame::new(w, h);
        let font = font();
        Cockpit::new().draw(
            &mut f,
            &font,
            CockpitStyle::COBRA_MK3,
            &values(MISSILE_SLOTS as u8),
        );
        (f, panel_geometry(w as u32, h as u32))
    }

    /// Кокпит рисует главную рамку и рамку нижней части.
    #[test]
    fn cockpit_draws_main_and_panel_frames() {
        let (f, g) = draw(1280, 960);
        let main = g.main_frame();
        let panel = g.panel_frame();
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

        assert_eq!(px(main.x, main.y), FRAME_COLOR, "главная рамка не нарисована");
        assert_eq!(
            px(main.x + main.w / 2, main.bottom()),
            FRAME_COLOR,
            "нижняя линия главной рамки не нарисована"
        );
        assert_eq!(
            px(panel.x, panel.y),
            FRAME_COLOR,
            "рамка нижней части не нарисована"
        );
        assert_eq!(
            px(panel.right(), panel.bottom()),
            FRAME_COLOR,
            "правый нижний угол нижней рамки не нарисован"
        );
    }

    /// Все отступы одинаковые: от краёв окна и между рамками.
    #[test]
    fn all_margins_are_equal() {
        let g = panel_geometry(1280, 960);
        let main = g.main_frame();
        let panel = g.panel_frame();
        // Отступ от краёв окна у обеих рамок.
        assert_eq!(main.x, g.margin, "главная рамка не на общем отступе слева");
        assert_eq!(main.y, g.margin, "главная рамка не на общем отступе сверху");
        assert_eq!(panel.x, g.margin, "нижняя рамка не на общем отступе слева");
        assert_eq!(
            g.width - 1 - panel.right(),
            g.margin,
            "нижняя рамка не на общем отступе справа"
        );
        assert_eq!(
            g.height - 1 - panel.bottom(),
            g.margin,
            "нижняя рамка не на общем отступе снизу"
        );
        // Зазор между главной рамкой и нижней — тот же отступ.
        let gap_rows = panel.y - main.bottom() - 1;
        assert_eq!(gap_rows, g.margin, "зазор между рамками не равен отступу");
        // И этот зазор действительно чёрный, а не общая линия.
        let (f, _) = draw(1280, 960);
        let mid_x = g.width / 2;
        for y in main.bottom() + 1..panel.y {
            assert_eq!(
                f.pixels()[y as usize * g.width as usize + mid_x as usize],
                BACKGROUND,
                "зазор между рамками не чёрный в строке {y}"
            );
        }
    }

    /// В панели по семь строк слева и справа: длинные клетки залиты, между
    /// строками — белые разделители, у нижней строки разделитель — сама рамка.
    #[test]
    fn panel_has_seven_indicator_rows_on_both_sides() {
        let (f, g) = draw(1280, 960);
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
        for right in [false, true] {
            for row in 0..PANEL_ROWS {
                let bar = g.bar_cell(row, right);
                assert!(!bar.is_empty(), "пустая клетка {row} (right={right})");
                // Нижняя левая клетка разделена: центр полосы попадает в
                // чёрную клетку режима, поэтому проверяем позицию ракеты.
                let (px_x, px_y) = if !right && row == MISSILE_ROW {
                    let slot = g.missile_cell(row, right);
                    (slot.x + slot.w / 2, slot.y + slot.h / 2)
                } else {
                    (bar.x + bar.w / 2, bar.y + bar.h / 2)
                };
                assert_eq!(
                    px(px_x, px_y),
                    BAR_RED,
                    "клетка {row} (right={right}) не залита"
                );
                // Между строками — белая линия шириной с рамку.
                if row + 1 < PANEL_ROWS {
                    let sep = g.row_separator(row);
                    assert_eq!(
                        px(bar.x + bar.w / 2, sep.y + LINE / 2),
                        FRAME_COLOR,
                        "нет линии под строкой {row} (right={right})"
                    );
                }
            }
        }
        // Нижняя строка опирается на нижнюю линию рамки панели.
        let inner = g.panel_inner();
        let last = g.side_row(PANEL_ROWS - 1);
        assert_eq!(
            last.y + last.h,
            inner.y + inner.h,
            "нижняя строка не доходит до рамки панели"
        );
    }

    /// Все семь строк боковых колонок одинаковой высоты, включая нижнюю.
    #[test]
    fn side_rows_have_equal_height() {
        let g = panel_geometry(1280, 960);
        let heights: Vec<i32> = (0..PANEL_ROWS).map(|r| g.side_row(r).h).collect();
        let min = *heights.iter().min().unwrap();
        let max = *heights.iter().max().unwrap();
        assert!(max - min <= 1, "строки разной высоты: {heights:?}");
        let inner = g.panel_inner();
        assert_eq!(g.side_row(0).y, inner.y, "первая строка не сверху панели");
        let last = g.side_row(PANEL_ROWS - 1);
        assert_eq!(
            last.y + last.h,
            inner.y + inner.h,
            "нижняя строка не доходит до рамки панели"
        );
        // Разделители стоят ровно между строками.
        for row in 0..PANEL_ROWS - 1 {
            let sep = g.row_separator(row);
            let next = g.side_row(row + 1);
            assert_eq!(sep.y, g.side_row(row).y + g.side_row(row).h);
            assert_eq!(sep.y + sep.h, next.y, "разрыв перед строкой {}", row + 1);
        }
    }

    /// Таблица с рамками: вертикальные разделители между всеми клетками,
    /// горизонтальные — только в боковых колонках, центр — одна большая клетка.
    #[test]
    fn panel_is_a_table_with_borders() {
        let (f, g) = draw(1280, 960);
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
        let inner = g.panel_inner();
        let center = g.center_cell();
        assert!(!center.is_empty(), "центральная клетка пуста");
        assert_eq!(center.y, inner.y, "центр не начинается сверху панели");
        assert_eq!(
            center.y + center.h,
            inner.y + inner.h,
            "центр не занимает всю высоту панели"
        );
        // Вертикальные линии отделяют подпись, полосу и центр.
        for x in g.column_separators() {
            assert_eq!(
                px(x + LINE / 2, center.y + center.h / 2),
                FRAME_COLOR,
                "нет вертикального разделителя в x={x}"
            );
        }
        // В центре горизонтальных линий нет.
        for row in 0..PANEL_ROWS - 1 {
            let sep = g.row_separator(row);
            assert_eq!(
                px(center.x + center.w / 2, sep.y + LINE / 2),
                BACKGROUND,
                "горизонтальная линия пересекает центр (строка {row})"
            );
        }
    }

    /// Ширина и высота клеток задаются долями панели и легко меняются.
    #[test]
    fn indicator_size_follows_panel_fractions() {
        let g = panel_geometry(1280, 960);
        let inner = g.panel_inner();
        let bar = g.bar_cell(0, false);
        let label = g.label_cell(0, false);
        assert_eq!(bar.w, inner.w * BAR_WIDTH_FRAC / 100);
        assert_eq!(label.w, g.side_row(0).h * LABEL_WIDTH_PER_ROW_PCT / 100);
        assert!(bar.h > 0 && bar.h == g.side_row(0).h);
        // Клетки разделены вертикальной линией.
        assert_eq!(bar.x, label.x + label.w + LINE);
        // Справа — зеркально: подпись правее полосы.
        let rbar = g.bar_cell(0, true);
        let rlabel = g.label_cell(0, true);
        assert_eq!(rlabel.x, rbar.x + rbar.w + LINE);
        assert_eq!(rlabel.x + rlabel.w, inner.x + inner.w);
    }

    /// Нижняя левая клетка: четыре позиции ракет фиксированной ширины,
    /// разделитель и клетка режима справа.
    #[test]
    fn missile_cell_has_four_fixed_slots_and_mode_cell() {
        let g = panel_geometry(1280, 960);
        let bar = g.bar_cell(MISSILE_ROW, false);
        let cell = g.missile_cell(MISSILE_ROW, false);
        let slot = g.missile_slot_width();
        assert_eq!(
            slot,
            g.side_row(MISSILE_ROW).h + LINE,
            "неверный шаг позиции ракеты"
        );
        assert_eq!(cell.w, slot * MISSILE_SLOTS, "не четыре позиции");
        assert_eq!(cell.x, bar.x);
        assert_eq!(cell.y, bar.y);
        assert_eq!(cell.h, bar.h);
        let mode = g.mode_cell(MISSILE_ROW, false);
        assert_eq!(
            mode.x,
            cell.x + cell.w + LINE,
            "клетка режима не сразу за ракетами"
        );
        assert_eq!(
            mode.x + mode.w,
            bar.x + bar.w,
            "клетка режима не доходит до края полосы"
        );
        assert!(mode.w > 0, "клетка режима пуста");
    }

    /// Ракеты — белые прямоугольники на красном фоне: максимум четыре,
    /// слева направо. При уменьшении запаса правые пропадают, а фон остаётся.
    #[test]
    fn missiles_are_white_rectangles_on_red_background() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let cell = g.missile_cell(MISSILE_ROW, false);
        let pitch = g.missile_slot_width();
        let mode = g.mode_cell(MISSILE_ROW, false);
        for count in 0..=MISSILE_SLOTS as u8 + 1 {
            let mut f = Frame::new(1280, 960);
            Cockpit::new().draw(
                &mut f,
                &font,
                CockpitStyle::COBRA_MK3,
                &values(count),
            );
            let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
            let filled = (count as i32).min(MISSILE_SLOTS);
            let mid_y = cell.y + cell.h / 2;
            // Фон ракетной части красный, включая пустые позиции.
            assert_eq!(px(cell.x, mid_y), BAR_RED, "нет фона слева от ракет");
            assert_eq!(
                px(cell.x + cell.w - 1, mid_y),
                BAR_RED,
                "нет фона справа от ракет"
            );
            for i in 0..MISSILE_SLOTS {
                let icon = missile_icon_rect(cell, pitch, i);
                let whites = (icon.x..icon.x + icon.w)
                    .flat_map(|x| (icon.y..icon.y + icon.h).map(move |y| (x, y)))
                    .filter(|&(x, y)| px(x, y) == FRAME_COLOR)
                    .count();
                if i < filled {
                    assert!(icon.w > icon.h, "ракета не шире своей высоты");
                    assert_eq!(
                        icon.h,
                        cell.h - 2 * LINE,
                        "высота ракеты не как у индикатора"
                    );
                    assert!(whites > 0, "ракета {i} не нарисована");
                    assert_eq!(
                        px(icon.x, icon.y),
                        FRAME_COLOR,
                        "у ракеты {i} нет белой рамки"
                    );
                    assert_eq!(
                        px(icon.x + icon.w / 2, icon.y + icon.h / 2),
                        BAR_RED,
                        "середина ракеты {i} не красная"
                    );
                } else {
                    assert_eq!(whites, 0, "лишняя ракета {i} при запасе {count}");
                }
            }
            // Между ракетами и режимом — белый разделитель.
            assert_eq!(
                px(cell.x + cell.w + LINE / 2, mid_y),
                FRAME_COLOR,
                "нет разделителя перед клеткой режима"
            );
            // Клетка режима чёрная по всей площади.
            for y in mode.y..mode.y + mode.h {
                for x in mode.x..mode.x + mode.w {
                    assert_eq!(
                        px(x, y),
                        BACKGROUND,
                        "клетка режима не чёрная ({x},{y}), ракет {count}"
                    );
                }
            }
        }
    }

    /// Шкала заполняется слева направо: пустая — весь фон полосы, полная —
    /// вся ширина клетки.
    #[test]
    fn gauge_fills_bar_from_left() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let cell = g.bar_cell(0, false);
        let y = cell.y + cell.h / 2;
        let sample = |f: &Frame, x: i32| f.pixels()[y as usize * 1280 + x as usize];

        let mut v = values(MISSILE_SLOTS as u8);
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        assert_eq!(sample(&f, cell.x + cell.w / 2), BAR_RED, "0% не пустая");

        v.left[0] = Indicator::Gauge(GAUGE_MAX / 2);
        f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        assert_eq!(sample(&f, cell.x + cell.w / 4), FRAME_COLOR, "нет заливки 50%");
        assert_eq!(
            sample(&f, cell.x + cell.w * 3 / 4),
            BAR_RED,
            "заливка 50% ушла дальше середины"
        );

        v.left[0] = Indicator::Gauge(GAUGE_MAX);
        f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        assert_eq!(
            sample(&f, cell.x + cell.w - 1),
            FRAME_COLOR,
            "100% не заполняет клетку"
        );
    }

    /// Значения не касаются верхней и нижней границы клетки: сверху и снизу
    /// остаётся цветной отступ толщиной [`LINE`], как у рамки.
    #[test]
    fn indicators_keep_gap_from_cell_borders() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let mut v = values(MISSILE_SLOTS as u8);
        v.left[0] = Indicator::Gauge(GAUGE_MAX);
        v.right[1] = Indicator::Deflection(0);
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

        let gauge = g.bar_cell(0, false);
        assert!(gauge.h > 2 * LINE, "клетка слишком низкая для отступа");
        let gx = gauge.x + gauge.w / 2;
        for d in 0..LINE {
            assert_eq!(px(gx, gauge.y + d), BAR_RED, "нет отступа сверху шкалы");
            assert_eq!(
                px(gx, gauge.y + gauge.h - 1 - d),
                BAR_RED,
                "нет отступа снизу шкалы"
            );
        }
        assert_eq!(
            px(gx, gauge.y + LINE),
            FRAME_COLOR,
            "заливка не начинается после отступа"
        );

        let mark = g.bar_cell(1, true);
        let mx = mark.x + mark.w / 2;
        for d in 0..LINE {
            assert_eq!(px(mx, mark.y + d), BAR_RED, "нет отступа сверху маркера");
            assert_eq!(
                px(mx, mark.y + mark.h - 1 - d),
                BAR_RED,
                "нет отступа снизу маркера"
            );
        }
        assert_eq!(
            px(mx, mark.y + LINE),
            FRAME_COLOR,
            "маркер не начинается после отступа"
        );
        assert_eq!(
            px(mx, mark.y + mark.h - 1 - LINE),
            FRAME_COLOR,
            "маркер не заканчивается до отступа"
        );
    }

    /// Индикатор влево/вправо ставит маркер по центру или сдвигает его.
    #[test]
    fn deflection_marker_moves_left_and_right() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let cell = g.bar_cell(1, true);
        let y = cell.y + cell.h / 2;
        let marker_center = |f: &Frame| {
            let xs: Vec<i32> = (cell.x..cell.x + cell.w)
                .filter(|&x| f.pixels()[y as usize * 1280 + x as usize] == FRAME_COLOR)
                .collect();
            assert!(!xs.is_empty(), "маркер не нарисован");
            (xs[0] + xs[xs.len() - 1]) / 2
        };
        let draw = |value: i8| {
            let mut v = values(MISSILE_SLOTS as u8);
            v.right[1] = Indicator::Deflection(value);
            let mut f = Frame::new(1280, 960);
            Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
            f
        };
        let center = cell.x + cell.w / 2;
        let neutral = draw(0);
        assert!(
            (marker_center(&neutral) - center).abs() <= 1,
            "нейтральное положение не по центру"
        );
        assert!(
            marker_center(&draw(-DEFLECTION_MAX)) < center,
            "маркер не ушёл влево"
        );
        assert!(
            marker_center(&draw(DEFLECTION_MAX)) > center,
            "маркер не ушёл вправо"
        );
    }

    /// Раскладка читает все значения из памяти и ограничивает их диапазоны.
    #[test]
    fn layout_reads_all_indicators_from_memory() {
        let l = KLADOV_COCKPIT;
        let mut mem = vec![0u8; 0x10000];
        mem[l.shield_front as usize] = 10;
        mem[l.shield_rear as usize] = 20;
        mem[l.fuel as usize] = 30;
        mem[l.temperature as usize] = 40;
        mem[l.laser_temp as usize] = 50;
        mem[l.altitude as usize] = 0xFF; // больше максимума — обрезается
        mem[l.speed as usize] = 65;
        mem[l.rotate_lr as usize] = 0x15;
        mem[l.rotate_du as usize] = 0x10;
        mem[l.shield_unit1 as usize] = 0xA0;
        mem[l.shield_unit1 as usize + 2] = 0x80;
        mem[l.shield_unit1 as usize + 4] = 0x40;
        mem[l.shield_unit1 as usize + 6] = 0;
        mem[l.missiles as usize] = 9; // больше четырёх — обрезается
        let v = l.read(&mem);
        assert_eq!(
            v.left,
            [
                Indicator::Gauge(10),
                Indicator::Gauge(20),
                Indicator::Gauge(30),
                Indicator::Gauge(40),
                Indicator::Gauge(50),
                Indicator::Gauge(GAUGE_MAX),
                Indicator::Missiles(MISSILE_SLOTS as u8),
            ]
        );
        assert_eq!(
            v.right,
            [
                Indicator::Gauge(65),
                Indicator::Deflection(0),
                Indicator::Deflection(-5),
                Indicator::Gauge(0xA0),
                Indicator::Gauge(0x80),
                Indicator::Gauge(0x40),
                Indicator::Gauge(0),
            ]
        );
    }

    /// Верхние кружки стоят в центральной клетке: опасность слева, компас
    /// справа, оба прижаты к верхнему краю.
    #[test]
    fn danger_and_compass_are_in_top_corners_of_center() {
        let g = panel_geometry(1280, 960);
        let center = g.center_cell();
        let danger = g.danger_cell();
        let compass = g.compass_cell();
        for (name, cell) in [("опасность", danger), ("компас", compass)] {
            assert!(!cell.is_empty(), "{name}: пустая клетка");
            assert_eq!(cell.w, cell.h, "{name}: клетка не квадратная");
            assert!(cell.x >= center.x, "{name}: левее центра");
            assert!(cell.x + cell.w <= center.x + center.w, "{name}: правее центра");
            assert!(cell.y >= center.y, "{name}: выше центра");
        }
        assert!(
            danger.x + danger.w < compass.x,
            "кружки перекрываются или стоят не по краям"
        );
    }

    /// Индикатор опасности — залитый круг цвета `IndDanger`; мигающий
    /// красный пропадает в выключенной фазе.
    #[test]
    fn danger_indicator_uses_original_colors() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let cell = g.danger_cell();
        let sample = |f: &Frame| {
            f.pixels()[(cell.y + cell.h / 2) as usize * 1280
                + (cell.x + cell.w / 2) as usize]
        };
        let draw = |danger: Danger, blink: bool| {
            let mut v = values(MISSILE_SLOTS as u8);
            v.danger = danger;
            v.blink = blink;
            let mut f = Frame::new(1280, 960);
            Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
            f
        };
        assert_eq!(sample(&draw(Danger::Off, true)), BACKGROUND);
        assert_eq!(sample(&draw(Danger::Green, true)), DANGER_GREEN);
        assert_eq!(sample(&draw(Danger::Yellow, true)), DANGER_YELLOW);
        assert_eq!(sample(&draw(Danger::Red, true)), DANGER_COLOR);
        assert_eq!(sample(&draw(Danger::BlinkingRed, true)), DANGER_COLOR);
        assert_eq!(
            sample(&draw(Danger::BlinkingRed, false)),
            BACKGROUND,
            "мигающий красный не погас"
        );
    }

    /// Компас: кольцо и указатель, который смещается по координатам цели.
    #[test]
    fn compass_ring_and_pointer_follow_target() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let cell = g.compass_cell();
        let unit = (cell.w.min(cell.h) / 20).max(1);
        let cx = cell.x + cell.w / 2;
        let cy = cell.y + cell.h / 2;
        let radius = (cell.w.min(cell.h) / 2 - 1).max(1);
        let draw = |compass: Compass| {
            let mut v = values(MISSILE_SLOTS as u8);
            v.compass = compass;
            let mut f = Frame::new(1280, 960);
            Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
            f
        };
        let px = |f: &Frame, x: i32, y: i32| {
            f.pixels()[y as usize * 1280 + x as usize]
        };

        // Кольцо нарисовано по краям кружка.
        let ring = draw(Compass::centered());
        assert_eq!(px(&ring, cx + radius, cy), FRAME_COLOR, "нет кольца");
        assert_eq!(px(&ring, cx - radius, cy), FRAME_COLOR, "нет кольца");

        // Заполненный указатель: в центре белый ромб.
        let filled = draw(Compass::centered());
        assert_eq!(
            px(&filled, cx, cy),
            FRAME_COLOR,
            "заполненный указатель не нарисован"
        );
        // Полый указатель: в центре дырка.
        let empty = draw(Compass {
            x: 0,
            y: 0,
            filled: false,
        });
        assert_eq!(
            px(&empty, cx, cy),
            BACKGROUND,
            "полый указатель не имеет дырки"
        );

        // Смещение вправо/влево и вверх.
        let right = draw(Compass {
            x: 4,
            y: 0,
            filled: true,
        });
        assert_eq!(px(&right, cx + 4 * unit, cy), FRAME_COLOR);
        assert_eq!(
            px(&right, cx - 4 * unit, cy),
            BACKGROUND,
            "указатель не ушёл вправо"
        );
        let left = draw(Compass {
            x: -4,
            y: 0,
            filled: true,
        });
        assert_eq!(px(&left, cx - 4 * unit, cy), FRAME_COLOR);
        let up = draw(Compass {
            x: 0,
            y: 4,
            filled: true,
        });
        assert_eq!(
            px(&up, cx, cy - 4 * unit),
            FRAME_COLOR,
            "положительный y должен уводить указатель вверх"
        );
        assert_eq!(px(&up, cx, cy + 4 * unit), BACKGROUND);
    }

    /// Компас читает цель по логике оригинала: вне системы — фиктивные
    /// координаты, рядом с планетой — станция, иначе планета.
    #[test]
    fn compass_reads_target_from_memory() {
        let l = KLADOV_COCKPIT;
        let mut mem = vec![0u8; 0x10000];
        let crd = |mem: &mut [u8], addr: u16, x: i32, y: i32, z: i32| {
            let put = |mem: &mut [u8], at: u16, v: i32| {
                let sign = if v < 0 { 0x80 } else { 0 };
                let mag = v.unsigned_abs();
                mem[at as usize] = mag as u8;
                mem[at as usize + 1] = (mag >> 8) as u8;
                mem[at as usize + 2] = ((mag >> 16) as u8 & 0x7F) | sign;
            };
            put(mem, addr, x);
            put(mem, addr + 3, y);
            put(mem, addr + 6, z);
        };

        // Планета впереди справа: указатель сдвинут вправо и заполнен.
        // J-драйв у станции недоступен, поэтому цель — сама планета.
        mem[l.j_drive_coriolis as usize] = 0xFF;
        crd(&mut mem, l.planet_crd, 1000, 0, 1000);
        let v = l.read(&mem);
        assert_eq!(v.compass.x, 6);
        assert_eq!(v.compass.y, 0);
        assert!(v.compass.filled);

        // Планета за спиной: указатель полый.
        crd(&mut mem, l.planet_crd, 1000, 0, -1000);
        assert!(!l.read(&mem).compass.filled);

        // Рядом с планетой цель — станция.
        mem[l.in_free_space as usize] = 0;
        mem[l.j_drive_coriolis as usize] = 0;
        mem[l.planet_crd as usize + 2] = 0;
        mem[l.planet_crd as usize + 5] = 0;
        mem[l.planet_crd as usize + 8] = 0;
        crd(&mut mem, l.coriolis_slot, 0, 1000, 1000);
        let v = l.read(&mem);
        assert_eq!(v.compass.y, 6, "компас не переключился на станцию");

        // Вне системы — фиктивные координаты, а не планета.
        mem[l.in_free_space as usize] = 1;
        crd(&mut mem, l.fake_planet_crd, 0, -1000, 1000);
        let v = l.read(&mem);
        assert_eq!(v.compass.x, 0);
        assert_eq!(v.compass.y, -6);

        // `IndDanger` разбирается в цвет индикатора.
        mem[l.ind_danger as usize] = 0x63;
        assert_eq!(l.read(&mem).danger, Danger::BlinkingRed);
    }

    /// Случайные значения заставки детерминированы и не выходят из диапазонов.
    #[test]
    fn splash_values_are_deterministic_and_in_range() {
        let a = CockpitValues::for_splash();
        assert_eq!(a, CockpitValues::for_splash(), "заставка будет мерцать");
        for indicator in a.left.iter().chain(a.right.iter()) {
            match *indicator {
                Indicator::Gauge(v) => assert!(v <= GAUGE_MAX),
                Indicator::Deflection(v) => {
                    assert!(v.abs() <= DEFLECTION_MAX)
                }
                Indicator::Missiles(v) => assert!(v <= MISSILE_SLOTS as u8),
                Indicator::Empty => {}
            }
        }
    }

    /// При смене соотношения сторон клетка с текстом не меняется, а полосы
    /// и центральная клетка растягиваются по ширине окна.
    #[test]
    fn label_cell_width_does_not_follow_aspect_ratio() {
        let narrow = panel_geometry(1280, 960);
        let wide = panel_geometry(1706, 960);
        assert_eq!(
            narrow.label_width(),
            wide.label_width(),
            "клетка с текстом растянулась вместе с окном"
        );
        assert!(
            wide.bar_width() > narrow.bar_width(),
            "длинная клетка-полоса не растянулась"
        );
        assert!(
            wide.center_cell().w > narrow.center_cell().w,
            "центральная клетка не растянулась"
        );
        // При изменении высоты окна клетка с текстом масштабируется вместе
        // со строками.
        let taller = panel_geometry(1280, 1080);
        assert!(taller.label_width() > narrow.label_width());
    }

    /// Подписи повторяют оригинальную панель: слева шесть обозначений и
    /// значок скорости, справа `SP`, `RL`, `DC` и номера энергоблоков.
    #[test]
    fn indicator_labels_match_original() {
        assert_eq!(
            LEFT_LABELS,
            [
                Some("FS"),
                Some("AS"),
                Some("FU"),
                Some("CT"),
                Some("LT"),
                Some("AL"),
                None
            ]
        );
        assert_eq!(
            RIGHT_LABELS,
            [
                Some("SP"),
                Some("RL"),
                Some("DC"),
                Some("1"),
                Some("2"),
                Some("3"),
                Some("4")
            ]
        );
        assert_eq!(
            indicator_label(PANEL_ROWS - 1, false),
            None,
            "слева снизу должен быть значок скорости"
        );
        assert!(
            !LEFT_LABELS
                .iter()
                .flatten()
                .chain(RIGHT_LABELS.iter().flatten())
                .any(|s| *s == "XX"),
            "в панели осталась заглушка XX"
        );
    }

    /// Подписи и значок скорости рисуются белым по чёрному фону, а не по полосе.
    #[test]
    fn indicator_labels_are_on_black_background() {
        // Шрифт с единственным горящим пикселем в каждом знаке: так легко
        // посчитать, сколько пикселей занимает каждая подпись.
        let mut font = vec![0u8; 0x300];
        for ch in 0x20..0x80u8 {
            font[(ch - 0x20) as usize * 8] = 0x80;
        }
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(
            &mut f,
            &font,
            CockpitStyle::COBRA_MK3,
            &values(MISSILE_SLOTS as u8),
        );
        let g = panel_geometry(1280, 960);
        for right in [false, true] {
            for row in 0..PANEL_ROWS {
                let area = g.label_cell(row, right);
                let mut lit = 0;
                for y in area.y..area.y + area.h {
                    for x in area.x..area.x + area.w {
                        let p = f.pixels()[y as usize * 1280 + x as usize];
                        if p == FRAME_COLOR {
                            lit += 1;
                        } else {
                            assert_eq!(p, BACKGROUND, "подпись не на чёрном фоне");
                        }
                    }
                }
                match indicator_label(row, right) {
                    Some(label) => {
                        let chars = label.len() as i32;
                        let scale = (area.h / 8).min(area.w / (8 * chars).max(1)).max(1);
                        assert_eq!(
                            lit as i32,
                            chars * scale * scale,
                            "подпись {label} не нарисована (right={right}, row={row})"
                        );
                    }
                    None => {
                        let scale = (area.h / 8).min(area.w / 12).max(1);
                        let ink: i32 = SPEED_ICON.iter().map(|b| b.count_ones() as i32).sum();
                        assert_eq!(
                            lit as i32,
                            ink * scale * scale,
                            "значок скорости не нарисован (right={right}, row={row})"
                        );
                    }
                }
            }
        }
    }

    /// Цвет длинных клеток — по типу корабля: заставка (Cobra Mk III) красная,
    /// Cobra Mk IV — фиолетовая.
    #[test]
    fn bar_color_follows_ship_type() {
        let font = font();
        let g = panel_geometry(1280, 960);
        let bar = g.bar_cell(0, false);
        let mut splash = Frame::new(1280, 960);
        Cockpit::new().draw(
            &mut splash,
            &font,
            CockpitStyle::COBRA_MK3,
            &values(MISSILE_SLOTS as u8),
        );
        assert_eq!(
            splash.pixels()[(bar.y + bar.h / 2) as usize * 1280
                + (bar.x + bar.w / 2) as usize],
            BAR_RED,
            "на заставке полосы должны быть красными"
        );
        let mut mk4 = Frame::new(1280, 960);
        Cockpit::new().draw(
            &mut mk4,
            &font,
            CockpitStyle::COBRA_MK4,
            &values(MISSILE_SLOTS as u8),
        );
        assert_eq!(
            mk4.pixels()[(bar.y + bar.h / 2) as usize * 1280 + (bar.x + bar.w / 2) as usize],
            BAR_MAGENTA,
            "у Cobra Mk IV полосы должны быть фиолетовыми"
        );
    }

    /// Нижняя часть непрозрачна по всей своей площади (закрывает подложку).
    #[test]
    fn cockpit_is_drawn_and_opaque() {
        let (f, g) = draw(1680, 1050);
        for y in g.panel_y..1050 {
            for x in 0..1680 {
                assert_ne!(
                    f.pixels()[y as usize * 1680 + x as usize],
                    0,
                    "прозрачный пиксель панели ({x},{y})"
                );
            }
        }
    }

    /// Кокпит растянут на всю ширину окна и прижат к нижнему краю.
    #[test]
    fn cockpit_spans_full_width_but_is_compact() {
        let (f, g) = draw(1680, 1050);
        let mid = (g.panel_y + g.panel_h / 2) as usize * 1680;
        assert_ne!(f.pixels()[mid], 0, "левый край панели пуст");
        assert_ne!(f.pixels()[mid + 1679], 0, "правый край панели пуст");
        assert!(g.panel_h < 1050 / 3, "панель всё ещё занимает треть экрана");
        assert!(g.panel_h > 0);
        assert_eq!(
            g.panel_y + g.panel_h,
            1050,
            "панель не прижата к низу окна"
        );
    }

    /// Панель занимает только часть окна: над ней остаётся область заставки.
    #[test]
    fn cockpit_leaves_room_for_splash_title() {
        let g = panel_geometry(1920, 1080);
        assert!(g.panel_y > 1080 / 2, "под панель не осталось места");
        assert_eq!(g.panel_y + g.panel_h, 1080);
        assert!(g.cell >= 1, "знакомест должен быть не нулевым");
    }

    /// Цвет линий можно переопределить (красная рамка при попадании).
    #[test]
    fn cockpit_line_color_can_change() {
        let mut f = Frame::new(1280, 960);
        let font = font();
        Cockpit::new().draw(
            &mut f,
            &font,
            CockpitStyle {
                line: DANGER_COLOR,
                bar: BAR_RED,
            },
            &values(MISSILE_SLOTS as u8),
        );
        let g = panel_geometry(1280, 960);
        let main = g.main_frame();
        let panel = g.panel_frame();
        assert_eq!(
            f.pixels()[main.y as usize * 1280 + main.x as usize],
            DANGER_COLOR
        );
        assert_eq!(
            f.pixels()[panel.y as usize * 1280 + panel.x as usize],
            DANGER_COLOR
        );
    }

    /// Стиль оригинала: фон панели чёрный, линии светлые, длинные клетки
    /// красные (заставка и Cobra Mk III).
    #[test]
    fn cockpit_style_matches_original() {
        let (f, g) = draw(1280, 960);
        let (mut black, mut light, mut red) = (0usize, 0usize, 0usize);
        for y in g.panel_y..960 {
            for x in 0..1280 {
                match f.pixels()[y as usize * 1280 + x as usize] {
                    BACKGROUND => black += 1,
                    FRAME_COLOR => light += 1,
                    BAR_RED => red += 1,
                    _ => {}
                }
            }
        }
        assert!(black > 0, "нет чёрного фона панели");
        assert!(light > 0, "панель не содержит белых линий");
        assert!(red > 0, "нет красных клеток");
        assert!(black > light + red, "панель слишком заполнена");
    }

    /// Геометрия не выходит за пределы окна.
    #[test]
    fn cockpit_panel_fits_window() {
        let g = panel_geometry(1920, 1080);
        assert!(g.panel_y >= 0, "панель начинается выше окна");
        assert!(g.panel_y + g.panel_h <= 1080, "панель выходит за окно");
        assert_eq!(g.width, 1920);
        let view = g.view();
        assert!(!view.is_empty(), "область вида пуста");
        assert_eq!(view.x, g.margin + LINE);
        assert_eq!(view.bottom(), g.main_frame().bottom() - LINE);
    }
}
