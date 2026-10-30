//! Сохранение и загрузка блока командира: файловые диалоги и вызовы
//! нативного сейва/лоада.

use std::path::{Path, PathBuf};

use crate::cli::Options;
use crate::machine::Spectrum;
use crate::native;
use crate::world;

/// Авто-имя файла сохранения: `elite-ГГГГ-ММ-ДД_ЧЧ-ММ-СС.zxelite`.
pub fn default_save_name() -> String {
    let now = chrono::Local::now();
    format!("elite-{}.zxelite", now.format("%Y-%m-%d_%H-%M-%S"))
}

/// Диалог сохранения командира. `last` — прошлый путь (задаёт каталог).
fn save_dialog(last: &str) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .add_filter("Elite (ZX)", &["zxelite"])
        .set_file_name(default_save_name());
    if let Some(dir) = Path::new(last)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
    {
        d = d.set_directory(dir);
    }
    d.save_file()
}

/// Путь сохранения: фиксированный `--save` или интерактивный диалог.
fn choose_save_path(opts: &Options, last: &str) -> Option<PathBuf> {
    match &opts.save {
        Some(p) => Some(PathBuf::from(p)),
        None => save_dialog(last),
    }
}

/// Сохранить блок командира; при успехе запомнить путь в `last`.
pub fn do_save(machine: &mut Spectrum, opts: &Options, last: &mut String) {
    let Some(path) = choose_save_path(opts, last) else {
        return;
    };
    match native::commander_save(machine, &path.to_string_lossy(), world::save_layout_kladov()) {
        Ok(()) => {
            println!("Сохранено в {}", path.display());
            *last = path.to_string_lossy().into_owned();
        }
        Err(e) => eprintln!("Ошибка сохранения: {e}"),
    }
}

/// Диалог открытия файла командира. `last` — прошлый путь (задаёт каталог).
fn load_dialog(last: &str) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new().add_filter("Elite (ZX)", &["zxelite"]);
    if let Some(dir) = Path::new(last)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
    {
        d = d.set_directory(dir);
    }
    d.pick_file()
}

/// Путь загрузки: фиксированный `--save` или интерактивный диалог открытия.
fn choose_load_path(opts: &Options, last: &str) -> Option<PathBuf> {
    match &opts.save {
        Some(p) => Some(PathBuf::from(p)),
        None => load_dialog(last),
    }
}

/// Загрузить блок командира из файла; при успехе запомнить путь в `last`.
pub fn do_load(machine: &mut Spectrum, opts: &Options, last: &mut String) {
    let Some(path) = choose_load_path(opts, last) else {
        return;
    };
    match native::commander_load(machine, &path.to_string_lossy(), world::save_layout_kladov()) {
        Ok(()) => {
            println!("Загружено из {}", path.display());
            *last = path.to_string_lossy().into_owned();
            // Прервать текущий процесс и запустить игру с новыми данными:
            native::request_load(machine, world::save_layout_kladov());
        }
        Err(e) => eprintln!("Ошибка загрузки: {e}"),
    }
}
