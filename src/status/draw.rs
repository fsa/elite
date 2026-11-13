//! Отрисовка экрана статуса: рамка как у карты галактики, заголовок и строки.

use super::*;

/// Фон экрана — чёрный, как поле карты.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана и разделители — как линии кокпита.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Фон верхней ячейки с заголовком — красный, как клетки Cobra Mk III.
const HEADER_BG: u32 = crate::cockpit::BAR_RED;
/// Текст заголовка — белый.
const HEADER_TEXT: u32 = crate::cockpit::FRAME_COLOR;
/// Основной текст.
const TEXT: u32 = 0xFFE0_E8F0;
/// Подписи полей и подсказка внизу.
const TEXT_DIM: u32 = 0xFF80_9098;

/// Ширина колонки подписи в символах.
const LABEL_CHARS: i32 = 14;

/// Геометрия экрана, не зависящая от содержимого.
pub(super) struct Layout {
    pub(super) margin: i32,
    pub(super) frame: i32,
    pub(super) text: usize,
    pub(super) line: i32,
    pub(super) header_y: i32,
    pub(super) header_h: i32,
    /// Верх области со строками (ниже заголовка и разделителя).
    pub(super) content_y: i32,
    /// Верх нижней панели (над разделителем).
    pub(super) panel_y: i32,
    pub(super) title_y: i32,
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
    let header_h = 2 * line;
    let header_y = margin + frame;
    let content_y = header_y + header_h + frame;
    // Нижняя панель — такая же ячейка, как у карты: подсказка и отступы.
    let panel_h = 2 * line + 4 * text as i32;
    let panel_y = (h as i32 - margin - frame - panel_h).max(content_y);
    Layout {
        margin,
        frame,
        text,
        line,
        header_y,
        header_h,
        content_y,
        panel_y,
        title_y: header_y + (header_h - line) / 2,
    }
}

/// Нарисовать экран во весь слой.
pub(super) fn draw(status: &Status, info: &Info, f: &mut Frame, font: &[u8]) {
    let l = layout(f.width as u32, f.height as u32);
    let ts = l.text;
    f.clear(BG);

    let inner_x = l.margin + l.frame;
    let inner_w = (f.width as i32 - 2 * inner_x).max(0);

    // Верхняя ячейка: красная шапка во всю ширину, заголовок по центру.
    f.fill_rect(
        inner_x,
        l.header_y,
        inner_w as usize,
        l.header_h.max(0) as usize,
        HEADER_BG,
    );
    let title = "STATUS";
    let tw = title.len() as i32 * 8 * ts as i32;
    let tx = inner_x + ((inner_w - tw) / 2).max(0);
    f.draw_text(font, tx, l.title_y, title, HEADER_TEXT, ts);

    // Строки данных. Подписи приглушённые, значения — основным цветом.
    let x = inner_x + 8 * ts as i32;
    let value_x = x + LABEL_CHARS * 8 * ts as i32;
    let row_h = l.line + l.line / 2;
    let mut y = l.content_y + 2 * l.line;
    let row = |f: &mut Frame, y: i32, label: &str, value: &str| {
        f.draw_text(font, x, y, label, TEXT_DIM, ts);
        f.draw_text(font, value_x, y, value, TEXT, ts);
    };
    row(f, y, "COMMANDER", &info.name);
    y += row_h;
    row(f, y, "SYSTEM", &info.system);
    y += row_h;
    let galaxy = info.galaxy.to_string();
    row(f, y, "GALAXY", &galaxy);
    y += 2 * row_h;
    let cash = status.cash_text();
    row(f, y, "CASH", &cash);
    y += row_h;
    let fuel = status.fuel_text();
    row(f, y, "FUEL", &fuel);
    y += row_h;
    row(f, y, "LEGAL STATUS", status.legal_text());
    y += row_h;
    row(f, y, "RATING", status.rating_text());

    // Нижняя панель с подсказкой — ещё одна ячейка таблицы.
    let tx = inner_x + 8 * ts as i32;
    let ty = l.panel_y + (l.line - 8 * ts as i32) / 2 + ts as i32;
    f.draw_text(font, tx, ty, "F9/Esc back", TEXT_DIM, ts);

    // Рамка и разделители рисуются последними, чтобы не перекрывались
    // содержимым (как у карты галактики).
    draw_screen_frame(f, &l);
}

/// Обвести экран белой рамкой и отделить верхнюю и нижнюю ячейки.
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
    // Верхняя и нижняя линии рамки.
    f.fill_rect(m, m, fw as usize, t, FRAME);
    f.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    // Боковые линии между ними.
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        f.fill_rect(m, m + t as i32, t, side_h, FRAME);
        f.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
    // Разделитель под ячейкой заголовка.
    let y = l.header_y + l.header_h;
    f.fill_rect(m, y, fw as usize, t, FRAME);
    // Разделитель над нижней панелью.
    let py = l.panel_y - l.frame;
    f.fill_rect(m, py, fw as usize, t, FRAME);
}
