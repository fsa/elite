//! Нативный экран-заглушка `DOCKED`.
//!
//! Показывается примерно на секунду, когда на станции нажата `G` или `H`,
//! а поверх экрана игры открыт нативный экран. Оригинал в этот момент тоже
//! выводит `DOCKED` и издаёт звук, но под нативной отрисовкой его экран не
//! виден.

use std::time::{Duration, Instant};

use crate::display::Frame;
use crate::font::ZX_FONT_LEN;

/// Сколько показывать заглушку. Оригинал держит свой экран примерно столько
/// же.
pub const SHOW_TIME: Duration = Duration::from_millis(800);

/// Экран-заглушка `DOCKED`.
#[derive(Default)]
pub struct Docked {
    active: bool,
    until: Option<Instant>,
}

impl Docked {
    /// Новый выключенный экран.
    pub fn new() -> Self {
        Self::default()
    }

    /// Показать заглушку на [`SHOW_TIME`].
    ///
    /// Звук воспроизводится нативным компонентом [`crate::sfx`] (вызывающий
    /// код запускает эффект `DockedBeep`).
    pub fn show(&mut self) {
        self.active = true;
        self.until = Some(Instant::now() + SHOW_TIME);
    }

    /// Выключить заглушку.
    pub fn close(&mut self) {
        self.active = false;
        self.until = None;
    }

    /// Показана ли заглушка.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Закрыть заглушку, если время показа истекло.
    pub fn expire(&mut self, now: Instant) {
        if self.active && self.until.is_some_and(|until| now >= until) {
            self.close();
        }
    }

    /// Нарисовать заглушку во весь слой.
    pub fn draw(&self, hd: &mut Frame, font: &[u8; ZX_FONT_LEN]) {
        if self.active {
            draw::draw(hd, font);
        }
    }
}

mod draw {
    use crate::display::Frame;

    /// Фон — чёрный.
    const BG: u32 = 0xFF00_0000;
    /// Белая рамка и надпись — как на остальных нативных экранах.
    const WHITE: u32 = crate::cockpit::FRAME_COLOR;

    /// Строка надписи от верхнего края рамки (нумерация с нуля): седьмая
    /// строка сверху.
    const TEXT_ROW: i32 = 6;

    pub(super) fn draw(hd: &mut Frame, font: &[u8]) {
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

        draw_screen_frame(hd, margin, frame);

        let message = "DOCKED";
        let tw = message.chars().count() as i32 * 8 * text as i32;
        let tx = (w as i32 - tw) / 2;
        let ty = margin + frame + TEXT_ROW * line;
        hd.draw_text(font, tx, ty, message, WHITE, text);
    }

    /// Обвести экран белой рамкой, как на экранах статуса и информации.
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
        hd.fill_rect(m, m, fw as usize, t, WHITE);
        hd.fill_rect(m, m + fh - t as i32, fw as usize, t, WHITE);
        let side_h = (fh - 2 * t as i32).max(0) as usize;
        if side_h > 0 {
            hd.fill_rect(m, m + t as i32, t, side_h, WHITE);
            hd.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, WHITE);
        }
    }
}

#[cfg(test)]
mod tests;
