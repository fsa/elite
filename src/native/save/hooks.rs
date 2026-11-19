//! Одноразовые хуки перезапуска, телепорта, загрузки и MenuSaveLoad.

use super::*;

/// Одноразовая нативная замена точек цикла заставки (см. [`start_game`]).
///
/// Хук срабатывает на первой же исполненной точке цикла (`intro_loop_start`)
/// и уводит Z80 на ветку переинициализации ([`SaveLayout::docked_setup`]) —
/// ровно туда, куда родная заставка попадала по `SPACE`. Снимаем хук со всех
/// точек цикла (они могли быть поставлены все сразу). `B` выставляем как
/// делает сама ветка: младшие 2 бита `FlagsSetUp` — тип устройства управления.
pub(crate) fn start_game_gate(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    for &addr in layout.intro_loop_start {
        m.remove_hook(addr);
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
    m.remove_hook(layout.game_main);
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

/// Тело хука загрузки (см. [`request_load`]).
///
/// Родная загрузка из меню возвращает управление на `loc_7394`: та сбрасывает
/// `WhatOnScreen`, очищает экран и рисует кабину из `DataCocpit`, после чего
/// попадает в цикл заставки. Когда игрок нажимает `SPACE`, ветка
/// [`SaveLayout::docked_setup`] вызывает `SaveCocpitInBuffer` — то есть
/// сохраняет в `DataCocpit` уже нарисованную кабину.
///
/// Поэтому просто перейти в `docked_setup` нельзя: при загрузке со станции
/// (а сохраниться можно только там) `SaveCocpitInBuffer` записал бы в буфер
/// текущий экран станции, и после вылета приборная панель оказалась бы
/// забита станционным текстом. Повторяем подготовку оригинала, но без
/// заставки и ожидания `SPACE`: `ClsAndPrintRamk` → `PrnCocpit_Ifn` →
/// `docked_init` (та же ветка старта, но после настройки `FlagsSetUp`).
pub(crate) fn load_into_docked(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    // Снимаем хук со всех точек, куда его ставил `request_load`.
    for &addr in layout.intro_loop_start {
        m.remove_hook(addr);
    }
    m.remove_hook(layout.game_main);
    m.load_requested = false;

    // На виде из кабины (и на заставке, где `WhatOnScreen` тоже 0) нижняя
    // треть экрана уже занята кабиной — `SaveCocpitInBuffer` ниже снимет её
    // ровно как при обычном старте. Перерисовывать нечего.
    if m.mem[layout.what_on_screen as usize] < 4 {
        cpu.pc = layout.docked_init;
        m.hook_yield = true;
        return 0;
    }

    // Станция, торговля, карта и прочие экраны: в нижней трети экрана лежит
    // не кабина. Родной `loc_7394` в этом случае сбрасывает `WhatOnScreen`,
    // очищает экран и рисует кабину из `DataCocpit`, а уже потом заставка по
    // `SPACE` уходит в `docked_setup`. Повторяем эту подготовку без заставки:
    // `ClsAndPrintRamk` → `PrnCocpit_Ifn` → `docked_init`.
    m.mem[layout.what_on_screen as usize] = 0;

    // Возвраты на стеке: `ClsAndPrintRamk` вернётся в `PrnCocpit_Ifn`, а та —
    // в `docked_init`. `docked_init` уже не возвращается (уходит в `GameMain`).
    let mut sp = cpu.sp.wrapping_sub(2);
    m.mem[sp as usize] = layout.docked_init as u8;
    m.mem[sp.wrapping_add(1) as usize] = (layout.docked_init >> 8) as u8;
    sp = sp.wrapping_sub(2);
    m.mem[sp as usize] = layout.print_cockpit as u8;
    m.mem[sp.wrapping_add(1) as usize] = (layout.print_cockpit >> 8) as u8;
    cpu.sp = sp;
    cpu.pc = layout.clear_screen;
    m.hook_yield = true;
    0
}

/// Хук загрузки для версии Кладова.
pub(crate) fn load_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    load_into_docked(cpu, m, KLADOV_SAVE)
}

/// Включить перехват игрового сохранения: при вызове `MenuSaveLoad` игра
/// выставляет флаг `machine.save_requested`, а главный цикл показывает диалог
/// выбора файла и записывает блок командира (родная рутина требует
/// 128K-пейджинга и в 48K-ядре не работает).
pub fn install_save(m: &mut Spectrum, layout: SaveLayout) {
    install_hook(m, layout.menu_save_load, layout.hook, 0);
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
/// Возвращает «не загружено» (`A=0`, `Z=1`) — как родная рутина при обычном
/// возврате: главный цикл не уходит на перезапуск, а только выставляет
/// `machine.save_requested`. Загрузка идёт отдельным путём ([`request_load`])
/// и `MenuSaveLoad` не использует.
pub(crate) fn menu_save_load(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    let a = layout.control_key1 as usize;
    m.mem[a..a + PRESSED_KEY_BYTES].fill(0);
    cpu.a = 0;
    m.save_requested = true;
    cpu.f |= ZF;
    0
}

/// Нативная замена `MenuSaveLoad` в версии Кладова (`0xD04E`).
pub(crate) fn menu_save_load_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, KLADOV_SAVE)
}
