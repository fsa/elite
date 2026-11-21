//! Обработка клавиатуры и мыши: нативные экраны, меню и передача ввода в Z80.

use winit::keyboard::KeyCode;
use winit::window::Fullscreen;

use super::App;
use crate::flight;
use crate::hdmap;
use crate::keys::{
    digit_of, emulator_handoff_key, function_game_key, function_view, is_game_key, kempston_bit,
    matrix_pos, SHIP_CONTROL_KEYS,
};
use crate::native;
use crate::state::OriginalScreen;
use crate::storage;
use crate::world;

impl App<'_> {
    /// Нажатие `F1`-`F9`: отдать игре соответствующую оригинальную клавишу
    /// или переключить нативный экран.
    ///
    /// На станции `F1` — вылет, `F5` — оборудование, `F6` — карта, `F7` —
    /// планеты, `F8` — цены, `F9` — статус. `F2`-`F4` пустые: их цифры увели бы
    /// эмулятор в несовместимые с нативным режимом экраны оригинала.
    /// `F5`-`F9` возвращают управление из эмулятора на нативный экран.
    fn function_key(&mut self, code: KeyCode) {
        if self.menu {
            return;
        }
        // F5 — нативная закупка оборудования на станции. В полёте и в других
        // состояниях отдельного действия нет.
        if code == KeyCode::F5 {
            self.close_hdmap();
            let docked = flight::z80_is_docked(&self.machine.mem);
            self.shop.close();
            self.close_status();
            self.close_sysinfo();
            self.close_market();
            self.pending_status = false;
            self.pending_sysinfo = false;
            self.pending_shop = false;
            self.pending_flight = None;
            self.pending_launch = false;
            self.pending_key = None;
            if docked && !self.machine.intro_seen {
                // На станции закупка полностью нативная: эмулятору не нужно
                // передавать `L` и ждать кадр. Открытие само заберёт управление.
                self.open_shop();
            }
            return;
        }

        let Some(key) = function_game_key(code) else {
            return;
        };
        // F2-F4 на станции не реализованы: оригинал по ним уходит в режимы
        // покупки/продажи и закупки и потом не понимает клавиши нативного
        // экрана. Поэтому цифры 2-4 в игру не отдаём и экран не переключаем.
        if flight::z80_is_docked(&self.machine.mem)
            && matches!(code, KeyCode::F2 | KeyCode::F3 | KeyCode::F4)
        {
            return;
        }
        let Some(dir) = function_view(code) else {
            return;
        };
        // Сначала вернуть экран игры: под картой в памяти стоит `SHORT RANGE
        // CHART`, и состояние полёта надо читать уже после восстановления.
        self.close_hdmap();
        let mem = &self.machine.mem;
        let docked = flight::z80_is_docked(mem);
        let in_cockpit = flight::z80_in_cockpit(mem);
        let in_transition = flight::z80_in_transition(mem);

        self.shop.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.menu = false;
        self.pending_shop = false;
        if docked {
            // На станции из F1-F4 доступен только `F1` (вылет): включает
            // нативный вид сразу с анимацией вылета. Закупка вынесена на `F5`.
            if code == KeyCode::F1 {
                // Вылет возвращает управление эмулятору: он должен получить
                // клавишу `1`, иначе Z80 останется замороженным на станции.
                self.cancel_pending_station();
                self.station_native = false;
                self.station_emulator = true;
            }
            if code == KeyCode::F1 && in_cockpit {
                // Снимок/переход, где станция уже отдала полётный экран.
                self.show_flight(dir);
            } else if code == KeyCode::F1 {
                self.pending_flight = Some(dir);
                self.pending_launch = true;
            } else {
                self.pending_flight = None;
                self.pending_launch = false;
            }
        } else if in_cockpit {
            self.show_flight(dir);
        } else if in_transition {
            // В полёте, но идёт прыжок/стыковка. Если рендер уже открыт, он
            // сам показывает переход; иначе включим его с анимацией.
            if self.flight.active {
                self.show_flight(dir);
            } else {
                self.pending_flight = Some(dir);
                self.pending_launch = false;
            }
        } else {
            // Оригинал на своём экране (карта, инвентарь, статус). Клавиши
            // `1`-`4` возвращают его в кабину, поэтому запоминаем вид и
            // включаем рендер, как только Z80 снова окажется в полёте. Иначе
            // первый `F1` только закрывал бы Z80-экран, а нативный вид
            // требовал бы второго нажатия.
            self.pending_flight = Some(dir);
            self.pending_launch = false;
        }
        self.pending_key = matrix_pos(key);
    }

    /// Оригинальные станционные клавиши из `docs/z80/coriolis_keys.md`,
    /// которые теперь открывают нативные экраны. Возвращает `true`, если
    /// клавиша обработана и её нельзя передавать дальше.
    ///
    /// `G`/`H` передаются эмулятору, `Backquote` — единственный выход в
    /// эмулятор без игровой клавиши. Цифры `1`–`3` в модальном окне крепления
    /// лазера остаются управлением самого экрана закупки.
    fn station_native_command(&mut self, code: KeyCode) -> bool {
        if self.shop.mount_open()
            && matches!(code, KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3)
        {
            return false;
        }
        match code {
            KeyCode::Digit1 => {
                // Клавиша `1` оригинала — вылет. Очищаем матрицу до вызова:
                // `function_key` сам поставит `pending_key` на один кадр.
                self.release_pressed(code);
                self.function_key(KeyCode::F1);
                return true;
            }
            KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::KeyK => {
                if !self.market.native {
                    self.open_market();
                }
            }
            KeyCode::Digit4 => {
                if !self.shop.active {
                    self.open_shop();
                }
            }
            KeyCode::KeyI => self.open_hdmap_mode(hdmap::MapMode::Galaxy),
            KeyCode::KeyO => self.open_hdmap_mode(hdmap::MapMode::Local),
            KeyCode::KeyP => {
                if !self.sysinfo.active {
                    self.open_sysinfo();
                }
            }
            KeyCode::KeyL => {
                if !self.status.active {
                    self.open_status();
                }
            }
            KeyCode::ControlLeft => {
                // В оригинале это CAPS SHIFT — пауза с дополнительными
                // функциями. Нативно показываем уже существующее меню Esc.
                self.menu = true;
                self.menu_sel = 0;
            }
            KeyCode::ShiftRight => {
                // Родная клавиша сохранения: сейв полностью нативный, З80
                // заморожен и в этом процессе не участвует.
                storage::do_save(&mut self.machine, self.opts, &mut self.last_path);
            }
            _ => return false,
        }
        self.release_pressed(code);
        true
    }

    /// Информационные клавиши в полёте: открывают нативные экраны, как на
    /// станции, но без остановки симуляции и управления кораблём.
    fn flight_info_command(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::KeyI => self.open_hdmap_mode(hdmap::MapMode::Galaxy),
            KeyCode::KeyO => self.open_hdmap_mode(hdmap::MapMode::Local),
            KeyCode::KeyP => {
                if self.sysinfo.active {
                    self.close_sysinfo();
                } else {
                    self.open_planet_info();
                }
            }
            KeyCode::KeyK => {
                if self.market.native {
                    self.close_market();
                } else {
                    self.open_market();
                }
            }
            KeyCode::KeyL => {
                if self.status.active {
                    self.close_status();
                } else {
                    self.open_status();
                }
            }
            _ => return false,
        }
        self.release_pressed(code);
        true
    }

    /// Нативный экран по повторному нажатию оригинальной клавиши в режиме
    /// эмулятора.
    ///
    /// Первое нажатие уходит в игру: оригинал сам переходит на свой экран.
    /// Повторное — когда игра уже показывает этот экран — забираем себе и
    /// открываем нативный экран. Ещё одно нажатие снимает нативную надстройку
    /// и снова отдаёт клавишу игре, чтобы она сама ушла со своего экрана.
    /// Fn-клавиши открывают натив сразу и обрабатываются отдельно.
    ///
    /// Возвращает `true`, если клавиша поглощена нативным экраном.
    fn emulator_screen_key(&mut self, code: KeyCode) -> bool {
        if !self.station_emulator && !self.flight_emulator {
            return false;
        }
        let screen = self.state.screen.original();
        // Если нативная надстройка уже открыта, клавиша возвращается игре —
        // иначе она застряла бы на своём экране без возможности выйти.
        match code {
            KeyCode::KeyL if self.status.active => {
                self.close_status();
                return false;
            }
            KeyCode::KeyP if self.sysinfo.active => {
                self.close_sysinfo();
                return false;
            }
            KeyCode::KeyK if self.market.native => {
                self.close_market();
                return false;
            }
            KeyCode::KeyL if screen == Some(OriginalScreen::CommanderStatus) => self.open_status(),
            KeyCode::KeyP if screen == Some(OriginalScreen::PlanetInfo) => self.open_planet_info(),
            KeyCode::KeyK if screen == Some(OriginalScreen::MarketPrices) => self.open_market(),
            KeyCode::KeyI | KeyCode::KeyO
                if self.hdmap.active
                    || matches!(
                        screen,
                        Some(OriginalScreen::GalacticChart | OriginalScreen::LocalChart)
                    ) =>
            {
                let mode = if code == KeyCode::KeyI {
                    hdmap::MapMode::Galaxy
                } else {
                    hdmap::MapMode::Local
                };
                self.open_hdmap_mode(mode);
            }
            _ => return false,
        }
        self.release_pressed(code);
        true
    }

    /// Обработать событие клавиши (по физической позиции, раскладка не важна).
    pub(super) fn key_event(&mut self, code: KeyCode, pressed: bool, repeat: bool) {
        if let Some((row, bit)) = matrix_pos(code) {
            self.matrix[row][bit as usize] = pressed;
        }
        if let Some(bit) = kempston_bit(code) {
            self.kempston[bit as usize] = pressed;
        }
        // Состояние станции могло измениться между кадрами (стыковка, вылет,
        // загрузка). Синхронизируем режим до обработки нажатия.
        self.maybe_enter_native_station();
        if !pressed {
            return;
        }
        // F12 — ускорение эмуляции Z80 вдвое. Работает в любом режиме и в игру
        // не передаётся (в матрице ZX такой клавиши нет).
        if code == KeyCode::F12 && !repeat {
            self.accelerated = !self.accelerated;
            return;
        }
        // F11 / Alt+Enter — переключение полноэкранного режима во всех режимах.
        if (code == KeyCode::F11 || (code == KeyCode::Enter && self.alt)) && !repeat {
            self.toggle_fullscreen();
            return;
        }
        // F10 — пустая клавиша: ни карту не открывает, ни в эмулятор
        // ничего не уходит (в матрице ZX её нет, но обработку всё равно
        // завершаем явно, чтобы она не задела другие режимы).
        if code == KeyCode::F10 {
            return;
        }
        // Временная страница клавиш паузы закрывается следующим нажатием;
        // само нажатие при этом продолжает обрабатываться обычным порядком.
        if self.pause_stub && !repeat {
            self.pause_stub = false;
        }

        // В режиме анимации (вылет, гиперпереход, стыковка) переключение
        // экранов недоступно: работает только переход между нативной
        // отрисовкой (F1) и экраном эмулятора (Backquote), а также пауза.
        // Эмулятор в это время проигрывает анимацию и игра не реагирует на кнопки.
        if !self.machine.intro_seen && !self.menu && flight::z80_in_transition(&self.machine.mem) {
            if repeat {
                return;
            }
            if code == KeyCode::F1 {
                // Показать нативную отрисовку текущей анимации.
                if self.flight.in_transition() {
                    // Анимация уже идёт (в том числе скрытая Backquote) —
                    // просто снова показываем нативный вид без сброса состояния.
                    self.flight.active = true;
                    self.flight.set_view_dir(flight::ViewDir::Forward);
                    self.pending_flight = None;
                    self.pending_launch = false;
                } else {
                    self.show_flight(flight::ViewDir::Forward);
                    // После отсчёта `JumpDocking` у прыжка и стыковки
                    // одинаков, поэтому вид берём из замеченного отсчёта.
                    let countdown = self
                        .machine
                        .mem
                        .get(flight::Z80_JUMP_COUNTDOWN)
                        .copied()
                        .unwrap_or(0);
                    let intergalactic = self.machine.mem[flight::Z80_JUMP_DOCKING] == 2;
                    let kind = if self.saw_jump_countdown || countdown > 0 || intergalactic {
                        flight::TransitionKind::Hyperspace
                    } else {
                        flight::TransitionKind::Docking
                    };
                    self.flight.start_transition_kind(kind, countdown);
                }
            } else if emulator_handoff_key(code) {
                // Показать анимацию, которую рисует эмулятор. Состояние
                // перехода сохраняем, чтобы F1 мог вернуться к той же анимации.
                self.flight.active = false;
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }

        // Пока заглушка `DOCKED` на экране, клавиши не должны
        // управлять скрытым нативным экраном и не передаются эмулятору,
        // кроме меню паузы.
        if self.docked.is_active() {
            if self.menu {
                self.menu_key(code);
            } else if code == KeyCode::Escape && !repeat {
                self.menu = true;
                self.menu_sel = 0;
            } else if let Some((row, bit)) = matrix_pos(code) {
                self.matrix[row][bit as usize] = false;
            }
            return;
        }
        // G/H на станции: показываем нативный DOCKED и проигрываем звук (без перехода в эмулятор).
        if !repeat
            && matches!(code, KeyCode::KeyG | KeyCode::KeyH)
            && !self.menu
            && self.station_native
            && !(self.hdmap.active && self.hdmap.is_searching())
        {
            self.docked.show();
            self.sfx.play(crate::sfx::Sfx::DockedBeep);
            if let Some((row, bit)) = matrix_pos(code) {
                self.matrix[row][bit as usize] = false;
            }
            self.pending_key = None;
            return;
        }
        // G/H в полёте под модальным нативным экраном (картой): отсчёт
        // гиперпрыжка ведёт Z80, поэтому клавиша должна дойти до него, но
        // закрывать экран не нужно — к тоннелю приложение само откроет F1.
        if matches!(code, KeyCode::KeyG | KeyCode::KeyH)
            && !repeat
            && !self.machine.intro_seen
            && !self.menu
            && !flight::z80_is_docked(&self.machine.mem)
            && !flight::z80_in_transition(&self.machine.mem)
            && self.hdmap.active
            && !self.hdmap.is_searching()
        {
            self.pending_key = matrix_pos(code);
            if let Some((row, bit)) = matrix_pos(code) {
                self.matrix[row][bit as usize] = false;
            }
            return;
        }
        // Единственный выход в эмулятор на станции: `Backquote` снимает
        // заморозку Z80, но не посылает в игру ни одной клавиши. Все остальные
        // клавиши из `coriolis_keys.md` открывают нативные экраны.
        if self.station_native
            && !self.menu
            && !repeat
            && emulator_handoff_key(code)
            && !(self.hdmap.active && self.hdmap.is_searching())
        {
            self.handoff_to_emulator();
            return;
        }
        // В полёте `Backquote` включает показ экрана эмулятора вместо
        // нативного вида кабины (симметрично выходу в эмулятор на станции).
        if !self.station_native
            && !self.menu
            && !repeat
            && emulator_handoff_key(code)
            && !self.machine.intro_seen
            && !flight::z80_is_docked(&self.machine.mem)
            && !(self.hdmap.active && self.hdmap.is_searching())
        {
            self.flight_emulator = true;
            self.pending_flight = None;
            self.pending_launch = false;
            if self.flight.active && !self.flight.in_animation_mode() {
                self.flight.close();
            } else if self.flight.active {
                // В анимации скрываем нативный вид, сохраняя состояние перехода.
                self.flight.active = false;
            }
            self.release_pressed(code);
            return;
        }
        // В режиме эмулятора буквенная клавиша сначала уходит в игру: нативный
        // экран открывает только повторное нажатие. Fn-клавиши работают сразу.
        if !self.menu
            && !repeat
            && !(self.hdmap.active && self.hdmap.is_searching())
            && self.emulator_screen_key(code)
        {
            return;
        }
        if self.station_native
            && !self.menu
            && !repeat
            && !(self.hdmap.active && self.hdmap.is_searching())
            && self.station_native_command(code)
        {
            return;
        }
        // В полёте информационные экраны открываются нативно, как на станции,
        // но эмулятор продолжает работать, чтобы сохранялось управление кораблём.
        // Пока экран эмулятора активен, буквенные клавиши принадлежат игре.
        if !self.station_native
            && !self.flight_emulator
            && !self.menu
            && !repeat
            && !self.machine.intro_seen
            && !flight::z80_is_docked(&self.machine.mem)
            && !(self.hdmap.active && self.hdmap.is_searching())
            && self.flight_info_command(code)
        {
            return;
        }
        // F6 открывает нативную карту в галактическом масштабе (`I` открывает
        // то же самое, `O` — детальный фрагмент). Повторное нажатие не закрывает
        // карту и не сбрасывает курсор: это отдельный режим.
        if code == KeyCode::F6 && !repeat {
            if !self.menu && !self.hdmap.active && !self.machine.intro_seen {
                self.close_market();
                self.open_hdmap();
            }
            return;
        }
        // Экран закупки (F5): выход обратно по F5 и Esc запрещён. Закрыть
        // его можно переключением на другой экран по Fn или игровой
        // (буквенной) клавише.
        if code == KeyCode::F5 && !repeat && self.shop.active {
            return;
        }
        // F7 — нативная альтернатива экрану информации о планете (`P`).
        if code == KeyCode::F7 && !repeat && !(self.hdmap.active && self.hdmap.is_searching()) {
            if !self.menu && !self.machine.intro_seen {
                let docked = flight::z80_is_docked(&self.machine.mem);
                if self.sysinfo.active && !docked {
                    self.close_sysinfo();
                } else {
                    self.open_planet_info();
                }
            }
            return;
        }
        // F9 — нативная альтернатива экрану капитана (`L`).
        if code == KeyCode::F9 && !repeat && !(self.hdmap.active && self.hdmap.is_searching()) {
            if !self.menu && !self.machine.intro_seen {
                let docked = flight::z80_is_docked(&self.machine.mem);
                if self.status.active && !docked {
                    self.close_status();
                } else {
                    let on_status = self.machine.mem[flight::Z80_WHAT_ON_SCREEN]
                        == flight::Z80_SCREEN_PILOT_INFO;
                    self.close_hdmap();
                    self.shop.close();
                    self.close_sysinfo();
                    self.flight.close();
                    self.close_market();
                    self.pending_shop = false;
                    self.pending_flight = None;
                    self.pending_launch = false;
                    self.pending_status = false;
                    self.pending_sysinfo = false;
                    self.pending_key = None;
                    self.open_status();
                    if !docked && !on_status && self.station_emulator {
                        self.pending_status = true;
                        self.pending_key = matrix_pos(KeyCode::KeyL);
                    }
                }
            }
            return;
        }
        // F8 — нативный показ экрана цен (альтернатива `K`).
        if code == KeyCode::F8 && !repeat && !(self.hdmap.active && self.hdmap.is_searching()) {
            if !self.menu && !self.machine.intro_seen {
                let docked = flight::z80_is_docked(&self.machine.mem);
                if self.market.native && !docked {
                    self.close_market();
                } else {
                    let on_market = self.machine.mem[flight::Z80_WHAT_ON_SCREEN]
                        == flight::Z80_SCREEN_MARKET;
                    self.close_hdmap();
                    self.shop.close();
                    self.close_status();
                    self.close_sysinfo();
                    self.flight.close();
                    self.close_market();
                    self.pending_status = false;
                    self.pending_sysinfo = false;
                    self.pending_market = false;
                    self.pending_flight = None;
                    self.pending_launch = false;
                    self.pending_key = None;
                    self.open_market();
                    if !docked && !on_market && self.station_emulator {
                        self.pending_market = true;
                        self.pending_key = matrix_pos(KeyCode::KeyK);
                    }
                }
            }
            return;
        }
        // `K` в режиме эмулятора показывает оригинальный экран рынка.
        // В нативных режимах (станция и полёт) он открывает нативный экран цен.
        // (Обрабатывается выше/ниже как информационная клавиша.)
        // Стрелки перехватываем только на станции: там они выбирают товар и
        // торгуют им. В полёте экран остаётся только для просмотра цен, поэтому
        // стрелки уходят эмулятору, как и все остальные клавиши.
        if self.market.native
            && !self.menu
            && !self.machine.intro_seen
            && flight::z80_is_docked(&self.machine.mem)
        {
            let trade = match code {
                KeyCode::ArrowUp => Some((-1, false)),
                KeyCode::ArrowDown => Some((1, false)),
                KeyCode::ArrowLeft => Some((0, false)),
                KeyCode::ArrowRight => Some((0, true)),
                _ => None,
            };
            if let Some((delta, buy)) = trade {
                let docked = self.state.is_docked()
                    || flight::z80_is_docked(&self.machine.mem);
                if delta != 0 {
                    self.market.move_selection(delta);
                } else if buy {
                    self.market.buy(&mut self.machine.mem, docked);
                } else {
                    self.market.sell(&mut self.machine.mem, docked);
                }
                // Стрелки не должны попадать в Kempston: для этого экрана они
                // означают выбор/сделку, а не движение корабля.
                if let Some(bit) = kempston_bit(code) {
                    self.kempston[bit as usize] = false;
                }
                return;
            }
        }
        // При открытой карте всё управление принадлежит ей; Z80 при этом
        // продолжает работать, но ввод в игру не передаётся.
        if self.hdmap.active {
            if self.menu {
                self.menu_key(code);
                return;
            }
            if self.hdmap.is_searching() {
                match code {
                    KeyCode::Escape if !repeat => {
                        self.hdmap.cancel_search();
                    }
                    KeyCode::Enter if !repeat => {
                        self.hdmap.confirm_search();
                        self.sync_hdmap_target();
                    }
                    KeyCode::Backspace => {
                        self.hdmap.search_backspace();
                        self.sync_hdmap_target();
                    }
                    _ => {
                        if !repeat {
                            if let Some(ch) = letter_of(code) {
                                self.hdmap.search_push(ch);
                                self.sync_hdmap_target();
                            }
                        }
                    }
                }
                return;
            }
            match code {
                KeyCode::ArrowLeft
                | KeyCode::ArrowRight
                | KeyCode::ArrowUp
                | KeyCode::ArrowDown
                | KeyCode::KeyN
                | KeyCode::KeyM
                | KeyCode::KeyS
                | KeyCode::KeyX => {
                    // Перемещение курсора обрабатывается плавно каждый кадр по
                    // удерживаемым клавишам направления.
                }
                KeyCode::Enter | KeyCode::Space => {
                    // На автоповторе не подтверждаем выбор повторно.
                    if !repeat {
                        self.hdmap.snap_to_nearest();
                        self.sync_hdmap_target();
                    }
                }
                // `D` — как в оригинале: выбрать ближайшую планету.
                KeyCode::KeyD => {
                    if !repeat {
                        self.hdmap.snap_to_nearest();
                        self.sync_hdmap_target();
                    }
                }
                // `B` — вернуть курсор на текущую планету.
                KeyCode::KeyB if !repeat => {
                    self.hdmap.cursor_to_current_system();
                    self.sync_hdmap_target();
                }
                // `R` — включить поиск планеты по имени.
                KeyCode::KeyR if !repeat => {
                    self.hdmap.start_search();
                }
                // Ctrl+F1 — временный чит: телепорт «на станцию» выбранной
                // планеты. Пишем координаты системы и уходим в родную ветку
                // переинициализации (см. `native::request_teleport`), затем
                // закрываем карту — игра перерисует экран станции.
                KeyCode::F1 if self.ctrl && !repeat => {
                    if let Some((x, y)) = self.hdmap.selected_coords() {
                        native::request_teleport(
                            &mut self.machine,
                            world::save_layout_kladov(),
                            x,
                            y,
                        );
                        self.close_hdmap();
                    }
                }
                // F1-F4 — те же клавиши 1-4 оригинала, F5 — нативная закупка:
                // доступные команды закрывают карту и уходят игре, нативный
                // вид/закупка включаются по фактическому состоянию эмулятора.
                // F2-F4 на станции остаются пустыми и карту не закрывают.
                KeyCode::F1 | KeyCode::F2 | KeyCode::F3 | KeyCode::F4 | KeyCode::F5 if !repeat => {
                    self.function_key(code);
                }
                KeyCode::Equal | KeyCode::NumpadAdd => {
                    self.hdmap.zoom_by(1);
                    self.sync_hdmap_target();
                }
                KeyCode::Minus | KeyCode::NumpadSubtract => {
                    self.hdmap.zoom_by(-1);
                    self.sync_hdmap_target();
                }
                // Esc не закрывает карту (это отдельный режим), но открывает меню паузы.
                KeyCode::Escape if !repeat => {
                    self.menu = true;
                    self.menu_sel = 0;
                }
                // Игровая клавиша: закрываем карту и отдаём нажатие игре,
                // чтобы из карты сразу перейти к экранам игры (командир,
                // станция и т.п.), не закрывая её вручную. В нативном режиме
                // станции карта сама обрабатывает своё управление, а всё
                // остальное игре не передаётся.
                _ => {
                    if is_game_key(code) {
                        if self.station_native {
                            self.release_pressed(code);
                        } else {
                            self.close_hdmap();
                            self.pending_key = matrix_pos(code);
                        }
                    }
                }
            }
            return;
        }
        // При открытом экране закупки всё управление принадлежит ему.
        // Выход с экрана по F5/Esc запрещён; закрыть можно переключением на
        // другой экран по Fn или игровой (буквенной) клавише.
        if self.shop.active {
            if self.menu {
                self.menu_key(code);
                return;
            }
            match code {
                KeyCode::F5 => {}
                KeyCode::Escape if !repeat => {
                    if self.shop.mount_open() {
                        self.shop.cancel_mount();
                    } else {
                        self.menu = true;
                        self.menu_sel = 0;
                    }
                }
                KeyCode::ArrowUp => self.shop.move_selection(-1),
                KeyCode::ArrowDown => self.shop.move_selection(1),
                // F1 — клавиша 1 оригинала: закупка закрывается и нажатие
                // уходит игре (смена экрана по Fn).
                KeyCode::F1 if !repeat => {
                    self.function_key(code);
                }
                KeyCode::Enter | KeyCode::Space => {
                    if !repeat {
                        self.shop.confirm(&mut self.machine.mem);
                    }
                }
                _ => {
                    if let Some(d) = digit_of(code) {
                        self.shop.input_digit(&mut self.machine.mem, d);
                    } else if is_game_key(code) {
                        // Игровая (буквенная) клавиша: закрываем закупку и
                        // отдаём нажатие игре (переход на экран эмулятора).
                        // В нативном режиме станции не-списочные клавиши
                        // закупку не закрывают и в эмулятор не уходят.
                        if self.station_native {
                            self.release_pressed(code);
                        } else {
                            self.shop.close();
                            self.pending_key = matrix_pos(code);
                        }
                    }
                }
            }
            return;
        }
        // Left Shift в оригинале Кладова включает паузу, в которой остальные
        // клавиши меняют настройки. В нативном режиме станции вместо будущего
        // меню настроек показывается заглушка, и открывает её только сама
        // паузная клавиша — остальные клавиши настроек пока ничего не делают.
        // В эмуляторе (после handoff) Shift работает по-родному. `Ctrl+Q`
        // остаётся глобальной командой выхода.
        if self.station_native && !self.menu && !repeat && code == KeyCode::ShiftLeft {
            self.pause_stub = true;
            self.release_pressed(code);
            return;
        }
        // Нативные экраны капитана (F9) и информации о планете (F7) не
        // перехватывают ввод: это альтернативная отрисовка экранов `L`/`P`,
        // управление остаётся за эмулятором.
        if repeat {
            return;
        }
        // Ctrl+Q — выход из игры.
        if code == KeyCode::KeyQ && self.ctrl {
            self.quit = true;
            return;
        }
        // Меню паузы: своё управление, в эмулятор клавиши не уходят.
        // В нативном режиме станции меню может быть открыто поверх текущего
        // экрана, поэтому обрабатываем его до станционных команд.
        if self.menu {
            self.menu_key(code);
            return;
        }
        // Экран полёта: F1-4 — те же клавиши 1-4 оригинала (передний/задний/
        // левый/правый вид) плюс переключение нативного вида; 1 возвращает
        // Z80-кадр, Esc открывает меню паузы.
        // В режиме анимации (вылет/прыжок/стыковка) переключение экранов
        // недоступно: работает только переход между F1 и 1 (нативная отрисовка
        // или эмулятор). Игра в это время сама не реагирует на кнопки.
        if self.flight.active {
            let in_animation = self.flight.transition_kind().is_some();
            if in_animation {
                if code == KeyCode::F1 {
                    // Оставляем нативный вид активным (уже открыт).
                    self.flight.set_view_dir(flight::ViewDir::Forward);
                } else if emulator_handoff_key(code) {
                    // Показать анимацию, которую рисует эмулятор. Состояние
                    // перехода сохраняем, чтобы F1 мог вернуться к ней.
                    self.flight.active = false;
                } else if code == KeyCode::Escape {
                    self.menu = true;
                    self.menu_sel = 0;
                }
                return;
            }
            let view = match code {
                KeyCode::F1 => Some(flight::ViewDir::Forward),
                KeyCode::F2 => Some(flight::ViewDir::Back),
                KeyCode::F3 => Some(flight::ViewDir::Left),
                KeyCode::F4 => Some(flight::ViewDir::Right),
                _ => None,
            };
            if let Some(view) = view {
                self.flight.set_view_dir(view);
                // На станции F2/F3 только переключают нативный вид: цифры
                // 2/3 ушли бы в покупку/продажу и сломали бы игру.
                self.pending_key = if flight::z80_is_docked(&self.machine.mem)
                    && matches!(code, KeyCode::F2 | KeyCode::F3)
                {
                    None
                } else {
                    function_game_key(code).and_then(matrix_pos)
                };
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }
        // На заставке своя нативная логика: запускает игру только SPACE,
        // стрелки влево/вправо переключают корабль (как в оригинальном
        // интро), остальные клавиши (кроме меню) игнорируются.
        if self.machine.intro_seen {
            if code == KeyCode::Space && !repeat {
                // SPACE запускает игру обычным родным путём. Ветка старта
                // (`loc_73FD`) сама вызывает `Cmd_L_PilotInfo`, поэтому
                // эмулятор выходит в режим капитана без всякой клавиши `L`.
                // Нативную страницу капитана поднимаем после кадра, когда
                // игра действительно перешла из заставки в игру. Сам `Space`
                // игре не отдаём: он не должен сработать как выстрел.
                if !self.start_requested {
                    native::start_game(&mut self.machine, world::save_layout_kladov());
                    self.start_requested = true;
                }
                if let Some((row, bit)) = matrix_pos(KeyCode::Space) {
                    self.matrix[row][bit as usize] = false;
                }
                self.pending_key = None;
                self.pending_status = true;
            } else if code == KeyCode::ArrowLeft {
                self.splash.cycle_ship(-1);
            } else if code == KeyCode::ArrowRight {
                self.splash.cycle_ship(1);
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }
        // F1-F4 повторяют клавиши 1-4 основной игры. Если Z80 уже в полёте,
        // сразу включается нативный вид; на станции сначала идёт её команда,
        // а рендер появляется только после перехода игры. F5 на станции
        // открывает нативную закупку.
        if code == KeyCode::F5 || function_game_key(code).is_some() {
            self.function_key(code);
            return;
        }
        match code {
            KeyCode::Escape => {
                self.menu = true;
                self.menu_sel = 0;
            }
            _ => {}
        }
        // На станции в нативном режиме любая неиспользованная клавиша
        // пропадает: эмулятор заморожен и не должен получать ввод.
        if self.station_native && !self.menu {
            self.release_pressed(code);
        }
    }

    /// Обработка клавиш в меню паузы (Esc). Пункты: 0 — продолжить,
    /// 1 — сохранить, 2 — загрузить, 3 — выйти. На заставке сохранение
    /// отключено (игрового процесса ещё нет), загрузка разрешена: данные
    /// ложатся в блок командира, и запуск по `SPACE` начнёт игру с ними.
    fn menu_key(&mut self, code: KeyCode) {
        let intro = self.machine.intro_seen;
        match code {
            KeyCode::Escape => self.menu = false,
            KeyCode::ArrowUp => self.menu_sel = self.menu_sel.saturating_sub(1),
            KeyCode::ArrowDown => self.menu_sel = (self.menu_sel + 1).min(3),
            KeyCode::Digit1 => self.menu_sel = 0,
            KeyCode::Digit2 => self.menu_sel = 1,
            KeyCode::Digit3 => self.menu_sel = 2,
            KeyCode::Digit4 => self.menu_sel = 3,
            KeyCode::Enter | KeyCode::Space => match self.menu_sel {
                0 => self.menu = false,
                1 => {
                    if !intro {
                        storage::do_save(&mut self.machine, self.opts, &mut self.last_path);
                        self.menu = false;
                    }
                }
                2 => {
                    storage::do_load(&mut self.machine, self.opts, &mut self.last_path);
                    self.menu = false;
                }
                _ => self.quit = true,
            },
            _ => {}
        }
    }

    /// Снять нажатие из матрицы и джойстика: клавиша не должна уйти в
    /// эмулятор, если её обработал нативный экран.
    fn release_pressed(&mut self, code: KeyCode) {
        if let Some((row, bit)) = matrix_pos(code) {
            self.matrix[row][bit as usize] = false;
        }
        if let Some(bit) = kempston_bit(code) {
            self.kempston[bit as usize] = false;
        }
        self.pending_key = None;
    }

    /// Переключить полноэкранный (borderless) режим.
    fn toggle_fullscreen(&mut self) {
        let Some(window) = &self.window else {
            return;
        };
        if window.fullscreen().is_some() {
            window.set_fullscreen(None);
        } else {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        // Возврат из полного экрана сразу показывает курсор, а вход отсчитывает
        // тайм-аут заново — успеем подвигать указатель до скрытия.
        self.note_mouse_activity();
    }

    /// Перенести состояние клавиатуры и джойстика в эмулятор.
    pub(super) fn apply_input(&mut self) {
        if self.station_native {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            self.pending_key = None;
            return;
        }
        if self.menu || self.shop.active {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            return;
        }
        if self.hdmap.active {
            let in_flight = !self.machine.intro_seen
                && !flight::z80_is_docked(&self.machine.mem)
                && (flight::z80_in_cockpit(&self.machine.mem)
                    || flight::z80_in_transition(&self.machine.mem));
            if !in_flight {
                self.machine.release_all_keys();
                self.machine.release_joystick();
                return;
            }
            // Под картой эмулятор продолжает работать, но управление кораблём
            // отфильтровывается в `forward_input`.
        }
        // В режиме анимации (вылет/прыжок/стыковка) эмулятор проигрывает
        // анимацию и игра не реагирует на нажатие кнопок.
        if flight::z80_in_transition(&self.machine.mem) {
            // Во время отсчёта под картой клавиши прыжка уже переданы отдельно;
            // остальной ввод в анимации не нужен.
            self.machine.release_all_keys();
            self.machine.release_joystick();
            return;
        }
        // Под нативным рынком стрелки обрабатывает сам экран. Не даём им
        // уйти в эмулятор через Kempston даже если клавиша была нажата до
        // открытия экрана.
        if self.market.native && flight::z80_is_docked(&self.machine.mem) {
            for bit in &mut self.kempston[..4] {
                *bit = false;
            }
        }
        self.forward_input();
    }

    /// Передать в эмулятор только одну отложенную клавишу. Используется
    /// модальными нативными экранами, которые иначе не пускают ввод в игру:
    /// так `G`/`H` запускает отсчёт гиперпрыжка под картой.
    pub(super) fn forward_pending_key(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
        if let Some((row, bit)) = self.pending_key.take() {
            self.machine.set_key(row, bit, true);
        }
    }

    /// Передать матрицу клавиш и джойстик в эмулятор. Управление кораблём
    /// уходит в игру только из нативного вида полёта (`F1`–`F4` / `1`–`4`).
    /// Под информационными экранами и на экране эмулятора стрельба, тангаж и
    /// крен не должны попадать в эмулятор.
    pub(super) fn forward_input(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
        // В режиме анимации игра не реагирует на нажатие кнопок.
        if flight::z80_in_transition(&self.machine.mem) {
            self.pending_key = None;
            return;
        }
        let controls = self.ship_controls_allowed();
        for (row, bits) in self.matrix.iter().enumerate() {
            for (bit, &down) in bits.iter().enumerate() {
                if down && (controls || !SHIP_CONTROL_KEYS.contains(&(row, bit as u8))) {
                    self.machine.set_key(row, bit as u8, true);
                }
            }
        }
        // Нажатие, которым закрыли нативный экран, подаём один кадр даже если
        // клавишу уже отпустили (иначе быстрый тап игра бы не заметила).
        if let Some((row, bit)) = self.pending_key.take() {
            self.machine.set_key(row, bit, true);
        }
        for (bit, &down) in self.kempston.iter().enumerate() {
            if controls {
                self.machine.set_kempston(bit as u8, down);
            }
        }
        if !controls {
            return;
        }
        // Дублируем джойстик на те же клавиши, чтобы стрелки и RightCtrl работали.
        if self.kempston[0] {
            self.machine.set_key(7, 2, true); // вправо
        }
        if self.kempston[1] {
            self.machine.set_key(7, 3, true); // влево
        }
        if self.kempston[2] {
            self.machine.set_key(0, 2, true); // вниз
        }
        if self.kempston[3] {
            self.machine.set_key(1, 1, true); // вверх
        }
        if self.kempston[4] {
            self.machine.set_key(1, 0, true); // огонь
        }
    }

    /// Разрешено ли управление кораблём: нативный вид полёта или экран
    /// эмулятора в полёте, без наложенных информационных экранов.
    fn ship_controls_allowed(&self) -> bool {
        let in_flight = self.flight.active
            || (self.flight_emulator
                && !self.machine.intro_seen
                && !flight::z80_is_docked(&self.machine.mem));
        in_flight
            && !self.hdmap.active
            && !self.shop.active
            && !self.status.active
            && !self.sysinfo.active
            && !self.market.native
            && !self.docked.is_active()
    }
}

/// Буква физической клавиши A-Z (для ввода в поиске на карте).
fn letter_of(code: KeyCode) -> Option<char> {
    Some(match code {
        KeyCode::KeyA => 'A',
        KeyCode::KeyB => 'B',
        KeyCode::KeyC => 'C',
        KeyCode::KeyD => 'D',
        KeyCode::KeyE => 'E',
        KeyCode::KeyF => 'F',
        KeyCode::KeyG => 'G',
        KeyCode::KeyH => 'H',
        KeyCode::KeyI => 'I',
        KeyCode::KeyJ => 'J',
        KeyCode::KeyK => 'K',
        KeyCode::KeyL => 'L',
        KeyCode::KeyM => 'M',
        KeyCode::KeyN => 'N',
        KeyCode::KeyO => 'O',
        KeyCode::KeyP => 'P',
        KeyCode::KeyQ => 'Q',
        KeyCode::KeyR => 'R',
        KeyCode::KeyS => 'S',
        KeyCode::KeyT => 'T',
        KeyCode::KeyU => 'U',
        KeyCode::KeyV => 'V',
        KeyCode::KeyW => 'W',
        KeyCode::KeyX => 'X',
        KeyCode::KeyY => 'Y',
        KeyCode::KeyZ => 'Z',
        _ => return None,
    })
}
