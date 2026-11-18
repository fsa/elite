//! Оконное приложение на winit: окно, кадр, ввод и нативные экраны.
//!
//! В игре показывается полный ZX-кадр 256x192 (вид из кабины и родная
//! растровая панель оригинала) целым множителем по центру окна (см.
//! [`crate::display::layout`]), поверх — HD-слой
//! [`Spectrum::hd`](crate::machine::Spectrum).
//! Нативный кокпит (`src/cockpit.rs`) рисуется на заставке и на экране
//! полёта. Отдельные нативные экраны занимают окно целиком.
//!
//! Клавиши читаются как `physical_key` (позиция клавиши), поэтому активная
//! раскладка (в частности русская) на управление не влияет.

use std::process::exit;
use std::rc::Rc;
use std::time::{Duration, Instant};

use softbuffer::{Context, Surface};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::audio::{self, Audio};
use crate::cli::Options;
use crate::cockpit;
use crate::display;
use crate::docked;
use crate::equip;
use crate::flight;
use crate::hdmap;
use crate::machine::Spectrum;
use crate::market;
use crate::splash;
use crate::state::{Animation, GameState, Location, OriginalScreen, Screen};
use crate::status;
use crate::sysinfo;
use crate::ui::present::{DebugView, SRC_H, SRC_W};

mod frame;
mod input;
mod screens;
#[cfg(test)]
mod tests;
mod window;

/// Частота кадров ZX Spectrum: 50 кадров в секунду (кадр каждые 20 мс).
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 50);

/// Запустить оконный режим и крутить цикл событий до выхода.
pub(crate) fn run(machine: Spectrum, opts: &Options) {
    let event_loop = match EventLoop::new() {
        Ok(el) => el,
        Err(e) => {
            eprintln!("cannot create event loop: {e}");
            exit(1);
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(machine, opts);
    let result = event_loop.run_app(&mut app);
    app.finish();
    if let Err(e) = result {
        eprintln!("event loop error: {e}");
        exit(1);
    }
}

/// Состояние окна и нативных экранов.
struct App<'a> {
    machine: Spectrum,
    opts: &'a Options,
    audio: Option<Audio>,
    audio_buf: Vec<i16>,
    wav_samples: Option<Vec<i16>>,
    last_path: String,
    /// Централизованное состояние игры (место игрока, текущий экран, меню).
    /// Нативные компоненты читают его вместо прямых обращений к эмулятору.
    state: GameState,
    /// Открыто ли меню паузы (Esc) — игра не выполняется.
    menu: bool,
    /// Отдельная нативная карта галактики (F6) — заменяет экран игры.
    hdmap: hdmap::Map,
    /// Значение `WhatOnScreen` до открытия карты, чтобы вернуть его при
    /// закрытии. Пока карта открыта, игра работает как `SHORT RANGE CHART`.
    hdmap_screen: Option<u8>,
    /// Нативный экран закупки оборудования (F4) — заменяет экран игры.
    shop: equip::Shop,
    /// Нативный экран статуса пилота (F9) — альтернатива экрану `L`.
    status: status::Status,
    /// После кадра эмулятора открыть нативный экран статуса: сначала игра
    /// должна обработать `L` и перейти в режим показа капитана.
    pending_status: bool,
    /// Нативный экран информации о планете (F7) — альтернатива экрану `P`.
    sysinfo: sysinfo::SysInfo,
    /// После кадра эмулятора открыть нативный экран информации о планете:
    /// сначала игра должна обработать `P` и заполнить поля планеты.
    pending_sysinfo: bool,
    /// Экран-заглушка `DOCKED`, который на станции заменяет ответ эмулятора
    /// на `G`/`H` и показывается примерно секунду.
    docked: docked::Docked,
    /// Нативный (альтернативный) показ экрана цен на станции (`K`).
    market: market::Market,
    /// Экран полёта (F1) — нативный рендер по данным Z80. Пока он активен,
    /// окно занимает вид из кабины и нативная приборная панель; эмулятор
    /// продолжает работать и задаёт мир, поэтому ввод идёт в игру.
    flight: flight::Flight,
    /// Нативная заставка (титульный экран): пока игра на интро, эмуляция не
    /// идёт, а окно занимает нативный рендер (`splash`).
    splash: splash::Splash,
    /// Нативный кокпит (приборная панель) — рисуется в нижней части окна
    /// поверх ZX-кадра, растянут на всю ширину (16:10).
    cockpit: cockpit::Cockpit,
    /// Показания панели на заставке: пока случайные, но стабильные между
    /// кадрами, чтобы шкалы не мерцали.
    splash_values: cockpit::CockpitValues,
    /// Выбранный пункт меню паузы (0 — продолжить, 1 — сохранить, 2 — загрузить,
    /// 3 — выйти). Стрелки двигают выбор, Enter подтверждает.
    menu_sel: usize,
    /// Признак выхода из игры.
    quit: bool,
    buf: Vec<u32>,
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    /// Состояние ZX-матрицы (полуряд, бит) — обновляется событиями клавиш.
    matrix: [[bool; 5]; 8],
    /// Нажатие игровой клавиши, которым был закрыт нативный экран: один кадр
    /// принудительно подаётся в эмулятор, даже если клавишу уже отпустили.
    pending_key: Option<(usize, u8)>,
    /// После кадра эмулятора открыть нативный экран закупки. Нужно для `F4`:
    /// сначала оригинал должен перейти в режим `4` (equip), и только потом
    /// нативный экран встаёт на паузу.
    pending_shop: bool,
    /// Нативный вид, который нужно включить, как только Z80 окажется в
    /// полёте или начнёт переход. Нужен для `F1` на станции и во время
    /// прыжка/стыковки.
    pending_flight: Option<flight::ViewDir>,
    /// Запрос пришёл с `F1` на станции: открыть нативный вид сразу с
    /// анимацией вылета, а не ждать, пока Z80 закончит переход.
    pending_launch: bool,
    /// Состояние джойстика Kempston (5 бит).
    kempston: [bool; 5],
    ctrl: bool,
    /// Зажата ли Alt (для переключения полного экрана по Alt+Enter).
    alt: bool,
    /// F12: ускорение эмуляции Z80 вдвое — за кадр показа прогоняются два
    /// кадра Z80. На отрисовку и звук это не влияет, кроме темпа.
    accelerated: bool,
    /// Что показывать под миниатюрой в отладочной полосе. По умолчанию —
    /// сокращённая сводка; дамп всех байтов включается второй кнопкой.
    debug_view: DebugView,
    /// Позиция указателя в координатах окна — для кнопок отладочной полосы.
    cursor_x: f64,
    cursor_y: f64,
    /// Позиция мыши в координатах экрана игры (0..255, 0..191)
    mouse_x: i32,
    mouse_y: i32,
    mouse_left: bool,
    mouse_right: bool,
    mouse_wheel_up: bool,
    mouse_wheel_down: bool,
    next_frame: Instant,
}

impl<'a> App<'a> {
    pub(crate) fn new(machine: Spectrum, opts: &'a Options) -> Self {
        let mut app = Self {
            state: GameState::default(),
            machine,
            opts,
            audio: audio::start_output(opts.mute),
            audio_buf: Vec::new(),
            wav_samples: opts.wav.as_ref().map(|_| Vec::new()),
            last_path: String::new(),
            menu: false,
            hdmap: hdmap::Map::new(),
            hdmap_screen: None,
            shop: equip::Shop::new(),
            status: status::Status::new(),
            pending_status: false,
            sysinfo: sysinfo::SysInfo::new(),
            pending_sysinfo: false,
            docked: docked::Docked::new(),
            market: market::Market::new(),
            flight: flight::Flight::new(),
            splash: splash::Splash::new(),
            cockpit: cockpit::Cockpit::new(),
            splash_values: cockpit::CockpitValues::for_splash(),
            menu_sel: 0,
            quit: false,
            buf: vec![0u32; SRC_W * SRC_H],
            window: None,
            context: None,
            surface: None,
            matrix: [[false; 5]; 8],
            pending_key: None,
            pending_shop: false,
            pending_flight: None,
            pending_launch: false,
            kempston: [false; 5],
            ctrl: false,
            alt: false,
            accelerated: false,
            debug_view: DebugView::default(),
            cursor_x: 0.0,
            cursor_y: 0.0,
            mouse_x: -1,
            mouse_y: -1,
            mouse_left: false,
            mouse_right: false,
            mouse_wheel_up: false,
            mouse_wheel_down: false,
            next_frame: Instant::now(),
        };
        app.sync_state();
        app
    }

    /// Размер игровой области окна. В режиме `--debug` справа остаётся
    /// отладочная полоса шириной [`display::DEBUG_W`] пикселей.
    pub(super) fn game_size(&self, win_w: u32, win_h: u32) -> (u32, u32) {
        display::game_area(win_w, win_h, self.opts.debug)
    }

    /// Раскладка ZX-кадра: в отладочном режиме — в левой части окна,
    /// чтобы не заходить под отладочную полосу.
    pub(super) fn zx_layout(&self, win_w: u32, win_h: u32) -> display::Layout {
        display::layout_for(win_w, win_h, self.opts.debug)
    }

    /// Записать накопленный звук в WAV (если запрошено `--wav`).
    pub(crate) fn finish(&mut self) {
        if let (Some(path), Some(samples)) = (&self.opts.wav, &self.wav_samples) {
            audio::write_wav(path, samples, audio::SAMPLE_RATE);
        }
    }

    /// Текущее централизованное состояние игры.
    #[allow(dead_code)]
    pub(super) fn game_state(&self) -> &GameState {
        &self.state
    }

    /// Обновить централизованное состояние из эмулятора и активных нативных экранов.
    ///
    /// Вызов должен происходить после всех шагов кадра, меняющих состояние
    /// (эмуляция, открытие/закрытие экранов), чтобы отрисовка опиралась на
    /// актуальный снимок.
    pub(super) fn sync_state(&mut self) {
        let intro = self.machine.intro_seen;
        let emu_screen = self
            .machine
            .mem
            .get(flight::Z80_WHAT_ON_SCREEN)
            .copied()
            .unwrap_or(0);
        let location = if flight::z80_is_docked(&self.machine.mem) {
            Location::Station
        } else {
            Location::Space
        };

        // Показывается ли нативная альтернатива экранов эмулятора (те же
        // режимы Z80, но отрисованные нативно). Это зеркалирует условия,
        // по которым кадр выбирает, что рисовать.
        let show_status = self.status.active
            && !intro
            && emu_screen == flight::Z80_SCREEN_PILOT_INFO;
        let show_sysinfo = self.sysinfo.active
            && !intro
            && emu_screen == flight::Z80_SCREEN_INFO_PLANET;
        let show_market = self.market.native
            && !intro
            && emu_screen == flight::Z80_SCREEN_MARKET;

        let screen = if intro {
            Screen::Intro
        } else if self.hdmap.active {
            Screen::GalacticMap
        } else if self.shop.active {
            Screen::EquipShop
        } else if show_status {
            Screen::PilotStatus
        } else if show_sysinfo {
            Screen::PlanetInfo
        } else if show_market {
            Screen::Market
        } else if self.flight.active {
            match self.flight.transition_kind() {
                Some(flight::TransitionKind::Launch) => Screen::Animation(Animation::Launch),
                Some(flight::TransitionKind::Hyperspace) => {
                    Screen::Animation(Animation::Hyperspace)
                }
                Some(flight::TransitionKind::Docking) => Screen::Animation(Animation::Docking),
                None => Screen::Flight,
            }
        } else {
            Screen::Emulator(OriginalScreen::from_what_on_screen(emu_screen))
        };

        self.state = GameState {
            location,
            screen,
            menu: self.menu,
            intro,
            emu_screen,
        };
    }
}

