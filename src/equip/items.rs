//! Товары экрана закупки: список, типы, названия и константы.

/// Товар списка закупки (номер 1..14).
pub(super) struct Item {
    /// Оригинальное название (токены `0x69..0x76`).
    pub(super) name: &'static str,
    /// Короткая подсказка (современное дополнение к оригинальному списку).
    pub(super) desc: &'static str,
    pub(super) kind: Kind,
}

/// Тип товара — определяет поведение покупки.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kind {
    /// Топливо: заправка до 70, обнуление динамической цены.
    Fuel,
    /// Ракета: +1, максимум 4.
    Missile,
    /// Большой грузовой отсек: +15 тонн.
    Bay,
    /// Обычный флаг в блоке командира.
    Flag,
    /// Лазер указанного типа (1..4) — с выбором крепления.
    Laser(u8),
}

/// Список товаров в порядке оригинала (как в `Cmd_4_EquipShip`).
pub(super) const ITEMS: [Item; 14] = [
    Item { name: "FUEL", desc: "refill the fuel tank", kind: Kind::Fuel },
    Item { name: "MISSILE", desc: "one more missile (max 4)", kind: Kind::Missile },
    Item { name: "LARGE CARGO BAY", desc: "+15 tons of cargo space", kind: Kind::Bay },
    Item { name: "E.C.M.SYSTEM", desc: "defends against missiles", kind: Kind::Flag },
    Item { name: "EXTRA PULSE LASERS", desc: "cheap lasers", kind: Kind::Laser(1) },
    Item { name: "EXTRA BEAM LASERS", desc: "fast lasers", kind: Kind::Laser(2) },
    Item { name: "FUEL SCOOPS", desc: "collect floating cargo", kind: Kind::Flag },
    Item { name: "ESCAPE POD", desc: "survive ship destruction", kind: Kind::Flag },
    Item { name: "ENERGY BOMB", desc: "dump all energy at once", kind: Kind::Flag },
    Item { name: "ENERGY UNIT", desc: "recharge energy faster", kind: Kind::Flag },
    Item { name: "DOCKING COMPUTER", desc: "automatic docking", kind: Kind::Flag },
    Item { name: "GALACTIC HYPERSPACE", desc: "jump to another galaxy", kind: Kind::Flag },
    Item { name: "MILITARY LASER", desc: "powerful lasers", kind: Kind::Laser(3) },
    Item { name: "MINING LASER", desc: "lasers for asteroid mining", kind: Kind::Laser(4) },
];

/// Крепления лазера: `pilot+41+индекс` (PilotFrontLaser и далее).
pub(super) const MOUNTS: [&str; 4] = ["FRONT", "REAR", "LEFT", "RIGHT"];
/// Названия типов лазеров (0 — нет лазера).
pub(super) const LASER_NAMES: [&str; 5] = ["-", "PULSE", "BEAM", "MILITARY", "MINING"];

/// Название типа лазера из байта памяти (в повреждённом сейве может быть
/// что угодно — не паникуем).
pub(super) fn laser_name(t: u8) -> &'static str {
    LASER_NAMES.get(t as usize).copied().unwrap_or("?")
}
/// Товары-лазеры по типу лазера (1..4) — из них оригиналы возвращают цену
/// при замене (`loc_FD87+2` = `{5, 6, 13, 14}`).
pub(super) const LASER_ITEMS: [u8; 5] = [0, 5, 6, 13, 14];

/// Максимум товаров в списке.
pub(super) const MAX_ITEMS: usize = 14;
/// Полный бак (SetPilotFuel: `ld a,46h; call SetPilotFuel`).
pub(super) const FUEL_MAX: u8 = 70;
/// Максимум ракет (`cp 4; jp nc`).
pub(super) const MISSILES_MAX: u8 = 4;
/// Тонн добавляет большой грузовой отсек (`add a,0Fh`).
pub(super) const BAY_TONS: u8 = 15;
/// Ширина колонки названий: самое длинное — «GALACTIC HYPERSPACE» (19).
pub(super) const NAME_CHARS: i32 = 19;
/// Ширина колонки цены: влезает и «6000.0», и «FITTED».
pub(super) const PRICE_CHARS: i32 = 7;
/// Ширина списка в символах: рамка + номер 2 + зазор + имя 19 + зазор +
/// цена 7 + отступ (см. `Shop::geo`).
pub(super) const LIST_CHARS: i32 = 1 + 2 + 1 + NAME_CHARS + 1 + PRICE_CHARS + 1;
/// Минимум символов, оставляемых панели корабля (описания переносятся).
pub(super) const PANEL_MIN_CHARS: i32 = 18;
/// Сколько кадров показывается тост (~5 с при 50 к/с).
pub(super) const TOAST_FRAMES: u32 = 250;
