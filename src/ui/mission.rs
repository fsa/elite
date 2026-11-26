//! Нативный диалог `Y/N` поверх экрана миссии original.
//!
//! Пока диалог открыт, Z80 стоит в цикле ожидания `sub_E1F7`: свой экран он
//! уже нарисовал, а символ ответа ещё не получил. Натив рисует только рамку и
//! вопрос, не подменяя игровую логику.

use crate::display::Frame;
use crate::font::ZX_FONT;

const BG: u32 = 0xFF00_0000;
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
const LABEL: u32 = 0xFF00_D7D7;
const WHITE: u32 = 0xFFD7_D7D7;
const GREY: u32 = 0xFF80_8080;
const SELECT_BG: u32 = 0xFF00_5458;

/// Нарисовать вопрос миссии с подсветкой выбранного ответа.
pub(crate) fn draw_prompt(frame: &mut Frame, accept: bool, marines: bool) {
    let c = ((frame.height as f32 / 300.0).floor().max(1.0) as usize * 8).max(8) as i32;
    let scale = (c / 8).max(1) as usize;
    let b = (c / 8).max(1) as i32;
    frame.clear(BG);
    frame.fill_rect(0, 0, frame.width, b as usize, FRAME);
    frame.fill_rect(0, (frame.height.saturating_sub(b as usize)) as i32, frame.width, b as usize, FRAME);
    frame.fill_rect(0, 0, b as usize, frame.height, FRAME);
    frame.fill_rect((frame.width.saturating_sub(b as usize)) as i32, 0, b as usize, frame.height, FRAME);

    let x = b + c;
    let mut y = b + c;
    draw_two_tone(frame, x, y, "MISSION", WHITE, LABEL, c);
    y += c * 3;

    let question = if marines {
        "TAKE THE MARINES ON BOARD?"
    } else {
        "RESCUE THE REFUGEES?"
    };
    frame.draw_text(ZX_FONT, x, y, question, WHITE, scale);
    y += c * 2;

    draw_answer(frame, x, y, "Y - ACCEPT", accept, c);
    draw_answer(frame, x, y + c * 2, "N - REFUSE", !accept, c);

    let hint = "Y/N OR ARROWS, ENTER";
    frame.draw_text(
        ZX_FONT,
        x,
        frame.height.saturating_sub((b + c) as usize) as i32,
        hint,
        GREY,
        scale,
    );
}

fn draw_answer(frame: &mut Frame, x: i32, y: i32, text: &str, selected: bool, c: i32) {
    if selected {
        frame.fill_rect(
            x - c / 2,
            y - 1,
            frame.width.saturating_sub((2 * x) as usize) + c as usize,
            c as usize,
            SELECT_BG,
        );
    }
    frame.draw_text(ZX_FONT, x, y, text, WHITE, (c / 8).max(1) as usize);
}

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
