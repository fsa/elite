//! Адреса блока командира и раскладки версий игры.

use crate::galaxy::{self, GalaxyLayout};

// ————— Смещения внутри блока командира (одинаковы в обеих версиях) —————

/// `PilotCash2low`: u16 LE, «тысячи» десятых (цена товара += `b0` сюда).
pub(super) const OFF_CASH2: usize = 18;
/// `PilotCash1`: u16 LE, «десятые» (0..9999, переполнение уходит в cash2).
pub(super) const OFF_CASH1: usize = 20;
/// `PilotFreeCargo`: свободные тонны трюма.
pub(super) const OFF_FREE_CARGO: usize = 40;
/// `PilotFrontLaser`: крепления лазеров FRONT/REAR/LEFT/RIGHT.
pub(super) const OFF_LASERS: usize = 41;
/// `PilotMission1`: <>0 выдана 1-я миссия.
pub(super) const OFF_MISSION1: usize = 45;
/// `Pilot_Fuel`.
pub(super) const OFF_FUEL: usize = 46;
/// `PilotMissiles`.
pub(super) const OFF_MISSILES: usize = 47;
/// `PilotMineralLife` (только у Кладова: конвертация миссионного отсека).
pub(super) const OFF_MINERAL: usize = 51;
/// Флаги товаров: номер товара `n` -> `pilot + 45 + n` (`sub_EB38`/`sub_B713`).
pub(super) const FLAG_BASE: usize = 45;
/// `byte_D08E` (у Кладова) — «<>0 есть Cobra Mk4», в проверке «установлено».
pub(super) const OFF_COBRA: usize = 58;
/// `PilotPlanetTechLvl`.
pub(super) const OFF_TECH: usize = 72;

/// Раскладка данных экрана закупки (адреса зависят от версии игры).
#[derive(Clone, Copy)]
pub struct ShopLayout {
    /// Адрес живого блока командира (`PilotName`).
    pub pilot: u16,
    /// База таблицы цен (14 записей по 3 байта).
    pub prices: u16,
    /// `IndDanger`: <>0 — в полёте, покупки запрещены.
    pub in_danger: u16,
    /// Производная переменная `Fuel = 2*fuel + fuel/4` (дальность прыжка).
    pub fuel_derived: u16,
    /// Прятать ли строку FUEL при выданной 1-й миссии (баг отображения
    /// оригинала Elite, см. комментарий в начале файла).
    pub hide_fuel_on_mission: bool,
    /// Проверка/отметка «установлено» по-кладовски: покупка делает
    /// `flag -= 1`, «установлено» = `(flag|0x80)+1 == byte_D08E` (при
    /// `byte_D08E == 0` — `flag ∈ {0x7F, 0xFF}`). В оригинале покупка пишет
    /// `0xFF`, а «установлено» = `flag != 0` (`sub_B616`).
    pub dec_flags: bool,
    /// Раскладка галактики — имя текущей системы в шапке.
    pub galaxy: GalaxyLayout,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_SHOP: ShopLayout = ShopLayout {
    pilot: 0xD054,
    prices: 0x705F,
    in_danger: 0x7104,
    fuel_derived: 0x70EC,
    hide_fuel_on_mission: false,
    dec_flags: true,
    galaxy: galaxy::KLADOV_GALAXY,
};

/// Раскладка оригинальной версии (`zx_bin/elite.bin`).
pub const _ELITE_SHOP: ShopLayout = ShopLayout {
    pilot: 0xA816,
    prices: 0x9236,
    in_danger: 0x7062,
    fuel_derived: 0x704A,
    hide_fuel_on_mission: true,
    dec_flags: false,
    galaxy: galaxy::KLADOV_GALAXY,
};
