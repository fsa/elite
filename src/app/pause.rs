//! Пауза Кладова: сервисные клавиши меняют настройки игры, а Z80 в это время
//! не исполняется.
//!
//! Оригинал обрабатывает эти клавиши в `ScanKeyInPause`, не выходя из игрового
//! цикла. Здесь то же самое делается прямо над памятью Z80: так экран эмулятора
//! остаётся неподвижным, но после выхода из паузы игра видит те же флаги, что
//! после настоящей паузы Кладова. Вид радара считывается из того же байта,
//! который меняет `L`, через [`flight::z80_radar_view`].

use winit::keyboard::KeyCode;

use super::App;
use crate::cockpit;
use crate::flight;

/// Бит `ControlKey2`, который включает паузу в оригинале.
const PAUSE_BIT: u8 = 0x80;

impl App<'_> {
    /// Пауза имеет смысл только в самой игре: на заставке, в редакторе сейва
    /// и в собственном меню `Esc` она не нужна.
    pub(super) fn can_enter_pause(&self) -> bool {
        !self.machine.intro_seen && !self.save_editor_open && !self.menu
    }

    /// Открыть паузу: запомнить её в памяти Z80 и убрать весь ввод, чтобы
    /// зажатые до паузы клавиши не выстрелили после выхода из неё.
    pub(super) fn enter_pause(&mut self) {
        if self.pause_active {
            return;
        }
        self.pause_active = true;
        if let Some(flag) = self.machine.mem.get_mut(flight::Z80_CONTROL_KEY2) {
            *flag |= PAUSE_BIT;
        }
        if let Some(symbol) = self.machine.mem.get_mut(flight::Z80_PRESS_KEY_SYMBOL) {
            *symbol = 0xFF;
        }
        self.clear_pause_input();
        self.sfx.play(crate::sfx::Sfx::PauseKey);
    }

    /// Закрыть паузу. Кладовая `Space` обнуляет блок сервисных клавиш; делать
    /// это здесь нужно явно, потому что Z80 во время паузы не работает.
    pub(super) fn exit_pause(&mut self) {
        if !self.pause_active {
            return;
        }
        self.clear_control_keys();
        self.pause_active = false;
        self.clear_pause_input();
    }

    /// Обработать нажатие в паузе. `Space` закрывает паузу, остальные клавиши
    /// меняют настройки так же, как таблицы `ScanKeyInPause` из Кладова.
    pub(super) fn handle_pause_key(&mut self, code: KeyCode, repeat: bool) {
        if !repeat {
            let handled = matches!(
                code,
                KeyCode::Space
                    | KeyCode::KeyB
                    | KeyCode::KeyD
                    | KeyCode::KeyF
                    | KeyCode::KeyR
                    | KeyCode::KeyY
                    | KeyCode::KeyS
                    | KeyCode::KeyL
                    | KeyCode::KeyA
                    | KeyCode::KeyK
                    | KeyCode::KeyC
                    | KeyCode::KeyQ
            );
            if handled {
                self.sfx.play(crate::sfx::Sfx::PauseKey);
            }
            match code {
                KeyCode::Space => self.exit_pause(),
                // Первая таблица `unk_72FB`: XOR бита в `FlagsSetUp`.
                KeyCode::KeyB => self.xor_flags(0x10),
                KeyCode::KeyD => self.xor_flags(0x08),
                KeyCode::KeyF => self.xor_flags(0x40),
                KeyCode::KeyR => self.xor_flags(0x04),
                KeyCode::KeyY => self.xor_flags(0x20),
                KeyCode::KeyS => self.xor_flags(0x80),
                // Вторая таблица: отдельные действия по адресам 0x73xx.
                KeyCode::KeyL => self.toggle_locator(),
                KeyCode::KeyA => self.swap_lasers(),
                KeyCode::KeyK => self.rotate_map(),
                KeyCode::KeyC => self.toggle_compass(),
                KeyCode::KeyQ => self.set_control_keys_from_flags(),
                _ => {}
            }
        }
        // `L` меняет байт в памяти Z80; подхватываем новый вид сразу, чтобы
        // он был виден без ожидания следующего кадра.
        self.radar_view = flight::z80_radar_view(&self.machine.mem);
        self.release_pressed(code);
    }

    /// В паузе ни одна клавиша не должна уходить в замороженный эмулятор.
    fn clear_pause_input(&mut self) {
        self.matrix = [[false; 5]; 8];
        self.kempston = [false; 5];
        self.pending_key = None;
        self.machine.release_all_keys();
        self.machine.release_joystick();
    }

    /// Кладовая `Space`: `ControlKey1..ControlKeyInMap` обнуляются, а код
    /// последней клавиши возвращается в исходное `0xFF`.
    fn clear_control_keys(&mut self) {
        self.write_control_keys(0);
    }

    /// Кладовая `Q` (`loc_731A`): процедура вызывается с `A = FlagsSetUp` и
    /// записывает его во все ячейки блока сервисных клавиш.
    fn set_control_keys_from_flags(&mut self) {
        let flags = self
            .machine
            .mem
            .get(flight::Z80_FLAGS_SETUP)
            .copied()
            .unwrap_or(0);
        self.write_control_keys(flags);
    }

    fn write_control_keys(&mut self, value: u8) {
        for addr in flight::Z80_CONTROL_KEY1..=flight::Z80_CONTROL_KEY2 + 3 {
            if let Some(cell) = self.machine.mem.get_mut(addr) {
                *cell = value;
            }
        }
        if let Some(symbol) = self.machine.mem.get_mut(flight::Z80_PRESS_KEY_SYMBOL) {
            *symbol = 0xFF;
        }
    }

    fn xor_flags(&mut self, mask: u8) {
        if let Some(flags) = self.machine.mem.get_mut(flight::Z80_FLAGS_SETUP) {
            *flags ^= mask;
        }
    }

    /// Клавиша `L` повторяет `sub_7A84`: переключает immediate в
    /// `CopyLocator2Scr` и готовит копирование локатора на экран.
    fn toggle_locator(&mut self) {
        if let Some(cell) = self.machine.mem.get_mut(flight::Z80_SCANNER_INVERT) {
            *cell ^= 0x90;
        }
        if let Some(cell) = self.machine.mem.get_mut(flight::Z80_SCANNER_MODE) {
            *cell ^= 0x07;
        }
        let on_cockpit = self
            .machine
            .mem
            .get(flight::Z80_WHAT_ON_SCREEN)
            .copied()
            .unwrap_or(0xFF)
            < 4;
        if on_cockpit {
            if let Some(cell) = self.machine.mem.get_mut(flight::Z80_FLG_SAVE_SCANNER) {
                *cell = 1;
            }
        }
    }

    /// Клавиша `A` поворачивает четвёрку лазеров Cobra Mk IV на два крепления
    /// (`loc_72E3`): FRONT/REAR меняются местами с LEFT/RIGHT.
    fn swap_lasers(&mut self) {
        let base = flight::Z80_LASER_MOUNTS;
        if self.machine.mem.get(flight::Z80_COBRA_MK4).copied() == Some(0)
            || self.machine.mem.len() <= base + 3
        {
            return;
        }
        let front = self.machine.mem[base];
        let rear = self.machine.mem[base + 1];
        let left = self.machine.mem[base + 2];
        let right = self.machine.mem[base + 3];
        self.machine.mem[base] = left;
        self.machine.mem[base + 1] = right;
        self.machine.mem[base + 2] = front;
        self.machine.mem[base + 3] = rear;
    }

    /// Клавиша `K` меняет поворот станины лазеров на карте (`loc_7328`).
    /// Оригинал XOR-ит ячейку полным `FlagsSetUp`, но только если её младшие
    /// два бита уже ненулевые; при нуле действие пропускается.
    fn rotate_map(&mut self) {
        let flags = self
            .machine
            .mem
            .get(flight::Z80_FLAGS_SETUP)
            .copied()
            .unwrap_or(0);
        if let Some(cell) = self.machine.mem.get_mut(flight::Z80_MAP_ROTATION) {
            if *cell & 3 != 0 {
                *cell ^= flags;
            }
        }
    }

    /// Клавиша `C` переключает бит `1` ячейки `0xFD0A`, которую читает
    /// оригинальный компас (`loc_7339` вызывает `sub_DFEE`). В нативном слое
    /// режим меняется сразу, чтобы изменение было видно без кадра Z80.
    fn toggle_compass(&mut self) {
        if let Some(cell) = self.machine.mem.get_mut(flight::Z80_COMPASS_FLAG) {
            *cell ^= 2;
        }
        self.compass_view = match self.compass_view {
            cockpit::CompassView::PlanetStation => cockpit::CompassView::PlanetSun,
            cockpit::CompassView::PlanetSun => cockpit::CompassView::PlanetStation,
        };
    }
}
