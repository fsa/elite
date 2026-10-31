//! Рамки кокпита: главная (вокруг вида) и нижняя (вокруг панели).

use crate::display::Frame;

use super::geometry::{CockpitGeom, LINE, Rect};
use super::palette::CockpitStyle;

/// Обвести главную рамку и рамку нижней части.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle) {
    stroke_rect(f, g.main_frame(), LINE, style.line);
    stroke_rect(f, g.panel_frame(), LINE, style.line);
}

/// Обвести прямоугольник `r` рамкой толщиной `t`.
pub(super) fn stroke_rect(f: &mut Frame, r: Rect, t: i32, color: u32) {
    if r.is_empty() {
        return;
    }
    let t = t.min(r.w).min(r.h).max(1);
    // Верхняя и нижняя полосы.
    f.fill_rect(r.x, r.y, r.w as usize, t as usize, color);
    f.fill_rect(r.x, r.y + r.h - t, r.w as usize, t as usize, color);
    // Боковые полосы между ними.
    let mid_h = (r.h - 2 * t).max(0) as usize;
    if mid_h > 0 {
        f.fill_rect(r.x, r.y + t, t as usize, mid_h, color);
        f.fill_rect(r.x + r.w - t, r.y + t, t as usize, mid_h, color);
    }
}
