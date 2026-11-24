//! Раскладка сейва, маркеры заставки и данные текущей системы.

use crate::galaxy;
use crate::native;

/// `PilotCash2low` — старшая часть денег, число полных порций по 10000
/// десятых кредита (то есть по 1000 кредитов).
const OFF_CASH2: usize = 18;
/// `PilotCash1` — младшая часть денег, остаток 0..9999 десятых кредита.
const OFF_CASH1: usize = 20;
/// Максимум для двух половин денег (`cash2*10000 + cash1`).
pub const MAX_CASH_TENTHS: u32 = 65535 * 10000 + 9999;

/// Поле `PlanetPresent` — имя текущей системы, подготовленное оригиналом
/// (версия Кладова). Именно его оригинал печатает в заголовке рынка.
const PLANET_PRESENT: usize = 0xDEE4;
/// Поле `PlanetName` — имя планеты, выбранной для показа (версия Кладова).
const PLANET_NAME: usize = 0xDEED;
/// Длина обоих полей имени.
const PLANET_NAME_LEN: usize = 8;

/// Раскладка адресов сейва/лоада версии Кладова.
pub fn save_layout_kladov() -> native::SaveLayout {
    native::KLADOV_SAVE
}

/// Прочитать `u16 LE` из памяти, не выходя за границы среза.
fn read_u16(mem: &[u8], addr: usize) -> u32 {
    let lo = mem.get(addr).copied().unwrap_or(0) as u32;
    let hi = mem.get(addr + 1).copied().unwrap_or(0) as u32;
    lo | (hi << 8)
}

/// Деньги командира в десятых кредита: `PilotCash2low*10000 + PilotCash1`.
///
/// Обе ячейки — `u16 LE`; в оригинале `IncCASH` переносит в `PilotCash2low`
/// каждую полную порцию в 10000 десятых (1000 кредитов). Отдельно читать
/// только `PilotCash1` нельзя: старшая часть задаёт тысячи кредитов.
pub fn commander_cash_tenths(mem: &[u8], base: usize) -> u32 {
    read_u16(mem, base + OFF_CASH2) * 10000 + read_u16(mem, base + OFF_CASH1)
}

/// Записать деньги в блок командира, нормализуя перенос между ячейками.
///
/// Записываются только два поля денег: соседние `PilotRnd` и груз не трогаются.
pub fn set_commander_cash_tenths(mem: &mut [u8], base: usize, tenths: u32) {
    let tenths = tenths.min(MAX_CASH_TENTHS);
    let cash2 = (tenths / 10000) as u16;
    let cash1 = (tenths % 10000) as u16;
    if base + OFF_CASH1 + 2 > mem.len() {
        return;
    }
    mem[base + OFF_CASH2..base + OFF_CASH2 + 2].copy_from_slice(&cash2.to_le_bytes());
    mem[base + OFF_CASH1..base + OFF_CASH1 + 2].copy_from_slice(&cash1.to_le_bytes());
}

/// Маркеры «заставки» и старта игры. Пока игра проигрывает заставку,
/// нативные экраны (карта F6, закупка F4) открывать нельзя — как и у
/// оригинальных команд, которых на заставке нет.
///
/// Интро определяется «залипающе»: адрес из первого диапазона взводит флаг
/// [`machine::Spectrum::intro_seen`], а вход в `GameMain` (второе значение) его
/// снимает. Просто держать флаг по кадру нельзя: цикл заставки вызывает
/// длинные рутины (очистка/копирование экрана) вне своего диапазона, и флаг
/// мерцал бы.
///
/// Заставка — аттракт-цикл `sub_728D` (`loc_739E`..`loc_73FD`), старт игры
/// `GameMain` = `0x7471`.
pub fn scene_markers() -> (Option<(u16, u16)>, Option<u16>) {
    (Some((0x739E, 0x73FD)), Some(0x7471))
}

/// Точка конца прохода `GameMain`, где можно снимать состояние игры.
///
/// `PrintCompass` (`0x81A9`) вызывается после `MoveAllObjects`,
/// `CalcCrdSunPlanet`, `CalcCrdCoriolis2`, отрисовки и индикаторов. Снимок на
/// входе в `GameMain` отставал бы на проход: при вылете или перехвате корабль
/// сначала показывался бы там, где только что была планета, и наоборот.
pub fn snapshot_marker() -> u16 {
    0x81A9
}

/// Цвет планеты текущей системы из живой памяти Z80 — оттенок планеты на
/// экране полёта. Если координаты не совпали ни с одной звездой (например,
/// игра ещё на заставке), берётся первая планета галактики.
pub fn current_planet_color(mem: &[u8]) -> u32 {
    let layout = galaxy::KLADOV_GALAXY;
    let seed: [u8; 6] = mem
        .get(layout.seed..layout.seed + 6)
        .and_then(|s| s.try_into().ok())
        .unwrap_or([0; 6]);
    let planets = galaxy::planets(seed);
    let x = mem.get(layout.sys_x).copied().unwrap_or(0) as i32;
    let y = mem.get(layout.sys_y).copied().unwrap_or(0) as i32;
    let planet = planets
        .iter()
        .find(|p| p.x == x && p.y_raw == y)
        .unwrap_or(&planets[0]);
    galaxy::color(planet.state)
}

/// Имя текущей планетной системы в виде, как печатает оригинал
/// (`LAVE` -> `Lave`). Источник — поле `PlanetPresent`, которое игра сама
/// заполняет при старте, гиперпрыжке и стыковке.
pub fn current_system_name(mem: &[u8]) -> String {
    read_name(mem, PLANET_PRESENT)
}

/// Имя планеты, выбранной для показа на экране `P`/F7 (`LAVE` -> `Lave`).
/// Источник — поле `PlanetName`, которое оригинал заполняет при подготовке
/// экрана информации о планете.
pub fn selected_planet_name(mem: &[u8]) -> String {
    read_name(mem, PLANET_NAME)
}

/// Прочитать имя из 8-байтного поля: первая буква заглавная, остальные
/// строчные, как при печати `{01}`/`{03}` в оригинале.
fn read_name(mem: &[u8], addr: usize) -> String {
    let mut out = String::new();
    for i in 0..PLANET_NAME_LEN {
        let b = mem.get(addr + i).copied().unwrap_or(0);
        if b == 0 || b == b' ' {
            break;
        }
        let ch = b as char;
        if out.is_empty() {
            out.extend(ch.to_uppercase());
        } else {
            out.extend(ch.to_lowercase());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planet_names_are_read_from_the_right_fields() {
        let mut mem = vec![0u8; 0x10000];
        mem[PLANET_PRESENT..PLANET_PRESENT + 8].copy_from_slice(b"LAVE    ");
        mem[PLANET_NAME..PLANET_NAME + 8].copy_from_slice(b"DISCO   ");
        assert_eq!(current_system_name(&mem), "Lave");
        assert_eq!(selected_planet_name(&mem), "Disco");
    }
}
