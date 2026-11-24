//! Elite — точка входа.
//!
//! Здесь только сборка подсистем: разбор аргументов, загрузка образа игры и
//! запуск одного из двух режимов — оконного (`app`) или headless
//! (`headless`). Дерево модулей:
//!
//! - `cli` — аргументы командной строки;
//! - `assets` — образы игры и ROM, встроенные в исполняемый файл;
//! - `boot` — загрузка образа игры и начальное состояние эмулятора;
//! - `world` — привязка к версии игры (сейв, заставка, текущая планета);
//! - `keys` — раскладка клавиш ZX/Kempston;
//! - `headless` — прогон без окна и отладочные дампы;
//! - `storage` — сохранение/загрузка командира;
//! - `ui` — меню паузы и показ кадра;
//! - `app` — окно, ввод, нативные экраны и кадр;
//! - `audio`, `machine`, `z80`, `native` — звук и эмуляция;
//! - `cockpit`, `display`, `docked`, `equip`, `flight`, `galaxy`, `hdmap`, `render3d`,
//!   `shipdata`, `splash`, `status` — графика и игровые подсистемы;
//! - `font` — встроенный знакогенератор ZX для нативного интерфейса.

mod app;
mod assets;
mod audio;
mod boot;
mod cli;
mod cockpit;
mod display;
mod docked;
mod equip;
mod flight;
mod font;
mod galaxy;
mod headless;
mod hdmap;
mod keys;
mod machine;
mod native;
mod render3d;
mod sfx;
mod shipdata;
mod splash;
mod market;
mod state;
mod status;
mod sysinfo;
mod storage;
mod ui;
mod world;
mod z80;

fn main() {
    let opts = cli::parse_args();
    let mut machine = boot::machine(&opts);
    // `--save FILE` теперь только стартовая загрузка: файл на 102 байта
    // импортируется в каталог, ложится в память и уводит игру на станцию,
    // минуя заставку. Обычное сохранение из игры идёт только в каталог.
    if let Some(path) = &opts.save {
        let path = std::path::Path::new(path);
        if let Err(e) = storage::load_startup_save(&mut machine, path) {
            eprintln!("Ошибка загрузки при старте: {e}");
        }
    }
    if opts.headless {
        headless::run(&mut machine, &opts);
    } else {
        app::run(machine, &opts);
    }
}
