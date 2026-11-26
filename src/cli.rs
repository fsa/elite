//! Разбор аргументов командной строки.

use std::env;
use std::process::exit;

/// Версия сборки. В релизных сборках CI задаёт `ELITE_VERSION` из имени тега;
/// при обычной локальной сборке используется версия из `Cargo.toml`.
pub const VERSION: &str = match option_env!("ELITE_VERSION") {
    Some(tag) => tag,
    None => env!("CARGO_PKG_VERSION"),
};

pub struct Options {
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
    /// Стартовый файл commander-блока ровно на 102 байта. При наличии он
    /// импортируется в каталог и загружается вместо заставки. Обычные SAVE/LOAD
    /// в игре работают только с каталогом FSA/Elite.
    pub save: Option<String>,
    /// Запустить игру сразу в полноэкранном режиме (borderless).
    /// Переключение во время игры: F11 или Alt+Enter.
    pub fullscreen: bool,
    /// Режим отладки: справа от игры показывается полоса 256 пикселей с
    /// кадром эмулятора 1:1. Под миниатюрой — кнопки «1 SUMMARY»/«2 HEX DUMP»:
    /// сокращённая сводка по полям блока командира или полный дамп байтов.
    pub debug: bool,
}

pub fn parse_args() -> Options {
    let mut opts = Options {
        headless: false,
        frames: 600,
        dump: None,
        mute: false,
        audio: false,
        wav: None,
        native: false,
        save: None,
        fullscreen: false,
        debug: false,
    };
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
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
            "--debug" => opts.debug = true,
            "-V" | "--version" => {
                println!("elite {VERSION}");
                exit(0);
            }
            "-h" | "--help" => {
                println!(
                    "usage: elite [--headless] [--frames N] [--dump out.ppm] [--mute] [--audio] [--wav out.wav] [--native] [--save FILE] [--fullscreen] [--debug] [--version]"
                );
                println!("  -V, --version показать версию сборки (тег релиза)");
                println!("  --mute     отключить вывод звука через PipeWire");
                println!("  --audio    выводить звук и в headless-режиме (реальное время)");
                println!("  --wav FILE сохранить звук в WAV-файл");
                println!("  --native   включить нативные (Rust) замены подпрограмм");
                println!("  --save FILE загрузить при старте 102-байтовый файл commander-блока");
                println!(
                    "             (импорт в каталог FSA/Elite и сразу на станцию, без заставки)"
                );
                println!("  --fullscreen запуск в полноэкранном режиме (borderless)");
                println!("  --debug    отладка: справа полоса 256 px с кадром");
                println!("             эмулятора 1:1; под ним кнопки SUMMARY/HEX DUMP;");
                println!("             стрелки на заставке перебирают все уникальные");
                println!("             модели из shipdata; название модели над нижней надписью");
                println!("В игре: F1-F4 — оригинальные клавиши 1-4 (виды, F1 включает");
                println!("        нативный вид полёта после вылета; F2-F4 на станции");
                println!("        пустые), F5 на станции — нативная закупка, F6 — карта,");
                println!("        F7 — информация о планете, F9 — экран капитана,");
                println!("        ` — выход в эмулятор из любого режима, Esc — меню паузы,");
                println!("        F11 — полный экран, F12 — множители x2/x5/x10");
                println!("        (удержание — сброс на x1).");
                println!();
                println!("Заставка нативная: [SPACE] — начать игру, ◄/► — корабль,");
                println!("  [Esc] — меню паузы (на заставке только загрузка, сохранение");
                println!("  без игры отключено), [`] — показать родное интро; повторный");
                println!("  [`] возвращает нативный титр.");
                println!();
                println!("Экран: ZX-кадр 256x192 показывается целым множителем (чёткие");
                println!("квадратные пиксели) по центру; нативные (Rust) реализации");
                println!("рисуют в HD-слой поверх него в полном разрешении окна.");
                println!();
                println!("Управление:");
                println!("  Esc        меню паузы (продолжить/сохранить/загрузить/выход)");
                println!("  F1-F4      клавиши 1-4 оригинала (виды); F1 включает нативный");
                println!("             вид полёта (на станции — после вылета), F2-F4 на");
                println!("             станции пустые");
                println!("  F5         нативная закупка оборудования (на станции)");
                println!("  F6         нативная карта галактики");
                println!("  F7         информация о выбранной планете (как P)");
                println!("  F9         альтернативный экран капитана (как L);");
                println!("             управление остаётся за эмулятором");
                println!("  Enter      нативный инвентарь: топливо и деньги");
                println!("  F11, Alt+Enter полный экран");
                println!("  F12        множитель эмуляции Z80: x2 → x5 → x10 → x1:");
                println!("             удержание дольше 1 с сбрасывает на x1");
                println!("  `          выход в эмулятор из любого режима: снимает паузу,");
                println!("             меню, редактор и нативные экраны; на заставке");
                println!("             переключает титр и интро Z80");
                println!("  Ctrl+Q     выход из игры");
                println!();
                println!("Заставка: SPACE запускает игру, ◄/► переключают корабль.");
                println!();
                println!("Карта галактики (F6): стрелки/NM/XS — курсор, Enter —");
                println!("  ближайшая планета, +/- или колесо — масштаб. Повторный F6");
                println!("  и Esc карту не закрывают: выход только переходом на другой");
                println!("  экран (F1,F5,F7-F9, игровые клавиши; F2-F4 на станции пустые).");
                println!("  Ctrl+F1 — чит: телепорт на станцию выбранной планеты.");
                println!();
                println!("Экран полёта (F1): управление как в оригинале — S/X или");
                println!("  ↑/↓ — тангаж, N/M или ←/→ — крен, Space — ускорение,");
                println!("  Shift — торможение, A/RightCtrl — огонь; Esc — меню паузы,");
                println!("  F1-F4 — виды вперёд/назад/влево/вправо (клавиши 1-4),");
                println!("  1 — назад к ZX-кадру. Переходы игры (O, Enter и т.п.)");
                println!("  закрывают нативный вид и показывают экран оригинала.");
                println!();
                println!("Закупка оборудования (F5): клик/Enter — купить,");
                println!("  1-9 (+вторая цифра) — номер товара, стрелки — выбор,");
                println!("  0-3 — крепление лазера, Esc — отмена крепления.");
                exit(0);
            }
            other => {
                eprintln!("elite: неизвестный аргумент '{other}'");
                eprintln!("запустите elite --help для списка аргументов");
                exit(2);
            }
        }
    }
    opts
}
