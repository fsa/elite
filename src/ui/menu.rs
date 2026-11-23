//! Наложение меню паузы в HD-слой.
//!
//! Оформление повторяет другие нативные экраны: чёрный фон, тонкая белая
//! рамка, двухцветный заголовок, голубые номера пунктов и подсветка выбранной
//! строки, как на экране рынка. На заставке пункт сохранения остаётся на
//! месте, но рисуется серым и не выделяется: игрового процесса ещё нет,
//! поэтому он недоступен.

use crate::display;

/// Фон — чёрный, как на остальных нативных экранах.
const BG: u32 = 0xFF00_0000;
/// Белая рамка, подсветка и обычный текст.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет нижней половины заголовка и номеров пунктов.
const LABEL: u32 = 0xFF00_D7D7;
/// Непрозрачный серый цвет недоступного пункта.
const DIM: u32 = 0xFF80_8080;
/// Подсветка выбранной строки, как в списке рынка.
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

/// Геометрия панели. Масштаб текста и толщина рамки берутся как на других
/// нативных страницах: текст зависит от высоты окна, рамка — от кокпита.
struct Layout {
    rect: MenuRect,
    text: usize,
    border: i32,
    cell: i32,
    line: i32,
    row_h: i32,
    title_y: i32,
    first_row_y: i32,
}

fn layout(width: usize, height: usize) -> Layout {
    let text = (height as f32 / 300.0).floor().max(1.0) as usize;
    let cell = 8 * text as i32;
    let line = cell;
    let row_h = line + line / 2;
    // Толщина внешней рамки экрана в пикселях, без умножения на масштаб шрифта.
    let border = crate::cockpit::LINE.max(1);
    // Отступ содержимого от рамки — не меньше одного знакоместа.
    let pad = cell.max(border + 1);
    let title_w = TITLE.len() as i32 * cell;
    let w = (title_w + 2 * (pad + border)).max(0) as usize;
    let h = (2 * border + 2 * pad + line + line / 2 + ITEMS.len() as i32 * row_h).max(0) as usize;
    let x = (width as i32 - w as i32) / 2;
    let y = (height as i32 - h as i32) / 2;
    let top = y + border + pad;
    Layout {
        rect: MenuRect { x, y, w, h },
        text,
        border,
        cell,
        line,
        row_h,
        title_y: top,
        first_row_y: top + line + line / 2,
    }
}

/// Прямоугольник панели для заданного размера HD-слоя. Используется только
/// тестами, поэтому в обычной сборке не включается.
#[cfg(test)]
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

    let tx = r.x + b + l.cell;
    draw_two_tone_text(frame, font, tx, l.title_y, TITLE, FRAME, LABEL, l.text);

    for (i, text) in ITEMS.iter().enumerate() {
        let ry = l.first_row_y + i as i32 * l.row_h;
        let dimmed = save_disabled && i == SAVE_ITEM;
        let selected = i == sel && !dimmed;
        if selected {
            let pad = (l.line / 4).max(1);
            frame.fill_rect(
                r.x + b,
                ry - pad,
                r.w.saturating_sub(2 * bw),
                (l.row_h + 2 * pad) as usize,
                SELECT_BG,
            );
        }
        let (number, label) = text.split_once("  ").unwrap_or((text, ""));
        let number_color = if dimmed {
            DIM
        } else if selected {
            FRAME
        } else {
            LABEL
        };
        let label_color = if dimmed { DIM } else { FRAME };
        frame.draw_text(font, tx, ry, number, number_color, l.text);
        frame.draw_text(font, tx + 2 * l.cell, ry, label, label_color, l.text);
    }
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
