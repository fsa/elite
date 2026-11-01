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
//! к гиперпрыжку слева в этой же строке идёт обратный отсчёт; других
//! надписей прыжок не показывает.
//!
//! Объекты (корабли, станция, планета, солнце) рисуются общим рендером
//! [`crate::render3d`], что и корабль на заставке: непрозрачные тёмные грани
//! и белые рёбра.
//!
//! Подсистемы модуля:
//!
//! - [`memory`] — адреса и разбор полётного состояния из памяти Z80;
//! - [`scene`] — объекты сцены из слотов Z80, солнце и планета;
//! - [`stars`] — звёздное поле, его вращение и темп;
//! - [`combat`] — лазеры, ответные лучи и вспышка попадания;
//! - [`transition`] — вылет, гиперпрыжок и стыковка;
//! - [`messages`] — перехваченные сообщения игры и надписи вида;
//! - [`view`] — направления кабины, камера и отсечение отрезков;
//! - [`render`] — отрисовка готового состояния в кадр.

use crate::cockpit::{self, CockpitGeom};
use crate::display::Frame;
use crate::machine::GameMsgTap;
use crate::render3d::Rotation;

use self::combat::Combat;
use self::memory::{
    Z80_COBRA_MK4, Z80_HIGH_ANGLE_DU, Z80_HIGH_ANGLE_LR, Z80_HIGH_DELTA_SPEED, Z80_MEM_MIN,
    Z80_SPEED_GAUGE, Z80_STARS_FAST_SCROLL, Z80_STARS_LEN, Z80_STARS_TABLE,
};
use self::messages::Messages;
use self::palette::{planet_green, PLANET_COLOR};
use self::scene::Body;
use self::stars::{
    StarField, SPEED_MAX, STAR_ANGLE_DU_PER_UNIT, STAR_ANGLE_LR_PER_UNIT, STAR_COUNT,
};
use self::transition::TransitionState;

mod combat;
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
    z80_in_cockpit, z80_in_transition, z80_is_docked, Z80_FLG_JUMP_DOCKING, Z80_IND_DANGER,
    Z80_JUMP_COUNTDOWN, Z80_JUMP_DOCKING, Z80_WHAT_ON_SCREEN,
};
pub use self::transition::TransitionKind;
pub use self::view::ViewDir;

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
    /// У игрока Cobra Mk IV (`byte_D08E`) — фиолетовые клетки панели.
    cobra_mk4: bool,
    bodies: Vec<Body>,
    stars: StarField,
    planet_color: u32,
    /// Нативная анимация вылета/прыжка/стыковки вместо ZX-ролика.
    transition: TransitionState,
    /// Боевые эффекты: лазеры и попадания.
    combat: Combat,
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
            cobra_mk4: false,
            bodies: Vec::new(),
            stars: StarField::new(STAR_COUNT),
            planet_color: PLANET_COLOR,
            transition: TransitionState::new(),
            combat: Combat::new(),
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
        self.cobra_mk4 = false;
        self.bodies.clear();
        self.transition.reset();
        self.combat.reset();
        self.messages.reset();
        if planet_color != 0 {
            self.planet_color = planet_green(planet_color);
        }
        self.stars.reset(STAR_COUNT);
    }

    /// Начать нативную анимацию перехода.
    ///
    /// `launch` передаётся из `F1` на станции: по памяти игры вылет и стыковку
    /// не отличить (оба взводят `JumpDocking = 1`), а вид должен знать, что
    /// показывать — уход станции или тоннель.
    pub fn start_transition(&mut self, launch: bool, mem: &[u8]) {
        self.transition.start(launch, mem);
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

    /// Переключить вид по сторонам (F1-F4 в нативном полёте): те же клавиши
    /// 1-4, которыми оригинал меняет `WhatOnScreen`.
    pub fn set_view_dir(&mut self, dir: ViewDir) {
        if !self.active {
            return;
        }
        // Поле сферическое и общее для всех видов: при смене направления его
        // не пересоздаём, иначе звёзды снова окажутся только перед кораблём.
        self.view_dir = dir;
    }

    /// Закрыть экран и вернуться к ZX-кадру (клавиша `1`).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Забрать перехваченное игровое сообщение (заполняет хук печати
    /// `sub_D499`; см. [`crate::machine::GameMsgTap`]).
    pub fn sync_game_msg(&mut self, tap: &GameMsgTap) {
        if !self.active {
            return;
        }
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
        if memory::z80_in_transition(mem) {
            // Ролик теперь рисуем сами: нативный вид остаётся открытым и
            // получает состояние перехода. Вылет/прыжок/стыковку по флагам
            // различает `start_transition` (для вылета признак передаёт App).
            if !self.transition.in_transition() {
                self.transition.start(false, mem);
            }
            return;
        }
        self.transition.reset();
        if !memory::z80_in_cockpit(mem) {
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

        // Отсчёт гиперпрыжка ведёт Z80; имя цели читается там же.
        self.transition.sync(mem);

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
            let roll = memory::read_signed8(mem[Z80_HIGH_ANGLE_LR]) * STAR_ANGLE_LR_PER_UNIT;
            let pitch = -memory::read_signed8(mem[Z80_HIGH_ANGLE_DU]) * STAR_ANGLE_DU_PER_UNIT;
            let turn = Rotation::axis_angle([0.0, 0.0, 1.0], roll)
                .mul(&Rotation::axis_angle([1.0, 0.0, 0.0], pitch));
            // Каждый проход `sub_78B1` поворачивает поле, поэтому при J-драйве
            // за такт набегает тройной поворот.
            for _ in 0..if fast { 3 } else { 1 } {
                self.stars.turn(&turn);
            }
            self.stars.mark_boost(fast);
            self.z80_fast_scroll = false;
        }

        // Выстрел в оригинале виден только по нагреву лазера.
        self.combat.observe_laser(mem);

        // Сцена собирается из слотов Z80; попадание ловим по падению щита
        // уже после того, как известны цели ответного луча.
        self.bodies = scene::build(mem, self.planet_color);
        self.combat
            .observe_shield(mem, &self.bodies, self.stars.rng_mut());
    }

    /// Один шаг анимации: таймеры лучей и движение звёзд.
    ///
    /// Мир не двигается — его координаты приходят из [`Self::sync_from_z80`].
    pub fn tick(&mut self, dt: f32) {
        if !self.active {
            return;
        }
        let dt = dt.clamp(0.0, 0.1);
        self.combat.tick(dt);
        self.transition.tick(dt);
        self.stars.advance(dt, self.speed);
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

impl Default for Flight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
