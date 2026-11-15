//! Индикатор опасности — залитый круг цвета `IndDanger`.

use crate::display::Frame;

use super::geometry::{CockpitGeom, Rect};
use super::values::CockpitValues;

/// Радиус кружка опасности в клетке `cell`.
///
/// В оригинале это 2x2 знакоместа с кругом радиусом около 7 пикселей ZX.
pub(super) fn radius(cell: Rect) -> i32 {
    let d = cell.w.min(cell.h);
    (d / 2 - (d / 16).max(1)).max(0)
}

/// Нарисовать индикатор опасности: залитый круг цвета `IndDanger`.
///
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
    f.disc(cell.x + cell.w / 2, cell.y + cell.h / 2, radius(cell), color);
}

/// Нарисовать нижнюю половину кружка опасности на верхней линии рамки
/// вспомогательного экрана — как `PrintIndDanger` на картах и экране
/// капитана в оригинале.
///
/// Центр кружка лежит на верхней линии рамки справа (в оригинале — в трёх
/// знакоместах от правого края), поэтому видна только нижняя половина:
/// всё, что выше рамки, отсечено. Цвет и диаметр — как у индикатора
/// опасности на радаре, поэтому кружок показывает текущий уровень опасности.
pub(super) fn draw_screen_half(f: &mut Frame, mem: &[u8], margin: i32, frame: i32) {
    let Some(color) = super::layout::read_danger(mem).color(true) else {
        return;
    };
    let cell = super::geometry::panel_geometry(f.width as u32, f.height as u32).danger_cell();
    let r = radius(cell);
    if r < 1 {
        return;
    }
    let m = margin.max(0);
    let fw = (f.width as i32 - 2 * m).max(0);
    // Левая граница правой линии рамки. В оригинале центр кружка стоит на
    // 23 пикселя левее её при радиусе 7 — сохраняем это соотношение.
    let right = m + fw - frame.max(0);
    let cx = right - (r * 23 / 7);
    let cy = m;
    for dy in 0..=r {
        let half = ((r * r - dy * dy) as f64).sqrt() as i32;
        f.fill_rect(cx - half, cy + dy, (2 * half + 1) as usize, 1, color);
    }
}
