//! Отрисовка экрана информации о планете.
//!
//! Оформление в стиле нативного экрана командира (`status`): белая рамка,
//! заголовок по центру, параметры строками с голубыми именами и белыми
//! значениями. Заголовок стилизован как на экране цен: верхняя часть букв
//! белая, нижняя — голубая.

use super::*;

/// Фон экрана — чёрный.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет заголовка и имён параметров.
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
    first_row_y: i32,
}

/// Сосчитать геометрию окна `w` x `h`.
fn layout(w: usize, h: usize) -> Layout {
    let margin = crate::cockpit::panel_geometry(w as u32, h as u32).margin;
    let frame = crate::cockpit::LINE.max(1);
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    let title_y = margin + frame + line;
    Layout {
        margin,
        frame,
        text,
        line,
        title_y,
        first_row_y: title_y + 2 * line,
    }
}

/// Нарисовать экран информации о планете.
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

    // Заголовок — по центру, двухцветный: верх белый, низ голубой.
    let title = format!("DATA ON {}", info.name.to_uppercase());
    let tw = title.chars().count() as i32 * 8 * ts as i32;
    let tx = inner_x + ((inner_w - tw) / 2).max(0);
    draw_two_tone_text(hd, font, tx, l.title_y, &title, VALUE, LABEL, ts);

    let x = inner_x + 8 * ts as i32;
    let row_h = l.line + l.line / 2;
    let mut y = l.first_row_y;

    let row = |hd: &mut Frame, y: i32, label: &str, value: &str| {
        hd.draw_text(font, x, y, label, LABEL, ts);
        let vx = x + label.chars().count() as i32 * 8 * ts as i32;
        hd.draw_text(font, vx, y, &format!(": {value}"), VALUE, ts);
    };

    if info.distance_tenths > 0 {
        row(
            hd,
            y,
            "Distance",
            &format!("{} light years", format_distance(info.distance_tenths)),
        );
        y += row_h;
    }

    row(hd, y, "Economy", &info.economy);
    y += row_h;
    row(hd, y, "Government", &info.government);
    y += row_h;
    row(hd, y, "Tech. level", &info.tech_level.to_string());
    y += row_h;
    row(
        hd,
        y,
        "Population",
        &format!(
            "{} billion ({})",
            format_population(info.population_tenths),
            info.species
        ),
    );
    y += row_h;
    row(
        hd,
        y,
        "Gross productivity",
        &format!("{} M Cr", info.gross_product),
    );
    y += row_h;
    row(
        hd,
        y,
        "Average radius",
        &format!("{} km", info.radius_km),
    );

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
