//! Чтение полётного состояния из памяти эмулятора Z80.
//!
//! Здесь собраны адреса переменных игры и разбор их форматов. Всё остальное
//! в подсистеме полёта работает с уже прочитанными значениями и не знает,
//! где именно они лежат в памяти Spectrum.

// Адреса в памяти Z80 (версия Кладова). Координаты объектов ведутся уже
// в системе координат игрока: камера стоит в начале координат вдоль +Z.
/// Минимальный объём памяти, покрывающий все адреса полётных данных
/// (включая таблицу звёзд и флаг J-драйва).
pub(super) const Z80_MEM_MIN: usize = 0x7EE0;
/// `WhatOnScreen` — что сейчас показывает игра: `0..=3` — вид из кабины
/// (вперёд, назад, влево, вправо), остальные значения — станция и её меню,
/// карты, инвентарь и прочие экраны оригинала.
pub const Z80_WHAT_ON_SCREEN: usize = 0x70E3;
/// `IndDanger`: `0` — корабль пристыкован к станции, иначе — в полёте.
pub const Z80_IND_DANGER: usize = 0x7104;
/// `JumpDocking`: `1` — прыжок/стыковка, `2` — межгалактический прыжок.
pub const Z80_JUMP_DOCKING: usize = 0xD03F;
/// `FlgJumpDocking`: `1`, пока идёт анимация прыжка/стыковки/вылета. В это
/// время оригинал игнорирует клавиши `1`-`4`.
pub const Z80_FLG_JUMP_DOCKING: usize = 0xD040;
/// `JumpCountdown` — обратный отсчёт до гиперпрыжка (`0x15` в начале).
///
/// Оригинал уменьшает его на единицу за кадр и показывает `>> 1`, то есть
/// на экране идут числа `10..1`. Ноль означает, что началась сама анимация
/// перехода.
pub const Z80_JUMP_COUNTDOWN: usize = 0xDBA1;
/// `HighAngleLR` — угловая скорость крена (знак-величина).
pub(super) const Z80_HIGH_ANGLE_LR: usize = 0x70E4;
/// `HighAngleDU` — угловая скорость тангажа (знак-величина).
pub(super) const Z80_HIGH_ANGLE_DU: usize = 0x70E5;
/// `HighDeltaSpeed` — текущая скорость, 0..=31.
pub(super) const Z80_HIGH_DELTA_SPEED: usize = 0x70E6;
/// `Speed` — индикатор скорости: `5 * HighDeltaSpeed + 5`, 0..=0xA0.
///
/// В отличие от `HighDeltaSpeed`, эту ячейку J-драйв не трогает, поэтому
/// скорость надёжнее брать отсюда.
pub(super) const Z80_SPEED_GAUGE: usize = 0x70F4;

/// `byte_D08E`: <>0 — у игрока Cobra Mk IV (в блоке командира `PilotName`).
/// От типа корабля зависит цвет длинных клеток приборной панели.
pub(super) const Z80_COBRA_MK4: usize = 0xD08E;
/// `ShieldFront` — передний щит, 0..0xA0.
pub(super) const Z80_SHIELD_FRONT: usize = 0x70E8;
/// `TempLaser` — нагрев лазера, 0..0xA0.
pub(super) const Z80_TEMP_LASER: usize = 0x70F0;
/// Таблица звёзд оригинала (`byte_7A39`): 16 пар координат.
///
/// Оригинал пересчитывает её только на игровом такте (около 5.5 Гц), а не
/// каждый кадр 50 Гц. По изменению таблицы удобно ловить сам такт.
pub(super) const Z80_STARS_TABLE: usize = 0x7A39;
/// Длина таблицы звёзд в байтах.
pub(super) const Z80_STARS_LEN: usize = 0x20;
/// `StarsFastScroll` — взводится на такт J-драйва, гасится в `PrintStars`.
pub(super) const Z80_STARS_FAST_SCROLL: usize = 0x7EDD;
/// Слот станции (`objCoriolis`).
pub(super) const Z80_OBJ_CORIOLIS: usize = 0x6048;
/// Первый из шести основных слотов объектов.
pub(super) const Z80_OBJ_SLOT0: usize = 0x606F;
/// Размер слота `SlotObj`.
pub(super) const Z80_SLOT_SIZE: usize = 0x27;
/// Слот ракеты (`objMissile`).
pub(super) const Z80_OBJ_MISSILE: usize = 0x6159;
/// `oEnerg` в слоте: 0 — слот пуст.
pub(super) const Z80_SLOT_OENERG: usize = 0x22;
/// `oAdrData` в слоте: адрес блока модели.
pub(super) const Z80_SLOT_OADRDATA: usize = 0x23;
/// Координаты солнца (9 байт, знак-величина).
pub(super) const Z80_SUN_CRD: usize = 0x7107;
/// Координаты планеты (9 байт, знак-величина).
pub(super) const Z80_PLANET_CRD: usize = 0x7110;

/// Идёт ли сейчас анимация вылета, прыжка или стыковки. В это время оригинал
/// игнорирует клавиши `1`-`4`, а нативный вид не должен закрывать ролик.
pub fn z80_in_transition(mem: &[u8]) -> bool {
    mem.get(Z80_JUMP_DOCKING).copied().unwrap_or(0) != 0
        || mem.get(Z80_FLG_JUMP_DOCKING).copied().unwrap_or(0) != 0
}

/// Показывает ли Z80 сейчас настоящий вид из кабины, а не переход.
///
/// Одного `WhatOnScreen < 4` мало: во время вылета со станции и прыжка игра
/// тоже держит `WhatOnScreen = 0`, но рисует тоннель/анимацию и игнорирует
/// управление. Нативные экраны (закупка, карта) должны ждать, пока оригинал
/// действительно окажется в полёте.
pub fn z80_in_cockpit(mem: &[u8]) -> bool {
    let Some(&screen) = mem.get(Z80_WHAT_ON_SCREEN) else {
        return false;
    };
    screen < 4 && !z80_in_transition(mem)
}

/// Пристыкован ли корабль к станции (`IndDanger == 0`).
pub fn z80_is_docked(mem: &[u8]) -> bool {
    mem.get(Z80_IND_DANGER).copied() == Some(0)
}

/// Прочитать 24-битную координату игры.
///
/// Формат — знак-величина: старший бит третьего байта — знак, остальные
/// 23 бита — координата в единицах моделей (как в оригинале).
pub(super) fn read_signed24(b: &[u8]) -> f32 {
    if b.len() < 3 {
        return 0.0;
    }
    let mag = (((b[2] & 0x7F) as u32) << 16) | ((b[1] as u32) << 8) | (b[0] as u32);
    let v = mag as f32;
    if (b[2] & 0x80) != 0 {
        -v
    } else {
        v
    }
}

/// Прочитать знаковый байт в формате знак-величина (оси объекта).
pub(super) fn read_signed8(b: u8) -> f32 {
    let mag = (b & 0x7F) as f32;
    if (b & 0x80) != 0 {
        -mag
    } else {
        mag
    }
}
