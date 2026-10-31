//! Геометрия окна: раскладка списка, панели и модального окна.

use super::Shop;
use super::items::{LIST_CHARS, NAME_CHARS, PANEL_MIN_CHARS, PRICE_CHARS};

/// Геометрия окна для отрисовки и попадания мышью (считается из размера
/// окна и числа строк списка).
#[derive(Clone, Copy)]
pub(super) struct Geo {
    /// Масштаб шрифта.
    pub(super) ts: usize,
    /// Ширина символа 8x8, увеличенного в `ts` раз.
    pub(super) ch: i32,
    /// Высота строки текста (= `ch`).
    pub(super) line: i32,
    /// Левое поле содержимого.
    pub(super) x0: i32,
    /// Низ верхней полосы (шапка).
    pub(super) top: i32,
    /// Верх строк списка.
    pub(super) list_y: i32,
    /// Высота строки списка.
    pub(super) row_h: i32,
    /// Ширина панели списка.
    pub(super) list_w: usize,
    pub(super) x_num: i32,
    pub(super) x_name: i32,
    /// Правая граница колонки цены (цена/«FITTED» выравниваются вправо).
    pub(super) x_price: i32,
    /// Ширина колонки цены в символах.
    pub(super) price_w: i32,
    /// Панель корабля справа.
    pub(super) panel_x: i32,
    pub(super) panel_w: usize,
    /// Строка помощи внизу.
    pub(super) help_y: i32,
    /// Низ области содержимого.
    pub(super) content_bottom: i32,
}

/// Прямоугольник `(x, y, w, h)` — для заливки и проверки попадания.
pub(super) type Rect = (i32, i32, usize, usize);

fn inside(r: Rect, x: f64, y: f64) -> bool {
    x >= r.0 as f64
        && x < (r.0 + r.2 as i32) as f64
        && y >= r.1 as f64
        && y < (r.1 + r.3 as i32) as f64
}

impl Shop {
    /// Навести указатель: строка списка или вариант крепления.
    pub fn point_at(&mut self, win_x: f64, win_y: f64) {
        self.hover = None;
        self.mount_hover = None;
        if self.mount.is_some() {
            for i in 0..4u8 {
                if inside(self.mount_rect(i), win_x, win_y) {
                    self.mount_hover = Some(i);
                }
            }
            return;
        }
        let g = self.geo();
        if win_x < g.x0 as f64 || win_x >= (g.x0 + g.list_w as i32) as f64 {
            return;
        }
        let i = ((win_y - g.list_y as f64) / g.row_h as f64).floor();
        if i >= 0.0 && i < self.rows.len() as f64 {
            let row = i as usize;
            if g.list_y + (row as i32 + 1) * g.row_h <= g.content_bottom {
                self.hover = Some(row);
            }
        }
    }

    pub(super) fn geo(&self) -> Geo {
        // Масштаб подбирается и по высоте (строки), и по ширине: список
        // занимает ровно свои колонки, всё остальное — панель корабля.
        let ts_h = (self.win_h as i32 / 300).max(1);
        let ts_w = (self.win_w as i32 / ((LIST_CHARS + PANEL_MIN_CHARS + 6) * 8)).max(1);
        let ts = ts_h.min(ts_w) as usize;
        let ch = 8 * ts as i32;
        let line = ch;
        let x0 = 2 * ch;
        // Шапка: заголовок + строка SYSTEM/TECH/CASH.
        let title_ts = ts + 1;
        let top = 4 * ts as i32 + 8 * title_ts as i32 + line + 8 * ts as i32;
        let help_y = self.win_h as i32 - line - 6 * ts as i32;
        let content_bottom = help_y - 4 * ts as i32;
        // Слева — только колонки списка, остальная ширина уходит панели.
        let list_w = (LIST_CHARS * ch).max(0) as usize;
        let panel_x = x0 + list_w as i32 + 2 * ch;
        let panel_w = (self.win_w as i32 - panel_x - x0).max(0) as usize;
        let list_y = top + 2 * ts as i32;
        let row_h = line * 5 / 2;
        Geo {
            ts,
            ch,
            line,
            x0,
            top,
            list_y,
            row_h,
            list_w,
            x_num: x0 + 1 * ch,
            x_name: x0 + 4 * ch,
            x_price: x0 + (4 + NAME_CHARS + 1 + PRICE_CHARS) * ch,
            price_w: PRICE_CHARS,
            panel_x,
            panel_w,
            help_y,
            content_bottom,
        }
    }

    /// Прямоугольник варианта крепления `i` в окне модалки.
    pub(super) fn mount_rect(&self, i: u8) -> Rect {
        let g = self.geo();
        let mw = (44 * g.ch).max(0) as usize;
        let opt_h = (g.line + 4 * g.ts as i32).max(1);
        let mh = 2 * g.line + 4 * opt_h + g.line + 4 * g.ts as i32;
        let mx = ((self.win_w as i32 - mw as i32) / 2).max(0);
        let my = ((g.top + g.content_bottom - mh as i32) / 2).max(0);
        (mx + 2 * g.ch, my + 2 * g.line + i as i32 * opt_h, mw - 4 * g.ch as usize, opt_h as usize)
    }
}
