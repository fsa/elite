//! Тесты оконного слоя: нативные экраны, ввод, заставка и отрисовка.
//!
//! Живут внутри модуля `app`, поэтому видят приватные поля [`App`] и его
//! методы; общие подсистемы импортируются явно.

use std::env;
use std::fs;

use winit::keyboard::KeyCode;

use super::*;
use crate::boot::{load_kladov_image, set_kladov_cpu};
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
use crate::ui::present::{
    DebugView, SRC_H, SRC_W, blit_zx_frame, debug_button_at, debug_button_rects, draw_debug_panel,
};
use crate::world::{current_planet_color, save_layout_kladov, scene_markers};


    /// Boot the real game image with the real ROM, as `main` does.
    fn boot() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let mut m = Spectrum::new(Some(&rom));
        load_kladov_image(&mut m);
        set_kladov_cpu(&mut m);
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
            headless: true,
            frames: 0,
            dump: None,
            mute: true,
            audio: false,
            wav: None,
            native: false,
            save: None,
            fullscreen: false,
            debug: false,
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

    /// Режим `--debug`: окно делится на игровую часть слева и полосу
    /// отладки справа. Полоса шириной ровно 256 пикселей, кадр эмулятора
    /// выводится в ней 1:1, ниже — чёрный фон.
    #[test]
    fn debug_mode_reserves_256px_panel_on_right() {
        let (w, h) = (1280u32, 768u32);
        let l = display::layout_for(w, h, true);
        assert_eq!(l.scale, 4);
        assert_eq!((l.x, l.y, l.w, l.h), (0, 0, 1024, 768));
        assert_eq!(display::debug_panel_x(w), 1024);
        assert_eq!(display::game_area(w, h, true), (1024, 768));
        // Без отладки игра по-прежнему занимает всё окно.
        assert_eq!(display::game_area(w, h, false), (w, h));

        // Миниатюра — сырой кадр эмулятора 1:1 (номер строки в младшем
        // байте), ниже 192 строк полоса чёрная.
        let mut src = vec![0u32; SRC_W * SRC_H];
        for y in 0..SRC_H {
            for x in 0..SRC_W {
                src[y * SRC_W + x] = 0xFF00_0000 | y as u32;
            }
        }
        let mut out = vec![0xFFFF_FFFFu32; w as usize * h as usize];
        draw_debug_panel(&src, &mut out, w as usize, h as usize, &[], DebugView::Summary);
        let x0 = display::debug_panel_x(w) as usize;
        assert_eq!(out[x0] & 0xFF, 0);
        assert_eq!(out[(SRC_H - 1) * w as usize + x0] & 0xFF, (SRC_H - 1) as u32);
        assert_eq!(out[SRC_H * w as usize + x0], 0, "ниже миниатюры не чёрно");
        // Отступ в одно знакоместо от основного экрана (верх дампа) пустой.
        assert_eq!(out[(SRC_H + 4) * w as usize + x0 + 8], 0);
        assert_eq!(out[0], 0xFFFF_FFFF, "игровая часть затёрта");
        assert_eq!(out[x0 - 1], 0xFFFF_FFFF, "полоса задела игру");
    }

    /// Под миниатюрой две кнопки режимов: по умолчанию сводка, вторая кнопка
    /// включает полный дамп. Клик определяется по координатам окна.
    #[test]
    fn debug_panel_buttons_switch_view() {
        let w = 1280usize;
        let rects = debug_button_rects(w);
        let center = |r: (i32, i32, usize, usize)| {
            (
                r.0 as f64 + r.2 as f64 / 2.0,
                r.1 as f64 + r.3 as f64 / 2.0,
            )
        };
        let (x0, y0) = center(rects[0]);
        let (x1, y1) = center(rects[1]);
        assert_eq!(debug_button_at(w, x0, y0), Some(DebugView::Summary));
        assert_eq!(debug_button_at(w, x1, y1), Some(DebugView::Dump));
        // Вне кнопок клик режим не меняет.
        assert_eq!(debug_button_at(w, 10.0, 10.0), None);
        assert_eq!(
            debug_button_at(w, x1, (rects[1].1 + rects[1].3 as i32 + 4) as f64),
            None
        );

        // Сводка и дамп дают разное содержимое под кнопками.
        let save: Vec<u8> = (0..0x66u8).collect();
        let src = vec![0u32; SRC_W * SRC_H];
        let mut summary = vec![0u32; w * 768];
        let mut dump = vec![0u32; w * 768];
        draw_debug_panel(
            &src,
            &mut summary,
            w,
            768,
            &save,
            DebugView::Summary,
        );
        draw_debug_panel(&src, &mut dump, w, 768, &save, DebugView::Dump);
        let content = rects[0].1 as usize + rects[0].3 + 8;
        let has_content = |buf: &[u32]| {
            (content..(content + 200).min(768))
                .any(|y| buf[y * w..y * w + w].iter().any(|&p| p != 0))
        };
        assert!(has_content(&summary));
        assert!(has_content(&dump));
        assert_ne!(summary, dump);
    }

    /// F6 открывает отдельную нативную карту галактики: игра не встаёт, но
    /// переходит в режим `SHORT RANGE CHART`, ввод уходит карте. Ни повторный
    /// F6, ни Esc не закрывают карту — выход только переходом на другой экран.
    #[test]
    fn f6_opens_and_closes_galaxy_map() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        let prev_screen = app.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        assert!(!app.hdmap.active);
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не открыл карту");
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
        // Повторное F6 не закрывает карту и не сбрасывает её состояние.
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не должен закрывать карту");
        assert_eq!(app.hdmap.cursor(), (map_x, map_y));
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_LOCAL_MAP
        );
        // Esc тоже не выход из режима карты.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_LOCAL_MAP
        );
        // Переход на другой экран (F1) закрывает карту и возвращает экран игры.
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.hdmap.active, "F1 не переключил экран");
        assert_eq!(app.machine.mem[flight::Z80_WHAT_ON_SCREEN], prev_screen);
        // После перехода F6 снова открывает карту — курсор остаётся на цели,
        // которую игра хранит как координаты гиперпрыжка.
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);
        assert_eq!(app.hdmap.cursor(), (map_x, map_y));
    }

    /// F8 включает нативный экран цен; на станции стрелки выбирают товар,
    /// `→` покупает, `←` продаёт, а сами стрелки в игру не уходят.
    #[test]
    fn f8_market_trades_with_arrows() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert_eq!(app.game_state().system, "Lave");
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_MARKET;

        let market_layout = market::KLADOV_MARKET;
        let p = market_layout.pilot as usize;
        let price_addr = market_layout.prices;
        app.machine.mem[price_addr] = 40;
        app.machine.mem[price_addr + 1] = 0;
        app.machine.mem[market_layout.quantities] = 2;
        app.machine.mem[p + 40] = 10; // PilotFreeCargo
        app.machine.mem[p + 20..p + 22].copy_from_slice(&1000u16.to_le_bytes()); // cash1
        app.machine.mem[p + 18..p + 20].copy_from_slice(&0u16.to_le_bytes()); // cash2

        app.key_event(KeyCode::F8, true, false);
        assert!(app.market.native, "F8 не открыл рынок");
        app.key_event(KeyCode::ArrowRight, true, false);
        assert_eq!(app.machine.mem[market_layout.quantities], 1);
        assert_eq!(app.machine.mem[p + 23], 1); // Food в трюме
        assert_eq!(app.machine.mem[p + 40], 9);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            960
        );
        app.key_event(KeyCode::ArrowLeft, true, false);
        assert_eq!(app.machine.mem[market_layout.quantities], 2);
        assert_eq!(app.machine.mem[p + 23], 0);
        assert_eq!(app.machine.mem[p + 40], 10);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            1000
        );

        // Стрелки остаются на экране рынка и не попадают в Kempston.
        app.key_event(KeyCode::ArrowDown, true, false);
        app.apply_input();
        assert_eq!(app.machine.kempston, 0);
    }

    /// В полёте F8 показывает цены, но стрелки не торгуют: экран остаётся
    /// только для просмотра, а ввод уходит эмулятору.
    #[test]
    fn f8_market_in_flight_is_read_only() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0; // вид из кабины

        let market_layout = market::KLADOV_MARKET;
        let p = market_layout.pilot as usize;
        app.machine.mem[market_layout.prices] = 40;
        app.machine.mem[market_layout.quantities] = 2;
        app.machine.mem[p + 40] = 10;
        app.machine.mem[p + 20..p + 22].copy_from_slice(&1000u16.to_le_bytes());

        app.key_event(KeyCode::F8, true, false);
        assert!(app.market.native, "F8 не открыл рынок в полёте");
        assert!(app.pending_market, "F8 не запросил переход в режим рынка");

        app.apply_input();
        app.machine.run_frame();
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_MARKET;
        app.open_pending_market();
        assert!(app.market.native, "F8 не открыл рынок в полёте");
        assert!(!app.pending_market, "переход в режим рынка не завершён");

        app.key_event(KeyCode::ArrowRight, true, false);
        assert_eq!(app.machine.mem[market_layout.quantities], 2);
        assert_eq!(app.machine.mem[p + 23], 0);
        assert_eq!(app.machine.mem[p + 40], 10);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            1000
        );

        // Стрелка в полёте остаётся обычным вводом для игры.
        app.apply_input();
        assert_eq!(app.machine.kempston, 1, "стрелка не дошла до эмулятора");
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

    /// F5 — нативный экран закупки оборудования на станции. Эмулятору
    /// уходит `L` (пассивный экран информации о пилоте), а не `4`, чтобы
    /// оригинальный экран 4 не ждал номер товара. Выход по F5 и Esc запрещён;
    /// закрыть можно Fn-клавишей или игровой буквенной клавишей.
    #[test]
    fn f5_opens_equip_shop_without_f5_or_esc_exit() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.shop.active);
        app.key_event(KeyCode::F5, true, false);
        // Закупка отложена: сначала кадр игры с клавишей L.
        assert!(!app.shop.active, "F5 открыл закупку до перехода игры");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F5 не передал игре клавишу L"
        );
        app.machine.run_frame();
        app.open_pending_shop();
        assert!(app.shop.active, "F5 не открыл экран закупки");
        assert!(!app.hdmap.active, "карта галактики не должна открыться");
        // Экран снял данные живого командира.
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

        // Esc и F5 не закрывают экран.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.shop.active, "Esc не должен закрывать экран закупки");
        app.key_event(KeyCode::F5, true, false);
        assert!(app.shop.active, "F5 не должен закрывать экран закупки");

        // Закрыть можно Fn-клавишей (F1 — вылет/вид).
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.shop.active, "F1 не закрыл экран закупки");
    }

    /// F7 — нативная альтернатива экрану информации о планете (`P`).
    /// Нативный экран открывается сразу, эмулятору уходит нажатие `P`, а
    /// переход в режим оригинала завершается после его обработки. Управление
    /// остаётся за эмулятором, Esc открывает меню паузы, а нативный слой
    /// закрывается сам, когда Z80 уходит с экрана планеты.
    #[test]
    fn f7_opens_planet_info() {
        let opts = test_options();
        let mut m = boot();
        let (intro, game) = scene_markers();
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        let mut app = App::new(m, &opts);
        native::start_game(&mut app.machine, save_layout_kladov());
        for _ in 0..5000 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                break;
            }
        }
        assert!(!app.machine.intro_seen, "подготовка: игра не запустилась");

        app.key_event(KeyCode::F7, true, false);
        assert!(app.sysinfo.active, "F7 не открыл нативный экран");
        assert!(app.pending_sysinfo, "F7 не запросил переход в режим планеты");
        let (prow, pbit) = matrix_pos(KeyCode::KeyP).expect("P в матрице");
        assert_eq!(app.pending_key, Some((prow, pbit)), "F7 не отдал игре P");

        // Кадр, в котором оригинал обрабатывает P и завершает переход.
        app.apply_input();
        app.machine.run_frame();
        app.open_pending_sysinfo();
        assert!(app.sysinfo.active, "F7 закрыл нативный экран");
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_INFO_PLANET,
            "эмулятор не перешёл в режим экрана планеты"
        );
        assert!(!app.sysinfo.info.name.is_empty(), "данные планеты не прочитаны");
        assert_eq!(
            app.game_state().planet,
            app.sysinfo.info.name,
            "имя планеты должно приходить из состояния игры"
        );

        // Esc открывает меню паузы, нативный экран под ним остаётся.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню паузы");
        assert!(app.sysinfo.active, "Esc закрыл нативный экран");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");

        // Z80 ушёл с экрана планеты — нативная надстройка закрывается.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_native_screens();
        assert!(!app.sysinfo.active, "нативный экран не закрылся вслед за Z80");

        // Если оригинал уже на экране планеты, F7 включает отрисовку без
        // повторной отправки `P`.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_INFO_PLANET;
        app.key_event(KeyCode::F7, true, false);
        assert!(app.sysinfo.active, "повторный F7 не открыл экран");
        assert_eq!(app.pending_key, None, "F7 зря отправил P в эмулятор");
    }

    /// F9 — нативная альтернатива экрану капитана (`L`). Работает так же,
    /// как родная команда `L`: и на станции, и в полёте. Нативный слой —
    /// альтернативная отрисовка: ввод получает эмулятор, Esc открывает меню
    /// паузы, а слой закрывается сам, когда Z80 уходит с экрана капитана.
    #[test]
    fn f9_opens_captain_alternative() {
        let opts = test_options();
        // Запускаем игру до станции: снапшот сам по себе ещё на заставке.
        let mut m = boot();
        let (intro, game) = scene_markers();
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        let mut app = App::new(m, &opts);
        native::start_game(&mut app.machine, save_layout_kladov());
        for _ in 0..5000 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                break;
            }
        }
        assert!(!app.machine.intro_seen, "подготовка: игра не запустилась");
        assert!(
            flight::z80_is_docked(&app.machine.mem),
            "подготовка: ожидалась станция"
        );
        // На станции оригинал уже показывает капитана; откатываем экран,
        // чтобы проверить, что F9 действительно возвращает его нажатием L.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        assert!(!app.status.active);
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не открыл нативный экран");
        assert!(app.pending_status, "F9 не запросил переход в режим капитана");

        // Эмулятору подготовлено нажатие `L`; первый кадр отдаёт его игре
        // через обычный ввод.
        let (lrow, lbit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(app.pending_key, Some((lrow, lbit)), "F9 не отдал игре L");
        app.apply_input();
        app.machine.run_frame();
        app.open_pending_status();
        app.follow_native_screens();
        assert_eq!(
            app.machine.keyboard[lrow] & (1 << lbit),
            0,
            "L не ушёл в эмулятор"
        );
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_PILOT_INFO,
            "эмулятор не перешёл в режим показа капитана"
        );
        assert!(app.status.active, "нативный экран закрылся в режиме капитана");
        assert!(!app.pending_status, "переход в режим капитана не завершён");

        // Ввод больше не перехватывается и уходит в эмулятор.
        app.key_event(KeyCode::ArrowUp, true, false);
        app.apply_input();
        assert_ne!(app.machine.kempston & 0b1000, 0, "ввод не ушёл в игру");

        // Esc открывает меню паузы, нативный экран под ним остаётся.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню паузы");
        assert!(app.status.active, "Esc закрыл экран капитана");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");

        // Повторный F9 не закрывает экран.
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "повторный F9 закрыл экран");

        // Z80 ушёл с экрана капитана — нативная надстройка закрывается.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_native_screens();
        assert!(!app.status.active, "нативный экран не закрылся вслед за Z80");

        // Если оригинал уже на экране капитана, F9 включает отрисовку без
        // повторной отправки `L`.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не включил экран поверх режима капитана");
        assert_eq!(app.pending_key, None, "F9 зря отправил L в эмулятор");

        // В полёте F9 тоже открывает альтернативу и отдаёт игре L.
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не открыл нативный экран в полёте");
        assert!(app.pending_status, "F9 не запросил экран капитана в полёте");
        assert!(!app.flight.active, "нативный вид остался под экраном");
        let (lrow, lbit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(app.pending_key, Some((lrow, lbit)), "F9 не отдал игре L");
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

        // F2-F4 — виды назад/влево/вправо и клавиши 2-4 оригинала. Это
        // работает только в полёте: на станции цифры 2/3 подавлены.
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
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
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не открыл карту поверх полёта");
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не должен закрывать карту");
        assert!(app.flight.active, "F6 выключил полёт под картой");
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        // F4 — переход к другому экрану (виду вправо), он закрывает карту.
        app.key_event(KeyCode::F4, true, false);
        assert!(!app.hdmap.active, "F4 не переключил экран");
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

    /// На станции F1 повторяет оригинальную клавишу 1 (вылет), а F5 открывает
    /// нативную закупку и передаёт эмулятору `L` (пассивный экран), чтобы не
    /// входить в режим ожидания номера товара на экране 4. F2-F4 не реализованы:
    /// их цифры не уходят в эмулятор, иначе он сбивается в покупку/продажу.
    #[test]
    fn function_keys_send_original_number_keys() {
        let opts = test_options();
        // F1 -> 1
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F1, true, false);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 не передал игре Digit1"
        );

        // F5 -> L (открытие закупки без входа в экран 4 оригинала)
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F5, true, false);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F5 не передал игре KeyL"
        );

        for fkey in [KeyCode::F2, KeyCode::F3, KeyCode::F4] {
            let mut app = App::new(boot(), &opts);
            app.key_event(fkey, true, false);
            app.apply_input();
            let digit = match fkey {
                KeyCode::F2 => KeyCode::Digit2,
                KeyCode::F3 => KeyCode::Digit3,
                KeyCode::F4 => KeyCode::Digit4,
                _ => unreachable!(),
            };
            let (row, bit) = matrix_pos(digit).expect("цифра в матрице");
            assert_ne!(
                app.machine.keyboard[row] & (1 << bit),
                0,
                "{fkey:?} не должен передавать игре {digit:?}"
            );
            assert_eq!(app.pending_key, None, "{fkey:?} оставил отложенную клавишу");
        }
    }

    /// На нативной карте F1 закрывает её и уходит игре. F5 закрывает карту и
    /// открывает закупку на станции (эмулятору уходит `L`). F2-F4 на станции
    /// остаются пустыми: карта не закрывается и цифры в игру не идут.
    #[test]
    fn map_function_keys_forward_original_keys() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F6, true, false);
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

        // F5 закрывает карту и запрашивает закупку (на станции), эмулятору L.
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::F5, true, false);
        assert!(!app.hdmap.active, "F5 не закрыл карту");
        assert!(!app.flight.active, "F5 не должен включать полёт");
        assert!(app.pending_shop, "F5 на станции должен запросить закупку");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F5 на карте не передал игре клавишу L"
        );

        // F2-F4 на станции не передают цифры; открытая карта при этом
        // остаётся — эти клавиши здесь просто не реализованы.
        for fkey in [KeyCode::F2, KeyCode::F3, KeyCode::F4] {
            let mut app = App::new(boot(), &opts);
            app.key_event(KeyCode::F6, true, false);
            assert!(app.hdmap.active);
            app.key_event(fkey, true, false);
            assert!(app.hdmap.active, "{fkey:?} закрыл карту");
            app.apply_input();
            let digit = match fkey {
                KeyCode::F2 => KeyCode::Digit2,
                KeyCode::F3 => KeyCode::Digit3,
                KeyCode::F4 => KeyCode::Digit4,
                _ => unreachable!(),
            };
            let (row, bit) = matrix_pos(digit).expect("цифра в матрице");
            assert_ne!(
                app.machine.keyboard[row] & (1 << bit),
                0,
                "{fkey:?} на карте не должен передавать игре {digit:?}"
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
        assert!(app.flight.active, "F1 не открыл нативный гиперпереход");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "прыжок показан не как гиперпереход"
        );
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);

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
    /// показывает нативную анимацию гиперперехода. В режиме анимации
    /// доступен только передний вид.
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

        // Начался прыжок: оригинал держит флаги перехода. Гиперпереход —
        // режим анимации, поэтому отображается только передний вид.
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
        // Синхронизация с памятью Z80 фиксирует передний вид в анимации.
        app.flight.sync_from_z80(&app.machine.mem);
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);

        // Переключение вида во время анимации не допускается и не сбрасывает её.
        app.key_event(KeyCode::F3, true, false);
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "смена вида сбросила переход"
        );

        // Прыжок закончился — нативный вид остаётся активным (в переднем виде).
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;
        app.follow_flight_screen();
        assert!(app.flight.active, "вид закрылся после прыжка");
        assert!(!app.flight.in_transition(), "переход не закончился");
        assert_eq!(app.flight.view_dir, flight::ViewDir::Forward);
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
        let (intro, game) = scene_markers();
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
        let mut m = Spectrum::new(Some(&rom));
        load_kladov_image(&mut m);
        set_kladov_cpu(&mut m);
        native::install_save(&mut m, native::KLADOV_SAVE);
        m
    }

    fn kladov_options() -> Options {
        test_options()
    }

    /// На заставке нативные экраны не открываются: F6/F5 игнорируются, пока
    /// игра проигрывает интро. F10 пуста и в эмулятор не передаётся.
    #[test]
    fn native_screens_blocked_on_intro() {
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers();
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F6, true, false);
        assert!(
            !app.hdmap.active,
            "F6 не должен открываться на заставке Кладова"
        );
        app.key_event(KeyCode::F10, true, false);
        assert!(
            !app.hdmap.active,
            "F10 не должен открывать карту"
        );
        app.key_event(KeyCode::F5, true, false);
        assert!(
            !app.shop.active && !app.pending_shop,
            "F5 не должен открываться на заставке Кладова"
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
        let (intro, game) = scene_markers();
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "сначала должны быть на заставке");
        m.note_pc(game.unwrap());
        assert!(!m.intro_seen, "вход в GameMain не снял флаг заставки");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F6, true, false);
        assert!(
            app.hdmap.active,
            "после старта игры F6 должен открывать карту"
        );
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "повторный F6 не должен закрывать карту");
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        app.close_hdmap();
        assert!(!app.hdmap.active, "подготовка: карта не закрылась");
        // F10 пуста: ни открывать карту, ни уходить в эмулятор.
        app.key_event(KeyCode::F10, true, false);
        assert!(!app.hdmap.active, "F10 не должен открывать карту");
        assert_eq!(app.pending_key, None, "F10 должен быть пустой клавишей");
    }

    /// Проверка на живом образе Кладова: пока не нажат SPACE, флаг заставки
    /// держится; после старта игры (вход в `GameMain`) снимается.
    #[test]
    fn kladov_intro_clears_after_space() {
        let mut m = boot_kladov();
        let (intro, game) = scene_markers();
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

    /// На заставке работает только SPACE: остальные клавиши (в т.ч. F6/F4)
    /// игнорируются и в эмулятор не уходят. SPACE запускает игру родным путём.
    #[test]
    fn intro_accepts_only_space() {
        let opts = test_options();
        let mut m = boot();
        let (intro, game) = scene_markers();
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
        app.key_event(KeyCode::F6, true, false);
        app.key_event(KeyCode::F4, true, false);
        app.key_event(KeyCode::Digit1, true, false);
        app.key_event(KeyCode::KeyO, true, false);
        assert!(!app.menu, "лишние клавиши не должны открывать меню");
        assert!(!app.hdmap.active && !app.shop.active && !app.pending_shop);
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
        let (intro, game) = scene_markers();
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
        app.key_event(KeyCode::Enter, false, false);
        assert!(!app.menu, "загрузка должна закрыть меню");
        assert!(app.machine.load_requested, "загрузка не запрошена");
    }

    /// Загрузка из меню с заставки доводит игру до станции, и после неё
    /// клавиши снова управляют игрой, а не остаются в нативном экране.
    #[test]
    fn menu_load_from_intro_leaves_station_playable() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        fs::create_dir_all(&dir).unwrap();
        let path = format!("{dir}/app_load_intro.zxelite");

        // Готовим сохранение: стартуем и сохраняемся на станции.
        let (intro, game) = scene_markers();
        let mut src = boot_kladov();
        src.intro_range = intro;
        src.game_marker = game;
        for f in 0..900u64 {
            src.release_all_keys();
            if f >= 30 && f % 16 < 6 {
                src.set_key(7, 0, true); // SPACE
            }
            src.run_frame();
        }
        assert_eq!(src.mem[0x7104], 0, "подготовка: не на станции");
        native::commander_save(&mut src, &path, save_layout_kladov()).unwrap();

        // Свежий запуск: заставка, меню паузы, пункт «загрузить».
        let mut m = boot_kladov();
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..60 {
            m.run_frame();
        }
        assert!(m.intro_seen, "подготовка: нет заставки");
        let mut opts = test_options();
        opts.save = Some(path.clone());
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню");
        app.menu_sel = 2;
        app.key_event(KeyCode::Enter, true, false);
        app.key_event(KeyCode::Enter, false, false);
        assert!(!app.menu, "загрузка должна закрыть меню");
        assert!(app.machine.load_requested, "загрузка не запрошена");

        // Даём игре дойти до станции.
        for _ in 0..900u64 {
            app.machine.release_all_keys();
            app.machine.run_frame();
        }
        let _ = fs::remove_file(&path);
        assert!(!app.machine.intro_seen, "после загрузки осталась заставка");
        assert_eq!(app.machine.mem[0x7104], 0, "после загрузки не станция");
        assert_eq!(
            app.machine.mem[0x70E3], 0x0B,
            "после загрузки не станционный экран"
        );

        // Управление работает: клавиша `1` запускает вылет.
        app.key_event(KeyCode::Digit1, true, false);
        for f in 0..900u64 {
            app.apply_input();
            if f == 2 {
                app.key_event(KeyCode::Digit1, false, false);
            }
            app.machine.run_frame();
        }
        assert_ne!(
            app.machine.mem[0x7104], 0,
            "после загрузки клавиша вылета не работает"
        );
    }


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
    /// заставки уводит Z80 на ветку старта. Она сама вызывает
    /// `Cmd_L_PilotInfo`, поэтому эмулятор выходит в режим капитана без
    /// клавиши `L`. Нативная страница капитана открывается только после того,
    /// как эмулятор действительно перешёл в режим игры.
    #[test]
    fn kladov_intro_space_shows_captain_page() {
        let opts = test_options();
        let mut m = boot_kladov();
        let (intro, gs) = scene_markers();
        m.intro_range = intro;
        m.game_marker = gs;
        for _ in 0..120 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::Space, true, false);
        assert!(app.pending_status, "SPACE не запросил страницу капитана");
        let mut started = false;
        for _ in 0..300 {
            // Так же, как кадр заставки делает до и после прогона эмулятора.
            // Важно: закрытие экранов на заставке не должно сбрасывать
            // отложенный запрос — иначе нативный экран не откроется.
            if app.machine.intro_seen {
                app.close_intro_screens();
            }
            app.machine.run_frame();
            app.open_pending_status();
            if !app.machine.intro_seen {
                started = true;
                break;
            }
        }
        assert!(started, "SPACE не перевёл эмулятор в режим игры");
        assert_eq!(
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN],
            flight::Z80_SCREEN_PILOT_INFO,
            "эмулятор сам не вышел на экран капитана"
        );
        assert!(app.status.active, "нативная страница капитана не открылась");
        app.sync_state();
        assert_eq!(app.game_state().screen, Screen::PilotStatus);
        // Сам `Space` не должен дойти до игры как её собственная клавиша.
        app.apply_input();
        assert_ne!(app.machine.keyboard[7] & 1, 0, "SPACE ушёл в игру");
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
        app.key_event(KeyCode::F6, true, false);
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
        app.key_event(KeyCode::F6, true, false);
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

        // F5 на станции: переход на другой экран закрывает карту, затем
        // открывается нативный экран закупки. Esc/F5 закупку не закрывают,
        // буквенная клавиша — закрывает и уходит в эмулятор.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        app.key_event(KeyCode::F5, true, false);
        assert!(!app.hdmap.active, "F5 не переключил экран");
        assert!(
            !app.shop.active,
            "закупка должна открыться после кадра игры"
        );
        app.apply_input();
        app.machine.run_frame();
        app.open_pending_shop();
        assert!(app.shop.active);
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.shop.active, "Esc не должен закрывать закупку");
        app.key_event(KeyCode::F5, true, false);
        assert!(app.shop.active, "F5 не должен закрывать закупку");
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

    /// На станции `G`/`H` при открытом нативном экране показывают заглушку
    /// `DOCKED` примерно на секунду и воспроизводят звук нативным компонентом
    /// эффектов. Нативный экран под заглушкой остаётся открытым и возвращается
    /// после её закрытия.
    #[test]
    fn g_and_h_on_station_show_docked_stub_over_native_screen() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "подготовка: карта не открылась");

        for key in [KeyCode::KeyG, KeyCode::KeyH] {
            let (row, bit) = matrix_pos(key).expect("клавиша в матрице");
            app.key_event(key, true, false);
            assert!(app.docked.is_active(), "{key:?} не показал DOCKED");
            assert!(app.hdmap.active, "{key:?} закрыл нативный экран");
            assert!(app.sfx.is_playing(), "{key:?} не запустил звук");

            // Клавиша не залипает в матрице.
            assert!(!app.matrix[row][bit as usize], "{key:?} залипла в матрице");
            assert_eq!(app.pending_key, None, "{key:?} не должна уходить в эмулятор");

            // После показа заглушки возвращается прежний нативный экран.
            app.docked.close();
            assert!(app.hdmap.active, "после заглушки карта не вернулась");
        }
    }
