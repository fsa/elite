//! Обработка клавиатуры и мыши: нативные экраны, меню и передача ввода в Z80.

use std::time::Instant;

use winit::keyboard::KeyCode;
use winit::window::Fullscreen;

use super::{App, SPEED_HOLD_RESET, SpeedKeyHold};
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

        // Любой переход на другой экран закрывает текущую страницу сразу.
        // В частности, `F1` со станции не должен оставлять закупку/карту/инвентарь
        // поверх уже открытого нативного вида вылета.
        self.shop.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_shop = false;
        self.pending_market = false;
        self.menu = false;
        if docked {
            // На станции из F1-F4 доступен только `F1` (вылет). Нативный вид
            // открывается сразу, даже если Z80 ещё не начал переход: иначе
            // окно закупки/карты оставалось бы поверх начавшегося вылета.
            if code == KeyCode::F1 {
                // Вылет возвращает управление эмулятору: он должен получить
                // клавишу `1`, иначе Z80 останется замороженным на станции.
                self.cancel_pending_station();
                self.station_native = false;
                self.station_emulator = true;
                self.show_flight(dir);
                // `show_flight` сбрасывает отложенные переходы, а признак
                // вылета нужен `switch_to_flight_on_transition()`, чтобы не
                // принять вылет за стыковку.
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
    /// `G`/`H` показывают нативный экран `DOCKED`, `Enter` открывает инвентарь,
    /// `Backquote` — единственный выход в эмулятор без игровой клавиши. Цифры
    /// `1`–`3` в модальном окне крепления лазера остаются управлением самого
    /// экрана закупки.
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
            KeyCode::Enter if !self.hdmap.active && !self.shop.active => {
                if !self.inventory.is_active() {
                    self.open_inventory();
                }
            }
            KeyCode::ControlLeft => {
                // В оригинале это CAPS SHIFT — пауза с дополнительными
                // функциями. Нативно показываем уже существующее меню Esc.
                self.menu = true;
                self.menu_sel = 0;
            }
            KeyCode::ShiftRight => {
                // Родная клавиша сохранения: открываем приглашение с именем
                // командира и комментарием, З80 заморожен.
                self.open_save_prompt();
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
            // Под картой Enter подтверждает ближайшую планету, а на закупке —
            // покупку: эти экраны обрабатывают его сами ниже.
            KeyCode::Enter if !self.hdmap.active && !self.shop.active => {
                if !self.inventory.is_active() {
                    self.open_inventory();
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
    /// открываем нативный экран. Ещё одно нажатие снимает надстройку и снова
    /// отдаёт клавишу игре, чтобы она сама ушла со своего экрана. Исключение —
    /// инвентарь: `Enter` его не закрывает, только переход на другую страницу.
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
            KeyCode::Enter if screen == Some(OriginalScreen::Inventory) => {
                if !self.inventory.is_active() {
                    self.open_inventory();
                }
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

    fn apply_save_editor_name(&mut self) {
        let layout = world::save_layout_kladov();
        let base = layout.pilot_name as usize;
        let name = self.save_editor_name_input.trim_end();
        let bytes = name.as_bytes();
        for i in 0..10 {
            if base + i >= self.machine.mem.len() {
                break;
            }
            let b = if i < bytes.len() { bytes[i] } else { b' ' };
            self.machine.mem[base + i] = b;
        }
        // byte 10 (11th) must remain unchanged (null)
    }

    fn apply_save_editor_input(&mut self) {
        if self.save_editor_input.is_empty() {
            return;
        }
        let layout = world::save_layout_kladov();
        let base = layout.pilot_name as usize;
        if self.save_editor_sel == 1 {
            // Сумма целиком в десятых кредита; две ячейки денег и их
            // нормализация — через общий доступ к блоку командира.
            if let Ok(v) = self.save_editor_input.parse::<u32>() {
                world::set_commander_cash_tenths(&mut self.machine.mem, base, v);
            }
        }
    }

    fn handle_save_editor_key(&mut self, code: KeyCode) -> bool {
        use KeyCode::*;
        if self.save_editor_name_editing {
            match code {
                Escape => {
                    self.save_editor_name_editing = false;
                    self.save_editor_name_input.clear();
                    true
                }
                Enter => {
                    self.apply_save_editor_name();
                    self.save_editor_name_editing = false;
                    self.save_editor_name_input.clear();
                    true
                }
                Backspace => {
                    self.save_editor_name_input.pop();
                    true
                }
                Space => {
                    if self.save_editor_name_input.len() < 10 {
                        self.save_editor_name_input.push(' ');
                    }
                    true
                }
                KeyA => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('A'); } true }
                KeyB => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('B'); } true }
                KeyC => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('C'); } true }
                KeyD => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('D'); } true }
                KeyE => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('E'); } true }
                KeyF => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('F'); } true }
                KeyG => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('G'); } true }
                KeyH => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('H'); } true }
                KeyI => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('I'); } true }
                KeyJ => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('J'); } true }
                KeyK => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('K'); } true }
                KeyL => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('L'); } true }
                KeyM => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('M'); } true }
                KeyN => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('N'); } true }
                KeyO => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('O'); } true }
                KeyP => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('P'); } true }
                KeyQ => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('Q'); } true }
                KeyR => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('R'); } true }
                KeyS => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('S'); } true }
                KeyT => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('T'); } true }
                KeyU => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('U'); } true }
                KeyV => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('V'); } true }
                KeyW => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('W'); } true }
                KeyX => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('X'); } true }
                KeyY => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('Y'); } true }
                KeyZ => { if self.save_editor_name_input.len() < 10 { self.save_editor_name_input.push('Z'); } true }
                _ => true,
            }
        } else if self.save_editor_editing {
            match code {
                Escape => {
                    self.save_editor_editing = false;
                    self.save_editor_input.clear();
                    return true;
                }
                Enter => {
                    self.apply_save_editor_input();
                    self.save_editor_editing = false;
                    self.save_editor_input.clear();
                    return true;
                }
                Backspace => {
                    self.save_editor_input.pop();
                    return true;
                }
                Digit0 | Numpad0 => { self.save_editor_input.push('0'); true }
                Digit1 | Numpad1 => { self.save_editor_input.push('1'); true }
                Digit2 | Numpad2 => { self.save_editor_input.push('2'); true }
                Digit3 | Numpad3 => { self.save_editor_input.push('3'); true }
                Digit4 | Numpad4 => { self.save_editor_input.push('4'); true }
                Digit5 | Numpad5 => { self.save_editor_input.push('5'); true }
                Digit6 | Numpad6 => { self.save_editor_input.push('6'); true }
                Digit7 | Numpad7 => { self.save_editor_input.push('7'); true }
                Digit8 | Numpad8 => { self.save_editor_input.push('8'); true }
                Digit9 | Numpad9 => { self.save_editor_input.push('9'); true }
                _ => true,
            }
        } else {
            if code == Escape {
                self.save_editor_open = false;
                return true;
            }
            let layout = world::save_layout_kladov();
            let base = layout.pilot_name as usize;
            match code {
                ArrowUp => {
                    if self.save_editor_sel > 0 {
                        self.save_editor_sel -= 1;
                    }
                    true
                }
                ArrowDown => {
                    if self.save_editor_sel < 11 {
                        self.save_editor_sel += 1;
                    }
                    true
                }
                Enter => {
                    if self.save_editor_sel == 1 {
                        self.save_editor_editing = true;
                        self.save_editor_input.clear();
                    } else if self.save_editor_sel == 0 {
                        // start name editing
                        self.save_editor_name_editing = true;
                        self.save_editor_name_input = String::new();
                        for i in 0..10 {
                            if let Some(&b) = self.machine.mem.get(base + i) {
                                let ch = if (0x20..=0x7F).contains(&b) { b as char } else { ' ' };
                                self.save_editor_name_input.push(ch);
                            }
                        }
                        self.save_editor_name_input = self.save_editor_name_input.trim_end().to_string();
                    }
                    true
                }
                ArrowLeft | ArrowRight => {
                    let delta: i32 = if code == ArrowLeft { -1 } else { 1 };
                    match self.save_editor_sel {
                        1 => {
                            let v = world::commander_cash_tenths(&self.machine.mem, base);
                            let nv = (v as i64 + (delta as i64) * 10)
                                .clamp(0, world::MAX_CASH_TENTHS as i64) as u32;
                            world::set_commander_cash_tenths(&mut self.machine.mem, base, nv);
                            true
                        }
                        2 => {
                            let off = base + 0x2E;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        3 => {
                            let off = base + 0x2F;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        4 => {
                            let off = base + 0x0B;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        5 => {
                            // RATING: очки лежат в PilotRating0..1. PilotRating2
                            // оригинал использует отдельно и здесь не трогаем.
                            let r0 = base + 0x0C;
                            let r1 = base + 0x0D;
                            if r1 < self.machine.mem.len() {
                                let rating = u16::from_le_bytes([
                                    self.machine.mem[r0],
                                    self.machine.mem[r1],
                                ]) as i64;
                                let nv = (rating + delta as i64 * 256).clamp(0, 0xFFFF) as u16;
                                self.machine.mem[r0..=r1].copy_from_slice(&nv.to_le_bytes());
                            }
                            true
                        }
                        6 => {
                            let off = base + 0x0E;
                            if off < self.machine.mem.len() {
                                self.machine.mem[off] ^= 0x80;
                            }
                            true
                        }
                        7 => {
                            let off = base + 0x0E;
                            if off < self.machine.mem.len() {
                                self.machine.mem[off] ^= 0x40;
                            }
                            true
                        }
                        8 => {
                            let off = base + 0x0E;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off];
                                let rank = ((v >> 1) & 0x07) as i32;
                                let nv = (rank + delta).rem_euclid(8) as u8;
                                self.machine.mem[off] = (v & !0x0E) | (nv << 1);
                            }
                            true
                        }
                        9 => {
                            let off = base + 0x0E;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        10 => {
                            let off = base + 0x28;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        11 => {
                            let off = base + 0x29;
                            if off < self.machine.mem.len() {
                                let v = self.machine.mem[off] as i32;
                                let nv = (v + delta).clamp(0, 255) as u8;
                                self.machine.mem[off] = nv;
                            }
                            true
                        }
                        _ => false,
                    }
                }
                _ => true,
            }
        }
    }

    /// Короткое нажатие `F12` выбирает следующий множитель, долгое — x1.
    ///
    /// Переключение происходит на отпускании: пока клавиша зажата, ещё нельзя
    /// знать, будет ли это короткий цикл или долгий сброс. Автоповтор не меняет
    /// множитель и не мешает отсчёту удержания.
    fn handle_speed_key(&mut self, pressed: bool, repeat: bool) {
        if pressed {
            if repeat || self.speed_key_hold.is_some() {
                return;
            }
            self.speed_key_hold = Some(SpeedKeyHold {
                started: Instant::now(),
                reset_done: false,
            });
            return;
        }
        let Some(hold) = self.speed_key_hold.take() else {
            return;
        };
        if !hold.reset_done && hold.started.elapsed() < SPEED_HOLD_RESET {
            self.speed_multiplier = match self.speed_multiplier {
                1 => 2,
                2 => 5,
                5 => 10,
                _ => 1,
            };
        }
    }

    /// Сбросить множитель, если `F12` удерживается дольше порога.
    pub(super) fn update_speed_hold(&mut self) {
        let Some(hold) = self.speed_key_hold.as_mut() else {
            return;
        };
        if !hold.reset_done && hold.started.elapsed() >= SPEED_HOLD_RESET {
            self.speed_multiplier = 1;
            hold.reset_done = true;
        }
    }

    /// Потеря фокуса не должна оставлять `F12` зажатой.
    pub(super) fn cancel_speed_hold(&mut self) {
        self.speed_key_hold = None;
    }

    /// F7/P — нативная альтернатива экрану данных о планете.
    fn toggle_planet_info(&mut self) {
        if self.menu || self.machine.intro_seen {
            return;
        }
        let docked = flight::z80_is_docked(&self.machine.mem);
        if self.sysinfo.active && !docked {
            self.close_sysinfo();
        } else {
            self.open_planet_info();
        }
    }

    /// F9/L — нативная альтернатива экрану капитана.
    fn toggle_status(&mut self) {
        if self.menu || self.machine.intro_seen {
            return;
        }
        let docked = flight::z80_is_docked(&self.machine.mem);
        if self.status.active && !docked {
            self.close_status();
            return;
        }
        let on_status =
            self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_PILOT_INFO;
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

    /// F8/K — нативная альтернатива экрану цен.
    fn toggle_market(&mut self) {
        if self.menu || self.machine.intro_seen {
            return;
        }
        let docked = flight::z80_is_docked(&self.machine.mem);
        if self.market.native && !docked {
            self.close_market();
            return;
        }
        let on_market = self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_MARKET;
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

    /// Стрелки на заставке: ◄/► переключают корабль, ▲/▼ в отладочном режиме
    /// наклоняют его. Возвращает `true`, если клавиша поглощена заставкой.
    ///
    /// Вызывается до общего запрета автоповтора: удержание ▲/▼ должно
    /// вращать модель, а ◄/► переключают её только по первому нажатию.
    fn splash_arrow_key(&mut self, code: KeyCode, repeat: bool) -> bool {
        if !self.machine.intro_seen || self.menu {
            return false;
        }
        match code {
            KeyCode::ArrowLeft if !repeat => self.splash.cycle_ship(-1),
            KeyCode::ArrowRight if !repeat => self.splash.cycle_ship(1),
            KeyCode::ArrowUp => self.splash.rotate_vertical(1),
            KeyCode::ArrowDown => self.splash.rotate_vertical(-1),
            _ => return false,
        }
        // Пока виден родной интро-кадр, стрелки не должны уходить в Z80:
        // они управляют витриной, а не кораблем в игре.
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
        // F12 обрабатываем и на отпускании: короткое нажатие переключает
        // множитель, а долгое удержание уже сбросило его в `update_speed_hold`.
        if code == KeyCode::F12 {
            self.handle_speed_key(pressed, repeat);
            return;
        }
        if !pressed {
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
        // Кнопка ` есть во всех режимах: она снимает нативный слой и отдаёт
        // кадр эмулятору, не посылая в игру ни одной клавиши. Обрабатываем её
        // раньше паузы, редактора и меню, иначе они поглотили бы нажатие.
        // На заставке это переключатель: повторное нажатие возвращает титр.
        if !repeat && emulator_handoff_key(code) {
            self.toggle_emulator_handoff();
            self.release_pressed(code);
            return;
        }
        // Экран сохранения/загрузки перехватывает весь ввод, включая игровые
        // клавиши. Системные команды выше (F11/F12/F10, выход в эмулятор) уже
        // отработаны.
        if self.save_screen.is_some() {
            self.handle_save_screen_key(code, repeat);
            self.release_pressed(code);
            return;
        }
        // Пауза Кладова перехватывает все игровые клавиши: Z80 в это время
        // заморожен, поэтому сервисные настройки применяет нативный слой.
        // Системные команды выше (полный экран, частота эмуляции) и Ctrl+Q
        // остаются доступными.
        if self.pause_active {
            if code == KeyCode::KeyQ && self.ctrl {
                self.quit = true;
                self.release_pressed(code);
                return;
            }
            self.handle_pause_key(code, repeat);
            return;
        }
        if code == KeyCode::ShiftLeft && !repeat && self.can_enter_pause() {
            self.enter_pause();
            return;
        }
        // Alt+E — открыть/закрыть редактор сейва (секретная комбинация). Игра на паузе, пока открыт.
        if code == KeyCode::KeyE && self.alt && !repeat {
            // `intro_seen` взведён на заставке: до входа в игру комбинация
            // не должна открывать редактор.
            if self.machine.intro_seen {
                return;
            }
            self.save_editor_open = !self.save_editor_open;
            if self.save_editor_open {
                self.save_editor_sel = 0;
                self.save_editor_editing = false;
                self.save_editor_input.clear();
                self.save_editor_name_editing = false;
                self.save_editor_name_input.clear();
                self.save_editor_hex_cursor = 0;
                self.save_editor_hex_editing = false;
                self.save_editor_hex_input.clear();
            }
            return;
        }
        // Полноэкранный редактор сейва: перехватываем ввод.
        if self.save_editor_open && !repeat {
            if self.handle_save_editor_key(code) {
                return;
            }
        }

        // Нативный диалог миссии: original уже ждёт символ `Y/N`, остальной
        // ввод не должен менять нативные экраны или передавать клавишу в Z80.
        if self.mission_prompt.is_some() {
            if !repeat {
                match code {
                    KeyCode::KeyY => self.answer_mission_prompt(true),
                    KeyCode::KeyN => self.answer_mission_prompt(false),
                    KeyCode::ArrowLeft | KeyCode::ArrowRight => self.toggle_mission_prompt(),
                    KeyCode::Enter => {
                        let accept = self.mission_prompt.map_or(true, |p| p.accept);
                        self.answer_mission_prompt(accept);
                    }
                    _ => {}
                }
            }
            if let Some((row, bit)) = matrix_pos(code) {
                self.matrix[row][bit as usize] = false;
            }
            return;
        }
        // Пока Z80 исполняет миссионные проверки, нативные экраны не переключаются.
        if self.market_mission.is_some() {
            return;
        }

        // В режиме анимации (вылет, гиперпереход, стыковка) не работают ни
        // переключения экранов, ни органы управления. Отсчёт гиперпрыжка ещё
        // не анимация: до обнуления счётчика экраны переключаются, а число
        // показывается поверх каждого из них. Переход сам открывает передний
        // нативный вид; выйти в эмулятор можно только кнопкой `, обработанной
        // выше. Системные комбинации (полный экран, скорость эмуляции) тоже
        // обработаны выше и на игру не влияют.
        if !self.machine.intro_seen
            && !self.menu
            && (flight::z80_in_transition_animation(&self.machine.mem)
                || self.flight.in_animation_mode())
        {
            self.release_pressed(code);
            return;
        }

        // Пока заглушка `DOCKED` на экране, клавиши не должны
        // управлять скрытым нативным экраном и не передаются эмулятору,
        // кроме меню паузы.
        if self.docked.is_active() {
            if self.menu {
                self.menu_key(code);
            } else if matches!(code, KeyCode::F1 | KeyCode::Digit1) && !repeat {
                // Вылет доступен и под заглушкой стыковки: снимаем её и
                // передаём клавишу `1` в общий обработчик F1.
                self.docked.close();
                self.function_key(KeyCode::F1);
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
        // Пока открыт инвентарь, посторонние клавиши не должны управлять
        // замороженной станцией или кораблём под наложением. Переходы на другие
        // нативные экраны остаются доступными, а Esc работает здесь же.
        if self.inventory.is_active() && !self.machine.intro_seen {
            if self.menu {
                self.menu_key(code);
                return;
            }
            let switch_screen = function_game_key(code).is_some()
                || matches!(
                    code,
                    KeyCode::F5
                        | KeyCode::F6
                        | KeyCode::F7
                        | KeyCode::F8
                        | KeyCode::F9
                        | KeyCode::Digit1
                        | KeyCode::Digit2
                        | KeyCode::Digit3
                        | KeyCode::Digit4
                        | KeyCode::KeyI
                        | KeyCode::KeyO
                        | KeyCode::KeyP
                        | KeyCode::KeyK
                        | KeyCode::KeyL
                );
            if !switch_screen {
                if code == KeyCode::Escape && !repeat {
                    self.menu = true;
                    self.menu_sel = 0;
                }
                self.release_pressed(code);
                return;
            }
        }
        // F6 открывает нативную карту в галактическом масштабе (`I` открывает
        // то же самое, `O` — детальный фрагмент). Повторное нажатие не закрывает
        // карту и не сбрасывает курсор: как и `I`, оно только ставит масштаб
        // «вся галактика».
        if code == KeyCode::F6 && !repeat {
            if !self.menu && !self.machine.intro_seen {
                self.close_market();
                if self.hdmap.active {
                    self.open_hdmap_mode(hdmap::MapMode::Galaxy);
                } else {
                    self.open_hdmap();
                }
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
            self.toggle_planet_info();
            return;
        }
        // F9 — нативная альтернатива экрану капитана (`L`).
        if code == KeyCode::F9 && !repeat && !(self.hdmap.active && self.hdmap.is_searching()) {
            self.toggle_status();
            return;
        }
        // F8 — нативный показ экрана цен (альтернатива `K`).
        if code == KeyCode::F8 && !repeat && !(self.hdmap.active && self.hdmap.is_searching()) {
            self.toggle_market();
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
                } else if self.station_native && self.start_market_mission(buy) {
                    // Original получит `2`/`3` и сам решит, выдаётся ли миссия.
                    // Если её нет, сеанс отменится и сделку применим нативно.
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
                // В режиме эмулятора оригинальные клавиши экранов должны
                // открывать нативные экраны, как F7-F9, а не уходить в Z80
                // вместе с закрытием карты. Матрицу чистим до открытия: оно
                // само может отложить клавишу `L` для заморозки станции.
                KeyCode::KeyL if !repeat => {
                    self.release_pressed(code);
                    self.toggle_status();
                }
                KeyCode::KeyP if !repeat => {
                    self.release_pressed(code);
                    self.toggle_planet_info();
                }
                KeyCode::KeyK if !repeat => {
                    self.release_pressed(code);
                    self.toggle_market();
                }
                KeyCode::Digit2 | KeyCode::Digit3
                    if !repeat && flight::z80_is_docked(&self.machine.mem) =>
                {
                    self.release_pressed(code);
                    self.toggle_market();
                }
                KeyCode::Digit4 if !repeat && flight::z80_is_docked(&self.machine.mem) => {
                    self.release_pressed(code);
                    self.close_market();
                    self.open_shop();
                }
                // Цифры 1-4 оригинала — те же переходы к видам/экранам, что
                // F1-F4. Отдаём их через общий обработчик, чтобы экран
                // переключился нативно, а клавиша ушла в игру только как часть
                // этого перехода.
                KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4
                    if !repeat =>
                {
                    let fkey = match code {
                        KeyCode::Digit1 => KeyCode::F1,
                        KeyCode::Digit2 => KeyCode::F2,
                        KeyCode::Digit3 => KeyCode::F3,
                        _ => KeyCode::F4,
                    };
                    self.function_key(fkey);
                }
                // Всё остальное при открытой карте поглощается: посторонняя
                // игровая клавиша не должна закрывать карту и тем более уходить
                // в эмулятор, меняя там экран. Единственный выход — Backquote,
                // он обработан раньше.
                _ => {}
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
                // F1 — клавиша 1 оригинала: закупка закрывается, открывается
                // нативный вид и нажатие уходит игре для вылета.
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
        // Нативные экраны капитана (F9) и информации о планете (F7) не
        // перехватывают ввод: это альтернативная отрисовка экранов `L`/`P`,
        // управление остаётся за эмулятором.
        if self.splash_arrow_key(code, repeat) {
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
        // Меню паузы: своё управление, в эмулятор клавиши не уходят.
        // В нативном режиме станции меню может быть открыто поверх текущего
        // экрана, поэтому обрабатываем его до станционных команд.
        if self.menu {
            self.menu_key(code);
            return;
        }
        // Экран полёта: F1-4 — те же клавиши 1-4 оригинала (передний/задний/
        // левый/правый вид), Esc открывает меню паузы. Пока нативная анимация
        // перехода продолжается после сброса флагов Z80, ввод тоже заблокирован.
        if self.flight.active {
            if self.flight.in_animation_mode() {
                self.release_pressed(code);
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
        // стрелки обработаны выше (◄/► — корабль, ▲/▼ — наклон в `--debug`),
        // остальные клавиши (кроме меню и ` из обработчика выше) игнорируются.
        // SPACE работает и после выхода в эмулятор, чтобы старт игры всегда
        // шёл через нативные хуки.
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
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
                self.release_pressed(code);
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
    /// недоступно: пункт остаётся видимым, но стрелки и цифра `2` его
    /// пропускают, а `Enter` не действует. Загрузка разрешена и на заставке:
    /// данные ложатся в блок командира, и запуск по `SPACE` начнёт игру с ними.
    fn menu_key(&mut self, code: KeyCode) {
        let intro = self.machine.intro_seen;
        let selectable = |i: usize| !intro || i != 1;
        match code {
            KeyCode::Escape => self.menu = false,
            KeyCode::ArrowUp => {
                while self.menu_sel > 0 {
                    self.menu_sel -= 1;
                    if selectable(self.menu_sel) {
                        break;
                    }
                }
            }
            KeyCode::ArrowDown => {
                while self.menu_sel < 3 {
                    self.menu_sel += 1;
                    if selectable(self.menu_sel) {
                        break;
                    }
                }
            }
            KeyCode::Digit1 => self.menu_sel = 0,
            KeyCode::Digit2 if !intro => self.menu_sel = 1,
            KeyCode::Digit3 => self.menu_sel = 2,
            KeyCode::Digit4 => self.menu_sel = 3,
            KeyCode::Enter | KeyCode::Space => match self.menu_sel {
                0 => self.menu = false,
                1 if !intro => self.open_save_prompt(),
                1 => {} // недоступный пункт «SAVE» на заставке
                2 => self.open_load_list(),
                _ => self.quit = true,
            },
            _ => {}
        }
    }

    /// Печатный символ для полей ввода без текстового события (тесты и старые
    /// вызовы key_event): буквы и цифры физической раскладки.
    fn physical_ascii(code: KeyCode) -> Option<char> {
        use KeyCode::*;
        Some(match code {
            KeyA => 'A',
            KeyB => 'B',
            KeyC => 'C',
            KeyD => 'D',
            KeyE => 'E',
            KeyF => 'F',
            KeyG => 'G',
            KeyH => 'H',
            KeyI => 'I',
            KeyJ => 'J',
            KeyK => 'K',
            KeyL => 'L',
            KeyM => 'M',
            KeyN => 'N',
            KeyO => 'O',
            KeyP => 'P',
            KeyQ => 'Q',
            KeyR => 'R',
            KeyS => 'S',
            KeyT => 'T',
            KeyU => 'U',
            KeyV => 'V',
            KeyW => 'W',
            KeyX => 'X',
            KeyY => 'Y',
            KeyZ => 'Z',
            Digit0 => '0',
            Digit1 => '1',
            Digit2 => '2',
            Digit3 => '3',
            Digit4 => '4',
            Digit5 => '5',
            Digit6 => '6',
            Digit7 => '7',
            Digit8 => '8',
            Digit9 => '9',
            Space => ' ',
            Minus => '-',
            Equal => '=',
            Comma => ',',
            Period => '.',
            Slash => '/',
            Semicolon => ';',
            Quote => '\'',
            BracketLeft => '[',
            BracketRight => ']',
            Backslash => '\\',
            _ => return None,
        })
    }

    /// Ввод на экране сохранения и таблицы загрузки.
    ///
    /// В приглашении Enter с поля NAME просто подтверждает имя и переводит
    /// фокус в комментарий; второй Enter записывает сейв. В таблице Enter
    /// грузит запись, E/I экспортируют/импортируют 102-байтовый файл, Del —
    /// удаляет запись из каталога.
    fn handle_save_screen_key(&mut self, code: KeyCode, repeat: bool) {
        use KeyCode::*;
        let typed = self.input_char.take().filter(|_| !repeat);
        match self.save_screen.clone() {
            Some(storage::SaveScreen::Save(mut prompt)) => match code {
                Escape => self.close_save_screen(),
                Tab | ArrowUp | ArrowDown if !repeat => {
                    prompt.field = match prompt.field {
                        storage::PromptField::Name => storage::PromptField::Comment,
                        storage::PromptField::Comment => storage::PromptField::Name,
                    };
                    prompt.status.clear();
                    self.save_screen = Some(storage::SaveScreen::Save(prompt));
                }
                Backspace => {
                    match prompt.field {
                        storage::PromptField::Name => {
                            prompt.name.pop();
                        }
                        storage::PromptField::Comment => {
                            prompt.comment.pop();
                        }
                    }
                    self.save_screen = Some(storage::SaveScreen::Save(prompt));
                }
                Enter if !repeat => match prompt.field {
                    storage::PromptField::Name => {
                        prompt.field = storage::PromptField::Comment;
                        prompt.status.clear();
                        self.save_screen = Some(storage::SaveScreen::Save(prompt));
                    }
                    storage::PromptField::Comment => {
                        match storage::save_to_catalog(
                            &mut self.machine,
                            &prompt.name,
                            &prompt.comment,
                            world::save_layout_kladov(),
                        ) {
                            Ok(_) => self.close_save_screen(),
                            Err(_) => {
                                prompt.status = "ERROR: CANNOT WRITE CATALOG".to_string();
                                self.save_screen = Some(storage::SaveScreen::Save(prompt));
                            }
                        }
                    }
                },
                _ => {
                    if let Some(c) = typed.or_else(|| Self::physical_ascii(code)) {
                        match prompt.field {
                            storage::PromptField::Name => {
                                if let Some(c) = storage::typed_name_char(c) {
                                    if prompt.name.len() < storage::NAME_LEN {
                                        prompt.name.push(c);
                                    }
                                }
                            }
                            storage::PromptField::Comment => {
                                if let Some(c) = storage::typed_comment_char(c) {
                                    if prompt.comment.len() < storage::COMMENT_LEN {
                                        prompt.comment.push(c);
                                    }
                                }
                            }
                        }
                        self.save_screen = Some(storage::SaveScreen::Save(prompt));
                    }
                }
            },
            Some(storage::SaveScreen::Load(mut list)) => {
                let typed = typed.or_else(|| {
                    if repeat {
                        None
                    } else {
                        Self::physical_ascii(code)
                    }
                });
                if list.confirm_delete {
                    match typed {
                        Some(c) if c.to_ascii_lowercase() == 'y' => {
                            if let Some(entry) = list.entries.get(list.sel).cloned() {
                                list.status = match storage::delete_entry(&entry) {
                                    Ok(mut entries) => {
                                        storage::sorted_for_table(&mut entries);
                                        list.sel = list.sel.min(entries.len().saturating_sub(1));
                                        list.entries = entries;
                                        "DELETED".to_string()
                                    }
                                    Err(_) => "ERROR: CANNOT DELETE".to_string(),
                                };
                            }
                            list.confirm_delete = false;
                        }
                        Some(c) if c.to_ascii_lowercase() == 'n' => {
                            list.confirm_delete = false;
                            list.status = "DELETE CANCELLED".to_string();
                        }
                        _ if code == Escape && !repeat => {
                            list.confirm_delete = false;
                            list.status = "DELETE CANCELLED".to_string();
                        }
                        _ => {}
                    }
                    self.save_screen = Some(storage::SaveScreen::Load(list));
                } else {
                    match code {
                        Escape => self.close_save_screen(),
                        ArrowUp if !repeat && list.sel > 0 => {
                            list.sel -= 1;
                            list.status.clear();
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                        ArrowDown if !repeat && list.sel + 1 < list.entries.len() => {
                            list.sel += 1;
                            list.status.clear();
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                        Enter if !repeat => {
                            if let Some(entry) = list.entries.get(list.sel).cloned() {
                                let intro = self.machine.intro_seen;
                                storage::load_entry(&mut self.machine, &entry);
                                self.loading = true;
                                self.close_save_screen();
                                // Хук загрузки сработает только когда Z80 будет
                                // исполняться. На станции надо снять нативный
                                // захват и отложенный вход, иначе эмулятор останется
                                // замороженным и заглушка не исчезнет.
                                self.close_all_native();
                                self.cancel_pending_station();
                                self.station_native = false;
                                self.station_emulator = false;
                                self.flight_emulator = false;
                                // Загрузка с заставки сразу показывает кадр Z80: хук
                                // отведёт игру на станцию, не дожидаясь `Space`.
                                if intro {
                                    self.splash_emulator = true;
                                }
                            }
                        }
                        KeyE if !repeat => {
                            if let Some(entry) = list.entries.get(list.sel).cloned() {
                                list.status = match storage::export_entry_dialog(&entry) {
                                    Ok(Some(_)) => "EXPORTED".to_string(),
                                    Ok(None) => "EXPORT CANCELLED".to_string(),
                                    Err(_) => "ERROR: EXPORT FAILED".to_string(),
                                };
                            }
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                        KeyI if !repeat => {
                            list.status = match storage::import_entry_dialog() {
                                Ok(Some(entry)) => match storage::read_entries() {
                                    Ok(mut entries) => {
                                        storage::sorted_for_table(&mut entries);
                                        list.sel = entries
                                            .iter()
                                            .position(|e| {
                                                e.saved_at == entry.saved_at && e.name == entry.name
                                            })
                                            .unwrap_or(0);
                                        list.entries = entries;
                                        "IMPORTED".to_string()
                                    }
                                    Err(_) => "ERROR: CANNOT READ CATALOG".to_string(),
                                },
                                Ok(None) => "IMPORT CANCELLED".to_string(),
                                Err(_) => "ERROR: IMPORT FAILED".to_string(),
                            };
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                        Delete | Backspace if !repeat => {
                            if list.entries.get(list.sel).is_some() {
                                list.confirm_delete = true;
                                list.status = "DELETE SAVE? Y/N".to_string();
                            }
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                        _ => {
                            self.save_screen = Some(storage::SaveScreen::Load(list));
                        }
                    }
                }
            }
            None => {}
        }
    }

    /// Снять нажатие из матрицы и джойстика: клавиша не должна уйти в
    /// эмулятор, если её обработал нативный экран.
    pub(super) fn release_pressed(&mut self, code: KeyCode) {
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
        // анимацию и игра не реагирует на нажатие кнопок. Во время отсчёта
        // ввод живёт: клавиши экранов доходят до нативного слоя, а игре они
        // всё равно не мешают — оригинал их в это время игнорирует.
        if flight::z80_in_transition_animation(&self.machine.mem) || self.flight.in_animation_mode() {
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
        // В режиме анимации игра не реагирует на нажатие кнопок; во время
        // отсчёта гиперпрыжка ввод продолжает работать.
        if flight::z80_in_transition_animation(&self.machine.mem) || self.flight.in_animation_mode() {
            self.pending_key = None;
            return;
        }
        // Под картой нажатия принадлежат ей: стрелки, Enter, D/B/R и прочие
        // клавиши управления не должны долетать до Z80 и переключать его
        // экраны. Исключение — одна отложенная клавиша (отсчёт G/H или `L` при
        // входе в нативный режим станции), которую карта передаёт сама.
        if self.hdmap.active {
            if let Some((row, bit)) = self.pending_key.take() {
                self.machine.set_key(row, bit, true);
            }
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
            && !self.inventory.is_active()
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
