//! Elite — точка входа.
//!
//! Здесь только сборка подсистем: разбор аргументов, загрузка образа игры и
//! запуск одного из двух режимов — оконного (`app`) или headless
//! (`headless`). Дерево модулей:
//!
//! - `cli` — аргументы командной строки;
//! - `boot` — загрузка образа игры и начальное состояние эмулятора;
//! - `world` — привязка к версии игры (сейв, заставка, текущая планета);
//! - `keys` — раскладка клавиш ZX/Kempston;
//! - `headless` — прогон без окна и отладочные дампы;
//! - `storage` — сохранение/загрузка командира;
//! - `ui` — меню паузы и показ кадра;
//! - `app` — окно, ввод, нативные экраны и кадр;
//! - `audio`, `machine`, `z80`, `native` — звук и эмуляция;
//! - `cockpit`, `display`, `equip`, `flight`, `galaxy`, `hdmap`, `render3d`,
//!   `shipdata`, `splash` — графика и игровые подсистемы.

mod app;
mod audio;
mod boot;
mod cli;
mod cockpit;
mod display;
mod equip;
mod flight;
mod galaxy;
mod headless;
mod hdmap;
mod keys;
mod machine;
mod native;
mod render3d;
mod shipdata;
mod splash;
mod storage;
mod ui;
mod world;
mod z80;

fn main() {
    let opts = cli::parse_args();
    let mut machine = boot::machine(&opts);
    if opts.headless {
        headless::run(&mut machine, &opts);
    } else {
        app::run(machine, &opts);
    }
}
