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

use crate::machine::{Hook, Spectrum, ROM_SIZE};
use crate::z80::{Bus, Z80, ZF, CF, HF, PF, SF, XF, YF};

/// Установить все нативные замены (хуки) на машину.
///
/// Адреса регистровых помощников соответствуют версии Кладова; для
/// оригинальной версии (`--elite`) хук по этим адресам был бы установлен на
/// чужой код, поэтому сейчас ставим их только если `kladov == true`.
pub fn install(m: &mut Spectrum, kladov: bool) {
    if !kladov {
        return;
    }
    install_hook(m, 0x5DFB, font_base_hook, 120);
    install_hook(m, 0xEB3E, eb3e_hook, 32);
    install_hook(m, 0xD3CE, d3ce_hook, 46);
    install_hook(m, 0xAB0B, mult_a_hib_c_hook, 86);
    install_hook(m, 0xE621, pix_crd_2_scr_adr_hook, 120);
    install_hook(m, 0x7A59, smag_add_mem_hook, 148);
    install_hook(m, 0x7FE7, scr_adr_next_line_hook, 87);
    install_hook(m, 0x8445, scr_adr_next_char_line_hook, 45);
    install_hook(m, 0xD546, scr_xor_col_down_hook, 48 * 256 + 5);
    // Копия длиннее кадра, поэтому хук внутри себя отдаёт управление Z80
    // (Yield) на границе кадра; верхняя оценка — одна итерация цикла.
    install_hook(m, 0x7FCD, scr_copy_alt_lines_hook, COPY_ALT_MAX_ITER);
    // Звуковой автомат: чанк эффекта тоже длиннее кадра, хук отдаёт Z80
    // каждую инструкцию, что не влезает в остаток кадра (оценка — одна
    // инструкция рутины).
    install_hook(m, 0x9579, sfx_update_hook, SFX_MAX_STEP);
}

/// Регистрирует хук вместе с верхней оценкой его тактов (см. `hook_max`).
fn install_hook(m: &mut Spectrum, addr: u16, run: Hook, max_cycles: u64) {
    m.hooks.insert(addr, run);
    m.hook_max.insert(addr, max_cycles);
}

/// Эмуляция обновления регистра `R` (Memory Refresh) за заменённые инструкции.
///
/// На Z80 `R` инкрементируется на единицу за каждую выполняемую инструкцию
/// (бит 7 сохраняется). Хук подменяет рутину целиком и выполняет ноль
/// инструкций, поэтому `R` нужно двигать самим — иначе расходится поток
/// случайных чисел: игра читает `R` через `ld a, r` в `PrintStars` (дрожание
/// звёзд), и любой пропущенный сдвиг в итоге меняет экран.
///
/// `steps` — число инструкций (не такт заменённого кода), `ED`/`CB`-префиксы
/// считаются за одну инструкцию, как в `Z80::step`.
#[inline]
fn bump_r(cpu: &mut Z80, steps: u32) {
    cpu.r = (cpu.r & 0x80) | (cpu.r.wrapping_add(steps as u8) & 0x7F);
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
    let a = cpu.a;
    bump_r(cpu, font_base_steps(a));
    font_base_cycles(a)
}

/// Число инструкций `sub_5DFB` по пути, выбираемому символом `A` (для
/// эмуляции `R`; пути и границы совпадают с `font_base_cycles`).
fn font_base_steps(a: u8) -> u32 {
    match a {
        0x20 => 3,
        _ if a < 0x30 => 7,
        _ if a < 0x3B => 7,
        _ if a < 0x41 => 11,
        _ if a < 0x5B => 11,
        _ if a < 0x61 => 15,
        _ if a < 0x7B => 15,
        _ => 17,
    }
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
    bump_r(cpu, 4); // ld d,0; ld e,a; add hl,de; ret
    32 // ld d,0(7) + ld e,a(4) + add hl,de(11) + ret(10)
}

/// `sub_D3CE`: `push hl; sbc hl,de; pop hl; ret` — выставляет флаги по
/// `HL - DE - C` (16-битное вычитание с заёмом), но `HL` оставляет прежним.
fn d3ce_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = cpu.hl();
    let de = cpu.de();
    cpu.sbc16(hl, de);
    bump_r(cpu, 4); // push hl; sbc hl,de; pop hl; ret
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
    bump_r(cpu, if no_borrow { 14 } else { 15 });
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
    bump_r(cpu, 24); // рутина линейная: 24 инструкции
    120
}

/// `smag_add_mem` (`sub_7A59`): прибавить к байту по `HL` значение `A` в
/// формате «знак + 7-битная величина» (знак — бит 7, величина — биты 0..6).
/// Рутина горячая: вызывается на каждое обновление координаты звезды в
/// `PrintStars` (звёздные координаты хранятся именно в этом формате).
///
/// Правила (по эталонному коду):
/// - одинаковые знаки: величины складываются; если сумма ≥ 80h, величина
///   обнуляется, знак сохраняется;
/// - разные знаки: величины вычитаются, знак результата — от большей величины
///   (при равенстве — от прежнего значения по адресу).
///
/// Результат записывается в `(HL)`, `BC` сохраняются (рутина делает
/// `push bc`/`pop bc`).
pub fn smag_add_mem(old: u8, a: u8) -> u8 {
    if (a ^ old) & 0x80 == 0 {
        let sum = (a & 0x7F) + (old & 0x7F);
        if sum & 0x80 != 0 { a & 0x80 } else { (a & 0x80) | sum }
    } else if old & 0x7F >= a & 0x7F {
        (old & 0x80) | (old & 0x7F) - (a & 0x7F)
    } else {
        (a & 0x80) | (a & 0x7F) - (old & 0x7F)
    }
}

/// Нативная замена `sub_7A59`. Итоговые флаги задаёт финальное `or c`
/// (операции `xor`/`and`/`add`/`sub`/`bit`/`neg` перезаписываются).
fn smag_add_mem_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let hl = cpu.hl() as usize;
    let old = m.mem[hl];
    let a = cpu.a;
    let res = smag_add_mem(old, a);
    m.mem[hl] = res;
    cpu.a = cpu.or8(res, 0); // финальные флаги — от `or c`
    let same_sign = (a ^ old) & 0x80 == 0;
    if same_sign {
        // без переполнения: 20 инструкций (132 такта); с переполнением
        // (`sbc a,a`): 21 инструкция (131 такт).
        if ((a & 0x7F) + (old & 0x7F)) & 0x80 == 0 {
            bump_r(cpu, 20);
            132
        } else {
            bump_r(cpu, 21);
            131
        }
    } else if old & 0x7F >= a & 0x7F {
        bump_r(cpu, 21); // `jr nc` берётся, `neg` не выполняется
        148
    } else {
        bump_r(cpu, 21);
        144
    }
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
        bump_r(cpu, 4); // ld a,l; add a,20h; ld l,a; ret c
        return 26; // ld a,l(4)+add a,20h(7)+ld l,a(4)+ret c(11)
    }
    let d = cpu.sub8(cpu.h, 8, 0);
    cpu.a = d;
    cpu.h = d;
    bump_r(cpu, 8); // + ld a,h; sub 8; ld h,a; ret
    45 // 4+7+4+5 + 4+7+4+10
}

/// `scr_adr_next_line` (`sub_7FE7`, `0x7FE7`): переход экранного адреса на
/// одну пиксельную строку вниз.
///
/// ```text
/// inc h ; ld a,h ; and 7 ; call z, compDownScrAdr_HL ; ret
/// ```
///
/// Это регистровый примитив: им пользуется и хук адреса `0x7FE7`, и хук
/// `0x7FCD`, который сам ведёт резиду стека в нужные слоты.
fn scr_adr_next_line_advance(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let h = cpu.h.wrapping_add(1);
    cpu.h = h;
    let a = cpu.and8(h, 7);
    cpu.a = a;
    // Свои инструкции: inc h, ld a,h, and 7, call z (взята или нет) и
    // финальный `ret` — всегда 5; внутренний хук добавляет свои 4/8.
    bump_r(cpu, 5);
    if a != 0 {
        return 35; // inc h(4)+ld a,h(4)+and 7(7)+call z not taken(10)+ret(10)
    }
    let inner = scr_adr_next_char_line_hook(cpu, m);
    // 4+4+7+17 (вход в ветку) + 10 (финальный ret)
    42 + inner
}

/// Хук адреса `0x7FE7`: примитив `scr_adr_next_line_advance` плюс резида
/// стека от `call z, compDownScrAdr_HL` — по [SP-2] ложится адрес возврата
/// 7FEEh, как у настоящего `call`. В игре стек живёт в экранной памяти
/// (SP=4116h..411Eh), и этот байт виден на экране, поэтому его нужно
/// воспроизводить. Хук работает и для обычного `call 0x7FE7h`, и для
/// jp-режима, который игра включает самомодификацией (см. `0xDA2F`).
fn scr_adr_next_line_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let taken = cpu.h.wrapping_add(1) & 7 == 0;
    let cycles = scr_adr_next_line_advance(cpu, m);
    if taken {
        let sp = cpu.sp;
        mem_write(m, sp.wrapping_sub(2), 0xEE);
        mem_write(m, sp.wrapping_sub(1), 0x7F);
    }
    cycles
}

/// Адрес самомодифицирующегося опкода `xor c` внутри `sub_D546`.
const XOR_COL_OP: usize = 0xD548;
/// Значение опкода `xor c` (режим XOR); игра пишет на это место только `00h`
/// (`nop` — режим копирования) и `A9h` (`xor c` — режим XOR).
const XOR_COL_XOR_OP: u8 = 0xA9;

/// `scr_xor_col_down` (`sub_D546`): отрисовка вертикальной колонки из `B`
/// байтов: `(HL+i*100h) ^= (DE+i)` (режим XOR) либо `(HL+i*100h) = (DE+i)`
/// (режим копирования).
///
/// ```text
/// loop: ld a,(de) ; ld c,(hl) ; xor c ; ld (hl),a ; inc h ; inc de
///       djnz loop
/// ret
/// ```
///
/// Рутина **самоизменяется**: по адресу `0xD548` лежит опкод `xor c`,
/// который игра переключает между `A9h` (XOR — двухпроходное рисование
/// спрайтов, вторая проходка стирает первый) и `00h` (`nop` — обычное
/// копирование). Значение читается из памяти в момент вызова. Других значений
/// игра на это место не пишет.
///
/// `B = 0` означает 256 проходов (как у `djnz`). `inc h` двигает адрес вниз
/// по колонке (только старший байт, `L` не меняется). Записи в адреса ниже
/// `writable_from` (область ПЗУ) отбрасываются — так же поступает шина
/// настоящего Spectrum. Возвращает итоговые `(A, C, HL, DE)`: `A` — последний
/// вычисленный байт (пишется он или нет), `C` — содержимое экрана до записи
/// в последнем проходе, `B = 0`.
pub fn scr_xor_col_down(
    mem: &mut [u8],
    hl: u16,
    de: u16,
    b: u8,
    xor_mode: bool,
    writable_from: usize,
) -> (u8, u8, u16, u16) {
    let mut hl = hl;
    let mut de = de;
    let mut a = 0u8;
    let mut c = 0u8;
    let mut n = if b == 0 { 256u32 } else { b as u32 };
    while n != 0 {
        a = mem[de as usize];
        c = mem[hl as usize];
        if xor_mode {
            a ^= c;
        }
        if hl as usize >= writable_from {
            mem[hl as usize] = a;
        }
        hl = hl.wrapping_add(0x0100); // `inc h`: только старший байт, L не меняется
        de = de.wrapping_add(1);
        n -= 1;
    }
    (a, c, hl, de)
}

/// Нативная замена `sub_D546`. Итоговые флаги задаёт последний `inc h`
/// (`SZ5.3` и `PV` переполнения от итогового `H`); `xor c` всегда обнуляет
/// `C`, а в режиме копирования ни одна инструкция цикла флаги не трогает —
/// `C` остаётся от входа в рутину.
fn scr_xor_col_down_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let n = if cpu.b == 0 { 256u64 } else { cpu.b as u64 };
    let xor_mode = m.mem[XOR_COL_OP] == XOR_COL_XOR_OP;
    let cf_in = cpu.f & CF;
    let (a, c, hl, de) =
        scr_xor_col_down(&mut m.mem, cpu.hl(), cpu.de(), cpu.b, xor_mode, ROM_SIZE);
    cpu.a = a;
    cpu.c = c;
    cpu.b = 0;
    cpu.h = (hl >> 8) as u8;
    cpu.d = (de >> 8) as u8;
    cpu.e = de as u8;
    let h = (hl >> 8) as u8;
    cpu.f = (h & (SF | XF | YF))
        | if h == 0 { ZF } else { 0 }
        | if h == 0x80 { PF } else { 0 }
        | if h & 0x0F == 0 { HF } else { 0 } // полуперенос последнего `inc h`
        | if xor_mode { 0 } else { cf_in };
    bump_r(cpu, (7 * n + 1) as u32); // 7 инструкций на проход + `ret`
    48 * n + 5 // 35 на проход + 13 (`djnz` взят) на все, кроме последнего, + 8 + 10
}

/// Число строк копирования в `scr_copy_alt_lines` (`ld b,3Ch`).
const COPY_ALT_LINES: u8 = 0x3C;
/// Длина строки в байтах (`ld bc,0020h` перед `ldir`).
const COPY_ALT_LINE: u16 = 0x20;
/// Верхняя оценка тактов одной итерации цикла `scr_copy_alt_lines`. Складывается
/// из push bc/hl/de (33), ld bc (10), ldir (37·31+32 = 1179), pop de/hl/bc (30),
/// двух юнитов call 7FE3/ex de,hl/call 7FE7 (2·38 = 76), четырёх сдвигов адреса
/// (4·87 = 348, худший случай) и djnz (13) — итого 1689. Хук гоняет итерацию
/// быстро, только если она целиком укладывается в остаток кадра; иначе отдаёт
/// управление Z80, чтобы прерывание сработало ровно там же, где без хука.
const COPY_ALT_MAX_ITER: u64 = 1689;

/// `scr_copy_alt_lines` (`0x7FCD`, внутренний цикл `CopyBuf2Screen`): копирует
/// `3Ch` строк по `20h` байтов из `HL` в `DE`, после каждой строки сдвигая оба
/// указателя на **две** пиксельные строки — то есть заполняет строки «через
/// одну». Записи в адреса ниже `ROM_SIZE` отбрасываются — так же поступает
/// шина настоящего Spectrum.
///
/// ```text
///         ld b,3Ch
/// loop:   push bc ; push hl ; push de
///         ld bc,20h
///         ldir
///         pop de ; pop hl ; pop bc
///         call unit ; call unit
///         djnz loop
///         ret
/// unit:   ex de,hl ; call 7FE7h
/// 7FE7h:  inc h ; ld a,h ; and 7 ; call z, compDownScrAdr_HL ; ret
/// ```
///
/// `call 7FE7h` кладёт в стек адрес, указывающий на собственное тело: после
/// первого `ret` тело выполняется повторно, поэтому каждый `unit` двигает
/// указатель на две строки, а не на одну. `CopyBuf2Screen` (`0x7FBE`) вызывает
/// рутину дважды подряд (0xC000→0x4020 и 0xC100→0x4120): проходы дополняют
/// друг друга — первый пишет строки 8, 10, …, второй 9, 11, … и вместе
/// покрывают экран начиная со строки 8.
///
/// `push`/`pop`/`ld bc` на регистрах в сумме следа не оставляют (кроме `B`,
/// который `djnz` обнуляет, и `F` от последнего `ldir`, перекрываемого телом
/// `scr_adr_next_line`), поэтому хук копирует прямо по текущим `HL`/`DE`, а
/// сдвиг указателей выполняют те же точные примитивы, что и настоящий код.
///
/// Резиду в СТЕКЕ, напротив, обязателен: в игре стек живёт прямо в экранной
/// памяти (SP=4116h..411Eh — строки 1–7, которые копирование не трогает),
/// и шесть байт под SP с регистрами и адресами возврата видны на экране.
/// Слоты: `push bc/hl/de` пишут в S-2/S-4/S-6; `call 7FE3h` перезаписывает
/// S-2 адресом возврата (7FDDh/7FE0h), `call 7FE7h` — S-4 (7FE7h). Тело
/// под `call 7FE7h` работает с SP=S-4 и пишет резиду `call z` (7FEEh) в
/// слот S-6, а его повторный прогон после `ret` идёт с SP=S-2 и
/// перезаписывает слот S-4. Хук пишет резиду в том же хронологическом
/// порядке, что и настоящие `push`/`call`, включая перезапись ldir-копией.
///
/// Прямые заходы в середину рутины (профиль показал единичные попадания на
/// `0x7FCF`/`0x7FD5` минуя вход) хук не перехватывает — код рутины в памяти
/// сохранён и выполняется как есть.
fn scr_copy_alt_lines_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let mut cycles = 7u64; // ld b,3Ch (в этом эмуляторе ld r,n = 7)
    bump_r(cpu, 1);
    cpu.b = COPY_ALT_LINES;
    let sp = cpu.sp;
    // Остаток кадра считаем от такта на входе; `frame_target` = u64::MAX вне
    // кадрового цикла (тесты) — тогда хук отрабатывает рутину целиком.
    let budget_end = m.frame_target.saturating_sub(cpu.cycles);
    loop {
        // Итерация длиннее остатка кадра — отдаём управление в точку цикла
        // 0x7FCF с состоянием на начало итерации. Дальше Z80 доиграет рутину
        // как обычно, и vblank-прерывание попадёт ровно в тот же такт, что и
        // без хука (иначе хук «проскочил» бы прерывание и разошёлся с plain).
        if cycles + COPY_ALT_MAX_ITER > budget_end {
            cpu.pc = 0x7FCF;
            m.hook_yield = true;
            return cycles;
        }
        // push bc ; push hl ; push de
        bump_r(cpu, 3);
        cycles += 33;
        let hl = cpu.hl();
        let de = cpu.de();
        mem_write(m, sp.wrapping_sub(2), cpu.c);
        mem_write(m, sp.wrapping_sub(1), cpu.b);
        mem_write(m, sp.wrapping_sub(4), cpu.l);
        mem_write(m, sp.wrapping_sub(3), cpu.h);
        mem_write(m, sp.wrapping_sub(6), cpu.e);
        mem_write(m, sp.wrapping_sub(5), cpu.d);
        // ld bc,20h + ldir. У эмулятора каждый байт ldir — отдельный step:
        // повторяющиеся байты стоят block_ldi(16)+21, последний 16+16.
        let mut last = 0u8;
        for i in 0..COPY_ALT_LINE {
            let s = hl.wrapping_add(i) as usize;
            let d = de.wrapping_add(i) as usize;
            last = m.mem[s];
            if d >= ROM_SIZE {
                m.mem[d] = last;
            }
        }
        // Флаги block_ldi: S,Z,C сохраняются от входа, X/Y от A+последнего
        // перенесённого байта, PV=0 (BC кончился), N=H=0.
        let n = cpu.a.wrapping_add(last);
        cpu.f = (cpu.f & (SF | ZF | CF)) | (n & XF) | if n & 0x02 != 0 { YF } else { 0 };
        bump_r(cpu, 1 + COPY_ALT_LINE as u32);
        cycles += 10 + 37 * (COPY_ALT_LINE as u64 - 1) + 32;
        // pop de ; pop hl ; pop bc. Регистры восстанавливаются ИЗ ПАМЯТИ:
        // когда копия перекрывает кадр стека (в игре SP=411Eh, и строка 1
        // экрана лежит в 4100h..411Fh), `pop` возвращает уже испорченные
        // `ldir` значения — ровно так, как это делает настоящий Z80.
        bump_r(cpu, 3);
        cycles += 30;
        cpu.e = m.mem[sp.wrapping_sub(6) as usize];
        cpu.d = m.mem[sp.wrapping_sub(5) as usize];
        cpu.l = m.mem[sp.wrapping_sub(4) as usize];
        cpu.h = m.mem[sp.wrapping_sub(3) as usize];
        cpu.c = m.mem[sp.wrapping_sub(2) as usize];
        cpu.b = m.mem[sp.wrapping_sub(1) as usize];
        for unit in 0..2u16 {
            std::mem::swap(&mut cpu.h, &mut cpu.d);
            std::mem::swap(&mut cpu.l, &mut cpu.e);
            bump_r(cpu, 3); // call unit + ex de,hl + call 7FE7h
            cycles += 17 + 4 + 17;
            // call 7FE3h кладёт в стек адрес возврата (7FDDh или 7FE0h),
            // затем call 7FE7h кладёт 7FE7h — по два байта на слот.
            let ret = if unit == 0 { 0x7FDDu16 } else { 0x7FE0 };
            mem_write(m, sp.wrapping_sub(2), ret as u8);
            mem_write(m, sp.wrapping_sub(1), (ret >> 8) as u8);
            mem_write(m, sp.wrapping_sub(4), 0xE7);
            mem_write(m, sp.wrapping_sub(3), 0x7F);
            // Первое тело работает под `call 7FE7h` (SP=S-4) и пишет резиду
            // `call z` в слот S-6; повторный прогон после `ret` идёт с
            // SP=S-2 и перезаписывает слот S-4 — адрес возврата 7FE7h.
            for slot in [6u16, 4] {
                // call z,8445h берётся, когда (H+1) mod 8 == 0 — так же
                // решает внутренний хук scr_adr_next_line.
                if cpu.h.wrapping_add(1) & 7 == 0 {
                    mem_write(m, sp.wrapping_sub(slot), 0xEE);
                    mem_write(m, sp.wrapping_sub(slot - 1), 0x7F);
                }
                cycles += scr_adr_next_line_advance(cpu, m);
            }
        }
        // djnz loop
        bump_r(cpu, 1);
        cpu.b = cpu.b.wrapping_sub(1);
        if cpu.b == 0 {
            cycles += 8;
            break;
        }
        cycles += 13;
    }
    bump_r(cpu, 1); // ret
    cycles + 10
}

/// Пишет байт так же, как шина `Spectrum::write`: ниже `ROM_SIZE` запись
/// отбрасывается. Хуки обязаны пользоваться этой функцией (а не `m.mem`),
/// чтобы ПЗУ оставалось неизменным, как на настоящей шине.
fn mem_write(m: &mut Spectrum, addr: u16, val: u8) {
    if addr as usize >= ROM_SIZE {
        m.mem[addr as usize] = val;
        if let Some((lo, hi)) = m.wtrace {
            if addr >= lo && addr <= hi {
                eprintln!("H 0x{addr:04X} <- 0x{val:02X}");
            }
        }
    }
}

/// `byte_9522` — состояние звукового автомата: 1/2/3 выбирают обработчик
/// активного эффекта, 0 — «свободен» (запуск нового по `nBeepFX`).
const SFX_STATE: u16 = 0x9522;
/// `word_9523` — счётчик `DE` активного эффекта (шаг/остаток).
const SFX_DE: u16 = 0x9523;
/// `word_9525` — указатель `HL` на данные активного эффекта.
const SFX_HL: u16 = 0x9525;
/// `byte_96BF` — номер канала, декодированный из `nBeepFX`.
const SFX_CHAN: u16 = 0x96BF;
/// `nBeepFX` — битовая маска запрошенных звуковых эффектов.
const SFX_REQUEST: u16 = 0x7128;
/// `FlagsSetUp` — бит 7 разрешает звук.
const SFX_FLAGS: u16 = 0xFC0E;
/// Адрес инструкции, которую игра пишет себе сама (`ld (95F2h),hl`):
/// `3FCBh` восстанавливает `srl a` (`CB 3F`), `10C6h` даёт `add a,10h`.
const SFX_PATCH: u16 = 0x95F2;

/// Верхняя оценка тактов, которые хук тратит до следующей проверки остатка
/// кадра: самая дорогая инструкция рутины — `ld (nn),de` (20 тактов).
const SFX_MAX_STEP: u64 = 20;

/// Читает 16-битное слово (little-endian) так же, как это делает Z80.
fn mem_read16(m: &Spectrum, addr: u16) -> u16 {
    m.mem[addr as usize] as u16 | ((m.mem[addr.wrapping_add(1) as usize] as u16) << 8)
}

/// Пишет 16-битное слово (little-endian) через шину (`mem_write`).
fn mem_write16(m: &mut Spectrum, addr: u16, val: u16) {
    mem_write(m, addr, val as u8);
    mem_write(m, addr.wrapping_add(1), (val >> 8) as u8);
}

/// `sub_9579` — покадровый автомат звуковых эффектов. Вызывается раз в кадр из
/// обработчика прерывания (`sub_7125` — это `jp 9579h`).
///
/// На входе состояние в `byte_9522`:
/// - `0` — эффекта нет: если `nBeepFX == 0`, выход; иначе по младшему
///   установленному биту `nBeepFX` выбирается обработчик из таблицы `jp`-ов
///   `0x9561` и запускает соответствующий эффект;
/// - `1` — эффект `0x9624` (пакет импульсов);
/// - `2` — эффект `0x95E7` (шум по данным `HL`);
/// - `3` — эффект `0x95A6` (тон с огибающей), продолжение.
///
/// Один вызов продвигает эффект на один «чанк» и возвращается по `ret nz`
/// (`0x95C7`, `0x960F`) либо `ret` (`0x95E6`, `0x9623`), когда `DE`/состояние
/// исчерпаны, поэтому вызов не обязан укладываться в кадр.
///
/// Рутина самомодифицирующаяся: она пишет себе операнды `ld c,n` (`0x95AE`),
/// `add a,n` (`0x95BD`, `0x962B`) и целую инструкцию по `0x95F2`, поэтому все
/// операнды хук читает из памяти. Обработчики таблицы (`0x9641`–`0x9699`) —
/// общий код: им пользуются и `sub_9579`, и стартовые рутины эффектов
/// (`sub_969B`/`sub_96A9`/`sub_96B7`), которые пока остаются на Z80; поэтому
/// хук обязан записывать операнды ровно так, как это делал бы Z80.
///
/// Кадровая граница: перед каждой инструкцией хук проверяет, влезает ли она в
/// остаток кадра, и при нехватке отдаёт управление Z80 ровно на этой
/// инструкции (`hook_yield`), оставив регистры и память как на Z80. Так
/// длинный чанк (до сотен тысяч тактов, больше кадра) переживает
/// vblank-прерывание ровно там же, где прогон без хука. Запись в порт `0xFE`
/// идёт через шину с тактом начала инструкции (`cycle_clock`), поэтому бипер
/// звучит идентично.
fn sfx_update_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    // Остаток кадра считаем от такта на входе; `frame_target` = u64::MAX вне
    // кадрового цикла (тесты) — тогда хук отрабатывает рутину целиком.
    let budget = m.frame_target.saturating_sub(cpu.cycles);
    let mut c = 0u64;
    // Выполнить одну инструкцию заменяемого кода. Не влезает в остаток
    // кадра — отдать её Z80, не меняя состояния.
    macro_rules! z80 {
        ($addr:expr, $cost:expr, $body:block) => {{
            if c + $cost > budget {
                cpu.pc = $addr;
                m.hook_yield = true;
                return c;
            }
            bump_r(cpu, 1);
            $body
            c += $cost;
        }};
    }
    // `out (0FEh),a`: бипер/бордюр; такт — начало инструкции, как это видит
    // `Spectrum::run_frame` (`cycle_clock = cpu.cycles` перед `step`).
    macro_rules! out_fe {
        () => {{
            m.set_cycle_clock(cpu.cycles + c);
            Bus::output(m, ((cpu.a as u16) << 8) | 0xFE, cpu.a);
        }};
    }
    macro_rules! djnz {
        ($addr:expr, $target:expr, $next:expr, $pc:ident) => {{
            let b = cpu.b.wrapping_sub(1);
            let taken = b != 0;
            z80!($addr, if taken { 13 } else { 8 }, { cpu.b = b; });
            $pc = if taken { $target } else { $next };
        }};
    }
    let mut pc = 0x9579u16;
    loop {
        match pc {
            // `ld a,(FlagsSetUp) ; bit 7,a ; ret z`
            0x9579 => {
                z80!(0x9579, 13, { cpu.a = m.mem[SFX_FLAGS as usize]; });
                pc = 0x957C;
            }
            0x957C => {
                z80!(0x957C, 8, { cpu.bit(7, cpu.a); });
                pc = 0x957E;
            }
            0x957E => {
                let taken = cpu.f & ZF != 0;
                z80!(0x957E, if taken { 11 } else { 5 }, {});
                if taken {
                    return c; // RET выполнит диспетчер хуков
                }
                pc = 0x957F;
            }
            // `ld a,(byte_9522) ; and a ; jr nz,959Fh`
            0x957F => {
                z80!(0x957F, 13, { cpu.a = m.mem[SFX_STATE as usize]; });
                pc = 0x9582;
            }
            0x9582 => {
                z80!(0x9582, 4, { cpu.a = cpu.and8(cpu.a, cpu.a); });
                pc = 0x9583;
            }
            0x9583 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x9583, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x959F } else { 0x9585 };
            }
            // Запуск нового эффекта: `ld a,(nBeepFX) ; and a ; ret z`; далее
            // B = номер младшего установленного бита, переход по таблице.
            0x9585 => {
                z80!(0x9585, 13, { cpu.a = m.mem[SFX_REQUEST as usize]; });
                pc = 0x9588;
            }
            0x9588 => {
                z80!(0x9588, 4, { cpu.a = cpu.and8(cpu.a, cpu.a); });
                pc = 0x9589;
            }
            0x9589 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x9589, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x958A;
            }
            0x958A => {
                z80!(0x958A, 7, { cpu.b = 0xFF; });
                pc = 0x958C;
            }
            0x958C => {
                z80!(0x958C, 4, { cpu.b = cpu.inc8(cpu.b); });
                pc = 0x958D;
            }
            0x958D => {
                z80!(0x958D, 4, { cpu.rra(); });
                pc = 0x958E;
            }
            0x958E => {
                let taken = cpu.f & CF == 0;
                z80!(0x958E, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x958C } else { 0x9590 };
            }
            0x9590 => {
                z80!(0x9590, 4, { cpu.a = cpu.b; });
                pc = 0x9591;
            }
            0x9591 => {
                z80!(0x9591, 13, { mem_write(m, SFX_CHAN, cpu.a); });
                pc = 0x9594;
            }
            0x9594 => {
                z80!(0x9594, 4, { cpu.a = cpu.add8(cpu.a, cpu.a, 0); });
                pc = 0x9595;
            }
            0x9595 => {
                z80!(0x9595, 4, { cpu.a = cpu.add8(cpu.a, cpu.b, 0); });
                pc = 0x9596;
            }
            0x9596 => {
                z80!(0x9596, 10, { cpu.h = 0x95; cpu.l = 0x61; });
                pc = 0x9599;
            }
            0x9599 => {
                z80!(0x9599, 4, { cpu.a = cpu.add8(cpu.a, cpu.l, 0); });
                pc = 0x959A;
            }
            0x959A => {
                z80!(0x959A, 4, { cpu.l = cpu.a; });
                pc = 0x959B;
            }
            0x959B => {
                let taken = cpu.f & CF == 0;
                z80!(0x959B, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x959E } else { 0x959D };
            }
            0x959D => {
                z80!(0x959D, 4, { cpu.h = cpu.inc8(cpu.h); });
                pc = 0x959E;
            }
            0x959E => {
                z80!(0x959E, 4, {});
                pc = cpu.hl(); // jp (hl)
            }
            // Таблица `jp nn` (0x9561..0x9577): цель читаем из образа.
            _ if (0x9561..=0x9577).contains(&pc) => {
                let target = mem_read16(m, pc + 1);
                z80!(pc, 10, {});
                pc = target;
            }
            // `dec a ; jp z,9624h ; dec a ; jr z,95E7h` — выбор по byte_9522.
            0x959F => {
                z80!(0x959F, 4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0x95A0;
            }
            0x95A0 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95A0, 10, {});
                pc = if taken { 0x9624 } else { 0x95A3 };
            }
            0x95A3 => {
                z80!(0x95A3, 4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0x95A4;
            }
            0x95A4 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95A4, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95E7 } else { 0x95A6 };
            }
            // Хвост эффекта 3 (тон с огибающей), 0x95A6..0x95C7.
            0x95A6 => {
                z80!(0x95A6, 16, {
                    let v = mem_read16(m, SFX_HL);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95AA;
            }
            0x95AA => {
                z80!(0x95AA, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x95AD;
            }
            0x95AD => {
                z80!(0x95AD, 7, { cpu.c = m.mem[0x95AE]; });
                pc = 0x95AF;
            }
            0x95AF => {
                z80!(0x95AF, 7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0x95B0;
            }
            0x95B0 => {
                z80!(0x95B0, 7, { cpu.a = cpu.and8(cpu.a, 0x18); });
                pc = 0x95B2;
            }
            0x95B2 => {
                z80!(0x95B2, 11, { out_fe!(); });
                pc = 0x95B4;
            }
            0x95B4 => {
                z80!(0x95B4, 4, { cpu.b = cpu.d; });
                pc = 0x95B5;
            }
            0x95B5 => djnz!(0x95B5, 0x95B5, 0x95B7, pc),
            0x95B7 => {
                z80!(0x95B7, 6, {
                    let v = cpu.hl().wrapping_add(1);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95B8;
            }
            0x95B8 => {
                z80!(0x95B8, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x95B9;
            }
            0x95B9 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x95B9, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95AF } else { 0x95BB };
            }
            0x95BB => {
                z80!(0x95BB, 4, { cpu.a = cpu.d; });
                pc = 0x95BC;
            }
            0x95BC => {
                let imm = m.mem[0x95BD];
                z80!(0x95BC, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                pc = 0x95BE;
            }
            0x95BE => {
                z80!(0x95BE, 4, { cpu.d = cpu.a; });
                pc = 0x95BF;
            }
            0x95BF => {
                z80!(0x95BF, 4, { cpu.e = cpu.dec8(cpu.e); });
                pc = 0x95C0;
            }
            0x95C0 => {
                z80!(0x95C0, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x95C4;
            }
            0x95C4 => {
                z80!(0x95C4, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x95C7;
            }
            0x95C7 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x95C7, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x95C8;
            }
            0x95C8 => {
                z80!(0x95C8, 13, { cpu.a = m.mem[SFX_STATE as usize]; });
                pc = 0x95CB;
            }
            0x95CB => {
                z80!(0x95CB, 8, { cpu.bit(7, cpu.a); });
                pc = 0x95CD;
            }
            0x95CD => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95CD, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x9610 } else { 0x95CF };
            }
            // Перезапуск эффекта 3 (byte_9522 = 3, C = 0A0h).
            0x95CF => {
                z80!(0x95CF, 7, { cpu.a = 3; });
                pc = 0x95D1;
            }
            0x95D1 => {
                z80!(0x95D1, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x95D4;
            }
            0x95D4 => {
                z80!(0x95D4, 7, { cpu.a = 0xA0; });
                pc = 0x95D6;
            }
            0x95D6 => {
                z80!(0x95D6, 13, { mem_write(m, 0x95AE, cpu.a); });
                pc = 0x95D9;
            }
            0x95D9 => {
                z80!(0x95D9, 10, { cpu.h = 0x01; cpu.l = 0x27; });
                pc = 0x95DC;
            }
            0x95DC => {
                z80!(0x95DC, 10, { cpu.d = 0x40; cpu.e = 0x02; });
                pc = 0x95DF;
            }
            0x95DF => {
                z80!(0x95DF, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x95E3;
            }
            0x95E3 => {
                z80!(0x95E3, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x95E6;
            }
            0x95E6 => {
                z80!(0x95E6, 10, {});
                return c;
            }
            // Эффект 2 (шум по данным HL), 0x95E7..0x9607.
            0x95E7 => {
                z80!(0x95E7, 16, {
                    let v = mem_read16(m, SFX_HL);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95EA;
            }
            0x95EA => {
                z80!(0x95EA, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x95EE;
            }
            0x95EE => {
                z80!(0x95EE, 7, { cpu.c = 0x64; });
                pc = 0x95F0;
            }
            0x95F0 => {
                z80!(0x95F0, 7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0x95F1;
            }
            0x95F1 => {
                z80!(0x95F1, 4, { cpu.l = cpu.inc8(cpu.l); });
                pc = 0x95F2;
            }
            0x95F2 => {
                // Инструкцию по этому адресу игра пишет себе сама
                // (`ld (95F2h),hl`): 3FCBh -> `srl a`, 10C6h -> `add a,10h`.
                match (m.mem[SFX_PATCH as usize], m.mem[SFX_PATCH as usize + 1]) {
                    (0xCB, 0x3F) => {
                        z80!(0x95F2, 8, { cpu.a = cpu.srl(cpu.a); });
                        pc = 0x95F4;
                    }
                    (0xC6, imm) => {
                        z80!(0x95F2, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                        pc = 0x95F4;
                    }
                    // В игре такого не бывает; отдаём инструкцию Z80, чтобы
                    // поведение осталось верным при любом содержимом.
                    _ => {
                        cpu.pc = 0x95F2;
                        m.hook_yield = true;
                        return c;
                    }
                }
            }
            0x95F4 => {
                z80!(0x95F4, 8, { cpu.a = cpu.srl(cpu.a); });
                pc = 0x95F6;
            }
            0x95F6 => {
                z80!(0x95F6, 4, { cpu.e = cpu.a; });
                pc = 0x95F7;
            }
            0x95F7 => {
                z80!(0x95F7, 4, { cpu.b = cpu.a; });
                pc = 0x95F8;
            }
            0x95F8 => {
                z80!(0x95F8, 7, { cpu.a = 0x30; });
                pc = 0x95FA;
            }
            0x95FA => {
                z80!(0x95FA, 11, { out_fe!(); });
                pc = 0x95FC;
            }
            0x95FC => djnz!(0x95FC, 0x95FC, 0x95FE, pc),
            0x95FE => {
                z80!(0x95FE, 4, { cpu.b = cpu.e; });
                pc = 0x95FF;
            }
            0x95FF => {
                z80!(0x95FF, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9600;
            }
            0x9600 => {
                z80!(0x9600, 11, { out_fe!(); });
                pc = 0x9602;
            }
            0x9602 => djnz!(0x9602, 0x9602, 0x9604, pc),
            0x9604 => {
                z80!(0x9604, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x9605;
            }
            0x9605 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x9605, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95F0 } else { 0x9607 };
            }
            // Эффект 1 (пакет импульсов), 0x9624..0x963F.
            0x9624 => {
                z80!(0x9624, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x9628;
            }
            0x9628 => {
                z80!(0x9628, 4, { cpu.a = cpu.e; });
                pc = 0x9629;
            }
            0x9629 => {
                z80!(0x9629, 4, { cpu.a = cpu.add8(cpu.a, cpu.a, 0); });
                pc = 0x962A;
            }
            0x962A => {
                let imm = m.mem[0x962B];
                z80!(0x962A, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                pc = 0x962C;
            }
            0x962C => {
                z80!(0x962C, 4, { cpu.e = cpu.a; });
                pc = 0x962D;
            }
            0x962D => {
                z80!(0x962D, 7, { cpu.c = 0x0F; });
                pc = 0x962F;
            }
            0x962F => {
                z80!(0x962F, 7, { cpu.a = 0x30; });
                pc = 0x9631;
            }
            0x9631 => {
                z80!(0x9631, 4, { cpu.b = cpu.e; });
                pc = 0x9632;
            }
            0x9632 => {
                z80!(0x9632, 11, { out_fe!(); });
                pc = 0x9634;
            }
            0x9634 => djnz!(0x9634, 0x9634, 0x9636, pc),
            0x9636 => {
                z80!(0x9636, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9637;
            }
            0x9637 => {
                z80!(0x9637, 4, { cpu.b = cpu.e; });
                pc = 0x9638;
            }
            0x9638 => {
                z80!(0x9638, 11, { out_fe!(); });
                pc = 0x963A;
            }
            0x963A => djnz!(0x963A, 0x963A, 0x963C, pc),
            0x963C => {
                z80!(0x963C, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x963D;
            }
            0x963D => {
                let taken = cpu.f & ZF == 0;
                z80!(0x963D, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x962F } else { 0x963F };
            }
            0x963F => {
                z80!(0x963F, 12, {});
                pc = 0x9607;
            }
            // Общий эпилог: `dec d ; ... ; ret nz` и сброс состояния.
            0x9607 => {
                z80!(0x9607, 4, { cpu.d = cpu.dec8(cpu.d); });
                pc = 0x9608;
            }
            0x9608 => {
                z80!(0x9608, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x960C;
            }
            0x960C => {
                z80!(0x960C, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x960F;
            }
            0x960F => {
                let taken = cpu.f & ZF == 0;
                z80!(0x960F, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x9610;
            }
            0x9610 => {
                z80!(0x9610, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9611;
            }
            0x9611 => {
                z80!(0x9611, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9614;
            }
            0x9614 => {
                z80!(0x9614, 13, { cpu.a = m.mem[SFX_CHAN as usize]; });
                pc = 0x9617;
            }
            0x9617 => {
                z80!(0x9617, 4, { cpu.a = cpu.inc8(cpu.a); });
                pc = 0x9618;
            }
            0x9618 => {
                z80!(0x9618, 4, { cpu.b = cpu.a; });
                pc = 0x9619;
            }
            0x9619 => {
                z80!(0x9619, 7, { cpu.a = 0x80; });
                pc = 0x961B;
            }
            0x961B => {
                z80!(0x961B, 4, { cpu.rlca(); });
                pc = 0x961C;
            }
            0x961C => djnz!(0x961C, 0x961B, 0x961E, pc),
            0x961E => {
                z80!(0x961E, 10, { cpu.h = 0x71; cpu.l = 0x28; });
                pc = 0x9621;
            }
            0x9621 => {
                z80!(0x9621, 7, { cpu.a = cpu.xor8(cpu.a, m.mem[cpu.hl() as usize]); });
                pc = 0x9622;
            }
            0x9622 => {
                let addr = cpu.hl();
                z80!(0x9622, 7, { mem_write(m, addr, cpu.a); });
                pc = 0x9623;
            }
            0x9623 => {
                z80!(0x9623, 10, {});
                return c;
            }
            // Обработчики таблицы запуска эффектов.
            0x9678 => {
                z80!(0x9678, 7, { cpu.c = 0xFE; });
                pc = 0x967A;
            }
            0x967A => {
                z80!(0x967A, 7, { cpu.a = 3; });
                pc = 0x967C;
            }
            0x967C => {
                z80!(0x967C, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x967F;
            }
            0x967F => {
                z80!(0x967F, 7, { cpu.b = 0x40; });
                pc = 0x9681;
            }
            0x9681 => {
                z80!(0x9681, 10, { cpu.d = 0x40; cpu.e = 0x08; });
                pc = 0x9684;
            }
            0x9684 => {
                z80!(0x9684, 10, { cpu.h = 0x01; cpu.l = 0x27; });
                pc = 0x9687;
            }
            0x9687 => {
                z80!(0x9687, 4, { cpu.a = cpu.c; });
                pc = 0x9688;
            }
            0x9688 => {
                z80!(0x9688, 13, { mem_write(m, 0x95BD, cpu.a); });
                pc = 0x968B;
            }
            0x968B => {
                z80!(0x968B, 4, { cpu.a = cpu.b; });
                pc = 0x968C;
            }
            0x968C => {
                z80!(0x968C, 13, { mem_write(m, 0x95AE, cpu.a); });
                pc = 0x968F;
            }
            0x968F => {
                z80!(0x968F, 10, {});
                pc = 0x95AD;
            }
            0x9674 => {
                z80!(0x9674, 7, { cpu.c = 0xE4; });
                pc = 0x9676;
            }
            0x9676 => {
                z80!(0x9676, 12, {});
                pc = 0x967A;
            }
            0x9692 => {
                z80!(0x9692, 7, { cpu.a = 0x83; });
                pc = 0x9694;
            }
            0x9694 => {
                z80!(0x9694, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9697;
            }
            0x9697 => {
                z80!(0x9697, 7, { cpu.c = 2; });
                pc = 0x9699;
            }
            0x9699 => {
                z80!(0x9699, 12, {});
                pc = 0x967F;
            }
            0x965E => {
                z80!(0x965E, 7, { cpu.a = 3; });
                pc = 0x9660;
            }
            0x9660 => {
                z80!(0x9660, 10, { cpu.h = 0x3F; cpu.l = 0xCB; });
                pc = 0x9663;
            }
            0x9657 => {
                z80!(0x9657, 7, { cpu.a = 0x1A; });
                pc = 0x9659;
            }
            0x9659 => {
                z80!(0x9659, 10, { cpu.h = 0x10; cpu.l = 0xC6; });
                pc = 0x965C;
            }
            0x965C => {
                z80!(0x965C, 12, {});
                pc = 0x9663;
            }
            0x9663 => {
                z80!(0x9663, 10, { cpu.d = 0x01; cpu.e = 0x00; });
                pc = 0x9666;
            }
            0x9666 => {
                z80!(0x9666, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_PATCH, hl);
                });
                pc = 0x9669;
            }
            0x9669 => {
                z80!(0x9669, 4, { cpu.h = cpu.a; });
                pc = 0x966A;
            }
            0x966A => {
                z80!(0x966A, 7, { cpu.l = 0x1A; });
                pc = 0x966C;
            }
            0x966C => {
                z80!(0x966C, 7, { cpu.a = 2; });
                pc = 0x966E;
            }
            0x966E => {
                z80!(0x966E, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9671;
            }
            0x9671 => {
                z80!(0x9671, 10, {});
                pc = 0x95EE;
            }
            0x9641 => {
                z80!(0x9641, 7, { cpu.a = 0xFD; });
                pc = 0x9643;
            }
            0x9643 => {
                z80!(0x9643, 10, { cpu.d = 0x02; cpu.e = 0x00; });
                pc = 0x9646;
            }
            0x9646 => {
                z80!(0x9646, 12, {});
                pc = 0x964D;
            }
            0x9648 => {
                z80!(0x9648, 7, { cpu.a = 0xD3; });
                pc = 0x964A;
            }
            0x964A => {
                z80!(0x964A, 10, { cpu.d = 0x08; cpu.e = 0x00; });
                pc = 0x964D;
            }
            0x964D => {
                z80!(0x964D, 13, { mem_write(m, 0x962B, cpu.a); });
                pc = 0x9650;
            }
            0x9650 => {
                z80!(0x9650, 7, { cpu.a = 1; });
                pc = 0x9652;
            }
            0x9652 => {
                z80!(0x9652, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9655;
            }
            0x9655 => {
                z80!(0x9655, 12, {});
                pc = 0x9628;
            }
            _ => unreachable!("sfx_update: неизвестный адрес 0x{pc:04X}"),
        }
    }
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
        run_routine_sp(m, addr, 0xF000u16.wrapping_sub(2));
    }

    /// То же, но со своим `SP`: стековый маркер ложится по нему.
    fn run_routine_sp(m: &mut Spectrum, addr: u16, sp: u16) {
        const RET: u16 = 0x1234;
        m.cpu.sp = sp;
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
        r: u8,
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
            r: c.r,
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

    /// Прогон рутины на Z80 со своим `SP`.
    fn z80_sub_sp(m: &mut Spectrum, addr: u16, sp: u16) -> Snap {
        m.cpu.cycles = 0;
        run_routine_sp(m, addr, sp);
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

    /// `sub_7A59` обязан совпасть с Z80 на всех 65536 парах (старое значение
    /// байта, прибавляемое `A`): регистры, флаги, такты и записанный байт.
    #[test]
    fn smag_add_mem_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        let addr = 0x7100u16;
        for old in 0u8..=255 {
            for a in 0u8..=255 {
                z.mem[addr as usize] = old;
                n.mem[addr as usize] = old;
                set(&mut z.cpu, a, addr, 0x5678, a ^ old);
                set(&mut n.cpu, a, addr, 0x5678, a ^ old);
                z.cpu.b = 0x11;
                z.cpu.c = 0x22;
                n.cpu.b = 0x11;
                n.cpu.c = 0x22;
                let zs = z80_sub(&mut z, 0x7A59);
                let ns = hook_sub(&mut n, 0x7A59);
                assert_eq!(zs, ns, "sub_7A59 old=0x{old:02X} a=0x{a:02X}");
                assert_eq!(
                    z.mem[addr as usize],
                    n.mem[addr as usize],
                    "sub_7A59: память по HL разошлась (old=0x{old:02X} a=0x{a:02X})"
                );
            }
        }
    }

    /// `sub_D546` обязан совпасть с Z80 по всем проходам: все `B` (включая
    /// 0 = 256 проходов, как у `djnz`), оба режима самоизменяющегося кода
    /// (`0xD548`: `A9h` = XOR, `00h` = копирование), краевые адреса
    /// (переносы `H` и `DE`), обе установки `C`-флага на входе. Сверяются
    /// регистры, флаги, такты, `R` и вся память (кроме 2 байт стекового
    /// маркера `run_routine` в 0xEFFE..0xEFFF, которых хук не пишет).
    ///
    /// Значения `L` в парах не задевают маркер стека (L≠0FEh,0FFh) и код
    /// рутины (0xD546..0xD54E, L≠46h..4Eh) — иначе настоящий Z80 перезаписал
    /// бы собственный стек или опкоды посреди цикла.
    #[test]
    fn scr_xor_col_down_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        // (HL, DE): обычные + краевые.
        let pairs: &[(u16, u16)] = &[
            (0x7000, 0x8000),
            (0x707E, 0x8100),
            (0x7FF7, 0x8FFF), // `inc h` со стыка страниц
            (0x7123, 0x9ABC),
            (0x7800, 0xFFFF), // перенос `DE` в 0000h
            (0x77F0, 0x0000),
            (0x7F80, 0x8080),
            (0x7ABD, 0x8DEF),
            (0x7600, 0x86FF),
            (0x7456, 0xF0F0),
            (0x7E00, 0x7EFF), // источник пересекается с колонкой назначения
            (0x7900, 0x8901),
        ];
        for &xor_mode in [true, false].iter() {
            let mode_byte = if xor_mode { XOR_COL_XOR_OP } else { 0x00 };
            for &(hl, de) in pairs {
                for b in 0u8..=255 {
                    let seed = (b as usize).wrapping_mul(0x9E37)
                        ^ (hl as usize)
                        ^ ((de as usize) << 1)
                        ^ ((mode_byte as usize) << 8);
                    // Колонка назначения: все адреса с данным младшим байтом
                    // (`inc h` меняет только старший), и 256 байт источника.
                    let lo = (hl & 0xFF) as usize;
                    for i in 0..256usize {
                        let v = (i.wrapping_mul(31).wrapping_add(seed) & 0xFF) as u8;
                        z.mem[(i << 8) | lo] = v;
                        n.mem[(i << 8) | lo] = v;
                        let s = (de as usize).wrapping_add(i) & 0xFFFF;
                        z.mem[s] = v ^ 0x5A;
                        n.mem[s] = v ^ 0x5A;
                    }
                    z.mem[XOR_COL_OP] = mode_byte;
                    n.mem[XOR_COL_OP] = mode_byte;
                    // 0xFF — C=1, 0xFE — C=0: в режиме копирования `C`-флаг
                    // рутина не трогает и обязан сохранить входной.
                    let f_in = if b & 1 == 0 { 0xFF } else { 0xFE };
                    set(&mut z.cpu, 0x11, hl, de, f_in);
                    set(&mut n.cpu, 0x11, hl, de, f_in);
                    z.cpu.b = b;
                    z.cpu.c = 0x22;
                    n.cpu.b = b;
                    n.cpu.c = 0x22;
                    let zs = z80_sub(&mut z, 0xD546);
                    let ns = hook_sub(&mut n, 0xD546);
                    assert_eq!(
                        zs, ns,
                        "sub_D546 hl=0x{hl:04X} de=0x{de:04X} b=0x{b:02X} xor={xor_mode}"
                    );
                    // Вся память, кроме 2 байт маркера стека (0xEFFE..0xEFFF):
                    // хук их не пишет, их пишет только run_routine у z.
                    let diff = if z.mem[0..0xEFFE] != n.mem[0..0xEFFE] {
                        (0..0xEFFE).find(|&i| z.mem[i] != n.mem[i])
                    } else if z.mem[0xF000..] != n.mem[0xF000..] {
                        (0xF000..0x1_0000).find(|&i| z.mem[i] != n.mem[i])
                    } else {
                        None
                    };
                    if let Some(i) = diff {
                        panic!(
                            "sub_D546: память разошлась по адресу 0x{i:04X} \
                             (z=0x{:02X} n=0x{:02X}) hl=0x{hl:04X} de=0x{de:04X} \
                             b=0x{b:02X} xor={xor_mode}",
                            z.mem[i], n.mem[i]
                        );
                    }
                }
            }
        }
    }

    /// `0x7FCD` (внутренний цикл `CopyBuf2Screen`) обязан совпасть с Z80:
    /// копирование 60 строк по 32 байта через одну строку — регистры, флаги,
    /// такты, `R` и вся память, включая резиду стека. Адреса назначения
    /// подобраны так, чтобы копия не задела код рутины (0x7FCD..0x7FEE),
    /// `compDownScrAdr_HL` (0x8445..0x8458) и страницу 0xEF00..0xF000 —
    /// иначе расхождение было бы артефактом теста, а не хука.
    ///
    /// SP взят игровыми значениями 4116h..411Eh: стек в игре живёт в
    /// строках 1–7 экрана, и резиду push/call виден на экране. SP внутри
    /// зоны записи копии тест не использует: там настоящий код сам себя
    /// ломает — ldir перезаписывает вытолкнутые регистры перед pop, — так
    /// что эквивалентность в таких условиях не определена.
    #[test]
    fn scr_copy_alt_lines_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        // (HL = источник, DE = назначение, входные A, F, C, SP).
        let cases: &[(u16, u16, u8, u8, u8, u16)] = &[
            (0xC000, 0x4020, 0x11, 0x00, 0x22, 0x411C), // реальная пара CopyBuf2Screen
            (0xC100, 0x4120, 0x23, 0xFF, 0x33, 0x411E), // реальная пара, второй проход
            (0xC000, 0x4020, 0x34, 0x01, 0x44, 0x4116), // игровой SP из полёта
            (0x0000, 0x0000, 0x67, 0x0F, 0x77, 0xEFFE), // записи в ПЗУ отбрасываются шиной
            (0x1234, 0x0800, 0x78, 0xF0, 0x88, 0xEFFE),
            (0x7E00, 0x08FF, 0x89, 0x3C, 0x99, 0xEFFE), // источник у кода рутины
            (0xFF00, 0x0C00, 0x9A, 0xC3, 0xAA, 0xEFFE), // H источника через FFh
            (0x8445, 0xC000, 0xAB, 0x00, 0xBB, 0xEFFE), // чтение со строки compDown
            (0xAABB, 0xFF00, 0xBC, 0x99, 0xCC, 0xEFFE),
            (0x4000, 0x9040, 0xCD, 0x69, 0xDD, 0xEFFE),
            (0x0100, 0x6000, 0xDE, 0x00, 0xEE, 0xEFFE),
            (0xE800, 0xB800, 0xEF, 0xA5, 0xFF, 0xEFFE),
            (0xFFFF, 0x0F00, 0xF0, 0x5A, 0x11, 0xEFFE), // чтения через 0000h
        ];
        for &(hl, de, a, f, c, sp) in cases {
            // Вся память — детерминированный паттерн, кроме кода рутины и
            // compDownScrAdr_HL (иначе Z80 перезаписал бы свои инструкции
            // посреди цикла).
            let seed = (hl as usize) ^ ((de as usize) << 1) ^ (a as usize);
            for i in 0..0x1_0000usize {
                if (0x7FCD..0x7FEF).contains(&i) || (0x8445..0x8459).contains(&i) {
                    continue;
                }
                let v = (i.wrapping_mul(31).wrapping_add(seed) & 0xFF) as u8;
                z.mem[i] = v;
                n.mem[i] = v;
            }
            set(&mut z.cpu, a, hl, de, f);
            set(&mut n.cpu, a, hl, de, f);
            z.cpu.b = 0x99; // входной B перезаписывается `ld b,3Ch`
            n.cpu.b = 0x99;
            z.cpu.c = c;
            n.cpu.c = c;
            let zs = z80_sub_sp(&mut z, 0x7FCD, sp);
            n.cpu.sp = sp;
            let ns = hook_sub(&mut n, 0x7FCD);
            assert_eq!(
                zs, ns,
                "0x7FCD hl=0x{hl:04X} de=0x{de:04X} a={a:02X} f={f:02X} c={c:02X} sp=0x{sp:04X}"
            );
            // Вся память, кроме страницы 0xEF00..0xF000 (маркер и резиду
            // стека харнесса при SP=0xEFFE) и двух байт маркера по SP.
            let skip = |i: usize| {
                (0xEF00..0xF000).contains(&i) || i == sp as usize || i == sp as usize + 1
            };
            if let Some(i) = (0..0x1_0000usize).find(|&i| !skip(i) && z.mem[i] != n.mem[i]) {
                panic!(
                    "0x7FCD: память разошлась по адресу 0x{i:04X} \
                     (z=0x{:02X} n=0x{:02X}) hl=0x{hl:04X} de=0x{de:04X} \
                     a={a:02X} f={f:02X} sp=0x{sp:04X}",
                    z.mem[i], n.mem[i]
                );
            }
        }
    }

    /// Ветка Yield: когда целая итерация копии не влезает в остаток кадра,
    /// хук обязан отдать управление в точку цикла `0x7FCF` с состоянием ровно
    /// на начало итерации, а рутину дальше доигрывает обычный Z80. Так длинная
    /// копия (≈99000 тактов, больше кадра) переживает vblank-прерывание в
    /// середине, не расходясь с прогоном без хука.
    ///
    /// Мини-диспетчер ниже повторяет `Spectrum::run_frame`: на хуке циклы
    /// прибавляются, `RET` выполняется только если хук не отдал управление.
    /// Состояние по достижении такта `budget` обязано совпасть с чистым
    /// Z80-прогоном — по регистрам, флагам, `R`, SP, PC и всей памяти.
    #[test]
    fn scr_copy_alt_lines_yield_matches_z80() {
        // Плотная сетка бюджетов: кадровая граница попадает и в середину
        // копии, и внутрь коротких хуков (0x7FE7/0x8445), которые должны
        // уступать место Z80, чтобы не «проскочить» границу.
        let mut budgets: Vec<u64> = vec![500, 2000, 58664, 59954, 60000, 60050, 69000];
        budgets.extend((5000..61000).step_by(1009));
        for &budget in &budgets {
            let mut z = kladov();
            let mut n = kladov();
            install(&mut n, true);
            // Детерминированный паттерн, кроме кода рутины и compDownScrAdr_HL.
            for i in 0..0x1_0000usize {
                if (0x7FCD..0x7FEF).contains(&i) || (0x8445..0x8459).contains(&i) {
                    continue;
                }
                let v = (i.wrapping_mul(31).wrapping_add(7) & 0xFF) as u8;
                z.mem[i] = v;
                n.mem[i] = v;
            }
            for m in [&mut z, &mut n] {
                set(&mut m.cpu, 0x11, 0xC000, 0x4020, 0x00);
                m.cpu.c = 0x22;
                m.cpu.sp = 0x411E; // игровой стек в экранной памяти
                m.mem[0x411E] = 0x34;
                m.mem[0x411F] = 0x12;
                m.cpu.cycles = 0;
                m.cpu.pc = 0x7FCD;
                m.frame_target = budget;
            }
            // Чистый Z80: шагаем, пока не выйдем за бюджет кадра.
            let mut cz = std::mem::take(&mut z.cpu);
            while cz.cycles < budget {
                cz.step(&mut z);
            }
            z.cpu = cz;
            // Хук + Z80: мини-диспетчер, как в `Spectrum::run_frame`.
            let mut cn = std::mem::take(&mut n.cpu);
            let mut yielded = false;
            while cn.cycles < budget {
                let pc = cn.pc;
                if let Some(hook) = n.hooks.get(&pc).copied() {
                    let fits = n
                        .hook_max
                        .get(&pc)
                        .copied()
                        .map_or(true, |max| cn.cycles + max <= budget);
                    if fits {
                        n.hook_yield = false;
                        let cyc = hook(&mut cn, &mut n);
                        cn.cycles += cyc;
                        if n.hook_yield {
                            yielded = true;
                            continue;
                        }
                        let lo = n.mem[cn.sp as usize] as u16;
                        let hi = n.mem[cn.sp.wrapping_add(1) as usize] as u16;
                        cn.sp = cn.sp.wrapping_add(2);
                        cn.pc = lo | (hi << 8);
                        continue;
                    }
                }
                cn.step(&mut n);
            }
            n.cpu = cn;
            // При бюджете меньше одной итерации хук не запускается (охрана
            // `hook_max`) — рутину целиком играет Z80, отдачи управления нет.
            if budget >= COPY_ALT_MAX_ITER {
                assert!(yielded, "budget={budget}: хук ни разу не отдал управление");
            }
            assert_eq!(snap(&z.cpu), snap(&n.cpu), "регистры budget={budget}");
            assert_eq!(z.cpu.pc, n.cpu.pc, "PC budget={budget}");
            assert_eq!(z.cpu.sp, n.cpu.sp, "SP budget={budget}");
            if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
                panic!(
                    "0x7FCD yield: память разошлась по адресу 0x{i:04X} \
                     (z=0x{:02X} n=0x{:02X}) budget={budget}",
                    z.mem[i], n.mem[i]
                );
            }
        }
    }

    /// Пара машин (чистый Z80 и с хуками) с одинаковым детерминированным
    /// паттерном памяти везде, кроме кода и таблицы звуковой рутины
    /// (`0x9560..0x96C0`): данные эффектов (`HL`), переменные автомата и
    /// самомодифицируемые операнды лежат вне этого диапазона либо задаются
    /// случаем явно.
    fn sfx_pair() -> (Spectrum, Spectrum) {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n, true);
        for i in 0..0x1_0000usize {
            if (0x9560..0x96C0).contains(&i) {
                continue;
            }
            let v = (i.wrapping_mul(37).wrapping_add(11) & 0xFF) as u8;
            z.mem[i] = v;
            n.mem[i] = v;
        }
        (z, n)
    }

    /// Готовит обе машины к запуску `sub_9579`: «мусорные» входные регистры
    /// (рутина обязана перезаписать их до использования), нулевые такты и
    /// стековый маркер возврата.
    fn sfx_prepare(m: &mut Spectrum) {
        let c = &mut m.cpu;
        c.cycles = 0;
        c.pc = 0x9579;
        c.sp = 0x411E;
        c.a = 0xA5;
        c.f = 0x00;
        c.b = 0x77;
        c.c = 0x66;
        c.d = 0x55;
        c.e = 0x44;
        c.h = 0x33;
        c.l = 0x22;
        m.mem[0x411E] = 0x34; // маркер возврата 0x1234
        m.mem[0x411F] = 0x12;
    }

    /// Выполняет рутину на Z80 до маркера, обновляя такт начала инструкции
    /// (как `Spectrum::run_frame`), чтобы переходы бипера встали на те же такты.
    fn sfx_z80_run(m: &mut Spectrum) {
        const RET: u16 = 0x1234;
        let mut cpu = std::mem::take(&mut m.cpu);
        let mut guard = 0u32;
        while cpu.pc != RET {
            assert!(
                guard < 20_000_000,
                "sub_9579 не вернулась (pc=0x{:04X})",
                cpu.pc
            );
            m.set_cycle_clock(cpu.cycles);
            cpu.step(m);
            guard += 1;
        }
        m.cpu = cpu;
    }

    /// Выполняет хук `sub_9579` и завершает его `RET` так же, как это делает
    /// диспетчер `Spectrum::run_frame` (хук сам `RET` не делает).
    fn sfx_hook_run(m: &mut Spectrum) {
        let mut cpu = std::mem::take(&mut m.cpu);
        m.frame_target = u64::MAX;
        m.hook_yield = false;
        let cyc = (m.hooks[&0x9579])(&mut cpu, m);
        cpu.cycles += cyc;
        assert!(!m.hook_yield, "хук 0x9579 отдал управление вне кадра");
        let lo = m.mem[cpu.sp as usize] as u16;
        let hi = m.mem[cpu.sp.wrapping_add(1) as usize] as u16;
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.pc = lo | (hi << 8);
        m.cpu = cpu;
    }

    /// Строгое сравнение двух машин после прогона звуковой рутины.
    fn sfx_assert(name: &str, z: &Spectrum, n: &Spectrum) {
        assert_eq!(snap(&z.cpu), snap(&n.cpu), "{name}: регистры/флаги/R/такты");
        assert_eq!(z.cpu.pc, n.cpu.pc, "{name}: PC");
        assert_eq!(z.cpu.sp, n.cpu.sp, "{name}: SP");
        assert_eq!(z.border, n.border, "{name}: бордюр");
        assert_eq!(z.beeper, n.beeper, "{name}: бипер");
        assert_eq!(z.beeper_edges(), n.beeper_edges(), "{name}: переходы бипера");
        assert_eq!(z.paging_writes, n.paging_writes, "{name}: обращения к 7FFD");
        assert_eq!(z.paging_last, n.paging_last, "{name}: значение 7FFD");
        if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
            panic!(
                "{name}: память разошлась по адресу 0x{i:04X} \
                 (z=0x{:02X} n=0x{:02X})",
                z.mem[i], n.mem[i]
            );
        }
    }

    /// `sub_9579` (звуковой автомат) обязан совпасть с Z80 по всем регистрам,
    /// флагам, `R`, тактам, `SP`, всей памяти, бордюру и биперу на всех ветвях:
    /// ранние выходы, состояния `byte_9522` = 1/2/3 и `0x83`, все 8 битов
    /// `nBeepFX` (таблица `jp`-ов `0x9561`), оба варианта самомодификации
    /// `0x95F2` и оба исхода `ret nz`.
    #[test]
    fn sfx_update_matches_z80() {
        let cases: &[(&str, fn(&mut Spectrum))] = &[
            // FlagsSetUp: бит 7 снят -> немедленный выход.
            ("flags bit7=0", |m| m.mem[0xFC0E] = 0x00),
            // Эффекта нет и не запрошен -> выход.
            ("idle: no state, no request", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0;
            }),
            // byte_9522 = 1: пакет импульсов (0x9624) с разными операндами
            // самомодифицируемого `add a,n` (0x962B).
            ("state 1, op 05", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 1;
                m.mem[0x962B] = 0x05;
                m.mem[0x9523] = 0x08; // E
                m.mem[0x9524] = 0x02; // D -> ret nz
                m.mem[0x96BF] = 0x02;
            }),
            ("state 1, op FD", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 1;
                m.mem[0x962B] = 0xFD;
                m.mem[0x9523] = 0x0F;
                m.mem[0x9524] = 0x01; // D -> эпилог 0x9610
                m.mem[0x96BF] = 0x01;
            }),
            ("state 1, op D3", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 1;
                m.mem[0x962B] = 0xD3;
                m.mem[0x9523] = 0x00;
                m.mem[0x9524] = 0x03; // D -> ret nz
                m.mem[0x96BF] = 0x07;
            }),
            // byte_9522 = 2: шум (0x95E7) с обеими самомодификациями 0x95F2.
            ("state 2, srl patch, D->9610", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 2;
                m.mem[0x95F2] = 0xCB;
                m.mem[0x95F3] = 0x3F;
                m.mem[0x9523] = 0x00;
                m.mem[0x9524] = 0x01; // D = 1
                m.mem[0x9525] = 0x1A;
                m.mem[0x9526] = 0x03; // HL = 0x031A
                m.mem[0x96BF] = 0x03;
            }),
            ("state 2, add 10h patch, D->ret nz", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 2;
                m.mem[0x95F2] = 0xC6;
                m.mem[0x95F3] = 0x10;
                m.mem[0x9523] = 0x00;
                m.mem[0x9524] = 0x02; // D = 2 -> ret nz
                m.mem[0x9525] = 0x00;
                m.mem[0x9526] = 0x80; // HL = 0x8000
                m.mem[0x96BF] = 0x00;
            }),
            // byte_9522 = 3: тон с огибающей (0x95A6).
            ("state 3, ret nz", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 3;
                m.mem[0x95AE] = 0x40; // C
                m.mem[0x95BD] = 0x00; // add a,n
                m.mem[0x9523] = 0x02; // E = 2
                m.mem[0x9524] = 0x01; // D = 1
                m.mem[0x9525] = 0x00;
                m.mem[0x9526] = 0x80;
                m.mem[0x96BF] = 0x01;
            }),
            ("state 3, exhausted -> 9610", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 3;
                m.mem[0x95AE] = 0x40;
                m.mem[0x95BD] = 0x60;
                m.mem[0x9523] = 0x01; // E = 1 -> dec e = 0
                m.mem[0x9524] = 0x01;
                m.mem[0x9525] = 0x00;
                m.mem[0x9526] = 0x80;
                m.mem[0x96BF] = 0x04;
            }),
            // byte_9522 = 0x83: исчерпание уводит на перезапуск 0x95CF.
            ("state 3 (0x83), restart -> 95CF", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0x83;
                m.mem[0x95AE] = 0x40;
                m.mem[0x95BD] = 0xA0;
                m.mem[0x9523] = 0x01;
                m.mem[0x9524] = 0x01;
                m.mem[0x9525] = 0x00;
                m.mem[0x9526] = 0x80;
                m.mem[0x96BF] = 0x05;
            }),
            // Запуск эффекта по каждому биту nBeepFX (таблица 0x9561).
            ("req bit0 -> 9678 (C=FE)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x01;
                m.mem[0x96BF] = 0x00;
            }),
            ("req bit1 -> 9674 (C=E4)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x02;
                m.mem[0x96BF] = 0x01;
            }),
            ("req bit2 -> 9692 (C=02)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x04;
                m.mem[0x96BF] = 0x02;
            }),
            ("req bit3 -> 965E (state 2)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x08;
                m.mem[0x96BF] = 0x03;
            }),
            ("req bit4 -> 9641 (state 1, FD)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x10;
                m.mem[0x96BF] = 0x04;
            }),
            ("req bit5 -> 9648 (state 1, D3)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x20;
                m.mem[0x96BF] = 0x05;
            }),
            ("req bit6 -> 9657 (add 10h)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x40;
                m.mem[0x96BF] = 0x06;
            }),
            ("req bit7 -> 9657 (add 10h)", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x80;
                m.mem[0x96BF] = 0x07;
            }),
            // Несколько битов сразу: берётся младший установленный.
            ("req 0x2A -> bit1", |m| {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x2A;
                m.mem[0x96BF] = 0x01;
            }),
        ];
        for &(name, setup) in cases {
            let (mut z, mut n) = sfx_pair();
            setup(&mut z);
            setup(&mut n);
            sfx_prepare(&mut z);
            sfx_prepare(&mut n);
            sfx_z80_run(&mut z);
            sfx_hook_run(&mut n);
            sfx_assert(name, &z, &n);
        }
    }

    /// Ветка Yield звукового автомата: один чанк эффекта длиннее кадра, поэтому
    /// на границе кадра хук обязан отдать управление Z80 ровно на той
    /// инструкции, что не влезает, с состоянием на её начало. Мини-диспетчер
    /// повторяет `Spectrum::run_frame`; состояние по достижении такта `budget`
    /// обязано совпасть с чистым Z80-прогоном по регистрам, флагам, `R`, `SP`,
    /// `PC`, всей памяти, бордюру и биперу.
    #[test]
    fn sfx_update_yield_matches_z80() {
        // Биты nBeepFX = 0x40 дают самый длинный чанк (100 итераций шумового
        // цикла), гарантированно длиннее кадра.
        let mut budgets: Vec<u64> = vec![10, 100, 500, 5000, 69000, 69888, 200_000, 400_000];
        budgets.extend((1000..300_000).step_by(7919));
        for &budget in &budgets {
            let (mut z, mut n) = sfx_pair();
            for m in [&mut z, &mut n] {
                m.mem[0xFC0E] = 0x80;
                m.mem[0x9522] = 0;
                m.mem[0x7128] = 0x40; // бит 6 -> 0x9657, длинный шум
                m.mem[0x96BF] = 0x06;
                sfx_prepare(m);
                m.frame_target = budget;
            }
            // Чистый Z80: шагаем, пока не выйдем за бюджет кадра или рутина
            // не вернётся по маркеру.
            {
                let mut cz = std::mem::take(&mut z.cpu);
                while cz.cycles < budget && cz.pc != 0x1234 {
                    z.set_cycle_clock(cz.cycles);
                    cz.step(&mut z);
                }
                z.cpu = cz;
            }
            // Хук + Z80: мини-диспетчер, как в `Spectrum::run_frame`.
            let mut cn = std::mem::take(&mut n.cpu);
            let mut yielded = false;
            while cn.cycles < budget && cn.pc != 0x1234 {
                let pc = cn.pc;
                if let Some(hook) = n.hooks.get(&pc).copied() {
                    let fits = n
                        .hook_max
                        .get(&pc)
                        .copied()
                        .map_or(true, |max| cn.cycles + max <= budget);
                    if fits {
                        n.hook_yield = false;
                        let cyc = hook(&mut cn, &mut n);
                        cn.cycles += cyc;
                        n.set_cycle_clock(cn.cycles);
                        if n.hook_yield {
                            yielded = true;
                            continue;
                        }
                        let lo = n.mem[cn.sp as usize] as u16;
                        let hi = n.mem[cn.sp.wrapping_add(1) as usize] as u16;
                        cn.sp = cn.sp.wrapping_add(2);
                        cn.pc = lo | (hi << 8);
                        continue;
                    }
                }
                n.set_cycle_clock(cn.cycles);
                cn.step(&mut n);
            }
            n.cpu = cn;
            // Если кадровая граница пришлась на середину чанка, хук обязан был
            // отдать управление хотя бы раз (иначе он «проскочил» бы границу).
            // При бюджете меньше одного шага охрана `hook_max` не пускает хук —
            // рутину целиком доигрывает Z80.
            if budget >= SFX_MAX_STEP && z.cpu.pc != 0x1234 {
                assert!(yielded, "budget={budget}: хук ни разу не отдал управление");
            }
            sfx_assert(&format!("yield budget={budget}"), &z, &n);
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
