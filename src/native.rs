//! Нативные (Rust) реализации подпрограмм игры.
//!
//! Каждая функция здесь повторяет поведение конкретной рутины из игры
//! (адрес указан в комментарии). Цель — постепенно заменить выполнение этих
//! рутин на Z80, вплоть до полного отказа от эмуляции.
//!
//! Инвариант: поведение обязано совпадать с исходной рутиной. Для каждой
//! функции есть тест эквивалентности (см. `mod tests`), который прогоняет
//! настоящий код из образа `zx_bin/kladov.bin` на эмуляторе и сверяет
//! результат с нативной реализацией.

use crate::machine::{Hook, Spectrum};
use crate::z80::{Z80, ZF, CF};

/// Установить все нативные замены (хуки) на машину.
///
/// Адреса регистровых помощников соответствуют версии Кладова; для
/// оригинальной версии (`--elite`) хук по этим адресам был бы установлен на
/// чужой код, поэтому сейчас ставим их только если `kladov == true`.
pub fn install(m: &mut Spectrum, kladov: bool) {
    if !kladov {
        return;
    }
    m.hooks.insert(0x5DFB, font_base_hook);
    m.hooks.insert(0xEB3E, eb3e_hook);
    m.hooks.insert(0xD3CE, d3ce_hook);
    m.hooks.insert(0xAB0B, mult_a_hib_c_hook);
    m.hooks.insert(0xE621, pix_crd_2_scr_adr_hook);
    m.hooks.insert(0x7FE7, scr_adr_next_line_hook);
    m.hooks.insert(0x8445, scr_adr_next_char_line_hook);
}

/// Сколько байт клавиш сбрасывает родная `ResetPressedKeys`.
const PRESSED_KEY_BYTES: usize = 4;

/// Раскладка сейва командира, зависящая от версии игры.
///
/// Адреса рутин и переменных у Кладова и в оригинале разные (original:
/// `MenuSaveLoad` = `0xD2E4`, `ControlKey1` = `0xFC0E`, `PilotName` = `0xA816`;
/// Кладова: `0xD04E`, `0xFC08`, `0xD232`), поэтому выбираем раскладку по
/// запущенной версии (`--elite`).
#[derive(Clone, Copy)]
pub struct SaveLayout {
    /// Точка входа родной рутины сохранения/загрузки (`MenuSaveLoad`).
    ///
    /// Вызывается из главного цикла только при нажатии игровой клавиши
    /// сохранения (у Кладова `0x749F: call 0xD04E`), поэтому перехват не
    /// срабатывает каждый кадр. Родная рутина в версии Кладова уходит в TR-DOS
    /// (`jp 0x3D2F`) с пейджингом 128K — в 48K-ядре это не работает.
    pub menu_save_load: u16,
    /// Первая из четырёх «залипающих» ячеек клавиш (`ControlKey1`).
    ///
    /// Родная `ResetPressedKeys` обнуляет четыре байта начиная отсюда:
    /// `ControlKey1`, `ControlKeyMove`, `ControlKey2`, `ControlKey3`.
    pub control_key1: u16,
    /// Адрес живого блока данных командира (`PilotName`) — из него берутся
    /// данные для сохранения.
    ///
    /// У Кладова это `0xD054` (а не `0xD232`!): `InitGame` копирует тень
    /// `0xD232`→`0xD054`, а `DemoModeOn` грузит демо-режим именно из
    /// `PilotName` длиной `66h` — то есть рабочий блок пилота начинается
    /// здесь. Тень (`aJameson_1`) игра при сохранении обновляет сама.
    pub pilot_name: u16,
    /// Длина блока командира. Кладова копирует 102 байта (`ld bc,66h` в
    /// `DemoModeOn`), оригинал сохраняет 104 (`ld ix,PilotName; ld de,68h` в
    /// `sub_D41B`) — на два байта больше: `PilotMission1_1` и
    /// `Pilot_CrdXTarget`.
    pub commander_len: usize,
    /// Точка входа перезапуска игры после загрузки — ветка `jp nz` главного
    /// цикла (см. комментарий к [`menu_save_load`]): в оригинале `loc_71BE`,
    /// у Кладова `loc_7394`. Игра заново проходит стартовую последовательность
    /// и оказывается пристыкованной на станции системы из загруженного сейва.
    pub restart_entry: u16,
    /// Теневой буфер, из которого `InitGame` восстанавливает пилота, и его
    /// длина.
    ///
    /// В оригинале `RestorePilotFromMemory` копирует `PilotDataSaved`→
    /// `PilotName` (102 байта), у Кладова `InitGame` делает то же самое:
    /// `21 32 D2 11 54 D0 01 5D 00 ED B0` = `ld hl,0xD232 / ld de,0xD054 /
    /// ld bc,5Dh / ldir` — плюс ещё копирует 9 байт координат солнца
    /// (`0xD28F`→`SunCrd`), то есть теневой буфер целиком занимает
    /// `0xD232..0xD297` = 102 байта. Поэтому при загрузке нужно обновить и
    /// его — иначе загруженные данные затрёт копия из тени (а тень
    /// заполняется при сохранении, см. [`Self::pilot_name`]).
    pub pilot_shadow: Option<(u16, usize)>,
    /// Адрес данных солнца для отрисовки (`SunCrd`, 9 байт).
    ///
    /// Родная `SaveGame` перед сохранением копирует их в поле
    /// `PilotDataOfSun` живого блока (`ld hl,SunCrd / ld de,PilotDataOfSun /
    /// ld bc,9 / ldir`), чтобы после загрузки солнце рисовалось по текущим
    /// координатам. Хук подменяет `MenuSaveLoad` целиком, поэтому копирование
    /// делает [`commander_save`]. Проверено по образам: у Кладова
    /// `SunCrd`=0x7107 (в `InitGame`: `ld de,7107h`), у оригинала 0x7065
    /// (`ld de,7065h`); поле `PilotDataOfSun` лежит на смещении 93 живого
    /// блока в обеих версиях (`byte_A96D` = `PilotDataSaved`+93).
    pub sun_crd: u16,
    /// Нативная реализация `MenuSaveLoad` для этой версии.
    hook: Hook,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD04E,
    control_key1: 0xFC08,
    pilot_name: 0xD054,
    commander_len: 0x66,
    restart_entry: 0x7394,
    pilot_shadow: Some((0xD232, 0x66)),
    sun_crd: 0x7107,
    hook: menu_save_load_kladov_hook,
};

/// Раскладка оригинальной версии (`zx_bin/elite.bin`).
pub const ELITE_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD2E4,
    control_key1: 0xFC0E,
    pilot_name: 0xA816,
    commander_len: 0x68,
    restart_entry: 0x71BE,
    pilot_shadow: Some((0xA910, 0x66)),
    sun_crd: 0x7065,
    hook: menu_save_load_elite_hook,
};

/// Блок сохранения командира в памяти.
pub fn commander_block(m: &Spectrum, layout: SaveLayout) -> &[u8] {
    let a = layout.pilot_name as usize;
    &m.mem[a..a + layout.commander_len]
}

/// Смещение поля `PilotDataOfSun` (9 байт координат солнца) внутри блока
/// командира. Одинаково в обеих версиях: в оригинале это `byte_A96D` =
/// `PilotDataSaved`+93, у Кладова `unk_D28F` = `aJameson_1`+93.
const SUN_FIELD_OFF: usize = 93;

/// Записать блок командира в файл `path`.
///
/// Перед записью выполняется то же, что делает родная `SaveGame`: координаты
/// солнца для отрисовки копируются в поле `PilotDataOfSun` блока командира
/// (`ld hl,SunCrd / ld de,PilotDataOfSun / ld bc,9 / ldir`), и теневой буфер
/// синхронизируется из живого блока (`ld hl,PilotName / ld de,PilotDataSaved /
/// ld bc,102 / ldir`). Из тени `InitGame` восстанавливает пилота при
/// перезапуске, поэтому без этой копии загрузка вернула бы устаревшие данные
/// и солнце рисовалось бы не по тем координатам.
pub fn commander_save(m: &mut Spectrum, path: &str, layout: SaveLayout) -> std::io::Result<()> {
    let src = layout.pilot_name as usize;
    let sun = layout.sun_crd as usize;
    let sun_data = m.mem[sun..sun + 9].to_vec();
    m.mem[src + SUN_FIELD_OFF..src + SUN_FIELD_OFF + 9].copy_from_slice(&sun_data);
    if let Some((addr, len)) = layout.pilot_shadow {
        let n = layout.commander_len.min(len);
        let dst = addr as usize;
        let data = m.mem[src..src + n].to_vec();
        m.mem[dst..dst + n].copy_from_slice(&data);
    }
    std::fs::write(path, commander_block(m, layout))
}

/// Прочитать блок командира из файла `path` и поместить в память.
pub fn commander_load(
    m: &mut Spectrum,
    path: &str,
    layout: SaveLayout,
) -> std::io::Result<()> {
    let data = std::fs::read(path)?;
    let n = data.len().min(layout.commander_len);
    let a = layout.pilot_name as usize;
    m.mem[a..a + n].copy_from_slice(&data[..n]);
    // Синхронизировать теневой буфер, из которого `InitGame` восстанавливает
    // пилота (оригинал), иначе загруженные данные затрёт копия из памяти.
    if let Some((addr, len)) = layout.pilot_shadow {
        let s = data.len().min(len);
        let a = addr as usize;
        m.mem[a..a + s].copy_from_slice(&data[..s]);
    }
    Ok(())
}

/// Прервать текущий игровой процесс и перезапустить игру с только что
/// загруженными данными командира.
///
/// Повторяет ветку `jp nz` главного цикла после `MenuSaveLoad` (в оригинале
/// `loc_71BE`, у Кладова `loc_7394`): игра заново проходит стартовую
/// последовательность (выбор корабля/устройства управления) и оказывается
/// пристыкованной на станции системы из загруженного сейва.
pub fn restart_after_load(m: &mut Spectrum, layout: SaveLayout) {
    // Сбросить «залипшие» клавиши, как родная `ResetPressedKeys` в конце
    // `MenuSaveLoad`: стартовая ветка ждёт отпускания клавиши, и без сброса
    // можно застрять на этом ожидании.
    let a = layout.control_key1 as usize;
    m.mem[a..a + PRESSED_KEY_BYTES].fill(0);
    m.save_requested = false;
    // Стартовая последовательность использует `halt`, поэтому прерывания
    // должны быть разрешены (в оригинале после загрузки с ленты идёт `ei`).
    m.cpu.iff1 = true;
    m.cpu.iff2 = true;
    m.cpu.pc = layout.restart_entry;
}

/// Включить перехват игрового сохранения: при вызове `MenuSaveLoad` игра
/// выставляет флаг `machine.save_requested`, а главный цикл показывает диалог
/// выбора файла и записывает блок командира (родная рутина требует
/// 128K-пейджинга и в 48K-ядре не работает).
pub fn install_save(m: &mut Spectrum, layout: SaveLayout) {
    m.hooks.insert(layout.menu_save_load, layout.hook);
}

/// Общий код нативной замены `MenuSaveLoad`: сбросить «залипшие» клавиши и
/// попросить главный цикл сохранить блок командира.
///
/// Родная рутина заканчивается `jp ResetPressedKeys`, который обнуляет
/// `ControlKey1`..`ControlKey3`. Это обязательная часть: главный цикл проверяет
/// бит 5 `ControlKeyMove` (клавиша сохранения), и без сброса он остаётся
/// установленным — сохранение повторяется на каждой итерации цикла (диалог
/// открывается бесконечно). Хук подменяет рутину целиком, поэтому сброс
/// делаем сами.
///
/// Возвращает «не загружено» (A=0, Z=1), как и родная рутина при обычном
/// возврате, чтобы главный цикл не уходил на ветку переинициализации вида
/// (`jp nz, 0x7394` у Кладова, `jp nz, loc_71BE` в оригинале).
fn menu_save_load(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    let a = layout.control_key1 as usize;
    m.mem[a..a + PRESSED_KEY_BYTES].fill(0);
    m.save_requested = true;
    cpu.a = 0;
    cpu.f |= ZF;
    0
}

/// Нативная замена `MenuSaveLoad` в версии Кладова (`0xD04E`).
fn menu_save_load_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, KLADOV_SAVE)
}

/// Нативная замена `MenuSaveLoad` в оригинальной версии (`0xD2E4`).
fn menu_save_load_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, ELITE_SAVE)
}

/// Точное число тактов Z80, которое рутина `sub_5DFB` занимает для кода `a`
/// (включая завершающий `RET`).
fn font_base_cycles(a: u8) -> u64 {
    match a {
        0x20 => 28,           // пробел: cp 20h -> ret z (taken)
        _ if a < 0x30 => 61,  // < '0'
        _ if a < 0x3B => 54,  // '0'..='9'
        _ if a < 0x41 => 87,  // ':'..'@'
        _ if a < 0x5B => 80,  // 'A'..'Z'
        _ if a < 0x61 => 113, // '['..'`'
        _ if a < 0x7B => 106, // a..z
        _ => 120,             // >= '{'
    }
}

/// Нативная замена `sub_5DFB` для механизма хуков.
fn font_base_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    #[cfg(test)]
    FONT_BASE_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let hl = font_base(cpu.a);
    cpu.h = (hl >> 8) as u8;
    cpu.l = (hl & 0xFF) as u8;
    font_base_cycles(cpu.a)
}

/// Счётчик обращений к хуку (только для тестов).
#[cfg(test)]
static FONT_BASE_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `sub_EB3E`: `ld d,0; ld e,a; add hl,de; ret` — прибавляет к `HL`
/// беззнаковый `A` (через `DE`), выставляя флаги сложения. Флаги `S/Z/PV`
/// 16-битного `ADD HL,rr` не трогает.
fn eb3e_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    cpu.d = 0;
    cpu.e = cpu.a;
    let r = cpu.add16(cpu.hl(), cpu.de());
    cpu.h = (r >> 8) as u8;
    cpu.l = r as u8;
    32 // ld d,0(7) + ld e,a(4) + add hl,de(11) + ret(10)
}

/// `sub_D3CE`: `push hl; sbc hl,de; pop hl; ret` — выставляет флаги по
/// `HL - DE - C` (16-битное вычитание с заёмом), но `HL` оставляет прежним.
fn d3ce_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = cpu.hl();
    let de = cpu.de();
    cpu.sbc16(hl, de);
    46 // push hl(11) + sbc hl,de(15) + pop hl(10) + ret(10)
}

/// Таблица умножения (`0xBF00`), к которой обращается `Mult_A_hiB_C`.
const MULT_TABLE: usize = 0xBF00;

/// Нативная замена `Mult_A_hiB_C` (`0xAB0B`).
///
/// Табличное умножение через таблицу квадратов по адресу `0xBF00`.
///
/// ```text
/// ld a,b ; sub c ; jr nc,keep ; neg ; keep:
/// srl a ; ld h,0BFh ; ld l,a ; ld d,(hl)
/// ld a,b ; add a,c ; rra ; ld l,a ; ld a,(hl) ; sub d ; ret
/// ```
///
/// Вход: `B`, `C`. Выход: `A = table[(B+C)/2] - table[|B-C|/2]`, `H=0xBF`,
/// `L=(B+C)/2`, `D=table[|B-C|/2]`. `B`, `C` не изменяются.
fn mult_a_hib_c_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let (b, c) = (cpu.b, cpu.c);
    let no_borrow = b >= c; // результат `sub c`: CF=0
    let diff = if no_borrow { b - c } else { c - b };
    let d = m.mem[MULT_TABLE + (diff >> 1) as usize];
    let idx = ((b as u16 + c as u16) >> 1) as u8; // `add a,c` + `rra`
    let a2 = m.mem[MULT_TABLE + idx as usize];
    cpu.a = cpu.sub8(a2, d, 0);
    cpu.h = 0xBF;
    cpu.l = idx;
    cpu.d = d;
    // `jr nc` — taken при отсутствии заёма (b >= c), тогда `neg` не выполняется.
    if no_borrow { 83 } else { 86 }
}

/// `sub_E621` (`PixCrd2ScrAdr` в оригинале): пиксельные координаты -> адрес
/// экранной области ZX Spectrum.
///
/// Вход: `X = A` (пиксельная колонка 0..255), `Y = C` (пиксельная строка
/// 0..191). Выход: `HL` — адрес байта битмапа, содержащего пиксель.
pub fn pix_crd_2_scr_adr(x: u8, y: u8) -> u16 {
    let l = ((y & 0x38) << 2) | ((x >> 3) & 0x1F);
    let h = 0x40 | (y & 7) | ((y & 0xC0) >> 3);
    ((h as u16) << 8) | l as u16
}

/// Нативная замена `sub_E621`. Рутина полностью линейная (ветвлений нет),
/// поэтому такты фиксированы (120). Единственная операция, задающая итоговые
/// флаги, — финальное `or h`; флаги `rra`/`and`/`add` до неё перезаписываются.
fn pix_crd_2_scr_adr_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = pix_crd_2_scr_adr(cpu.a, cpu.c);
    let h = (hl >> 8) as u8;
    let a_before = ((cpu.c & 0xC0) >> 3) & 0x1F; // результат `and 1Fh`
    cpu.a = cpu.or8(a_before, h); // финальное `or h` (выставляет флаги)
    cpu.h = h;
    cpu.l = hl as u8;
    120
}

/// `scr_adr_next_char_line` (`compDownScrAdr_HL`, `0x8445`): продолжение
/// расчёта экранного адреса при переходе на строку чартов ниже.
///
/// ```text
/// ld a,l ; add a,20h ; ld l,a ; ret c ; ld a,h ; sub 8 ; ld h,a ; ret
/// ```
fn scr_adr_next_char_line_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let sum = cpu.add8(cpu.l, 0x20, 0);
    cpu.a = sum;
    cpu.l = sum;
    if cpu.f & CF != 0 {
        return 26; // ld a,l(4)+add a,20h(7)+ld l,a(4)+ret c(11)
    }
    let d = cpu.sub8(cpu.h, 8, 0);
    cpu.a = d;
    cpu.h = d;
    45 // 4+7+4+5 + 4+7+4+10
}

/// `scr_adr_next_line` (`sub_7FE7`, `0x7FE7`): переход экранного адреса на
/// одну пиксельную строку вниз.
///
/// ```text
/// inc h ; ld a,h ; and 7 ; call z, compDownScrAdr_HL ; ret
/// ```
fn scr_adr_next_line_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let h = cpu.h.wrapping_add(1);
    cpu.h = h;
    let a = cpu.and8(h, 7);
    cpu.a = a;
    if a != 0 {
        return 35; // inc h(4)+ld a,h(4)+and 7(7)+call z not taken(10)+ret(10)
    }
    let inner = scr_adr_next_char_line_hook(cpu, m);
    // 4+4+7+17 (вход в ветку) + 10 (финальный ret)
    42 + inner
}

/// `sub_5DFB`: по коду символа в `A` выбирает базовый адрес шрифта.
///
/// Возвращает значение `HL`: `0x3D00` для «печатных» символов (пробел, цифры
/// `0`–`9`, латиница `A`–`Z`/`a`–`z`) и `0x5D70` для остальных. Регистр `A` не
/// изменяется.
pub fn font_base(a: u8) -> u16 {
    let printable = a == 0x20
        || (0x30..=0x3A).contains(&a)
        || (0x41..=0x5A).contains(&a)
        || (0x61..=0x7A).contains(&a);
    if printable { 0x3D00 } else { 0x5D70 }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Загружает полный образ Кладова (для прогона настоящих рутин на Z80).
    fn kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let data = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(None);
        m.load(0x4000, &data);
        m
    }

    /// Вызывает рутину по адресу `addr` как подпрограмму (`CALL`/`RET`) и
    /// выполняет её на Z80 до возврата в адрес-маркер.
    fn run_routine(m: &mut Spectrum, addr: u16) {
        const RET: u16 = 0x1234;
        m.cpu.sp = 0xF000;
        m.cpu.sp = m.cpu.sp.wrapping_sub(2);
        let [lo, hi] = RET.to_le_bytes();
        m.mem[m.cpu.sp as usize] = lo;
        m.mem[m.cpu.sp as usize + 1] = hi;
        m.cpu.pc = addr;
        let mut cpu = std::mem::take(&mut m.cpu);
        let mut guard = 0;
        while cpu.pc != RET && guard < 10_000 {
            cpu.step(m);
            guard += 1;
        }
        assert_eq!(cpu.pc, RET, "рутина 0x{addr:04X} не вернулась (guard={guard})");
        m.cpu = cpu;
    }

    /// Готовит машину к запуску версии Кладова так же, как `main.rs`.
    fn boot_kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = crate::KLADOV_STATE_A;
        c.f = crate::KLADOV_STATE_F;
        c.b = (crate::KLADOV_STATE_BC >> 8) as u8;
        c.c = (crate::KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (crate::KLADOV_STATE_DE >> 8) as u8;
        c.e = (crate::KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (crate::KLADOV_STATE_HL >> 8) as u8;
        c.l = (crate::KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = crate::KLADOV_STATE_IX;
        c.iy = crate::KLADOV_STATE_IY;
        c.sp = crate::KLADOV_STATE_SP;
        c.pc = crate::KLADOV_STATE_PC;
        c.i = crate::KLADOV_I;
        c.r = crate::KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        m
    }

    /// Готовит машину к запуску оригинальной версии так же, как `main.rs`:
    /// образ по адресу `0x6048`, таблица IM2 на обработчик `0xFF25` и подмена
    /// опроса Kempston (`in a,(c)` -> `ld a,2` в `BeginGame`), без которой игра
    /// останавливается на выборе устройства управления.
    fn boot_elite() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/elite.bin"))
            .expect("zx_bin/elite.bin (собрать: ./build.sh)");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::GAME_LOAD_ADDR, &game);
        let [lo, hi] = crate::INTERRUPT_HANDLER.to_le_bytes();
        for i in 0..128u16 {
            let a = crate::IM2_TABLE + i * 2;
            m.mem[a as usize] = lo;
            m.mem[a as usize + 1] = hi;
        }
        m.mem[0x71F8] = 0x3E;
        m.mem[0x71F9] = 0x02;
        m.cpu.pc = crate::GAME_ENTRY;
        m.cpu.sp = crate::GAME_SP;
        m
    }

    /// Полный прогон с хуками обязан побайтово совпасть с обычным прогоном:
    /// такты, регистры и содержимое экранной области.
    #[test]
    fn hooks_match_plain_run() {
        FONT_BASE_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);
        let mut plain = boot_kladov();
        let mut hooked = boot_kladov();
        install(&mut hooked, true);
        for _ in 0..120 {
            plain.run_frame();
            hooked.run_frame();
        }
        assert!(
            FONT_BASE_CALLS.load(std::sync::atomic::Ordering::Relaxed) > 0,
            "хук font_base ни разу не вызван за 120 кадров"
        );
        assert_eq!(plain.cpu.cycles, hooked.cpu.cycles, "такт разошёлся");
        assert_eq!(plain.cpu.pc, hooked.cpu.pc, "PC разошёлся");
        assert_eq!(plain.cpu.sp, hooked.cpu.sp, "SP разошёлся");
        assert_eq!(plain.cpu.hl(), hooked.cpu.hl(), "HL разошёлся");
        assert_eq!(plain.cpu.bc(), hooked.cpu.bc(), "BC разошёлся");
        assert_eq!(plain.cpu.de(), hooked.cpu.de(), "DE разошёлся");
        assert_eq!(
            &plain.mem[0x4000..0x5B00],
            &hooked.mem[0x4000..0x5B00],
            "экран разошёлся"
        );
    }

    /// Снимок регистров и тактов для строгого сравнения.
    #[derive(Debug, PartialEq)]
    struct Snap {
        a: u8,
        f: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        h: u8,
        l: u8,
        cycles: u64,
    }

    fn snap(c: &Z80) -> Snap {
        Snap {
            a: c.a,
            f: c.f,
            b: c.b,
            c: c.c,
            d: c.d,
            e: c.e,
            h: c.h,
            l: c.l,
            cycles: c.cycles,
        }
    }

    fn set(c: &mut Z80, a: u8, hl: u16, de: u16, f: u8) {
        c.a = a;
        c.f = f;
        c.h = (hl >> 8) as u8;
        c.l = hl as u8;
        c.d = (de >> 8) as u8;
        c.e = de as u8;
    }

    /// Прогон рутины на Z80 со счётчиком тактов от нуля.
    fn z80_sub(m: &mut Spectrum, addr: u16) -> Snap {
        m.cpu.cycles = 0;
        run_routine(m, addr);
        snap(&m.cpu)
    }

    /// Прогон нативной рутины через таблицу хуков.
    fn hook_sub(m: &mut Spectrum, addr: u16) -> Snap {
        let mut cpu = std::mem::take(&mut m.cpu);
        cpu.cycles = 0;
        let cyc = (m.hooks[&addr])(&mut cpu, m);
        cpu.cycles = cyc;
        m.cpu = cpu;
        snap(&m.cpu)
    }

    /// Регистровые рутины обязаны совпасть с Z80 по ВСЕМ регистрам, флагам
    /// и числу тактов.
    #[test]
    fn register_helpers_match_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        let cases: &[(u8, u16, u16, u8)] = &[
            (0x00, 0x0000, 0x0000, 0x00),
            (0x01, 0x0001, 0x0000, 0x00),
            (0xFF, 0x00FF, 0x0001, 0x01),
            (0x80, 0xFF00, 0x00FF, 0x00),
            (0x7F, 0x0FFF, 0x0001, 0x00),
            (0x55, 0x8000, 0x8000, 0x01),
            (0xAA, 0x1234, 0x5678, 0x00),
            (0x00, 0xFFFF, 0x0001, 0x01),
            (0x03, 0x00FF, 0xFF00, 0x00),
        ];
        for &(a, hl, de, f) in cases {
            for &addr in &[0xEB3Eu16, 0xD3CEu16] {
                set(&mut z.cpu, a, hl, de, f);
                set(&mut n.cpu, a, hl, de, f);
                let zs = z80_sub(&mut z, addr);
                let ns = hook_sub(&mut n, addr);
                assert_eq!(
                    zs, ns,
                    "рутина 0x{addr:04X}: вход a={a:02X} hl={hl:04X} de={de:04X} f={f:02X}"
                );
            }
        }
    }

    /// `Mult_A_hiB_C` обязан совпасть с Z80 на всех 256×256 парах `B`,`C`
    /// по всем регистрам, флагам и числу тактов.
    #[test]
    fn mult_a_hib_c_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                z.cpu.b = b;
                z.cpu.c = c;
                z.cpu.f = b ^ c;
                n.cpu.b = b;
                n.cpu.c = c;
                n.cpu.f = b ^ c;
                let zs = z80_sub(&mut z, 0xAB0B);
                let ns = hook_sub(&mut n, 0xAB0B);
                assert_eq!(zs, ns, "Mult_A_hiB_C: b={b:02X} c={c:02X}");
            }
        }
    }

    /// `sub_E621` (`PixCrd2ScrAdr`) обязан совпасть с Z80 по всем регистрам,
    /// флагам и тактам для всех пар `X` (A) и `Y` (C).
    #[test]
    fn pix_crd_2_scr_adr_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        for x in 0u8..=255 {
            for y in 0u8..=255 {
                z.cpu.a = x;
                z.cpu.c = y;
                z.cpu.f = x ^ y; // произвольные входные флаги
                n.cpu.a = x;
                n.cpu.c = y;
                n.cpu.f = x ^ y;
                let zs = z80_sub(&mut z, 0xE621);
                let ns = hook_sub(&mut n, 0xE621);
                assert_eq!(zs, ns, "PixCrd2ScrAdr: x={x:02X} y={y:02X}");
            }
        }
    }

    #[test]
    fn scr_adr_next_char_line_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        for hl in 0..=0xFFFFu16 {
            let h = (hl >> 8) as u8;
            let l = hl as u8;
            z.cpu.h = h;
            z.cpu.l = l;
            z.cpu.a = 0x55;
            z.cpu.b = 0xAA;
            z.cpu.c = 0x33;
            z.cpu.d = 0xCC;
            z.cpu.e = 0x77;
            z.cpu.f = hl as u8 ^ ((hl >> 8) as u8);
            n.cpu.h = h;
            n.cpu.l = l;
            n.cpu.a = 0x55;
            n.cpu.b = 0xAA;
            n.cpu.c = 0x33;
            n.cpu.d = 0xCC;
            n.cpu.e = 0x77;
            n.cpu.f = z.cpu.f;
            let zs = z80_sub(&mut z, 0x8445);
            let ns = hook_sub(&mut n, 0x8445);
            assert_eq!(zs, ns, "compDownScrAdr_HL hl=0x{hl:04X}");
        }
    }

    #[test]
    fn scr_adr_next_line_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        for hl in 0..=0xFFFFu16 {
            let h = (hl >> 8) as u8;
            let l = hl as u8;
            z.cpu.h = h;
            z.cpu.l = l;
            z.cpu.a = 0x12;
            z.cpu.b = 0x34;
            z.cpu.c = 0x56;
            z.cpu.d = 0x78;
            z.cpu.e = 0x9A;
            z.cpu.f = hl as u8;
            n.cpu.h = h;
            n.cpu.l = l;
            n.cpu.a = 0x12;
            n.cpu.b = 0x34;
            n.cpu.c = 0x56;
            n.cpu.d = 0x78;
            n.cpu.e = 0x9A;
            n.cpu.f = z.cpu.f;
            let zs = z80_sub(&mut z, 0x7FE7);
            let ns = hook_sub(&mut n, 0x7FE7);
            assert_eq!(zs, ns, "sub_7FE7 hl=0x{hl:04X}");
        }
    }

    #[test]
    fn font_base_matches_z80() {
        let mut m = kladov();
        for a in 0u8..=255 {
            m.cpu.a = a;
            run_routine(&mut m, 0x5DFB);
            let z80 = m.cpu.hl();
            let native = font_base(a);
            assert_eq!(
                native, z80,
                "font_base(0x{a:02X}): native=0x{native:04X} z80=0x{z80:04X}"
            );
        }
    }

    /// Сохранение и загрузка блока командира в файл — round-trip (обе версии).
    #[test]
    fn commander_roundtrip() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        std::fs::create_dir_all(&dir).unwrap();

        for layout in [KLADOV_SAVE, ELITE_SAVE] {
            let path = format!("{dir}/commander_roundtrip_{:04X}.zxelite", layout.pilot_name);
            let mut m = kladov();
            let a = layout.pilot_name as usize;
            let n = layout.commander_len;
            m.mem[a] = b'T';
            m.mem[a + 1] = b'E';
            m.mem[a + 2] = b'S';
            m.mem[a + 3] = b'T';
            m.mem[a + n - 1] = 0x5A;
            let expected: Vec<u8> = {
                let mut ex = m.mem[a..a + n].to_vec();
                let sc = layout.sun_crd as usize;
                ex[SUN_FIELD_OFF..SUN_FIELD_OFF + 9].copy_from_slice(&m.mem[sc..sc + 9]);
                ex
            };
            assert_eq!(expected.len(), n);

            // Синхронизация тени при сохранении: `InitGame` восстанавливает
            // пилота именно из неё, поэтому после сохранения она должна
            // совпадать с живым блоком.
            let shadow_a = layout.pilot_shadow.map(|(addr, _)| addr as usize);
            commander_save(&mut m, &path, layout).unwrap();
            if let Some(sa) = shadow_a {
                let sn = layout.commander_len.min(layout.pilot_shadow.unwrap().1);
                // После сохранения теневой буфер должен полностью совпадать с
                // живым блоком (оригинал: SaveGame копирует PilotName→PilotDataSaved).
                let live_now = &m.mem[a..a + sn];
                assert_eq!(
                    &m.mem[sa..sa + sn],
                    live_now,
                    "теневой буфер (0x{sa:04X}) не синхронизировался при сохранении"
                );
            }
            for b in &mut m.mem[a..a + n] {
                *b = 0xAA;
            }
            commander_load(&mut m, &path, layout).unwrap();
            assert_eq!(
                commander_block(&m, layout),
                &expected[..],
                "блок командира (0x{:04X}) не совпал",
                layout.pilot_name
            );

            let _ = std::fs::remove_file(&path);
        }
    }

    /// Перехват `MenuSaveLoad`: хук должен выставить запрос на сохранение и
    /// сбросить «залипшие» клавиши (`ControlKey1`..`ControlKey3`), как это
    /// делает родная рутина через `ResetPressedKeys`. Проверяем обе версии.
    #[test]
    fn menu_save_load_hook_sets_request() {
        for layout in [KLADOV_SAVE, ELITE_SAVE] {
            let mut m = kladov();
            install_save(&mut m, layout);
            assert!(
                m.hooks.contains_key(&layout.menu_save_load),
                "хук MenuSaveLoad 0x{:04X} не установлен",
                layout.menu_save_load
            );
            assert!(!m.save_requested);
            // Имитируем нажатие клавиши сохранения (Symbol Shift -> бит 5
            // `ControlKeyMove`) и других залипших клавиш.
            let a = layout.control_key1 as usize;
            m.mem[a] = 0x50;
            m.mem[a + 1] = 0x20;
            m.mem[a + 2] = 0x80;
            m.mem[a + 3] = 0x01;
            let hook = m.hooks.get(&layout.menu_save_load).copied().unwrap();
            let mut cpu = std::mem::take(&mut m.cpu);
            let cycles = hook(&mut cpu, &mut m);
            m.cpu = cpu;
            assert_eq!(cycles, 0);
            assert!(m.save_requested, "запрос на сохранение не выставлен");
            assert_eq!(
                &m.mem[a..a + PRESSED_KEY_BYTES],
                &[0u8; PRESSED_KEY_BYTES][..],
                "залипшие клавиши не сброшены (0x{:04X})",
                layout.control_key1
            );
        }
    }

    /// Прогон сценария «нажали клавишу сохранения» и подсчёт запросов.
    fn count_save_requests(m: &mut Spectrum) -> usize {
        let mut requests = 0;
        for f in 0..260 {
            m.release_all_keys();
            if (30..80).contains(&f) {
                m.set_key(7, 0, true); // SPACE — старт полёта с титула
            }
            if f == 150 {
                m.set_key(7, 1, true); // Symbol Shift — клавиша сохранения (1 кадр)
            }
            m.run_frame();
            if m.save_requested {
                m.save_requested = false;
                requests += 1;
            }
        }
        requests
    }

    /// Регрессия на бесконечный цикл сохранения: одно нажатие клавиши
    /// сохранения (Symbol Shift) должно дать ровно один запрос, а не
    /// повторяться на каждой итерации главного цикла. Проверяем обе версии.
    #[test]
    fn save_request_fires_once_per_press() {
        let mut kladov = boot_kladov();
        install_save(&mut kladov, KLADOV_SAVE);
        let requests = count_save_requests(&mut kladov);
        assert_eq!(
            requests, 1,
            "Кладова: сохранение сработало {requests} раз вместо одного"
        );

        let mut elite = boot_elite();
        install_save(&mut elite, ELITE_SAVE);
        let requests = count_save_requests(&mut elite);
        assert_eq!(
            requests, 1,
            "оригинал: сохранение сработало {requests} раз вместо одного"
        );
    }

    /// Загрузка прерывает текущий процесс и перезапускает игру: пилот из файла
    /// сохраняется, а после стартовой последовательности корабль пристыкован
    /// (`IndDanger=0`) — то есть мы на станции. Проверяем обе версии.
    #[test]
    fn load_restarts_docked_with_saved_pilot() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        std::fs::create_dir_all(&dir).unwrap();

        for (layout, ind_danger, mut m) in [
            (KLADOV_SAVE, 0x7104u16, boot_kladov()),
            (ELITE_SAVE, 0x7062u16, boot_elite()),
        ] {
            install_save(&mut m, layout);

            // Стартовая последовательность сначала ждёт отпускания клавиши, а
            // затем её нажатия (выбор устройства управления). Подаём SPACE
            // импульсами, чтобы попасть в оба состояния.
            let impulse = |m: &mut Spectrum, f: u64| {
                m.release_all_keys();
                if f >= 30 && f % 16 < 6 {
                    m.set_key(7, 0, true);
                }
            };

            // Приводим игру в рабочее состояние и имитируем полёт.
            for f in 0..150u64 {
                impulse(&mut m, f);
                m.run_frame();
            }
            m.mem[ind_danger as usize] = 4; // «в полёте» (вылет со станции)
            assert_ne!(
                m.mem[ind_danger as usize], 0,
                "подготовка: корабль должен быть в полёте"
            );

            // Готовим файл: валидный командир из образа с помеченным именем.
            let mut data = commander_block(&m, layout).to_vec();
            data[0..8].copy_from_slice(b"TESTPILO");
            let path = format!("{dir}/load_restart_{:04X}.zxelite", layout.pilot_name);
            std::fs::write(&path, &data).unwrap();

            commander_load(&mut m, &path, layout).unwrap();
            restart_after_load(&mut m, layout);
            assert_eq!(
                m.cpu.pc, layout.restart_entry,
                "перезапуск не инициирован (0x{:04X})",
                layout.restart_entry
            );

            // Проходим стартовую последовательность до пристыковки.
            for f in 0..700u64 {
                impulse(&mut m, f);
                m.run_frame();
            }

            assert_eq!(
                &m.mem[layout.pilot_name as usize..][..8],
                b"TESTPILO",
                "загруженный пилот сброшен после перезапуска (0x{:04X})",
                layout.pilot_name
            );
            assert_eq!(
                m.mem[ind_danger as usize], 0,
                "после загрузки корабль не пристыкован (IndDanger 0x{:04X})",
                ind_danger
            );

            let _ = std::fs::remove_file(&path);
        }
    }

    /// Регрессия на сохранение из «мёртвого» блока.
    ///
    /// У Кладова блок командира живёт в `PilotName` (0xD054), а `aJameson_1`
    /// (0xD232) — тень, из которой `InitGame` восстанавливает пилота при
    /// старте. Если читать при сохранении тень, на диск уходит стартовое
    /// состояние «JAMESON», прогресс теряется, а после загрузки игра
    /// рисует солнце по устаревшим координатам из хвоста тени. Проверяем:
    /// сохранение берёт данные из живого блока, а загрузка возвращает их
    /// обратно (в т.ч. хвост 93..101, который `InitGame` не переносит).
    #[test]
    fn save_load_uses_live_commander_block() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        std::fs::create_dir_all(&dir).unwrap();

        // `live` — настоящий живой блок пилота по данным дизассемблирования,
        // независимый от того, что записано в раскладке: у Кладова это
        // назначение `ldir` из тени в `InitGame` (`ld de,0xD054`) и `ld de,
        // PilotName` в `DemoModeOn`; в оригинале — `ld de,PilotName` из
        // `RestorePilotFromMemory` (копия `0xA910`→`0xA816`).
        for (layout, live, ind_danger, mut m) in [
            (KLADOV_SAVE, 0xD054usize, 0x7104u16, boot_kladov()),
            (ELITE_SAVE, 0xA816usize, 0x7062u16, boot_elite()),
        ] {
            assert_eq!(
                layout.pilot_name as usize, live,
                "раскладка должна читать сохранение из живого блока"
            );
            install_save(&mut m, layout);
            // Имитируем полёт до пристыковки.
            for f in 0..150u64 {
                m.release_all_keys();
                if f >= 30 && f % 16 < 6 {
                    m.set_key(7, 0, true);
                }
                m.run_frame();
            }
            let a = live;
            const FREE_CARGO: usize = 40; // PilotFreeCargo
            const SUN: std::ops::Range<usize> = 93..102; // PilotDataOfSun

            // «Игрок» изменил трюм в живом блоке, а солнце на небе сдвинулось —
            // игра хранит его координаты в `SunCrd` и кладёт в блок только при
            // сохранении.
            let sun = [0xF2u8, 0x88, 0x05, 0x27, 0x08, 0x01, 0x2A, 0x4D, 0x8F];
            m.mem[a + FREE_CARGO] = 0x20;
            let sc = layout.sun_crd as usize;
            m.mem[sc..sc + 9].copy_from_slice(&sun);

            let path = format!("{dir}/live_block_{:04X}.zxelite", layout.pilot_name);
            commander_save(&mut m, &path, layout).unwrap();
            let file = std::fs::read(&path).unwrap();
            assert_eq!(
                file[FREE_CARGO], 0x20,
                "0x{:04X}: сохранение не взяло трюм из живого блока (0x{:02X})",
                layout.pilot_name, file[FREE_CARGO]
            );
            assert_eq!(
                &file[SUN.start..SUN.end], &sun,
                "0x{:04X}: сохранение не перенесло SunCrd в PilotDataOfSun",
                layout.pilot_name
            );

            // Загрузка и перезапуск: игра проходит стартовую последовательность
            // и пристыковывается, вернув загруженные данные.
            commander_load(&mut m, &path, layout).unwrap();
            restart_after_load(&mut m, layout);
            for f in 0..700u64 {
                m.release_all_keys();
                if f >= 30 && f % 16 < 6 {
                    m.set_key(7, 0, true);
                }
                m.run_frame();
            }
            let ind_danger = ind_danger as usize;
            assert_eq!(
                m.mem[ind_danger], 0,
                "после загрузки корабль не пристыкован (IndDanger 0x{ind_danger:04X})"
            );
            assert_eq!(
                m.mem[a + FREE_CARGO], 0x20,
                "0x{:04X}: трюм не вернулся после загрузки (0x{:02X})",
                layout.pilot_name, m.mem[a + FREE_CARGO]
            );
            assert_eq!(
                &m.mem[a + SUN.start..a + SUN.end], &sun,
                "0x{:04X}: координаты солнца не вернулись после загрузки",
                layout.pilot_name
            );
            // Главное для отрисовки: `InitGame` восстановила `SunCrd` из тени,
            // иначе солнце рисуется по старым координатам.
            let sc = layout.sun_crd as usize;
            assert_eq!(
                &m.mem[sc..sc + 9], &sun,
                "0x{:04X}: SunCrd (0x{:04X}) не восстановилась после загрузки",
                layout.pilot_name, layout.sun_crd
            );

            let _ = std::fs::remove_file(&path);
        }
    }



    /// ВРЕМЕННЫЙ: где лежат SunCrd и поле солнца в живом блоке.
    #[test]
    fn probe_sun_fields() {
        for (tag, layout, sun, mut m) in [
            ("kladov", KLADOV_SAVE, 0x7107u16, boot_kladov()),
            ("elite", ELITE_SAVE, 0x7065u16, boot_elite()),
        ] {
            install_save(&mut m, layout);
            for f in 0..150u64 {
                m.release_all_keys();
                if f >= 30 && f % 16 < 6 { m.set_key(7, 0, true); }
                m.run_frame();
            }
            let live = &m.mem[layout.pilot_name as usize..layout.pilot_name as usize + layout.commander_len];
            let pat = [0xF0u8, 0x52, 0x05, 0x33, 0x98, 0x07, 0xB6, 0xFA, 0x8B];
            let off = live.windows(9).position(|w| w == pat);
            println!("{tag}: SunCrd(0x{sun:04X})={} ; паттерн PilotDataOfSun в живом блоке на смещении {off:?}",
                m.mem[sun as usize..sun as usize + 9].iter().map(|b| format!("{b:02X}")).collect::<String>());
            println!("{tag}: живой блок[93..102]={} ; тень[93..102]={}",
                live[93..102].iter().map(|b| format!("{b:02X}")).collect::<String>(),
                m.mem[layout.pilot_shadow.unwrap().0 as usize + 93..layout.pilot_shadow.unwrap().0 as usize + 102].iter().map(|b| format!("{b:02X}")).collect::<String>());
        }
    }
}
