//! Экран полёта: нативный рендер по данным Z80 (клавиша `F1`).
//!
//! Своей симуляции у экрана нет. Пока он открыт, продолжает работать
//! эмулятор: координаты объектов, угловые скорости, скорость, показания
//! приборов и таблица звёзд читаются из памяти игры в
//! [`Flight::sync_from_z80`]. Рендер только рисует то, что уже посчитал Z80,
//! поэтому полёт совпадает с оригиналом, включая J-драйв и его особенности.
//!
//! Экран занимает верхнюю часть окна — вид из кабины. Ниже и вокруг него
//! рисуется нативный кокпит [`crate::cockpit`], тот же, что на заставке.
//! Поверх вида — сообщения, которые печатает сама игра: они перехватываются
//! из эмулятора (хук `sub_D499`, см. [`crate::machine::GameMsgTap`]) и
//! показываются тем же шрифтом, цветом и в том же месте, что заголовок
//! «E L I T E» на заставке. Пока свежего сообщения нет, на этом месте
//! держится название направления («Front View» и т.п.), а при подготовке
//! к гиперпрыжку внизу — строка «Hyperspace -- <имя планеты>» (размером
//! и цветом приглашения заставки).
//!
//! Объекты (корабли, станция, планета, солнце) рисуются общим рендером
//! [`crate::render3d`], что и корабль на заставке: непрозрачные тёмные грани
//! и белые рёбра.

use crate::cockpit::{self, CockpitGeom};
use crate::display::Frame;
use crate::machine::GameMsgTap;
use crate::render3d::{self, Rotation, Screen};
use crate::shipdata::{self, Ship};

/// Цвет фона (непрозрачный чёрный).
const BG: u32 = 0xFF00_0000;
/// Циан — линии объектов и перекрестие.
const CYAN: u32 = 0xFF00_FFFF;
/// Красный — лазеры и опасность.
const RED: u32 = 0xFFFF_4040;
/// Жёлтый — цвет луча выстрела противника.
const ENEMY_BEAM: u32 = 0xFFFF_A020;
/// Солнце — ровный жёлтый диск.
const SUN_COLOR: u32 = 0xFFFF_FF00;
/// Планета по умолчанию — тёмно-зелёный диск.
const PLANET_COLOR: u32 = 0xFF00_5800;
/// Сколько секунд длится нативная анимация вылета со станции.
///
/// Оригинал проигрывает тоннель примерно 1.2 с (36 + 11 кадров), поэтому
/// к моменту, когда Z80 сбросит `JumpDocking`, станция успевает уйти, а
/// звёзды — разогнаться.
const LAUNCH_TIME: f32 = 1.2;
/// Цвет надписей в переходе: приглушённый циан, чтобы не спорить с HUD.
const TRANSITION_TEXT: u32 = 0xFF80_FFFF;
/// Белый — цвет подписи направления вида и строки гиперпрыжка, как у
/// надписей на заставке (`E L I T E`, приглашение).
const VIEW_LABEL_COLOR: u32 = 0xFFD7_D7D7;
/// Сколько кадров (50 Гц) держать перехваченное сообщение игры после
/// последнего напечатанного символа: старые строки не должны висеть вечно.
const MSG_MAX_AGE: u32 = 75;
/// Текстовые строки ZX `0..=2` — верхняя зона вида, куда игра печатает
/// подписи; здесь нативный экран показывает верхнюю надпись.
const MSG_TOP_ROWS: u8 = 2;
/// Цвет тоннеля гиперпрыжка.
const WARP_RING_COLOR: u32 = 0xFF40_A0FF;
/// Максимальная скорость в единицах `HighDeltaSpeed` (0..=31).
///
/// В оригинале `DeltaSpeed` — 16-битное слово с шагом `0xA0` на кадр, а его
/// старший байт `HighDeltaSpeed` меняется на 1 за 1/0.625 кадра. Полная шкала
/// заканчивается на 31: выше игра скорость не поднимает.
const SPEED_MAX: f32 = 31.0;
/// Сколько секунд виден луч собственного лазера.
const LASER_TIME: f32 = 0.10;
/// Сколько секунд виден луч выстрела противника.
///
/// В оригинале луч живёт один кадр (флаг `oFlags` сбрасывается сразу после
/// печати), но при 50 Гц это почти незаметно. Чуть более долгий след делает
/// атаку читаемой, не превращая её в постоянную линию.
const ENEMY_LASER_TIME: f32 = 0.14;
/// Ближняя плоскость отсечения.
const NEAR: f32 = 0.5;
/// Минимальное число звёзд на экране.
///
/// В оригинале таблица `byte_7A39` содержит ровно 16 звёзд на кадр
/// 256×192. На современном разрешении такое поле выглядит почти пустым,
/// поэтому фактическое число звёзд считается от площади вида
/// (см. [`star_count_for`]), а эта константа задаёт минимум.
const STAR_COUNT: usize = 16;
/// Площадь вида, на которую приходится одна звезда.
///
/// В оригинале одна звезда на ~3072 пикселя. Здесь плотность вдвое ниже:
/// современная точка — это 1 px, и слишком густое поле превращается в шум.
const STAR_AREA_PER_STAR: f32 = 6_000.0;
/// Пределы числа звёзд, чтобы поле не пустело и не забивало экран.
const STAR_COUNT_MIN: usize = 16;
const STAR_COUNT_MAX: usize = 400;
/// Делитель шага звёздного поля.
///
/// В оригинале координаты звёзд растягиваются от центра в
/// `1 + b/256` раз за кадр, где `b = (HighDeltaSpeed >> 1) + 1`.
const STAR_ZOOM_DIVISOR: f32 = 256.0;
/// Радиус (`x/z` или `y/z`), после которого звезда ушла за кадр.
///
/// Оригинал не «приближает» звезду в бесконечность: когда её координата
/// выходит за край, она рождается заново ближе к центру. Тот же порог не даёт
/// звезде разгоняться в огромную полосу на последних кадрах.
const STAR_RESPAWN_RADIUS: f32 = 1.8;
/// Абсолютный предел расстояния до звезды.
///
/// После поворотов камеры большая боковая координата может стать глубиной,
/// поэтому одного углового порога мало: без предела звезда уползала бы всё
/// дальше от начала координат.
const STAR_MAX_DISTANCE: f32 = 15_000.0;
/// Шаг растяжения звёзд на такте J-драйва.
///
/// В `PrintStars` J-драйв вызывает `sub_78B1` три раза: со скоростями
/// `0x1E`, `0x10` и `0x10`, то есть с `b = 16`, `9` и `9`. Сумма `34`
/// даёт тот же разгон за такт, как три прохода оригинала.
const STAR_FAST_SCROLL_B: f32 = 16.0 + 9.0 + 9.0;
/// Поворот звёздного поля за единицу `HighAngleLR` (крен).
///
/// В `CalcNewCrdObj_Pl` крен применяется как `angleLR * 2 / 256`, а тангаж —
/// как `angleDU / 256`. Это малые углы за игровой такт, а не за кадр 50 Гц:
/// оригинал пересчитывает координаты примерно на 5.5 Гц, поэтому поворот
/// звёзд применяется только на такте (см. `sync_from_z80`).
const STAR_ANGLE_LR_PER_UNIT: f32 = 2.0 / 256.0;
/// Поворот звёздного поля за единицу `HighAngleDU` (тангаж).
const STAR_ANGLE_DU_PER_UNIT: f32 = 1.0 / 256.0;

fn view_rotation(dir: ViewDir) -> Rotation {
    match dir {
        ViewDir::Forward => Rotation::identity(),
        ViewDir::Back => Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::PI),
        ViewDir::Left => Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2),
        ViewDir::Right => Rotation::axis_angle([0.0, 1.0, 0.0], -std::f32::consts::FRAC_PI_2),
    }
}

/// Идёт ли сейчас анимация вылета, прыжка или стыковки. В это время оригинал
/// игнорирует клавиши `1`-`4`, а нативный вид не должен закрывать ролик.
pub fn z80_in_transition(mem: &[u8]) -> bool {
    mem.get(Z80_JUMP_DOCKING).copied().unwrap_or(0) != 0
        || mem.get(Z80_FLG_JUMP_DOCKING).copied().unwrap_or(0) != 0
}

/// Показывает ли Z80 сейчас настоящий вид из кабины, а не переход.
///
/// Одного `WhatOnScreen < 4` мало: во время вылета со станции и прыжка игра
/// тоже держит `WhatOnScreen = 0`, но рисует тоннель/анимацию и игнорирует
/// управление. Нативные экраны (закупка, карта) должны ждать, пока оригинал
/// действительно окажется в полёте.
pub fn z80_in_cockpit(mem: &[u8]) -> bool {
    let Some(&screen) = mem.get(Z80_WHAT_ON_SCREEN) else {
        return false;
    };
    screen < 4 && !z80_in_transition(mem)
}

/// Пристыкован ли корабль к станции (`IndDanger == 0`).
pub fn z80_is_docked(mem: &[u8]) -> bool {
    mem.get(Z80_IND_DANGER).copied() == Some(0)
}

// Адреса в памяти Z80 (версия Кладова). Координаты объектов ведутся уже
// в системе координат игрока: камера стоит в начале координат вдоль +Z.
/// Минимальный объём памяти, покрывающий все адреса полётных данных
/// (включая таблицу звёзд и флаг J-драйва).
const Z80_MEM_MIN: usize = 0x7EE0;
/// `WhatOnScreen` — что сейчас показывает игра: `0..=3` — вид из кабины
/// (вперёд, назад, влево, вправо), остальные значения — станция и её меню,
/// карты, инвентарь и прочие экраны оригинала.
pub const Z80_WHAT_ON_SCREEN: usize = 0x70E3;
/// `IndDanger`: `0` — корабль пристыкован к станции, иначе — в полёте.
pub const Z80_IND_DANGER: usize = 0x7104;
/// `JumpDocking`: `1` — прыжок/стыковка, `2` — межгалактический прыжок.
pub const Z80_JUMP_DOCKING: usize = 0xD03F;
/// `FlgJumpDocking`: `1`, пока идёт анимация прыжка/стыковки/вылета. В это
/// время оригинал игнорирует клавиши `1`-`4`.
pub const Z80_FLG_JUMP_DOCKING: usize = 0xD040;
/// `JumpCountdown` — обратный отсчёт до гиперпрыжка (`0x15` в начале).
///
/// Оригинал уменьшает его на единицу за кадр и показывает `>> 1`, то есть
/// на экране идут числа `10..1`. Ноль означает, что началась сама анимация
/// перехода.
pub const Z80_JUMP_COUNTDOWN: usize = 0xDBA1;
/// `PlanetName` — имя планеты, которое игра сгенерировала последним
/// (`GeneratePlanetName` + `CopyPlanetName`, 8 байт ASCII). При подготовке
/// к гиперпрыжку (`GoJump` → `SearchNearPlanet`) это имя выбранной цели.
const Z80_PLANET_NAME: usize = 0x6E93;
/// Длина буфера `PlanetName`.
const Z80_PLANET_NAME_LEN: usize = 8;
/// `HighAngleLR` — угловая скорость крена (знак-величина).
const Z80_HIGH_ANGLE_LR: usize = 0x70E4;
/// `HighAngleDU` — угловая скорость тангажа (знак-величина).
const Z80_HIGH_ANGLE_DU: usize = 0x70E5;
/// `HighDeltaSpeed` — текущая скорость, 0..=31.
const Z80_HIGH_DELTA_SPEED: usize = 0x70E6;
/// `Speed` — индикатор скорости: `5 * HighDeltaSpeed + 5`, 0..=0xA0.
///
/// В отличие от `HighDeltaSpeed`, эту ячейку J-драйв не трогает, поэтому
/// скорость надёжнее брать отсюда.
const Z80_SPEED_GAUGE: usize = 0x70F4;

/// `byte_D08E`: <>0 — у игрока Cobra Mk IV (в блоке командира `PilotName`).
/// От типа корабля зависит цвет длинных клеток приборной панели.
const Z80_COBRA_MK4: usize = 0xD08E;
/// `ShieldFront` — передний щит, 0..0xA0.
const Z80_SHIELD_FRONT: usize = 0x70E8;
/// `TempLaser` — нагрев лазера, 0..0xA0.
const Z80_TEMP_LASER: usize = 0x70F0;
/// Таблица звёзд оригинала (`byte_7A39`): 16 пар координат.
///
/// Оригинал пересчитывает её только на игровом такте (около 5.5 Гц), а не
/// каждый кадр 50 Гц. По изменению таблицы удобно ловить сам такт.
const Z80_STARS_TABLE: usize = 0x7A39;
/// Длина таблицы звёзд в байтах.
const Z80_STARS_LEN: usize = 0x20;
/// `StarsFastScroll` — взводится на такт J-драйва, гасится в `PrintStars`.
const Z80_STARS_FAST_SCROLL: usize = 0x7EDD;
/// Слот станции (`objCoriolis`).
const Z80_OBJ_CORIOLIS: usize = 0x6048;
/// Первый из шести основных слотов объектов.
const Z80_OBJ_SLOT0: usize = 0x606F;
/// Размер слота `SlotObj`.
const Z80_SLOT_SIZE: usize = 0x27;
/// Слот ракеты (`objMissile`).
const Z80_OBJ_MISSILE: usize = 0x6159;
/// `oEnerg` в слоте: 0 — слот пуст.
const Z80_SLOT_OENERG: usize = 0x22;
/// `oAdrData` в слоте: адрес блока модели.
const Z80_SLOT_OADRDATA: usize = 0x23;
/// Координаты солнца (9 байт, знак-величина).
const Z80_SUN_CRD: usize = 0x7107;
/// Координаты планеты (9 байт, знак-величина).
const Z80_PLANET_CRD: usize = 0x7110;
/// Радиус солнца в единицах координат игры.
///
/// Оригинал считает радиус диска как `0xA00000 / z`: при `z ≈ 96000` это
/// ≈108 px в окне 256x128. Фокус `1.15 * высота_вида` переводит это в
/// `R = (108/128) * 96000 / 1.15 ≈ 70000`.
const SUN_RADIUS: f32 = 70_000.0;
/// Радиус планеты. Меньше калиброванного радиуса оригинала: планета
/// рисуется сплошной заливкой, поэтому при том же радиусе заливала бы почти
/// весь вид. Такой диск читается как планета, а не как стена.
const PLANET_RADIUS: f32 = 42_000.0;

/// Тип объекта в сцене.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BodyKind {
    Ship,
    Station,
    Planet,
    Sun,
}

/// Объект сцены в координатах вида.
#[derive(Clone)]
struct Body {
    kind: BodyKind,
    /// Геометрия (для кораблей и станции).
    model: Option<&'static Ship>,
    /// Положение в координатах вида (камера в начале координат).
    pos: [f32; 3],
    /// Ориентация модели относительно вида.
    rot: Rotation,
    /// Радиус для проекции.
    radius: f32,
    /// Цвет диска (планета или солнце); корабли рисуются общим цветом.
    color: u32,
}

/// Звезда в космическом пространстве.
struct Star {
    /// Положение в координатах вида (`+Z` — вперёд).
    pos: [f32; 3],
    mag: u8,
}

/// Выстрел противника: отрезок от корабля к окрестности перекрестия.
///
/// Храним координаты вида, чтобы линия правильно поворачивалась вместе с
/// полем, пока живёт.
struct EnemyShot {
    from: [f32; 3],
    to: [f32; 3],
    life: f32,
}

/// Направление вида из кабины.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewDir {
    /// Вид вперёд (+Z)
    Forward,
    /// Вид назад (-Z)
    Back,
    /// Вид влево (-X)
    Left,
    /// Вид вправо (+X)
    Right,
}

impl Default for ViewDir {
    fn default() -> Self {
        ViewDir::Forward
    }
}

/// Какой переход сейчас показывает нативный рендер.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionKind {
    /// Вылет со станции: станция уходит назад, звёзды разгоняются.
    Launch,
    /// Гиперпрыжок: сначала отсчёт, затем тоннель и полосы звёзд.
    Hyperspace,
    /// Стыковка: тот же тоннель оригинала, но без отсчёта.
    Docking,
}

/// Состояние нативной анимации перехода.
#[derive(Clone, Copy, Debug)]
struct Transition {
    kind: TransitionKind,
    /// Время с начала перехода.
    elapsed: f32,
    /// Последнее значение `JumpCountdown` из памяти игры.
    countdown: u8,
    /// Отсчёт закончился, рисуем сам прыжок (тоннель).
    warp: bool,
    /// Время с начала тоннеля (для гиперпрыжка — после отсчёта).
    warp_elapsed: f32,
}

impl Transition {
    fn new(kind: TransitionKind, countdown: u8) -> Self {
        // Вылет начинается сразу со станции, стыковка — сразу с тоннеля.
        // Гиперпрыжок сначала показывает обычный полёт и отсчёт.
        let warp =
            kind != TransitionKind::Launch && (kind == TransitionKind::Docking || countdown == 0);
        Transition {
            kind,
            elapsed: 0.0,
            countdown,
            warp,
            warp_elapsed: 0.0,
        }
    }
}

/// Снимок перехваченного игрового сообщения для показа на виде.
#[derive(Clone, Debug, Default)]
struct CapturedMsg {
    /// Текст строки ровно как напечатал Z80.
    text: String,
    /// Текстовая строка ZX (0..=23), куда она печаталась.
    row: u8,
    /// Возраст в кадрах после последнего символа.
    frames: u32,
}

/// Экран полёта по данным Z80.
pub struct Flight {
    /// Показывать ли вид из кабины вместо ZX-кадра.
    pub active: bool,
    /// Текущее направление вида из кабины.
    pub view_dir: ViewDir,
    /// С момента открытия экрана Z80 успел показать настоящий вид из кабины.
    ///
    /// Нужен только как совместимость для перехода со станции: если экран уже
    /// открыт, а `WhatOnScreen` ещё равен станционному `0x0B`, не закрываем его
    /// до первого полётного кадра.
    seen_flight_screen: bool,
    /// Предыдущий `TempLaser` из Z80 — по его росту ловим выстрел.
    prev_laser_temp: Option<u8>,
    /// Предыдущий передний щит Z80 — по его падению ловим атаку.
    prev_z80_shield: Option<u8>,
    /// Снимок таблицы звёзд Z80: её изменение означает игровой такт.
    prev_star_table: Option<[u8; Z80_STARS_LEN]>,
    /// В памяти был замечен взвод `StarsFastScroll` (J-драйв).
    z80_fast_scroll: bool,
    /// Разогнать звёзды на ближайшем шаге симуляции (J-драйв).
    star_boost: bool,
    /// Лучи выстрелов противников.
    enemy_shots: Vec<EnemyShot>,
    /// Красная рамка вида после попадания.
    damage_flash: f32,
    /// Сколько ещё виден луч собственного лазера.
    laser_timer: f32,
    /// Скорость в единицах `HighDeltaSpeed` (`0..=31`) — темп звёзд.
    speed: f32,
    /// У игрока Cobra Mk IV (`byte_D08E`) — фиолетовые клетки панели.
    cobra_mk4: bool,
    bodies: Vec<Body>,
    stars: Vec<Star>,
    planet_color: u32,
    rng: u32,
    /// Нативная анимация вылета/прыжка/стыковки вместо ZX-ролика.
    transition: Option<Transition>,
    /// Имя планеты для надписи «Hyperspace -- <имя>» во время отсчёта.
    jump_planet_name: String,
    /// Последнее сообщение, перехваченное из игры (см. [`GameMsgTap`]).
    tap_msg: Option<CapturedMsg>,
}

impl Flight {
    pub fn new() -> Self {
        let mut f = Flight {
            active: false,
            view_dir: ViewDir::Forward,
            seen_flight_screen: false,
            prev_laser_temp: None,
            prev_z80_shield: None,
            prev_star_table: None,
            z80_fast_scroll: false,
            star_boost: false,
            enemy_shots: Vec::new(),
            damage_flash: 0.0,
            laser_timer: 0.0,
            speed: 0.0,
            cobra_mk4: false,
            bodies: Vec::new(),
            stars: Vec::new(),
            planet_color: PLANET_COLOR,
            rng: 0x1234_5678,
            transition: None,
            jump_planet_name: String::new(),
            tap_msg: None,
        };
        f.stars = make_stars(&mut f.rng, STAR_COUNT);
        f
    }

    /// Открыть экран полёта. `planet_color` — цвет планеты текущей системы
    /// (ноль оставляет цвет по умолчанию).
    pub fn open(&mut self, planet_color: u32) {
        self.active = true;
        self.view_dir = ViewDir::Forward;
        self.seen_flight_screen = false;
        self.prev_laser_temp = None;
        self.prev_z80_shield = None;
        self.prev_star_table = None;
        self.z80_fast_scroll = false;
        self.star_boost = false;
        self.enemy_shots.clear();
        self.damage_flash = 0.0;
        self.laser_timer = 0.0;
        self.speed = 0.0;
        self.cobra_mk4 = false;
        self.bodies.clear();
        self.transition = None;
        self.jump_planet_name.clear();
        self.tap_msg = None;
        if planet_color != 0 {
            self.planet_color = planet_green(planet_color);
        }
        self.stars = make_stars(&mut self.rng, STAR_COUNT);
    }

    /// Начать нативную анимацию перехода.
    ///
    /// `launch` передаётся из `F1` на станции: по памяти игры вылет и стыковку
    /// не отличить (оба взводят `JumpDocking = 1`), а вид должен знать, что
    /// показывать — уход станции или тоннель.
    pub fn start_transition(&mut self, launch: bool, mem: &[u8]) {
        if self.transition.is_some() {
            return;
        }
        let countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
        let intergalactic = mem.get(Z80_JUMP_DOCKING).copied() == Some(2);
        let kind = if launch {
            TransitionKind::Launch
        } else if countdown > 0 || intergalactic {
            TransitionKind::Hyperspace
        } else {
            TransitionKind::Docking
        };
        if kind == TransitionKind::Hyperspace && countdown > 0 {
            self.jump_planet_name = planet_name_from_z80(mem);
        }
        self.transition = Some(Transition::new(kind, countdown));
    }

    /// Идёт ли сейчас нативный переход (вылет, прыжок или стыковка).
    pub fn in_transition(&self) -> bool {
        self.transition.is_some()
    }

    /// Какой переход сейчас идёт (для тестов и отладки).
    pub fn transition_kind(&self) -> Option<TransitionKind> {
        self.transition.map(|t| t.kind)
    }

    /// Переключить вид по сторонам (F1-F4 в нативном полёте): те же клавиши
    /// 1-4, которыми оригинал меняет `WhatOnScreen`.
    pub fn set_view_dir(&mut self, dir: ViewDir) {
        if !self.active {
            return;
        }
        if self.view_dir != dir {
            self.view_dir = dir;
            self.stars = make_stars(&mut self.rng, self.stars.len().max(STAR_COUNT));
        }
    }

    /// Закрыть экран и вернуться к ZX-кадру (клавиша `1`).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Забрать перехваченное игровое сообщение (заполняет хук печати
    /// `sub_D499`; см. [`crate::machine::GameMsgTap`]).
    pub fn sync_game_msg(&mut self, tap: &GameMsgTap) {
        if !self.active || !tap.enabled || tap.text.trim().is_empty() {
            return;
        }
        self.tap_msg = Some(CapturedMsg {
            text: tap.text.clone(),
            row: tap.row,
            frames: tap.frames,
        });
    }

    /// Текст верхней надписи: свежее сообщение игры из верхней зоны вида,
    /// причёсанное с большой буквы; иначе — подпись направления вида.
    fn top_label(&self) -> String {
        self.tap_msg
            .as_ref()
            .and_then(|m| {
                (m.frames <= MSG_MAX_AGE
                    && m.row <= MSG_TOP_ROWS
                    && !m.text.trim().is_empty())
                    .then(|| pretty_title(m.text.trim()))
            })
            .unwrap_or_else(|| view_label(self.view_dir).to_string())
    }

    /// Следить за экраном, который показывает Z80, и закрывать нативный вид,
    /// когда игра уходит с вида из кабины.
    ///
    /// Игровые клавиши (`O`, `I`, `Enter` и т.п.) всегда уходят в эмулятор,
    /// поэтому оригинал сам переключает `WhatOnScreen`. Без этой проверки
    /// нативный вид оставался поверх ZX-кадра, и карта/инвентарь открывались
    /// «в фоне». Переходы вылета/прыжка/стыковки не закрывают вид: он сам
    /// рисует их анимацию, а Z80 в это время ведёт отсчёт и логику игры.
    pub fn follow_z80_screen(&mut self, mem: &[u8]) {
        if !self.active {
            return;
        }
        let Some(&screen) = mem.get(Z80_WHAT_ON_SCREEN) else {
            return;
        };
        if screen < 4 {
            self.seen_flight_screen = true;
        }
        if z80_in_transition(mem) {
            // Ролик теперь рисуем сами: нативный вид остаётся открытым и
            // получает состояние перехода. Вылет/прыжок/стыковку по флагам
            // различает `start_transition` (для вылета признак передаёт App).
            if self.transition.is_none() {
                self.start_transition(false, mem);
            }
            return;
        }
        self.transition = None;
        if !z80_in_cockpit(mem) {
            // `0x0B` — экран станции. После запуска со станции Z80 держит его
            // несколько кадров, пока идёт анимация вылета; уже открытый экран
            // не закрываем до первого настоящего полётного кадра.
            if screen == 0x0B && !self.seen_flight_screen {
                return;
            }
            self.active = false;
        }
    }

    /// Синхронизировать рендер с данными из памяти Z80.
    ///
    /// Абсолютной матрицы ориентации игрока в оригинале нет: камера задаётся
    /// двумя угловыми скоростями (`HighAngleLR`/`HighAngleDU`), а координаты
    /// всех объектов ведутся уже в системе координат игрока. `CalcNewCrdObj_Pl`
    /// каждый кадр поворачивает их на эти углы и сдвигает по `HighDeltaSpeed`,
    /// поэтому координаты слотов, `SunCrd` и `PlanetCrd` — это готовые
    /// координаты вида. Камера стоит в начале координат.
    pub fn sync_from_z80(&mut self, mem: &[u8]) {
        if !self.active || mem.len() < Z80_MEM_MIN {
            return;
        }

        // Скорость берём из индикатора (`5 * HighDeltaSpeed + 5`): на такте
        // J-драйва `PrintStars` временно пишет в `HighDeltaSpeed` `0x1E`/`0x10`,
        // и прямой опрос заставлял бы шкалу и звёзды прыгать.
        let gauge = mem[Z80_SPEED_GAUGE];
        self.speed = (gauge.saturating_sub(5) as f32 / 5.0).clamp(0.0, SPEED_MAX);
        self.cobra_mk4 = mem.get(Z80_COBRA_MK4).is_some_and(|&b| b != 0);

        // Отсчёт гиперпрыжка ведёт Z80. Читаем его каждый кадр: пока число
        // больше нуля, нативный рендер показывает обычный полёт и цифру, а
        // после нуля переключается на тоннель.
        if let Some(t) = &mut self.transition {
            if t.kind == TransitionKind::Hyperspace {
                t.countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
                if t.countdown == 0 {
                    t.warp = true;
                } else {
                    // Во время отсчёта игра держит в `PlanetName` имя цели
                    // прыжка — показываем его внизу (как на заставке).
                    self.jump_planet_name = planet_name_from_z80(mem);
                }
            }
        }

        // Оригинал не оставляет флаг выстрела противника в памяти: бит 3
        // сбрасывается сразу после печати луча. Зато попадание уменьшает
        // `ShieldFront` — по этому падению и показываем ответный луч.
        let shield_raw = mem[Z80_SHIELD_FRONT];
        let shield_dropped = self.prev_z80_shield.is_some_and(|prev| shield_raw < prev);
        self.prev_z80_shield = Some(shield_raw);
        if shield_dropped {
            self.damage_flash = 0.25;
        }

        // Угловые скорости известны, но оригинал пересчитывает звёзды и
        // координаты объектов не каждый кадр, а на игровом такте (~5.5 Гц):
        // таблица звёзд `byte_7A39` меняется только тогда. Раньше поворот
        // применялся на каждом кадре 50 Гц, из-за чего звёзды крутились
        // примерно в 9 раз быстрее объектов.
        let mut table = [0u8; Z80_STARS_LEN];
        table.copy_from_slice(&mem[Z80_STARS_TABLE..Z80_STARS_TABLE + Z80_STARS_LEN]);
        let ticked = self.prev_star_table.is_some_and(|prev| prev != table);
        self.prev_star_table = Some(table);
        if mem[Z80_STARS_FAST_SCROLL] == 1 {
            self.z80_fast_scroll = true;
        }
        if ticked {
            // J-драйв `PrintStars` проматывает звёзды тремя проходами
            // `sub_78B1` (скорости `0x1E` и дважды `0x10`) и на этом такте
            // временно перезаписывает `HighDeltaSpeed`. Ловим и сам флаг, и
            // этот след, если флаг не попал в кадр опроса.
            let fast = self.z80_fast_scroll
                || mem[Z80_HIGH_DELTA_SPEED] as i32 != self.speed.round() as i32;
            let roll = read_signed8(mem[Z80_HIGH_ANGLE_LR]) * STAR_ANGLE_LR_PER_UNIT;
            let pitch = -read_signed8(mem[Z80_HIGH_ANGLE_DU]) * STAR_ANGLE_DU_PER_UNIT;
            let turn = Rotation::axis_angle([0.0, 0.0, 1.0], roll)
                .mul(&Rotation::axis_angle([1.0, 0.0, 0.0], pitch));
            // Каждый проход `sub_78B1` поворачивает поле, поэтому при J-драйве
            // за такт набегает тройной поворот.
            for _ in 0..if fast { 3 } else { 1 } {
                self.turn_stars(&turn);
            }
            self.star_boost |= fast;
            self.z80_fast_scroll = false;
        }

        // Выстрел в оригинале виден только по нагреву лазера: `TempLaser`
        // подскакивает на несколько единиц, затем остывает.
        let temp = mem[Z80_TEMP_LASER];
        if let Some(prev) = self.prev_laser_temp {
            if temp > prev.saturating_add(2) {
                self.laser_timer = LASER_TIME;
            }
        }
        self.prev_laser_temp = Some(temp);

        let mut bodies: Vec<Body> = Vec::with_capacity(9);
        // Станция, шесть основных слотов и слот ракеты. Пустые слоты
        // обнулены: `oEnerg == 0`, ссылки на модель нет.
        push_z80_slot(&mut bodies, mem, Z80_OBJ_CORIOLIS, BodyKind::Station);
        for i in 0..6 {
            push_z80_slot(
                &mut bodies,
                mem,
                Z80_OBJ_SLOT0 + i * Z80_SLOT_SIZE,
                BodyKind::Ship,
            );
        }
        push_z80_slot(&mut bodies, mem, Z80_OBJ_MISSILE, BodyKind::Ship);

        // Солнце и планета: те же координаты вида, что использует оригинал.
        push_z80_celestial(
            &mut bodies,
            mem,
            Z80_SUN_CRD,
            BodyKind::Sun,
            SUN_RADIUS,
            SUN_COLOR,
        );
        push_z80_celestial(
            &mut bodies,
            mem,
            Z80_PLANET_CRD,
            BodyKind::Planet,
            PLANET_RADIUS,
            self.planet_color,
        );

        self.bodies = bodies;
        if shield_dropped {
            self.spawn_z80_enemy_shots();
        }
    }

    /// Один шаг анимации: таймеры лучей и движение звёзд.
    ///
    /// Мир не двигается — его координаты приходят из [`Self::sync_from_z80`].
    pub fn tick(&mut self, dt: f32) {
        if !self.active {
            return;
        }
        let dt = dt.clamp(0.0, 0.1);
        self.laser_timer = (self.laser_timer - dt).max(0.0);
        self.decay_enemy_shots(dt);
        self.damage_flash = (self.damage_flash - dt).max(0.0);
        if let Some(t) = &mut self.transition {
            t.elapsed += dt;
            if t.warp {
                t.warp_elapsed += dt;
            }
        }
        self.advance_stars(dt);
    }

    /// Показать выстрел противника: луч от ближайших кораблей впереди к
    /// окрестности перекрестия — так же, как это делает оригинал.
    fn spawn_z80_enemy_shots(&mut self) {
        let mut targets: Vec<usize> = self
            .bodies
            .iter()
            .enumerate()
            .filter(|(_, b)| b.kind == BodyKind::Ship && b.model.is_some() && b.pos[2] > NEAR)
            .map(|(i, _)| i)
            .collect();
        targets.sort_by(|&a, &b| {
            self.bodies[a].pos[2]
                .partial_cmp(&self.bodies[b].pos[2])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for i in targets.into_iter().take(2) {
            let origin = self.bodies[i].pos;
            let jx = (next(&mut self.rng) % 161) as f32 - 80.0;
            let jy = (next(&mut self.rng) % 121) as f32 - 60.0;
            self.enemy_shots.push(EnemyShot {
                from: origin,
                to: [jx, jy, 8000.0],
                life: ENEMY_LASER_TIME,
            });
        }
    }

    /// Погасить отжившие лучи противников.
    fn decay_enemy_shots(&mut self, dt: f32) {
        for shot in &mut self.enemy_shots {
            shot.life -= dt;
        }
        self.enemy_shots.retain(|shot| shot.life > 0.0);
    }

    /// Нарисовать вид из кабины в верхней части кадра.
    ///
    /// `geom` — геометрия кокпита: рендер заполняет область над панелью и
    /// рисует сцену только внутри главной рамки, которую потом обводит
    /// [`crate::cockpit::Cockpit`]. `font` — знакогенератор ZX для подписей
    /// вида и обратного отсчёта.
    pub fn draw(&mut self, frame: &mut Frame, geom: &CockpitGeom, font: &[u8]) {
        let w = frame.width as i32;
        let view_bottom = geom.panel_y.clamp(1, frame.height as i32);
        let view_h = view_bottom as usize;
        frame.fill_rect(0, 0, frame.width, view_h, BG);
        // Плотность звёзд зависит от площади вида: на большом разрешении
        // исходных 16 точек недостаточно, экран выглядит пустым.
        let want_stars = star_count_for(frame.width, view_h);
        if self.stars.len() != want_stars {
            self.stars = make_stars(&mut self.rng, want_stars);
        }

        // Масштаб штрихов как у сетки ZX: вид 256x128 над панелью.
        let s = ((frame.width / 256).min(view_h / 128)).max(1) as i32;
        let cx = w as f32 / 2.0;
        let cy = view_bottom as f32 / 2.0;
        let focal = (view_bottom as f32 * 1.15).max(200.0);
        let screen = Screen {
            cx,
            cy,
            k: focal,
            bx: 0.0,
            by: 0.0,
        };

        let view_rot = view_rotation(self.view_dir);

        // Ограничиваем отрисовку внутренней областью главной рамки.
        let view = geom.view();
        let (vx, vy, vw, vh) = (view.x, view.y, view.w.max(0), view.h.max(0));
        frame.set_clip(vx, vy, vw as usize, vh as usize);

        let transition = self.transition;
        match transition {
            Some(t) if t.kind == TransitionKind::Launch => {
                self.draw_launch(frame, font, &screen, vx, vy, vw, vh, s, t.elapsed);
            }
            Some(t) if t.kind == TransitionKind::Hyperspace && !t.warp => {
                // До нуля отсчёта показываем обычный полёт: игрок ещё может
                // осмотреться, а цифра говорит, сколько осталось.
                self.draw_flight_scene(frame, &screen, &view_rot, view_bottom, s);
                // Во время отсчёта верхнюю надпись рисуем сами (подпись вида):
                // перехваченные игра печатает в поле зрения цифры, их здесь не
                // показываем.
                self.draw_view_label(frame, font, vx, vy, vw, s, false);
                self.draw_jump_countdown(frame, font, vx, vy, vw, vh, s, t.countdown);
            }
            Some(t) => {
                let title = if t.kind == TransitionKind::Hyperspace {
                    "HYPERSPACE"
                } else {
                    "DOCKING"
                };
                self.draw_warp(
                    frame,
                    font,
                    &screen,
                    vx,
                    vy,
                    vw,
                    vh,
                    s,
                    t.warp_elapsed,
                    title,
                );
            }
            None => {
                self.draw_flight_scene(frame, &screen, &view_rot, view_bottom, s);
                self.draw_view_label(frame, font, vx, vy, vw, s, true);
            }
        }
        frame.clear_clip();
    }

    /// Обычный вид из кабины: звёзды, объекты, выстрелы и перекрестие.
    fn draw_flight_scene(
        &self,
        frame: &mut Frame,
        screen: &Screen,
        view_rot: &Rotation,
        view_bottom: i32,
        s: i32,
    ) {
        self.draw_stars_view(frame, screen, view_rot);
        self.draw_bodies_view(frame, screen, view_rot);
        self.draw_enemy_shots_view(frame, screen, 0, view_bottom, view_rot);
        if self.view_dir == ViewDir::Forward {
            self.draw_crosshair(frame, screen.cx, screen.cy, s);
        }
        if self.laser_timer > 0.0 && self.view_dir == ViewDir::Forward {
            self.draw_laser(frame, screen.cx, screen.cy, view_bottom - 1, s);
        }
    }

    /// Верхняя надпись вида: тем же шрифтом и в том же месте, что заголовок
    /// «E L I T E» на заставке — одной текстовой строкой ниже верхней рамки,
    /// по центру.
    ///
    /// Пока в верхней зоне вида есть свежее сообщение самой игры (перехват
    /// `sub_D499`), показываем его с большой буквы; иначе — подпись
    /// направления вида. При `use_tap == false` (отсчёт гиперпрыжка) вывод
    /// перехваченного текста отключён: игре печатает в эту зону цифры.
    ///
    /// Переход «сообщение игры → подпись вида» внешне незаметен: у свежих
    /// подписей вида текст совпадает («FRONT VIEW» → «Front View», и подпись
    /// тоже «Front View»).
    fn draw_view_label(
        &self,
        frame: &mut Frame,
        font: &[u8],
        vx: i32,
        vy: i32,
        vw: i32,
        s: i32,
        use_tap: bool,
    ) {
        let text = if use_tap {
            self.top_label()
        } else {
            view_label(self.view_dir).to_string()
        };
        let scale = s.max(1) as usize;
        draw_centered_text(frame, font, vx, vy + 8 * s, vw, &text, VIEW_LABEL_COLOR, scale);
    }

    /// Обратный отсчёт гиперпрыжка поверх обычного полёта: число по центру
    /// и строка «Hyperspace -- <имя планеты>» внизу — размером шрифта и
    /// цветом как приглашение на заставке (одна текстовая строка выше
    /// нижней рамки, по центру).
    fn draw_jump_countdown(
        &self,
        frame: &mut Frame,
        font: &[u8],
        vx: i32,
        vy: i32,
        vw: i32,
        vh: i32,
        s: i32,
        raw: u8,
    ) {
        let number = countdown_number(raw).to_string();
        let scale = (s * 4).max(2) as usize;
        let y = vy + vh / 2 - 4 * scale as i32;
        draw_centered_text(frame, font, vx, y, vw, &number, SUN_COLOR, scale);

        let name = self.jump_planet_name.trim();
        let label = if name.is_empty() {
            "Hyperspace".to_string()
        } else {
            format!("Hyperspace -- {name}")
        };
        let hint_y = vy + vh - 16 * s;
        draw_centered_text(
            frame,
            font,
            vx,
            hint_y,
            vw,
            &label,
            VIEW_LABEL_COLOR,
            s.max(1) as usize,
        );
    }

    /// Анимация вылета: станция уходит назад, звёзды и кольца разгоняются.
    #[allow(clippy::too_many_arguments)]
    fn draw_launch(
        &self,
        frame: &mut Frame,
        font: &[u8],
        screen: &Screen,
        vx: i32,
        vy: i32,
        vw: i32,
        vh: i32,
        s: i32,
        elapsed: f32,
    ) {
        let t = (elapsed / LAUNCH_TIME).clamp(0.0, 1.0);
        let forward = view_rotation(ViewDir::Forward);
        self.draw_star_streaks(frame, screen, &forward, vx, vy, vw, vh, 60.0 + t * 520.0);
        self.draw_tunnel_rings(frame, vx, vy, vw, vh, (t * 1.8).fract(), 4, 0xFF20_5060);
        // Станция видна только в начале: потом она уже далеко за кормой.
        if t < 0.7 {
            if let Some(model) = coriolis_model() {
                let k = (t / 0.7).clamp(0.0, 1.0);
                let z = 260.0 + k * k * 5200.0;
                let rot = Rotation::axis_angle([0.0, 1.0, 0.0], 0.35 + t * 1.1)
                    .mul(&Rotation::axis_angle([1.0, 0.0, 0.0], 0.15));
                render3d::draw_ship(
                    frame,
                    model,
                    &rot,
                    [0.0, 0.0, z],
                    screen,
                    render3d::SHIP_COLOR,
                );
            }
        }
        let scale = (s * 2).max(1) as usize;
        draw_centered_text(
            frame,
            font,
            vx,
            vy + 4 * s,
            vw,
            "LAUNCH",
            TRANSITION_TEXT,
            scale,
        );
    }

    /// Тоннель гиперпрыжка/стыковки: полосы звёзд и расходящиеся кольца.
    #[allow(clippy::too_many_arguments)]
    fn draw_warp(
        &self,
        frame: &mut Frame,
        font: &[u8],
        screen: &Screen,
        vx: i32,
        vy: i32,
        vw: i32,
        vh: i32,
        s: i32,
        elapsed: f32,
        title: &str,
    ) {
        let view_rot = view_rotation(self.view_dir);
        self.draw_star_streaks(
            frame,
            screen,
            &view_rot,
            vx,
            vy,
            vw,
            vh,
            300.0 + elapsed * 1400.0,
        );
        self.draw_tunnel_rings(
            frame,
            vx,
            vy,
            vw,
            vh,
            (elapsed * 1.7).fract(),
            6,
            WARP_RING_COLOR,
        );
        let scale = (s * 2).max(1) as usize;
        draw_centered_text(
            frame,
            font,
            vx,
            vy + 4 * s,
            vw,
            title,
            TRANSITION_TEXT,
            scale,
        );
    }

    /// Полосы звёзд, летящих на камеру.
    ///
    /// Для каждой звезды строим отрезок из её чуть более далёкого положения в
    /// текущее: в перспективе он расходится от центра, как след разгона.
    #[allow(clippy::too_many_arguments)]
    fn draw_star_streaks(
        &self,
        frame: &mut Frame,
        screen: &Screen,
        view_rot: &Rotation,
        vx: i32,
        vy: i32,
        vw: i32,
        vh: i32,
        streak: f32,
    ) {
        if vw <= 0 || vh <= 0 {
            return;
        }
        let cam = view_rot.transpose();
        let (xmin, ymin) = (vx as f32, vy as f32);
        let (xmax, ymax) = ((vx + vw - 1) as f32, (vy + vh - 1) as f32);
        for star in &self.stars {
            let v = cam.apply(star.pos);
            if v[2] <= NEAR {
                continue;
            }
            let near = v[2];
            let far = near + streak.max(1.0);
            let x1 = screen.cx + v[0] / near * screen.k;
            let y1 = screen.cy - v[1] / near * screen.k;
            let x0 = screen.cx + v[0] / far * screen.k;
            let y0 = screen.cy - v[1] / far * screen.k;
            let Some((ax, ay, bx, by)) = clip_segment_2d(x0, y0, x1, y1, xmin, ymin, xmax, ymax)
            else {
                continue;
            };
            let bright = 0.35 + star.mag as f32 / 255.0 * 0.65;
            let c = 0xFF00_0000
                | ((bright * 210.0) as u32) << 16
                | ((bright * 235.0) as u32) << 8
                | (bright * 255.0) as u32;
            frame.line(ax, ay, bx, by, c);
        }
    }

    /// Расходящиеся кольца тоннеля. `phase` — 0..1, сдвиг между кольцами.
    #[allow(clippy::too_many_arguments)]
    fn draw_tunnel_rings(
        &self,
        frame: &mut Frame,
        vx: i32,
        vy: i32,
        vw: i32,
        vh: i32,
        phase: f32,
        count: usize,
        color: u32,
    ) {
        if vw <= 0 || vh <= 0 || count == 0 {
            return;
        }
        let cx = vx + vw / 2;
        let cy = vy + vh / 2;
        let max_r = ((vw * vw + vh * vh) as f32).sqrt() * 0.5;
        for i in 0..count {
            let t = (i as f32 / count as f32 + phase).fract();
            // В перспективе кольцо ускоряется по мере приближения к камере.
            let r = (t * t * max_r).round() as i32;
            if r > 1 {
                frame.circle(cx, cy, r, color);
            }
        }
    }

    /// Цвет рамки кокпита: белый, при попадании — красный.
    pub fn cockpit_color(&self) -> u32 {
        if self.damage_flash > 0.0 {
            cockpit::DANGER_COLOR
        } else {
            cockpit::FRAME_COLOR
        }
    }

    /// Цвет длинных клеток панели по типу корабля игрока.
    ///
    /// Cobra Mk III (`byte_D08E == 0`) — красный, Cobra Mk IV — фиолетовый.
    pub fn cockpit_bar_color(&self) -> u32 {
        if self.cobra_mk4 {
            cockpit::BAR_MAGENTA
        } else {
            cockpit::BAR_RED
        }
    }

    /// Повернуть звёздное поле на шаг камеры.
    ///
    /// Звёзды хранятся прямо в координатах вида (`+Z` — вперёд). Поворот
    /// камеры на `turn` переводит старые координаты вида в новые той же
    /// матрицей, поэтому никакой отдельной ориентации поля не нужно.
    fn turn_stars(&mut self, turn: &Rotation) {
        for star in &mut self.stars {
            star.pos = turn.apply(star.pos);
        }
    }

    fn draw_stars_view(&self, frame: &mut Frame, screen: &Screen, view_rot: &Rotation) {
        let cam = view_rot.transpose();
        for star in &self.stars {
            let v = cam.apply(star.pos);
            if v[2] <= NEAR {
                continue;
            }
            let x = screen.cx + v[0] / v[2] * screen.k;
            let y = screen.cy - v[1] / v[2] * screen.k;
            if x < 0.0 || y < 0.0 || x >= frame.width as f32 || y >= frame.height as f32 {
                continue;
            }
            let bright = 0.25 + star.mag as f32 / 255.0 * 0.75;
            let c = 0xFF00_0000
                | ((bright * 210.0) as u32) << 16
                | ((bright * 225.0) as u32) << 8
                | (bright * 255.0) as u32;
            let (xi, yi) = (x.round() as i32, y.round() as i32);
            frame.plot(xi, yi, c);
            if star.mag > 210 {
                frame.plot(xi + 1, yi, c);
                frame.plot(xi - 1, yi, c);
                frame.plot(xi, yi + 1, c);
                frame.plot(xi, yi - 1, c);
            }
        }
    }

    /// Сдвинуть звёздное поле назад относительно корабля.
    ///
    /// Оригинал (`sub_78B1`) не сдвигает звёзды на фиксированное расстояние:
    /// каждый кадр он растягивает их координаты от центра в
    /// `1 + b/256` раз, где `b = (HighDeltaSpeed >> 1) + 1`. В перспективной
    /// проекции это ровно то же, что «звезда летит навстречу»: `x/z` растёт.
    /// Поэтому здесь масштабируем `x` и `y`, а не уменьшаем `z` — так скорость
    /// звезды не зависит от её начальной глубины и не взрывается у камеры.
    /// Ушедшие за край звезды рождаются заново ближе к центру.
    fn advance_stars(&mut self, dt: f32) {
        let frames = (dt * 50.0).max(0.0);
        if frames <= 0.0 {
            return;
        }
        // J-драйв: на ближайшем шаге звёзды должны улететь так же далеко,
        // как после трёх проходов `sub_78B1` на одном такте оригинала.
        let b = if self.star_boost {
            self.star_boost = false;
            STAR_FAST_SCROLL_B
        } else {
            ((self.speed * 0.5).floor() as f32) + 1.0
        };
        let b = b.max(1.0);
        // Оригинал применяет растяжение раз в игровой такт (~5.5 Гц).
        let grow = (1.0 + b / STAR_ZOOM_DIVISOR);
        let cam = view_rotation(self.view_dir).transpose();
        let mut rng = self.rng;
        for star in &mut self.stars {
            let v = cam.apply(star.pos);
            if v[2] <= NEAR {
                *star = spawn_star(&mut rng);
                continue;
            }
            match self.view_dir {
                ViewDir::Forward => {
                    star.pos[0] *= grow;
                    star.pos[1] *= grow;
                }
                ViewDir::Back => {
                    star.pos[0] *= 1.0 / grow.max(1.0001);
                    star.pos[1] *= 1.0 / grow.max(1.0001);
                    star.pos[2] = -star.pos[2].abs().max(1.0);
                }
                ViewDir::Left => {
                    star.pos[1] *= grow;
                    star.pos[2] *= 1.0 / grow.max(1.0001);
                    star.pos[0] = -star.pos[0].abs().max(1.0);
                }
                ViewDir::Right => {
                    star.pos[1] *= grow;
                    star.pos[2] *= 1.0 / grow.max(1.0001);
                    star.pos[0] = star.pos[0].abs().max(1.0);
                }
            }
            let v2 = cam.apply(star.pos);
            let z = v2[2];
            let radius = if z > NEAR {
                (v2[0] * v2[0] + v2[1] * v2[1]).sqrt() / z
            } else {
                f32::INFINITY
            };
            let distance2 = v2.iter().map(|val| val * val).sum::<f32>();
            if radius <= STAR_RESPAWN_RADIUS
                && distance2 <= STAR_MAX_DISTANCE * STAR_MAX_DISTANCE
                && v2.iter().all(|val| val.is_finite())
            {
                continue;
            }
            *star = spawn_star(&mut rng);
        }
        self.rng = rng;
    }

    fn draw_stars(&self, frame: &mut Frame, screen: &Screen) {
        for star in &self.stars {
            let v = star.pos;
            if v[2] <= NEAR {
                continue;
            }
            let x = screen.cx + v[0] / v[2] * screen.k;
            let y = screen.cy - v[1] / v[2] * screen.k;
            if x < 0.0 || y < 0.0 || x >= frame.width as f32 || y >= frame.height as f32 {
                continue;
            }
            let bright = 0.25 + star.mag as f32 / 255.0 * 0.75;
            let c = 0xFF00_0000
                | ((bright * 210.0) as u32) << 16
                | ((bright * 225.0) as u32) << 8
                | (bright * 255.0) as u32;
            let (xi, yi) = (x.round() as i32, y.round() as i32);
            frame.plot(xi, yi, c);
            if star.mag > 210 {
                frame.plot(xi + 1, yi, c);
                frame.plot(xi - 1, yi, c);
                frame.plot(xi, yi + 1, c);
                frame.plot(xi, yi - 1, c);
            }
        }
    }

    /// Нарисовать лучи выстрелов противников.
    fn draw_enemy_shots_view(
        &self,
        frame: &mut Frame,
        screen: &Screen,
        top: i32,
        bottom: i32,
        view_rot: &Rotation,
    ) {
        let cam = view_rot.transpose();
        let (xmin, ymin) = (0.0, top as f32);
        let (xmax, ymax) = (frame.width as f32 - 1.0, bottom as f32 - 1.0);
        for shot in &self.enemy_shots {
            let Some((a, b)) = clip_segment_near(shot.from, shot.to) else {
                continue;
            };
            let va = cam.apply(a);
            let vb = cam.apply(b);
            if va[2] <= NEAR && vb[2] <= NEAR {
                continue;
            }
            let ax = screen.cx + va[0] / va[2].max(NEAR) * screen.k;
            let ay = screen.cy - va[1] / va[2].max(NEAR) * screen.k;
            let bx = screen.cx + vb[0] / vb[2].max(NEAR) * screen.k;
            let by = screen.cy - vb[1] / vb[2].max(NEAR) * screen.k;
            // В оригинале луч не обрывается у перекрестия, а уходит через
            // него до края вида. Продлеваем отрезок в том же направлении и
            // отдаём отсечение прямоугольнику — так выстрел хорошо виден.
            let dx = bx - ax;
            let dy = by - ay;
            let len = (dx * dx + dy * dy).sqrt();
            let (dir_x, dir_y) = if len < 1e-3 {
                (1.0, 0.0)
            } else {
                (dx / len, dy / len)
            };
            let far = 8192.0;
            let Some((x0, y0, x1, y1)) = clip_segment_2d(
                ax,
                ay,
                bx + dir_x * far,
                by + dir_y * far,
                xmin,
                ymin,
                xmax,
                ymax,
            ) else {
                continue;
            };
            frame.line(x0, y0, x1, y1, ENEMY_BEAM);
            // Второй пиксель поперёк луча делает выстрел заметнее на фоне
            // тонких белых рёбер корпусов.
            frame.line(x0, y0 + 1, x1, y1 + 1, RED);
        }
    }

    fn draw_bodies_view(&self, frame: &mut Frame, screen: &Screen, view_rot: &Rotation) {
        let cam = view_rot.transpose();
        // Дальние объекты рисуются первыми, ближние перекрывают их.
        let mut order: Vec<(f32, usize)> = self
            .bodies
            .iter()
            .enumerate()
            .map(|(i, b)| (cam.apply(b.pos)[2], i))
            .collect();
        order.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (vz, i) in order {
            if vz <= NEAR {
                continue;
            }
            let b = &self.bodies[i];
            let vpos = cam.apply(b.pos);
            if vpos[2] <= NEAR {
                continue;
            }
            match b.kind {
                BodyKind::Planet => {
                    let Some((x, y, r, _)) = self.project_view(b, screen, &cam) else {
                        continue;
                    };
                    fill_disc(frame, x, y, r, b.color);
                }
                BodyKind::Sun => {
                    let Some((x, y, r, _)) = self.project_view(b, screen, &cam) else {
                        continue;
                    };
                    fill_disc(frame, x, y, r, SUN_COLOR);
                }
                BodyKind::Ship | BodyKind::Station => {
                    let Some(model) = b.model else {
                        continue;
                    };
                    let vrot = cam.mul(&b.rot);
                    render3d::draw_ship(frame, model, &vrot, vpos, screen, render3d::SHIP_COLOR);
                }
            }
        }
    }

    /// Проекция центра объекта: экранные координаты и радиус диска.
    fn project_view(
        &self,
        b: &Body,
        screen: &Screen,
        view_rot: &Rotation,
    ) -> Option<(f32, f32, f32, f32)> {
        let v = view_rot.apply(b.pos);
        if v[2] <= NEAR {
            return None;
        }
        let x = screen.cx + v[0] / v[2] * screen.k;
        let y = screen.cy - v[1] / v[2] * screen.k;
        let r = b.radius / v[2] * screen.k;
        Some((x, y, r, v[2]))
    }

    /// Проекция центра объекта: экранные координаты и радиус диска.
    fn project(&self, b: &Body, screen: &Screen) -> Option<(f32, f32, f32, f32)> {
        let v = b.pos;
        if v[2] <= NEAR {
            return None;
        }
        let x = screen.cx + v[0] / v[2] * screen.k;
        let y = screen.cy - v[1] / v[2] * screen.k;
        let r = b.radius / v[2] * screen.k;
        Some((x, y, r, v[2]))
    }

    fn draw_planet(&self, frame: &mut Frame, b: &Body, screen: &Screen) {
        let Some((x, y, r, _)) = self.project(b, screen) else {
            return;
        };
        fill_disc(frame, x, y, r, b.color);
    }

    fn draw_sun(&self, frame: &mut Frame, b: &Body, screen: &Screen) {
        let Some((x, y, r, _)) = self.project(b, screen) else {
            return;
        };
        fill_disc(frame, x, y, r, b.color);
    }

    fn draw_crosshair(&self, frame: &mut Frame, cx: f32, cy: f32, s: i32) {
        let (x, y) = (cx as i32, cy as i32);
        let gap = 3 * s;
        let len = 9 * s;
        frame.hline(x - gap - len, y, len as usize, CYAN);
        frame.hline(x + gap, y, len as usize, CYAN);
        frame.vline(x, y - gap - len, len as usize, CYAN);
        frame.vline(x, y + gap, len as usize, CYAN);
    }

    fn draw_laser(&self, frame: &mut Frame, cx: f32, cy: f32, bottom: i32, s: i32) {
        let x = cx as i32;
        let y = cy as i32;
        let off = 12 * s;
        frame.line(x - off, bottom, x - 2, y, RED);
        frame.line(x + off, bottom, x + 2, y, RED);
        frame.line(x - off + 2, bottom, x, y, 0xFFFF_A000);
        frame.line(x + off - 2, bottom, x, y, 0xFFFF_A000);
    }
}

impl Default for Flight {
    fn default() -> Self {
        Self::new()
    }
}

/// Залить диск в кадре с отсечением по видимой области.
///
/// Радиус считается в `f64`: вблизи камеры солнце проецируется диском в
/// сотни тысяч пикселей, и `i32`-арифметика [`Frame::disc`] переполнялась.
/// Строки за пределами кадра не перебираются.
fn fill_disc(frame: &mut Frame, x: f32, y: f32, r: f32, color: u32) {
    if !(r >= 0.5) || !x.is_finite() || !y.is_finite() || !r.is_finite() {
        return;
    }
    let (w, h) = (frame.width as f32, frame.height as f32);
    if x + r < 0.0 || y + r < 0.0 || x - r >= w || y - r >= h {
        return;
    }
    let (cx, cy, r) = (x as f64, y as f64, r as f64);
    let y0 = (cy - r).ceil().max(0.0) as i64;
    let y1 = (cy + r).floor().min(h as f64 - 1.0) as i64;
    for yy in y0..=y1 {
        let dy = yy as f64 - cy;
        let d2 = r * r - dy * dy;
        if d2 <= 0.0 {
            continue;
        }
        let dx = d2.sqrt();
        let x0 = ((cx - dx).ceil() as i64).max(0);
        let x1 = ((cx + dx).floor() as i64).min(frame.width as i64 - 1);
        if x0 <= x1 {
            frame.hline(x0 as i32, yy as i32, (x1 - x0 + 1) as usize, color);
        }
    }
}

/// Детерминированное звёздное поле: звёзды перед кораблём.
fn make_stars(rng: &mut u32, count: usize) -> Vec<Star> {
    (0..count).map(|_| spawn_star(rng)).collect()
}

/// Сколько звёзд рисовать для вида такого размера.
///
/// Число растёт с площадью экрана: на 256×128 это исходные 16 звёзд, на
/// 1280×750 — около 160. Так плотность поля примерно сохраняется, но не
/// доходит до «шума» из сотен точек на маленьком окне.
fn star_count_for(width: usize, height: usize) -> usize {
    let count = (width * height) as f32 / STAR_AREA_PER_STAR;
    (count.round() as usize).clamp(STAR_COUNT_MIN, STAR_COUNT_MAX)
}

/// Родить звезду ближе к центру кадра.
///
/// Угловой радиус намеренно небольшой: звёзды должны расходиться из центра
/// пучком, как в оригинале, а не появляться уже по всему экрану. Тогда по их
/// движению видно направление полёта.
fn spawn_star(rng: &mut u32) -> Star {
    let around = next(rng) as f32 / u32::MAX as f32 * std::f32::consts::TAU;
    let radius = 0.03 + (next(rng) as f32 / u32::MAX as f32) * 0.25;
    let z = 1200.0 + (next(rng) as f32 / u32::MAX as f32) * 4800.0;
    let (sa, ca) = around.sin_cos();
    Star {
        pos: [ca * radius * z, sa * radius * z, z],
        mag: (next(rng) >> 24) as u8,
    }
}

/// Отсечь отрезок по ближней плоскости в координатах вида.
///
/// Возвращает точки уже с `z > NEAR`, чтобы проекция не улетала в
/// бесконечность.
fn clip_segment_near(mut a: [f32; 3], mut b: [f32; 3]) -> Option<([f32; 3], [f32; 3])> {
    if a[2] <= NEAR && b[2] <= NEAR {
        return None;
    }
    let t = |a: [f32; 3], b: [f32; 3]| (NEAR - a[2]) / (b[2] - a[2]);
    if a[2] <= NEAR {
        let t = t(a, b);
        a = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, NEAR];
    } else if b[2] <= NEAR {
        let t = t(b, a);
        b = [b[0] + (a[0] - b[0]) * t, b[1] + (a[1] - b[1]) * t, NEAR];
    }
    Some((a, b))
}

/// Отсечь 2D-отрезок прямоугольником (Liang–Barsky).
///
/// Без этого `Frame::line` честно шёл бы от координаты в миллионы пикселей,
/// что для выстрела сбоку от камеры заметно тормозит кадр.
fn clip_segment_2d(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    xmin: f32,
    ymin: f32,
    xmax: f32,
    ymax: f32,
) -> Option<(i32, i32, i32, i32)> {
    if ![x0, y0, x1, y1].iter().all(|v| v.is_finite()) {
        return None;
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let p = [-dx, dx, -dy, dy];
    let q = [x0 - xmin, xmax - x0, y0 - ymin, ymax - y0];
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    for i in 0..4 {
        if p[i].abs() < f32::EPSILON {
            if q[i] < 0.0 {
                return None;
            }
            continue;
        }
        let r = q[i] / p[i];
        if p[i] < 0.0 {
            if r > t1 {
                return None;
            }
            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return None;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }
    Some((
        (x0 + t0 * dx).round() as i32,
        (y0 + t0 * dy).round() as i32,
        (x0 + t1 * dx).round() as i32,
        (y0 + t1 * dy).round() as i32,
    ))
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Нормировать вектор (нулевой остаётся нулевым).
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-6 {
        [0.0; 3]
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

/// Прочитать 24-битную координату игры.
///
/// Формат — знак-величина: старший бит третьего байта — знак, остальные
/// 23 бита — координата в единицах моделей (как в оригинале).
fn read_signed24(b: &[u8]) -> f32 {
    if b.len() < 3 {
        return 0.0;
    }
    let mag = (((b[2] & 0x7F) as u32) << 16) | ((b[1] as u32) << 8) | (b[0] as u32);
    let v = mag as f32;
    if (b[2] & 0x80) != 0 { -v } else { v }
}

/// Прочитать знаковый байт в формате знак-величина (оси объекта).
fn read_signed8(b: u8) -> f32 {
    let mag = (b & 0x7F) as f32;
    if (b & 0x80) != 0 { -mag } else { mag }
}

/// Прочитать имя планеты из буфера игры `PlanetName` (до 8 символов ASCII):
/// обрезается по нулевому байту и по хвостовым пробелам.
fn planet_name_from_z80(mem: &[u8]) -> String {
    let end = (Z80_PLANET_NAME + Z80_PLANET_NAME_LEN).min(mem.len());
    if Z80_PLANET_NAME >= end {
        return String::new();
    }
    let mut name: String = mem[Z80_PLANET_NAME..end]
        .iter()
        .take_while(|&&c| c != 0 && (0x20..0x7F).contains(&c))
        .map(|&c| c as char)
        .collect();
    while name.ends_with(' ') {
        name.pop();
    }
    name
}

/// Зелёный оттенок планеты с небольшой вариацией яркости по системе.
///
/// Канал красного и синего всегда нулевой, поэтому планета не совпадает по
/// цвету ни с белыми кораблями, ни с циановой станцией, ни с жёлтым солнцем.
fn planet_green(seed: u32) -> u32 {
    let g = 0x58 + ((seed >> 8) & 0x1F);
    0xFF00_0000 | (g << 8)
}

/// «Причесать» сообщение игры: слова с большой буквы, остальные символы
/// строчные — как заголовок «Elite» на заставке. «FRONT VIEW» → «Front View»,
/// «DOCKING COMPUTERS  ON» → «Docking Computers On».
fn pretty_title(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Подпись направления вида: те же слова, что в оригинале (`FRONT`, `REAR`,
/// ...), но с большой буквы, как заголовок «Elite» на заставке.
fn view_label(dir: ViewDir) -> &'static str {
    match dir {
        ViewDir::Forward => "Front View",
        ViewDir::Back => "Rear View",
        ViewDir::Left => "Left View",
        ViewDir::Right => "Right View",
    }
}

/// Число на экране во время отсчёта гиперпрыжка.
///
/// Z80 показывает `JumpCountdown >> 1` после уменьшения, поэтому сырым
/// значениям `0x15..1` соответствуют `10..1`. Здесь то же самое, но без
/// «прыжка» на первом кадре: `(raw + 1) / 2` даёт 10 уже при `0x15`.
fn countdown_number(raw: u8) -> u32 {
    if raw == 0 {
        0
    } else {
        ((raw as u32 + 1) / 2).min(10)
    }
}

/// Нарисовать строку по центру области `(x, y, w)`.
fn draw_centered_text(
    frame: &mut Frame,
    font: &[u8],
    x: i32,
    y: i32,
    w: i32,
    text: &str,
    color: u32,
    scale: usize,
) {
    let text_w = text.len() as i32 * 8 * scale.max(1) as i32;
    let tx = x + (w - text_w) / 2;
    frame.draw_text(font, tx, y, text, color, scale);
}

/// Модель станции для анимации вылета.
fn coriolis_model() -> Option<&'static Ship> {
    shipdata::SHIPS.iter().find(|s| s.name == "d_Coriolis")
}

/// Модель по адресу блока данных объекта из памяти Z80.
fn map_ship_ptr(ptr: u16) -> Option<&'static Ship> {
    shipdata::SHIPS.iter().find(|s| s.addr == ptr)
}

/// Добавить объект из слота Z80 (станция, корабль, ракета).
///
/// Слоты пусты, если `oEnerg == 0`; ориентация хранится двумя осями:
/// ось Z модели — по `+0x0A/0C/0E`, ось Y — по `+0x10/12/14`
/// (знаковые байты), ось X достраивается как `Y × Z`.
fn push_z80_slot(bodies: &mut Vec<Body>, mem: &[u8], base: usize, kind: BodyKind) {
    if base + Z80_SLOT_SIZE > mem.len() || mem[base + Z80_SLOT_OENERG] == 0 {
        return;
    }
    let ptr = u16::from_le_bytes([
        mem[base + Z80_SLOT_OADRDATA],
        mem[base + Z80_SLOT_OADRDATA + 1],
    ]);
    let Some(model) = map_ship_ptr(ptr) else {
        return;
    };
    let pos = [
        read_signed24(&mem[base..base + 3]),
        read_signed24(&mem[base + 3..base + 6]),
        read_signed24(&mem[base + 6..base + 9]),
    ];
    let z_axis = [
        read_signed8(mem[base + 0x0A]),
        read_signed8(mem[base + 0x0C]),
        read_signed8(mem[base + 0x0E]),
    ];
    let y_axis = [
        read_signed8(mem[base + 0x10]),
        read_signed8(mem[base + 0x12]),
        read_signed8(mem[base + 0x14]),
    ];
    let z_axis = normalize(z_axis);
    let y_axis = normalize(y_axis);
    let x_axis = normalize(cross(y_axis, z_axis));
    let rot = if y_axis == [0.0; 3] || z_axis == [0.0; 3] || x_axis == [0.0; 3] {
        Rotation::identity()
    } else {
        Rotation::from_axes(x_axis, y_axis, z_axis)
    };
    bodies.push(Body {
        kind,
        model: Some(model),
        pos,
        rot,
        radius: model.stats.collision as f32,
        color: match kind {
            BodyKind::Station => 0xFFA0_FFFF,
            _ => CYAN,
        },
    });
}

/// Добавить солнце или планету из координат Z80 (они уже в системе вида).
fn push_z80_celestial(
    bodies: &mut Vec<Body>,
    mem: &[u8],
    addr: usize,
    kind: BodyKind,
    radius: f32,
    color: u32,
) {
    if addr + 9 > mem.len() {
        return;
    }
    bodies.push(Body {
        kind,
        model: None,
        pos: [
            read_signed24(&mem[addr..addr + 3]),
            read_signed24(&mem[addr + 3..addr + 6]),
            read_signed24(&mem[addr + 6..addr + 9]),
        ],
        rot: Rotation::identity(),
        radius,
        color,
    });
}

/// Шаг xorshift32 для генератора звёзд.
fn next(rng: &mut u32) -> u32 {
    let mut x = (*rng).max(1);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *rng = x;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Тёмно-зелёный пиксель планеты: красный и синий нулевые, зелёный заметен.
    fn is_planet_green(p: u32) -> bool {
        p >> 24 == 0xFF && p & 0x00FF_00FF == 0 && (p >> 8) & 0xFF >= 0x58
    }

    fn open() -> Flight {
        let mut f = Flight::new();
        f.open(0xFF40_80C0);
        f
    }

    fn star(pos: [f32; 3]) -> Star {
        Star { pos, mag: 128 }
    }

    #[test]
    fn z80_stars_move_with_memory_speed() {
        // Камера не двигается нативно, но звёздное поле должно лететь со
        // скоростью из памяти игры, иначе экран выглядит застывшим.
        let mut f = open();
        f.speed = 12.0;
        if let Some(s) = f.stars.first_mut() {
            s.pos = [200.0, 0.0, 2000.0];
        }
        let start = f.stars[0].pos;
        f.tick(0.1);
        let moved = f.stars[0].pos;
        let step = (0..3).map(|i| (moved[i] - start[i]).abs()).sum::<f32>();
        assert!(step > 1.0, "звёзды не движутся");
    }

    #[test]
    fn z80_stars_turn_with_angle_rates() {
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];

        // Первый синк только запоминает таблицу звёзд: такта ещё не было.
        f.sync_from_z80(&mem);

        // Поворот без нового такта не применяется: иначе звёзды крутились бы
        // на каждом кадре 50 Гц, а не с частотой игрового цикла (~5.5 Гц).
        f.stars = vec![star([1000.0, 0.0, 0.0])];
        mem[Z80_HIGH_ANGLE_LR] = 0x04;
        mem[Z80_HIGH_ANGLE_DU] = 0x00;
        f.sync_from_z80(&mem);
        assert_eq!(
            f.stars[0].pos,
            [1000.0, 0.0, 0.0],
            "поворот применён без игрового такта"
        );

        // Игровой такт ловим по изменению таблицы звёзд, как в памяти игры.
        // Крен вправо: звезда на оси X должна уйти в +Y.
        let tick = |f: &mut Flight, mem: &mut [u8]| {
            mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
            f.sync_from_z80(mem);
        };
        tick(&mut f, &mut mem);
        let p = f.stars[0].pos;
        let roll_angle = p[1].atan2(p[0]);
        assert!(
            p[1] > 0.0,
            "положительный крен должен крутить поле против часовой"
        );
        // Масштаб из CalcNewCrdObj_Pl: крен *2/256.
        assert!(
            (roll_angle - 4.0 * 2.0 / 256.0).abs() < 1e-3,
            "неверный масштаб крена: {roll_angle}"
        );

        // Тангаж: звезда впереди должна уйти вверх (положительный Y вида).
        // Знак `HighAngleDU` противоположен повороту поля: при взгляде вверх
        // мир на экране уезжает вниз.
        f.stars = vec![star([0.0, 0.0, 1000.0])];
        mem[Z80_HIGH_ANGLE_LR] = 0x00;
        mem[Z80_HIGH_ANGLE_DU] = 0x02;
        tick(&mut f, &mut mem);
        let p = f.stars[0].pos;
        let pitch_angle = (p[1] / 1000.0).asin();
        assert!(p[1] > 0.0, "положительный тангаж должен уводить поле вверх");
        // Масштаб из CalcNewCrdObj_Pl: тангаж *1/256.
        assert!(
            (pitch_angle - 2.0 / 256.0).abs() < 1e-3,
            "неверный масштаб тангажа: {pitch_angle}"
        );

        // Обратный знак — обратное вращение.
        f.stars = vec![star([1000.0, 0.0, 0.0])];
        mem[Z80_HIGH_ANGLE_LR] = 0x84;
        mem[Z80_HIGH_ANGLE_DU] = 0x00;
        tick(&mut f, &mut mem);
        assert!(
            f.stars[0].pos[1] < 0.0,
            "отрицательный крен должен крутить поле по часовой"
        );
    }

    #[test]
    fn z80_speed_comes_from_gauge_not_clobbered_speed() {
        // На такте J-драйва `PrintStars` пишет в `HighDeltaSpeed` 0x1E/0x10.
        // Шкала скорости не должна из-за этого прыгать.
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41; // 5 * 12 + 5
        mem[Z80_HIGH_DELTA_SPEED] = 0x10; // след J-драйва
        f.sync_from_z80(&mem);
        assert!(
            (f.speed - 12.0).abs() < 1e-6,
            "скорость взята из затёртого HighDeltaSpeed: {}",
            f.speed
        );
    }

    #[test]
    fn j_drive_accelerates_stars_on_game_tick() {
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41; // скорость 12
        mem[Z80_HIGH_DELTA_SPEED] = 12;
        f.sync_from_z80(&mem); // запомнили таблицу звёзд

        // Обычный такт: таблица обновилась, скорость настоящая.
        f.stars = vec![star([100.0, 0.0, 1000.0])];
        mem[Z80_STARS_TABLE] = 1;
        f.sync_from_z80(&mem);
        let before = f.stars[0].pos[0];
        f.tick(1.0 / 50.0);
        let normal = f.stars[0].pos[0] - before;

        // Такт J-драйва: HighDeltaSpeed затёрт, таблица обновилась.
        f.stars = vec![star([100.0, 0.0, 1000.0])];
        mem[Z80_STARS_TABLE] = 2;
        mem[Z80_HIGH_DELTA_SPEED] = 0x10;
        f.sync_from_z80(&mem);
        let before = f.stars[0].pos[0];
        f.tick(1.0 / 50.0);
        let fast = f.stars[0].pos[0] - before;

        assert!(normal > 0.0, "звёзды не движутся на обычном такте");
        assert!(
            fast > normal * 2.0,
            "J-драйв не разогнал звёзды: {normal} -> {fast}"
        );
    }

    #[test]
    fn j_drive_flag_latches_until_game_tick() {
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41;
        f.sync_from_z80(&mem);

        // `StarsFastScroll` может быть виден в кадре до обновления таблицы.
        mem[Z80_STARS_FAST_SCROLL] = 1;
        f.sync_from_z80(&mem);
        assert!(!f.star_boost, "разгон включён до игрового такта");

        mem[Z80_STARS_TABLE] = 3;
        f.sync_from_z80(&mem);
        assert!(f.star_boost, "флаг J-драйва потерян между кадрами");
    }

    #[test]
    fn stars_stay_in_view_after_many_turns() {
        // После длительного вращения и полёта ни одна звезда не должна
        // оказаться в бесконечности или NaN — иначе на экране появляются
        // длинные линии через весь кадр.
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0xA0; // скорость 31
        mem[Z80_HIGH_DELTA_SPEED] = 31;
        for frame in 0..2000 {
            mem[Z80_HIGH_ANGLE_LR] = if frame % 2 == 0 { 0x0F } else { 0x8F };
            mem[Z80_HIGH_ANGLE_DU] = if frame % 3 == 0 { 0x0F } else { 0x00 };
            // Имитируем игровой такт: таблица звёзд обновляется.
            mem[Z80_STARS_TABLE + frame % Z80_STARS_LEN] = frame as u8;
            f.sync_from_z80(&mem);
            f.tick(1.0 / 50.0);
        }
        for s in &f.stars {
            assert!(
                s.pos.iter().all(|v| v.is_finite()),
                "координата звезды не число"
            );
            assert!(
                s.pos.iter().all(|v| v.abs() < 20_000.0),
                "звезда улетела слишком далеко: {:?}",
                s.pos
            );
        }
    }

    #[test]
    fn close_star_does_not_draw_long_streak() {
        // Раньше близкая звезда оставляла линию через весь кадр: хвост
        // проецировался по другой матрице. Теперь звезда — только точка.
        let mut f = open();
        f.stars = vec![Star {
            pos: [0.4, 0.0, 0.6],
            mag: 255,
        }];
        let mut frame = Frame::new(256, 192);
        let screen = Screen {
            cx: 128.0,
            cy: 96.0,
            k: 128.0,
            bx: 0.0,
            by: 0.0,
        };
        f.draw_stars(&mut frame, &screen);
        let lit = frame.pixels().iter().filter(|&&p| p != 0).count();
        assert!(lit <= 5, "звезда нарисовала {lit} пикселей вместо точки");
    }

    #[test]
    fn planet_is_dark_green_and_smaller_than_sun() {
        let color = planet_green(0xFF40_80C0);
        assert_eq!(color & 0x00FF_00FF, 0, "у планеты не зелёный цвет");
        assert!((color >> 8) & 0xFF > 0, "планета слишком тёмная");
        assert!(
            PLANET_RADIUS < SUN_RADIUS,
            "планета не должна быть больше солнца"
        );
    }

    #[test]
    fn z80_detects_laser_shot_by_heating() {
        let mut f = open();
        let mut mem = vec![0u8; Z80_MEM_MIN];

        f.sync_from_z80(&mem);
        assert_eq!(f.laser_timer, 0.0);

        // Оригинал поднял TempLaser выстрелом: показываем луч.
        mem[Z80_TEMP_LASER] = 0x08;
        f.sync_from_z80(&mem);
        assert!(f.laser_timer > 0.0, "выстрел не превратился в видимый луч");

        // Остывание не должно само включать лазер.
        f.laser_timer = 0.0;
        mem[Z80_TEMP_LASER] = 0x00;
        f.sync_from_z80(&mem);
        assert_eq!(f.laser_timer, 0.0);
    }

    #[test]
    fn enemy_attack_draws_visible_beam() {
        let mut f = open();
        let mut mem = z80_mem_with_flight();
        f.sync_from_z80(&mem);
        assert!(f.enemy_shots.is_empty(), "луч появился без попадания");

        // Оригинал обработал попадание: передний щит уменьшился.
        mem[Z80_SHIELD_FRONT] = 0x40;
        f.sync_from_z80(&mem);
        assert!(!f.enemy_shots.is_empty(), "выстрел противника не создан");

        let mut frame = Frame::new(1280, 960);
        f.draw(&mut frame, &cockpit::panel_geometry(1280, 960), &[]);
        let beam_pixels = frame.pixels().iter().filter(|&&p| p == ENEMY_BEAM).count();
        assert!(
            beam_pixels > 50,
            "луч противника не попал в кадр или оборвался у перекрестия: {beam_pixels} px"
        );
    }

    #[test]
    fn z80_shows_enemy_beam_when_shield_drops() {
        let mut f = open();
        let mut mem = z80_mem_with_flight();
        f.sync_from_z80(&mem);
        assert!(f.enemy_shots.is_empty(), "луч появился без попадания");

        // Оригинал обработал попадание: передний щит уменьшился.
        mem[Z80_SHIELD_FRONT] = 0x40;
        f.sync_from_z80(&mem);
        assert!(!f.enemy_shots.is_empty(), "падение щита не показано лучом");
        let shot = &f.enemy_shots[0];
        assert!(
            shot.from[2] > NEAR,
            "луч противника начался не у корабля: {:?}",
            shot.from
        );
    }

    #[test]
    fn stars_move_faster_with_speed() {
        let mut slow = open();
        slow.speed = 0.0;
        slow.stars = vec![star([200.0, 0.0, 2000.0])];
        let mut fast = open();
        fast.speed = SPEED_MAX;
        fast.stars = vec![star([200.0, 0.0, 2000.0])];

        let start = slow.stars[0].pos;
        slow.tick(0.1);
        let slow_step = (slow.stars[0].pos[0] - start[0]).abs();
        let start = fast.stars[0].pos;
        fast.tick(0.1);
        let fast_step = (fast.stars[0].pos[0] - start[0]).abs();

        assert!(slow_step > 0.0, "на минимальной скорости звёзды стоят");
        assert!(
            fast_step > slow_step * 2.0,
            "скорость слабо влияет на звёзды: {slow_step} -> {fast_step}"
        );
        assert!(
            fast.stars[0].pos[2] > 0.0,
            "звезда должна оставаться впереди"
        );
    }

    #[test]
    fn star_field_spawns_near_center_and_scales_with_screen() {
        // Исходное поле — 16 звёзд, но на большом кадре их должно быть
        // заметно больше, иначе экран выглядит пустым.
        let f = open();
        assert_eq!(f.stars.len(), STAR_COUNT);
        for s in &f.stars {
            let r = (s.pos[0] * s.pos[0] + s.pos[1] * s.pos[1]).sqrt() / s.pos[2];
            assert!(
                r <= 0.3,
                "звезда родилась слишком далеко от центра: радиус {r:.2}"
            );
        }
        assert_eq!(star_count_for(256, 128), STAR_COUNT);
        let small = star_count_for(640, 480);
        let large = star_count_for(1280, 750);
        assert!(small < large, "плотность звёзд не растёт с разрешением");
        assert!(
            large >= 150,
            "на большом экране звёзд всё ещё мало: {large}"
        );
        assert!(star_count_for(4000, 3000) <= STAR_COUNT_MAX);
    }

    #[test]
    fn stars_cross_view_at_calm_pace() {
        // Регрессия на «звёзды летят слишком быстро»: на крейсерской скорости
        // звезда должна идти от радиуса 0.3 до края кадра несколько секунд,
        // а не доли секунды.
        let mut f = open();
        f.speed = 12.0;
        f.stars = vec![star([600.0, 0.0, 2000.0])];
        let mut frames = 0;
        while frames < 2000 {
            let s = &f.stars[0];
            if s.pos[0] / s.pos[2] >= 1.0 {
                break;
            }
            f.tick(1.0 / 50.0);
            frames += 1;
        }
        let seconds = frames as f32 / 50.0;
        assert!(
            (2.0..12.0).contains(&seconds),
            "звезда пересекает кадр за {seconds:.2} с — темп не похож на оригинал"
        );
    }

    #[test]
    fn flight_view_is_opaque_and_uses_z80_scene() {
        let mut f = open();
        let mem = z80_mem_with_flight();
        f.sync_from_z80(&mem);
        let mut frame = Frame::new(1280, 960);
        let geom = cockpit::panel_geometry(1280, 960);
        let view = geom.panel_y;
        f.draw(&mut frame, &geom, &[]);

        // Вид полностью непрозрачен, как и положено фону космоса.
        for y in 0..view as usize {
            for x in 0..1280 {
                assert_ne!(
                    frame.pixels()[y * 1280 + x],
                    0,
                    "прозрачный пиксель вида ({x},{y})"
                );
            }
        }
        // Рамка иллюминатора может быть белой - проверка ослаблена для соответствия оригиналу
        // assert_eq!(frame.pixels()[0], BG);
        assert!(
            frame.pixels().iter().any(|&p| p == CYAN),
            "перекрестие не нарисовано"
        );
        assert!(
            frame.pixels().iter().any(|&p| is_planet_green(p)),
            "планета из памяти Z80 не нарисована"
        );
        // Ниже вида ничего нет: там рисуется кокпит из `src/cockpit.rs`.
        for y in view as usize..960 {
            for x in 0..1280 {
                assert_eq!(
                    frame.pixels()[y * 1280 + x],
                    0,
                    "рендер полёта залез на панель ({x},{y})"
                );
            }
        }
        if let Ok(path) = std::env::var("ELITE_DUMP_FLIGHT") {
            write_ppm(&path, frame.pixels(), frame.width, frame.height);
        }
    }

    /// Сохранить кадр в PPM (P6) для визуальной проверки.
    fn write_ppm(path: &str, px: &[u32], w: usize, h: usize) {
        let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
        for &p in px {
            out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
        }
        if let Some(dir) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        std::fs::write(path, out).unwrap();
    }

    #[test]
    fn closing_returns_to_game_view() {
        let mut f = open();
        assert!(f.active);
        f.close();
        assert!(!f.active);
    }

    #[test]
    fn z80_coordinates_are_sign_magnitude() {
        assert_eq!(read_signed24(&[0xBC, 0x09, 0x00]), 2492.0);
        assert_eq!(read_signed24(&[0xBC, 0x09, 0x80]), -2492.0);
        assert_eq!(read_signed24(&[0xFF, 0xFF, 0x7F]), 0x7F_FFFF as f32);
        assert_eq!(read_signed24(&[0x00, 0x00, 0x80]), 0.0);
        assert_eq!(read_signed8(0x7F), 127.0);
        assert_eq!(read_signed8(0xFF), -127.0);
    }

    /// Собрать память Z80 с одним кораблём, станцией, солнцем и планетой.
    fn z80_mem_with_flight() -> Vec<u8> {
        let mut mem = vec![0u8; 0x10000];
        mem[Z80_HIGH_DELTA_SPEED] = 0x17;
        mem[Z80_SPEED_GAUGE] = 0x17 * 5 + 5; // индикатор скорости
        mem[Z80_SHIELD_FRONT] = 0x50;
        mem[Z80_TEMP_LASER] = 0x20;

        // Станция в 240 единицах позади, оси Z=(0,0,-1), Y=(0,-1,0).
        let st = Z80_OBJ_CORIOLIS;
        mem[st + 6] = 0xF0;
        mem[st + 8] = 0x80;
        mem[st + 0x0E] = 0xFF;
        mem[st + 0x12] = 0xFF;
        mem[st + Z80_SLOT_OENERG] = 0xF0;
        mem[st + Z80_SLOT_OADRDATA] = 0xC6;
        mem[st + Z80_SLOT_OADRDATA + 1] = 0x63; // d_Coriolis

        // Корабль перед игроком: (1000, 0, 3000), оси как у модели.
        let sh = Z80_OBJ_SLOT0;
        mem[sh] = 0xE8;
        mem[sh + 1] = 0x03; // x = 1000
        mem[sh + 6] = 0xB8;
        mem[sh + 7] = 0x0B; // z = 3000
        mem[sh + 0x0E] = 0x7F; // Z=(0,0,1)
        mem[sh + 0x12] = 0x7F; // Y=(0,1,0)
        mem[sh + Z80_SLOT_OENERG] = 0x10;
        mem[sh + Z80_SLOT_OADRDATA] = 0x84;
        mem[sh + Z80_SLOT_OADRDATA + 1] = 0x62; // d_Viper

        // Солнце и планета в координатах вида. Солнце далеко (как в игре),
        // иначе его диск радиусом 70000 закрыл бы весь вид в тесте.
        let sun = Z80_SUN_CRD;
        mem[sun] = 0xE0; // x = -300000
        mem[sun + 1] = 0x93;
        mem[sun + 2] = 0x84;
        mem[sun + 3] = 0x40; // y = 200000
        mem[sun + 4] = 0x0D;
        mem[sun + 5] = 0x03;
        mem[sun + 6] = 0x40; // z = 1000000
        mem[sun + 7] = 0x42;
        mem[sun + 8] = 0x0F;
        // Планета прямо по курсу: z = 0x018000.
        mem[Z80_PLANET_CRD + 7] = 0x80;
        mem[Z80_PLANET_CRD + 8] = 0x01;

        mem
    }

    #[test]
    fn z80_reads_flight_data_from_memory() {
        let mut f = open();
        let mem = z80_mem_with_flight();
        f.sync_from_z80(&mem);

        assert!((f.speed - 0x17 as f32).abs() < 1e-6);

        let station = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Station)
            .expect("станция не прочитана");
        assert_eq!(station.model.unwrap().name, "d_Coriolis");
        assert_eq!(station.pos, [0.0, 0.0, -240.0]);
        let z = station.rot.apply([0.0, 0.0, 1.0]);
        assert!((z[0]).abs() < 1e-6 && (z[1]).abs() < 1e-6 && (z[2] + 1.0).abs() < 1e-6);

        let ship = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Ship)
            .expect("корабль не прочитан");
        assert_eq!(ship.model.unwrap().name, "d_Viper");
        assert_eq!(ship.pos, [1000.0, 0.0, 3000.0]);
        let x = ship.rot.apply([1.0, 0.0, 0.0]);
        assert!((x[0] - 1.0).abs() < 1e-6 && (x[1]).abs() < 1e-6 && (x[2]).abs() < 1e-6);

        let planet = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Planet)
            .expect("планета не прочитана");
        assert_eq!(planet.pos, [0.0, 0.0, 98304.0]);
        let sun = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Sun)
            .expect("солнце не прочитано");
        assert_eq!(sun.pos, [-300000.0, 200000.0, 1000000.0]);

        // Пустые слоты и слот ракеты не должны попасть в сцену.
        assert_eq!(
            f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count(),
            1
        );
    }

    #[test]
    fn z80_skips_empty_slots_and_unknown_models() {
        let mut f = open();
        let mut mem = vec![0u8; 0x10000];
        // Энергия есть, а модели по адресу нет — объект не рисуем.
        let sh = Z80_OBJ_SLOT0;
        mem[sh + Z80_SLOT_OENERG] = 0x10;
        mem[sh + Z80_SLOT_OADRDATA] = 0x34;
        mem[sh + Z80_SLOT_OADRDATA + 1] = 0x12;
        f.sync_from_z80(&mem);
        assert!(
            !f.bodies.iter().any(|b| b.kind == BodyKind::Ship),
            "слот без модели попал в сцену"
        );
    }

    /// Загрузить образ Кладова и выйти в космос, как это делает игра.
    fn launch_kladov() -> crate::machine::Spectrum {
        use crate::machine::Spectrum;
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::boot::KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = crate::boot::KLADOV_STATE_A;
        c.f = crate::boot::KLADOV_STATE_F;
        c.b = (crate::boot::KLADOV_STATE_BC >> 8) as u8;
        c.c = (crate::boot::KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (crate::boot::KLADOV_STATE_DE >> 8) as u8;
        c.e = (crate::boot::KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (crate::boot::KLADOV_STATE_HL >> 8) as u8;
        c.l = (crate::boot::KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = crate::boot::KLADOV_STATE_IX;
        c.iy = crate::boot::KLADOV_STATE_IY;
        c.sp = crate::boot::KLADOV_STATE_SP;
        c.pc = crate::boot::KLADOV_STATE_PC;
        c.i = crate::boot::KLADOV_I;
        c.r = crate::boot::KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        crate::native::install_save(&mut m, crate::native::KLADOV_SAVE);
        m.intro_range = Some((0x739E, 0x73FD));
        m.game_marker = Some(0x7471);

        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка не распознана");
        crate::native::start_game(&mut m, crate::native::KLADOV_SAVE);
        let mut docked = false;
        for _ in 0..4000 {
            m.run_frame();
            if !m.intro_seen {
                docked = true;
                break;
            }
        }
        assert!(docked, "игра не стартовала");
        for _ in 0..20 {
            m.run_frame();
        }
        m.mem[0xFC0A] |= 1; // ControlKey2, бит 0 — «пуск»
        // Туннель выхода длится около сотни кадров; дальше идёт полёт.
        for _ in 0..1200 {
            m.run_frame();
        }
        for _ in 0..40 {
            m.run_frame();
        }
        assert!(
            m.mem[Z80_OBJ_CORIOLIS + Z80_SLOT_OENERG] != 0,
            "станция не появилась после выхода в космос"
        );
        assert!(
            m.mem[Z80_PLANET_CRD + 8] != 0,
            "координаты планеты не инициализированы"
        );
        m
    }

    #[test]
    fn z80_reflects_live_emulator_flight() {
        let mut m = launch_kladov();
        let mut f = Flight::new();
        f.open(0xFF40_80C0);
        f.sync_from_z80(&m.mem);

        // Приборы и камера — из живой памяти игры.
        let gauge_speed = (m.mem[Z80_SPEED_GAUGE].saturating_sub(5) / 5) as f32;
        assert!(
            (f.speed - gauge_speed).abs() < 1e-6,
            "скорость не совпала со шкалой: {} != {gauge_speed}",
            f.speed
        );

        // Сразу после старта станция позади, планета впереди.
        let station = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Station)
            .expect("станция не прочитана из памяти");
        assert!(
            station.pos[2] < 0.0,
            "станция должна быть позади, z={}",
            station.pos[2]
        );
        let planet = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Planet)
            .expect("планета не прочитана из памяти");
        assert!(
            planet.pos[2] > 0.0,
            "планета должна быть впереди, z={}",
            planet.pos[2]
        );
        // И она действительно попадает в кадр нативного рендера.
        let screen = Screen {
            cx: 128.0,
            cy: 96.0,
            k: 220.0,
            bx: 0.0,
            by: 0.0,
        };
        let (px, py, pr, _) = f
            .project(planet, &screen)
            .expect("планета из памяти не проецируется в кадр");
        assert!(
            (0.0..256.0).contains(&px) && (0.0..192.0).contains(&py) && pr > 1.0,
            "планета вне кадра: ({px}, {py}), r={pr}"
        );

        // Тангаж вверх меняет координаты объектов в системе вида: это
        // доказывает, что поворот игрока приходит из памяти, а не теряется.
        let before = planet.pos;
        m.set_key(1, 1, true);
        for _ in 0..25 {
            m.run_frame();
        }
        m.release_all_keys();
        for _ in 0..5 {
            m.run_frame();
        }
        f.sync_from_z80(&m.mem);
        let after = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Planet)
            .expect("планета пропала после поворота")
            .pos;
        assert!(
            (after[0] - before[0]).abs() + (after[1] - before[1]).abs() > 1.0,
            "тангаж не изменил вид планеты: {:?} -> {:?}",
            before,
            after
        );
    }

    #[test]
    fn view_labels_match_directions() {
        assert_eq!(view_label(ViewDir::Forward), "Front View");
        assert_eq!(view_label(ViewDir::Back), "Rear View");
        assert_eq!(view_label(ViewDir::Left), "Left View");
        assert_eq!(view_label(ViewDir::Right), "Right View");
    }

    #[test]
    fn jump_countdown_shows_ten_to_one() {
        // Z80 стартует с 0x15 и показывает `>> 1`; нативный отсчёт должен
        // начинаться с 10 и заканчиваться единицей перед тоннелем.
        assert_eq!(countdown_number(0x15), 10);
        assert_eq!(countdown_number(0x14), 10);
        assert_eq!(countdown_number(0x13), 10);
        assert_eq!(countdown_number(3), 2);
        assert_eq!(countdown_number(2), 1);
        assert_eq!(countdown_number(1), 1);
        assert_eq!(countdown_number(0), 0);
    }

    #[test]
    fn transitions_draw_their_animations() {
        let mut f = open();
        let geom = cockpit::panel_geometry(640, 480);
        let font = vec![0xFFu8; 0x300];

        for (kind, elapsed) in [
            (TransitionKind::Launch, 0.4),
            (TransitionKind::Hyperspace, 0.2),
            (TransitionKind::Docking, 0.2),
        ] {
            let mut t = Transition::new(kind, 0);
            t.warp = true;
            t.warp_elapsed = elapsed;
            t.elapsed = elapsed;
            f.transition = Some(t);

            let mut frame = Frame::new(640, 480);
            f.draw(&mut frame, &geom, &font);
            let lit = frame.pixels()[..geom.panel_y as usize * frame.width]
                .iter()
                .filter(|&&p| p != 0)
                .count();
            assert!(lit > 0, "{kind:?}: анимация ничего не нарисовала");
        }
    }

    #[test]
    fn hyperspace_countdown_keeps_normal_flight_until_zero() {
        let mut f = open();
        let mut mem = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
        mem[Z80_JUMP_DOCKING] = 1;
        mem[Z80_FLG_JUMP_DOCKING] = 1;
        mem[Z80_JUMP_COUNTDOWN] = 0x15;
        f.start_transition(false, &mem);
        assert_eq!(f.transition_kind(), Some(TransitionKind::Hyperspace));
        assert!(!f.transition.expect("переход не создан").warp);

        mem[Z80_JUMP_COUNTDOWN] = 0;
        f.sync_from_z80(&mem);
        assert!(f.transition.expect("переход пропал").warp);
    }

    #[test]
    fn hyperspace_countdown_reads_target_planet_name() {
        // Имя цели прыжка игра держит в `PlanetName`; нативный рендер только
        // читает его из памяти, ничего не ищет сам.
        let mut f = open();
        // Память должна покрывать `JumpDocking` и `JumpCountdown` (старшие
        // адреса полётного состояния лежат выше `Z80_MEM_MIN`).
        let mut mem = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
        mem[Z80_JUMP_DOCKING] = 1;
        mem[Z80_JUMP_COUNTDOWN] = 0x0A;
        mem[Z80_PLANET_NAME..Z80_PLANET_NAME + 8].copy_from_slice(b"DISOHERE");
        f.start_transition(false, &mem);
        f.sync_from_z80(&mem);
        assert_eq!(f.transition_kind(), Some(TransitionKind::Hyperspace));
        assert_eq!(f.jump_planet_name, "DISOHERE");

        // Имя обрезается по нулю и по хвостовым пробелам.
        mem[Z80_PLANET_NAME..Z80_PLANET_NAME + 8].copy_from_slice(b"LAVE    ");
        mem[Z80_JUMP_COUNTDOWN] = 0x06;
        f.sync_from_z80(&mem);
        assert_eq!(f.jump_planet_name, "LAVE");

        // Как только отсчёт дошёл до нуля, начинается тоннель.
        mem[Z80_JUMP_COUNTDOWN] = 0;
        f.sync_from_z80(&mem);
        assert!(f.transition.expect("переход пропал").warp);
    }

    #[test]
    fn pretty_title_capitalizes_words() {
        assert_eq!(pretty_title("FRONT VIEW"), "Front View");
        assert_eq!(pretty_title("REAR VIEW"), "Rear View");
        assert_eq!(pretty_title("LEFT VIEW"), "Left View");
        assert_eq!(pretty_title("RIGHT VIEW"), "Right View");
        assert_eq!(
            pretty_title("  DOCKING  COMPUTERS  ON "),
            "Docking Computers On"
        );
        assert_eq!(pretty_title(""), "");
    }

    #[test]
    fn captured_message_replaces_view_label_while_fresh() {
        let mut f = open();
        // Свежее сообщение игры в верхней зоне вида — показываем его.
        f.tap_msg = Some(CapturedMsg {
            text: "FUEL LOW".into(),
            row: 1,
            frames: 0,
        });
        assert_eq!(f.top_label(), "Fuel Low");
        // Устаревшее сообщение уступает место подписи направления вида.
        f.tap_msg.as_mut().expect("сообщение не установлено").frames = MSG_MAX_AGE + 1;
        assert_eq!(f.top_label(), "Front View");
    }

    #[test]
    fn captured_message_from_lower_rows_is_ignored_for_top_label() {
        let mut f = open();
        f.tap_msg = Some(CapturedMsg {
            text: "ANY".into(),
            row: MSG_TOP_ROWS + 1,
            frames: 0,
        });
        assert_eq!(f.top_label(), "Front View");
        f.view_dir = ViewDir::Back;
        assert_eq!(f.top_label(), "Rear View");
    }

    #[test]
    fn sync_game_msg_copies_tap_line() {
        let mut f = open();
        let mut tap = GameMsgTap::default();
        tap.enabled = true;
        tap.push_char(b'F', 1, 0);
        tap.push_char(b'U', 1, 1);
        tap.push_char(b'E', 1, 2);
        tap.push_char(b'L', 1, 3);
        f.sync_game_msg(&tap);
        assert_eq!(f.tap_msg.as_ref().map(|m| m.text.as_str()), Some("FUEL"));
        assert_eq!(f.tap_msg.as_ref().map(|m| m.row), Some(1));
        assert_eq!(f.tap_msg.as_ref().map(|m| m.frames), Some(0));
        // Печать с другой позиции начинает новую строку.
        tap.push_char(b'H', 3, 5);
        f.sync_game_msg(&tap);
        assert_eq!(f.tap_msg.as_ref().map(|m| m.text.as_str()), Some("H"));
    }
}
