//! Привязка к конкретной версии игры: раскладка сейва, маркеры заставки и
//! данные текущей системы.

use crate::galaxy;
use crate::native;

/// Раскладка адресов сейва/лоада выбранной версии игры.
pub fn save_layout_kladov() -> native::SaveLayout {
    native::KLADOV_SAVE
}

/// Маркеры «заставки» и старта игры для выбранной версии. Пока игра
/// проигрывает заставку, нативные экраны (карта F10, закупка F4) открывать
/// нельзя — как и у оригинальных команд, которых на заставке нет.
///
/// Интро определяется «залипающе»: адрес из первого диапазона взводит флаг
/// [`machine::Spectrum::intro_seen`], а вход в `GameMain` (второе значение) его
/// снимает. Просто держать флаг по кадру нельзя: цикл заставки вызывает
/// длинные рутины (очистка/копирование экрана) вне своего диапазона, и флаг
/// мерцал бы.
///
/// * Кладова — аттракт-цикл `sub_728D` (`loc_739E`..`loc_73FD`), старт игры
///   `GameMain` = `0x7471`.
/// * Оригинал — `BeginGame` (`loc_71A2`..`loc_721F`), старт игры `GameMain`
///   = `0x7287`.
pub fn scene_markers(_kladov: bool) -> (Option<(u16, u16)>, Option<u16>) {
    (Some((0x739E, 0x73FD)), Some(0x7471))
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
