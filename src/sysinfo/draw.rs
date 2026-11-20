//! Отрисовка экрана информации о планете.
//!
//! Оформление в стиле нативных экранов (`status`, `market`): белая рамка,
//! двухцветный заголовок по центру, параметры строками с голубыми именами
//! и белыми значениями. Показываются данные текущей и выбранной на карте
//! систем.

use super::*;

/// Фон экрана — чёрный.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет заголовков секций и имён параметров.
const LABEL: u32 = 0xFF00_D7D7;
/// Значения — белые.
const VALUE: u32 = crate::cockpit::FRAME_COLOR;

/// Нарисовать экран информации о текущей и/или выбранной планете.
pub(super) fn draw(
    hd: &mut Frame,
    font: &[u8; crate::font::ZX_FONT_LEN],
    current: &Info,
    selected: &Info,
) {
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
    let row_h = line + line / 2;
    let ts = text;

    let inner_x = margin + frame;
    let inner_w = (w as i32 - 2 * inner_x).max(0);
    let title_y = margin + frame + line;
    let x = inner_x + 8 * ts as i32;

    if current.name == selected.name {
        // Как на исходном экране: только выбранная (она же текущая) система.
        let title = format!("DATA ON {}", selected.name.to_uppercase());
        let tw = title.chars().count() as i32 * 8 * ts as i32;
        let tx = inner_x + ((inner_w - tw) / 2).max(0);
        draw_two_tone_text(hd, font, tx, title_y, &title, VALUE, LABEL, ts);

        let mut y = title_y + 2 * line;
        draw_rows(hd, font, x, &mut y, row_h, ts, selected, false);
    } else {
        let mut y = title_y;

        let title = format!("DATA ON {}", selected.name.to_uppercase());
        let tw = title.chars().count() as i32 * 8 * ts as i32;
        let tx = inner_x + ((inner_w - tw) / 2).max(0);
        draw_two_tone_text(hd, font, tx, y, &title, VALUE, LABEL, ts);
        y += 2 * line;
        draw_rows(hd, font, x, &mut y, row_h, ts, selected, true);
        y += row_h / 2;

        let title = format!("PRESENT SYSTEM: {}", current.name.to_uppercase());
        let tw = title.chars().count() as i32 * 8 * ts as i32;
        let tx = inner_x + ((inner_w - tw) / 2).max(0);
        draw_two_tone_text(hd, font, tx, y, &title, VALUE, LABEL, ts);
        y += 2 * line;
        draw_rows(hd, font, x, &mut y, row_h, ts, current, false);
    }

    draw_screen_frame(hd, margin, frame);
}

/// Нарисовать параметры одной системы.
fn draw_rows(
    hd: &mut Frame,
    font: &[u8; crate::font::ZX_FONT_LEN],
    x: i32,
    y: &mut i32,
    row_h: i32,
    ts: usize,
    info: &Info,
    show_distance: bool,
) {
    let row = |hd: &mut Frame, y: i32, label: &str, value: &str| {
        hd.draw_text(font, x, y, label, LABEL, ts);
        let vx = x + label.chars().count() as i32 * 8 * ts as i32;
        hd.draw_text(font, vx, y, &format!(": {value}"), VALUE, ts);
    };

    if show_distance {
        let dist = if info.distance_tenths == 0 {
            "Current system".to_string()
        } else {
            format!("{} light years", format_distance(info.distance_tenths))
        };
        row(hd, *y, "Distance", &dist);
        *y += row_h;
    }

    row(hd, *y, "Economy", &info.economy);
    *y += row_h;
    row(hd, *y, "Government", &info.government);
    *y += row_h;
    row(hd, *y, "Tech. level", &info.tech_level.to_string());
    *y += row_h;
    row(
        hd,
        *y,
        "Population",
        &format!(
            "{} billion ({})",
            format_population(info.population_tenths),
            info.species
        ),
    );
    *y += row_h;
    row(
        hd,
        *y,
        "Gross productivity",
        &format!("{} M Cr", info.gross_product),
    );
    *y += row_h;
    row(hd, *y, "Average radius", &format!("{} km", info.radius_km));
    *y += row_h;
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
    hd.fill_rect(m, m, fw as usize, t, FRAME);
    hd.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        hd.fill_rect(m, m + t as i32, t, side_h, FRAME);
        hd.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
}
