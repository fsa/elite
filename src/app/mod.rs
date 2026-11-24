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
use crate::sfx;
use crate::splash;
use crate::state::{Animation, GameState, Location, OriginalScreen, Screen};
use crate::storage;
use crate::status;
use crate::sysinfo;
use crate::ui::present::{DebugView, SRC_H, SRC_W};
use crate::world;

mod frame;
mod input;
mod pause;
mod screens;
#[cfg(test)]
mod tests;
mod window;

/// Удержание клавиши переключения множителя эмуляции.
///
/// Короткое нажатие переключает множитель по кругу, а долгое — сразу сбрасывает
/// его на x1. `reset_done` не даёт последующему отпусканию клавиши снова
/// сдвинуть множитель.
#[derive(Clone, Copy)]
struct SpeedKeyHold {
    started: Instant,
    reset_done: bool,
}

/// Причина отложенного входа в нативный режим станции. Перед заморозкой Z80
/// должен получить `L`, чтобы оригинал вышел из режима покупки.
#[derive(Clone, Copy, PartialEq, Eq)]
enum StationNativePending {
    /// Обычный вход после старта/стыковки: закрыть экраны и открыть капитана.
    Docked,
    /// Игрок открыл нативный экран из эмулятора: открытый экран сохраняем.
    Screen,
}

/// Частота кадров ZX Spectrum: 50 кадров в секунду (кадр каждые 20 мс).
const Z80_FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 50);
/// Целевая частота отображения. Держим 50 Гц, чтобы 1 кадр рендера = 1 кадр
/// Z80/звука и форма бипера сохранялась по тактам CPU.
const DISPLAY_FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 50);
/// Удержание `F12` дольше этого времени сбрасывает множитель эмуляции на x1.
const SPEED_HOLD_RESET: Duration = Duration::from_secs(1);

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
    sfx: sfx::SfxPlayer,
    /// Полноэкранный экран сохранения/загрузки. Пока он открыт, `menu` тоже
    /// взведён: Z80 заморожен, а кадр рисует именно этот экран.
    save_screen: Option<storage::SaveScreen>,
    /// Символ из текстового события клавиатуры (ввод имени/комментария).
    /// Физические клавиши A-Z обрабатываются и без него, но пунктуация и
    /// регистр приходят именно отсюда.
    input_char: Option<char>,
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
    /// Текущая скорость перемещения курсора карты (единиц поля за кадр).
    hdmap_move_speed: f32,
    /// Дробный остаток смещения курсора карты по горизонтали.
    hdmap_move_rem_x: f32,
    /// Дробный остаток смещения курсора карты по вертикали.
    hdmap_move_rem_y: f32,
    /// Нативный экран закупки оборудования (F5) — заменяет экран игры.
    shop: equip::Shop,
    /// Вид радара в нативном кокпите: зеркало режима, включённого в эмуляторе.
    /// Клик по радару его не меняет — переключает только `L` в паузе Кладова.
    radar_view: cockpit::RadarView,
    /// Режим круглого компаса: обычный указатель или дополнительный режим.
    /// Переключается кликом по правому верхнему кружку панели и `C` в паузе.
    compass_view: cockpit::CompassView,
    /// Начало отсчёта фазы наложения солнца и планеты на компасе. Идёт по
    /// настенным часам, а не по кадрам Z80: в паузе эмулятор стоит, а фаза
    /// должна продолжать мигать, и её частота не должна зависеть от FPS.
    visual_phase_start: Instant,
    /// Нативный экран статуса пилота (F9) — альтернатива экрану `L`.
    status: status::Status,
    /// После кадра эмулятора открыть нативный экран статуса: сначала игра
    /// должна обработать `L` и перейти в режим показа капитана.
    pending_status: bool,
    /// `Space` на заставке уже поставил одноразовые хуки старта. Повторные
    /// нажатия до выхода эмулятора в игру не должны ставить их заново.
    start_requested: bool,
    /// Нативный экран информации о планете (F7) — альтернатива экрану `P`.
    sysinfo: sysinfo::SysInfo,
    /// После кадра эмулятора открыть нативный экран информации о планете:
    /// сначала игра должна обработать `P` и заполнить поля планеты.
    pending_sysinfo: bool,
    /// Экран информации о планете открыт по ПКМ с карты: повторный ПКМ
    /// возвращает на карту в прежнем состоянии.
    sysinfo_from_map: bool,
    /// Нативный режим станции: эмулятор заморожен, ввод и данные полностью
    /// принадлежат нативным экранам. Этот режим включается сразу после
    /// старта игры и после возврата на станцию, пока игрок не нажмёт
    /// `Backquote` — единственную клавишу передачи управления эмулятору.
    station_native: bool,
    /// Признак того, что на станции управление передано эмулятору. Держится до
    /// вылета/новой стыковки или до открытия нативного экрана по `F`-клавише.
    station_emulator: bool,
    /// Вход в нативный режим станции отложен до обработки `L` эмулятором.
    pending_station_native: Option<StationNativePending>,
    /// Пауза Кладова: Z80 заморожен, сервисные клавиши меняют его настройки,
    /// пока `Space` не вернёт игру в обычный режим.
    pause_active: bool,
    /// Экран-заглушка `DOCKED`, который на станции заменяет ответ эмулятора
    /// на `G`/`H` и показывается примерно секунду.
    docked: docked::Docked,
    /// Полноэкранная заглушка `LOADING` на время перехода загрузки в игру.
    /// Без неё на старте с `--save` виден вращающийся корабль интро.
    loading: bool,
    /// Нативный (альтернативный) показ экрана цен на станции (`K`).
    market: market::Market,
    /// После кадра эмулятора открыть нативный экран цен: сначала игра
    /// должна обработать `K` и перейти в режим рынка.
    pending_market: bool,
    /// Экран полёта (F1) — нативный рендер по данным Z80. Пока он активен,
    /// окно занимает вид из кабины и нативная приборная панель; эмулятор
    /// продолжает работать и задаёт мир, поэтому ввод идёт в игру.
    flight: flight::Flight,
    /// Нативная заставка (титульный экран): пока игра на интро, окно занимает
    /// нативный рендер (`splash`). Кнопка ` может временно показать родное
    /// интро эмулятора.
    splash: splash::Splash,
    /// После выхода с заставки по ` показан экран эмулятора. Повторный `
    /// возвращает нативный титр, не прерывая интро Z80.
    splash_emulator: bool,
    /// ` нажата во время нативной анимации перехода: родной ролик должен
    /// остаться на экране до конца вместо повторного открытия F1.
    emulator_transition_lock: bool,
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
    /// После кадра эмулятора открыть нативный экран закупки. Нужно для `F5`:
    /// сначала оригинал должен перейти в режим `4` (equip), и только потом
    /// нативный экран встаёт на паузу.
    pending_shop: bool,
    /// Отсчёт гиперпрыжка был замечен в памяти Z80 в текущем переходе.
    /// Позволяет отличить прыжок от стыковки после того, как счётчик обнулился:
    /// в этот момент `JumpDocking` у обоих переходов одинаков.
    saw_jump_countdown: bool,
    /// Нативный вид, который нужно включить, как только Z80 окажется в
    /// полёте или начнёт переход. Нужен для `F1` на станции и во время
    /// прыжка/стыковки.
    pending_flight: Option<flight::ViewDir>,
    /// Запрос пришёл с `F1` на станции: открыть нативный вид сразу с
    /// анимацией вылета, а не ждать, пока Z80 закончит переход.
    pending_launch: bool,
    /// В полёте пользователь явно выбрал показ экрана эмулятора вместо
    /// нативного вида кабины. Автооткрытие нативного вида при этом
    /// подавляется до явного возврата по `F1`-`F4`.
    flight_emulator: bool,
    /// Состояние джойстика Kempston (5 бит).
    kempston: [bool; 5],
    ctrl: bool,
    /// Зажата ли Alt (для переключения полного экрана по Alt+Enter).
    alt: bool,
    /// Множитель эмуляции Z80: 1, 2, 5 или 10 кадров за кадр показа. На
    /// отрисовку это не влияет, а звук играет во столько же раз быстрее.
    speed_multiplier: u32,
    /// Текущее удержание `F12`, если клавиша нажата.
    speed_key_hold: Option<SpeedKeyHold>,
    /// Что показывать под миниатюрой в отладочной полосе. По умолчанию —
    /// сокращённая сводка; дамп всех байтов включается второй кнопкой.
    debug_view: DebugView,
    /// Открыт ли полноэкранный редактор сейва.
    save_editor_open: bool,
    /// Выбранное поле в редакторе сейва (0..6): 0=NAME,1=CASH,2=FUEL,3=MISSILES,4=LEGAL,5=BAY,6=FRONT.
    save_editor_sel: usize,
    /// Режим ввода числа в редакторе.
    save_editor_editing: bool,
    /// Буфер ввода числа.
    save_editor_input: String,
    /// Режим ввода имени.
    save_editor_name_editing: bool,
    /// Буфер ввода имени.
    save_editor_name_input: String,
    /// Курсор в HEX-редакторе (индекс байта).
    save_editor_hex_cursor: usize,
    /// Режим ввода HEX-байта (2 символа).
    save_editor_hex_editing: bool,
    /// Буфер ввода HEX.
    save_editor_hex_input: String,
    /// Позиция указателя в координатах окна — для кнопок отладочной полосы.
    cursor_x: f64,
    cursor_y: f64,
    /// Видимость системного курсора. В полноэкранном режиме он прячется через
    /// несколько секунд без движения мыши и возвращается при первом движении.
    cursor_visible: bool,
    /// Момент последнего события мыши — для тайм-аута курсора.
    last_mouse_activity: Instant,
    /// Позиция мыши в координатах экрана игры (0..255, 0..191)
    mouse_x: i32,
    mouse_y: i32,
    mouse_left: bool,
    mouse_right: bool,
    mouse_wheel_up: bool,
    mouse_wheel_down: bool,
    next_frame: Instant,
    /// Накопитель реального времени для фиксированных шагов Z80 (50 Гц).
    z80_accum: Duration,
    last_tick: Instant,
}

impl<'a> App<'a> {
    pub(crate) fn new(machine: Spectrum, opts: &'a Options) -> Self {
        let radar_view = flight::z80_radar_view(&machine.mem);
        // Хук стартовой загрузки уже поставлен: титр не нужен, сразу показываем
        // кадр эмулятора. Значение берём до перемещения `machine` в структуру.
        let splash_emulator = machine.load_requested;
        let loading = machine.load_requested;
        let mut app = Self {
            state: GameState::default(),
            machine,
            opts,
            audio: audio::start_output(opts.mute),
            audio_buf: Vec::new(),
            wav_samples: opts.wav.as_ref().map(|_| Vec::new()),
            sfx: sfx::SfxPlayer::new(),
            save_screen: None,
            input_char: None,
            menu: false,
            hdmap: hdmap::Map::new(),
            hdmap_screen: None,
            hdmap_move_speed: 0.0,
            hdmap_move_rem_x: 0.0,
            hdmap_move_rem_y: 0.0,
            shop: equip::Shop::new(),
            radar_view,
            compass_view: cockpit::CompassView::Pointer,
            visual_phase_start: Instant::now(),
            status: status::Status::new(),
            pending_status: false,
            start_requested: false,
            sysinfo: sysinfo::SysInfo::new(),
            pending_sysinfo: false,
            sysinfo_from_map: false,
            station_native: false,
            station_emulator: false,
            pending_station_native: None,
            pause_active: false,
            docked: docked::Docked::new(),
            loading,
            market: market::Market::new(),
            pending_market: false,
            flight: flight::Flight::new(),
            splash: splash::Splash::with_debug(opts.debug),
            splash_emulator,
            emulator_transition_lock: false,
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
            saw_jump_countdown: false,
            pending_flight: None,
            pending_launch: false,
            flight_emulator: false,
            kempston: [false; 5],
            ctrl: false,
            alt: false,
            speed_multiplier: 1,
            speed_key_hold: None,
            debug_view: DebugView::default(),
            save_editor_open: false,
            save_editor_sel: 0,
            save_editor_editing: false,
            save_editor_input: String::new(),
            save_editor_name_editing: false,
            save_editor_name_input: String::new(),
            save_editor_hex_cursor: 0,
            save_editor_hex_editing: false,
            save_editor_hex_input: String::new(),
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_visible: true,
            last_mouse_activity: Instant::now(),
            mouse_x: -1,
            mouse_y: -1,
            mouse_left: false,
            mouse_right: false,
            mouse_wheel_up: false,
            mouse_wheel_down: false,
            next_frame: Instant::now(),
            z80_accum: Duration::ZERO,
            last_tick: Instant::now(),
        };
        // Снапшот может сразу быть пристыкованным. Это не переход из
        // эмулятора, а начальное состояние, поэтому `L` не посылаем.
        app.enter_native_station();
        app.sync_state();
        app
    }

    /// Открыть приглашение сохранения: имя берём из текущего блока commander,
    /// а комментарий подставляем из имени текущей планеты.
    pub(super) fn open_save_prompt(&mut self) {
        let name = {
            let layout = world::save_layout_kladov();
            storage::name_from_block(crate::native::commander_block(&self.machine, layout))
        };
        self.save_screen = Some(storage::SaveScreen::Save(storage::SavePrompt {
            name,
            comment: storage::sanitize_comment(&self.state.system),
            field: storage::PromptField::Name,
            status: String::new(),
        }));
        self.menu = true;
        self.menu_sel = 0;
        self.input_char = None;
    }

    /// Открыть таблицу сохранений, свежие записи сверху.
    pub(super) fn open_load_list(&mut self) {
        let (entries, status) = match storage::read_entries() {
            Ok(mut entries) => {
                storage::sorted_for_table(&mut entries);
                let status = if entries.is_empty() {
                    "NO SAVES YET".to_string()
                } else {
                    String::new()
                };
                (entries, status)
            }
            Err(_) => (Vec::new(), "ERROR: CANNOT READ CATALOG".to_string()),
        };
        self.save_screen = Some(storage::SaveScreen::Load(storage::LoadList {
            entries,
            sel: 0,
            status,
        }));
        self.menu = true;
        self.menu_sel = 0;
        self.input_char = None;
    }

    /// Закрыть экран сохранения/загрузки и снять модальную заморозку Z80.
    pub(super) fn close_save_screen(&mut self) {
        self.save_screen = None;
        self.menu = false;
        self.input_char = None;
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
        let show_status = self.status.active && !intro;
        let show_sysinfo = self.sysinfo.active && !intro;
        let show_market = self.market.native && !intro;

        let screen = if intro && !self.splash_emulator {
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
            menu: self.menu || self.save_screen.is_some(),
            intro,
            emu_screen,
            system: world::current_system_name(&self.machine.mem),
            planet: world::selected_planet_name(&self.machine.mem),
        };
        // Нативный радар — только зеркало включённого в эмуляторе режима.
        self.radar_view = flight::z80_radar_view(&self.machine.mem);
    }
}

