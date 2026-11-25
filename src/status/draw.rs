//! Отрисовка экрана статуса: рамка, строка командира по центру, поля и
//! список установленного оборудования.

use super::*;

/// Фон экрана — чёрный.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет заголовка и имён параметров.
const LABEL: u32 = 0xFF00_D7D7;
/// Двоеточия и значения — белые.
const VALUE: u32 = crate::cockpit::FRAME_COLOR;
/// Отступ списка оборудования от колонки заголовков (в символах):
/// в оригинале `EQUIPMENT:` печатается с колонки 1, предметы — с колонки 6.
const EQUIP_INDENT: i32 = 5;

/// Геометрия экрана, не зависящая от содержимого.
pub(super) struct Layout {
    pub(super) margin: i32,
    pub(super) frame: i32,
    pub(super) text: usize,
    pub(super) line: i32,
    /// Строка с именем командира (по центру экрана).
    pub(super) title_y: i32,
    /// Строка первого параметра.
    pub(super) first_row_y: i32,
}

/// Сосчитать геометрию окна `w` x `h`.
///
/// Толщина линий и отступ от краёв берутся с кокпита и карты галактики:
/// при переключении между экранами внешняя рамка не смещается.
pub(super) fn layout(w: u32, h: u32) -> Layout {
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    let frame = crate::cockpit::LINE.max(1);
    let margin = crate::cockpit::panel_geometry(w, h).margin;
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

/// Нарисовать экран во весь слой.
pub(super) fn draw(
    status: &Status,
    info: &Info,
    missions: &MissionInfo,
    f: &mut Frame,
    font: &[u8],
) {
    let l = layout(f.width as u32, f.height as u32);
    let ts = l.text;
    f.clear(BG);

    let inner_x = l.margin + l.frame;
    let inner_w = (f.width as i32 - 2 * inner_x).max(0);

    // Имя командира — единственная строка по центру экрана.
    let title = format!("COMMANDER {}", info.name);
    let tw = title.len() as i32 * 8 * ts as i32;
    let tx = inner_x + ((inner_w - tw) / 2).max(0);
    f.draw_text(font, tx, l.title_y, &title, LABEL, ts);

    // Поля: имя параметра голубое, двоеточие и значение — белые.
    let x = inner_x + 8 * ts as i32;
    let row_h = l.line + l.line / 2;
    let mut y = l.first_row_y;
    let row = |f: &mut Frame, y: i32, label: &str, value: &str| {
        f.draw_text(font, x, y, label, LABEL, ts);
        let vx = x + label.len() as i32 * 8 * ts as i32;
        f.draw_text(font, vx, y, &format!(": {value}"), VALUE, ts);
    };
    row(f, y, "Present System", &info.system);
    y += row_h;
    row(f, y, "Hyperspace System", &info.target);
    y += row_h;
    row(f, y, "Fuel", &status.fuel_text());
    y += row_h;
    row(f, y, "Cash", &status.cash_text());
    y += row_h;
    row(f, y, "Legal Status", status.legal_text());
    y += row_h;
    row(f, y, "Rating", status.rating_text());

    // Состояние миссий — продолжение снимка, пришедшего из `GameState`.
    y += row_h;
    for (label, value) in status.mission_rows(missions) {
        row(f, y, label, &value);
        y += row_h;
    }

    // Блок оборудования: пустая строка, белый заголовок и белый список —
    // как в `Cmd_L_PilotInfo` (сразу после рейтинга).
    let ix = x + EQUIP_INDENT * 8 * ts as i32;
    y += 2 * row_h;
    f.draw_text(font, x, y, "EQUIPMENT:", FRAME, ts);
    for item in &info.equipment {
        y += row_h;
        f.draw_text(font, ix, y, item, FRAME, ts);
    }

    // Рамка рисуется последней, чтобы не перекрывалась содержимым.
    draw_screen_frame(f, &l);
}

/// Обвести экран белой рамкой.
fn draw_screen_frame(f: &mut Frame, l: &Layout) {
    let w = f.width as i32;
    let h = f.height as i32;
    let m = l.margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = l.frame.clamp(1, fw.min(fh).max(1)) as usize;
    if fw <= 0 || fh <= 0 {
        return;
    }
    f.fill_rect(m, m, fw as usize, t, FRAME);
    f.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        f.fill_rect(m, m + t as i32, t, side_h, FRAME);
        f.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
}
