//! Наложение меню паузы в HD-слой.
//!
//! Оформление повторяет другие нативные экраны: чёрный фон, белая рамка,
//! голубой заголовок и подсветка выбранной строки, как на экране рынка.
//! На заставке пункт сохранения остаётся на месте, но рисуется серым и не
//! выделяется: игрового процесса ещё нет, поэтому он недоступен.

use crate::display;
use crate::ui::present::SRC_W;

/// Фон — чёрный, как на остальных нативных экранах.
const BG: u32 = 0xFF00_0000;
/// Белая рамка и обычный текст.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет заголовка и номеров пунктов.
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

/// Геометрия панели в знакоместах ZX, увеличенная до целого масштаба слоя.
struct Layout {
    rect: MenuRect,
    scale: usize,
    border: i32,
    cell: i32,
    line: i32,
    row_h: i32,
    title_y: i32,
    first_row_y: i32,
}

fn layout(width: usize, height: usize) -> Layout {
    let scale = (width / SRC_W).max(1);
    let s = scale as i32;
    let cell = 8 * s;
    let line = cell;
    let row_h = line + line / 2;
    // Толщина рамки и отступы согласованы с нативными экранами: рамка общая,
    // текст отступает от неё на одно знакоместо.
    let border = crate::cockpit::LINE.max(1) * s;
    let pad_x = cell;
    let pad_y = 2 * s;
    let text_w = TITLE.len() as i32 * cell;
    let w = (text_w + 2 * (pad_x + border)).max(0) as usize;
    let h = (2 * border + 2 * pad_y + line + line / 2 + ITEMS.len() as i32 * row_h).max(0) as usize;
    let x = (width as i32 - w as i32) / 2;
    let y = (height as i32 - h as i32) / 2;
    let top = y + border + pad_y;
    Layout {
        rect: MenuRect { x, y, w, h },
        scale,
        border,
        cell,
        line,
        row_h,
        title_y: top,
        first_row_y: top + line + line / 2,
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

    let tx = r.x + b + l.cell;
    frame.draw_text(font, tx, l.title_y, TITLE, LABEL, l.scale);

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
        frame.draw_text(font, tx, ry, number, number_color, l.scale);
        frame.draw_text(font, tx + 2 * l.cell, ry, label, label_color, l.scale);
    }
}
