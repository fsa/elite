//! Сообщения поверх игрового кадра.
//!
//! `draw` — полноэкранная заглушка нативных экранов `DOCKED` и `LOADING`;
//! `draw_banner` — маленькая надпись по центру уже нарисованного кадра
//! (пауза Кладова).

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

/// Нарисовать небольшую непрозрачную табличку с сообщением строго по центру
/// кадра. Используется для паузы: замороженный экран игры остаётся виден.
pub fn draw_banner(hd: &mut Frame, font: &[u8], message: &str) {
    let w = hd.width as i32;
    let h = hd.height as i32;
    if w <= 0 || h <= 0 || message.is_empty() {
        return;
    }
    let scale = ((h / 120).max(2) as usize).min(12);
    let text_w = message.chars().count() as i32 * 8 * scale as i32;
    let text_h = 8 * scale as i32;
    let frame = scale as i32;
    let pad = scale as i32;
    let box_w = text_w + 2 * (frame + pad);
    let box_h = text_h + 2 * (frame + pad);
    let x = (w - box_w) / 2;
    let y = (h - box_h) / 2;
    hd.fill_rect(x, y, box_w as usize, box_h as usize, BG);
    hd.fill_rect(x, y, box_w as usize, frame as usize, FG);
    hd.fill_rect(x, y + box_h - frame, box_w as usize, frame as usize, FG);
    hd.fill_rect(x, y + frame, frame as usize, (box_h - 2 * frame) as usize, FG);
    hd.fill_rect(
        x + box_w - frame,
        y + frame,
        frame as usize,
        (box_h - 2 * frame) as usize,
        FG,
    );
    hd.draw_text(
        font,
        x + frame + pad,
        y + frame + pad,
        message,
        FG,
        scale,
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::ZX_FONT;

    /// Паузная табличка рисуется по центру и не затирает остальной кадр.
    #[test]
    fn banner_is_centered_and_leaves_surroundings_untouched() {
        let (w, h) = (200usize, 120usize);
        let unchanged = 0xFF12_3456;
        let mut hd = Frame::new(w, h);
        hd.clear(unchanged);

        draw_banner(&mut hd, ZX_FONT, "PAUSE");

        let scale = 2;
        let text_w = 5 * 8 * scale;
        let box_w = text_w + 4 * scale;
        let box_h = 8 * scale + 4 * scale;
        let x = (w as i32 - box_w as i32) / 2;
        let y = (h as i32 - box_h as i32) / 2;
        let at = |px: usize, py: usize| hd.pixels()[py * w + px];

        assert_eq!(at(x as usize, y as usize), FG, "нет верхней рамки");
        assert_eq!(
            at((x + box_w - 1) as usize, (y + box_h - 1) as usize),
            FG,
            "нет нижней рамки"
        );
        assert_eq!(at(0, 0), unchanged, "затерт угол кадра");
        assert_eq!(
            at((x + box_w) as usize, (y + box_h / 2) as usize),
            unchanged,
            "табличка вышла за свои границы"
        );

        let text_start = (
            (x + 2 * scale as i32) as usize,
            (y + 2 * scale as i32) as usize,
        );
        let text_end = (
            (x + box_w as i32 - 2 * scale as i32) as usize,
            (y + box_h as i32 - 2 * scale as i32) as usize,
        );
        let has_glyph = (text_start.1..text_end.1)
            .any(|py| (text_start.0..text_end.0).any(|px| at(px, py) == FG));
        assert!(has_glyph, "внутри таблички нет надписи PAUSE");
    }
}
