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
            crate::ui::message::draw(hd, font, "DOCKED");
        }
    }
}

#[cfg(test)]
mod tests;
