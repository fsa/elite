//! Обработка клавиатуры и мыши: нативные экраны, меню и передача ввода в Z80.

use winit::keyboard::KeyCode;
use winit::window::Fullscreen;

use super::App;
use crate::flight;
use crate::keys::{
    digit_of, function_game_key, function_view, is_game_key, kempston_bit, matrix_pos,
};
use crate::native;
use crate::storage;
use crate::world;

impl App<'_> {
    /// Нажатие `F1`-`F4`/`F5`: отдать игре соответствующую оригинальную
    /// клавишу или переключить нативный экран.
    ///
    /// На станции доступны только `F1` (вылет) и `F5` (оборудование): `F1`
    /// сразу открывает нативный вид и запускает анимацию вылета. `F2`-`F4`
    /// пустые — их цифры увели бы эмулятор в режимы покупки/продажи и закупки
    /// оригинала. В прыжке и стыковке уже открытый вид сам показывает переход,
    /// а закрытый — открывается с нативной анимацией.
    fn function_key(&mut self, code: KeyCode) {
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
            self.menu = false;
            self.pending_shop = docked;
            self.pending_flight = None;
            self.pending_launch = false;
            // Для закупки эмулятору передаём `L` (режим информации о пилоте),
            // а не `4`: экран 4 в оригинале ждёт номер товара и мешает
            // нативному экрану, а экран `L` пассивен.
            self.pending_key = if docked {
                matrix_pos(KeyCode::KeyL)
            } else {
                None
            };
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

    /// Обработать событие клавиши (по физической позиции, раскладка не важна).
    pub(super) fn key_event(&mut self, code: KeyCode, pressed: bool, repeat: bool) {
        if let Some((row, bit)) = matrix_pos(code) {
            self.matrix[row][bit as usize] = pressed;
        }
        if let Some(bit) = kempston_bit(code) {
            self.kempston[bit as usize] = pressed;
        }
        if !pressed {
            return;
        }
        // F12 — ускорение эмуляции Z80 вдвое. Работает в любом режиме и в игру
        // не передаётся (в матрице ZX такой клавиши нет).
        if code == KeyCode::F12 && !repeat {
            self.accelerated = !self.accelerated;
            return;
        }
        // F10 — пустая клавиша: ни карту не открывает, ни в эмулятор
        // ничего не уходит (в матрице ZX её нет, но обработку всё равно
        // завершаем явно, чтобы она не задела другие режимы).
        if code == KeyCode::F10 {
            return;
        }

        // В режиме анимации (вылет, гиперпереход, стыковка) переключение
        // экранов недоступно: работает только переход между F1 и 1 (нативная
        // отрисовка или эмулятор) и пауза. Эмулятор в это время проигрывает
        // анимацию и игра не реагирует на кнопки.
        if !self.machine.intro_seen && !self.menu && flight::z80_in_transition(&self.machine.mem) {
            if repeat {
                return;
            }
            if code == KeyCode::F1 {
                // Показать нативную отрисовку текущей анимации.
                if self.flight.in_transition() {
                    // Анимация уже идёт (в том числе скрытая клавишей `1`) —
                    // просто снова показываем нативный вид без сброса состояния.
                    self.flight.active = true;
                    self.flight.set_view_dir(flight::ViewDir::Forward);
                    self.pending_flight = None;
                    self.pending_launch = false;
                } else {
                    self.show_flight(flight::ViewDir::Forward);
                    self.flight.start_transition(false, &self.machine.mem);
                }
            } else if code == KeyCode::Digit1 {
                // Показать анимацию, которую рисует эмулятор. Состояние
                // перехода сохраняем, чтобы F1 мог вернуться к той же анимации.
                self.flight.active = false;
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }

        // G/H на станции: оригинал не выполняет гиперпрыжок, а показывает
        // экран `DOCKED` и издаёт звук. Под нативным экраном этот ответ
        // эмулятора не виден, поэтому на время показываем свою заглушку.
        // Нажатие всё равно отдаём эмулятору — он рисует такой же экран и
        // проигрывает звук (см. `docked::Docked::show`).
        if !repeat
            && matches!(code, KeyCode::KeyG | KeyCode::KeyH)
            && !self.menu
            && !self.machine.intro_seen
            && flight::z80_is_docked(&self.machine.mem)
            && self.native_screen_active()
        {
            self.docked.show();
            self.pending_key = matrix_pos(code);
            // В матрице клавишу не держим: эмулятор должен получить одно
            // нажатие, а не повторять его каждый кадр, пока идёт заглушка.
            if let Some((row, bit)) = matrix_pos(code) {
                self.matrix[row][bit as usize] = false;
            }
            return;
        }
        // Пока заглушка `DOCKED` на экране, остальные клавиши не должны
        // управлять скрытым нативным экраном (например, автоповтор `G` не
        // должен закрыть карту), кроме меню паузы.
        if self.docked.is_active() {
            if self.menu {
                self.menu_key(code);
            } else if code == KeyCode::Escape && !repeat {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }
        // F6 — основная клавиша отдельной нативной карты галактики.
        // Повторное нажатие не закрывает карту и не сбрасывает курсор/зум:
        // это отдельный режим, выход из него — только переход на другой экран.
        if code == KeyCode::F6 && !repeat {
            if !self.hdmap.active && !self.machine.intro_seen {
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
        // Эмулятору отдаём `P`, а нативный экран включаем после кадра, когда
        // оригинал обработает команду и заполнит поля планеты. Если оригинал
        // уже на этом экране, достаточно включить нативную отрисовку.
        if code == KeyCode::F7 && !repeat {
            if !self.sysinfo.active && !self.menu && !self.machine.intro_seen {
                self.close_hdmap();
                self.shop.close();
                self.close_status();
                self.flight.close();
                self.close_market();
                self.pending_shop = false;
                self.pending_flight = None;
                self.pending_launch = false;
                self.pending_status = false;
                self.pending_sysinfo = false;
                if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_INFO_PLANET {
                    self.open_sysinfo();
                } else {
                    self.open_sysinfo();
                    self.pending_sysinfo = true;
                    self.pending_key = matrix_pos(KeyCode::KeyP);
                }
                self.menu = false;
            }
            return;
        }
        // F9 — нативная альтернатива экрану капитана (`L`). Эмулятору при
        // необходимости отдаём `L`, а нативный экран включаем после кадра,
        // когда оригинал перейдёт в режим показа капитана. Если оригинал уже
        // на этом экране, достаточно включить нативную отрисовку.
        if code == KeyCode::F9 && !repeat {
            if !self.status.active && !self.menu && !self.machine.intro_seen {
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
                if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_PILOT_INFO {
                    self.open_status();
                } else {
                    self.open_status();
                    self.pending_status = true;
                    self.pending_key = matrix_pos(KeyCode::KeyL);
                }
                self.menu = false;
            }
            return;
        }
        // F8 — нативный показ экрана цен (альтернатива `K`). Смотреть цены
        // можно и в полёте, поэтому стойку не требуем. Эмулятору при
        // необходимости отдаём `K`, а нативный экран включаем после кадра,
        // когда оригинал перейдёт в режим рынка. Если оригинал уже на этом
        // экране, достаточно включить нативную отрисовку. Повторное нажатие
        // не выключает показ. Работает и при открытых нативных экранах.
        // Торговля отдельно разрешена только на станции (см. блок стрелок
        // ниже и `Market::trade`).
        if code == KeyCode::F8 && !repeat {
            if !self.menu && !self.machine.intro_seen {
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
                if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_MARKET {
                    self.open_market();
                } else {
                    self.open_market();
                    self.pending_market = true;
                    self.pending_key = matrix_pos(KeyCode::KeyK);
                }
                return;
            }
        }
        // `K` — оригинальная клавиша: показать оригинальный (эмуляторный)
        // экран рынка. Нативная альтернатива выключается, ввод уходит в игру.
        if code == KeyCode::KeyK && !repeat {
            self.close_market();
        }
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
            match code {
                KeyCode::ArrowLeft | KeyCode::KeyN => {
                    self.hdmap.move_by(-4, 0);
                    self.sync_hdmap_target();
                }
                KeyCode::ArrowRight | KeyCode::KeyM => {
                    self.hdmap.move_by(4, 0);
                    self.sync_hdmap_target();
                }
                KeyCode::ArrowUp | KeyCode::KeyS => {
                    self.hdmap.move_by(0, -4);
                    self.sync_hdmap_target();
                }
                KeyCode::ArrowDown | KeyCode::KeyX => {
                    self.hdmap.move_by(0, 4);
                    self.sync_hdmap_target();
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
                // станция и т.п.), не закрывая её вручную.
                _ => {
                    if is_game_key(code) {
                        self.close_hdmap();
                        self.pending_key = matrix_pos(code);
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
                        self.shop.close();
                        self.pending_key = matrix_pos(code);
                    }
                }
            }
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
        // F11 / Alt+Enter — переключение полноэкранного режима.
        if code == KeyCode::F11 || (code == KeyCode::Enter && self.alt) {
            self.toggle_fullscreen();
            return;
        }
        // Меню паузы: своё управление, в эмулятор клавиши не уходят.
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
                } else if code == KeyCode::Digit1 {
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
            } else if code == KeyCode::Digit1 {
                self.flight.close();
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
                native::start_game(&mut self.machine, world::save_layout_kladov());
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
    }

    /// Перенести состояние клавиатуры и джойстика в эмулятор.
    pub(super) fn apply_input(&mut self) {
        if self.menu || self.hdmap.active || self.shop.active {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            return;
        }
        // В режиме анимации (вылет/прыжок/стыковка) эмулятор проигрывает
        // анимацию и игра не реагирует на нажатие кнопок.
        if flight::z80_in_transition(&self.machine.mem) {
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

    /// Передать матрицу клавиш и джойстик в эмулятор без проверки нативных
    /// экранов. Нужно экрану полёта: под нативным рендером продолжает
    /// работать эмулятор, и ему требуется живой ввод.
    pub(super) fn forward_input(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
        // В режиме анимации игра не реагирует на нажатие кнопок.
        if flight::z80_in_transition(&self.machine.mem) {
            self.pending_key = None;
            return;
        }
        for (row, bits) in self.matrix.iter().enumerate() {
            for (bit, &down) in bits.iter().enumerate() {
                if down {
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
            self.machine.set_kempston(bit as u8, down);
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
}
