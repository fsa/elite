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
    save_editor_button_at, save_editor_button_rect,
};
use crate::world::{
    MAX_CASH_TENTHS, commander_cash_tenths, current_planet_color, save_layout_kladov,
    scene_markers,
};


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

    /// Клик по овалу радара переключает вид локатора только в полёте.
    #[test]
    fn radar_click_toggles_view_only_in_flight() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        let (w, h) = (1280u32, 960u32);
        let cell = cockpit::panel_geometry(w, h).oval_radar_cell();
        let x = cell.x + cell.w / 2;
        let y = cell.y + cell.h / 2;

        assert_eq!(app.radar_view, cockpit::RadarView::Classic);
        app.state.screen = Screen::Flight;
        assert!(app.toggle_radar_at(x, y, w, h), "клик по овалу не сработал");
        assert_eq!(app.radar_view, cockpit::RadarView::Kladov);
        assert!(app.toggle_radar_at(x, y, w, h), "повторный клик не сработал");
        assert_eq!(app.radar_view, cockpit::RadarView::Classic);
        assert!(!app.toggle_radar_at(0, 0, w, h), "клик вне овала сработал");
        assert_eq!(app.radar_view, cockpit::RadarView::Classic);

        app.state.screen = Screen::Intro;
        assert!(!app.toggle_radar_at(x, y, w, h), "клик на заставке сработал");
        assert_eq!(app.radar_view, cockpit::RadarView::Classic);
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
        draw_debug_panel(&src, &mut out, w as usize, h as usize, &[], DebugView::Summary, false);
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
            false,
        );
        draw_debug_panel(&src, &mut dump, w, 768, &save, DebugView::Dump, false);
        let content = rects[0].1 as usize + rects[0].3 + 8;
        let has_content = |buf: &[u32]| {
            (content..(content + 200).min(768))
                .any(|y| buf[y * w..y * w + w].iter().any(|&p| p != 0))
        };
        assert!(has_content(&summary));
        assert!(has_content(&dump));
        assert_ne!(summary, dump);
    }

    /// Кнопка редактора рисуется в отладочной панели только в игре.
    /// На заставке `intro_seen` взведён, и кнопка скрыта.
    #[test]
    fn editor_button_shown_only_in_game() {
        let (w, h) = (1280usize, 768usize);
        let src = vec![0u32; SRC_W * SRC_H];
        let (bx, by, bw, bh) = save_editor_button_rect(w, h);
        let mut intro = vec![0u32; w * h];
        let mut game = vec![0u32; w * h];
        draw_debug_panel(&src, &mut intro, w, h, &[], DebugView::Summary, false);
        draw_debug_panel(&src, &mut game, w, h, &[], DebugView::Summary, true);
        let px = (bx + 1) as usize + (by + 1) as usize * w;
        assert_eq!(intro[px], 0, "кнопка нарисована на заставке");
        assert_ne!(game[px], 0, "кнопка не нарисована в игре");
        assert!(save_editor_button_at(
            w,
            h,
            bx as f64 + bw as f64 / 2.0,
            by as f64 + bh as f64 / 2.0
        ));
    }

    /// ALT+E не открывает редактор на заставке и работает после входа в игру.
    #[test]
    fn alt_e_editor_only_in_game() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.alt = true;
        app.machine.intro_seen = true;
        app.key_event(KeyCode::KeyE, true, false);
        assert!(!app.save_editor_open, "ALT+E сработал на заставке");

        app.machine.intro_seen = false;
        app.key_event(KeyCode::KeyE, true, false);
        assert!(app.save_editor_open, "ALT+E не открыл редактор в игре");
        app.key_event(KeyCode::KeyE, true, false);
        assert!(!app.save_editor_open, "ALT+E не закрыл редактор");
    }

    /// Кадр не должен сам закрывать редактор в игре; закрытие — только на заставке.
    #[test]
    fn frame_keeps_editor_open_in_game() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.save_editor_open = true;
        app.sync_intro_state();
        assert!(app.save_editor_open, "кадр закрыл редактор в игре");

        app.machine.intro_seen = true;
        app.sync_intro_state();
        assert!(!app.save_editor_open, "кадр не закрыл редактор на заставке");
    }

    /// Деньги в редакторе — сумма двух половин блока командира. Стрелки и
    /// ввод меняют её целиком и не затирают соседние поля (`PilotRnd`, груз).
    #[test]
    fn save_editor_cash_uses_both_halves() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        let p = save_layout_kladov().pilot_name as usize;
        app.machine.mem[p + 18..p + 20].copy_from_slice(&327u16.to_le_bytes());
        app.machine.mem[p + 20..p + 22].copy_from_slice(&9868u16.to_le_bytes());
        app.machine.mem[p + 22] = 0xA5; // PilotRnd
        app.machine.mem[p + 23] = 0x5A; // первый товар груза

        app.save_editor_open = true;
        app.save_editor_sel = 1;
        app.key_event(KeyCode::ArrowRight, true, false);
        assert_eq!(commander_cash_tenths(&app.machine.mem, p), 3_279_878);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 18], app.machine.mem[p + 19]]),
            327
        );
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            9878
        );

        app.key_event(KeyCode::Enter, true, false);
        for digit in "2500000".bytes() {
            let key = match digit {
                b'0' => KeyCode::Digit0,
                b'1' => KeyCode::Digit1,
                b'2' => KeyCode::Digit2,
                b'3' => KeyCode::Digit3,
                b'4' => KeyCode::Digit4,
                b'5' => KeyCode::Digit5,
                b'6' => KeyCode::Digit6,
                b'7' => KeyCode::Digit7,
                b'8' => KeyCode::Digit8,
                _ => KeyCode::Digit9,
            };
            app.key_event(key, true, false);
        }
        app.key_event(KeyCode::Enter, true, false);
        assert_eq!(commander_cash_tenths(&app.machine.mem, p), 2_500_000);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 18], app.machine.mem[p + 19]]),
            250
        );
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            0
        );
        assert_eq!(app.machine.mem[p + 22], 0xA5);
        assert_eq!(app.machine.mem[p + 23], 0x5A);

        // Слишком большая сумма ограничивается максимумом двух ячеек.
        app.key_event(KeyCode::Enter, true, false);
        for _ in 0..9 {
            app.key_event(KeyCode::Digit9, true, false);
        }
        app.key_event(KeyCode::Enter, true, false);
        assert_eq!(commander_cash_tenths(&app.machine.mem, p), MAX_CASH_TENTHS);
    }

    /// F6 открывает нативную карту галактики на станции. Пока карта открыта,
    /// ввод уходит карте, а эмулятор заморожен. Повторный F6 и Esc карту не
    /// закрывают. F5 переключает на другой нативный экран, F1 возвращает
    /// управление эмулятору для вылета.
    #[test]
    fn f6_opens_and_closes_galaxy_map() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        let prev_screen = app.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        assert!(!app.hdmap.active);
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не открыл карту");
        assert!(app.station_native, "карта не забрала управление");
        assert_eq!(app.hdmap.planet_count(), galaxy::COUNT);
        // Курсор карты стартует на текущей системе (из памяти игры), и эта
        // точка сразу передана в эмулятор как цель карты.
        let sys_x = app.machine.mem[galaxy::KLADOV_GALAXY.sys_x] as i32;
        let sys_y = app.machine.mem[galaxy::KLADOV_GALAXY.sys_y] as i32;
        assert_eq!(app.hdmap.cursor(), (sys_x, sys_y));
        let live = save_layout_kladov().pilot_name as usize;
        assert_eq!(app.machine.mem[live + native::CRD_X_TARGET_OFF], sys_x as u8);
        assert_eq!(app.machine.mem[live + native::CRD_Y_TARGET_OFF], sys_y as u8);
        // Стрелка двигает курсор карты, а не корабль, и обновляет цель.
        // Движение ускоренное: за один кадр смещения может ещё не быть.
        app.key_event(KeyCode::ArrowRight, true, false);
        for _ in 0..9 {
            app.advance_hdmap_cursor();
        }
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
        assert_eq!(app.pending_key, None, "карта не должна отдавать клавиши");
        // Повторное F6 не закрывает карту и не сбрасывает её состояние.
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "F6 не должен закрывать карту");
        assert_eq!(app.hdmap.cursor(), (map_x, map_y));
        // Esc открывает меню паузы, но не закрывает карту.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню");
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");

        // Переход на другой нативный экран (F5) закрывает карту.
        app.key_event(KeyCode::F5, true, false);
        assert!(!app.hdmap.active, "F5 не переключил экран");
        assert!(app.shop.active, "F5 не открыл закупку");
        assert!(app.station_native, "закупка не оставила нативный режим");
        // Закрываем закупку переключением на вылет: управление уходит эмулятору.
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.shop.active, "F1 не закрыл закупку");
        assert!(!app.station_native, "F1 не передал управление эмулятору");
        assert!(app.station_emulator, "F1 не включил режим эмулятора");
        assert_eq!(app.machine.mem[flight::Z80_WHAT_ON_SCREEN], prev_screen);

        // После возврата в нативный режим карта открывается снова и курсор
        // остаётся на сохранённой цели гиперпрыжка.
        app.enter_native_station();
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

    /// В полёте F8 показывает цены нативно поверх симуляции, но стрелки не
    /// торгуют: экран остаётся только для просмотра, а ввод уходит эмулятору.
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
        assert!(
            !app.pending_market,
            "F8 в полёте не должен переключать экран Z80"
        );
        assert_eq!(app.pending_key, None, "F8 в полёте не должен отдавать K эмулятору");

        app.key_event(KeyCode::ArrowRight, true, false);
        assert_eq!(app.machine.mem[market_layout.quantities], 2);
        assert_eq!(app.machine.mem[p + 23], 0);
        assert_eq!(app.machine.mem[p + 40], 10);
        assert_eq!(
            u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]),
            1000
        );

        // Под нативным экраном цен стрелка — это управление кораблём, и в
        // эмулятор она не уходит.
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "стрелка дошла до эмулятора");
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

    /// F5 — нативный экран закупки оборудования на станции. Экран открывается
    /// сразу и не отправляет эмулятору ни `4`, ни `L`: все данные и покупки
    /// уже нативные. Выход по F5 и Esc запрещён; закрыть можно Fn-клавишей или
    /// клавишей возврата управления эмулятору.
    #[test]
    fn f5_opens_equip_shop_without_f5_or_esc_exit() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.shop.active);
        app.key_event(KeyCode::F5, true, false);
        assert!(app.shop.active, "F5 не открыл закупку сразу");
        assert!(app.station_native, "закупка не забрала управление");
        assert!(!app.pending_shop, "F5 оставил отложенную закупку");
        assert_eq!(app.pending_key, None, "F5 отправил клавишу в эмулятор");
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

        // Esc открывает меню паузы, но не закрывает закупку.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню паузы");
        assert!(app.shop.active, "Esc не должен закрывать экран закупки");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");
        app.key_event(KeyCode::F5, true, false);
        assert!(app.shop.active, "F5 не должен закрывать экран закупки");

        // Закрыть можно Fn-клавишей (F1 — вылет/вид).
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.shop.active, "F1 не закрыл экран закупки");
    }

    /// F7 — нативная альтернатива экрану информации о планете (`P`). На
    /// станции экран открывается сразу по живым данным, эмулятор заморожен и
    /// `P` ему не передаётся. Esc открывает меню паузы. В полёте сохраняется
    /// прежняя схема: оригинал сам переходит на экран `P`.
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
        assert!(!app.pending_sysinfo, "F7 оставил отложенный переход");
        // Первый вход на станцию из эмулятора отложен до клавиши `L`.
        assert!(app.pending_station_native.is_some());
        assert_eq!(app.pending_key, matrix_pos(KeyCode::KeyL));
        app.forward_input();
        let (lrow, lbit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(app.machine.keyboard[lrow] & (1 << lbit), 0);
        app.machine.run_frame();
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.finish_pending_station_native();
        assert!(app.station_native, "F7 не забрал управление у эмулятора");
        assert_eq!(app.pending_key, None, "F7 оставил отложенную клавишу");
        assert!(
            !app.sysinfo.selected.name.is_empty(),
            "данные планеты не прочитаны"
        );
        assert_eq!(
            app.game_state().planet,
            app.sysinfo.selected.name,
            "имя планеты должно приходить из состояния игры"
        );

        // Esc открывает меню паузы, нативный экран под ним остаётся.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню паузы");
        assert!(app.sysinfo.active, "Esc закрыл нативный экран");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");

        // В нативном режиме экран станции не зависит от экранной переменной
        // замороженного Z80 и не закрывается сам.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_native_screens();
        assert!(app.sysinfo.active, "нативный экран закрылся вместе с Z80");

        // Повторный F7 не отправляет `P` в эмулятор.
        app.key_event(KeyCode::F7, true, false);
        assert!(app.sysinfo.active, "повторный F7 не открыл экран");
        assert_eq!(app.pending_key, None, "F7 зря отправил P в эмулятор");

        // В полёте F7 показывает нативный экран информации о планете поверх
        // симуляции, без переключения экрана Z80.
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.key_event(KeyCode::F7, true, false);
        assert!(app.sysinfo.active, "F7 не открыл экран в полёте");
        assert!(
            !app.pending_sysinfo,
            "F7 в полёте не должен переключать экран Z80"
        );
        assert_eq!(app.pending_key, None, "F7 в полёте не должен отдавать P эмулятору");
    }

    /// F9 — нативная альтернатива экрану капитана (`L`). На станции экран
    /// открывается сразу, эмулятор заморожен и `L` ему не передаётся. В
    /// полёте это по-прежнему нативная отрисовка родного экрана, поэтому
    /// оригиналу уходит `L`.
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
        // На станции нативный режим уже показывает капитана. Уходим с этого
        // экрана, чтобы проверить повторное открытие по F9.
        app.close_status();
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        assert!(!app.status.active);
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не открыл нативный экран");
        assert!(!app.pending_status, "F9 оставил отложенный переход");
        // Возврат в нативный режим начинается с `L` в эмулятор.
        assert!(app.pending_station_native.is_some());
        assert_eq!(app.pending_key, matrix_pos(KeyCode::KeyL));
        app.forward_input();
        let (lrow, lbit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(app.machine.keyboard[lrow] & (1 << lbit), 0);
        app.machine.run_frame();
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.finish_pending_station_native();
        assert_eq!(app.pending_key, None, "F9 оставил отложенную клавишу");
        assert!(app.station_native, "F9 не забрал управление");

        // Ввод больше не перехватывается эмулятором.
        app.key_event(KeyCode::ArrowUp, true, false);
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "ввод ушёл в замороженный эмулятор");

        // Esc открывает меню паузы, нативный экран под ним остаётся.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc не открыл меню паузы");
        assert!(app.status.active, "Esc закрыл экран капитана");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");

        // Повторный F9 не закрывает экран.
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "повторный F9 закрыл экран");

        // В нативном режиме экран не зависит от экранной переменной Z80.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_native_screens();
        assert!(app.status.active, "нативный экран закрылся вместе с Z80");

        // В полёте F9 показывает нативный экран капитана поверх симуляции,
        // без переключения экрана Z80.
        let mut app = App::new(boot(), &opts);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не открыл нативный экран в полёте");
        assert!(
            !app.pending_status,
            "F9 в полёте не должен переключать экран Z80"
        );
        assert!(!app.flight.active, "нативный вид остался под экраном");
        assert_eq!(app.pending_key, None, "F9 в полёте не должен отдавать L эмулятору");
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
        // Esc открыл меню: закрываем его, чтобы F4 дошёл до переключения
        // экрана, а не до пунктов меню.
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "повторный Esc не закрыл меню");
        // F4 — переход к другому экрану (виду вправо), он закрывает карту.
        app.key_event(KeyCode::F4, true, false);
        assert!(!app.hdmap.active, "F4 не переключил экран");
        assert!(app.flight.active, "закрытие карты выключило полёт");

        // Esc — пауза, но экран остаётся выбранным; Esc снова — продолжить.
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu && app.flight.active, "Esc не открыл меню");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu && app.flight.active, "Esc не вернул в полёт");

        // `Backquote` — назад к Z80-кадру.
        app.key_event(KeyCode::Backquote, true, false);
        assert!(!app.flight.active, "Backquote не закрыл экран полёта");

        // F1 снова включает экран полёта (и остаётся включённым).
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        app.key_event(KeyCode::F1, true, false);
        assert!(
            app.flight.active,
            "F1 должен включать рендер, а не закрывать"
        );
    }

    /// В режиме эмулятора полёта буквенные клавиши уходят в игру. Нативный
    /// экран открывает только повторное нажатие, когда оригинал уже показывает
    /// этот экран; Fn-клавиши продолжают работать сразу.
    #[test]
    fn flight_emulator_letters_go_to_game_until_repeated() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.sync_state();

        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active, "подготовка: нативный полёт не открылся");
        app.key_event(KeyCode::Backquote, true, false);
        assert!(!app.flight.active, "Backquote не скрыл нативный полёт");
        assert!(app.flight_emulator, "Backquote не включил экран эмулятора");
        app.sync_state();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");

        // Первое нажатие уходит в игру и не открывает нативный экран.
        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.status.active, "L сразу открыл нативный экран");
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "L не дошёл до эмулятора"
        );
        app.key_event(KeyCode::KeyL, false, false);

        // Оригинал сам перешёл на экран капитана; повторное L берём себе.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.sync_state();
        app.key_event(KeyCode::KeyL, true, false);
        assert!(app.status.active, "повторный L не открыл нативный экран");
        app.apply_input();
        assert_ne!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "L ушёл в игру при открытии нативного экрана"
        );
        app.key_event(KeyCode::KeyL, false, false);

        // Следующее нажатие снимает надстройку и снова отдаётся игре.
        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.status.active, "L не закрыл нативный экран");
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "L не ушёл в игру после закрытия нативного экрана"
        );
        app.key_event(KeyCode::KeyL, false, false);

        // Fn-клавиша открывает натив сразу, без ожидания повторного нажатия.
        app.key_event(KeyCode::F9, true, false);
        assert!(app.status.active, "F9 не открыл нативный экран из эмулятора");
    }

    /// На станции в режиме эмулятора оригинальные клавиши ведут себя так же:
    /// сначала уходят в игру, а нативный экран открывает повторное нажатие.
    #[test]
    fn station_emulator_letters_open_native_on_repeat() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.sync_state();
        app.handoff_to_emulator();
        assert!(app.station_emulator, "подготовка: эмулятор не включён");
        app.sync_state();

        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.status.active, "L сразу открыл нативный экран");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "L не дошёл до эмулятора станции"
        );
        app.key_event(KeyCode::KeyL, false, false);

        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        app.sync_state();
        app.key_event(KeyCode::KeyL, true, false);
        assert!(app.status.active, "повторный L не открыл нативный экран");
        assert!(
            app.pending_station_native.is_some(),
            "вход в нативный режим станции не отложен до кадра"
        );
    }

    /// Кнопки управления кораблём уходят в эмулятор из нативного вида полёта
    /// и с экрана эмулятора в полёте. Под нативной картой Z80 продолжает
    /// работать, но стрельба и движение корабля должны быть остановлены.
    #[test]
    fn ship_controls_reach_emulator_in_flight_views() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.sync_state();

        let (fire_row, fire_bit) = matrix_pos(KeyCode::Space).expect("Space в матрице");
        let (pitch_row, pitch_bit) = matrix_pos(KeyCode::KeyS).expect("S в матрице");

        // Нативный полёт: управление идёт в игру.
        app.key_event(KeyCode::F1, true, false);
        app.key_event(KeyCode::Space, true, false);
        app.key_event(KeyCode::KeyS, true, false);
        app.apply_input();
        assert_eq!(app.machine.keyboard[fire_row] & (1 << fire_bit), 0);
        assert_eq!(app.machine.keyboard[pitch_row] & (1 << pitch_bit), 0);
        app.key_event(KeyCode::Space, false, false);
        app.key_event(KeyCode::KeyS, false, false);

        // Экран эмулятора в полёте: управление кораблём тоже идёт в игру.
        app.key_event(KeyCode::Backquote, true, false);
        assert!(app.flight_emulator && !app.flight.active);
        app.key_event(KeyCode::Space, true, false);
        app.key_event(KeyCode::ArrowUp, true, false);
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[fire_row] & (1 << fire_bit),
            0,
            "огонь не дошёл до эмулятора"
        );
        assert_ne!(app.machine.kempston, 0, "джойстик не дошёл до эмулятора");
        app.key_event(KeyCode::Space, false, false);
        app.key_event(KeyCode::ArrowUp, false, false);

        // Вернуться в нативный полёт и открыть карту поверх него: под
        // информационным экраном кнопки управления в игру не идут.
        app.key_event(KeyCode::F1, true, false);
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active, "подготовка: карта не открылась");
        app.key_event(KeyCode::Space, true, false);
        app.key_event(KeyCode::ArrowUp, true, false);
        app.apply_input();
        assert_ne!(
            app.machine.keyboard[fire_row] & (1 << fire_bit),
            0,
            "огонь ушёл в эмулятор под картой"
        );
        assert_eq!(
            app.machine.kempston, 0,
            "джойстик ушёл в эмулятор под картой"
        );
        assert!(
            app.matrix[fire_row][fire_bit as usize],
            "карта должна сама видеть удержанный Space"
        );
    }

    /// Кнопка огня доходит до Z80 в каждом из четырёх видов кабины: лазер может
    /// стоять в любом креплении, и нативный слой не должен фильтровать ввод.
    #[test]
    fn fire_key_reaches_emulator_in_all_cockpit_views() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.sync_state();
        app.key_event(KeyCode::F1, true, false);

        let (fire_row, fire_bit) = matrix_pos(KeyCode::KeyA).expect("A в матрице");
        for view in [KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4] {
            app.key_event(view, true, false);
            app.key_event(KeyCode::KeyA, true, false);
            app.apply_input();
            assert_eq!(
                app.machine.keyboard[fire_row] & (1 << fire_bit),
                0,
                "огонь не дошёл до Z80 в виде {view:?}"
            );
            app.key_event(KeyCode::KeyA, false, false);
        }
    }

    /// На экране эмулятора в полёте кнопки управления кораблём уходят в игру,
    /// как и обычные игровые клавиши.
    #[test]
    fn emulator_screen_passes_ship_controls_in_flight() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.sync_state();
        app.key_event(KeyCode::F1, true, false);
        app.key_event(KeyCode::Backquote, true, false);
        assert!(
            app.flight_emulator && !app.flight.active,
            "подготовка: экран эмулятора не включён"
        );
        app.sync_state();

        app.key_event(KeyCode::Space, true, false);
        app.key_event(KeyCode::ArrowUp, true, false);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Space).expect("Space в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "огонь не дошёл до эмулятора"
        );
        assert_ne!(app.machine.kempston, 0, "джойстик не дошёл до эмулятора");

        // Обычная игровая клавиша продолжает доходить до Z80.
        app.key_event(KeyCode::KeyL, true, false);
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "L не дошёл до эмулятора"
        );
    }

    /// На станции F1 отдаёт эмулятору вылет (клавиша 1), а F5 открывает
    /// нативную закупку сразу, без передачи клавиш Z80. F2-F4 остаются
    /// пустыми: их цифры не уходят в эмулятор.
    #[test]
    fn function_keys_send_original_number_keys() {
        let opts = test_options();
        // F1 -> 1
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.station_native, "F1 не передал управление эмулятору");
        assert!(app.station_emulator, "F1 не включил режим эмулятора");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 не передал игре Digit1"
        );

        // F5 — нативная закупка, без клавиши L в эмулятор.
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.key_event(KeyCode::F5, true, false);
        assert!(app.shop.active, "F5 не открыл закупку");
        assert!(app.station_native, "F5 не оставил станцию в нативном режиме");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_ne!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F5 не должен передавать игре KeyL"
        );
        assert_eq!(app.pending_key, None, "F5 оставил отложенную клавишу");

        for fkey in [KeyCode::F2, KeyCode::F3, KeyCode::F4] {
            let mut app = App::new(boot(), &opts);
            app.machine.intro_seen = false;
            app.machine.mem[flight::Z80_IND_DANGER] = 0;
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
            app.key_event(fkey, true, false);
            assert!(app.station_native, "{fkey:?} не должен отдавать управление");
            assert!(!app.station_emulator, "{fkey:?} включил эмулятор");
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

    /// На нативной станции F1 с открытой карты запускает вылет и отдаёт
    /// эмулятору клавишу 1. F5 переключает на нативную закупку сразу. F2-F4
    /// остаются пустыми.
    #[test]
    fn map_function_keys_forward_original_keys() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::F1, true, false);
        assert!(!app.hdmap.active, "F1 не закрыл карту");
        assert!(!app.station_native, "F1 не передал управление эмулятору");
        assert!(app.station_emulator, "F1 не включил режим эмулятора");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::Digit1).expect("1 в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "F1 на карте не передал игре клавишу 1"
        );

        // F5 закрывает карту и открывает нативную закупку без передачи L.
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::F5, true, false);
        assert!(!app.hdmap.active, "F5 не закрыл карту");
        assert!(app.shop.active, "F5 не открыл закупку");
        assert!(app.station_native, "F5 не оставил нативный режим");
        assert_eq!(app.pending_key, None, "F5 передал клавишу эмулятору");

        // F2-F4 на станции не передают цифры; открытая карта при этом
        // остаётся — эти клавиши здесь просто не реализованы.
        for fkey in [KeyCode::F2, KeyCode::F3, KeyCode::F4] {
            let mut app = App::new(boot(), &opts);
            app.machine.intro_seen = false;
            app.machine.mem[flight::Z80_IND_DANGER] = 0;
            app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
            app.key_event(KeyCode::F6, true, false);
            assert!(app.hdmap.active);
            app.key_event(fkey, true, false);
            assert!(app.hdmap.active, "{fkey:?} закрыл карту");
            assert!(app.station_native, "{fkey:?} отдал управление эмулятору");
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

    /// Холодный старт: снапшот уже имеет `IndDanger = 0`, но игра должна
    /// начинаться на заставке. Нативный режим станции включается только после
    /// `Space` и выхода Z80 из заставки. Конфигурация сохранения ставится до
    /// запуска игры, чтобы на станции можно было сразу сохраниться.
    #[test]
    fn cold_start_shows_intro_and_space_enters_station() {
        let opts = test_options();
        let mut app = App::new(crate::boot::machine(&opts), &opts);
        let layout = save_layout_kladov();
        assert!(
            app.machine.hooks.contains_key(&layout.menu_save_load),
            "конфигурация сохранения должна быть установлена до старта"
        );
        assert!(
            app.machine.intro_seen,
            "холодный старт должен находиться в режиме заставки"
        );
        assert!(
            !app.station_native,
            "станция не должна захватывать управление до заставки"
        );
        assert_eq!(app.game_state().screen, Screen::Intro);

        // Без `Space` игра остаётся на заставке, сколько её ни прогоняй.
        for _ in 0..120 {
            app.machine.run_frame();
        }
        app.maybe_enter_native_station();
        app.sync_state();
        assert!(app.machine.intro_seen, "заставка самовольно закончилась");
        assert_eq!(app.game_state().screen, Screen::Intro);
        assert!(!app.station_native);

        // `Space` уводит Z80 на ветку старта; после `GameMain` станция сама
        // переходит в нативный режим.
        app.key_event(KeyCode::Space, true, false);
        let mut started = false;
        for _ in 0..5000 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                started = true;
                break;
            }
        }
        assert!(started, "`Space` не запустил игру");
        app.maybe_enter_native_station();
        assert!(
            app.pending_station_native.is_some(),
            "перед заморозкой Z80 должен быть запрос на `L`"
        );
        assert_eq!(
            app.pending_key,
            matrix_pos(KeyCode::KeyL),
            "в эмулятор должна уйти клавиша `L`"
        );
        // как кадр с `L` делает `frame()`: сначала ввод в матрицу, затем Z80
        app.forward_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "`L` не дошёл до эмулятора"
        );
        app.machine.run_frame();
        app.open_pending_status();
        app.finish_pending_station_native();
        app.sync_state();
        assert!(
            app.station_native,
            "после `Space` станция должна стать нативной"
        );
        assert!(
            app.status.active || app.market.native || app.hdmap.active || app.shop.active,
            "на станции должен быть открыт нативный экран"
        );
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

    /// В нативном режиме станции оригинальные клавиши открывают нативные
    /// экраны, а единственный выход в эмулятор — `Backquote`. Клавиши вне
    /// станционного списка не попадают в замороженный Z80, а управление
    /// нативных экранов продолжает работать локально.
    #[test]
    fn station_keys_open_native_screens_and_backquote_leaves_emulator() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;

        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);
        assert!(app.station_native);

        // `T` — игровая клавиша вне станционного списка: в нативном режиме
        // она не должна закрывать карту или уходить в эмулятор.
        app.key_event(KeyCode::KeyT, true, false);
        assert!(app.hdmap.active, "не-станционная клавиша закрыла карту");
        assert!(app.station_native, "не-станционная клавиша отдала управление");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyT).expect("T в матрице");
        assert_ne!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "не-станционная клавиша ушла в замороженный эмулятор"
        );
        app.key_event(KeyCode::KeyT, false, false);

        // Управляющие клавиши карты продолжают работать.
        app.key_event(KeyCode::ArrowRight, true, false);
        assert!(app.hdmap.active, "стрелка не должна закрывать карту");
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc должен открывать меню");
        assert!(app.hdmap.active, "Esc не должен закрывать карту");
        app.key_event(KeyCode::Escape, true, false);
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

        // `L` — нативная альтернатива экрана капитана: карта закрывается,
        // управление остаётся у нативного режима станции.
        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.hdmap.active, "L не закрыл карту");
        assert!(app.status.active, "L не открыл экран капитана");
        assert!(app.station_native, "L отдал управление эмулятору");
        assert!(!app.station_emulator, "L включил эмулятор");
        assert_eq!(app.pending_key, None, "L оставил отложенную клавишу");

        // `O` — тот же нативный экран карты в детальном масштабе.
        app.key_event(KeyCode::KeyO, true, false);
        assert!(app.hdmap.active, "O не открыл карту");
        assert!(!app.status.active, "O не закрыл экран капитана");
        assert_eq!(app.hdmap.mode(), hdmap::MapMode::Local);
        assert!(app.station_native, "O отдал управление эмулятору");

        // `Backquote` — единственный выход: все нативные экраны закрываются,
        // эмулятор снимается с паузы без единой игровой клавиши.
        app.key_event(KeyCode::Backquote, true, false);
        assert!(!app.hdmap.active, "Backquote не закрыл карту");
        assert!(!app.station_native, "Backquote не снял нативный режим");
        assert!(app.station_emulator, "Backquote не включил эмулятор");
        assert_eq!(app.pending_key, None, "Backquote отправил клавишу в игру");
    }

    /// Открытие нативного экрана из эмулятора сначала посылает оригиналу `L`.
    /// Только после этого Z80 замораживается, а только что открытый экран
    /// остаётся активным.
    #[test]
    fn opening_native_station_screen_sends_l_first() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        // Имитируем режим покупки: `L` должен вывести оригинал отсюда.
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 4;
        app.handoff_to_emulator();
        assert!(app.station_emulator);
        assert!(!app.station_native);

        app.key_event(KeyCode::F8, true, false);
        assert!(app.market.native, "F8 не открыл нативный рынок");
        assert!(
            !app.station_native,
            "нативный режим не должен замораживать Z80 до `L`"
        );
        assert!(app.pending_station_native.is_some());
        assert_eq!(
            app.pending_key,
            matrix_pos(KeyCode::KeyL),
            "в эмулятор должна уйти клавиша `L`"
        );

        app.forward_input();
        let (row, bit) = matrix_pos(KeyCode::KeyL).expect("L в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "`L` не дошёл до эмулятора"
        );
        app.machine.run_frame();
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
        // Пока `L` не обработан, смена экрана Z80 не должна закрыть экран.
        app.follow_native_screens();
        assert!(app.market.native, "рынок закрылся до входа в нативный режим");

        app.finish_pending_station_native();
        assert!(app.station_native, "нативный режим не включился после `L`");
        assert!(!app.station_emulator);
        assert!(app.market.native, "вход в станцию закрыл открытый экран");
        assert_eq!(app.pending_key, None);
    }

    /// Оригинальные станционные цифры и буквы открывают те же нативные
    /// экраны, что и F-клавиши, и не передают ничего в эмулятор.
    #[test]
    fn station_keys_map_to_native_screens() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;

        for code in [KeyCode::Digit2, KeyCode::Digit3, KeyCode::KeyK] {
            app.key_event(code, true, false);
            assert!(app.market.native, "{code:?} не открыл рынок");
            assert!(app.station_native, "{code:?} отдал управление эмулятору");
            assert_eq!(app.pending_key, None, "{code:?} отправил клавишу в игру");
            app.key_event(code, false, false);
        }

        app.key_event(KeyCode::Digit4, true, false);
        assert!(app.shop.active, "4 не открыл закупку");
        assert!(!app.market.native, "4 не закрыл рынок");
        assert!(app.station_native, "4 отдал управление эмулятору");
        app.key_event(KeyCode::Digit4, false, false);

        app.key_event(KeyCode::KeyP, true, false);
        assert!(app.sysinfo.active, "P не открыл данные о планете");
        assert!(!app.shop.active, "P не закрыл закупку");
        app.key_event(KeyCode::KeyP, false, false);

        app.key_event(KeyCode::KeyI, true, false);
        assert!(app.hdmap.active, "I не открыл карту");
        assert_eq!(app.hdmap.mode(), hdmap::MapMode::Galaxy);
        assert!(!app.sysinfo.active, "I не закрыл данные о планете");
        app.key_event(KeyCode::KeyI, false, false);

        app.key_event(KeyCode::KeyO, true, false);
        assert_eq!(
            app.hdmap.mode(),
            hdmap::MapMode::Local,
            "O не переключил масштаб карты"
        );
        app.key_event(KeyCode::KeyO, false, false);

        // `Left Ctrl` — родная клавиша паузы: открывается нативное меню.
        app.key_event(KeyCode::ControlLeft, true, false);
        assert!(app.menu, "Left Ctrl не открыл меню паузы");
        assert!(app.station_native, "Left Ctrl отдал управление эмулятору");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "Esc не закрыл меню");
    }

    /// Right Shift сохраняет блок командира нативно: эмулятор остаётся
    /// замороженным, а файл появляется без передачи клавиши в Z80.
    #[test]
    fn right_shift_saves_natively_on_station() {
        let path = env::temp_dir().join("elite-station-save-test.zxelite");
        let _ = fs::remove_file(&path);
        let mut opts = test_options();
        opts.save = Some(path.to_string_lossy().into_owned());

        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.maybe_enter_native_station();
        assert!(app.station_native, "подготовка: станция не в нативном режиме");

        app.key_event(KeyCode::ShiftRight, true, false);
        assert!(path.exists(), "Right Shift не создал файл сохранения");
        assert!(app.station_native, "Right Shift отдал управление эмулятору");
        assert!(!app.station_emulator, "Right Shift включил эмулятор");
        assert_eq!(app.pending_key, None, "Right Shift отправил клавишу в игру");
        fs::remove_file(&path).ok();
    }

    /// На станции `G`/`H` теперь передаются эмулятору: нативный режим
    /// станции снимается, а клавиша уходит в игру.
    #[test]
    fn g_and_h_on_station_handoff_to_emulator() {
        let opts = test_options();
        for key in [KeyCode::KeyG, KeyCode::KeyH] {
            let mut app = App::new(boot(), &opts);
            app.machine.intro_seen = false;
            app.machine.mem[flight::Z80_IND_DANGER] = 0;
            app.key_event(KeyCode::F6, true, false);
            assert!(app.hdmap.active, "подготовка: карта не открылась");

            let (row, bit) = matrix_pos(key).expect("клавиша в матрице");
            app.key_event(key, true, false);
            assert!(
                !app.docked.is_active(),
                "{key:?} не должен показывать заглушку DOCKED"
            );
            assert!(!app.station_native, "{key:?} не снял нативный режим станции");
            assert!(app.station_emulator, "{key:?} не передал управление эмулятору");
            assert!(!app.hdmap.active, "{key:?} не закрыл нативный экран");
            assert_eq!(app.pending_key, Some((row, bit)), "{key:?} не ушёл в эмулятор");
        }
    }

    /// В нативном режиме станции только Left Shift (в оригинале — вход в
    /// паузу) показывает временную заглушку. Клавиши настроек из
    /// `kladov_keys.md` сами по себе заглушку не открывают и в эмулятор не
    /// уходят. Следующее нажатие закрывает заглушку и обрабатывается штатно.
    #[test]
    fn pause_key_shows_stub_on_station() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.maybe_enter_native_station();

        assert!(app.station_native, "подготовка: станция не в нативном режиме");
        assert!(!app.pause_stub, "заглушка паузы уже показана");

        // Клавиши настроек из `kladov_keys.md` больше не реагируют на заглушку.
        for key in [
            KeyCode::KeyY,
            KeyCode::KeyB,
            KeyCode::KeyC,
            KeyCode::KeyR,
            KeyCode::KeyF,
            KeyCode::KeyQ,
            KeyCode::KeyS,
            KeyCode::Space,
        ] {
            app.key_event(key, true, false);
            assert!(!app.pause_stub, "{key:?} открыл заглушку паузы");
            assert!(app.station_native, "{key:?} отдал управление эмулятору");
        }

        // Left Shift включает паузу в оригинале — он один открывает заглушку.
        app.key_event(KeyCode::ShiftLeft, true, false);
        assert!(app.pause_stub, "Shift не показал заглушку паузы");
        assert!(app.station_native, "заглушка паузы отдала управление эмулятору");
        assert!(!app.station_emulator, "заглушка паузы включила эмулятор");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::ShiftLeft).expect("Shift в матрице");
        assert_ne!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "Shift ушёл в замороженный эмулятор"
        );
        assert_eq!(app.pending_key, None, "Shift оставил pending_key");

        // Следующее нажатие закрывает заглушку и обрабатывается штатно.
        app.key_event(KeyCode::KeyY, true, false);
        assert!(!app.pause_stub, "следующее нажатие не закрыло заглушку");
        assert!(app.station_native, "следующее нажатие отдало управление");

        // Повторный Shift снова показывает заглушку.
        app.key_event(KeyCode::ShiftLeft, true, false);
        assert!(app.pause_stub, "повторный Shift не показал заглушку");
    }

    /// В нативном режиме станции эмулятор заморожен: ввод не передаётся,
    /// отложенные клавиши очищаются, кадры Z80 не исполняются.
    #[test]
    fn station_native_freezes_emulator_and_clears_input() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 0;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0x0B;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.maybe_enter_native_station();
        assert!(app.station_native, "подготовка: станция не в нативном режиме");

        let frames = app.machine.frames;
        let pc = app.machine.cpu.pc;
        let sp = app.machine.cpu.sp;

        app.matrix[1][1] = true; // S
        app.kempston = [true, true, true, true, true];
        app.pending_key = Some((1, 1));

        app.apply_input();
        assert_eq!(app.pending_key, None, "pending_key не очищен в нативном режиме");
        assert_eq!(app.machine.kempston, 0, "джойстик ушёл в замороженный эмулятор");
        for (row, &bits) in app.machine.keyboard.iter().enumerate() {
            assert_eq!(bits, 0xFF, "ряд {row} клавиатуры ушёл в замороженный эмулятор");
        }

        // Без прогона кадров состояние процессора/эмулятора не меняется.
        assert_eq!(app.machine.frames, frames, "кадры Z80 изменились без прогона");
        assert_eq!(app.machine.cpu.pc, pc, "PC изменился без прогона Z80");
        assert_eq!(app.machine.cpu.sp, sp, "SP изменился без прогона Z80");
    }

    /// Гиперпрыжок, начатый под нативным экраном: отсчёт идёт в эмуляторе,
    /// экран не закрывается, а к началу тоннеля включается вид F1 с нативной
    /// анимацией прыжка.
    #[test]
    fn jump_countdown_under_native_screen_switches_to_f1() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_MARKET;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.key_event(KeyCode::F8, true, false);
        assert!(app.market.native);

        // Z80 начал отсчёт, но его экранная переменная уже ушла в кабину.
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0x15;
        app.track_jump_countdown();
        assert!(app.saw_jump_countdown);
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.follow_native_screens();
        app.switch_to_flight_on_jump();
        assert!(app.market.native, "рынок закрылся во время отсчёта");
        assert!(!app.flight.active, "вид F1 открылся до конца отсчёта");

        // Счётчик обнулился — пора переключаться на анимацию гиперпрыжка.
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;
        app.switch_to_flight_on_jump();
        assert!(app.flight.active, "не открылся нативный вид F1");
        assert!(!app.market.native, "рынок остался поверх полёта");
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "переход показан не как гиперпрыжок"
        );
    }

    /// `F1` после окончания отсчёта не должен принимать прыжок за вылет/стыковку:
    /// после обнуления `JumpCountdown` флаги Z80 у них одинаковые.
    #[test]
    fn f1_after_jump_countdown_keeps_hyperspace_kind() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 1;
        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0x15;
        app.track_jump_countdown();

        app.machine.mem[flight::Z80_JUMP_COUNTDOWN] = 0;
        app.key_event(KeyCode::F1, true, false);
        assert!(app.flight.active);
        assert_eq!(
            app.flight.transition_kind(),
            Some(flight::TransitionKind::Hyperspace),
            "гиперпрыжок перепутан со стыковкой"
        );
    }

    /// Под картой `G` доходит до эмулятора одной клавишей, не закрывая карту;
    /// сам тоннель переключит экран на `F1`.
    #[test]
    fn jump_key_under_map_keeps_map_open() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.mem[flight::Z80_IND_DANGER] = 1;
        app.machine.mem[flight::Z80_WHAT_ON_SCREEN] = 0;
        app.machine.mem[flight::Z80_JUMP_DOCKING] = 0;
        app.machine.mem[flight::Z80_FLG_JUMP_DOCKING] = 0;
        app.key_event(KeyCode::F6, true, false);
        assert!(app.hdmap.active);

        app.key_event(KeyCode::KeyG, true, false);
        assert!(app.hdmap.active, "карта закрылась при нажатии G");
        let (row, bit) = matrix_pos(KeyCode::KeyG).expect("G в матрице");
        assert_eq!(app.pending_key, Some((row, bit)), "G не дошёл до эмулятора");
        app.forward_pending_key();
        assert_eq!(app.machine.keyboard[row] & (1 << bit), 0);
        assert_eq!(app.pending_key, None);
    }

    /// Индикатор ускорения эмуляции появляется только при `F12` и занимает
    /// клетку слева от радара, зеркальную паре `ES` справа.
    #[test]
    fn speed_indicator_shows_only_when_accelerated() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        let (w, h) = (1024usize, 768usize);
        app.machine.hd.resize(w, h);
        app.machine.hd.clear(display::TRANSPARENT);
        app.draw_speed_indicator();
        assert!(
            app.machine.hd.pixels().iter().all(|&p| p == display::TRANSPARENT),
            "индикатор нарисован без ускорения"
        );

        app.accelerated = true;
        app.draw_speed_indicator();
        let g = cockpit::panel_geometry(w as u32, h as u32);
        let speed = g.speed_cell();
        let e = g.ecm_cell();
        let s = g.station_cell();
        let es = cockpit::Rect {
            x: e.x,
            y: e.y,
            w: s.x + s.w - e.x,
            h: e.h,
        };
        assert!(
            es.x > g.center_cell().x + g.center_cell().w / 2,
            "подготовка: пара ES не справа от центра"
        );
        assert!(
            speed.x + speed.w <= g.center_cell().x + g.center_cell().w / 2,
            "индикатор скорости не слева от радара"
        );
        assert_eq!(speed.y, es.y, "индикатор скорости не симметричен ES по высоте");
        assert_eq!(speed.h, es.h, "размер индикатора скорости отличается от ES");
        let lit_in = |r: cockpit::Rect| {
            app.machine.hd.pixels().iter().enumerate().any(|(i, &p)| {
                let (x, y) = (i % w, i / w);
                p != display::TRANSPARENT
                    && x >= r.x.max(0) as usize
                    && x < (r.x + r.w).max(0) as usize
                    && y >= r.y.max(0) as usize
                    && y < (r.y + r.h).max(0) as usize
            })
        };
        assert!(lit_in(speed), "индикатор x2 не нарисован в своей клетке");
        let top = cockpit::Rect {
            x: 0,
            y: 0,
            w: w as i32,
            h: (g.panel_y - g.margin).max(0),
        };
        assert!(!lit_in(top), "индикатор ушёл в верхнюю часть экрана");
    }
