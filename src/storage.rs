//! Каталог сохранений и обмен одиночными блоками командира.
//!
//! Внутри приложения все сейвы лежат в одном файле `saves.bin` в стандартном
//! пользовательском каталоге (`FSA/Elite`). Запись каталога имеет фиксированную
//! длину: имя командира (10 байт), комментарий (10), штамп времени UTC (i64)
//! и собственно 102 байта оригинального блока. Метаданные поэтому не меняют
//! формат игры, а экспорт/импорт работают с теми же 102 байтами без конверта.

#[cfg(test)]
use std::cell::RefCell;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Local, Utc};

use crate::machine::Spectrum;
use crate::native;
use crate::world;

/// Длина блока командира в оригинальной игре (`0x66`).
pub const COMMANDER_LEN: usize = 102;
/// Имя пилота: 10 символов, одиннадцатый байт блока — завершающий ноль.
pub const NAME_LEN: usize = 10;
/// Комментарий к сохранению: 10 символов, как колонка COMMENT в таблице.
/// Больше не влезло бы в экран 32x24 вместе с NAME и DATE.
pub const COMMENT_LEN: usize = 10;

const MAGIC: &[u8; 8] = b"FSAELITE";
const VERSION: u32 = 1;
const HEADER_LEN: usize = 16;
const TIME_LEN: usize = 8;
const RECORD_LEN: usize = NAME_LEN + COMMENT_LEN + TIME_LEN + COMMANDER_LEN;

/// Одна запись каталога. `saved_at` — секунды UTC; показывается местным временем.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Имя командира, уже ограниченное 10 символами ZX-набора.
    pub name: String,
    /// Комментарий длиной до 64 печатных ASCII-символов.
    pub comment: String,
    /// Штамп сохранения (для импорта — mtime исходного файла).
    pub saved_at: i64,
    /// Ровно 102 байта оригинального блока commander.
    pub data: Vec<u8>,
}

/// Поле, которое редактируется в приглашении сохранения.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptField {
    Name,
    Comment,
}

/// Приглашение перед сохранением: имя уже заполнено из текущего блока.
#[derive(Clone, Debug)]
pub struct SavePrompt {
    pub name: String,
    pub comment: String,
    pub field: PromptField,
    pub status: String,
}

/// Таблица сохранений на экране загрузки.
#[derive(Clone, Debug)]
pub struct LoadList {
    pub entries: Vec<Entry>,
    pub sel: usize,
    pub status: String,
}

/// Полноэкранный экран работы с сохранениями.
#[derive(Clone, Debug)]
pub enum SaveScreen {
    Save(SavePrompt),
    Load(LoadList),
}

/// Имя файла для экспорта/headless: `elite-ГГГГ-ММ-ДД_ЧЧ-ММ-СС.zxelite`.
pub fn default_save_name() -> String {
    format!("elite-{}.zxelite", Utc::now().format("%Y-%m-%d_%H-%M-%S"))
}

#[cfg(test)]
thread_local! {
    /// Подмена пути каталога в тестах: сериализуется потоком, поэтому
    /// параллельные тесты не перетирают друг друга.
    static TEST_CATALOG: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// Подменить путь каталога только внутри текущего тестового потока.
#[cfg(test)]
pub fn set_test_catalog(path: Option<PathBuf>) {
    TEST_CATALOG.with(|slot| *slot.borrow_mut() = path);
}

/// Путь каталога: стандартный data_dir плюс вендор FSA и приложение Elite.
///
/// `ProjectDirs` на Linux отбрасывает вендор, поэтому каталог собираем явно из
/// `dirs::data_dir()`: Linux — `$XDG_DATA_HOME/FSA/Elite`, macOS —
/// `~/Library/Application Support/FSA/Elite`, Windows — `%APPDATA%\FSA\Elite`.
pub fn catalog_path() -> Option<PathBuf> {
    #[cfg(test)]
    if let Some(path) = TEST_CATALOG.with(|slot| slot.borrow().clone()) {
        return Some(path);
    }
    dirs::data_dir().map(|p| p.join("FSA").join("Elite").join("saves.bin"))
}

/// Привести имя к ограничению поля внутри 102-байтового блока.
///
/// Допустимы только заглавные латинские буквы и пробел; всё остальное стаёт
/// пробелом. Хвостовые пробелы убираются — в таблице имя не должно занимать
/// десять клеток, если игрок стёр часть имени.
pub fn sanitize_name(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars().take(NAME_LEN) {
        let c = match ch {
            'a'..='z' => ch.to_ascii_uppercase(),
            'A'..='Z' | ' ' => ch,
            _ => ' ',
        };
        out.push(c);
    }
    out.trim_end().to_string()
}

/// Прочитать имя из блока commander: первые 10 байт, завершающий ноль не трогаем.
pub fn name_from_block(data: &[u8]) -> String {
    let sliced = data.iter().take(NAME_LEN).copied().collect::<Vec<_>>();
    sanitize_name(&String::from_utf8_lossy(&sliced))
}

/// Вписать имя в блок commander: 10 байт пробелами, байт 10 остаётся нулём.
fn write_name_into_block(data: &mut [u8], name: &str) {
    if data.len() <= NAME_LEN {
        return;
    }
    data[..NAME_LEN].fill(b' ');
    for (i, b) in sanitize_name(name).bytes().take(NAME_LEN).enumerate() {
        data[i] = b;
    }
    data[NAME_LEN] = 0;
}

/// Привести комментарий к 64 печатным символам встроенного шрифта ZX.
pub fn sanitize_comment(input: &str) -> String {
    input
        .chars()
        .take(COMMENT_LEN)
        .map(|c| if (' '..='~').contains(&c) { c } else { '?' })
        .collect()
}

/// Символ, который можно ввести в имя командира.
pub fn typed_name_char(c: char) -> Option<char> {
    match c {
        'a'..='z' => Some(c.to_ascii_uppercase()),
        'A'..='Z' | ' ' => Some(c),
        _ => None,
    }
}

/// Символ, который можно ввести в комментарий.
pub fn typed_comment_char(c: char) -> Option<char> {
    (' '..='~').contains(&c).then_some(c)
}

/// Комментарий импорта: время импорта в 10 символах (`IMP ЧЧ:ММ`).
pub fn format_import_comment() -> String {
    format!("IMP {}", Utc::now().with_timezone(&Local).format("%H:%M"))
}

/// Дата в таблице: `ГГ-ММ-ДД` — ровно 8 знакомест.
pub fn format_local_short(timestamp: i64) -> String {
    match DateTime::<Utc>::from_timestamp(timestamp, 0) {
        Some(dt) => dt.with_timezone(&Local).format("%y-%m-%d").to_string(),
        None => "-- -- --".to_string(),
    }
}

fn fixed_string(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    text.trim_matches(|c| c == ' ' || c == '\0').to_string()
}

fn padded_string(text: &str, len: usize, out: &mut Vec<u8>) {
    let bytes = text.as_bytes();
    out.extend_from_slice(&bytes[..bytes.len().min(len)]);
    out.resize(out.len() + len.saturating_sub(bytes.len()), b' ');
}

fn read_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]))
}

/// Прочитать каталог. Отсутствующий файл означает пустой список.
pub fn read_entries_from(path: &Path) -> io::Result<Vec<Entry>> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    if bytes.len() < HEADER_LEN {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "короткий заголовок каталога",
        ));
    }
    if &bytes[..8] != MAGIC {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "не каталог сохранений Elite",
        ));
    }
    let version = read_u32(&bytes[8..12]);
    if version != VERSION {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("неизвестная версия каталога: {version}"),
        ));
    }
    let count = read_u32(&bytes[12..16]) as usize;
    let needed = HEADER_LEN.saturating_add(count.saturating_mul(RECORD_LEN));
    if bytes.len() != needed {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "каталог сохранений повреждён",
        ));
    }
    let mut entries = Vec::with_capacity(count);
    for chunk in bytes[HEADER_LEN..].chunks_exact(RECORD_LEN) {
        let mut off = 0;
        let name = sanitize_name(&fixed_string(&chunk[off..off + NAME_LEN]));
        off += NAME_LEN;
        let comment = sanitize_comment(&fixed_string(&chunk[off..off + COMMENT_LEN]));
        off += COMMENT_LEN;
        let saved_at = i64::from_le_bytes(chunk[off..off + TIME_LEN].try_into().unwrap());
        off += TIME_LEN;
        entries.push(Entry {
            name,
            comment,
            saved_at,
            data: chunk[off..off + COMMANDER_LEN].to_vec(),
        });
    }
    Ok(entries)
}

/// Записать каталог целиком: сначала временный файл, потом замена основного.
///
/// При ошибке замены прежний файл остаётся в `.bak`, чтобы неудачная запись не
/// уничтожила все сохранения разом.
pub fn write_entries_to(path: &Path, entries: &[Entry]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut bytes = Vec::with_capacity(HEADER_LEN + entries.len() * RECORD_LEN);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for entry in entries {
        padded_string(&sanitize_name(&entry.name), NAME_LEN, &mut bytes);
        padded_string(&sanitize_comment(&entry.comment), COMMENT_LEN, &mut bytes);
        bytes.extend_from_slice(&entry.saved_at.to_le_bytes());
        let n = entry.data.len().min(COMMANDER_LEN);
        bytes.extend_from_slice(&entry.data[..n]);
        bytes.resize(bytes.len() + COMMANDER_LEN - n, 0);
    }

    let temp = path.with_extension("tmp");
    let backup = path.with_extension("bak");
    fs::write(&temp, bytes)?;
    if path.exists() {
        if backup.exists() {
            fs::remove_file(&backup)?;
        }
        fs::rename(path, &backup)?;
    }
    if let Err(e) = fs::rename(&temp, path) {
        if backup.exists() {
            let _ = fs::rename(&backup, path);
        }
        return Err(e);
    }
    let _ = fs::remove_file(backup);
    Ok(())
}

pub fn read_entries() -> io::Result<Vec<Entry>> {
    let Some(path) = catalog_path() else {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "не найден стандартный каталог данных пользователя",
        ));
    };
    read_entries_from(&path)
}

fn append_entry(entry: Entry) -> io::Result<Vec<Entry>> {
    let Some(path) = catalog_path() else {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "не найден стандартный каталог данных пользователя",
        ));
    };
    let mut entries = read_entries_from(&path)?;
    entries.push(entry);
    write_entries_to(&path, &entries)?;
    Ok(entries)
}

/// Отсортировать записи для таблицы: свежие сверху.
pub fn sorted_for_table(entries: &mut [Entry]) {
    entries.sort_by(|a, b| {
        b.saved_at
            .cmp(&a.saved_at)
            .then_with(|| a.name.cmp(&b.name))
    });
}

/// Сохранить текущий блок с введёнными именем и комментарием.
///
/// Имя записывается и в 102-байтовый блок (байт 10 остаётся нулевым), поэтому
/// загруженная игра будет показывать именно подтверждённое имя.
pub fn save_to_catalog(
    machine: &mut Spectrum,
    name: &str,
    comment: &str,
    layout: crate::native::SaveLayout,
) -> io::Result<Vec<Entry>> {
    let mut data = native::commander_snapshot(machine, layout);
    write_name_into_block(&mut data, name);
    append_entry(Entry {
        name: sanitize_name(name),
        comment: sanitize_comment(comment),
        saved_at: Utc::now().timestamp(),
        data,
    })
}

/// Удалить одну запись из каталога (сравнение по полному содержимому, чтобы
/// порядок сортировки таблицы не влиял на выбор).
pub fn delete_entry(entry: &Entry) -> io::Result<Vec<Entry>> {
    let Some(path) = catalog_path() else {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "не найден стандартный каталог данных пользователя",
        ));
    };
    let mut entries = read_entries_from(&path)?;
    if let Some(pos) = entries.iter().position(|e| e == entry) {
        entries.remove(pos);
    }
    write_entries_to(&path, &entries)?;
    Ok(entries)
}

/// Загрузить запись в память и перезапустить игру на станции без заставки.
pub fn load_entry(machine: &mut Spectrum, entry: &Entry) {
    let layout = world::save_layout_kladov();
    native::commander_load_bytes(machine, &entry.data, layout);
    native::request_load(machine, layout);
}

/// Имя экспортируемого файла: дата берётся из штампа записи.
pub fn export_name(entry: &Entry) -> String {
    let stamp = DateTime::<Utc>::from_timestamp(entry.saved_at, 0)
        .unwrap_or_else(|| Utc::now())
        .format("%Y-%m-%d_%H-%M-%S");
    format!("elite-{stamp}.zxelite")
}

/// Записать отдельное сохранение ровно в 102 байта.
pub fn export_entry_to(entry: &Entry, path: &Path) -> io::Result<()> {
    fs::write(path, &entry.data)
}

/// Экспорт через системный диалог.
pub fn export_entry_dialog(entry: &Entry) -> io::Result<Option<PathBuf>> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Elite (ZX)", &["zxelite"])
        .set_file_name(export_name(entry))
        .save_file()
    else {
        return Ok(None);
    };
    export_entry_to(entry, &path)?;
    Ok(Some(path))
}

/// Собрать запись из внешнего файла ровно в 102 байта.
///
/// В дате остаётся mtime файла (это единственная дата, которую несёт блок),
/// а в комментарий пишется момент импорта.
pub fn entry_from_save_file(path: &Path) -> io::Result<Entry> {
    let data = fs::read(path)?;
    if data.len() != COMMANDER_LEN {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("ожидалось {COMMANDER_LEN} байт, получено {}", data.len()),
        ));
    }
    let saved_at = fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or_else(|| Utc::now().timestamp());
    Ok(Entry {
        name: name_from_block(&data),
        comment: format_import_comment(),
        saved_at,
        data,
    })
}

/// Импорт файла через системный диалог: добавить запись в каталог.
pub fn import_entry_dialog() -> io::Result<Option<Entry>> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Elite (ZX)", &["zxelite"])
        .pick_file()
    else {
        return Ok(None);
    };
    let entry = entry_from_save_file(&path)?;
    append_entry(entry.clone())?;
    Ok(Some(entry))
}

/// Стартовая загрузка из `--save FILE`: импортировать файл в каталог, поместить
/// блок в память и увести Z80 на станцию, минуя заставку.
pub fn load_startup_save(machine: &mut Spectrum, path: &Path) -> io::Result<()> {
    let entry = entry_from_save_file(path)?;
    append_entry(entry.clone())?;
    let layout = world::save_layout_kladov();
    native::commander_load_bytes(machine, &entry.data, layout);
    native::request_load(machine, layout);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn temp_path(ext: &str) -> PathBuf {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "elite-storage-test-{}-{n}.{ext}",
            std::process::id()
        ))
    }

    fn sample(ts: i64) -> Entry {
        let mut data = vec![0u8; COMMANDER_LEN];
        data[0..10].copy_from_slice(b"JAMESON   ");
        Entry {
            name: "JAMESON".to_string(),
            comment: "TEST SAVE".to_string(),
            saved_at: ts,
            data,
        }
    }

    #[test]
    fn catalog_roundtrip() {
        let path = temp_path("bin");
        let entries = vec![sample(1_700_000_000), sample(1_700_000_100)];
        write_entries_to(&path, &entries).unwrap();
        assert_eq!(read_entries_from(&path).unwrap(), entries);
        assert_eq!(
            fs::metadata(&path).unwrap().len() as usize,
            HEADER_LEN + 2 * RECORD_LEN
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn missing_catalog_is_empty() {
        let path = temp_path("bin");
        assert!(read_entries_from(&path).unwrap().is_empty());
    }

    #[test]
    fn name_is_limited_by_save_block() {
        assert_eq!(sanitize_name("abcdefghijkLMN"), "ABCDEFGHIJ");
        assert_eq!(sanitize_name("JAMESON   "), "JAMESON");
        assert_eq!(name_from_block(b"JAMESON   \0rest"), "JAMESON");
    }

    #[test]
    fn write_name_keeps_terminator() {
        let mut data = vec![0xFFu8; COMMANDER_LEN];
        write_name_into_block(&mut data, "JAMESON");
        assert_eq!(&data[..10], b"JAMESON   ");
        assert_eq!(data[10], 0);
    }

    #[test]
    fn comment_is_limited_to_printable_ascii() {
        let input = format!("{}Ж", "X".repeat(COMMENT_LEN + 5));
        assert_eq!(sanitize_comment(&input), "X".repeat(COMMENT_LEN));
    }

    #[test]
    fn import_takes_file_mtime() {
        let path = temp_path("zxelite");
        fs::write(&path, vec![0u8; COMMANDER_LEN]).unwrap();
        let stamp = UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_123);
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(stamp)
            .unwrap();
        let entry = entry_from_save_file(&path).unwrap();
        assert_eq!(entry.saved_at, 1_700_000_123);
        assert!(entry.comment.starts_with("IMP "));
        assert_eq!(entry.data.len(), COMMANDER_LEN);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn import_rejects_wrong_size() {
        let path = temp_path("zxelite");
        fs::write(&path, vec![0u8; COMMANDER_LEN - 1]).unwrap();
        assert!(entry_from_save_file(&path).is_err());
        let _ = fs::remove_file(&path);
    }
}
