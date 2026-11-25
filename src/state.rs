//! Централизованное состояние игры.
//!
//! Нативные компоненты получают отсюда текущее состояние, не обращаясь
//! напрямую к эмулятору. Состояние собирается из живой памяти Z80 и из
//! состояния нативных экранов; по мере замены экранов эмулятора нативными
//! источник данных меняется, а внешний интерфейс остаётся тем же.
//!
//! От этого состояния зависит выбор того, что показывается на экране.

use crate::flight;
use crate::world;

/// Где находится игрок.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    /// Корабль пристыкован к станции.
    Station,
    /// Корабль в космосе (в полёте, в прыжке, на стыковке/вылете).
    Space,
}

/// Анимация, отображаемая в режиме анимации.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Animation {
    /// Вылет со станции.
    Launch,
    /// Гиперпереход (отсчёт и прыжок).
    Hyperspace,
    /// Стыковка со станцией.
    Docking,
}

/// Экран оригинальной игры, соответствующий значению `WhatOnScreen`.
///
/// Перечисляет все известные экраны оригинала; неизвестные/ещё не названные
/// коды сохраняются как [`Other`](Self::Other). В каждый момент активен ровно
/// один экран.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginalScreen {
    /// Вид из кабины: вперёд (`0`).
    FrontView,
    /// Вид из кабины: назад (`1`).
    RearView,
    /// Вид из кабины: влево (`2`).
    LeftView,
    /// Вид из кабины: вправо (`3`).
    RightView,
    /// Покупка товаров на станции (клавиша `2`), экран `4`.
    BuyCargo,
    /// Продажа товаров на станции (клавиша `3`), экран `5`.
    SellCargo,
    /// Закупка оборудования на станции (клавиша `4`), экран `6`.
    EquipShip,
    /// Карта галактики (клавиша `I`), экран `7`.
    GalacticChart,
    /// Локальная карта (клавиша `O`), экран `8`.
    LocalChart,
    /// Данные о планете (клавиша `P`), экран `9`.
    PlanetInfo,
    /// Рыночные цены (клавиша `K`), экран `0x0A`.
    MarketPrices,
    /// Статус командира (клавиша `L`), экран `0x0B`.
    CommanderStatus,
    /// Инвентарь (Enter), экран `0x0C`.
    Inventory,
    /// Экран оригинала с иным/ещё не описанным кодом `WhatOnScreen`.
    Other(u8),
}

impl OriginalScreen {
    /// Разобрать экран по значению `WhatOnScreen`.
    pub fn from_what_on_screen(value: u8) -> Self {
        match value {
            0 => Self::FrontView,
            1 => Self::RearView,
            2 => Self::LeftView,
            3 => Self::RightView,
            4 => Self::BuyCargo,
            5 => Self::SellCargo,
            6 => Self::EquipShip,
            7 => Self::GalacticChart,
            8 => Self::LocalChart,
            9 => Self::PlanetInfo,
            0x0A => Self::MarketPrices,
            0x0B => Self::CommanderStatus,
            0x0C => Self::Inventory,
            other => Self::Other(other),
        }
    }

    /// Значение `WhatOnScreen`, соответствующее экрану.
    #[allow(dead_code)]
    pub fn what_on_screen(self) -> u8 {
        match self {
            Self::FrontView => 0,
            Self::RearView => 1,
            Self::LeftView => 2,
            Self::RightView => 3,
            Self::BuyCargo => 4,
            Self::SellCargo => 5,
            Self::EquipShip => 6,
            Self::GalacticChart => 7,
            Self::LocalChart => 8,
            Self::PlanetInfo => 9,
            Self::MarketPrices => 0x0A,
            Self::CommanderStatus => 0x0B,
            Self::Inventory => 0x0C,
            Self::Other(value) => value,
        }
    }

    /// Вид из кабины (`0..=3`).
    #[allow(dead_code)]
    pub fn is_cockpit_view(self) -> bool {
        matches!(
            self,
            Self::FrontView | Self::RearView | Self::LeftView | Self::RightView
        )
    }
}

/// Логический экран, который сейчас отображается.
///
/// В каждый момент активен ровно один экран. Варианты покрывают заставку,
/// переходы, нативные экраны (в том числе нативные альтернативы экранов
/// оригинала) и все экраны оригинальной игры, которые ещё рисует эмулятор.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    /// Заставка (интро) — нативный титульный экран.
    Intro,
    /// Режим анимации: вылет со станции, гиперпереход или стыковка.
    /// Показывается только передний вид (`Front View`) с соответствующей
    /// анимацией.
    Animation(Animation),
    /// Нативный вид из кабины (полёт).
    Flight,
    /// Нативная карта галактики (F6).
    GalacticMap,
    /// Нативный экран закупки оборудования (F5).
    EquipShop,
    /// Нативный экран инвентаря (Enter), альтернатива экрану `0x0C`.
    Inventory,
    /// Нативный экран статуса командира (F9), альтернатива экрану `L`.
    PilotStatus,
    /// Нативный экран информации о планете (F7), альтернатива экрану `P`.
    PlanetInfo,
    /// Нативный экран цен (альтернатива экрану `K`).
    Market,
    /// Экран оригинальной игры, который рисует эмулятор.
    Emulator(OriginalScreen),
}

impl Screen {
    /// Экран оригинала, отрисованный эмулятором, если активен именно он.
    #[allow(dead_code)]
    pub fn emulator(self) -> Option<OriginalScreen> {
        match self {
            Self::Emulator(screen) => Some(screen),
            _ => None,
        }
    }

    /// Значение `WhatOnScreen` экрана оригинала, отрисованного эмулятором.
    #[allow(dead_code)]
    pub fn emulator_what_on_screen(self) -> Option<u8> {
        self.emulator().map(OriginalScreen::what_on_screen)
    }

    /// Логический экран оригинала, которому соответствует активный экран.
    ///
    /// Для нативных альтернатив возвращается экран оригинала с тем же
    /// назначением; для заставки и анимаций оригинального экрана нет.
    #[allow(dead_code)]
    pub fn original(self) -> Option<OriginalScreen> {
        match self {
            Self::Intro | Self::Animation(_) => None,
            Self::Flight => Some(OriginalScreen::FrontView),
            Self::GalacticMap => Some(OriginalScreen::LocalChart),
            Self::EquipShop => Some(OriginalScreen::EquipShip),
            Self::Inventory => Some(OriginalScreen::Inventory),
            Self::PilotStatus => Some(OriginalScreen::CommanderStatus),
            Self::PlanetInfo => Some(OriginalScreen::PlanetInfo),
            Self::Market => Some(OriginalScreen::MarketPrices),
            Self::Emulator(screen) => Some(screen),
        }
    }

    /// Показывается ли нативный вид из кабины.
    #[allow(dead_code)]
    pub fn is_flight(self) -> bool {
        matches!(self, Self::Flight)
    }

    /// Показывается ли режим анимации (вылет, гиперпереход или стыковка).
    #[allow(dead_code)]
    pub fn is_animation(self) -> bool {
        matches!(self, Self::Animation(_))
    }

    /// Показывается ли нативная карта галактики.
    #[allow(dead_code)]
    pub fn is_galactic_map(self) -> bool {
        matches!(self, Self::GalacticMap)
    }

    /// Показывается ли нативный экран закупки.
    #[allow(dead_code)]
    pub fn is_equip_shop(self) -> bool {
        matches!(self, Self::EquipShop)
    }
}

/// Состояние миссий в версии Кладова.
///
/// Read-only снимок полей Z80: нативный слой только показывает его и никогда
/// не меняет. Адреса собраны здесь, чтобы обработчики экранов не читали
/// память эмулятора напрямую (см. `docs/z80/kladov-missions.md`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MissionInfo {
    /// `PilotMission1` (`0xD081`): `1` — первая миссия ожидает сдачи.
    pub mission1: u8,
    /// `PilotMission1_1` (`0xD0BA`): миссия активна в полёте. Вне сейва.
    pub mission1_in_flight: u8,
    /// `Mission2` (`0x9CB2`, банк 2): состояние ASP второй миссии.
    pub mission2: u8,
    /// `PilotMission3` (`0xD0AF`): состояние миссии таргонов.
    pub mission3: u8,
    /// `PilotExtWeapon` (`0xD0B0`): биты 0..2 — маркеры выдачи миссий,
    /// бит 3 — завершение 3-й, бит 5 — пассажиры в трюме.
    pub ext_weapon: u8,
    /// `PilotMineralLife` (`0xD087`): `0` — нет, `0xFF` — погибла.
    pub mineral_life: u8,
    /// `PilotTargonWar` (`0xD08F`): бит 7 — война, `0x01` — конец войны.
    pub targon_war: u8,
    /// `PilotRating2` (`0xD062`): ранг войны в битах 3..1.
    pub rating2: u8,
    /// `byte_70DC` (`0x70DC`): накопитель хода войны. Вне сейва.
    pub war_progress: u8,
    /// `CargoFound` (`0xD0C7`): упакованная пара `de` для `sub_D162`,
    /// не адрес текста. Вне сейва.
    pub cargo_found: u16,
}

impl MissionInfo {
    /// Прочитать миссии из снимка памяти Z80. Отсутствующие адреса дают нули.
    pub fn from_memory(mem: &[u8]) -> Self {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        Self {
            mission1: byte(0xD081),
            mission1_in_flight: byte(0xD0BA),
            mission2: byte(0x9CB2),
            mission3: byte(0xD0AF),
            ext_weapon: byte(0xD0B0),
            mineral_life: byte(0xD087),
            targon_war: byte(0xD08F),
            rating2: byte(0xD062),
            war_progress: byte(0x70DC),
            cargo_found: u16::from_le_bytes([byte(0xD0C7), byte(0xD0C8)]),
        }
    }
}

/// Текущее состояние игры.
///
/// Это снимок: его нужно обновлять при смене состояния эмулятора или
/// нативных экранов (обычно раз в кадр или при обработке ввода).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    /// Где находится игрок — на станции или в космосе.
    pub location: Location,
    /// Текущий активный экран. В каждый момент отображается ровно один экран.
    pub screen: Screen,
    /// Открыто ли меню паузы (поверх текущего экрана).
    pub menu: bool,
    /// Идёт ли заставка (игра ещё не начата).
    pub intro: bool,
    /// Экран эмулятора (`WhatOnScreen`) на момент снимка.
    pub emu_screen: u8,
    /// Текущая планетная система для заголовков нативных экранов.
    pub system: String,
    /// Планета, выбранная для показа на экране информации (F7).
    pub planet: String,
    /// Read-only состояние миссий.
    pub missions: MissionInfo,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            location: Location::Station,
            screen: Screen::Intro,
            menu: false,
            intro: true,
            emu_screen: 0,
            system: String::new(),
            planet: String::new(),
            missions: MissionInfo::default(),
        }
    }
}

impl GameState {
    /// Состояние по данным эмулятора, когда нативные экраны не активны.
    #[allow(dead_code)]
    pub fn from_memory(mem: &[u8], intro: bool) -> Self {
        let location = if flight::z80_is_docked(mem) {
            Location::Station
        } else {
            Location::Space
        };
        let emu_screen = mem
            .get(flight::Z80_WHAT_ON_SCREEN)
            .copied()
            .unwrap_or(0);
        let screen = if intro {
            Screen::Intro
        } else {
            Screen::Emulator(OriginalScreen::from_what_on_screen(emu_screen))
        };
        Self {
            location,
            screen,
            menu: false,
            intro,
            emu_screen,
            system: world::current_system_name(mem),
            planet: world::selected_planet_name(mem),
            missions: MissionInfo::from_memory(mem),
        }
    }

    /// Игрок на станции.
    pub fn is_docked(&self) -> bool {
        self.location == Location::Station
    }

    /// Игрок в космосе.
    #[allow(dead_code)]
    pub fn in_space(&self) -> bool {
        self.location == Location::Space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Поля читаются по адресам версии Кладова, включая runtime-значения вне
    /// 102-байтового сейва. `CargoFound` — упакованная пара, не адрес текста.
    #[test]
    fn mission_info_reads_kladow_addresses() {
        let mut mem = vec![0u8; 0x10000];
        mem[0xD062] = 0x87;
        mem[0xD081] = 1;
        mem[0xD087] = 0x42;
        mem[0xD08F] = 0x82;
        mem[0xD0AF] = 3;
        mem[0xD0B0] = 0xA9;
        mem[0xD0BA] = 1;
        mem[0xD0C7..0xD0C9].copy_from_slice(&[0x82, 0xF0]);
        mem[0x70DC] = 0x11;
        mem[0x9CB2] = 2;

        let m = MissionInfo::from_memory(&mem);
        assert_eq!(m.rating2, 0x87);
        assert_eq!(m.mission1, 1);
        assert_eq!(m.mineral_life, 0x42);
        assert_eq!(m.targon_war, 0x82);
        assert_eq!(m.mission3, 3);
        assert_eq!(m.ext_weapon, 0xA9);
        assert_eq!(m.mission1_in_flight, 1);
        assert_eq!(m.cargo_found, 0xF082);
        assert_eq!(m.war_progress, 0x11);
        assert_eq!(m.mission2, 2);
    }

    /// Короткий снимок не должен паниковать на любом адресе миссий.
    #[test]
    fn mission_info_tolerates_short_memory() {
        assert_eq!(MissionInfo::from_memory(&[]), MissionInfo::default());
    }
}
