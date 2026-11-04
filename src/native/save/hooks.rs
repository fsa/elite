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
/// (`jp nz, 0x7394`).
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
        if layout.save_cocpit.is_some() {
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
