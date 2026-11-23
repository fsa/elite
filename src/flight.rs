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
//! держится название направления («Front View» и т.п.). При подготовке
//! к гиперпрыжку слева в этой же строке идёт обратный отсчёт, а внизу —
//! перехваченная из игры строка `HYPERSPACE -- <планета>`.
//!
//! Объекты (корабли, станция, планета, солнце) рисуются общим рендером
//! [`crate::render3d`], что и корабль на заставке: непрозрачные
//! металлические грани и белые рёбра. В полёте небольшой оттенок получают
//! только боевые корабли и их обломки; станция, ракеты, солнце и планета
//! остаются исходных цветов.
//!
//! Подсистемы модуля:
//!
//! - [`memory`] — адреса и разбор полётного состояния из памяти Z80;
//! - [`scene`] — объекты сцены из слотов Z80, солнце и планета;
//! - [`stars`] — звёздное поле, его вращение и темп;
//! - [`combat`] — лазеры, ответные лучи и вспышка попадания;
//! - [`explosions`] — облака частиц после уничтожения объекта;
//! - [`transition`] — вылет, гиперпрыжок и стыковка;
//! - [`messages`] — перехваченные сообщения игры и надписи вида;
//! - [`view`] — направления кабины, камера и отсечение отрезков;
//! - [`render`] — отрисовка готового состояния в кадр.

use crate::cockpit::{self, CockpitGeom};
use crate::display::Frame;
use crate::machine::GameMsgTap;
use crate::render3d::Rotation;

use self::combat::Combat;
use self::explosions::Explosions;
use self::memory::{
    Z80_HIGH_DELTA_SPEED, Z80_MEM_MIN, Z80_SPEED_GAUGE, Z80_STARS_FAST_SCROLL, Z80_STARS_LEN,
    Z80_STARS_TABLE,
};
use self::messages::Messages;
use self::palette::{planet_green, PLANET_COLOR};
use self::scene::{Body, ShipTints};
use self::stars::{
    StarField, SPEED_MAX, STAR_ANGLE_DU_PER_UNIT, STAR_ANGLE_LR_PER_UNIT, STAR_COUNT,
};
use self::transition::TransitionState;

mod combat;
mod explosions;
mod memory;
mod messages;
mod palette;
mod render;
mod rng;
mod scene;
mod stars;
mod transition;
mod view;

// Часть адресов нужна только тестам `app`, но остаётся публичной частью API
// модуля полёта, поэтому не считаем их неиспользуемыми в обычной сборке.
#[allow(unused_imports)]
pub use self::memory::{
    z80_in_cockpit, z80_in_transition, z80_is_docked, z80_player_dead, z80_radar_view,
    z80_stop_rotation, Z80_COBRA_MK4, Z80_COMPASS_FLAG, Z80_CONTROL_KEY1, Z80_CONTROL_KEY2,
    Z80_FLG_SAVE_SCANNER, Z80_FLG_JUMP_DOCKING, Z80_FLAGS_SETUP, Z80_HIGH_ANGLE_DU,
    Z80_HIGH_ANGLE_LR, Z80_IND_DANGER, Z80_JUMP_COUNTDOWN, Z80_JUMP_DOCKING, Z80_LASER_MOUNTS,
    Z80_MAP_ROTATION, Z80_PLAYER_GAME_OVER, Z80_PRESS_KEY_SYMBOL, Z80_SCANNER_INVERT,
    Z80_SCANNER_MODE,
    Z80_SCREEN_INFO_PLANET, Z80_SCREEN_LOCAL_MAP, Z80_SCREEN_MARKET, Z80_SCREEN_PILOT_INFO,
    Z80_WHAT_ON_SCREEN,
};
pub use self::transition::TransitionKind;
pub use self::view::ViewDir;

/// Число отсчёта для показа вне вида из кабины (`JumpCountdown >> 1`).
///
/// Нативный вид рисует его сам через `render`, а informational-экраны берут
/// готовое число отсюда — то же самое, что показывает подпись в кабине.
pub fn countdown_number(raw: u8) -> u32 {
    messages::countdown_number(raw)
}

/// Цвет отсчёта на информационных экранах: тот же, что подпись вида в кабине.
pub fn countdown_color() -> u32 {
    self::palette::VIEW_LABEL_COLOR
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
    /// Снимок таблицы звёзд Z80: её изменение означает игровой такт.
    prev_star_table: Option<[u8; Z80_STARS_LEN]>,
    /// В памяти был замечен взвод `StarsFastScroll` (J-драйв).
    z80_fast_scroll: bool,
    /// Скорость в единицах `HighDeltaSpeed` (`0..=31`) — темп звёзд.
    speed: f32,
    /// Ждём, пока `HighDeltaSpeed` снова совпадёт с показанием скорости.
    ///
    /// После вылета со станции Z80 на несколько тактов пишет в
    /// `HighDeltaSpeed` служебное `7` (конец ролика), хотя индикатор скорости
    /// ещё хранит прежнее значение. Это не J-драйв, и разгон звёзд по такому
    /// расхождению включать нельзя.
    speed_resync: bool,
    /// У игрока Cobra Mk IV (`byte_D08E`) — фиолетовые клетки панели.
    cobra_mk4: bool,
    /// Типы лазеров в креплениях FRONT/REAR/LEFT/RIGHT (`0` — нет лазера).
    lasers: [u8; 4],
    /// Корабль игрока погиб (`PlayerGameOver`): на виде рисуется «Game Over»
    /// вместо подписи направления.
    game_over: bool,
    bodies: Vec<Body>,
    /// Оттенки кораблей, привязанные к слотам Z80.
    ship_tints: ShipTints,
    /// Положение/ориентация тел на предыдущем игровом такте Z80.
    /// Используется для интерполяции между тактами (~5.5 Гц), чтобы корабли
    /// не «прыгали» при отрисовке 50 Гц.
    prev_bodies: Vec<Body>,
    /// Время, прошедшее с последнего игрового такта Z80.
    tick_accum: f32,
    /// Длительность последнего интервала между игровыми тактами Z80.
    tick_interval: f32,
    stars: StarField,
    planet_color: u32,
    /// Цвет системы на карте галактики — оттенок почти белой звезды.
    system_color: u32,
    /// Нативная анимация вылета/прыжка/стыковки вместо ZX-ролика.
    transition: TransitionState,
    /// После стыковки оригинал сбрасывает флаги перехода раньше, чем уходит
    /// с кабинного экрана, а `IndDanger = 0` может записать ещё позже. В этот
    /// промежуток вид должен оставаться тёмным: обычная сцена показывалась бы
    /// «сквозь станцию».
    pub(super) docking_blackout: bool,
    /// Боевые эффекты: лазеры и попадания.
    combat: Combat,
    /// Облака частиц на месте уничтоженных объектов.
    explosions: Explosions,
    /// Последнее сообщение, перехваченное из игры (см. [`GameMsgTap`]).
    messages: Messages,
}

impl Flight {
    pub fn new() -> Self {
        Flight {
            active: false,
            view_dir: ViewDir::Forward,
            seen_flight_screen: false,
            prev_star_table: None,
            z80_fast_scroll: false,
            speed: 0.0,
            speed_resync: false,
            cobra_mk4: false,
            lasers: [0; 4],
            game_over: false,
            bodies: Vec::new(),
            ship_tints: ShipTints::new(),
            prev_bodies: Vec::new(),
            tick_accum: 0.0,
            tick_interval: 1.0 / 5.5,
            stars: StarField::new(STAR_COUNT),
            planet_color: PLANET_COLOR,
            system_color: 0,
            transition: TransitionState::new(),
            docking_blackout: false,
            combat: Combat::new(),
            explosions: Explosions::new(),
            messages: Messages::new(),
        }
    }

    /// Открыть экран полёта. `planet_color` — цвет планеты текущей системы
    /// (ноль оставляет цвет по умолчанию).
    pub fn open(&mut self, planet_color: u32) {
        self.active = true;
        self.view_dir = ViewDir::Forward;
        self.seen_flight_screen = false;
        self.prev_star_table = None;
        self.z80_fast_scroll = false;
        self.speed = 0.0;
        self.speed_resync = false;
        self.cobra_mk4 = false;
        self.lasers = [0; 4];
        self.game_over = false;
        self.bodies.clear();
        self.ship_tints.reset();
        self.prev_bodies.clear();
        self.tick_accum = 0.0;
        self.tick_interval = 1.0 / 5.5;
        self.transition.reset();
        self.docking_blackout = false;
        self.combat.reset();
        self.explosions.reset();
        self.messages.reset();
        if planet_color != 0 {
            self.planet_color = planet_green(planet_color);
        }
        self.system_color = planet_color;
        self.stars.reset(STAR_COUNT);
    }

    /// Начать нативную анимацию перехода.
    ///
    /// `launch` передаётся из `F1` на станции: по памяти игры вылет и стыковку
    /// не отличить (оба взводят `JumpDocking = 1`), а вид должен знать, что
    /// показывать — уход станции или тоннель.
    pub fn start_transition(&mut self, launch: bool, mem: &[u8]) {
        self.transition.start(launch, mem);
        self.docking_blackout = false;
        if launch {
            self.speed_resync = true;
        }
        self.explosions.reset();
    }

    /// Начать анимацию перехода с заранее известным видом (например,
    /// гиперпрыжка, отсчёт которого уже закончился к моменту открытия F1).
    pub fn start_transition_kind(&mut self, kind: TransitionKind, countdown: u8) {
        self.transition.start_kind(kind, countdown);
        self.docking_blackout = false;
        if kind == TransitionKind::Launch {
            self.speed_resync = true;
        }
        self.explosions.reset();
    }

    /// Идёт ли сейчас нативный переход (вылет, прыжок или стыковка).
    ///
    /// Используется тестами; боевой код смотрит на состояние перехода напрямую.
    #[allow(dead_code)]
    pub fn in_transition(&self) -> bool {
        self.transition.in_transition()
    }

    /// Какой переход сейчас идёт (для тестов и отладки).
    #[allow(dead_code)]
    pub fn transition_kind(&self) -> Option<TransitionKind> {
        self.transition.kind()
    }

    /// Идёт ли режим анимации: вылет, гиперпереход или стыковка. В этом
    /// режиме отображается только передний вид.
    pub fn in_animation_mode(&self) -> bool {
        self.transition.kind().is_some()
    }

    /// Переключить вид по сторонам (F1-F4 в нативном полёте): те же клавиши
    /// 1-4, которыми оригинал меняет `WhatOnScreen`.
    ///
    /// В режиме анимации (вылет/гиперпереход) вид всегда передний.
    pub fn set_view_dir(&mut self, dir: ViewDir) {
        if !self.active {
            return;
        }
        if self.in_animation_mode() {
            self.view_dir = ViewDir::Forward;
            return;
        }
        // Поле сферическое и общее для всех видов: при смене направления его
        // не пересоздаём, иначе звёзды снова окажутся только перед кораблём.
        self.view_dir = dir;
    }

    /// Есть ли лазер в направлении текущего вида.
    ///
    /// Прицел и луч собственного лазера показываются только там, где оригинал
    /// действительно может стрелять. Сам выстрел по-прежнему решает Z80.
    pub(super) fn laser_in_view(&self) -> bool {
        self.lasers[self.view_dir.laser_mount()] != 0
    }

    /// Закрыть экран и вернуться к ZX-кадру (клавиша `1`).
    ///
    /// Состояние анимации тоже сбрасывается: закрытый вид не может её
    /// проигрывать. Временное скрытие ролика через `Backquote` делает это
    /// напрямую через `active`, чтобы сохранить анимацию для возврата.
    pub fn close(&mut self) {
        self.active = false;
        self.transition.reset();
        self.docking_blackout = false;
        self.messages.reset();
    }

    /// Постареть нижнее сообщение на `frames` кадров. Вызывается до
    /// [`Self::sync_game_msg`], чтобы свежее событие начинало новый отсчёт.
    pub fn advance_messages(&mut self, frames: u64) {
        self.messages.advance(frames.min(u32::MAX as u64) as u32);
    }

    /// Забрать перехваченные строки игры (см. [`crate::machine::GameMsgTap`]).
    /// Для показа берётся только нижняя строка (`0x0D`).
    pub fn sync_game_msg(&mut self, tap: &mut GameMsgTap) {
        self.messages.sync(tap);
    }

    /// Следить за экраном, который показывает Z80, и закрывать нативный вид,
    /// когда игра уходит с вида из кабины.
    ///
    /// Игровые клавиши (`O`, `I`, `Enter` и т.п.) всегда уходят в эмулятор,
    /// поэтому оригинал сам переключает `WhatOnScreen`. Без этой проверки
    /// нативный вид оставался поверх ZX-кадра, и карта/инвентарь открывались
    /// «в фоне». Переходы вылета/прыжка/стыковки не закрывают вид: он сам
    /// рисует их анимацию, а Z80 в это время ведёт отсчёт и логику игры.
    /// Гибель корабля (`PlayerGameOver`) тоже не закрывает вид — оригинал ещё
    /// держит кабину, а нативный рендер показывает «Game Over»; на заставку
    /// вид закрывает приложение (см. `App::follow_flight_screen`).
    ///
    /// `launch` передаёт приложение: по памяти в момент перехода вылет и
    /// стыковку различить нельзя (`JumpDocking` у обоих равен `1`, а
    /// `IndDanger` к моменту проверки уже изменён обеими сторонами).
    pub fn follow_z80_screen(&mut self, mem: &[u8], launch: bool) {
        let Some(&screen) = mem.get(Z80_WHAT_ON_SCREEN) else {
            return;
        };
        if screen < 4 {
            self.seen_flight_screen = true;
        }
        if memory::z80_in_transition(mem) {
            // Ролик теперь рисуем сами: нативный вид остаётся открытым и
            // получает состояние перехода. Если вид временно скрыт (показ
            // эмулятора), состояние перехода всё равно поддерживаем, чтобы F1
            // вернул ту же анимацию.
            if !self.transition.in_transition() {
                self.transition.start(launch, mem);
                self.explosions.reset();
            }
            return;
        }
        let was_docking = matches!(self.transition.kind(), Some(TransitionKind::Docking));
        self.transition.reset();
        if !self.active {
            return;
        }
        if !memory::z80_in_cockpit(mem) {
            // `0x0B` — экран станции. После запуска со станции Z80 держит его
            // несколько кадров, пока идёт анимация вылета; уже открытый экран
            // не закрываем до первого настоящего полётного кадра.
            if screen == 0x0B && !self.seen_flight_screen {
                return;
            }
            self.docking_blackout = false;
            self.active = false;
            return;
        }
        // Оригинал гасит флаги перехода, пока ещё держит вид из кабины.
        // `IndDanger = 0` он может записать позже, поэтому одним только
        // признаком «пристыкован» пользоваться нельзя: между сбросом флагов
        // и уходом на информационный экран нативный вид рисовал бы обычный
        // космос. Держим чёрный фон до смены экрана Z80.
        if was_docking && memory::z80_in_cockpit(mem) {
            self.docking_blackout = true;
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
        if mem.len() < Z80_MEM_MIN {
            return;
        }

        // Скорость берём из индикатора (`5 * HighDeltaSpeed + 5`): на такте
        // J-драйва `PrintStars` временно пишет в `HighDeltaSpeed` `0x1E`/`0x10`,
        // и прямой опрос заставлял бы шкалу и звёзды прыгать.
        let gauge = mem[Z80_SPEED_GAUGE];
        self.speed = (gauge.saturating_sub(5) as f32 / 5.0).clamp(0.0, SPEED_MAX);
        // После вылета ждём, пока служебное значение `HighDeltaSpeed` сменится
        // настоящей скоростью: до этого расхождение с индикатором — не J-драйв.
        // Пока идёт сам ролик вылета, совпадение тоже ничего не значит: в его
        // конце игра ещё раз пишет служебное значение.
        let launching = matches!(self.transition.kind(), Some(TransitionKind::Launch));
        if self.speed_resync
            && !launching
            && mem[Z80_HIGH_DELTA_SPEED] as i32 == self.speed.round() as i32
        {
            self.speed_resync = false;
        }
        self.cobra_mk4 = mem.get(Z80_COBRA_MK4).is_some_and(|&b| b != 0);
        for (mount, laser) in self.lasers.iter_mut().enumerate() {
            *laser = memory::laser_type(mem, mount);
        }
        self.game_over = memory::z80_player_dead(mem);
        if self.game_over {
            // Оригинал на гибели пишет `HighDeltaSpeed = 0`, но индикатор
            // скорости больше не обновляет. Старое значение из индикатора
            // заставляло бы звёзды лететь дальше, хотя камера уже стоит.
            self.speed = 0.0;
        }
        if self.game_over || self.in_animation_mode() {
            // При взрыве корабля и в режиме анимации (вылет/гиперпереход)
            // показывается только передний вид.
            self.view_dir = ViewDir::Forward;
        }

        // Отсчёт гиперпрыжка ведёт Z80; имя цели читается там же.
        self.transition.sync(mem);
        if let Some(t) = &mut self.transition.current {
            if t.kind == TransitionKind::Hyperspace && t.warp && t.warp_initial_speed.is_none() {
                t.warp_initial_speed = Some(self.speed);
            }
        }

        // Переход в режим анимации гиперперехода мог произойти на этом шаге
        // (например, отсчёт закончился и начался тоннель) — фиксируем вид.
        if self.in_animation_mode() {
            self.view_dir = ViewDir::Forward;
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
        let prev_bodies_snapshot = self.bodies.clone();
        if ticked {
            // J-драйв `PrintStars` проматывает звёзды тремя проходами
            // `sub_78B1` (скорости `0x1E` и дважды `0x10`) и на этом такте
            // временно перезаписывает `HighDeltaSpeed`. Ловим и сам флаг, и
            // этот след, если флаг не попал в кадр опроса. При гибели игра
            // сама обнуляет `HighDeltaSpeed`, это не J-драйв: иначе звёзды
            // получают ложное ускорение.
            let fast = !self.game_over
                && (self.z80_fast_scroll
                    || (!self.speed_resync
                        && mem[Z80_HIGH_DELTA_SPEED] as i32 != self.speed.round() as i32));
            let roll = memory::read_signed8(mem[Z80_HIGH_ANGLE_LR]) * STAR_ANGLE_LR_PER_UNIT;
            let pitch = -memory::read_signed8(mem[Z80_HIGH_ANGLE_DU]) * STAR_ANGLE_DU_PER_UNIT;
            let turn = Rotation::axis_angle([0.0, 0.0, 1.0], roll)
                .mul(&Rotation::axis_angle([1.0, 0.0, 0.0], pitch));
            if self.game_over {
                // При гибели оригинал обнуляет скорость и блокирует управление:
                // камера стоит. Обломки из слотов Z80 уносятся своей скоростью,
                // а облака взрывов не поворачиваются и не летят на игрока.
                // Шаг `CalcNewCrdObj_Pl` при `HighDeltaSpeed == 0` равен 2.
                let delta_z = 2.0;
                self.explosions.tick_death(delta_z);
            } else {
                // Каждый проход `sub_78B1` поворачивает поле, поэтому при
                // J-драйве за такт набегает тройной поворот.
                for _ in 0..if fast { 3 } else { 1 } {
                    self.stars.turn(&turn);
                }
                self.stars.mark_boost(fast);
                // Облака живут в той же системе координат, что и тела: на
                // такте они поворачиваются вместе с миром и сдвигаются к
                // игроку на шаг `CalcNewCrdObj_Pl`.
                let delta_z = 2.0 * (self.speed.round() + 1.0);
                self.explosions.tick_world(&turn, delta_z);
            }
            self.z80_fast_scroll = false;
            self.prev_bodies = prev_bodies_snapshot;
            self.tick_interval = self.tick_accum.max(1.0 / 60.0);
            self.tick_accum = 0.0;
        }

        // Выстрел в оригинале виден только по нагреву лазера.
        self.combat.observe_laser(mem);

        // Сцена собирается из слотов Z80; попадание ловим по урону щитов уже
        // после того, как известны цели ответного луча.
        self.bodies = scene::build(
            mem,
            self.planet_color,
            self.system_color,
            &mut self.ship_tints,
        );
        if !ticked && self.prev_bodies.is_empty() {
            self.prev_bodies = self.bodies.clone();
        }
        self.combat.observe_shield(
            mem,
            &self.bodies,
            self.view_dir,
            ticked,
            self.stars.rng_mut(),
        );
        // Уничтожение ловится по `oEnerg == 0xFF`: так `FireLaser` помечает
        // объект перед `DestroyObj`.
        self.explosions.observe(mem);
    }

    /// Один шаг анимации: таймеры лучей и движение звёзд.
    ///
    /// Мир не двигается — его координаты приходят из [`Self::sync_from_z80`].
    pub fn tick(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.1);
        self.tick_accum += dt;
        self.combat.tick(dt);
        self.explosions.tick(dt);
        self.transition.tick(dt);
        // После гибели камера стоит: даже нулевая скорость даёт базовый шаг
        // `speed_b(0) == 1`, поэтому звёзды надо остановить явно.
        if self.game_over {
            return;
        }
        if let Some(t) = self.transition.current {
            if t.kind == TransitionKind::Hyperspace && t.warp {
                let initial_speed = t.warp_initial_speed.unwrap_or(self.speed);
                // Длительность совпадает с тоннелем: звёзды возвращаются к
                // исходной скорости к моменту, когда кольцо уходит за экран.
                let progress = t.warp_elapsed / render::transition::HYPERSPACE_EXIT_TIME;
                self.stars.advance_hyperspace(dt, initial_speed, progress);
            } else {
                self.stars.advance(dt, self.speed);
            }
        } else {
            self.stars.advance(dt, self.speed);
        }
    }

    /// Нарисовать вид из кабины в верхней части кадра.
    ///
    /// `geom` — геометрия кокпита: рендер заполняет область над панелью и
    /// рисует сцену только внутри главной рамки, которую потом обводит
    /// [`crate::cockpit::Cockpit`]. `font` — знакогенератор ZX для подписей
    /// вида и обратного отсчёта.
    pub fn draw(&mut self, frame: &mut Frame, geom: &CockpitGeom, font: &[u8]) {
        render::draw(self, frame, geom, font);
    }

    /// Тела сцены для рендера с интерполяцией между игровыми тактами Z80.
    ///
    /// Z80 обновляет координаты объектов на своём такте (~5.5 Гц), а рендер
    /// идёт с частотой кадра. Без интерполяции корабли и станция заметно
    /// «прыгают». Звёзды не интерполируются: они рисуются из живого поля,
    /// которое уже обновляется по `dt`.
    pub(in crate::flight) fn render_bodies(&self) -> Vec<Body> {
        let alpha = if self.tick_interval > 0.0 {
            (self.tick_accum / self.tick_interval).clamp(0.0, 1.0)
        } else {
            1.0
        };
        if alpha >= 0.999 || self.prev_bodies.is_empty() {
            return self.bodies.clone();
        }
        self.bodies
            .iter()
            .map(|body| {
                let Some(prev) = self.prev_bodies.iter().find(|p| p.source == body.source) else {
                    return body.clone();
                };
                // Другой тип, модель или оттенок в том же слоте — это уже
                // новый объект, интерполировать его положение не нужно.
                if prev.kind != body.kind
                    || prev.model.map(|m| m.addr) != body.model.map(|m| m.addr)
                    || prev.tint != body.tint
                {
                    return body.clone();
                }
                let mut out = body.clone();
                for k in 0..3 {
                    out.pos[k] = prev.pos[k] + (body.pos[k] - prev.pos[k]) * alpha;
                }
                out.rot = lerp_rotation(&prev.rot, &body.rot, alpha);
                out
            })
            .collect()
    }

    /// Цвет рамки кокпита: белый, при попадании — красный.
    pub fn cockpit_color(&self) -> u32 {
        if self.combat.damage_flash > 0.0 {
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
}

fn lerp_rotation(a: &Rotation, b: &Rotation, t: f32) -> Rotation {
    let mut m = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            m[i][j] = a.m[i][j] + (b.m[i][j] - a.m[i][j]) * t;
        }
    }
    // Переортонормализация: интерполяция матриц портит ортогональность,
    // а рендер ожидает оси СКО.
    let z = normalize3([m[0][2], m[1][2], m[2][2]]);
    let mut y = normalize3([m[0][1], m[1][1], m[2][1]]);
    let mut x = cross3(y, z);
    if len3(x) < 1e-6 {
        x = [1.0, 0.0, 0.0];
        y = cross3(z, x);
    } else {
        x = normalize3(x);
        y = cross3(z, x);
    }
    if len3(y) < 1e-6 {
        y = [0.0, 1.0, 0.0];
    } else {
        y = normalize3(y);
    }
    Rotation::from_axes(x, y, z)
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let l = len3(v);
    if l < 1e-6 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / l, v[1] / l, v[2] / l]
    }
}

fn len3(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

impl Default for Flight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
