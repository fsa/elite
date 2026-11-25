//! Нативный экран инвентаря (клавиша `Enter`).
//!
//! Оформление как на других нативных страницах: рамка, центрированный
//! двухцветный заголовок; строки параметров — с большой буквы, как на странице
//! командира. Показываются только топливо и наличные из блока командира. Экран
//! ничего не меняет: данные читаются из живой памяти игры, а вводом управляет
//! слой приложения.

use crate::display::Frame;
use crate::world;

/// `PilotFuel` относительно начала блока командира (`PilotName`).
const OFF_FUEL: usize = 46;

/// Фон экрана — чёрный, как на других нативных информационных экранах.
const BG: u32 = 0xFF00_0000;
/// Белая рамка, верх заголовка и значения.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубые имена параметров и низ заголовка.
const LABEL: u32 = 0xFF00_D7D7;

/// Экран инвентаря.
#[derive(Default)]
pub struct Inventory {
    active: bool,
    fuel: u8,
    cash: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Открыть экран и снять поля из блока командира.
    pub fn open(&mut self, mem: &[u8], pilot: u16) {
        self.fuel = mem.get(pilot as usize + OFF_FUEL).copied().unwrap_or(0);
        self.cash = world::commander_cash_tenths(mem, pilot as usize);
        self.active = true;
    }

    /// Закрыть экран (только при переходе на другую страницу).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Показывается ли экран сейчас.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Прочитанное топливо в десятых светового года.
    #[cfg(test)]
    pub fn fuel(&self) -> u8 {
        self.fuel
    }

    /// Прочитанные деньги в десятых кредита.
    #[cfg(test)]
    pub fn cash_tenths(&self) -> u32 {
        self.cash
    }

    /// Нарисовать экран во весь HD-слой.
    pub fn draw(&self, f: &mut Frame, font: &[u8]) {
        if !self.active {
            return;
        }
        let w = f.width;
        let h = f.height;
        if w == 0 || h == 0 {
            return;
        }
        f.clear(BG);

        let text = (h as f32 / 300.0).floor().max(1.0) as usize;
        let cell = 8 * text as i32;
        let line = cell;
        let row_h = line + line / 2;
        let frame = crate::cockpit::LINE.max(1);
        let margin = crate::cockpit::panel_geometry(w as u32, h as u32).margin;

        let inner_x = margin + frame;
        let inner_w = (w as i32 - 2 * inner_x).max(0);
        let title_y = margin + frame + line;

        // Как на других нативных страницах: заголовок по центру и капсом,
        // верхняя половина белая, нижняя голубая.
        let title = "INVENTORY";
        let tw = title.len() as i32 * cell;
        let tx = inner_x + ((inner_w - tw) / 2).max(0);
        draw_two_tone_text(f, font, tx, title_y, title, FRAME, LABEL, text);

        // Имена параметров с большой буквы, двоеточия и значения белые.
        let x = inner_x + cell;
        let mut y = title_y + 2 * line;
        let fuel = format!("{}.{} Light Years", self.fuel / 10, self.fuel % 10);
        let cash = format!("{}.{} Cr", self.cash / 10, self.cash % 10);
        row(f, font, x, y, "Fuel", &fuel, text);
        y += row_h;
        row(f, font, x, y, "Cash", &cash, text);

        // Рамка рисуется последней, чтобы не перекрывалась содержимым.
        draw_screen_frame(f, margin, frame);
    }
}

/// Одна строка параметра: имя голубое, двоеточие и значение белые.
fn row(f: &mut Frame, font: &[u8], x: i32, y: i32, label: &str, value: &str, text: usize) {
    f.draw_text(font, x, y, label, LABEL, text);
    let vx = x + label.len() as i32 * 8 * text as i32;
    f.draw_text(font, vx, y, &format!(": {value}"), FRAME, text);
}

/// Строка заголовка: верхняя половина глифов `top`, нижняя — `bottom`.
fn draw_two_tone_text(
    f: &mut Frame,
    font: &[u8],
    x: i32,
    y: i32,
    s: &str,
    top: u32,
    bottom: u32,
    scale: usize,
) {
    let h = 8 * scale as i32;
    let mid = h / 2;
    let w = s.len() as i32 * 8 * scale as i32;

    // Нижняя половина.
    f.set_clip(x, y + mid, w.max(0) as usize, (h - mid).max(0) as usize);
    f.draw_text(font, x, y, s, bottom, scale);
    f.clear_clip();

    // Верхняя половина.
    f.set_clip(x, y, w.max(0) as usize, mid.max(0) as usize);
    f.draw_text(font, x, y, s, top, scale);
    f.clear_clip();
}

/// Обвести экран рамкой, как на экранах статуса и информации.
fn draw_screen_frame(f: &mut Frame, margin: i32, frame: i32) {
    let w = f.width as i32;
    let h = f.height as i32;
    let m = margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = frame.clamp(1, fw.min(fh).max(1)) as usize;
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

#[cfg(test)]
mod tests;
