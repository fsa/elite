//! Индикатор опасности — залитый круг цвета `IndDanger`.

use crate::display::Frame;

use super::geometry::CockpitGeom;
use super::values::CockpitValues;

/// Нарисовать индикатор опасности: залитый круг цвета `IndDanger`.
///
/// В оригинале это 2x2 знакоместа с кругом радиусом около 7 пикселей ZX.
/// Мигающий красный (`0x63`) в выключенной фазе не рисуется — фон панели
/// уже залит чёрным.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, values: &CockpitValues) {
    let cell = g.danger_cell();
    if cell.is_empty() {
        return;
    }
    let Some(color) = values.danger.color(values.blink) else {
        return;
    };
    let d = cell.w.min(cell.h);
    let radius = (d / 2 - (d / 16).max(1)).max(0);
    f.disc(cell.x + cell.w / 2, cell.y + cell.h / 2, radius, color);
}
