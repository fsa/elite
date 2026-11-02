//! Обработка клавиатуры и мыши: нативные экраны, меню и передача ввода в Z80.

use winit::keyboard::KeyCode;
use winit::window::Fullscreen;

use super::App;
use crate::flight;
use crate::galaxy;
use crate::keys::{
    digit_of, function_game_key, function_view, is_game_key, kempston_bit, matrix_pos,
};
use crate::native;
use crate::storage;
use crate::world;

impl App<'_> {
    /// Нажатие `F1`-`F4`: отдать игре оригинальную клавишу `1`-`4` и включить
    /// соответствующий нативный вид, если игра уже в полёте.
    ///
    /// На станции это родные команды (вылет/покупка/продажа/оборудование):
    /// `F1` сразу открывает нативный вид и запускает анимацию вылета. В
    /// прыжке и стыковке уже открытый вид сам показывает переход, а закрытый —
    /// открывается с нативной анимацией.
    fn function_key(&mut self, code: KeyCode) {
        let Some(key) = function_game_key(code) else {
            return;
        };
        let Some(dir) = function_view(code) else {
            return;
        };
        let mem = &self.machine.mem;
        let docked = flight::z80_is_docked(mem);
        let in_cockpit = flight::z80_in_cockpit(mem);
        let in_transition = flight::z80_in_transition(mem);

        self.hdmap.close();
        self.shop.close();
        self.menu = false;
        self.pending_shop = false;
        if docked {
            // На станции `F1`-`F4` — родные команды. `F1` (вылет) включает
            // нативный вид сразу с анимацией вылета; `F4` — нативную закупку
            // после кадра игры; `F2`/`F3` остаются родными экранами
            // покупки/продажи.
            self.pending_shop = code == KeyCode::F4;
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
        // F10 — отдельная нативная карта галактики.
        if code == KeyCode::F10 && !repeat {
            if self.hdmap.active {
                self.hdmap.close();
            } else if !self.machine.intro_seen {
                self.hdmap.open(&self.machine.mem, galaxy::KLADOV_GALAXY);
                if let Some(w) = &self.window {
                    let size = w.inner_size();
                    self.hdmap.resize(size.width, size.height);
                }
                self.shop.close();
                self.menu = false;
                self.pending_key = None;
                self.pending_flight = None;
                self.pending_launch = false;
            }
            return;
        }
        // F4 при открытой закупке закрывает её. В остальных режимах это
        // обычная функция `F4` (вид вправо или команда станции).
        if code == KeyCode::F4 && !repeat && self.shop.active {
            self.shop.close();
            return;
        }
        // При открытой карте всё управление принадлежит ей; Z80 при этом
        // продолжает работать, но ввод в игру не передаётся.
        if self.hdmap.active {
            match code {
                KeyCode::ArrowLeft | KeyCode::KeyN => self.hdmap.move_by(-4, 0),
                KeyCode::ArrowRight | KeyCode::KeyM => self.hdmap.move_by(4, 0),
                KeyCode::ArrowUp | KeyCode::KeyS => self.hdmap.move_by(0, -4),
                KeyCode::ArrowDown | KeyCode::KeyX => self.hdmap.move_by(0, 4),
                KeyCode::Enter | KeyCode::Space => {
                    // На автоповторе не подтверждаем выбор повторно.
                    if !repeat {
                        self.hdmap.snap_to_nearest();
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
                        self.hdmap.close();
                    }
                }
                // F1-F4 — те же клавиши 1-4 оригинала: карта закрывается,
                // нажатие уходит игре, а нативный вид/закупка включаются по
                // фактическому состоянию эмулятора.
                KeyCode::F1 | KeyCode::F2 | KeyCode::F3 | KeyCode::F4 if !repeat => {
                    self.function_key(code);
                }
                KeyCode::Equal | KeyCode::NumpadAdd => self.hdmap.zoom_by(1),
                KeyCode::Minus | KeyCode::NumpadSubtract => self.hdmap.zoom_by(-1),
                KeyCode::Escape if !repeat => self.hdmap.close(),
                // Игровая клавиша: закрываем карту и отдаём нажатие игре,
                // чтобы из карты сразу перейти к экранам игры (командир,
                // станция и т.п.), не закрывая её вручную.
                _ => {
                    if is_game_key(code) {
                        self.hdmap.close();
                        self.pending_key = matrix_pos(code);
                    }
                }
            }
            return;
        }
        // При открытом экране закупки всё управление принадлежит ему.
        if self.shop.active {
            match code {
                KeyCode::Escape if !repeat => {
                    if self.shop.mount_open() {
                        self.shop.cancel_mount();
                    } else {
                        self.shop.close();
                    }
                }
                KeyCode::ArrowUp => self.shop.move_selection(-1),
                KeyCode::ArrowDown => self.shop.move_selection(1),
                // F1-F3 — клавиши 1-3 оригинала: закупка закрывается и
                // нажатие уходит игре. F4 выше уже закрывает экран.
                KeyCode::F1 | KeyCode::F2 | KeyCode::F3 if !repeat => {
                    self.function_key(code);
                }
                KeyCode::Enter | KeyCode::Space => {
                    // На автоповторе не подтверждаем выбор повторно.
                    if !repeat {
                        self.shop.confirm(&mut self.machine.mem);
                    }
                }
                _ => {
                    if let Some(d) = digit_of(code) {
                        self.shop.input_digit(&mut self.machine.mem, d);
                    } else if is_game_key(code) {
                        // Игровая клавиша: закрываем закупку и отдаём нажатие игре.
                        self.shop.close();
                        self.pending_key = matrix_pos(code);
                    }
                }
            }
            return;
        }
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
        if self.flight.active {
            let view = match code {
                KeyCode::F1 => Some(flight::ViewDir::Forward),
                KeyCode::F2 => Some(flight::ViewDir::Back),
                KeyCode::F3 => Some(flight::ViewDir::Left),
                KeyCode::F4 => Some(flight::ViewDir::Right),
                _ => None,
            };
            if let Some(view) = view {
                self.flight.set_view_dir(view);
                self.pending_key = function_game_key(code).and_then(matrix_pos);
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
            if code == KeyCode::Space {
                native::start_game(&mut self.machine, world::save_layout_kladov());
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
        // а рендер (вид/закупка) появляется только после перехода игры.
        if function_game_key(code).is_some() {
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
        self.forward_input();
    }

    /// Передать матрицу клавиш и джойстик в эмулятор без проверки нативных
    /// экранов. Нужно экрану полёта: под нативным рендером продолжает
    /// работать эмулятор, и ему требуется живой ввод.
    pub(super) fn forward_input(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
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
