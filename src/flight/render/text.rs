//! Текст поверх вида: центрирование строк по области рамки.

use crate::display::Frame;

/// Нарисовать строку по центру области `(x, y, w)`.
pub(super) fn draw_centered_text(
    frame: &mut Frame,
    font: &[u8],
    x: i32,
    y: i32,
    w: i32,
    text: &str,
    color: u32,
    scale: usize,
) {
    let text_w = text.len() as i32 * 8 * scale.max(1) as i32;
    let tx = x + (w - text_w) / 2;
    frame.draw_text(font, tx, y, text, color, scale);
}
