//! Статическая таблица панели: горизонтальные и вертикальные разделители.

use crate::display::Frame;

use super::geometry::{CockpitGeom, LINE};
use super::palette::CockpitStyle;
use super::values::PANEL_ROWS;

/// Нарисовать сетку таблицы внутри панели.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle) {
    let inner = g.panel_inner();
    if inner.is_empty() {
        return;
    }
    // Горизонтальные разделители только в боковых колонках: центральная
    // клетка одна на все семь строк.
    let center = g.center_cell();
    for row in 0..PANEL_ROWS - 1 {
        let sep = g.row_separator(row);
        let left_w = (center.x - inner.x).max(0);
        if left_w > 0 {
            f.fill_rect(inner.x, sep.y, left_w as usize, sep.h as usize, style.line);
        }
        let right_x = center.x + center.w;
        let right_w = (inner.x + inner.w - right_x).max(0);
        if right_w > 0 {
            f.fill_rect(right_x, sep.y, right_w as usize, sep.h as usize, style.line);
        }
    }
    // Вертикальные разделители между всеми клетками таблицы.
    for x in g.column_separators() {
        f.fill_rect(x, inner.y, LINE as usize, inner.h as usize, style.line);
    }
}
