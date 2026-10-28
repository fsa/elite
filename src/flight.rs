//! Нативный движок космического полёта (клавиша F1).
//!
//! Это альтернатива Z80-полёту: пока движок активен, окно целиком занимает
//! нативный рендер. Возврат к Z80 — клавиша `1` (или `F1`).
//! Оформление повторяет титульный экран: чёрный фон, заголовок
//! `---- E L I T E ----` шрифтом сетки ZX, жёлтая подсказка внизу.
//! Объекты (корабли, станция) рисуются тем же каркасным рендером, что и
//! вращающийся корабль на заставке ([`crate::render3d`]).
//!
//! Мир — простая ньютоновская модель: у корабля есть ориентация
//! ([`Rotation`]), скорость вдоль носа и положение. Управление повторяет
//! привычные клавиши оригинала (стрелки/Kempston и `X/S/N/M`), поэтому
//! переключение между движками не меняет мышечную память.
//!
//! Что уже есть в модели:
//!
//! * звёздное поле: точки летят навстречу, и чем выше скорость, тем быстрее;
//! * непрозрачная планета с фазой освещения от солнца;
//! * солнце с ореолом;
//! * станция и несколько кораблей с собственным вращением;
//! * лазеры с перегревом и попаданием по цели;
//! * щиты, топливо, скорость и температура лазера на нижней панели.
//!
//! Движок не пишет в память Z80: возврат по `1` продолжает игру с того же
//! места. В Native эмулятор всё же крутится в фоне — ради оригинального
//! биперного звука; в Z80 LINK он ещё и задаёт координаты мира.
//! Скорость/топливо/щиты при входе читаются из памяти игры как начальные
//! значения.

use crate::display::Frame;
pub use crate::flight_model::Controls;
use crate::render3d::{self, Rotation, Screen};
use crate::shipdata::{self, Ship};

/// Цвет фона (непрозрачный чёрный).
const BG: u32 = 0xFF00_0000;
/// Белый цвет заголовка и рамок — как на заставке.
const FG: u32 = 0xFFD7_D7D7;
/// Жёлтый цвет подсказок.
const HINT: u32 = 0xFFFF_FF00;
/// Циан — линии объектов.
const CYAN: u32 = 0xFF00_FFFF;
/// Красный — лазеры и опасность.
const RED: u32 = 0xFFFF_4040;
/// Тусклый серый — второстепенные подписи.
const DIM: u32 = 0xFF80_8080;
/// Солнце — ровный жёлтый диск.
const SUN_COLOR: u32 = 0xFFFF_FF00;

/// Максимальная скорость в единицах `HighDeltaSpeed` (0..=31).
///
/// В оригинале `DeltaSpeed` — 16-битное слово с шагом `0xA0` на кадр, а его
/// старший байт `HighDeltaSpeed` меняется на 1 за 1/0.625 кадра. Полная шкала
/// заканчивается на 31: выше игра скорость не поднимает.
const SPEED_MAX: f32 = 31.0;
/// Стартовая скорость оригинала (`HighDeltaSpeed = 12`).
const SPEED_START: f32 = 12.0;
/// Прирост/сброс скорости в единицах `HighDeltaSpeed` в секунду.
///
/// `DeltaSpeed` меняется на `0xA0` за кадр 50 Гц, то есть старший байт — на
/// `0xA0 / 256 * 50 = 31.25` единицы в секунду.
const SPEED_ACCEL: f32 = 31.25;
/// Мировых единиц на единицу `HighDeltaSpeed` в секунду.
///
/// По замерам оригинала планета уходит примерно на 0.2 единицы координат за
/// кадр на каждую единицу скорости (`speed=12` — 2.6/кадр, `speed=31` —
/// 7.68/кадр). При 50 Гц это 10 мировых единиц в секунду.
const WORLD_SPEED_PER_UNIT: f32 = 10.0;
/// Угловая скорость управления (радиан в секунду).
const ROT_SPEED: f32 = 1.15;
/// Перегрев лазера: прирост за выстрел и остывание в секунду.
const LASER_HEAT: f32 = 0.16;
const LASER_COOL: f32 = 0.30;
/// Сколько секунд виден луч лазера.
const LASER_TIME: f32 = 0.10;
/// Сколько секунд виден луч выстрела противника.
///
/// В оригинале луч живёт один кадр (флаг `oFlags` сбрасывается сразу после
/// печати), но при 50 Гц это почти незаметно. Чуть более долгий след делает
/// атаку читаемой, не превращая её в постоянную линию.
const ENEMY_LASER_TIME: f32 = 0.14;
/// Ближняя плоскость отсечения.
const NEAR: f32 = 0.5;
/// Число звёзд на экране по умолчанию.
///
/// В оригинале таблица `byte_7A39` содержит ровно 16 звёзд на кадр
/// 256×192. На современном разрешении такое поле выглядит почти пустым,
/// поэтому фактическое число звёзд считается от площади кадра
/// (см. [`star_count_for`]), а эта константа задаёт минимум.
const STAR_COUNT: usize = 16;
/// Площадь кадра, на которую приходится одна звезда.
///
/// В оригинале одна звезда на ~3072 пикселя. Здесь плотность вдвое ниже:
/// современная точка — это 1 px, и слишком густое поле превращается в шум.
const STAR_AREA_PER_STAR: f32 = 6_000.0;
/// Пределы числа звёзд, чтобы поле не пустело и не забивало экран.
const STAR_COUNT_MIN: usize = 16;
const STAR_COUNT_MAX: usize = 400;
/// Делитель шага звёздного поля.
///
/// В оригинале координаты звёзд растягиваются от центра примерно на
/// `(HighDeltaSpeed >> 1) + 1` единиц за кадр. Прямой перенос этого шага в
/// перспективную проекцию даёт слишком резкое движение, поэтому темп подобран
/// так, чтобы на крейсерской скорости звезда пересекала кадр за несколько
/// секунд, как в оригинале.
const STAR_ZOOM_DIVISOR: f32 = 3000.0;
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
const STAR_FAST_SCROLL_B: f32 = 34.0;
/// Поворот звёздного поля за единицу `HighAngleLR` (крен).
///
/// В `CalcNewCrdObj_Pl` крен применяется как `angleLR * 2 / 256`, а тангаж —
/// как `angleDU / 256`. Это малые углы за игровой такт, а не за кадр 50 Гц:
/// оригинал пересчитывает координаты примерно на 5.5 Гц, поэтому в Z80 LINK
/// поворот звёзд применяется только на такте (см. `sync_from_z80`).
const STAR_ANGLE_LR_PER_UNIT: f32 = 2.0 / 256.0;
/// Поворот звёздного поля за единицу `HighAngleDU` (тангаж).
const STAR_ANGLE_DU_PER_UNIT: f32 = 1.0 / 256.0;

// Адреса в памяти Z80 (версия Кладова). Координаты объектов ведутся уже
// в системе координат игрока: камера стоит в начале координат вдоль +Z.
/// Минимальный объём памяти, покрывающий все адреса полётных данных
/// (включая таблицу звёзд и флаг J-драйва).
const Z80_MEM_MIN: usize = 0x7EE0;
/// `HighAngleLR` — угловая скорость крена (знак-величина).
const Z80_HIGH_ANGLE_LR: usize = 0x70E4;
/// `HighAngleDU` — угловая скорость тангажа (знак-величина).
const Z80_HIGH_ANGLE_DU: usize = 0x70E5;
/// `HighDeltaSpeed` — текущая скорость, 0..=31.
const Z80_HIGH_DELTA_SPEED: usize = 0x70E6;
/// `Speed` — индикатор скорости: `5 * HighDeltaSpeed + 5`, 0..=0xA0.
///
/// В отличие от `HighDeltaSpeed`, эту ячейку J-драйв не трогает, поэтому
/// скорость в Z80 LINK надёжнее брать отсюда.
const Z80_SPEED_GAUGE: usize = 0x70F4;
/// `ShieldFront` — передний щит, 0..0xA0.
const Z80_SHIELD_FRONT: usize = 0x70E8;
/// `Fuel` — топливо, 0..0xA0.
const Z80_FUEL: usize = 0x70EC;
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
/// Планета — тёмно-зелёный диск: такого оттенка нет ни у одного другого
/// объекта, и на его фоне хорошо видны белые, циановые и жёлтые детали.
const PLANET_COLOR: u32 = 0xFF00_5800;

/// Начальные значения, снятые с Z80-игры при входе в нативный полёт.
#[derive(Clone, Copy, Debug)]
pub struct FlightSeed<'a> {
    /// Имя текущей системы (для строки состояния).
    pub system: &'a str,
    /// Значение индикатора скорости из памяти игры (`Speed`, 0..0xA0).
    /// Оригинал хранит там `5 * HighDeltaSpeed + 5`.
    pub speed: u8,
    /// Топливо (`Fuel`, 0..0xA0).
    pub fuel: u8,
    /// Передний щит (`ShieldFront`, 0..0xA0).
    pub shield: u8,
    /// Цвет системы из [`crate::galaxy::color`]; задаёт оттенок зелёной
    /// планеты, но не может сделать её не зелёной.
    pub planet_color: u32,
}


/// Тип объекта в сцене.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BodyKind {
    Ship,
    Station,
    Planet,
    Sun,
}

/// Объект сцены.
#[derive(Clone)]
struct Body {
    kind: BodyKind,
    /// Геометрия (для кораблей и станции).
    model: Option<&'static Ship>,
    /// Положение в мировых координатах.
    pos: [f32; 3],
    /// Ориентация модели относительно мира.
    rot: Rotation,
    /// Ось собственного вращения (в мире).
    spin: [f32; 3],
    /// Скорость собственного вращения, радиан/с.
    spin_rate: f32,
    /// Радиус для проекции и попадания.
    radius: f32,
    /// Цвет каркаса или диска.
    color: u32,
    /// Таймер выстрела (для противников).
    fire_timer: f32,
}

/// Звезда в космическом пространстве.
struct Star {
    /// Положение относительно камеры/мира в упрощённой модели.
    pos: [f32; 3],
    mag: u8,
}

/// Выстрел противника: отрезок от корабля к окрестности перекрестия.
///
/// Храним мировые координаты, чтобы линия правильно поворачивалась вместе с
/// камерой, пока живёт.
struct EnemyShot {
    from: [f32; 3],
    to: [f32; 3],
    life: f32,
}

/// Источник данных для нативного рендера.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlightSource {
    /// Своя нативная симуляция.
    Native,
    /// Данные из эмуляции Z80 (адаптер).
    Z80,
}

/// Нативный движок полёта.
pub struct Flight {
    /// Показывать ли нативный экран вместо Z80-игры.
    pub active: bool,
    /// Источник данных для рендера.
    pub source: FlightSource,
    started: bool,
    /// Ориентация камеры: мир → вид.
    rot: Rotation,
    /// Положение корабля в мире.
    pos: [f32; 3],
    /// Текущая скорость в единицах `HighDeltaSpeed` (`0..=31`).
    speed: f32,
    /// Угловые скорости с инерцией (радиан/с).
    ang_pitch: f32,
    ang_roll: f32,
    shield: f32,
    fuel: f32,
    laser_heat: f32,
    laser_timer: f32,
    /// Предыдущий `TempLaser` из Z80 — по его росту ловим выстрел в LINK.
    prev_laser_temp: Option<u8>,
    /// Предыдущий передний щит Z80 — по его падению ловим атаку в LINK.
    prev_z80_shield: Option<u8>,
    /// Снимок таблицы звёзд Z80: её изменение означает игровой такт.
    prev_star_table: Option<[u8; Z80_STARS_LEN]>,
    /// В памяти был замечен взвод `StarsFastScroll` (J-драйв).
    z80_fast_scroll: bool,
    /// Разогнать звёзды на ближайшем шаге симуляции (J-драйв).
    star_boost: bool,
    /// Лучи выстрелов противников.
    enemy_shots: Vec<EnemyShot>,
    damage_flash: f32,
    message_timer: f32,
    respawn_timer: f32,
    message: &'static str,
    kills: u32,
    tick: u64,
    bodies: Vec<Body>,
    stars: Vec<Star>,
    system: String,
    planet_color: u32,
    rng: u32,
}

impl Flight {
    pub fn new() -> Self {
        let mut f = Flight {
            active: false,
            source: FlightSource::Native,
            started: false,
            rot: Rotation::identity(),
            pos: [0.0; 3],
            speed: SPEED_START,
            ang_pitch: 0.0,
            ang_roll: 0.0,
            shield: 1.0,
            fuel: 1.0,
            laser_heat: 0.0,
            laser_timer: 0.0,
            prev_laser_temp: None,
            prev_z80_shield: None,
            prev_star_table: None,
            z80_fast_scroll: false,
            star_boost: false,
            enemy_shots: Vec::new(),
            damage_flash: 0.0,
            message_timer: 0.0,
            respawn_timer: 0.0,
            message: "",
            kills: 0,
            tick: 0,
            bodies: Vec::new(),
            stars: Vec::new(),
            system: String::new(),
            planet_color: PLANET_COLOR,
            rng: 0x1234_5678,
        };
        f.reset_scene();
        f
    }

    /// Войти в нативный полёт, подхватив начальные показания Z80-игры.
    pub fn open(&mut self, seed: FlightSeed<'_>) {
        self.active = true;
        // По умолчанию Z80 LINK для сравнения с оригиналом
        self.source = FlightSource::Z80;
        self.prev_laser_temp = None;
        self.prev_z80_shield = None;
        self.prev_star_table = None;
        self.z80_fast_scroll = false;
        self.star_boost = false;
        self.enemy_shots.clear();
        self.system = if seed.system.is_empty() {
            "UNKNOWN".to_string()
        } else {
            seed.system.to_uppercase()
        };
        if seed.planet_color != 0 {
            self.planet_color = planet_green(seed.planet_color);
        }
        if !self.started {
            self.started = true;
            self.reset_scene();
        } else {
            // Цвет планеты мог смениться вместе с системой — обновляем диск.
            for b in &mut self.bodies {
                if b.kind == BodyKind::Planet {
                    b.color = self.planet_color;
                }
            }
        }
        // Индикатор `Speed` в памяти игры — это `5 * HighDeltaSpeed + 5`
        // (0..0xA0), поэтому переводим его обратно в единицы оригинала.
        // Нулевая скорость допустима: даже тогда звёзды ползут (в оригинале
        // шаг звезды равен `(HighDeltaSpeed >> 1) + 1`).
        self.speed = ((seed.speed as f32 - 5.0) / 5.0).clamp(0.0, SPEED_MAX);
        self.fuel = (seed.fuel as f32 / 0xA0 as f32).clamp(0.0, 1.0);
        self.shield = (seed.shield as f32 / 0xA0 as f32).clamp(0.0, 1.0);
    }

    /// Вернуться к Z80-версии (клавиша `1` или повторный `F1`).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Текущая скорость в единицах `HighDeltaSpeed` (0..=31).
    #[cfg(test)]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Доля полной шкалы скорости, как у индикатора оригинала.
    ///
    /// Оригинал показывает `5 * HighDeltaSpeed + 5` из `0xA0`, то есть
    /// `(HighDeltaSpeed + 1) / 32`: на нуле шкала не пустая, а на 31 —
    /// заполнена полностью.
    fn speed_fraction(&self) -> f32 {
        ((self.speed + 1.0) / (SPEED_MAX + 1.0)).clamp(0.0, 1.0)
    }

    /// Положение корабля в мире.
    #[cfg(test)]
    pub fn position(&self) -> [f32; 3] {
        self.pos
    }

    /// Направление взгляда (нос корабля) в мировых координатах.
    #[cfg(test)]
    pub fn forward(&self) -> [f32; 3] {
        self.rot.transpose().apply([0.0, 0.0, 1.0])
    }

    /// Сбросить сцену в исходное положение.
    fn reset_scene(&mut self) {
        self.rot = Rotation::identity();
        self.pos = [0.0; 3];
        self.ang_pitch = 0.0;
        self.ang_roll = 0.0;
        self.stars = make_stars(&mut self.rng, STAR_COUNT);
        self.bodies.clear();
        self.enemy_shots.clear();

        let station = shipdata::SHIPS
            .iter()
            .find(|s| s.name == "d_Coriolis")
            .or_else(|| shipdata::SHIPS.first());
        self.bodies.push(Body {
            kind: BodyKind::Station,
            model: station,
            pos: [1800.0, -420.0, 4200.0],
            rot: Rotation::identity(),
            spin: [0.0, 0.0, 1.0],
            spin_rate: 0.25,
            radius: 260.0,
            color: 0xFFA0_FFFF,
            fire_timer: 0.0,
        });

        for (name, pos, spin_rate) in [
            ("d_Viper", [-980.0, 160.0, 2500.0], 0.9),
            ("d_Cobra1", [1250.0, -260.0, 3900.0], -0.7),
            ("dSidewinderH", [420.0, 520.0, 6200.0], 0.5),
        ] {
            let model = shipdata::SHIPS.iter().find(|s| s.name == name);
            self.bodies.push(Body {
                kind: BodyKind::Ship,
                model,
                pos,
                rot: Rotation::axis_angle([0.0, 1.0, 0.0], spin_rate),
                spin: [0.2, 1.0, 0.15],
                spin_rate,
                radius: 110.0,
                color: CYAN,
                fire_timer: 2.0 + (self.rng % 100) as f32 / 40.0,
            });
        }

        // Планета и солнце — далёкие объекты, их размеры в мировых единицах
        // подобраны так, чтобы на экране они читались как диски, а не точки.
        self.bodies.push(Body {
            kind: BodyKind::Planet,
            model: None,
            pos: [-1600.0, 2300.0, 96000.0],
            rot: Rotation::identity(),
            spin: [0.0, 1.0, 0.0],
            spin_rate: 0.02,
            radius: PLANET_RADIUS,
            color: self.planet_color,
            fire_timer: 0.0,
        });
        self.bodies.push(Body {
            kind: BodyKind::Sun,
            model: None,
            pos: [-270000.0, 78000.0, 390000.0],
            rot: Rotation::identity(),
            spin: [0.0; 3],
            spin_rate: 0.0,
            radius: SUN_RADIUS,
            color: SUN_COLOR,
            fire_timer: 0.0,
        });
    }

    /// Переключить источник данных для рендера (без остановки эмуляции).
    pub fn toggle_source(&mut self) {
        if !self.active {
            return;
        }
        self.source = match self.source {
            FlightSource::Native => FlightSource::Z80,
            FlightSource::Z80 => FlightSource::Native,
        };
        // Снимок таблицы звёзд после смены источника устаревает: иначе первый
        // синк применил бы накопленный поворот одним скачком.
        self.prev_star_table = None;
        self.z80_fast_scroll = false;
        self.star_boost = false;
        if self.source == FlightSource::Native {
            // Адаптер Z80 LINK полностью заменяет `bodies`, `rot` и `pos`
            // данными из памяти эмулятора. Без пересборки сцены в Native
            // рисовать нечего — остаются только звёзды.
            self.rebuild_native_scene();
        }
    }

    /// Пересобрать собственную сцену нативного полёта.
    fn rebuild_native_scene(&mut self) {
        self.reset_scene();
        self.prev_z80_shield = None;
        // Нулевая скорость допустима: индикатор показывает минимум 1/32, а
        // звёзды всё равно медленно расходятся, как в оригинале.
        self.speed = self.speed.clamp(0.0, SPEED_MAX);
    }

    /// Синхронизировать нативный рендер с данными из памяти Z80.
    ///
    /// Абсолютной матрицы ориентации игрока в оригинале нет: камера задаётся
    /// двумя угловыми скоростями (`HighAngleLR`/`HighAngleDU`), а координаты
    /// всех объектов ведутся уже в системе координат игрока. `CalcNewCrdObj_Pl`
    /// каждый кадр поворачивает их на эти углы и сдвигает по `HighDeltaSpeed`,
    /// поэтому координаты слотов, `SunCrd` и `PlanetCrd` — это готовые
    /// координаты вида. Камера нативного рендера стоит в начале координат,
    /// а оси объектов читаются из их слотов.
    pub fn sync_from_z80(&mut self, mem: &[u8], _kladov: bool) {
        if !self.active || self.source != FlightSource::Z80 || mem.len() < Z80_MEM_MIN {
            return;
        }

        // Координаты Z80 уже в системе координат вида.
        self.pos = [0.0; 3];
        self.rot = Rotation::identity();

        // Приборы. Скорость берём из индикатора (`5 * HighDeltaSpeed + 5`):
        // на такте J-драйва `PrintStars` временно пишет в `HighDeltaSpeed`
        // `0x1E`/`0x10`, и прямой опрос заставлял бы шкалу прыгать.
        let full = 0xA0 as f32;
        let gauge = mem[Z80_SPEED_GAUGE];
        self.speed = (gauge.saturating_sub(5) as f32 / 5.0).clamp(0.0, SPEED_MAX);
        let shield_raw = mem[Z80_SHIELD_FRONT];
        // Оригинал не оставляет флаг выстрела противника в памяти: бит 3
        // сбрасывается сразу после печати луча. Зато попадание уменьшает
        // `ShieldFront` — по этому падению и показываем ответный луч.
        let shield_dropped = self
            .prev_z80_shield
            .is_some_and(|prev| shield_raw < prev);
        self.prev_z80_shield = Some(shield_raw);
        self.shield = (shield_raw as f32 / full).clamp(0.0, 1.0);
        self.fuel = (mem[Z80_FUEL] as f32 / full).clamp(0.0, 1.0);

        // В Z80 LINK нет абсолютной ориентации, но угловые скорости известны.
        // Оригинал пересчитывает звёзды и координаты объектов не каждый кадр,
        // а на игровом такте (~5.5 Гц): таблица звёзд `byte_7A39` меняется
        // только тогда. Раньше поворот применялся на каждом кадре 50 Гц,
        // из-за чего звёзды крутились примерно в 9 раз быстрее объектов.
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
        self.laser_heat = (temp as f32 / full).clamp(0.0, 1.0);

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

    /// Один шаг симуляции.
    pub fn tick(&mut self, c: Controls, dt: f32) {
        if !self.active {
            return;
        }
        if self.source == FlightSource::Z80 {
            // В режиме Z80 данные обновляются извне через sync_from_z80.
            self.tick += 1;
            let dt = dt.clamp(0.0, 0.1);
            self.laser_timer = (self.laser_timer - dt).max(0.0);
            self.decay_enemy_shots(dt);
            // Нагрев лазера приходит из памяти игры, сам его не остываем.
            self.damage_flash = (self.damage_flash - dt).max(0.0);
            self.message_timer = (self.message_timer - dt).max(0.0);
            self.respawn_timer = (self.respawn_timer - dt).max(0.0);
            // Камеру в Z80 не двигаем сами, но звёздное поле должно лететь
            // со скоростью, прочитанной из памяти игры.
            self.advance_stars(dt);
            return;
        }
        self.tick += 1;
        let dt = dt.clamp(0.0, 0.1);

        // Ориентация с инерцией: набираем угловую скорость при нажатии,
        // затем затухаем (как в оригинальном Elite).
        let pitch_in = (c.pitch_down as i32 - c.pitch_up as i32) as f32;
        let roll_in = (c.roll_right as i32 - c.roll_left as i32) as f32;
        let accel = ROT_SPEED * 3.2;
        let damp = 1.0 - (4.5 * dt).min(0.9);
        self.ang_pitch += pitch_in * accel * dt;
        self.ang_roll += roll_in * accel * dt;
        self.ang_pitch *= damp;
        self.ang_roll *= damp;
        self.ang_pitch = self.ang_pitch.clamp(-ROT_SPEED * 1.4, ROT_SPEED * 1.4);
        self.ang_roll = self.ang_roll.clamp(-ROT_SPEED * 1.4, ROT_SPEED * 1.4);
        // Тот же поворот применяем и к камере, и к звёздам: звёзды хранятся
        // в координатах вида, поэтому им достаточно повернуть свои позиции.
        let mut turn = Rotation::identity();
        if self.ang_pitch.abs() > 1e-4 {
            // Знак обратный угловой скорости: при взгляде вверх (S) мир
            // должен уезжать вниз по экрану, а не наоборот.
            turn = Rotation::axis_angle([1.0, 0.0, 0.0], -self.ang_pitch * dt).mul(&turn);
        }
        if self.ang_roll.abs() > 1e-4 {
            turn = Rotation::axis_angle([0.0, 0.0, 1.0], self.ang_roll * dt).mul(&turn);
        }
        self.rot = turn.mul(&self.rot);
        self.turn_stars(&turn);

        // Тяга. В оригинале `DeltaSpeed` меняется на 0xA0 за кадр в обе
        // стороны, то есть на 31.25 единицы `HighDeltaSpeed` в секунду.
        let throttle = (c.throttle_up as i32 - c.throttle_down as i32) as f32;
        if throttle > 0.0 && self.fuel > 0.0 {
            self.speed = (self.speed + SPEED_ACCEL * dt).min(SPEED_MAX);
            self.fuel = (self.fuel - 0.020 * dt).max(0.0);
        } else if throttle < 0.0 {
            self.speed = (self.speed - SPEED_ACCEL * dt).max(0.0);
        } else {
            self.speed = self.speed.clamp(0.0, SPEED_MAX);
        }

        // Лазер: выстрел, попадание и остывание.
        self.laser_timer = (self.laser_timer - dt).max(0.0);
        self.decay_enemy_shots(dt);
        self.laser_heat = (self.laser_heat - LASER_COOL * dt).max(0.0);
        // В Z80 LINK попадания считает сама игра; здесь только показываем
        // луч, который приходит через рост `TempLaser` в `sync_from_z80`.
        if c.fire
            && self.source != FlightSource::Z80
            && self.laser_heat < 0.95
            && self.laser_timer <= 0.0
        {
            self.fire();
        }
        self.damage_flash = (self.damage_flash - dt).max(0.0);
        self.message_timer = (self.message_timer - dt).max(0.0);
        self.respawn_timer = (self.respawn_timer - dt).max(0.0);

        // Движение вперёд вдоль носа. transpose возвращает мировое
        // направление взгляда из матрицы «мир → вид».
        if self.source != FlightSource::Z80 {
            let forward = self.rot.transpose().apply([0.0, 0.0, 1.0]);
            for i in 0..3 {
                self.pos[i] +=
                    forward[i] * self.speed * WORLD_SPEED_PER_UNIT * dt;
            }
            self.advance_stars(dt);
        } else {
            // В Z80 режиме не двигаем камеру самостоятельно, только звёзды
            self.advance_stars(dt);
        }

        // Собственное вращение и простая тактика противников.
        let forward_cur = self.rot.transpose().apply([0.0, 0.0, 1.0]);
        let right_cur = self.rot.transpose().apply([1.0, 0.0, 0.0]);
        let up_cur = self.rot.transpose().apply([0.0, 1.0, 0.0]);
        for b in &mut self.bodies {
            if b.spin_rate != 0.0 {
                b.rot = Rotation::axis_angle(b.spin, b.spin_rate * dt).mul(&b.rot);
            }
            if b.kind != BodyKind::Ship || self.source == FlightSource::Z80 {
                continue;
            }
            let mut d = [
                self.pos[0] - b.pos[0],
                self.pos[1] - b.pos[1],
                self.pos[2] - b.pos[2],
            ];
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0);
            let sign = if dist > 2200.0 { 1.0 } else { -1.0 };
            let step = 36.0 * dt * sign;
            for v in &mut d {
                *v /= dist;
            }
            for i in 0..3 {
                b.pos[i] += d[i] * step;
            }
            // Противник изредка стреляет, если смотрит в сторону игрока.
            b.fire_timer -= dt;
            if b.fire_timer <= 0.0 && dist < 6500.0 {
                let dot = (-d[0]) * forward_cur[0] + (-d[1]) * forward_cur[1] + (-d[2]) * forward_cur[2];
                if dot > 0.35 {
                    self.shield = (self.shield - 0.045).max(0.02);
                    self.damage_flash = 0.25;
                    // Луч из точки выстрела корабля к перекрестию. В
                    // оригинале он тоже рисуется от вершины лазера к
                    // случайной точке возле центра экрана.
                    let origin = [
                        b.pos[0] + d[0] * b.radius * 0.6,
                        b.pos[1] + d[1] * b.radius * 0.6,
                        b.pos[2] + d[2] * b.radius * 0.6,
                    ];
                    let jx = (next(&mut self.rng) % 161) as f32 - 80.0;
                    let jy = (next(&mut self.rng) % 121) as f32 - 60.0;
                    let target = [
                        self.pos[0] + forward_cur[0] * 6000.0 + right_cur[0] * jx + up_cur[0] * jy,
                        self.pos[1] + forward_cur[1] * 6000.0 + right_cur[1] * jx + up_cur[1] * jy,
                        self.pos[2] + forward_cur[2] * 6000.0 + right_cur[2] * jx + up_cur[2] * jy,
                    ];
                    self.enemy_shots.push(EnemyShot {
                        from: origin,
                        to: target,
                        life: ENEMY_LASER_TIME,
                    });
                }
                b.fire_timer = 2.2 + (next(&mut self.rng) % 180) as f32 / 50.0;
            }
        }

        if self.source != FlightSource::Z80 && self.respawn_timer <= 0.0 && !self.bodies.iter().any(|b| b.kind == BodyKind::Ship) {
            self.spawn_enemy();
        }
    }

    /// Выстрел из лазера: попадание по кораблю рядом с перекрестием.
    fn fire(&mut self) {
        self.laser_timer = LASER_TIME;
        self.laser_heat = (self.laser_heat + LASER_HEAT).min(1.0);
        let forward = self.rot.transpose().apply([0.0, 0.0, 1.0]);
        let mut hit = None;
        let mut best = 0.0f32;
        for (i, b) in self.bodies.iter().enumerate() {
            if b.kind != BodyKind::Ship {
                continue;
            }
            let d = [
                b.pos[0] - self.pos[0],
                b.pos[1] - self.pos[1],
                b.pos[2] - self.pos[2],
            ];
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0);
            let dot = (d[0] * forward[0] + d[1] * forward[1] + d[2] * forward[2]) / dist;
            // Угловой радиус цели; минимум 2° — прощаем неточное прицеливание.
            let limit = (b.radius / dist).clamp(0.0, 1.0).asin().max(0.035);
            if dot >= limit.cos() && (hit.is_none() || dist < best) {
                hit = Some(i);
                best = dist;
            }
        }
        if let Some(i) = hit {
            self.bodies.remove(i);
            self.kills += 1;
            self.message = "TARGET DESTROYED";
            self.message_timer = 1.5;
            self.respawn_timer = 4.0;
        }
    }

    /// Погасить отжившие лучи противников.
    fn decay_enemy_shots(&mut self, dt: f32) {
        for shot in &mut self.enemy_shots {
            shot.life -= dt;
        }
        self.enemy_shots.retain(|shot| shot.life > 0.0);
    }

    /// Показать выстрел противника в Z80 LINK.
    ///
    /// Луч пускаем от ближайших кораблей, которые находятся впереди, к
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

    fn spawn_enemy(&mut self) {
        let models = ["d_Viper", "d_Cobra1", "dSidewinderH", "d_Krait"];
        let name = models[(self.next_rng() as usize) % models.len()];
        let model = shipdata::SHIPS.iter().find(|s| s.name == name);
        let x = (self.next_rng() % 4000) as f32 - 2000.0;
        let y = (self.next_rng() % 2400) as f32 - 1200.0;
        self.bodies.push(Body {
            kind: BodyKind::Ship,
            model,
            pos: [x, y, 6500.0],
            rot: Rotation::axis_angle([0.2, 1.0, 0.1], 1.0),
            spin: [0.2, 1.0, 0.15],
            spin_rate: 0.8,
            radius: 110.0,
            color: CYAN,
            fire_timer: 2.0,
        });
    }

    fn next_rng(&mut self) -> u32 {
        // xorshift32 — детерминированный и без зависимостей.
        let mut x = self.rng.max(1);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        x
    }

    /// Нарисовать полёт на весь кадр.
    pub fn draw(&mut self, frame: &mut Frame, font: &[u8]) {
        let w = frame.width as i32;
        let h = frame.height as i32;
        frame.fill_rect(0, 0, frame.width, frame.height, BG);
        // Плотность звёзд зависит от площади кадра: на большом разрешении
        // исходных 16 точек недостаточно, экран выглядит пустым.
        let want_stars = star_count_for(frame.width, frame.height);
        if self.stars.len() != want_stars {
            self.stars = make_stars(&mut self.rng, want_stars);
        }
        // В режиме Z80 LINK всегда добавляем планету/солнце если их нет
        let has_planet = self.bodies.iter().any(|b| matches!(b.kind, BodyKind::Planet));
        let has_sun = self.bodies.iter().any(|b| matches!(b.kind, BodyKind::Sun));
        if !has_planet {
            self.bodies.push(Body {
                kind: BodyKind::Planet,
                model: None,
                pos: [-1600.0, 2300.0, 96000.0],
                rot: Rotation::identity(),
                spin: [0.0, 1.0, 0.0],
                spin_rate: 0.02,
                radius: PLANET_RADIUS,
                color: self.planet_color,
                fire_timer: 0.0,
            });
        }
        if !has_sun {
            self.bodies.push(Body {
                kind: BodyKind::Sun,
                model: None,
                pos: [-270000.0, 78000.0, 390000.0],
                rot: Rotation::identity(),
                spin: [0.0; 3],
                spin_rate: 0.0,
                radius: SUN_RADIUS,
                color: SUN_COLOR,
                fire_timer: 0.0,
            });
        }
        // Сетка ZX 256x192, как на заставке: заголовок и подсказка остаются
        // в тех же местах, поэтому экран полёта выглядит её продолжением.
        let s = ((frame.width / 256).min(frame.height / 192)).max(1) as i32;
        let ox = (w - 256 * s) / 2;
        let title_y = 4 * s;
        frame.draw_text(
            font,
            ox + 40 * s,
            title_y,
            "---- E L I T E ----",
            FG,
            s as usize,
        );
        let sub = match self.source {
            FlightSource::Native => "NATIVE FLIGHT",
            FlightSource::Z80 => "Z80 LINK FLIGHT",
        };
        frame.draw_text(font, ox + 8 * s, 14 * s, sub, HINT, s as usize);
        // Имя системы — в правой части той же строки (не длиннее 11 знаков,
        // чтобы не налезть на «NATIVE FLIGHT»).
        let system: String = self.system.chars().take(11).collect();
        let sx = ox + (256 - system.len() as i32 * 8) * s;
        frame.draw_text(font, sx, 14 * s, &system, HINT, s as usize);
        // Число побед — коротко в правом верхнем углу, рядом с заголовком.
        let kills = format!("K:{}", self.kills);
        let kx = ox + (256 - kills.len() as i32 * 8) * s;
        frame.draw_text(font, kx, title_y, &kills, DIM, s as usize);

        let hud_top = h - 34 * s;
        // Строка состояния — 14-я строка сетки (14..22); вид начинается
        // ниже неё, иначе звёзды закрашивают нижнюю часть букв.
        let view_top = 24 * s;
        let view_bottom = (hud_top - 4 * s).max(view_top + 8 * s);
        let cx = w as f32 / 2.0;
        let cy = (view_top + view_bottom) as f32 / 2.0;
        let focal = ((view_bottom - view_top) as f32 * 1.15).max(200.0);
        let screen = Screen {
            cx,
            cy,
            k: focal,
            bx: 0.0,
            by: 0.0,
        };

        frame.set_clip(
            0,
            view_top,
            frame.width,
            (view_bottom - view_top).max(1) as usize,
        );
        self.draw_stars(frame, &screen);
        self.draw_bodies(frame, &screen);
        self.draw_enemy_shots(frame, &screen, view_top, view_bottom);
        self.draw_crosshair(frame, cx, cy, s);
        if self.laser_timer > 0.0 {
            self.draw_laser(frame, cx, cy, view_bottom, s);
        }
        if self.damage_flash > 0.0 {
            frame.hline(0, view_top, frame.width, RED);
            frame.hline(0, view_bottom - 1, frame.width, RED);
            frame.vline(0, view_top, (view_bottom - view_top) as usize, RED);
            frame.vline(w - 1, view_top, (view_bottom - view_top) as usize, RED);
        }
        frame.clear_clip();

        if self.message_timer > 0.0 {
            let tw = self.message.len() as i32 * 8 * s;
            frame.draw_text(
                font,
                (w - tw) / 2,
                view_top + 4 * s,
                self.message,
                RED,
                s as usize,
            );
        }
        self.draw_hud(frame, font, s, hud_top, ox);
        frame.draw_text(
            font,
            ox + 8 * s,
            h - 12 * s,
            "A FIRE  1 Z80  TAB LINK  ESC",
            HINT,
            s as usize,
        );
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
            (self.speed * 0.5).floor() + 1.0
        };
        let grow = (1.0 + b / STAR_ZOOM_DIVISOR).powf(frames);
        let mut rng = self.rng;
        for star in &mut self.stars {
            star.pos[0] *= grow;
            star.pos[1] *= grow;
            let z = star.pos[2];
            let radius = if z > NEAR {
                (star.pos[0] * star.pos[0] + star.pos[1] * star.pos[1]).sqrt() / z
            } else {
                f32::INFINITY
            };
            let distance2 = star.pos.iter().map(|v| v * v).sum::<f32>();
            if radius <= STAR_RESPAWN_RADIUS
                && distance2 <= STAR_MAX_DISTANCE * STAR_MAX_DISTANCE
                && star.pos.iter().all(|v| v.is_finite())
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

    /// Перевести мировую точку в координаты вида (камера в `self.pos`,
    /// матрица `self.rot` — мир → вид).
    fn view_point(&self, p: [f32; 3]) -> [f32; 3] {
        self.rot.apply([
            p[0] - self.pos[0],
            p[1] - self.pos[1],
            p[2] - self.pos[2],
        ])
    }

    /// Нарисовать лучи выстрелов противников.
    fn draw_enemy_shots(&self, frame: &mut Frame, screen: &Screen, top: i32, bottom: i32) {
        let (xmin, ymin) = (0.0, top as f32);
        let (xmax, ymax) = (frame.width as f32 - 1.0, bottom as f32 - 1.0);
        for shot in &self.enemy_shots {
            let Some((a, b)) = clip_segment_near(self.view_point(shot.from), self.view_point(shot.to))
            else {
                continue;
            };
            let ax = screen.cx + a[0] / a[2] * screen.k;
            let ay = screen.cy - a[1] / a[2] * screen.k;
            let bx = screen.cx + b[0] / b[2] * screen.k;
            let by = screen.cy - b[1] / b[2] * screen.k;
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
            frame.line(x0, y0, x1, y1, 0xFFFF_A020);
            // Второй пиксель поперёк луча делает выстрел заметнее на фоне
            // тонких циановых корпусов.
            frame.line(x0, y0 + 1, x1, y1 + 1, RED);
        }
    }

    fn draw_bodies(&self, frame: &mut Frame, screen: &Screen) {
        // Дальние объекты рисуются первыми, ближние перекрывают их.
        let mut order: Vec<(f32, usize)> = self
            .bodies
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let rel = [
                    b.pos[0] - self.pos[0],
                    b.pos[1] - self.pos[1],
                    b.pos[2] - self.pos[2],
                ];
                (self.rot.apply(rel)[2], i)
            })
            .collect();
        order.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (z, i) in order {
            if z <= NEAR {
                continue;
            }
            let b = &self.bodies[i];
            match b.kind {
                BodyKind::Planet => self.draw_planet(frame, b, screen),
                BodyKind::Sun => self.draw_sun(frame, b, screen),
                BodyKind::Ship | BodyKind::Station => {
                    let Some(model) = b.model else {
                        continue;
                    };
                    let rel = [
                        b.pos[0] - self.pos[0],
                        b.pos[1] - self.pos[1],
                        b.pos[2] - self.pos[2],
                    ];
                    let pos = self.rot.apply(rel);
                    let cam_rot = self.rot.mul(&b.rot);
                    render3d::draw_ship(frame, model, &cam_rot, pos, screen, b.color);
                }
            }
        }
    }

    /// Проекция центра объекта: экранные координаты и радиус диска.
    fn project(&self, b: &Body, screen: &Screen) -> Option<(f32, f32, f32, f32)> {
        let rel = [
            b.pos[0] - self.pos[0],
            b.pos[1] - self.pos[1],
            b.pos[2] - self.pos[2],
        ];
        let v = self.rot.apply(rel);
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
        if r < 0.8
            || x + r < 0.0
            || y + r < 0.0
            || x - r > frame.width as f32
            || y - r > frame.height as f32
        {
            return;
        }
        let (cx, cy, r) = (x.round() as i32, y.round() as i32, r.round() as i32);
        // Планета — один ровный диск без колец и градиентов.
        frame.disc(cx, cy, r, b.color);
    }

    fn draw_sun(&self, frame: &mut Frame, b: &Body, screen: &Screen) {
        let Some((x, y, r, _)) = self.project(b, screen) else {
            return;
        };
        if r < 0.8
            || x + r < 0.0
            || y + r < 0.0
            || x - r > frame.width as f32
            || y - r > frame.height as f32
        {
            return;
        }
        let (cx, cy, r) = (x.round() as i32, y.round() as i32, r.round().max(1.0) as i32);
        // Солнце — просто жёлтый диск, без ореола из нескольких окружностей.
        frame.disc(cx, cy, r, b.color);
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

    fn draw_hud(&self, frame: &mut Frame, font: &[u8], s: i32, top: i32, ox: i32) {
        // Рамка панели и разделительная линия — как у приборной панели
        // оригинала, но в графике заставки (белые линии по чёрному).
        frame.hline(0, top, frame.width, FG);
        // Четыре прибора в две колонки по две строки: при 16:10 и масштабе
        // x5 в нижнюю полосу иначе не помещаются четыре строки шрифта.
        let rows = [
            [
                ("SPEED", self.speed_fraction(), CYAN),
                (
                    "SHIELD",
                    self.shield,
                    if self.shield < 0.25 { RED } else { CYAN },
                ),
            ],
            [
                ("FUEL", self.fuel, if self.fuel < 0.2 { RED } else { HINT }),
                (
                    "LASER",
                    1.0 - self.laser_heat,
                    if self.laser_heat > 0.75 { RED } else { HINT },
                ),
            ],
        ];
        let col = |c: i32| ox + c * 8 * s;
        for (row, pair) in rows.iter().enumerate() {
            let y = top + (2 + row as i32 * 8) * s;
            for (half, (label, frac, fill)) in pair.iter().enumerate() {
                let base = if half == 0 { 1 } else { 17 };
                frame.draw_text(font, col(base), y, label, FG, s as usize);
                // Между самым длинным названием (SHIELD, 6 знаков) и шкалой
                // остаётся пустая колонка.
                let bar_x = col(base + 7);
                let bar_w = 4 * 8 * s;
                draw_bar(frame, *frac, bar_x, y + s, bar_w, 6 * s, *fill);
                let val_x = col(base + 12);
                let pct = format!("{:3}", (*frac * 100.0).round().clamp(0.0, 100.0) as i32);
                frame.draw_text(font, val_x, y, &pct, FG, s as usize);
            }
        }
        // Тонкая разделительная черта между колонками приборов.
        frame.vline(col(16), top + 2 * s, 20 * s as usize, DIM);
    }
}

impl Default for Flight {
    fn default() -> Self {
        Self::new()
    }
}

/// Горизонтальная шкала: белая рамка, заливка по значению `0..1`.
fn draw_bar(frame: &mut Frame, frac: f32, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w <= 2 || h <= 2 {
        return;
    }
    frame.fill_rect(x, y, w as usize, h as usize, FG);
    frame.fill_rect(x + 1, y + 1, (w - 2) as usize, (h - 2) as usize, BG);
    let fill = ((w - 2) as f32 * frac.clamp(0.0, 1.0)).round() as i32;
    if fill > 0 {
        frame.fill_rect(x + 1, y + 1, fill as usize, (h - 2) as usize, color);
    }
}

/// Детерминированное звёздное поле: звёзды перед кораблём.
fn make_stars(rng: &mut u32, count: usize) -> Vec<Star> {
    (0..count).map(|_| spawn_star(rng)).collect()
}

/// Сколько звёзд рисовать для кадра такого размера.
///
/// Число растёт с площадью экрана: на 256×192 это исходные 16 звёзд, на
/// 1280×800 — около 170. Так плотность поля примерно сохраняется, но не
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

/// Векторное произведение.
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
        a = [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            NEAR,
        ];
    } else if b[2] <= NEAR {
        let t = t(b, a);
        b = [
            b[0] + (a[0] - b[0]) * t,
            b[1] + (a[1] - b[1]) * t,
            NEAR,
        ];
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
    if (b[2] & 0x80) != 0 {
        -v
    } else {
        v
    }
}

/// Прочитать знаковый байт в формате знак-величина (оси объекта).
fn read_signed8(b: u8) -> f32 {
    let mag = (b & 0x7F) as f32;
    if (b & 0x80) != 0 {
        -mag
    } else {
        mag
    }
}

/// Зелёный оттенок планеты с небольшой вариацией яркости по системе.
///
/// Канал красного и синего всегда нулевой, поэтому планета не совпадает по
/// цвету ни с белыми кораблями, ни с циановой станцией, ни с жёлтым солнцем.
fn planet_green(seed: u32) -> u32 {
    let g = 0x58 + ((seed >> 8) & 0x1F);
    0xFF00_0000 | (g << 8)
}

/// Модель по адресу блока данных объекта из памяти Z80.
fn map_ship_ptr(ptr: u16, _kladov: bool) -> Option<&'static Ship> {
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
    let Some(model) = map_ship_ptr(ptr, true) else {
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
        spin: [0.0; 3],
        spin_rate: 0.0,
        radius: model.stats.collision as f32,
        color: match kind {
            BodyKind::Station => 0xFFA0_FFFF,
            _ => CYAN,
        },
        fire_timer: 0.0,
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
        spin: [0.0; 3],
        spin_rate: 0.0,
        radius,
        color,
        fire_timer: 0.0,
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
        f.open(FlightSeed {
            system: "Lave",
            speed: 0x50,
            fuel: 0xA0,
            shield: 0xA0,
            planet_color: 0xFF40_80C0,
        });
        f.source = FlightSource::Native;
        f
    }

    fn keys(pressed: &[(usize, usize)]) -> ([[bool; 5]; 8], [bool; 5]) {
        let mut m = [[false; 5]; 8];
        for &(r, b) in pressed {
            m[r][b] = true;
        }
        (m, [false; 5])
    }

    #[test]
    fn controls_follow_original_keys_and_arrows() {
        // Оригинал: S — вверх, X — вниз, N/M — крен, Space/Shift — скорость,
        // A — огонь (ControlKeyMove и ControlKey3).
        let (m, k) = keys(&[(1, 1), (0, 2), (7, 3), (7, 2), (7, 0), (7, 1), (1, 0)]);
        let c = Controls::from_matrix(&m, &k);
        assert!(c.pitch_up && c.pitch_down);
        assert!(c.roll_left && c.roll_right);
        assert!(c.throttle_up && c.throttle_down);
        assert!(c.fire);
        // Q/E в оригинале — не рыскание (Escape Pod / ECM), полёт их не трогает.
        let (m, _) = keys(&[(2, 0), (2, 2)]);
        let c = Controls::from_matrix(&m, &[false; 5]);
        assert_eq!(c, Controls::default());
        // Kempston: вверх/вниз/влево/вправо/огонь.
        let c = Controls::from_matrix(&[[false; 5]; 8], &[true, true, true, true, true]);
        assert!(c.roll_right && c.roll_left && c.pitch_down && c.pitch_up && c.fire);
    }

    #[test]
    fn z80_link_stars_move_with_memory_speed() {
        // В Z80 LINK камера не двигается нативно, но звёздное поле должно
        // лететь со скоростью из памяти игры, иначе экран выглядит застывшим.
        let mut f = open();
        f.source = FlightSource::Z80;
        f.speed = 12.0;
        if let Some(s) = f.stars.first_mut() {
            s.pos = [200.0, 0.0, 2000.0];
        }
        let start = f.stars[0].pos;
        f.tick(Controls::default(), 0.1);
        let moved = f.stars[0].pos;
        let step = (0..3).map(|i| (moved[i] - start[i]).abs()).sum::<f32>();
        assert!(step > 1.0, "в Z80 LINK звёзды не движутся");
    }

    #[test]
    fn z80_link_stars_turn_with_angle_rates() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];
        let star = |pos: [f32; 3]| Star { pos, mag: 128 };

        // Первый синк только запоминает таблицу звёзд: такта ещё не было.
        f.sync_from_z80(&mem, true);

        // Поворот без нового такта не применяется: иначе звёзды крутились бы
        // на каждом кадре 50 Гц, а не с частотой игрового цикла (~5.5 Гц).
        f.stars = vec![star([1000.0, 0.0, 0.0])];
        mem[Z80_HIGH_ANGLE_LR] = 0x04;
        mem[Z80_HIGH_ANGLE_DU] = 0x00;
        f.sync_from_z80(&mem, true);
        assert_eq!(
            f.stars[0].pos,
            [1000.0, 0.0, 0.0],
            "поворот применён без игрового такта"
        );

        // Игровой такт ловим по изменению таблицы звёзд, как в памяти игры.
        // Крен вправо: звезда на оси X должна уйти в +Y.
        let mut tick = |f: &mut Flight, mem: &mut [u8]| {
            mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
            f.sync_from_z80(mem, true);
        };
        tick(&mut f, &mut mem);
        let p = f.stars[0].pos;
        let roll_angle = p[1].atan2(p[0]);
        assert!(p[1] > 0.0, "положительный крен должен крутить поле против часовой");
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
    fn z80_link_speed_comes_from_gauge_not_clobbered_speed() {
        // На такте J-драйва `PrintStars` пишет в `HighDeltaSpeed` 0x1E/0x10.
        // Шкала скорости не должна из-за этого прыгать.
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41; // 5 * 12 + 5
        mem[Z80_HIGH_DELTA_SPEED] = 0x10; // след J-драйва
        f.sync_from_z80(&mem, true);
        assert!(
            (f.speed - 12.0).abs() < 1e-6,
            "скорость взята из затёртого HighDeltaSpeed: {}",
            f.speed
        );
    }

    #[test]
    fn j_drive_accelerates_stars_on_game_tick() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41; // скорость 12
        mem[Z80_HIGH_DELTA_SPEED] = 12;
        f.sync_from_z80(&mem, true); // запомнили таблицу звёзд
        let star = |pos: [f32; 3]| Star { pos, mag: 128 };

        // Обычный такт: таблица обновилась, скорость настоящая.
        f.stars = vec![star([100.0, 0.0, 1000.0])];
        mem[Z80_STARS_TABLE] = 1;
        f.sync_from_z80(&mem, true);
        let before = f.stars[0].pos[0];
        f.tick(Controls::default(), 1.0 / 50.0);
        let normal = f.stars[0].pos[0] - before;

        // Такт J-драйва: HighDeltaSpeed затёрт, таблица обновилась.
        f.stars = vec![star([100.0, 0.0, 1000.0])];
        mem[Z80_STARS_TABLE] = 2;
        mem[Z80_HIGH_DELTA_SPEED] = 0x10;
        f.sync_from_z80(&mem, true);
        let before = f.stars[0].pos[0];
        f.tick(Controls::default(), 1.0 / 50.0);
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
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0x41;
        f.sync_from_z80(&mem, true);

        // `StarsFastScroll` может быть виден в кадре до обновления таблицы.
        mem[Z80_STARS_FAST_SCROLL] = 1;
        f.sync_from_z80(&mem, true);
        assert!(!f.star_boost, "разгон включён до игрового такта");

        mem[Z80_STARS_TABLE] = 3;
        f.sync_from_z80(&mem, true);
        assert!(f.star_boost, "флаг J-драйва потерян между кадрами");
    }

    #[test]
    fn stars_stay_in_view_after_many_turns() {
        // После длительного вращения и полёта ни одна звезда не должна
        // оказаться в бесконечности или NaN — иначе на экране появляются
        // длинные линии через весь кадр.
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];
        mem[Z80_SPEED_GAUGE] = 0xA0; // скорость 31
        mem[Z80_HIGH_DELTA_SPEED] = 31;
        for frame in 0..2000 {
            mem[Z80_HIGH_ANGLE_LR] = if frame % 2 == 0 { 0x0F } else { 0x8F };
            mem[Z80_HIGH_ANGLE_DU] = if frame % 3 == 0 { 0x0F } else { 0x00 };
            // Имитируем игровой такт: таблица звёзд обновляется.
            mem[Z80_STARS_TABLE + frame % Z80_STARS_LEN] = frame as u8;
            f.sync_from_z80(&mem, true);
            f.tick(Controls::default(), 1.0 / 50.0);
        }
        for s in &f.stars {
            assert!(s.pos.iter().all(|v| v.is_finite()), "координата звезды не число");
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
        let f = open();
        let planet = f.bodies.iter().find(|b| b.kind == BodyKind::Planet).unwrap();
        assert_eq!(planet.color & 0x00FF_00FF, 0, "у планеты не зелёный цвет");
        assert!((planet.color >> 8) & 0xFF > 0, "планета слишком тёмная");
        assert_eq!(planet.radius, PLANET_RADIUS);
        assert!(planet.radius < SUN_RADIUS, "планета не должна быть больше солнца");
    }

    #[test]
    fn z80_link_detects_laser_shot_by_heating() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; Z80_MEM_MIN];

        f.sync_from_z80(&mem, true);
        assert_eq!(f.laser_timer, 0.0);

        // Оригинал поднял TempLaser выстрелом: показываем луч.
        mem[Z80_TEMP_LASER] = 0x08;
        f.sync_from_z80(&mem, true);
        assert!(
            f.laser_timer > 0.0,
            "выстрел в Z80 LINK не превратился в видимый луч"
        );

        // Остывание не должно само включать лазер.
        f.laser_timer = 0.0;
        mem[Z80_TEMP_LASER] = 0x00;
        f.sync_from_z80(&mem, true);
        assert_eq!(f.laser_timer, 0.0);
    }

    #[test]
    fn enemy_attack_draws_visible_beam() {
        let mut f = open();
        f.bodies.clear();
        f.bodies.push(Body {
            kind: BodyKind::Ship,
            model: shipdata::SHIPS.iter().find(|s| s.name == "d_Viper"),
            pos: [0.0, 0.0, 1400.0],
            rot: Rotation::identity(),
            spin: [0.0; 3],
            spin_rate: 0.0,
            radius: 110.0,
            color: CYAN,
            fire_timer: 0.0,
        });
        let shield_before = f.shield;
        f.tick(Controls::default(), 1.0 / 50.0);
        assert!(f.shield < shield_before, "атака не сняла щит");
        assert!(!f.enemy_shots.is_empty(), "выстрел противника не создан");

        let font = vec![0xFFu8; 0x300];
        let mut frame = Frame::new(1280, 960);
        f.draw(&mut frame, &font);
        let beam_pixels = frame
            .pixels()
            .iter()
            .filter(|&&p| p == 0xFFFF_A020)
            .count();
        assert!(
            beam_pixels > 50,
            "луч противника не попал в кадр или оборвался у перекрестия: {beam_pixels} px"
        );
    }

    #[test]
    fn z80_link_shows_enemy_beam_when_shield_drops() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = z80_mem_with_flight();
        f.sync_from_z80(&mem, true);
        assert!(f.enemy_shots.is_empty(), "луч появился без попадания");

        // Оригинал обработал попадание: передний щит уменьшился.
        mem[Z80_SHIELD_FRONT] = 0x40;
        f.sync_from_z80(&mem, true);
        assert!(
            !f.enemy_shots.is_empty(),
            "падение щита в Z80 LINK не показано лучом"
        );
        let shot = &f.enemy_shots[0];
        assert!(
            shot.from[2] > NEAR,
            "луч противника начался не у корабля: {:?}",
            shot.from
        );
    }

    #[test]
    fn z80_link_does_not_simulate_native_hits() {
        // В Z80 LINK попадания считает оригинальная игра. Локальный тик не
        // должен удалять объект, пришедший из памяти, и накручивать победы.
        let mut f = open();
        f.source = FlightSource::Z80;
        f.bodies.clear();
        f.bodies.push(Body {
            kind: BodyKind::Ship,
            model: None,
            pos: [0.0, 0.0, 1000.0],
            rot: Rotation::identity(),
            spin: [0.0; 3],
            spin_rate: 0.0,
            radius: 100.0,
            color: FG,
            fire_timer: 0.0,
        });
        f.tick(
            Controls {
                fire: true,
                ..Controls::default()
            },
            1.0 / 50.0,
        );
        assert_eq!(f.kills, 0, "победа накручена без участия Z80");
        assert_eq!(
            f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count(),
            1,
            "корабль из памяти Z80 удалён нативным выстрелом"
        );
    }

    #[test]
    fn switching_to_native_rebuilds_scene_after_z80_sync() {
        // Z80 LINK затирает сцену данными из памяти эмулятора.
        let mut f = open();
        f.source = FlightSource::Z80;
        let mem = vec![0u8; 0x10000];
        f.sync_from_z80(&mem, true);
        assert_eq!(f.source, FlightSource::Z80);

        // Переключение в Native обязано собрать собственный мир заново.
        f.toggle_source();
        assert_eq!(f.source, FlightSource::Native);
        assert_eq!(
            f.bodies.len(),
            6,
            "нативная сцена не пересобрана (станция/3 корабля/планета/солнце)"
        );
        assert!(f.bodies.iter().any(|b| b.kind == BodyKind::Station));
        let station = f
            .bodies
            .iter()
            .find(|b| b.kind == BodyKind::Station)
            .unwrap();
        assert_eq!(station.pos, [1800.0, -420.0, 4200.0]);
        assert!(
            f.speed_fraction() > 0.0,
            "нулевая скорость — пустая шкала индикатора"
        );

        // После возврата в Native мир снова движется.
        if let Some(s) = f.stars.first_mut() {
            s.pos = [200.0, 0.0, 2000.0];
        }
        let start = f.stars[0].pos;
        for _ in 0..20 {
            f.tick(Controls::default(), 1.0 / 50.0);
        }
        let moved = f.stars[0].pos;
        let step = (0..3).map(|i| (moved[i] - start[i]).abs()).sum::<f32>();
        assert!(step > 1.0, "в Native звёзды не движутся");

        // И это действительно рисуется: на кадре есть цвет планеты.
        let font = vec![0xFFu8; 0x300];
        let mut frame = Frame::new(1280, 960);
        f.draw(&mut frame, &font);
        assert!(
            frame.pixels().iter().any(|&p| is_planet_green(p)),
            "планета не нарисована после возврата в Native"
        );
    }

    #[test]
    fn pitch_changes_orientation_and_forward_motion() {
        let mut f = open();
        let before = f.rot;
        let speed_before = f.speed;
        f.tick(
            Controls {
                pitch_up: true,
                throttle_up: true,
                ..Controls::default()
            },
            1.0 / 50.0,
        );
        assert!(f.rot.m != before.m, "тангаж не изменил ориентацию");
        assert!(f.speed > speed_before, "тяга не увеличила скорость");
        let start = f.pos;
        for _ in 0..50 {
            f.tick(Controls::default(), 1.0 / 50.0);
        }
        assert!(f.pos != start, "корабль не движется вперёд");
    }

    #[test]
    fn pitch_up_moves_world_down() {
        // Если нос поднимается (S), звёзды и объекты впереди должны уезжать
        // вниз по экрану. Раньше знак был перепутан.
        let mut f = open();
        f.stars = vec![Star {
            pos: [0.0, 0.0, 1000.0],
            mag: 128,
        }];
        f.tick(Controls { pitch_up: true, ..Controls::default() }, 1.0 / 50.0);
        assert!(
            f.stars[0].pos[1] < 0.0,
            "при подъёме носа звезда ушла вверх: {}",
            f.stars[0].pos[1]
        );
    }

    #[test]
    fn stars_move_faster_with_speed() {
        let star = |pos: [f32; 3]| Star { pos, mag: 128 };
        let mut slow = open();
        slow.speed = 0.0;
        slow.stars = vec![star([200.0, 0.0, 2000.0])];
        let mut fast = open();
        fast.speed = SPEED_MAX;
        fast.stars = vec![star([200.0, 0.0, 2000.0])];

        let start = slow.stars[0].pos;
        slow.tick(Controls::default(), 0.1);
        let slow_step = (slow.stars[0].pos[0] - start[0]).abs();
        let start = fast.stars[0].pos;
        fast.tick(Controls::default(), 0.1);
        let fast_step = (fast.stars[0].pos[0] - start[0]).abs();

        assert!(slow_step > 0.0, "на минимальной скорости звёзды стоят");
        assert!(
            fast_step > slow_step * 2.0,
            "скорость слабо влияет на звёзды: {slow_step} -> {fast_step}"
        );
        assert!(fast.stars[0].pos[2] > 0.0, "звезда должна оставаться впереди");
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
        assert_eq!(star_count_for(256, 192), STAR_COUNT);
        let small = star_count_for(640, 480);
        let large = star_count_for(1280, 800);
        assert!(small < large, "плотность звёзд не растёт с разрешением");
        assert!(large >= 150, "на большом экране звёзд всё ещё мало: {large}");
        assert!(star_count_for(4000, 3000) <= STAR_COUNT_MAX);
    }

    #[test]
    fn stars_cross_view_at_calm_pace() {
        // Регрессия на «звёзды летят слишком быстро»: на крейсерской скорости
        // звезда должна идти от радиуса 0.3 до края кадра несколько секунд,
        // а не доли секунды.
        let mut f = open();
        f.speed = 12.0;
        f.stars = vec![Star {
            pos: [600.0, 0.0, 2000.0],
            mag: 128,
        }];
        let mut frames = 0;
        while frames < 2000 {
            let s = &f.stars[0];
            if s.pos[0] / s.pos[2] >= 1.0 {
                break;
            }
            f.tick(Controls::default(), 1.0 / 50.0);
            frames += 1;
        }
        let seconds = frames as f32 / 50.0;
        assert!(
            (2.0..12.0).contains(&seconds),
            "звезда пересекает кадр за {seconds:.2} с — темп не похож на оригинал"
        );
    }

    #[test]
    fn speed_gauge_matches_original_full_range() {
        let mut f = open();
        // В памяти было 0x50 = 5*15+5, значит скорость 15 из 31.
        assert!((f.speed - 15.0).abs() < 1e-6);
        assert!((f.speed_fraction() - 16.0 / 32.0).abs() < 1e-6);
        // На нуле шкала не пустая, на максимуме — полная.
        f.speed = 0.0;
        assert!((f.speed_fraction() - 1.0 / 32.0).abs() < 1e-6);
        f.speed = SPEED_MAX;
        assert!((f.speed_fraction() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn thrust_reaches_maximum_and_brake_returns_to_minimum() {
        let mut f = open();
        let frames = (SPEED_MAX / SPEED_ACCEL * 50.0).ceil() as usize + 10;
        for _ in 0..frames {
            f.tick(
                Controls {
                    throttle_up: true,
                    ..Controls::default()
                },
                1.0 / 50.0,
            );
        }
        assert!(
            (f.speed - SPEED_MAX).abs() < 1e-3,
            "тяга не выводит на максимум: {}",
            f.speed
        );
        for _ in 0..frames {
            f.tick(
                Controls {
                    throttle_down: true,
                    ..Controls::default()
                },
                1.0 / 50.0,
            );
        }
        assert!(f.speed < 1e-3, "торможение не сбрасывает скорость: {}", f.speed);
    }

    #[test]
    fn flight_screen_is_opaque_and_styled_like_splash() {
        let mut f = open();
        let font = vec![0xFFu8; 0x300];
        let mut frame = Frame::new(1280, 960);
        f.draw(&mut frame, &font);
        // Фон полностью непрозрачный, как на заставке.
        assert!(
            frame.pixels().iter().all(|&p| p & 0xFF00_0000 != 0),
            "на экране полёта есть прозрачные пиксели"
        );
        assert_eq!(frame.pixels()[0], BG);
        assert!(
            frame.pixels().iter().any(|&p| p == FG),
            "нет заголовка/приборов"
        );
        assert!(
            frame.pixels().iter().any(|&p| p == HINT),
            "нет жёлтой подсказки"
        );
        assert!(
            frame.pixels().iter().any(|&p| p == CYAN),
            "объекты и перекрестие не нарисованы"
        );
        assert!(
            frame.pixels().iter().any(|&p| is_planet_green(p)),
            "цвет планеты не взят из начальных данных системы"
        );
        if let Ok(path) = std::env::var("ELITE_DUMP_FLIGHT") {
            write_ppm(&path, frame.pixels(), frame.width, frame.height);
        }
        // Also dump Z80 mode
        if let Ok(path) = std::env::var("ELITE_DUMP_FLIGHT_Z80") {
            let mut f2 = open();
            f2.source = FlightSource::Z80;
            // Create dummy memory with some objects
            let mut mem = vec![0u8; 65536];
            // put some values
            let mut frame2 = Frame::new(1280, 960);
            f2.draw(&mut frame2, &font);
            write_ppm(&path, frame2.pixels(), frame2.width, frame2.height);
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
    fn firing_destroys_target_ahead() {
        let mut f = open();
        // Ставим первый корабль точно по курсу, чтобы попадание было
        // предсказуемым (сцена сдвигается только в тесте).
        if let Some(b) = f.bodies.iter_mut().find(|b| b.kind == BodyKind::Ship) {
            b.pos = [0.0, 0.0, 2500.0];
        }
        let before = f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count();
        f.tick(
            Controls {
                fire: true,
                ..Controls::default()
            },
            1.0 / 50.0,
        );
        let after = f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count();
        assert_eq!(after, before - 1, "выстрел по цели прямо по курсу не попал");
        assert_eq!(f.kills, 1);
        assert!(f.laser_heat > 0.0);
    }

    #[test]
    fn closing_returns_to_z80() {
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
        mem[Z80_FUEL] = 0xA0;
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

        // Солнце и планета в координатах вида.
        let sun = Z80_SUN_CRD;
        mem[sun] = 0x64; // x = 100
        mem[sun + 3] = 0xC8; // y = 200
        mem[sun + 6] = 0x2C;
        mem[sun + 7] = 0x01; // z = 300
        // Планета прямо по курсу: z = 0x018000.
        mem[Z80_PLANET_CRD + 7] = 0x80;
        mem[Z80_PLANET_CRD + 8] = 0x01;

        mem
    }

    #[test]
    fn z80_link_reads_flight_data_from_memory() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mem = z80_mem_with_flight();
        f.sync_from_z80(&mem, true);

        assert!((f.speed - 0x17 as f32).abs() < 1e-6);
        assert!((f.shield - 0x50 as f32 / 0xA0 as f32).abs() < 1e-6);
        assert!((f.fuel - 1.0).abs() < 1e-6);
        assert!((f.laser_heat - 0x20 as f32 / 0xA0 as f32).abs() < 1e-6);
        // Камера — в начале координат вида.
        assert_eq!(f.pos, [0.0; 3]);

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
        assert_eq!(sun.pos, [100.0, 200.0, 300.0]);

        // Пустые слоты и слот ракеты не должны попасть в сцену.
        assert_eq!(
            f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count(),
            1
        );
    }

    #[test]
    fn z80_link_skips_empty_slots_and_unknown_models() {
        let mut f = open();
        f.source = FlightSource::Z80;
        let mut mem = vec![0u8; 0x10000];
        // Энергия есть, а модели по адресу нет — объект не рисуем.
        let sh = Z80_OBJ_SLOT0;
        mem[sh + Z80_SLOT_OENERG] = 0x10;
        mem[sh + Z80_SLOT_OADRDATA] = 0x34;
        mem[sh + Z80_SLOT_OADRDATA + 1] = 0x12;
        f.sync_from_z80(&mem, true);
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
        m.load(crate::KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = crate::KLADOV_STATE_A;
        c.f = crate::KLADOV_STATE_F;
        c.b = (crate::KLADOV_STATE_BC >> 8) as u8;
        c.c = (crate::KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (crate::KLADOV_STATE_DE >> 8) as u8;
        c.e = (crate::KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (crate::KLADOV_STATE_HL >> 8) as u8;
        c.l = (crate::KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = crate::KLADOV_STATE_IX;
        c.iy = crate::KLADOV_STATE_IY;
        c.sp = crate::KLADOV_STATE_SP;
        c.pc = crate::KLADOV_STATE_PC;
        c.i = crate::KLADOV_I;
        c.r = crate::KLADOV_STATE_R;
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
    fn z80_link_reflects_live_emulator_flight() {
        let mut m = launch_kladov();
        let mut f = Flight::new();
        f.open(FlightSeed {
            system: "Lave",
            speed: 0,
            fuel: 0xA0,
            shield: 0xA0,
            planet_color: 0xFF40_80C0,
        });
        f.source = FlightSource::Z80;
        f.sync_from_z80(&m.mem, true);

        // Приборы и камера — из живой памяти игры.
        let gauge_speed = (m.mem[Z80_SPEED_GAUGE].saturating_sub(5) / 5) as f32;
        assert!(
            (f.speed - gauge_speed).abs() < 1e-6,
            "скорость не совпала со шкалой: {} != {gauge_speed}",
            f.speed
        );
        assert_eq!(f.pos, [0.0; 3], "камера Z80 LINK должна быть в начале координат");

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
        f.sync_from_z80(&m.mem, true);
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
}
