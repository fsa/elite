//! Подписи коротких клеток и значок скорости.
//!
//! Подписи повторяют обозначения оригинала и печатаются шрифтом ПЗУ ZX на
//! чёрном фоне клетки. В последней левой строке вместо текста рисуется
//! растровый значок скорости.

use crate::display::Frame;

use super::geometry::{CockpitGeom, Rect};
use super::values::PANEL_ROWS;

/// Подписи левой колонки сверху вниз.
///
/// Последняя строка — не текст, а значок скорости ([`SPEED_ICON`]), поэтому
/// для неё указан `None`.
pub const LEFT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("FS"),
    Some("AS"),
    Some("FU"),
    Some("CT"),
    Some("LT"),
    Some("AL"),
    None,
];
/// Подписи правой колонки сверху вниз: `SP`, `RL`, `DC` и номера энергоблоков.
pub const RIGHT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("SP"),
    Some("RL"),
    Some("DC"),
    Some("1"),
    Some("2"),
    Some("3"),
    Some("4"),
];
/// Растр значка скорости из оригинальной панели: 12×5 пикселей.
///
/// Горизонтальная шкала со стрелкой влево; старший бит — левый пиксель.
pub(super) const SPEED_ICON: [u16; 5] = [
    0b011000000000,
    0b010111111111,
    0b011111111111,
    0b010111111111,
    0b011000000000,
];

/// Нарисовать подписи/значки всех коротких клеток.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, font: &[u8], line_color: u32) {
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            draw_indicator_label(f, g, row, right, line_color, font);
        }
    }
}

/// Подпись индикатора строки `row` (слева или справа).
///
/// `None` означает, что в клетке вместо текста рисуется значок скорости.
pub(super) fn indicator_label(row: i32, right: bool) -> Option<&'static str> {
    let table = if right { &RIGHT_LABELS } else { &LEFT_LABELS };
    table.get(row as usize).copied().flatten()
}

/// Нарисовать подпись индикатора (или значок скорости) по центру короткой клетки.
fn draw_indicator_label(
    f: &mut Frame,
    g: &CockpitGeom,
    row: i32,
    right: bool,
    line_color: u32,
    font: &[u8],
) {
    let area = g.label_cell(row, right);
    if area.is_empty() {
        return;
    }
    let Some(label) = indicator_label(row, right) else {
        draw_speed_icon(f, area, line_color);
        return;
    };
    if font.is_empty() {
        return;
    }
    let chars = label.len() as i32;
    let scale = (area.h / 8).min(area.w / (8 * chars).max(1)).max(1);
    let text_w = 8 * scale * chars;
    let text_h = 8 * scale;
    let x = area.x + (area.w - text_w) / 2;
    let y = area.y + (area.h - text_h) / 2;
    // На очень маленьких окнах подпись может не поместиться в клетку —
    // обрезаем её, чтобы не залезть на соседние строки и полосы.
    f.set_clip(
        area.x,
        area.y,
        area.w.max(0) as usize,
        area.h.max(0) as usize,
    );
    f.draw_text(font, x, y, label, line_color, scale as usize);
    f.clear_clip();
}

/// Нарисовать значок скорости по центру короткой клетки.
///
/// Масштаб тот же, что у подписей: один пиксель растра —
/// [`crate::cockpit::geometry::LINE`] и более пикселей окна, в зависимости от
/// высоты строки.
fn draw_speed_icon(f: &mut Frame, area: Rect, color: u32) {
    const W: i32 = 12;
    const H: i32 = SPEED_ICON.len() as i32;
    let scale = (area.h / 8).min(area.w / W).max(1);
    let w = W * scale;
    let h = H * scale;
    let x = area.x + (area.w - w) / 2;
    let y = area.y + (area.h - h) / 2;
    f.set_clip(
        area.x,
        area.y,
        area.w.max(0) as usize,
        area.h.max(0) as usize,
    );
    for (row, bits) in SPEED_ICON.iter().enumerate() {
        for col in 0..W {
            if bits & (1 << (W - 1 - col)) != 0 {
                f.fill_rect(
                    x + col * scale,
                    y + row as i32 * scale,
                    scale as usize,
                    scale as usize,
                    color,
                );
            }
        }
    }
    f.clear_clip();
}
