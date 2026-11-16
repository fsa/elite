//! Отрисовка экрана цен на станции.
//!
//! Оформление в стиле экранов информации о планете и командира: чёрный фон,
//! белая рамка, голубые подписи/заголовки колонок и белые значения. Заголовок
//! стилизован как в оригинале: верхняя часть букв белая, нижняя — голубая.

use super::*;

/// Фон экрана — чёрный.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет подписей.
const LABEL: u32 = 0xFF00_D7D7;
/// Значения — белые.
const VALUE: u32 = crate::cockpit::FRAME_COLOR;

/// Геометрия экрана.
struct Layout {
    margin: i32,
    frame: i32,
    text: usize,
    line: i32,
    title_y: i32,
    header_top_y: i32,
    header_bottom_y: i32,
    first_row_y: i32,
}

/// Сосчитать геометрию окна `w` x `h`.
fn layout(w: usize, h: usize) -> Layout {
    let margin = crate::cockpit::panel_geometry(w as u32, h as u32).margin;
    let frame = crate::cockpit::LINE.max(1);
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    let title_y = margin + frame + line;
    let header_top_y = title_y + 2 * line;
    let header_bottom_y = header_top_y + line;
    Layout {
        margin,
        frame,
        text,
        line,
        title_y,
        header_top_y,
        header_bottom_y,
        first_row_y: header_bottom_y + line,
    }
}

/// Нарисовать экран цен на станции.
pub(super) fn draw(hd: &mut Frame, font: &[u8; crate::font::ZX_FONT_LEN], info: &Info) {
    let w = hd.width;
    let h = hd.height;
    if w == 0 || h == 0 {
        return;
    }
    hd.clear(BG);
    let l = layout(w, h);
    let ts = l.text;

    let inner_x = l.margin + l.frame;
    let inner_w = (w as i32 - 2 * inner_x).max(0);
    let ch = 8 * ts as i32;

    // Заголовок — по центру, двухцветный: верх белый, низ голубой.
    let title = "MARKET PRICES";
    let tw = title.chars().count() as i32 * ch;
    let tx = inner_x + ((inner_w - tw) / 2).max(0);
    draw_two_tone_text(hd, font, tx, l.title_y, title, VALUE, LABEL, ts);

    // Колонки на всю ширину экрана. UNIT PRICE и QUANTITY FOR SALE — в две
    // строки; PRODUCT и UNIT опущены на вторую строку заголовка.
    let left = inner_x;
    let pad = ch / 2;
    let x_name = left + pad;
    let x_unit = left + inner_w * 45 / 100;
    let x_price = left + inner_w * 55 / 100;
    let x_qty = left + inner_w * 72 / 100;

    hd.draw_text(font, x_name, l.header_bottom_y, "PRODUCT", LABEL, ts);
    hd.draw_text(font, x_unit, l.header_bottom_y, "UNIT", LABEL, ts);
    hd.draw_text(font, x_price, l.header_top_y, "UNIT", LABEL, ts);
    hd.draw_text(font, x_price, l.header_bottom_y, "PRICE", LABEL, ts);
    hd.draw_text(font, x_qty, l.header_top_y, "QUANTITY", LABEL, ts);
    hd.draw_text(font, x_qty, l.header_bottom_y, "FOR SALE", LABEL, ts);

    let row_h = l.line + l.line / 4;
    let table_bottom = h as i32 - l.margin - l.frame;
    for (i, item) in info.items.iter().enumerate() {
        let y = l.first_row_y + i as i32 * row_h;
        if y + l.line > table_bottom {
            break;
        }
        hd.draw_text(font, x_name, y, item.name, VALUE, ts);
        hd.draw_text(font, x_unit, y, item.unit, VALUE, ts);

        let price = format!("{:>7.1}", item.price_tenths as f32 / 10.0);
        hd.draw_text(font, x_price, y, &price, VALUE, ts);

        let qty = if item.quantity == 0 {
            "-".to_string()
        } else {
            format!("{:>3}", item.quantity)
        };
        hd.draw_text(font, x_qty, y, &qty, VALUE, ts);
    }

    draw_screen_frame(hd, &l);
}

/// Нарисовать строку двухцветным шрифтом: верхняя половина глифов `top`,
/// нижняя — `bottom` (как в оригинальной игре).
fn draw_two_tone_text(
    hd: &mut Frame,
    font: &[u8; crate::font::ZX_FONT_LEN],
    x: i32,
    y: i32,
    s: &str,
    top: u32,
    bottom: u32,
    scale: usize,
) {
    let h = 8 * scale as i32;
    let mid = h / 2;
    let w = s.chars().count() as i32 * 8 * scale as i32;

    // Нижняя половина.
    hd.set_clip(x, y + mid, w.max(0) as usize, (h - mid).max(0) as usize);
    hd.draw_text(font, x, y, s, bottom, scale);
    hd.clear_clip();

    // Верхняя половина.
    hd.set_clip(x, y, w.max(0) as usize, mid.max(0) as usize);
    hd.draw_text(font, x, y, s, top, scale);
    hd.clear_clip();
}

/// Обвести экран белой рамкой.
fn draw_screen_frame(hd: &mut Frame, l: &Layout) {
    let w = hd.width as i32;
    let h = hd.height as i32;
    let m = l.margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = l.frame.clamp(1, fw.min(fh).max(1)) as usize;
    if fw <= 0 || fh <= 0 {
        return;
    }
    hd.fill_rect(m, m, fw as usize, t, FRAME);
    hd.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        hd.fill_rect(m, m + t as i32, t, side_h, FRAME);
        hd.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
}
