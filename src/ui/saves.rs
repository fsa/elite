//! Полноэкранные экраны сохранения и загрузки командира.
//!
//! Оформление повторяет нативные страницы игры: чёрный фон, тонкая рамка,
//! двухцветный заголовок, голубые подписи и подсветка выбранной строки.
//! Таблица показывается только из каталога приложения; Z80 в это время заморожен.
//!
//! Масштаб шрифта такой же, как на экране маркета: одна строка текста на
//! 300 пикселей высоты HD-слоя. Это даёт достаточно колонок, чтобы показать
//! `NAME`, `COMMENT` и полные дату с временем в одной строке таблицы.

use crate::display::Frame;
use crate::font::ZX_FONT;
use crate::storage::{LoadList, PromptField, SavePrompt};

const BG: u32 = 0xFF00_0000;
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
const LABEL: u32 = 0xFF00_D7D7;
const WHITE: u32 = 0xFFD7_D7D7;
const GREY: u32 = 0xFF80_8080;
const SELECT_BG: u32 = 0xFF00_5458;

const PROMPT_TITLE: &str = "COMMANDER SAVE";
const LIST_TITLE: &str = "COMMANDER LOAD";

/// Масштаб в пикселях на знакоместо — как в маркете (`h / 300`), но ещё и с
/// оглядкой на ширину: все три колонки таблицы должны помещаться в рамку.
fn cell(frame: &Frame) -> i32 {
    use crate::storage::{COMMENT_LEN, DATE_TIME_LEN, NAME_LEN};

    let by_height = (frame.height as f32 / 300.0).floor().max(1.0) as usize;
    let row_chars = NAME_LEN + 1 + COMMENT_LEN + 1 + DATE_TIME_LEN + 2;
    let by_width = (frame.width / (8 * row_chars)).max(1);
    8 * by_height.min(by_width) as i32
}

/// Сколько символов влезает в строку с отступом от рамки.
fn columns(frame: &Frame, c: i32) -> usize {
    ((frame.width as i32 / c) - 1).max(1) as usize
}

/// Толщина рамки кратна масштабу шрифта, но не меньше пикселя.
fn border(c: i32) -> i32 {
    (c / 8).max(1)
}

fn draw_base(frame: &mut Frame, c: i32, title: &str) -> i32 {
    frame.clear(BG);
    let b = border(c);
    let bu = b as usize;
    frame.fill_rect(0, 0, frame.width, bu, FRAME);
    frame.fill_rect(0, frame.height as i32 - b, frame.width, bu, FRAME);
    frame.fill_rect(0, 0, bu, frame.height, FRAME);
    frame.fill_rect(frame.width as i32 - b, 0, bu, frame.height, FRAME);

    let x = b + c;
    draw_two_tone(frame, x, b + c, title, FRAME, LABEL, c);
    x
}

/// Заголовок: верх глифов белый, низ голубой, как на других экранах.
fn draw_two_tone(frame: &mut Frame, x: i32, y: i32, text: &str, top: u32, bottom: u32, c: i32) {
    let scale = (c / 8).max(1) as usize;
    let w = (text.chars().count() * c as usize).max(1);
    let h = c as usize;
    let mid = h / 2;
    frame.set_clip(x, y + mid as i32, w, h - mid);
    frame.draw_text(ZX_FONT, x, y, text, bottom, scale);
    frame.clear_clip();
    frame.set_clip(x, y, w, mid);
    frame.draw_text(ZX_FONT, x, y, text, top, scale);
    frame.clear_clip();
}

fn row_y(top: i32, c: i32, row: usize) -> i32 {
    top + c * row as i32
}

/// Подсветка строки целиком — от левого поля до правой рамки.
fn highlight(frame: &mut Frame, x: i32, y: i32, c: i32) {
    frame.fill_rect(
        x - c / 2,
        y - 1,
        frame.width.saturating_sub((2 * x) as usize) + c as usize,
        c as usize,
        SELECT_BG,
    );
}

fn truncate(text: &str, max: usize) -> String {
    text.chars().take(max).collect()
}

/// Приглашение перед сохранением: имя можно подтвердить или дописать, затем
/// ввести комментарий и нажать Enter. Оба поля видны целиком на одной строке.
pub(crate) fn draw_save_prompt(frame: &mut Frame, prompt: &SavePrompt) {
    let c = cell(frame);
    let cols = columns(frame, c);
    let x = draw_base(frame, c, PROMPT_TITLE);
    let scale = (c / 8).max(1) as usize;
    let name_y = row_y(x, c, 2);

    let name_selected = prompt.field == PromptField::Name;
    let name = format!(
        "NAME: {}{}",
        prompt.name,
        if name_selected { "_" } else { "" }
    );
    if name_selected {
        highlight(frame, x, name_y, c);
    }
    frame.draw_text(
        ZX_FONT,
        x,
        name_y,
        &name,
        if name_selected { WHITE } else { LABEL },
        scale,
    );

    let comment_y = row_y(name_y, c, 2);
    let comment_selected = prompt.field == PromptField::Comment;
    let comment = format!(
        "COMMENT: {}{}",
        prompt.comment,
        if comment_selected { "_" } else { "" }
    );
    if comment_selected {
        highlight(frame, x, comment_y, c);
    }
    frame.draw_text(
        ZX_FONT,
        x,
        comment_y,
        &comment,
        if comment_selected { WHITE } else { FRAME },
        scale,
    );

    let help_y = row_y(comment_y, c, 2);
    frame.draw_text(
        ZX_FONT,
        x,
        help_y,
        "TAB: FIELD  ENTER: NEXT/SAVE",
        GREY,
        scale,
    );
    frame.draw_text(
        ZX_FONT,
        x,
        row_y(help_y, c, 1),
        "ESC: CANCEL  NAME: A-Z SPACE",
        GREY,
        scale,
    );
    frame.draw_text(
        ZX_FONT,
        x,
        row_y(help_y, c, 2),
        "COMMENT: ASCII 20-7E",
        GREY,
        scale,
    );
    if !prompt.status.is_empty() {
        frame.draw_text(
            ZX_FONT,
            x,
            row_y(help_y, c, 4),
            &truncate(&prompt.status, cols),
            LABEL,
            scale,
        );
    }
}

/// Подтверждение удаления поверх таблицы. Пока оно открыто, остальные
/// клавиши не меняют выбор и не запускают загрузку.
fn draw_delete_confirmation(frame: &mut Frame, list: &LoadList, c: i32, scale: usize) {
    let entry = list.entries.get(list.sel);
    let name = entry
        .map(|e| truncate(&e.name, crate::storage::NAME_LEN))
        .unwrap_or_default();
    let comment = entry
        .map(|e| truncate(&e.comment, crate::storage::COMMENT_LEN))
        .unwrap_or_default();
    let lines = [
        "DELETE SAVE?".to_string(),
        format!("NAME: {name}"),
        format!("COMMENT: {comment}"),
        "Y DELETE  N KEEP  ESC CANCEL".to_string(),
    ];
    let chars = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1) as i32;
    let w = (chars + 2) * c;
    let h = (lines.len() as i32 + 2) * c;
    let x = (frame.width as i32 - w).max(0) / 2;
    let y = (frame.height as i32 - h).max(0) / 2;
    let b = border(c);

    frame.fill_rect(x, y, w as usize, h as usize, BG);
    frame.fill_rect(x, y, w as usize, b as usize, FRAME);
    frame.fill_rect(x, y + h - b, w as usize, b as usize, FRAME);
    frame.fill_rect(x, y, b as usize, h as usize, FRAME);
    frame.fill_rect(x + w - b, y, b as usize, h as usize, FRAME);
    for (i, line) in lines.iter().enumerate() {
        let color = if i == 0 {
            WHITE
        } else if i + 1 == lines.len() {
            LABEL
        } else {
            FRAME
        };
        frame.draw_text(ZX_FONT, x + c, row_y(y + c, c, i), line, color, scale);
    }
}

/// Таблица сохранений. Строки приходят уже отсортированными: свежие сверху.
pub(crate) fn draw_load_list(frame: &mut Frame, list: &LoadList) {
    let c = cell(frame);
    let scale = (c / 8).max(1) as usize;
    let cols = columns(frame, c);
    let x = draw_base(frame, c, LIST_TITLE);
    let header_y = row_y(x, c, 2);
    let name_x = x;
    let comment_x = name_x + (crate::storage::NAME_LEN + 1) as i32 * c;
    let date_x = comment_x + (crate::storage::COMMENT_LEN + 1) as i32 * c;

    frame.draw_text(ZX_FONT, name_x, header_y, "NAME", LABEL, scale);
    frame.draw_text(ZX_FONT, comment_x, header_y, "COMMENT", LABEL, scale);
    frame.draw_text(ZX_FONT, date_x, header_y, "DATE & TIME", LABEL, scale);

    let first_row = row_y(header_y, c, 1);
    let total_rows = ((frame.height as i32 - 2 * border(c)).max(0) / c).max(1) as usize;
    let used_rows = (first_row / c).max(0) as usize + 3;
    let visible = total_rows
        .saturating_sub(used_rows)
        .max(1)
        .min(list.entries.len().max(1));
    let first = if list.sel >= visible {
        list.sel + 1 - visible
    } else {
        0
    };

    if list.entries.is_empty() {
        frame.draw_text(ZX_FONT, x, first_row, "NO SAVES", GREY, scale);
    }
    for (row, entry) in list.entries.iter().enumerate().skip(first).take(visible) {
        let y = row_y(first_row, c, row - first);
        let selected = row == list.sel;
        if selected {
            highlight(frame, x, y, c);
        }
        let color = if selected { WHITE } else { FRAME };
        frame.draw_text(ZX_FONT, name_x, y, &truncate(&entry.name, 10), color, scale);
        frame.draw_text(
            ZX_FONT,
            comment_x,
            y,
            &truncate(&entry.comment, crate::storage::COMMENT_LEN),
            color,
            scale,
        );
        frame.draw_text(
            ZX_FONT,
            date_x,
            y,
            &crate::storage::format_local_short(entry.saved_at),
            color,
            scale,
        );
    }

    let footer_y = frame.height as i32 - border(c) - 3 * c;
    frame.draw_text(
        ZX_FONT,
        x,
        footer_y,
        "ENTER LOAD  E EXPORT  I IMPORT",
        GREY,
        scale,
    );
    frame.draw_text(
        ZX_FONT,
        x,
        row_y(footer_y, c, 1),
        "DEL DELETE  ESC CLOSE",
        GREY,
        scale,
    );
    if !list.status.is_empty() {
        frame.draw_text(
            ZX_FONT,
            x,
            row_y(footer_y, c, 2),
            &truncate(&list.status, cols),
            LABEL,
            scale,
        );
    }
    if list.confirm_delete {
        draw_delete_confirmation(frame, list, c, scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Entry;

    #[test]
    fn delete_confirmation_is_drawn_over_table() {
        let mut frame = Frame::new(1280, 960);
        let list = LoadList {
            entries: vec![Entry {
                name: "LAVE".to_string(),
                comment: "Lave".to_string(),
                saved_at: 0,
                data: vec![0; crate::storage::COMMANDER_LEN],
            }],
            sel: 0,
            status: String::new(),
            confirm_delete: false,
        };
        draw_load_list(&mut frame, &list);
        let plain = frame.pixels().to_vec();

        let mut list = list;
        list.confirm_delete = true;
        draw_load_list(&mut frame, &list);
        assert_ne!(
            frame.pixels(),
            plain.as_slice(),
            "подтверждение должно менять кадр таблицы"
        );
    }
}
