//! Наложение меню паузы в HD-слой.
//!
//! Панель построена как таблица: заголовок отделён линией, пункты разбиты на
//! колонки номера и действия, между строками есть разделители. Цвета, масштаб
//! шрифта и подсветка выбранной строки взяты с нативных страниц (рынка).
//! На заставке пункт сохранения остаётся на месте, но рисуется серым и не
//! выделяется: игрового процесса ещё нет, поэтому он недоступен.

use crate::display;

/// Фон — чёрный, как на остальных нативных экранах.
const BG: u32 = 0xFF00_0000;
/// Белая рамка, подсветка и обычный текст.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет нижней половины заголовка.
const LABEL: u32 = 0xFF00_D7D7;
/// Непрозрачный серый цвет недоступного пункта.
const DIM: u32 = 0xFF80_8080;
/// Более тёмный серый для линий таблицы.
const GRID: u32 = 0xFF60_6060;
/// Подсветка выбранной строки, как на экране рынка.
const SELECT_BG: u32 = 0xFF00_5458;

const TITLE: &str = "ELITE - PAUSED";
/// Индекс пункта «SAVE» в списке; на заставке он недоступен.
const SAVE_ITEM: usize = 1;
const ITEMS: [&str; 4] = ["1  RESUME", "2  SAVE", "3  LOAD", "4  QUIT GAME"];

/// Прямоугольник панели в пикселях HD-слоя. Нужен тестам.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MenuRect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) w: usize,
    pub(crate) h: usize,
}

/// Геометрия таблицы. Масштаб текста берётся как на других нативных
/// страницах, толщина рамки — как у внешней рамки экрана.
struct Layout {
    rect: MenuRect,
    text: usize,
    border: i32,
    pad_x: i32,
    cell: i32,
    line: i32,
    row_h: i32,
    gap: i32,
    inner_x: i32,
    inner_w: usize,
    title_y: i32,
    header_sep_y: i32,
    first_row_y: i32,
}

fn layout(width: usize, height: usize) -> Layout {
    let text = (height as f32 / 300.0).floor().max(1.0) as usize;
    let cell = 8 * text as i32;
    let line = cell;
    let row_h = 2 * line;
    let gap = cell / 2;
    // Толщина внешней рамки экрана в пикселях, без умножения на масштаб шрифта.
    let border = crate::cockpit::LINE.max(1);
    // Свободное поле вокруг текста: два знакоместа по бокам и полтора сверху
    // и снизу, чтобы таблица не выглядела сжатой.
    let pad_x = 2 * cell;
    let pad_y = cell + cell / 2;
    let title_w = TITLE.len() as i32 * cell;
    let w = (title_w + 2 * (pad_x + border)).max(0) as usize;
    let h = (2 * border
        + 2 * pad_y
        + line
        + 2 * gap
        + 1
        + ITEMS.len() as i32 * row_h)
        .max(0) as usize;
    let x = (width as i32 - w as i32) / 2;
    let y = (height as i32 - h as i32) / 2;
    let title_y = y + border + pad_y;
    let header_sep_y = title_y + line + gap;
    Layout {
        rect: MenuRect { x, y, w, h },
        text,
        border,
        pad_x,
        cell,
        line,
        row_h,
        gap,
        inner_x: x + border,
        inner_w: w.saturating_sub(2 * border.max(1) as usize),
        title_y,
        header_sep_y,
        first_row_y: header_sep_y + 1 + gap,
    }
}

/// Прямоугольник панели для заданного размера HD-слоя.
pub(crate) fn menu_rect(width: usize, height: usize) -> MenuRect {
    layout(width, height).rect
}

/// Нарисовать панель меню поверх замороженного кадра. `sel` — индекс
/// выбранного пункта, `save_disabled` — признак заставки: пункт «SAVE»
/// рисуется серым и никогда не выделяется.
pub(crate) fn draw_pause_menu(
    frame: &mut display::Frame,
    font: &[u8],
    sel: usize,
    save_disabled: bool,
) {
    let l = layout(frame.width, frame.height);
    let r = l.rect;
    let b = l.border.max(1);
    let bw = b as usize;

    frame.fill_rect(r.x, r.y, r.w, r.h, BG);
    frame.fill_rect(r.x, r.y, r.w, bw, FRAME);
    frame.fill_rect(r.x, r.y + r.h as i32 - b, r.w, bw, FRAME);
    frame.fill_rect(r.x, r.y, bw, r.h, FRAME);
    frame.fill_rect(r.x + r.w as i32 - b, r.y, bw, r.h, FRAME);

    let tx = l.inner_x + l.pad_x;
    let label_x = tx + 3 * l.cell;
    let col_sep_x = tx + 2 * l.cell;

    for (i, text) in ITEMS.iter().enumerate() {
        let ry = l.first_row_y + i as i32 * l.row_h;
        let dimmed = save_disabled && i == SAVE_ITEM;
        if i == sel && !dimmed {
            // Как в маркете: сплошная полоса на всю внутреннюю ширину,
            // выступающая на две строки пикселей за пределы глифов.
            let pad = 2 * l.text as i32;
            frame.fill_rect(
                l.inner_x,
                ry - pad,
                l.inner_w,
                (l.line + 2 * pad) as usize,
                SELECT_BG,
            );
        }
        let (number, label) = text.split_once("  ").unwrap_or((text, ""));
        let color = if dimmed { DIM } else { FRAME };
        frame.draw_text(font, tx, ry, number, color, l.text);
        frame.draw_text(font, label_x, ry, label, color, l.text);
    }

    // Линии таблицы поверх подсветки: шапка, разделители строк и колонок.
    frame.hline(l.inner_x, l.header_sep_y, l.inner_w, FRAME);
    for i in 1..ITEMS.len() {
        let y = l.first_row_y + i as i32 * l.row_h - l.gap / 2;
        frame.hline(l.inner_x, y, l.inner_w, GRID);
    }
    let col_top = l.first_row_y - l.gap / 2;
    let col_h = ITEMS.len() as i32 * l.row_h - l.gap / 2 + l.gap / 2;
    frame.vline(col_sep_x, col_top, col_h.max(1) as usize, GRID);

    draw_two_tone_text(frame, font, tx, l.title_y, TITLE, FRAME, LABEL, l.text);
}

/// Нарисовать строку двумя цветами: верх глифов белый, низ — голубой, как
/// в заголовках нативных страниц.
fn draw_two_tone_text(
    frame: &mut display::Frame,
    font: &[u8],
    x: i32,
    y: i32,
    text: &str,
    top: u32,
    bottom: u32,
    scale: usize,
) {
    let w = (text.chars().count() as i32 * 8 * scale as i32).max(0) as usize;
    let h = 8 * scale as i32;
    let mid = h / 2;

    frame.set_clip(x, y + mid, w, (h - mid).max(0) as usize);
    frame.draw_text(font, x, y, text, bottom, scale);
    frame.clear_clip();

    frame.set_clip(x, y, w, mid.max(0) as usize);
    frame.draw_text(font, x, y, text, top, scale);
    frame.clear_clip();
}
