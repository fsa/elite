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
use crate::equip;
use crate::flight;
use crate::hdmap;
use crate::machine::Spectrum;
use crate::splash;
use crate::ui::present::{SRC_H, SRC_W};

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
    /// Открыто ли меню паузы (Esc) — игра не выполняется.
    menu: bool,
    /// Отдельная нативная карта галактики (F10) — заменяет экран игры.
    hdmap: hdmap::Map,
    /// Нативный экран закупки оборудования (F4) — заменяет экран игры.
    shop: equip::Shop,
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
        Self {
            machine,
            opts,
            audio: audio::start_output(opts.mute),
            audio_buf: Vec::new(),
            wav_samples: opts.wav.as_ref().map(|_| Vec::new()),
            last_path: String::new(),
            menu: false,
            hdmap: hdmap::Map::new(),
            shop: equip::Shop::new(),
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
            mouse_x: -1,
            mouse_y: -1,
            mouse_left: false,
            mouse_right: false,
            mouse_wheel_up: false,
            mouse_wheel_down: false,
            next_frame: Instant::now(),
        }
    }

    /// Записать накопленный звук в WAV (если запрошено `--wav`).
    pub(crate) fn finish(&mut self) {
        if let (Some(path), Some(samples)) = (&self.opts.wav, &self.wav_samples) {
            audio::write_wav(path, samples, audio::SAMPLE_RATE);
        }
    }
}
