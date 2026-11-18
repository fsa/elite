//! Централизованное состояние игры.
//!
//! Нативные компоненты получают отсюда текущее состояние, не обращаясь
//! напрямую к эмулятору. Состояние собирается из живой памяти Z80 и из
//! состояния нативных экранов; по мере замены экранов эмулятора нативными
//! источник данных меняется, а внешний интерфейс остаётся тем же.
//!
//! От этого состояния зависит выбор того, что показывается на экране.

use crate::flight;

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

/// Логический экран, который сейчас отображается.
///
/// Варианты покрывают как нативные экраны, так и ещё не перенесённые
/// экраны эмулятора, чтобы переход от эмуляторных экранов к нативным не
/// менял способа выбора отображения.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    /// Заставка (интро) — нативный титульный экран.
    Intro,
    /// Режим анимации: вылет со станции или гиперпереход. Показывается
    /// только передний вид (`Front View`) с соответствующей анимацией.
    Animation(Animation),
    /// Нативный вид из кабины (полёт).
    Flight,
    /// Нативная карта галактики (F10).
    GalacticMap,
    /// Нативный экран закупки оборудования (F4).
    EquipShop,
    /// Нативный экран статуса пилота (F9), альтернатива экрану `L`.
    PilotStatus,
    /// Нативный экран информации о планете (F7), альтернатива экрану `P`.
    PlanetInfo,
    /// Нативный экран цен (альтернатива экрану `K`).
    Market,
    /// Экран, который ещё рисует эмулятор (значение `WhatOnScreen`).
    Emulator(u8),
}

impl Screen {
    /// Экран эмулятора (`WhatOnScreen`), если сейчас показывается он.
    #[allow(dead_code)]
    pub fn emulator(self) -> Option<u8> {
        match self {
            Self::Emulator(what) => Some(what),
            _ => None,
        }
    }

    /// Показывается ли нативный вид из кабины.
    #[allow(dead_code)]
    pub fn is_flight(self) -> bool {
        matches!(self, Self::Flight)
    }

    /// Показывается ли режим анимации (вылет или гиперпереход).
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

/// Текущее состояние игры.
///
/// Это снимок: его нужно обновлять при смене состояния эмулятора или
/// нативных экранов (обычно раз в кадр или при обработке ввода).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    /// Где находится игрок — на станции или в космосе.
    pub location: Location,
    /// Какой экран отображается сейчас.
    pub screen: Screen,
    /// Открыто ли меню паузы (поверх текущего экрана).
    pub menu: bool,
    /// Идёт ли заставка (игра ещё не начата).
    pub intro: bool,
    /// Экран эмулятора (`WhatOnScreen`) на момент снимка.
    pub emu_screen: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            location: Location::Station,
            screen: Screen::Intro,
            menu: false,
            intro: true,
            emu_screen: 0,
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
            Screen::Emulator(emu_screen)
        };
        Self {
            location,
            screen,
            menu: false,
            intro,
            emu_screen,
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
