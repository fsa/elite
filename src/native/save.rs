//! Подсистема сохранения/загрузки командира и перезапуска игры.
//!
//! Раскладка адресов ([`SaveLayout`]) для версий Кладова и оригинала, нативное
//! сохранение/загрузка блока командира в файл, телепорт по карте галактики,
//! одноразовые «ворота» загрузки в полёте и перехват `MenuSaveLoad`.

use super::*;

/// Сколько байт клавиш сбрасывает родная `ResetPressedKeys`.
pub(crate) const PRESSED_KEY_BYTES: usize = 4;

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
    /// Адрес вызова `MenuSaveLoad` в главном цикле (у Кладова `0x749F`, в
    /// оригинале `0x72B5`). Сюда встаёт [`request_load`], минуя проверку
    /// клавиши движения, — игра сама вызывает хук и уходит на перезапуск.
    pub menu_save_load_callsite: u16,
    /// Родная `SaveCocpitInBuffer` — сохраняет текущий экран в буфер кокпита
    /// (`DataCocpit` и `0x5B00`).
    ///
    /// Буфер кокпита актуален только на станции: его обновляет эта рутина при
    /// стыковке (`call SaveCocpitInBuffer` в `LaunchDocking`). В полёте его
    /// затирает буфер космического вида, а родная игра в полёте загрузку и не
    /// разрешает. [`request_load`] же работает в любом состоянии, поэтому перед
    /// перезапуском [`load_gate`] вызывает эту рутину, чтобы перезапуск нарисовал
    /// кокпит, а не мусор.
    pub save_cocpit: Option<u16>,
    /// Адрес ветки «в полёте» (`jr nz, <полёт>`) перед вызовом
    /// `MenuSaveLoad` (у Кладова `0x7493`, в оригинале `0x72A9`).
    ///
    /// Родная игра разрешает сохранение/загрузку только на станции
    /// (`IndDanger == 0`); в полёте эта ветка пропускает вызов `MenuSaveLoad`.
    /// [`request_load`] ставит сюда одноразовый `gate_hook`, чтобы загрузка
    /// работала и в полёте.
    pub load_gate: Option<u16>,
    /// Одноразовая нативная замена ветки `jr nz, <полёт>`: переводит
    /// управление на [`Self::menu_save_load_callsite`] (см. [`request_load`]).
    gate_hook: Hook,
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
    /// Точка входа главного цикла игры (`GameMain`, у Кладова `0x7471`, в
    /// оригинале `0x7287`).
    ///
    /// Достигается переходом (`jr`/`jp`), поэтому стек сбалансирован —
    /// [`request_teleport`] может повесить сюда одноразовый хук и увести
    /// управление в сторону, не ломая стек.
    pub game_main: u16,
    /// Ветка «переинициализация и старт на станции» (у Кладова `0x73FD`, в
    /// оригинале `0x721F`): зовёт `InitGame`, обнуляет `InFreeSpace`/`IndDanger`
    /// и печатает экран станции, после чего `jr GameMain`.
    ///
    /// Сюда переводит управление хук чита телепортации (`F1` на карте), чтобы
    /// оказаться пристыкованным в текущей (только что переписанной) системе.
    pub docked_setup: u16,
    /// Адрес системного флага `FlagsSetUp` (у Кладова `0xFC0E`, в оригинале
    /// `0xFC14`). Младшие 2 бита — тип устройства управления; ветка
    /// переинициализации делает `and 0FCh / or b`, поэтому хук восстанавливает
    /// их из этого байта.
    pub flags_setup: u16,
    /// Нативная реализация `MenuSaveLoad` для этой версии.
    hook: Hook,
    /// Одноразовый хук чита телепортации (см. [`request_teleport`]).
    teleport_hook: Hook,
    /// Точки цикла заставки, на которые [`start_game`] вешает одноразовый хук
    /// запуска игры. Хук срабатывает на первой же исполненной точке и уводит Z80
    /// на ветку переинициализации ([`Self::docked_setup`]) — ровно то, что
    /// делала заставка по `SPACE`.
    ///
    /// У каждой версии цикл заставки состоит из нескольких участков, и в
    /// зависимости от состояния входа исполняются разные: у Кладова первый
    /// участок (`loc_739E`) крутится только пока «нажата» клавиша
    /// (`inc a; jr nz` при `PressKeyPrnSymbol != 0xFF`), поэтому в начале игры
    /// исполняется второй участок (`loc_73C3`↔`loc_73E2`); в оригинале —
    /// наоборот, первый (`loc_71A2`). Чтобы запуск не зависел от того, в каком
    /// участке застала игра, хук вешается сразу на все точки.
    pub intro_loop_start: &'static [u16],
    /// Одноразовый хук запуска игры с заставки (см. [`start_game`]).
    start_hook: Hook,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD04E,
    menu_save_load_callsite: 0x749F,
    save_cocpit: Some(0x817E),
    load_gate: Some(0x7493),
    gate_hook: load_gate_kladov_hook,
    control_key1: 0xFC08,
    pilot_name: 0xD054,
    commander_len: 0x66,
    pilot_shadow: Some((0xD232, 0x66)),
    sun_crd: 0x7107,
    game_main: 0x7471,
    docked_setup: 0x73FD,
    flags_setup: 0xFC0E,
    hook: menu_save_load_kladov_hook,
    teleport_hook: teleport_kladov_hook,
    intro_loop_start: &[0x739E, 0x73C3, 0x73E2],
    start_hook: start_game_kladov_hook,
};

/// Раскладка оригинальной версии (`zx_bin/elite.bin`).
pub const _ELITE_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD2E4,
    menu_save_load_callsite: 0x72B5,
    save_cocpit: Some(0x8F40),
    load_gate: Some(0x72A9),
    gate_hook: load_gate_elite_hook,
    control_key1: 0xFC0E,
    pilot_name: 0xA816,
    commander_len: 0x68,
    pilot_shadow: Some((0xA910, 0x66)),
    sun_crd: 0x7065,
    game_main: 0x7287,
    docked_setup: 0x721F,
    flags_setup: 0xFC14,
    hook: menu_save_load_elite_hook,
    teleport_hook: teleport_elite_hook,
    intro_loop_start: &[0x71A2, 0x71E5],
    start_hook: start_game_elite_hook,
};

/// Блок сохранения командира в памяти.
pub fn commander_block(m: &Spectrum, layout: SaveLayout) -> &[u8] {
    let a = layout.pilot_name as usize;
    &m.mem[a..a + layout.commander_len]
}

/// Смещение поля `PilotDataOfSun` (9 байт координат солнца) внутри блока
/// командира. Одинаково в обеих версиях: в оригинале это `byte_A96D` =
/// `PilotDataSaved`+93, у Кладова `unk_D28F` = `aJameson_1`+93.
pub(crate) const SUN_FIELD_OFF: usize = 93;

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

/// Попросить игру загрузиться с уже помещёнными в память данными командира
/// (см. [`commander_load`]).
///
/// Загрузка выполняется родным путём: главный цикл вызывает `MenuSaveLoad`,
/// перехват отвечает «загружено» (NZ) — и игра сама уходит на перезапуск
/// (`jp nz, 0x7394` у Кладова): заново проходит стартовую последовательность и
/// оказывается пристыкованной на станции системы из сейва.
///
/// Важно не подменять `PC`/`SP` вручную: вход в ветку перезапуска с кадровой
/// границы (из произвольного места программы) ломает стек и через ~1000 кадров
/// игру выносит в ПЗУ. Поэтому игра доходит до вызова сама.
pub fn request_load(m: &mut Spectrum, layout: SaveLayout) {
    m.load_requested = true;
    m.save_requested = false;
    // Родная игра разрешает сохранение/загрузку только на станции
    // (`IndDanger == 0`); в полёте ветка `jr nz` уводит мимо вызова. Ставим
    // одноразовый хук на эту ветку, чтобы загрузка работала в любом состоянии.
    if let Some(gate) = layout.load_gate {
        install_hook(m, gate, layout.gate_hook, 0);
    }
}

/// Смещение `Pilot_CrdYSystem` в блоке командира (`PilotName`).
///
/// Одинаково в обеих версиях: у Кладова `0xD097 - 0xD054`, в оригинале
/// `0xA859 - 0xA816`.
pub(crate) const CRD_Y_SYSTEM_OFF: usize = 0x43;
/// Смещение `Pilot_CrdXSystem` в блоке командира (`PilotName`).
pub(crate) const CRD_X_SYSTEM_OFF: usize = 0x45;

/// Одноразовая нативная замена точек цикла заставки (см. [`start_game`]).
///
/// Хук срабатывает на первой же исполненной точке цикла (`intro_loop_start`)
/// и уводит Z80 на ветку переинициализации ([`SaveLayout::docked_setup`]) —
/// ровно туда, куда родная заставка попадала по `SPACE`. Снимаем хук со всех
/// точек цикла (они могли быть поставлены все сразу). `B` выставляем как
/// делает сама ветка: младшие 2 бита `FlagsSetUp` — тип устройства управления.
pub(crate) fn start_game_gate(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    for &addr in layout.intro_loop_start {
        m.hooks.remove(&addr);
        m.hook_max.remove(&addr);
    }
    cpu.b = m.mem[layout.flags_setup as usize] & 3;
    cpu.pc = layout.docked_setup;
    m.hook_yield = true;
    0
}

/// Хук старта для версии Кладова.
pub(crate) fn start_game_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    start_game_gate(cpu, m, KLADOV_SAVE)
}

/// Хук старта для оригинальной версии.
pub(crate) fn start_game_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    start_game_gate(cpu, m, _ELITE_SAVE)
}

/// Запустить игру с нативной заставки (клавиша `SPACE`).
///
/// Родного цикла ожидания на заставке больше нет — он заменён нативным
/// (`splash`), поэтому запуск выполняем так же, как телепорт: одноразовыми
/// хуками на все точки цикла заставки ([`SaveLayout::intro_loop_start`]) уводим
/// Z80 на ветку переинициализации ([`SaveLayout::docked_setup`]). Она зовёт
/// `InitGame`, очищает флаги полёта, печатает экран станции и переходит в
/// `GameMain`, поэтому [`machine::Spectrum::intro_seen`] снимается корректно.
/// Как и при загрузке/телепорте, стек не ломаем — переход с кадровой границы
/// на ветку, достигаемую обычно тоже переходом.
pub fn start_game(m: &mut Spectrum, layout: SaveLayout) {
    for &addr in layout.intro_loop_start {
        install_hook(m, addr, layout.start_hook, 0);
    }
}

/// Телепорт «на станцию» системы с координатами `(x, y_raw)` — временный чит
/// (клавиша `F1` на нативной карте галактики, см. `hdmap`).
///
/// Всё делает родной код:
/// 1. Синхронизируем теневой блок командира из живого — ровно как при
///    сохранении. `InitGame` восстанавливает пилота из тени, поэтому без этой
///    копии телепорт откатил бы деньги/груз/приборы к последнему сохранению.
/// 2. Пишем координаты системы в живой и теневой блоки командира.
/// 3. Ставим одноразовый хук на [`SaveLayout::game_main`]. Дойдя до `GameMain`,
///    хук уводит управление на ветку переинициализации
///    ([`SaveLayout::docked_setup`]): она зовёт `InitGame` (пересобирает данные
///    текущей планеты по координатам, печатает название и цены), обнуляет
///    `InFreeSpace`/`IndDanger` и печатает экран станции — корабль оказывается
///    пристыкованным в новой системе.
pub fn request_teleport(m: &mut Spectrum, layout: SaveLayout, x: u8, y: u8) {
    // 1. Тень ← живой блок (как в `commander_save`).
    if let Some((shadow, len)) = layout.pilot_shadow {
        let live = layout.pilot_name as usize;
        let n = layout.commander_len.min(len);
        let data = m.mem[live..live + n].to_vec();
        m.mem[shadow as usize..shadow as usize + n].copy_from_slice(&data);
    }
    // 2. Координаты системы в живом и теневом блоках. Цель (`Pilot_CrdXTarget`)
    //    выставит сам `InitGame` (`target = system`).
    let live = layout.pilot_name as usize;
    m.mem[live + CRD_Y_SYSTEM_OFF] = y;
    m.mem[live + CRD_X_SYSTEM_OFF] = x;
    if let Some((shadow, len)) = layout.pilot_shadow {
        if len > CRD_X_SYSTEM_OFF {
            let s = shadow as usize;
            m.mem[s + CRD_Y_SYSTEM_OFF] = y;
            m.mem[s + CRD_X_SYSTEM_OFF] = x;
        }
    }
    // 3. Одноразовый хук на главный цикл.
    install_hook(m, layout.game_main, layout.teleport_hook, 0);
}

/// Тело хука телепортации (см. [`request_teleport`]).
pub(crate) fn teleport(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    m.hooks.remove(&layout.game_main);
    m.hook_max.remove(&layout.game_main);
    // Ветка переинициализации выполняет `and 0FCh / or b`, где `b` — младшие
    // два бита `FlagsSetUp` (тип устройства управления). Берём их из памяти,
    // чтобы не сбить настройку.
    cpu.b = m.mem[layout.flags_setup as usize] & 3;
    cpu.pc = layout.docked_setup;
    m.hook_yield = true;
    0
}

/// Хук телепортации для версии Кладова.
pub(crate) fn teleport_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    teleport(cpu, m, KLADOV_SAVE)
}

/// Хук телепортации для оригинальной версии.
pub(crate) fn teleport_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    teleport(cpu, m, _ELITE_SAVE)
}

/// Нативная замена ветки «в полёте» перед вызовом `MenuSaveLoad`: переводит
/// управление прямо на вызов, как если бы игрок был на станции.
///
/// Перед этим обязательно вызывается родная `SaveCocpitInBuffer`
/// ([`SaveLayout::save_cocpit`]): буфер кокпита актуален только на станции, а в
/// полёте его затирает буфер космического вида. Без этого перезапуск нарисовал
/// бы вместо кокпита мусор (перезапуск копирует `DataCocpit` на экран, а затем
/// сохраняет получившийся экран обратно в буфер). Вызов делаем «вложенным»:
/// кладём адрес возврата на стек игры и встаём на вход рутины — её `ret` вернёт
/// управление на вызов `MenuSaveLoad`. Стек остаётся сбалансированным.
pub(crate) fn load_gate(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    // Удаляем одноразовый хук в любом случае
    let gate_addr = layout.load_gate;
    if let Some(gate) = gate_addr {
        m.hooks.remove(&gate);
    }
    // Если загрузка не запрошена (случайный проход через gate), просто
    // пропускаем инструкцию jr nz (как будто не взята в docked-состоянии)
    if !m.load_requested {
        m.hook_yield = true;
        if let Some(gate) = gate_addr {
            cpu.pc = gate + 2;
        } else {
            // fallback: не трогаем PC
        }
        return 0;
    }
    // load_requested == true: форсируем путь к MenuSaveLoad с обновлением кокпит-буферов
    m.hook_yield = true;
    match layout.save_cocpit {
        Some(save) => {
            let ret = layout.menu_save_load_callsite;
            cpu.sp = cpu.sp.wrapping_sub(2);
            m.mem[cpu.sp as usize] = ret as u8;
            m.mem[cpu.sp.wrapping_add(1) as usize] = (ret >> 8) as u8;
            cpu.pc = save;
        }
        None => cpu.pc = layout.menu_save_load_callsite,
    }
    0
}

/// Ветка «в полёте» для версии Кладова (`0x7493`).
pub(crate) fn load_gate_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    load_gate(cpu, m, KLADOV_SAVE)
}

/// Ветка «в полёте» для оригинальной версии (`0x72A9`).
pub(crate) fn load_gate_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    load_gate(cpu, m, _ELITE_SAVE)
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
/// Если приложение попросило загрузку ([`request_load`]), возвращает
/// «загружено» (A=0, Z=0) — главный цикл уходит на ветку переинициализации
/// (`jp nz, 0x7394` у Кладова, `jp nz, loc_71BE` в оригинале).
///
/// Иначе возвращает «не загружено» (A=0, Z=1), как родная рутина при обычном
/// возврате: главный цикл только выставит `machine.save_requested`.
pub(crate) fn menu_save_load(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    let a = layout.control_key1 as usize;
    m.mem[a..a + PRESSED_KEY_BYTES].fill(0);
    cpu.a = 0;
    if m.load_requested {
        m.load_requested = false;
        m.save_requested = false;
        // Обновим буферы кокпита перед перезапуском, чтобы панель
        // отрисовалась корректно (особенно при загрузке со станции).
        if let Some(save_cocpit) = layout.save_cocpit {
            // Вложенный вызов: кладём адрес возврата 0 (пустышка) и
            // временно уходим в SaveCocpitInBuffer; она сделает RET.
            // Но нам нужно вернуться обратно в этот хук? Проще - 
            // Копируем буферы кокпита вручную (DataCocpit -> 0xC000, 0x5A00 -> 0x5B00)
            {
                let mem = &mut m.mem;
                for i in 0..0x800 {
                    mem[0xC000 + i] = mem[0x5000 + i];
                }
                for i in 0..0x100 {
                    mem[0x5B00 + i] = mem[0x5A00 + i];
                }
            }
        }
        cpu.f &= !ZF;
    } else {
        m.save_requested = true;
        cpu.f |= ZF;
    }
    0
}

/// Нативная замена `MenuSaveLoad` в версии Кладова (`0xD04E`).
pub(crate) fn menu_save_load_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, KLADOV_SAVE)
}

/// Нативная замена `MenuSaveLoad` в оригинальной версии (`0xD2E4`).
pub(crate) fn menu_save_load_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, _ELITE_SAVE)
}
