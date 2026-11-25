//! Чтение полётного состояния из памяти эмулятора Z80.
//!
//! Здесь собраны адреса переменных игры и разбор их форматов. Всё остальное
//! в подсистеме полёта работает с уже прочитанными значениями и не знает,
//! где именно они лежат в памяти Spectrum.

use crate::cockpit::RadarView;

// Адреса в памяти Z80 (версия Кладова). Координаты объектов ведутся уже
// в системе координат игрока: камера стоит в начале координат вдоль +Z.
/// Минимальный объём памяти, покрывающий все адреса полётных данных
/// (включая таблицу звёзд и флаг J-драйва).
pub(super) const Z80_MEM_MIN: usize = 0x7EE0;
/// `WhatOnScreen` — что сейчас показывает игра: `0..=3` — вид из кабины
/// (вперёд, назад, влево, вправо), остальные значения — станция и её меню,
/// карты, инвентарь и прочие экраны оригинала.
pub const Z80_WHAT_ON_SCREEN: usize = 0x70E3;
/// `WhatOnScreen` для локальной карты (`SHORT RANGE CHART`).
pub const Z80_SCREEN_LOCAL_MAP: u8 = 8;
/// `WhatOnScreen` для экрана информации о планете (`Cmd_P_InfoPlanet`, `P`).
pub const Z80_SCREEN_INFO_PLANET: u8 = 9;
/// `WhatOnScreen` для экрана рынка (`Cmd_K_MarketPrices`, клавиша `K`).
pub const Z80_SCREEN_MARKET: u8 = 0x0A;
/// `WhatOnScreen` для экрана пилота (`Cmd_L_PilotInfo`, клавиша `L`).
pub const Z80_SCREEN_PILOT_INFO: u8 = 0x0B;
/// `WhatOnScreen` для инвентаря (клавиша `Enter`).
pub const Z80_SCREEN_INVENTORY: u8 = 0x0C;
/// `InFreeSpace`: `<>0` — корабль вне планетной системы (межпланетное
/// пространство, в том числе после перехвата таргонами).
///
/// Оригинал в `GameMain` вызывает `PrnSunPlanet` только при
/// `InFreeSpace == 0`, поэтому в космосе солнце и планета не рисуются,
/// хотя их координаты продолжают обновляться для компаса.
pub const Z80_IN_FREE_SPACE: usize = 0x7103;
/// `IndDanger`: `0` — корабль пристыкован к станции, иначе — в полёте.
pub const Z80_IND_DANGER: usize = 0x7104;
/// `PlayerGameOver`: `<>0` — корабль игрока погиб. Оригинал проигрывает
/// анимацию гибели и уходит на заставку.
pub const Z80_PLAYER_GAME_OVER: usize = 0x711F;
/// `JumpDocking`: `1` — прыжок/стыковка, `2` — межгалактический прыжок.
pub const Z80_JUMP_DOCKING: usize = 0xD03F;
/// `FlgJumpDocking`: `1`, пока идёт отсчёт и анимация прыжка/стыковки/вылета.
/// В это время оригинал игнорирует клавиши `1`-`4`.
pub const Z80_FLG_JUMP_DOCKING: usize = 0xD040;
/// `JumpCountdown` — обратный отсчёт до гиперпрыжка (`0x15` в начале).
///
/// Оригинал уменьшает его на единицу за кадр и показывает `>> 1`, то есть
/// на экране идут числа `10..1`. Ноль означает, что началась сама анимация
/// перехода.
pub const Z80_JUMP_COUNTDOWN: usize = 0xDBA1;
/// `HighAngleLR` — угловая скорость крена (знак-величина).
pub const Z80_HIGH_ANGLE_LR: usize = 0x70E4;
/// `HighAngleDU` — угловая скорость тангажа (знак-величина).
pub const Z80_HIGH_ANGLE_DU: usize = 0x70E5;
/// `ControlKey1` — флаги сервисных клавиш (инверсия тангажа, маскировка и т.п.).
pub const Z80_CONTROL_KEY1: usize = 0xFC08;
/// `ControlKey2`: бит `7` взведён, пока игра находится в паузе Кладова.
pub const Z80_CONTROL_KEY2: usize = 0xFC0A;
/// `PressKeyPrnSymbol`: символ последней клавиши, разобранной `ScanKeys`.
pub const Z80_PRESS_KEY_SYMBOL: usize = 0xFC0D;
/// `FlagsSetUp` — битовая маска настроек, переключаемых в паузе Кладова.
pub const Z80_FLAGS_SETUP: usize = 0xFC0E;
/// Байт `byte_72A2` — поворот станины лазеров на карте.
pub const Z80_MAP_ROTATION: usize = 0x72A2;
/// Флаг режима компаса, который переключает клавиша `C` в паузе.
pub const Z80_COMPASS_FLAG: usize = 0xFD0A;
/// Модифицируемый immediate в `CopyLocator2Scr`: `7` — сканер, `0` — локатор.
pub const Z80_SCANNER_MODE: usize = 0xB3B3;
/// `sub_7A84` меняет этот байт на `0x90` при включении локатора.
pub const Z80_SCANNER_INVERT: usize = 0x972F;
/// `FlgSaveScaner`: локатор копируется на экран на следующем игровом такте.
pub const Z80_FLG_SAVE_SCANNER: usize = 0x96C4;
/// `HighDeltaSpeed` — текущая скорость, 0..=31.
pub(super) const Z80_HIGH_DELTA_SPEED: usize = 0x70E6;
/// `Speed` — индикатор скорости: `5 * HighDeltaSpeed + 5`, 0..=0xA0.
///
/// В отличие от `HighDeltaSpeed`, эту ячейку J-драйв не трогает, поэтому
/// скорость надёжнее брать отсюда.
pub(super) const Z80_SPEED_GAUGE: usize = 0x70F4;

/// `byte_D08E`: <>0 — у игрока Cobra Mk IV (в блоке командира `PilotName`).
/// От типа корабля зависит цвет длинных клеток приборной панели.
pub const Z80_COBRA_MK4: usize = 0xD08E;
/// `PilotFrontLaser`: типы лазеров в креплениях FRONT, REAR, LEFT, RIGHT.
/// `0` — лазер не установлен.
pub const Z80_LASER_MOUNTS: usize = 0xD07D;
/// `ShieldFront` — передний щит, 0..0xA0.
pub(super) const Z80_SHIELD_FRONT: usize = 0x70E8;
/// `ShieldRear` — задний щит, 0..0xA0.
pub(super) const Z80_SHIELD_REAR: usize = 0x70EA;
/// `DamageFrontShield`: за такт сюда складывается урон переднему щиту.
///
/// Оригинал обнуляет ячейку в начале `MoveAllObjects`, а `Damage2Pilot`
/// читает её в конце игрового такта. Это надёжнее, чем падение самого щита:
/// при нулевом щите попадание лазера уже ничего не отнимает, но луч на экране
/// оригинала всё равно есть.
pub(super) const Z80_DAMAGE_FRONT_SHIELD: usize = 0x9CA3;
/// `DamageRearShield` — то же для заднего щита.
pub(super) const Z80_DAMAGE_REAR_SHIELD: usize = 0x9CA4;
/// `TempLaser` — нагрев лазера, 0..0xA0.
pub(super) const Z80_TEMP_LASER: usize = 0x70F0;
/// Таблица звёзд оригинала (`byte_7A39`): 16 пар координат.
///
/// Изменяется чаще, чем координаты объектов: используется только тестами как
/// пример кадра, на котором игровая модель ещё не шагнула.
#[cfg(test)]
pub(super) const Z80_STARS_TABLE: usize = 0x7A39;
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
/// `seed up` в слоте (`SlotObj+0x1C`): приращение скорости за игровой такт.
///
/// В оригинале это команда движения AI, а не булевый флаг двигателя, но это
/// ближайший пер-корабельный признак работающего двигателя: нулевое значение
/// означает полёт по инерции, ненулевое — активное ускорение или торможение.
pub(super) const Z80_SLOT_OACCEL: usize = 0x1C;
/// `oEnerg` в слоте: 0 — слот пуст.
pub(super) const Z80_SLOT_OENERG: usize = 0x22;
/// `oEnerg` уничтоженного объекта: `FireLaser` пишет `0xFF` перед
/// `DestroyObj`, а на следующем игровом такте слот очищается до нуля.
pub(super) const Z80_OENERG_DESTROYED: u8 = 0xFF;
/// `oAdrData` в слоте: адрес блока модели.
pub(super) const Z80_SLOT_OADRDATA: usize = 0x23;
/// Координаты солнца (9 байт, знак-величина).
pub(super) const Z80_SUN_CRD: usize = 0x7107;
/// Координаты планеты (9 байт, знак-величина).
pub(super) const Z80_PLANET_CRD: usize = 0x7110;

/// Находится ли корабль вне планетной системы (`InFreeSpace <> 0`).
pub fn z80_in_free_space(mem: &[u8]) -> bool {
    mem.get(Z80_IN_FREE_SPACE).copied().unwrap_or(0) != 0
}

/// Интенсивность двигателя NPC по приращению скорости в слоте.
///
/// `0` соответствует полёту по инерции: двигатель не работает, хотя сам
/// объект может двигаться. Любое ненулевое приращение считается работающим
/// двигателем; масштаб грубый, так как AI пишет в поле и `1`, и `0xFF`.
pub(super) fn z80_slot_engine(mem: &[u8], base: usize) -> f32 {
    let Some(&accel) = mem.get(base + Z80_SLOT_OACCEL) else {
        return 0.0;
    };
    let step = (accel as i8).unsigned_abs() as f32;
    if step == 0.0 {
        0.0
    } else {
        (step / 10.0).clamp(0.35, 1.0)
    }
}

/// Вид радара, который включён в эмуляторе.
///
/// `sub_7A84` (клавиша `L` в паузе Кладова) меняет immediate в
/// `CopyLocator2Scr` с `7` на `0`: при `7` вызывается классический сканер, при
/// `0` — локатор Кладова. Именно этот байт и является источником правды для
/// нативного кокпита, а не состояние мыши.
pub fn z80_radar_view(mem: &[u8]) -> RadarView {
    if mem.get(Z80_SCANNER_MODE).copied() == Some(0) {
        RadarView::Kladov
    } else {
        RadarView::Classic
    }
}

/// Идёт ли переход: отсчёт гиперпрыжка, сам прыжок, вылет или стыковка.
pub fn z80_in_transition(mem: &[u8]) -> bool {
    mem.get(Z80_JUMP_DOCKING).copied().unwrap_or(0) != 0
        || mem.get(Z80_FLG_JUMP_DOCKING).copied().unwrap_or(0) != 0
}

/// Идёт ли уже сама анимация перехода, а не только отсчёт гиперпрыжка.
///
/// Оригинал взводит `JumpDocking`/`FlgJumpDocking` при запуске прыжка, но до
/// обнуления `JumpCountdown` ещё не рисует тоннель. Нативный слой в это время
/// разрешает переключать экраны и показывает число поверх каждого из них;
/// блокировка ввода нужна только когда началась анимация.
pub fn z80_in_transition_animation(mem: &[u8]) -> bool {
    z80_in_transition(mem) && mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0) == 0
}

/// Тип лазера в креплении `mount` (0 — лазера нет).
pub(super) fn laser_type(mem: &[u8], mount: usize) -> u8 {
    mem.get(Z80_LASER_MOUNTS + mount).copied().unwrap_or(0)
}

/// Погиб ли корабль игрока (`PlayerGameOver != 0`).
///
/// После гибели оригинал ещё держит `WhatOnScreen < 4` и проигрывает анимацию,
/// а затем уходит на заставку. Нативный вид в это время остаётся на экране,
/// но вместо подписи направления показывает «Game Over».
pub fn z80_player_dead(mem: &[u8]) -> bool {
    mem.get(Z80_PLAYER_GAME_OVER).copied().unwrap_or(0) != 0
}

/// Обнулить угловые скорости при гибели игрока.
///
/// Оригинал в `GameMain` при `BlockControl != 0` перестаёт вызывать
/// `CalcAngSpeed4Prn`, но оставляет последние `HighAngleLR/DU` в памяти.
/// `CalcNewCrdObj_Pl` продолжает поворачивать слоты на эти углы, поэтому
/// обломки уезжали в сторону последнего поворота корабля. Гасим скорости,
/// чтобы после гибели они летели только от игрока.
pub fn z80_stop_rotation(mem: &mut [u8]) {
    if mem.len() <= Z80_HIGH_ANGLE_DU {
        return;
    }
    mem[Z80_HIGH_ANGLE_LR] = 0;
    mem[Z80_HIGH_ANGLE_DU] = 0;
}

/// Показывает ли Z80 сейчас настоящий вид из кабины, а не переход.
///
/// Одного `WhatOnScreen < 4` мало: во время вылета со станции и прыжка игра
/// тоже держит `WhatOnScreen = 0`, но рисует тоннель/анимацию и игнорирует
/// управление. Нативные экраны (закупка, карта) должны ждать, пока оригинал
/// действительно окажется в полёте. Гибель корабля не отменяет вид: оригинал
/// ещё рисует кабину несколько секунд, пока идёт анимация.
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
