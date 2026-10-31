//! Палитра кокпита и цвета в зависимости от типа корабля.
//!
//! Здесь только цвета: сами индикаторы и геометрия о них не знают и получают
//! готовый [`CockpitStyle`] от вызывающего кода.

/// Цвет линий кокпита — «белый» ZX (ink 7), как в оригинальном растре.
pub const FRAME_COLOR: u32 = 0xFFD7_D7D7;
/// Красный индикатор опасности и цвет рамки при попадании.
pub const DANGER_COLOR: u32 = 0xFFFF_4040;
/// Зелёный индикатор опасности (`IndDanger = 0x04`).
pub const DANGER_GREEN: u32 = 0xFF00_D700;
/// Жёлтый индикатор опасности (`IndDanger = 0x06`).
pub const DANGER_YELLOW: u32 = 0xFFD7_D700;
/// Цвет фона кокпита (paper 0).
pub const BACKGROUND: u32 = 0xFF00_0000;
/// Цвет длинных клеток у Cobra Mk III — красный (ZX paper 2).
pub const BAR_RED: u32 = 0xFFD7_0000;
/// Цвет длинных клеток у Cobra Mk IV — фиолетовый (ZX paper 3).
pub const BAR_MAGENTA: u32 = 0xFFD7_00D7;

/// Цвета кокпита: линии/индикаторы и фон длинных клеток.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CockpitStyle {
    pub line: u32,
    pub bar: u32,
}

impl CockpitStyle {
    /// Cobra Mk III и заставка: белые линии, красные полосы.
    pub const COBRA_MK3: CockpitStyle = CockpitStyle {
        line: FRAME_COLOR,
        bar: BAR_RED,
    };
    /// Cobra Mk IV: белые линии, фиолетовые полосы.
    ///
    /// Пока используется только в тестах: боевой код выбирает цвет полос
    /// отдельно (`Flight::cockpit_bar_color`).
    #[allow(dead_code)]
    pub const COBRA_MK4: CockpitStyle = CockpitStyle {
        line: FRAME_COLOR,
        bar: BAR_MAGENTA,
    };
}

impl Default for CockpitStyle {
    fn default() -> Self {
        Self::COBRA_MK3
    }
}
