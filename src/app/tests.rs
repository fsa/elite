//! Тесты оконного слоя: нативные экраны, ввод, заставка и отрисовка.
//!
//! Живут внутри модуля `app`, поэтому видят приватные поля [`App`] и его
//! методы; общие подсистемы импортируются явно.

use std::env;
use std::fs;

use winit::keyboard::KeyCode;

use super::*;
use crate::boot::{
    KLADOV_I, KLADOV_LOAD_ADDR, KLADOV_STATE_A, KLADOV_STATE_BC, KLADOV_STATE_DE,
    KLADOV_STATE_F, KLADOV_STATE_HL, KLADOV_STATE_IX, KLADOV_STATE_IY, KLADOV_STATE_PC,
    KLADOV_STATE_R, KLADOV_STATE_SP,
};
use crate::cli::Options;
use crate::cockpit;
use crate::display;
use crate::equip;
use crate::flight;
use crate::font::ZX_FONT;
use crate::galaxy;
use crate::headless::write_ppm;
use crate::keys::{MATRIX_MAP, kempston_bit, matrix_pos};
use crate::machine::Spectrum;
use crate::native;
use crate::splash;
use crate::ui::menu::draw_pause_menu;
use crate::ui::present::{SRC_H, SRC_W, blit_zx_frame};
use crate::world::{current_planet_color, save_layout_kladov, scene_markers};


    /// Boot the real game image with the real ROM, as `main` does.
    fn boot() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
        let mut m = Spectrum::new(Some(&rom));
        m.load(KLADOV_LOAD_ADDR, &game);
        m.cpu.a = KLADOV_STATE_A;
        m.cpu.f = KLADOV_STATE_F;
        m.cpu.b = (KLADOV_STATE_BC >> 8) as u8;
        m.cpu.c = (KLADOV_STATE_BC & 0xFF) as u8;
        m.cpu.d = (KLADOV_STATE_DE >> 8) as u8;
        m.cpu.e = (KLADOV_STATE_DE & 0xFF) as u8;
        m.cpu.h = (KLADOV_STATE_HL >> 8) as u8;
        m.cpu.l = (KLADOV_STATE_HL & 0xFF) as u8;
        m.cpu.ix = KLADOV_STATE_IX;
        m.cpu.iy = KLADOV_STATE_IY;
        m.cpu.sp = KLADOV_STATE_SP;
        m.cpu.pc = KLADOV_STATE_PC;
        m.cpu.i = KLADOV_I;
        m.cpu.r = KLADOV_STATE_R;
        m.cpu.im = 2;
        m.cpu.iff1 = true;
        m.cpu.iff2 = true;
        native::install_save(&mut m, native::KLADOV_SAVE);
        m
    }

    /// The intro used to crash: a wrong `CP` corrupted the line-drawing clip
    /// logic, the drawer walked out of the screen buffer and self-modified
    /// code at 0xBAF7, after which the message printer recursed and leaked SP.
    #[test]
    fn intro_runs_without_corruption_or_leak() {
        let mut m = boot_kladov();
        m.counts.push((0x739E, 0)); // intro loop
        for _ in 0..60 {
            m.run_frame();
        }
        assert!(m.cpu.pc != 0, "CPU ran off the rails (pc=0)");
        assert!(m.cpu.sp >= 0x4000, "stack leaked: sp=0x{:04X}", m.cpu.sp);
    }

    /// Меню паузы (Esc) рисуется в HD-слой поверх кадра: рамка и текст
    /// шрифтом ПЗУ в полном разрешении (масштаб x5 для слоя 1280x960).
    /// `ELITE_DUMP_MENU=<file>` сохраняет слой в PPM для визуальной проверки.
    #[test]
    fn pause_menu_overlay_renders() {
        let m = boot();
        let mut hd = display::Frame::new(256 * 5, 192 * 5);
        let font = ZX_FONT;
        draw_pause_menu(&mut hd, font, 0, false);

        const WHITE: u32 = 0xFFD7_D7D7;
        // Угол рамки панели (ZX 40,52 -> HD 200,260) — белый.
        assert_eq!(
            hd.pixels()[260 * 1280 + 200],
            WHITE,
            "panel border not drawn"
        );
        // Внутри панели нарисован текст: заметное число белых пикселей.
        let white = hd.pixels().iter().filter(|&&p| p == WHITE).count();
        assert!(white > 5000, "menu text too sparse: white={white}");
        // Панель непрозрачная (старший байт != 0), фон слоя — прозрачный.
        assert_eq!(
            hd.pixels()[261 * 1280 + 201],
            0xFF00_0000,
            "panel bg not opaque"
        );
        assert_eq!(
            hd.pixels()[0],
            display::TRANSPARENT,
            "outside panel must stay transparent"
        );

        // Композитинг поверх масштабированного ZX-кадра действительно меняет картинку.
        let mut zx = vec![0u32; 256 * 192];
        m.render_rgba(&mut zx);
        let mut out = vec![0u32; 1280 * 960];
        for y in 0..960 {
            for x in 0..1280 {
                out[y * 1280 + x] = zx[(y / 5) * 256 + x / 5];
            }
        }
        let before = out.clone();
        hd.composite_onto(&mut out, 1280, 960, 0, 0);
        assert!(out != before, "overlay did not change the frame");
        assert_eq!(out[260 * 1280 + 200], WHITE, "overlay not composited");

        if let Ok(path) = env::var("ELITE_DUMP_MENU") {
            write_ppm(&path, &out, 1280, 960);
        }
    }

    /// Физические клавиши раскладываются по матрице ZX Spectrum без зависимости
    /// от активной раскладки: (полуряд, бит) совпадают с оригинальной схемой.
    #[test]
    fn matrix_map_covers_keyboard() {
        assert_eq!(matrix_pos(KeyCode::ShiftLeft), Some((0, 0)));
        assert_eq!(matrix_pos(KeyCode::KeyQ), Some((2, 0)));
        assert_eq!(matrix_pos(KeyCode::Digit1), Some((3, 0)));
        assert_eq!(matrix_pos(KeyCode::Enter), Some((6, 0)));
        assert_eq!(matrix_pos(KeyCode::Space), Some((7, 0)));
        assert_eq!(matrix_pos(KeyCode::KeyB), Some((7, 4)));
        // Вне матрицы (управление игрой) — ничего.
        assert_eq!(matrix_pos(KeyCode::ArrowUp), None);
        // F5/F9 removed from UI
        assert!(matrix_pos(KeyCode::F5).is_none());
        // Все 40 клавиш матрицы заданы и уникальны.
        assert_eq!(MATRIX_MAP.len(), 40);
        let mut seen = std::collections::HashSet::new();
        for &(code, row, bit) in MATRIX_MAP {
            assert!(
                row < 8 && bit < 5,
                "позиция вне матрицы: {code:?} ({row},{bit})"
            );
            assert!(seen.insert((row, bit)), "дубликат позиции {row},{bit}");
        }
        // Kempston: стрелки и правая Ctrl.
        assert_eq!(kempston_bit(KeyCode::ArrowRight), Some(0));
        assert_eq!(kempston_bit(KeyCode::ControlRight), Some(4));
        assert_eq!(kempston_bit(KeyCode::Space), None);
    }

    fn test_options() -> Options {
        Options {
            game: String::new(),
            rom: None,
            headless: true,
            frames: 0,
            dump: None,
            mute: true,
            audio: false,
            wav: None,
            native: false,
            save: None,
            fullscreen: false,
        }
    }

    /// В игре показывается полный ZX-кадр: нижние 64 строки (родная растровая
    /// панель оригинала) не обрезаются — нативная панель `src/cockpit.rs` в
    /// игре не рисуется, она осталась только на заставке.
    #[test]
    fn game_screen_shows_full_zx_frame() {
        let (w, h) = (1024usize, 768usize);
        let l = display::layout(w as u32, h as u32);
        assert_eq!(l.scale, 4);
        // Кадр: номер строки ZX записан в младший байт цвета.
        let mut src = vec![0u32; SRC_W * SRC_H];
        for y in 0..SRC_H {
            for x in 0..SRC_W {
                src[y * SRC_W + x] = 0xFF00_0000 | y as u32;
            }
        }
        let mut out = vec![0u32; w * h];
        blit_zx_frame(&src, &mut out, w, h, &l);
        // Верхняя строка кадра — на месте.
        assert_eq!(out[l.y as usize * w + l.x as usize] & 0xFF, 0);
        // Нижняя строка ZX-кадра (191) тоже попала в окно: родная панель
        // оригинала на месте, кадр не обрезан.
        let last = l.y as usize + (SRC_H - 1) * l.scale;
        assert!(last + l.scale <= h, "нижние строки кадра обрезаны");
        assert_eq!(out[last * w + l.x as usize] & 0xFF, (SRC_H - 1) as u32);
    }

    /// F10 открывает отдельную нативную карту галактики: игра не встаёт, но
    /// переходит в режим `SHORT RANGE CHART`, ввод уходит карте, повторный
    /// F10/Esc возвращает экран игры.
    #[test]
    fn f10_opens_and_closes_galaxy_map() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        let prev_screen = app.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        assert!(!app.hdmap.active);
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active, "F10 не открыл карту");
        assert_eq!(app.hdmap.planet_count(), galaxy::COUNT);
        // Пока карта открыта, игра считает, что на экране локальная карта.
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_LOCAL_MAP
        );
        // Курсор карты стартует на текущей системе (из памяти игры), и эта
        // точка сразу передана в эмулятор как цель карты.
        let sys_x = app.machine.mem[galaxy::KLADOV_GALAXY.sys_x] as i32;
        let sys_y = app.machine.mem[galaxy::KLADOV_GALAXY.sys_y] as i32;
        assert_eq!(app.hdmap.cursor(), (sys_x, sys_y));
        let live = save_layout_kladov().pilot_name as usize;
        assert_eq!(app.machine.mem[live + native::CRD_X_TARGET_OFF], sys_x as u8);
        assert_eq!(app.machine.mem[live + native::CRD_Y_TARGET_OFF], sys_y as u8);
        // Стрелка двигает курсор карты, а не корабль, и обновляет цель.
        app.key_event(KeyCode::ArrowRight, true, false);
        let moved_x = (sys_x + 4).min(255);
        assert_eq!(app.hdmap.cursor().0, moved_x);
        assert_eq!(app.machine.mem[live + native::CRD_X_TARGET_OFF], moved_x as u8);
        // `D` ставит курсор точно на ближайшую звезду.
        app.key_event(KeyCode::KeyD, true, false);
        let map_x = app.hdmap.cursor().0;
        let map_y = app.hdmap.cursor().1;
        assert_eq!(app.machine.mem[live + native::CRD_X_TARGET_OFF], map_x as u8);
        assert_eq!(app.machine.mem[live + native::CRD_Y_TARGET_OFF], map_y as u8);
        // Пока карта открыта, ввод не попадает в эмулятор.
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "ввод не должен уходить в игру");
        // Повторное F10 закрывает карту и возвращает прежний экран.
        app.key_event(KeyCode::F10, true, false);
        assert!(!app.hdmap.active, "F10 не закрыл карту");
        assert_eq!(app.machine.mem[flight::Z80_WHAT_ON_SCREEN], prev_screen);
        // Открыть заново и закрыть по Esc.
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.hdmap.active, "Esc не закрыл карту");
        assert_eq!(app.machine.mem[flight::Z80_WHAT_ON_SCREEN], prev_screen);
    }

    /// F12 переключает ускорение эмуляции Z80 вдвое. Автоповтор не должен
    /// мигать режимом, а за кадр показа при ускорении идёт два кадра Z80.
    #[test]
    fn f12_toggles_double_speed() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.accelerated);
        assert_eq!(app.emulation_frames(), 1);

        app.key_event(KeyCode::F12, true, false);
        assert!(app.accelerated, "F12 не включил ускорение");
        assert_eq!(app.emulation_frames(), 2);

        // Автоповтор удерживаемой клавиши режим не переключает.
        app.key_event(KeyCode::F12, true, true);
        assert!(app.accelerated, "автоповтор F12 сбросил ускорение");

        app.key_event(KeyCode::F12, false, false);
        app.key_event(KeyCode::F12, true, false);
        assert!(!app.accelerated, "F12 не выключил ускорение");
        assert_eq!(app.emulation_frames(), 1);
    }

    /// F4 — оригинальная клавиша 4 плюс нативный экран закупки: сначала
    /// эмулятору уходит клавиша 4 (игра переходит в equip ship), затем поверх
    /// открывается закупка. Esc/F4 возвращают экран игры.
    #[test]
    fn f4_opens_and_closes_equip_shop() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.shop.active);
        app.key_event(KeyCode::F4, true, false);
        // Закупка отложена: сначала кадр игры с клавишей 4.
        assert!(!app.shop.active, "F4 открыл закупку до перехода игры");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit4).expect("4 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F4 не передал игре клавишу 4"
        );
        app.machine.run_frame();
        app.open_pending_shop();
        assert!(app.shop.active, "F4 не открыл экран закупки");
        assert!(!app.hdmap.active, "карта галактики не должна открыться");
        // Экран снят данные живого командира.
        let p = equip::KLADOV_SHOP.pilot as usize;
        let cash = u16::from_le_bytes([app.machine.mem[p + 18], app.machine.mem[p + 19]]) as u32
            * 10000
            + u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]) as u32;
        assert_eq!(app.shop.cash_tenths(), cash, "деньги не из памяти игры");
        // Пока экран открыт, ввод не попадает в эмулятор.
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "ввод не должен уходить в игру");
        // Стрелка двигает выбор списка, а не корабль.
        app.key_event(KeyCode::ArrowDown, true, false);
        // Esc закрывает экран (окна крепления не было).
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.shop.active, "Esc не закрыл экран закупки");
        // Повторное открытие (с тем же кадром игры) и закрытие по F4.
        app.key_event(KeyCode::F4, true, false);
        app.apply_input();
        app.machine.run_frame();
        app.open_pending_shop();
        assert!(app.shop.active);
        app.key_event(KeyCode::F4, true, false);
        assert!(!app.shop.active, "F4 не закрыл экран закупки");
    }

    /// F1 включает нативный экран полёта и одновременно отдаёт игре клавишу 1.
    /// F1-4 под полётом — это клавиши 1-4 оригинала (виды), а `1` возвращает
    /// ZX-кадр; Esc открывает меню паузы.
    #[test]
    fn f1_opens_and_closes_flight() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.flight.active);
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active, "F1 не открыл экран полёта");
        assert!(!app.hdmap.active && !app.shop.active);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 не передал игре клавишу 1"
        );

        // Под экраном продолжает работать Z80, поэтому ввод должен доходить
        // до эмулятора.
        app.key_event(KeyCode::KeyS, true, false);
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[1] & 0x02,
            0,
            "ввод не дошёл до эмулятора под экраном полёта"
        );
        app.key_event(KeyCode::KeyS, false, false);

        // F2-F4 — виды назад/влево/вправо и клавиши 2-4 оригинала.
        app.key_event(KeyCode::F2, true, false);
        assert_eq!(app.flight.view_dir, flight::ViewDir::Back);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit2).expect("2 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F2 не передал игре клавишу 2"
        );
        // Поверх полёта карта тоже открывается: полёт остаётся под ней и
        // продолжается после закрытия карты.
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active, "F10 не открыл карту поверх полёта");
        app.key_event(KeyCode::F10, true, false);
        assert!(!app.hdmap.active, "F10 не закрыл карту");
        assert!(app.flight.active, "закрытие карты выключило полёт");

        // Esc — пауза, но экран остаётся выбранным; Esc снова — продолжить.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu && app.flight.active, "Esc не открыл меню");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu && app.flight.active, "Esc не вернул в полёт");

        // Клавиша 1 — назад к Z80-кадру.
        app.key_event(KeyCode::Digit1, true, false);
        assert!(!app.flight.active, "1 не закрыла экран полёта");

        // F1 снова включает экран полёта (и остаётся включённым).
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        app.key_event(KeyCode::F1, true, false);
        assert!(
            app.flight.active,
            "F1 должен включать рендер, а не закрывать"
        );
    }

    /// F1-F4 на станции повторяют оригинальные клавиши 1-4: игра получает
    /// ту же матричную клавишу (1 — вылет, 2 — покупка, 3 — продажа,
    /// 4 — оборудование).
    #[test]
    fn function_keys_send_original_number_keys() {
        for (fkey, digit) in [
            (KeyCode::F1, KeyCode::Digit1),
            (KeyCode::F2, KeyCode::Digit2),
            (KeyCode::F3, KeyCode::Digit3),
            (KeyCode::F4, KeyCode::Digit4),
        ] {
            let opts = test_options();
            let mut app = App::new(boot(), &opts);
            app.key_event(fkey, true, false);
            app.apply_input();
            let (row, bit) = matrix_pos(digit).expect("цифра в матрице");
            assert_eq!(
                app.machine.keyboard[row] & (1 << bit),
                0,
                "{fkey:?} не передал игре {digit:?}"
            );
        }
    }

    /// F1-F4 на нативной карте тоже работают как клавиши 1-4: карта
    /// закрывается, нажатие уходит игре; F1 включает нативный вид полёта.
    #[test]
    fn map_function_keys_forward_original_keys() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.hdmap.active, "F1 не закрыл карту");
        assert!(app.flight.active, "F1 не включил нативный вид полёта");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 на карте не передал игре клавишу 1"
        );

        // F2-F4 закрывают карту и передают свои цифры.
        for (fkey, digit) in [
            (KeyCode::F2, KeyCode::Digit2),
            (KeyCode::F3, KeyCode::Digit3),
            (KeyCode::F4, KeyCode::Digit4),
        ] {
            let mut app = App::new(boot(), &opts);
            app.key_event(KeyCode::F10, true, false);
            assert!(app.hdmap.active);
            app.key_event(fkey, true, false);
            assert!(!app.hdmap.active, "{fkey:?} не закрыл карту");
            assert!(!app.flight.active, "{fkey:?} не должен включать полёт");
            app.apply_input();
            let (row, bit) = matrix_pos(digit).expect("цифра в матрице");
            assert_eq!(
                app.machine.keyboard[row] & (1 << bit),
                0,
                "{fkey:?} на карте не передал игре {digit:?}"
            );
        }
    }

    /// Когда Z80 уходит с вида из кабины (карта по `O`, инвентарь по `Enter`
    /// и т.п.), нативный вид закрывается и показывается настоящий экран игры.
    #[test]
    fn flight_closes_when_z80_leaves_cockpit_view() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        // Ставим игру «в полёт»: на станции `F1` — это команда вылета, и
        // нативный вид включается только после её анимации.
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        // Экран станции до первого полётного вида не закрывает нативный вид:
        // так идёт анимация вылета после запуска.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.follow_flight_screen();
        assert!(app.flight.active, "станция закрыла вид во время вылета");
        // Любой другой неполётный экран закрывает сразу, даже если полётный
        // вид ещё не успел появиться.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 4;
        app.follow_flight_screen();
        assert!(
            !app.flight.active,
            "меню покупки осталось под нативным видом"
        );

        // Первый полётный экран взводит «нативный вид уже видел полёт».
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_flight_screen();
        assert!(app.flight.active);
        // Карта/инвентарь оригинала — нативный вид больше не нужен.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 8;
        app.follow_flight_screen();
        assert!(!app.flight.active, "карта открылась под нативным видом");
    }

    /// После гибели игра уходит на заставку (`loc_739E`), снова выставляя
    /// `WhatOnScreen = 0`. До этого нативный вид должен оставаться на экране
    /// с надписью «Game Over», а на заставке — закрыться.
    #[test]
    fn flight_closes_when_game_returns_to_title() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);

        // Гибель: оригинал ещё держит вид из кабины (`WhatOnScreen = 0`).
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_PLAYER_GAME_OVER] = 1;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся во время Game Over");

        // Оригинал на заставке: `WhatOnScreen = 0`, но это уже не кабина.
        app.machine.intro_seen = true;
        app.follow_flight_screen();
        assert!(
            !app.flight.active,
            "нативный вид остался поверх заставки"
        );
    }

    /// `F1` на станции — родная команда вылета. Нативный вид включается уже
    /// во время перехода и проигрывает свою анимацию вылета.
    #[test]
    fn f1_at_station_plays_native_launch() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        // Станционный экран оригинала.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;

        app.key_event(KeyCode::F1, true, false);
        assert!(!app.flight.active, "F1 включил вид до начала вылета");
        assert!(app.pending_launch, "F1 на станции не запомнил вылет");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 на станции не передал игре клавишу 1"
        );

        // Идёт вылет: Z80 уже в полёте, но держит флаги перехода.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 4;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.open_pending_flight();
        assert!(app.flight.active, "F1 не открыл нативный вид на вылете");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Launch),
            "вылет показан не как вылет"
        );

        // Переход закончился — остаётся обычный вид вперёд.
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся после вылета");
        assert!(
            !app.flight.in_transition(),
            "анимация вылета не закончилась"
        );
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);
    }

    /// `F1` во время прыжка сразу открывает нативную анимацию гиперперехода,
    /// а не ждёт окончания ZX-ролика.
    #[test]
    fn f1_during_jump_plays_native_hyperspace() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0x15;

        app.key_event(KeyCode::F1, true, false);
        assert!(!app.flight.active, "F1 включил вид во время перехода");
        app.open_pending_flight();
        assert!(app.flight.active, "F1 не открыл нативный гиперпереход");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "прыжок показан не как гиперпереход"
        );

        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся после прыжка");
        assert!(!app.flight.in_transition(), "переход не закончился");
    }

    /// Возврат из Z80-экрана (инвентарь по `Enter`): `F1` должен сразу
    /// вернуть игру в кабину и включить нативный вид, а не требовать второго
    /// нажатия.
    #[test]
    fn f1_returns_from_z80_screen_in_one_press() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        // Игра в полёте, но открыт родной экран (например, инвентарь).
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 8;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        assert!(!app.flight.active);

        app.key_event(KeyCode::F1, true, false);
        assert!(!app.flight.active, "вид открылся поверх Z80-экрана");
        assert_eq!(
            app.pending_flight,
            Some(flight::ViewDir::Forward),
            "F1 не запомнил вид для возврата"
        );
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 не передал игре клавишу 1"
        );

        // Z80 обработал клавишу и снова показывает кабину.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.open_pending_flight();
        assert!(
            app.flight.active,
            "первый F1 не вернул нативный вид без повторного нажатия"
        );
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);
    }

    /// Если прыжок начался при открытом нативном виде, рендер не прячется, а
    /// сам показывает нативную анимацию — с тем же направлением вида.
    #[test]
    fn flight_stays_active_during_jump_transition() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;

        app.key_event(KeyCode::F4, true, false);
        assert!(app.flight.active);
        assert_eq!(app.flight.view_dir, flight::ViewDir::Right);

        // Начался прыжок: оригинал держит флаги перехода.
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0x15;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся на время прыжка");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "прыжок показан не как гиперпереход"
        );
        assert_eq!(app.flight.view_dir, flight::ViewDir::Right);

        // Переключение вида во время перехода не сбрасывает анимацию.
        app.key_event(KeyCode::F3, true, false);
        assert_eq!(app.flight.view_dir, flight::ViewDir::Left);
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "смена вида сбросила переход"
        );

        // Прыжок закончился — нативный вид остаётся с тем же направлением.
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся после прыжка");
        assert!(!app.flight.in_transition(), "переход не закончился");
        assert_eq!(app.flight.view_dir, flight::ViewDir::Left);
    }

    /// Стыковка при открытом нативном виде: рендер показывает тоннель, а
    /// после посадки закрывается и не оставляет запроса на возврат.
    #[test]
    fn flight_request_cleared_after_docking() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;

        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        // Даём рендеру увидеть настоящий полётный экран.
        app.follow_flight_screen();

        // Стыковка: тоннель, затем станционный экран.
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся на время стыковки");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Docking),
            "стыковка показана не как стыковка"
        );

        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.follow_flight_screen();
        app.open_pending_flight();
        assert!(!app.flight.active, "вид включился на станции");
        assert!(app.pending_flight.is_none(), "запрос вида пережил стыковку");
    }

    /// Реальная игра: `F1` на станции запускает вылет, нативный рендер
    /// включается уже во время перехода и показывает свою анимацию вылета.
    #[test]
    fn real_launch_shows_native_transition() {
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        native::start_game(&mut m, save_layout_kladov());
        // Даём игре дойти до станции.
        for _ in 0..1500 {
            m.run_frame();
        }
        assert_eq!(
            m.mem[flight::Z80_IND_DANGER],
            0,
            "игра не оказалась на станции"
        );
        assert_eq!(
            m.mem[flight::Z80_WHAT_ON_SCREEN],
            0x0B,
            "игра не на станционном экране"
        );

        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.flight.active, "F1 включил вид до вылета");
        app.apply_input();

        // Ждём, пока оригинал начнёт переход вылета: нативный вид должен
        // включиться сразу с анимацией, не дожидаясь её конца.
        let mut frames = 0;
        while frames < 1500 && !app.flight.active {
            app.machine.run_frame();
            app.open_pending_flight();
            frames += 1;
        }
        assert!(app.flight.active, "F1 не включил вид во время вылета");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Launch),
            "вылет показан не как вылет"
        );
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);

        // Переход заканчивается — остаётся обычный нативный полёт.
        let mut frames = 0;
        while frames < 1500 && app.flight.in_transition() {
            app.machine.run_frame();
            app.follow_flight_screen();
            frames += 1;
        }
        assert!(
            !app.flight.in_transition(),
            "анимация вылета не закончилась"
        );
        assert!(app.flight.active, "вид закрылся после вылета");
    }

    /// В настоящем полёте `F4` — это оригинальный вид вправо, а не закупка.
    #[test]
    fn f4_in_flight_does_not_open_shop() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;

        app.key_event(KeyCode::F4, true, false);
        assert!(app.flight.active, "F4 не включил нативный вид");
        assert!(!app.shop.active, "F4 открыл закупку в полёте");
        assert_eq!(app.flight.view_dir, flight::ViewDir::Right);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit4).expect("4 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F4 в полёте не передал игре клавишу 4"
        );
    }

    /// `F2`-`F4` в полёте переключают нативные виды назад/влево/вправо.
    #[test]
    fn f2_f4_in_flight_switch_views() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;

        for (fkey, view) in [
            (KeyCode::F1, flight::ViewDir::Forward),
            (KeyCode::F2, flight::ViewDir::Back),
            (KeyCode::F3, flight::ViewDir::Left),
            (KeyCode::F4, flight::ViewDir::Right),
        ] {
            app.key_event(fkey, true, false);
            assert!(app.flight.active, "{fkey:?} не открыл нативный вид");
            assert_eq!(app.flight.view_dir, view, "{fkey:?}: не тот вид");
            assert!(!app.shop.active, "{fkey:?} открыл закупку");
        }
    }

    /// Под экраном полёта эмулятор продолжает работать, поэтому ввод
    /// обязательно должен доходить до Z80-игры (иначе полёт «зависает»).
    #[test]
    fn flight_forwards_input_to_emulator() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        // S — тангаж вверх в матрице (полуряд 1, бит 1).
        app.key_event(KeyCode::KeyS, true, false);
        app.forward_input();
        assert_eq!(
            app.machine.keyboard[1] & 0x02,
            0,
            "клавиша не дошла до эмулятора"
        );
        // Отпускание обязано снять клавишу.
        app.key_event(KeyCode::KeyS, false, false);
        app.forward_input();
        assert_eq!(app.machine.keyboard[1] & 0x02, 0x02);
        // Джойстик тоже передаётся.
        app.key_event(KeyCode::ArrowUp, true, false);
        app.forward_input();
        assert_eq!(app.machine.kempston & 0x08, 0x08, "Kempston не дошёл");
    }

    /// Цвет планеты для экрана полёта берётся из памяти игры.
    #[test]
    fn current_planet_color_reads_live_memory() {
        let m = boot();
        let color = current_planet_color(&m.mem);
        assert_eq!(color >> 24, 0xFF, "цвет планеты должен быть непрозрачным");
    }

    /// Загрузка версии Кладова: снапшот сразу после запуска — на заставке.
    fn boot_kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
        let mut m = Spectrum::new(Some(&rom));
        m.load(KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = KLADOV_STATE_A;
        c.f = KLADOV_STATE_F;
        c.b = (KLADOV_STATE_BC >> 8) as u8;
        c.c = (KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (KLADOV_STATE_DE >> 8) as u8;
        c.e = (KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (KLADOV_STATE_HL >> 8) as u8;
        c.l = (KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = KLADOV_STATE_IX;
        c.iy = KLADOV_STATE_IY;
        c.sp = KLADOV_STATE_SP;
        c.pc = KLADOV_STATE_PC;
        c.i = KLADOV_I;
        c.r = KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        native::install_save(&mut m, native::KLADOV_SAVE);
        m
    }

    fn kladov_options() -> Options {
        test_options()
    }

    /// На заставке нативные экраны не открываются: F10/F4 игнорируются, пока
    /// игра проигрывает интро.
    #[test]
    fn native_screens_blocked_on_intro() {
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(
            !app.hdmap.active,
            "F10 не должен открываться на заставке Кладова"
        );
        app.key_event(KeyCode::F4, true, false);
        assert!(
            !app.shop.active,
            "F4 не должен открываться на заставке Кладова"
        );
        app.key_event(KeyCode::F1, true, false);
        assert!(
            !app.flight.active,
            "F1 не должен открывать экран полёта на заставке Кладова"
        );
    }

    /// Сброс флага заставки по входу в `GameMain`: как только игра вошла в
    /// игровой цикл, нативные экраны снова доступны.
    #[test]
    fn native_screens_available_after_game_main() {
        // Кладова: интро взводит флаг, вход в `GameMain` его снимает.
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "сначала должны быть на заставке");
        m.note_pc(game.unwrap());
        assert!(!m.intro_seen, "вход в GameMain не снял флаг заставки");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(
            app.hdmap.active,
            "после старта игры F10 должен открывать карту"
        );
    }

    /// Проверка на живом образе Кладова: пока не нажат SPACE, флаг заставки
    /// держится; после старта игры (вход в `GameMain`) снимается.
    #[test]
    fn kladov_intro_clears_after_space() {
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "должны быть на заставке");
        m.set_key(7, 0, true); // SPACE
        let mut cleared_at = None;
        for f in 0..3000 {
            m.run_frame();
            if !m.intro_seen {
                cleared_at = Some(f);
                break;
            }
        }
        assert!(
            cleared_at.is_some(),
            "SPACE не запустил игру (флаг не снят)"
        );
    }

    /// Точная логика «залипающего» флага заставки.
    #[test]
    fn intro_flag_is_sticky() {
        let mut m = Spectrum::new(None);
        m.intro_range = Some((0x1000, 0x1010));
        m.game_marker = Some(0x2000);
        m.note_pc(0x0800);
        assert!(!m.intro_seen, "флаг не должен взводиться вне диапазонов");
        m.note_pc(0x1005);
        assert!(m.intro_seen, "адрес интро должен взводить флаг");
        m.note_pc(0x0800);
        assert!(m.intro_seen, "флаг заставки должен «залипать»");
        m.note_pc(0x2000);
        assert!(!m.intro_seen, "вход в GameMain должен снимать флаг");
        m.note_pc(0x1010);
        assert!(m.intro_seen, "верхняя граница диапазона включается");
        m.note_pc(0x1011);
        assert!(m.intro_seen, "за верхней границей флаг держится");
    }

    /// На заставке работает только SPACE: остальные клавиши (в т.ч. F10/F4)
    /// игнорируются и в эмулятор не уходят. SPACE запускает игру родным путём.
    #[test]
    fn intro_accepts_only_space() {
        let opts = test_options();
        let mut m = boot();
        let (intro, game) = scene_markers(false);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "интро не распознано");
        let mut app = App::new(m, &opts);
        // Лишние клавиши на заставке ничего не делают.
        app.key_event(KeyCode::F5, true, false);
        app.key_event(KeyCode::F9, true, false);
        app.key_event(KeyCode::F10, true, false);
        app.key_event(KeyCode::F4, true, false);
        app.key_event(KeyCode::Digit1, true, false);
        app.key_event(KeyCode::KeyO, true, false);
        assert!(!app.menu, "лишние клавиши не должны открывать меню");
        assert!(!app.hdmap.active && !app.shop.active);
        assert!(app.machine.intro_seen, "игра должна остаться на заставке");
        // Esc открывает меню паузы (в нём сохранить недоступно).
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc на заставке должен открывать меню");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "Esc должен закрывать меню");
        // SPACE запускает игру: хук переводит Z80 на ветку переинициализации,
        // она зовёт `InitGame`, печатает станцию и уходит в `GameMain`.
        native::start_game(&mut app.machine, save_layout_kladov());
        let mut cleared = false;
        for _ in 0..5000 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                cleared = true;
                break;
            }
        }
        assert!(cleared, "SPACE не запустил игру");
    }

    /// Нативная заставка рисуется в HD-слой во всё окно: непрозрачный фон,
    /// заголовок и подсказки, вращающаяся фигура. `ELITE_DUMP_SPLASH=<file>`
    /// сохраняет слой в PPM для визуальной проверки.
    #[test]
    fn splash_overlay_renders() {
        let mut hd = display::Frame::new(256 * 5, 192 * 5);
        let font = ZX_FONT;
        let st = splash::Splash::new();
        st.draw(&mut hd, font);
        // Фон полностью непрозрачный.
        assert_eq!(hd.pixels()[0], 0xFF00_0000, "фон заставки не непрозрачный");
        // Заголовок «---- E L I T E ----» и приглашение нарисованы
        // белым цветом; лишних надписей на заставке нет.
        let white = hd.pixels().iter().filter(|&&p| p == 0xFFD7_D7D7).count();
        let grey = hd.pixels().iter().filter(|&&p| p == 0xFF90_9090).count();
        assert!(white > 1300, "надписи заставки не нарисованы: {white}");
        assert_eq!(grey, 0, "на заставке остались лишние надписи: {grey}");
        if let Ok(path) = env::var("ELITE_DUMP_SPLASH") {
            write_ppm(&path, hd.pixels(), 256 * 5, 192 * 5);
        }
    }

    /// Нативный кокпит: панель рисуется под заставкой (пока игра не
    /// запущена). `ELITE_DUMP_COCKPIT=<file>` сохраняет заставку вместе с
    /// панелью в PPM для визуальной проверки.
    #[test]
    fn cockpit_overlay_renders() {
        let base = env!("CARGO_MANIFEST_DIR");
        let font = ZX_FONT;
        let (w, h) = (256 * 5, 192 * 5);
        let mut hd = display::Frame::new(w, h);
        let g = cockpit::panel_geometry(w as u32, h as u32);
        // Заставка в верхней области, кокпит — вокруг неё и внизу (как в
        // игровом цикле).
        splash::Splash::new().draw_in(&mut hd, font, &g);
        cockpit::Cockpit::new().draw(
            &mut hd,
            font,
            cockpit::CockpitStyle::COBRA_MK3,
            &cockpit::CockpitValues::for_splash(),
        );
        // Панель непрозрачна.
        for y in g.panel_y..h as i32 {
            for x in 0..w {
                assert_ne!(
                    hd.pixels()[y as usize * w + x],
                    0,
                    "прозрачный пиксель панели ({x},{y})"
                );
            }
        }
        if let Ok(path) = env::var("ELITE_DUMP_COCKPIT") {
            std::fs::create_dir_all(format!("{base}/tmp")).unwrap();
            write_ppm(&path, hd.pixels(), w, h);
        }
    }

    /// На заставке в меню паузы доступна только загрузка: сохранение отключено.
    #[test]
    fn intro_menu_allows_load_but_not_save() {
        let base = env!("CARGO_MANIFEST_DIR");
        let mut opts = test_options();
        opts.save = Some(format!("{base}/tmp/test_intro_load.zxelite"));
        // Создаём минимальный файл для загрузки.
        std::fs::create_dir_all(format!("{base}/tmp")).unwrap();
        std::fs::write(&opts.save.as_ref().unwrap(), vec![0u8; 0x68]).unwrap();

        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..120 {
            m.run_frame();
        }
        let mut app = App::new(m, &opts);
        app.machine.intro_seen = true;
        app.machine.save_requested = false;
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu);
        // Пункт «сохранить» (1) на заставке не срабатывает.
        app.menu_sel = 1;
        app.key_event(KeyCode::Enter, true, false);
        assert!(app.menu, "на заставке сохранение не должно закрывать меню");
        assert!(!app.machine.save_requested, "на заставке сохранять нельзя");
        // Пункт «загрузить» (2) работает: просит загрузку у игры.
        app.menu_sel = 2;
        app.key_event(KeyCode::Enter, true, false);
        assert!(!app.menu, "загрузка должна закрыть меню");
        assert!(app.machine.load_requested, "загрузка не запрошена");
    }

    /// Меню паузы в игре: стрелки двигают выбор, Enter по «сохранить»
    /// закрывает меню и просит игру сохраниться.
    #[test]
    fn pause_menu_selection_and_actions() {
        let base = env!("CARGO_MANIFEST_DIR");
        let mut opts = test_options();
        opts.save = Some(format!("{base}/tmp/test_menu_save.zxelite"));
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.save_requested = false;
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu);
        assert_eq!(app.menu_sel, 0);
        // Стрелка вниз двигает выбор, вверх — назад (не ниже нуля).
        app.key_event(KeyCode::ArrowDown, true, false);
        assert_eq!(app.menu_sel, 1);
        app.key_event(KeyCode::ArrowUp, true, false);
        app.key_event(KeyCode::ArrowUp, true, false);
        assert_eq!(app.menu_sel, 0, "выбор не должен уходить ниже нуля");
        // Цифра задаёт пункт напрямую, Enter по «сохранить» закрывает меню и
        // сохраняет блок командира.
        app.key_event(KeyCode::Digit2, true, false);
        assert_eq!(app.menu_sel, 1);
        app.key_event(KeyCode::Enter, true, false);
        assert!(!app.menu, "сохранение должно закрыть меню");
        assert!(
            std::path::Path::new(&opts.save.as_ref().unwrap()).exists(),
            "файл сохранения не создан"
        );
    }

    /// На заставке Кладова `SPACE` запускает игру: хук на точках цикла
    /// заставки уводит Z80 на ветку переинициализации и снимает `intro_seen`.
    #[test]
    fn kladov_intro_accepts_space() {
        let opts = test_options();
        let mut m = boot_kladov();
        let (intro, gs) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = gs;
        for _ in 0..120 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::Space, true, false);
        let mut started = false;
        for _ in 0..300 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                started = true;
                break;
            }
        }
        assert!(started, "SPACE не запустил игру Кладова");
    }

    /// Экран полёта: вид из кабины занимает окно до панели, ниже — нативный
    /// кокпит. Вместе они покрывают всё окно без прозрачных щелей.
    /// `ELITE_DUMP_FLIGHT_SCREEN=<file>` сохраняет кадр в PPM.
    #[test]
    fn flight_screen_combines_view_and_cockpit() {
        let base = env!("CARGO_MANIFEST_DIR");
        let m = boot();
        let font = ZX_FONT;
        let mut fl = flight::Flight::new();
        fl.open(current_planet_color(&m.mem));
        fl.sync_from_z80(&m.mem);
        let (w, h) = (1280usize, 960usize);
        let mut hd = display::Frame::new(w, h);
        let g = cockpit::panel_geometry(w as u32, h as u32);
        fl.draw(&mut hd, &g, font);
        // Выше панели вид непрозрачен, ниже — прозрачен (там панель).
        for y in 0..g.panel_y as usize {
            assert!(
                hd.pixels()[y * w..(y + 1) * w]
                    .iter()
                    .all(|&p| p >> 24 == 0xFF),
                "вид из кабины не покрывает строку {y}"
            );
        }
        for y in g.panel_y as usize..h {
            assert!(
                hd.pixels()[y * w..(y + 1) * w].iter().all(|&p| p == 0),
                "вид залез на панель в строке {y}"
            );
        }
        cockpit::Cockpit::new().draw(
            &mut hd,
            font,
            cockpit::CockpitStyle::COBRA_MK3,
            &cockpit::KLADOV_COCKPIT.read(&m.mem),
        );
        assert!(
            hd.pixels().iter().all(|&p| p >> 24 == 0xFF),
            "на стыке вида и панели осталась прозрачная щель"
        );
        if let Ok(path) = env::var("ELITE_DUMP_FLIGHT_SCREEN") {
            std::fs::create_dir_all(format!("{base}/tmp")).unwrap();
            write_ppm(&path, hd.pixels(), w, h);
        }
    }

    /// Меню паузы рисует затемнённый пункт SAVE на заставке.
    #[test]
    fn intro_pause_menu_dims_save() {
        let font = ZX_FONT;
        // Цвета пунктов SAVE: затемнённый на заставке и обычный в игре.
        let counts = |intro: bool| {
            let mut hd = display::Frame::new(256 * 5, 192 * 5);
            draw_pause_menu(&mut hd, font, 0, intro);
            let dim = hd.pixels().iter().filter(|&&p| p == 0x0080_8080).count();
            let white = hd.pixels().iter().filter(|&&p| p == 0xFFD7_D7D7).count();
            (dim, white)
        };
        let (dim_on_intro, white_on_intro) = counts(true);
        let (dim_in_game, white_in_game) = counts(false);
        assert!(
            dim_on_intro > 0,
            "на заставке пункт SAVE должен быть затемнён"
        );
        assert_eq!(dim_in_game, 0, "в игре SAVE не затемняется");
        assert!(
            white_on_intro < white_in_game,
            "затемнение должно уменьшать белый текст"
        );
    }

    /// Игровая клавиша при открытом нативном экране закрывает его и уходит в
    /// игру — из карты можно сразу перейти к экранам игры, не закрывая карту.
    #[test]
    fn game_key_closes_native_screen_and_forwards() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        // `L` (матрица 6,1) — игровая клавиша: карта закрывается.
        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.hdmap.active, "игровая клавиша не закрыла карту");
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[6] & (1 << 1),
            0,
            "нажатие L не передано игре"
        );
        // Управляющая клавиша карты (стрелка) карту не закрывает.
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::ArrowRight, true, false);
        assert!(app.hdmap.active, "стрелка не должна закрывать карту");
        // Автоповтор управляющей клавиши тоже не закрывает (иначе Space
        // «залипал» бы в игру как огонь).
        app.key_event(KeyCode::Space, true, false);
        assert!(
            app.hdmap.active,
            "Space должен листать карту, а не закрывать"
        );
        app.key_event(KeyCode::Space, true, true);
        assert!(
            app.hdmap.active,
            "автоповтор Space не должен закрывать карту"
        );

        // F4 на станции: сначала клавиша 4 уходит игре, затем открывается
        // нативный экран закупки.
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.hdmap.active);
        app.key_event(KeyCode::F4, true, false);
        assert!(
            !app.shop.active,
            "закупка должна открыться после кадра игры"
        );
        app.apply_input();
        app.machine.run_frame();
        app.open_pending_shop();
        assert!(app.shop.active);
        app.key_event(KeyCode::KeyO, true, false);
        assert!(!app.shop.active, "игровая клавиша не закрыла закупку");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyO).expect("O в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "нажатие O не передано игре"
        );
    }
