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
    /// Необязательный позиционный аргумент: файл commander-блока ровно на
    /// 102 байта. При наличии он импортируется в каталог и загружается вместо
    /// заставки. Обычные SAVE/LOAD в игре работают только с каталогом FSA/Elite.
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
    parse(env::args().skip(1))
}

fn parse<I>(args: I) -> Options
where
    I: IntoIterator<Item = String>,
{
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
    let mut args = args.into_iter();
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
            "--fullscreen" => opts.fullscreen = true,
            "--debug" => opts.debug = true,
            "-V" | "--version" => {
                println!("elite {VERSION}");
                exit(0);
            }
            "-h" | "--help" => {
                println!("usage: elite [OPTIONS] [FILE]");
                println!();
                println!("Аргументы:");
                println!("  FILE             загрузить при старте 102-байтовый файл commander-блока");
                println!("                   (импорт в каталог FSA/Elite и сразу на станцию)");
                println!();
                println!("Опции командной строки:");
                println!("  -h, --help       показать эту справку");
                println!("  -V, --version    показать версию сборки (тег релиза)");
                println!("      --headless   работать без графического окна");
                println!("      --frames N   число кадров в headless-режиме (по умолчанию 600)");
                println!("      --dump FILE  сохранить кадр в PPM-файл");
                println!("      --mute       отключить вывод звука через PipeWire");
                println!("      --audio      выводить звук и в headless-режиме (реальное время)");
                println!("      --wav FILE   сохранить звук в WAV-файл");
                println!("      --native     включить нативные (Rust) замены подпрограмм");
                println!("      --fullscreen запуск в полноэкранном режиме (borderless)");
                println!("      --debug      отладка: справа полоса 256 px с кадром эмулятора 1:1");
                exit(0);
            }
            other if other.starts_with('-') => {
                eprintln!("elite: неизвестный аргумент '{other}'");
                eprintln!("запустите elite --help для списка аргументов");
                exit(2);
            }
            other => {
                if opts.save.is_some() {
                    eprintln!("elite: указано более одного файла загрузки: '{other}'");
                    eprintln!("запустите elite --help для списка аргументов");
                    exit(2);
                }
                opts.save = Some(other.to_string());
            }
        }
    }
    opts
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn file_is_optional() {
        let opts = parse(["--debug".to_string()]);
        assert!(opts.debug);
        assert_eq!(opts.save, None);
    }

    #[test]
    fn positional_file_is_startup_save() {
        let opts = parse(["--mute".to_string(), "commander.zxelite".to_string()]);
        assert!(opts.mute);
        assert_eq!(opts.save.as_deref(), Some("commander.zxelite"));
    }

    #[test]
    fn file_can_precede_options() {
        let opts = parse(["commander.zxelite".to_string(), "--native".to_string()]);
        assert!(opts.native);
        assert_eq!(opts.save.as_deref(), Some("commander.zxelite"));
    }
}
