//! Разбор аргументов командной строки.

use std::env;
use std::fs;
use std::process::exit;

pub struct Options {
    pub game: String,
    pub rom: Option<String>,
    pub headless: bool,
    pub frames: u64,
    pub dump: Option<String>,
    /// Отключить звук (PipeWire).
    pub mute: bool,
    /// Включить звук и в headless-режиме (реальное время).
    pub audio: bool,
    /// Сохранить сведённый звук в WAV-файл.
    pub wav: Option<String>,
    /// Включить нативные (Rust) замены подпрограмм игры вместо Z80.
    pub native: bool,
    /// Фиксированный путь сохранения командира. `None` — спрашивать через
    /// нативный диалог выбора файла (по умолчанию, расширение `.zxelite`).
    pub save: Option<String>,
    /// Запустить игру сразу в полноэкранном режиме (borderless).
    /// Переключение во время игры: F11 или Alt+Enter.
    pub fullscreen: bool,
}

pub fn parse_args() -> Options {
    let mut opts = Options {
        game: String::new(),
        rom: None,
        headless: false,
        frames: 600,
        dump: None,
        mute: false,
        audio: false,
        wav: None,
        native: false,
        save: None,
        fullscreen: false,
    };
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--rom" => opts.rom = args.next(),
            "--frames" => opts.frames = args.next().and_then(|s| s.parse().ok()).unwrap_or(600),
            "--dump" => {
                opts.dump = args.next();
                opts.headless = true;
            }
            "--headless" => opts.headless = true,
            "--mute" => opts.mute = true,
            "--audio" => opts.audio = true,
            "--native" => opts.native = true,
            "--wav" => opts.wav = args.next(),
            "--save" => opts.save = args.next(),
            "--fullscreen" => opts.fullscreen = true,
            "-h" | "--help" => {
                println!(
                    "usage: elite [game.bin] [--rom rom.bin] [--headless] [--frames N] [--dump out.ppm] [--mute] [--audio] [--wav out.wav] [--native] [--save FILE] [--fullscreen]"
                );
                println!("  загружается версия Кладова (zx_bin/kladov.bin)");
                println!("  --mute     отключить вывод звука через PipeWire");
                println!("  --audio    выводить звук и в headless-режиме (реальное время)");
                println!("  --wav FILE сохранить звук в WAV-файл");
                println!("  --native   включить нативные (Rust) замены подпрограмм");
                println!("  --save FILE фиксированный файл командира вместо диалога");
                println!("  --fullscreen запуск в полноэкранном режиме (borderless)");
                println!("В игре: F1-F4 — оригинальные клавиши 1-4 (F1 включает");
                println!("        нативный вид полёта после вылета, F4 на станции —");
                println!("        нативную закупку), F10 — карта галактики,");
                println!("        Esc — меню паузы, F11 — полный экран, F12 — ускорение x2.");
                println!();
                println!("Заставка нативная: [SPACE] — начать игру, [Esc] — меню");
                println!("  паузы (на заставке только загрузка, сохранение без игры");
                println!("  отключено).");
                println!();
                println!("Экран: ZX-кадр 256x192 показывается целым множителем (чёткие");
                println!("квадратные пиксели) по центру; нативные (Rust) реализации");
                println!("рисуют в HD-слой поверх него в полном разрешении окна.");
                println!();
                println!("Управление:");
                println!("  Esc        меню паузы (продолжить/сохранить/загрузить/выход)");
                println!("  F1-F4      клавиши 1-4 оригинала; F1 включает нативный");
                println!("             вид полёта (на станции — после вылета), F4 на");
                println!("             станции — нативную закупку");
                println!("  F10        нативная карта галактики");
                println!("  F11, Alt+Enter полный экран");
                println!("  F12        ускорение эмуляции Z80 вдвое (вкл/выкл)");
                println!("  Ctrl+Q     выход из игры");
                println!();
                println!("Заставка: SPACE запускает игру, остальные клавиши не срабатывают.");
                println!();
                println!("Карта галактики (F10): стрелки/NM/XS — курсор, Enter —");
                println!("  ближайшая планета, +/- или колесо — масштаб, Esc — закрыть;");
                println!("  F1-F4 — клавиши 1-4 игры (F1 ещё и включает вид полёта),");
                println!("  Ctrl+F1 — чит: телепорт на станцию выбранной планеты.");
                println!();
                println!("Экран полёта (F1): управление как в оригинале — S/X или");
                println!("  ↑/↓ — тангаж, N/M или ←/→ — крен, Space — ускорение,");
                println!("  Shift — торможение, A/RightCtrl — огонь; Esc — меню паузы,");
                println!("  F1-F4 — виды вперёд/назад/влево/вправо (клавиши 1-4),");
                println!("  1 — назад к ZX-кадру. Переходы игры (O, Enter и т.п.)");
                println!("  закрывают нативный вид и показывают экран оригинала.");
                println!();
                println!("Закупка оборудования (F4): клик/Enter — купить,");
                println!("  1-9 (+вторая цифра) — номер товара, стрелки — выбор,");
                println!("  0-3 — крепление лазера, Esc — закрыть.");
                exit(0);
            }
            other => opts.game = other.to_string(),
        }
    }
    if opts.game.is_empty() {
        opts.game = "zx_bin/kladov.bin".to_string();
    }
    if opts.rom.is_none() {
        for cand in [
            "zx_bin/48.rom",
            "/usr/share/fuse/48.rom",
            "/usr/share/fuse/48.ZX.rom",
        ] {
            if fs::metadata(cand).is_ok() {
                opts.rom = Some(cand.to_string());
                break;
            }
        }
    }
    opts
}
