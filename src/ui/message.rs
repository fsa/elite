//! Общая полноэкранная заглушка с сообщением.
//!
//! Используется нативным экраном `DOCKED` и временной страницей клавиш режима
//! паузы: одинаковый чёрный фон, белая рамка и центровка текста по образцу
//! остальных нативных экранов станции.

use crate::display::Frame;

/// Фон — чёрный.
pub const BG: u32 = 0xFF00_0000;
/// Белая рамка и надпись — как на остальных нативных экранах.
pub const FG: u32 = crate::cockpit::FRAME_COLOR;

/// Строка надписи от верхнего края рамки (нумерация с нуля): седьмая
/// строка сверху.
const TEXT_ROW: i32 = 6;

/// Нарисовать сообщение во весь HD-слой.
pub fn draw(hd: &mut Frame, font: &[u8], message: &str) {
    let w = hd.width;
    let h = hd.height;
    if w == 0 || h == 0 {
        return;
    }
    hd.clear(BG);

    let margin = crate::cockpit::panel_geometry(w as u32, h as u32).margin;
    let frame = crate::cockpit::LINE.max(1);
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;

    draw_screen_frame(hd, margin, frame);

    let tw = message.chars().count() as i32 * 8 * text as i32;
    let tx = (w as i32 - tw) / 2;
    let ty = margin + frame + TEXT_ROW * line;
    hd.draw_text(font, tx, ty, message, FG, text);
}

/// Обвести экран белой рамкой, как на экранах статуса и информации.
fn draw_screen_frame(hd: &mut Frame, margin: i32, frame: i32) {
    let w = hd.width as i32;
    let h = hd.height as i32;
    let m = margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = frame.clamp(1, fw.min(fh).max(1)) as usize;
    if fw <= 0 || fh <= 0 {
        return;
    }
    hd.fill_rect(m, m, fw as usize, t, FG);
    hd.fill_rect(m, m + fh - t as i32, fw as usize, t, FG);
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        hd.fill_rect(m, m + t as i32, t, side_h, FG);
        hd.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FG);
    }
}
