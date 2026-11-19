//! Отрисовка экрана закупки, панели корабля и окна крепления.

use crate::cockpit::FRAME_COLOR;
use crate::display::Frame;

use super::Shop;
use super::geometry::Geo;
use super::items::{FUEL_MAX, ITEMS, Kind, LASER_NAMES, MISSILES_MAX, MOUNTS, laser_name};

// ————— Цвета HD-слоя (непрозрачные, `0xFFRRGGBB`) —————
// Палитра общая с экранами статуса, планеты и цен: чёрный фон, белая рамка,
// голубые подписи, белые значения. Выделение строк остаётся только как
// подсветка, а не как отдельная цветовая схема экрана.
pub(super) const BG: u32 = 0xFF00_0000;
pub(super) const FRAME: u32 = FRAME_COLOR;
pub(super) const TEXT: u32 = FRAME_COLOR;
pub(super) const LABEL: u32 = 0xFF00_D7D7;
pub(super) const BORDER: u32 = FRAME;
pub(super) const TEXT_DIM: u32 = LABEL;
pub(super) const HOVER_BG: u32 = 0xFF00_1A20;
pub(super) const CURSOR_BG: u32 = 0xFF00_5458;
pub(super) const FITTED_FG: u32 = LABEL;
pub(super) const PRICE_FG: u32 = FRAME_COLOR;
pub(super) const WARN_BG: u32 = 0xFF30_0C08;
pub(super) const WARN_FG: u32 = 0xFFFF_C0A0;

/// «327986.8» — десятые без десятичной мусорины.
fn fmt_tenths(t: u32) -> String {
    format!("{}.{}", t / 10, t % 10)
}

/// Имя системы в виде, как его печатает оригинал: первая буква заглавная,
/// остальные — строчные (`LAVE` -> `Lave`).
fn title_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => {
            first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
        }
        None => String::new(),
    }
}

/// Топливо в десятых долях светового года: `70` → «7.0» (как в игре,
/// где `Pilot_Fuel` хранит `0x46` = 7.0 св. года).
pub(super) fn fmt_fuel(tenths: u8) -> String {
    format!("{}.{}", tenths / 10, tenths % 10)
}

/// Разбить текст на строки не длиннее `cols` символов по словам.
pub(super) fn wrap(text: &str, cols: usize) -> Vec<String> {
    let cols = cols.max(1);
    let mut out = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > cols {
            out.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        out.push(line);
    }
    if out.is_empty() {
        out.push(String::new());
    }
    out
}

impl Shop {
    /// Нарисовать экран во весь слой. `mem` — живая память игры: из неё
    /// берутся цены (в том числе динамическая цена топлива).
    pub fn draw(&mut self, f: &mut Frame, font: &[u8], mem: &[u8]) {
        self.win_w = f.width as u32;
        self.win_h = f.height as u32;
        if self.toast_frames > 0 {
            self.toast_frames -= 1;
        } else {
            self.toast = None;
        }
        let g = self.geo();
        let ts = g.ts;
        let ch = g.ch;
        let line = g.line;
        let x0 = g.x0;
        f.clear(BG);

        // ——— Шапка: заголовок и строки данных командира ———
        // Как на экранах рынка и информации: верх глифов белый, низ — голубой.
        let title = "EQUIP SHIP";
        let tw = title.len() as i32 * 8 * ts as i32;
        let tx = ((f.width as i32 - tw) / 2).max(0);
        draw_two_tone_text(f, font, tx, g.title_y, title, TEXT, LABEL, ts);

        // Информация: название параметра голубым, двоеточие и значение белым.
        // Между параметрами — два пробела.
        let mut x = x0;
        let draw = |f: &mut Frame, x: &mut i32, s: &str, color: u32| {
            f.draw_text(font, *x, g.info_y, s, color, ts);
            *x += s.chars().count() as i32 * ch;
        };
        draw(f, &mut x, "System", LABEL);
        draw(f, &mut x, &format!(": {}", title_case(&self.system)), TEXT);
        draw(f, &mut x, "  ", TEXT);
        draw(f, &mut x, "Tech. level", LABEL);
        draw(f, &mut x, &format!(": {}", self.tech), TEXT);
        draw(f, &mut x, "  ", TEXT);
        draw(f, &mut x, "Cash", LABEL);
        draw(f, &mut x, &format!(": {} Cr", fmt_tenths(self.cash)), TEXT);

        // ——— Список товаров ———
        let content_h = (g.content_bottom - g.list_y).max(1) as usize;
        f.set_clip(x0, g.list_y, g.list_w, content_h);
        let hi_x = x0;
        let hi_w = g.list_w;
        let sel_pad = 2 * ts as i32;
        let sel_h = (line + 2 * sel_pad).max(1) as usize;
        for (i, &number) in self.rows.iter().enumerate() {
            let ry = g.list_y + i as i32 * g.row_h;
            let sel = i == self.cursor;
            let hov = self.hover == Some(i);
            let ty = ry + (g.row_h - line) / 2;
            if sel {
                f.fill_rect(hi_x, ty - sel_pad, hi_w, sel_h, CURSOR_BG);
            } else if hov {
                f.fill_rect(hi_x, ty - sel_pad, hi_w, sel_h, HOVER_BG);
            }
            let item = &ITEMS[(number - 1) as usize];
            // «Установлено» есть только у флаговых товаров: у лазеров свой
            // слот крепления, а байт «флага» номера лазера — чужое поле
            // командира (в оригинале лазеры «PRESENT» не проверяют).
            let fitted = matches!(item.kind, Kind::Flag | Kind::Bay)
                && self.fitted[number as usize];
            let num_s = format!("{number:>2}");
            f.draw_text(font, g.x_num, ty, &num_s, TEXT_DIM, ts);
            let name_color = if fitted { FITTED_FG } else { TEXT };
            f.draw_text(font, g.x_name, ty, item.name, name_color, ts);
            // На месте цены у установленного товара — «Fitted» (иначе
            // название и цена налезали бы друг на друга на узком списке).
            // Иначе цена показывается «как в оригинале» (со сдвигом бага Elita).
            let (price_s, price_color) = if fitted {
                ("Fitted".to_string(), FITTED_FG)
            } else {
                (
                    format!("{:>w$}", fmt_tenths(self.price_shown(mem, number)), w = g.price_w as usize),
                    PRICE_FG,
                )
            };
            let px = g.x_price - price_s.len() as i32 * ch;
            f.draw_text(font, px, ty, &price_s, price_color, ts);
        }
        f.clear_clip();

        // ——— Панель корабля справа ———
        if g.panel_w > 0 {
            let ph = (g.content_bottom - g.list_y).max(1) as usize;
            let px = g.panel_x + 2 * ch;
            // Ширина панели в символах (за вычетом рамки и боковых отступов).
            let pch = (g.panel_w as i32 / ch - 4).max(1) as usize;
            let step = line + 2 * ts as i32;
            let mut texts: Vec<(String, u32)> = vec![
                ("Your ship".to_string(), TEXT),
                (
                    format!("{:<9}{}/{} LY", "Fuel", fmt_fuel(self.fuel), fmt_fuel(FUEL_MAX)),
                    TEXT_DIM,
                ),
                (
                    format!("{:<9}{}/{}", "Missiles", self.missiles, MISSILES_MAX),
                    TEXT_DIM,
                ),
                (
                    format!("{:<9}{} T", "Cargo", self.free_cargo),
                    TEXT_DIM,
                ),
                ("Lasers".to_string(), TEXT),
            ];
            for (i, m) in MOUNTS.iter().enumerate() {
                texts.push((
                    format!("{:<7}{}", m, laser_name(self.lasers[i])),
                    TEXT_DIM,
                ));
            }
            // Описание выбранного/наведённого товара; длинные подсказки
            // переносятся по словам под ширину панели.
            if let Some(i) = self.hover.or(Some(self.cursor)) {
                if let Some(&number) = self.rows.get(i) {
                    let item = &ITEMS[(number - 1) as usize];
                    texts.push((String::new(), TEXT_DIM));
                    texts.push((item.name.to_string(), TEXT));
                    texts.push((
                        format!("{} CR", fmt_tenths(self.price(mem, number))),
                        PRICE_FG,
                    ));
                    for l in wrap(item.desc, pch) {
                        texts.push((l, TEXT_DIM));
                    }
                }
            }
            f.set_clip(g.panel_x, g.list_y, g.panel_w, ph);
            let mut py = g.list_y + 2 * ts as i32;
            for (s, c) in &texts {
                f.draw_text(font, px, py, s, *c, ts);
                py += step;
            }
            f.clear_clip();
        }

        // ——— Окно выбора крепления лазера ———
        if let Some(number) = self.mount {
            self.draw_mount_modal(f, font, number);
        }

        // ——— Баннер «в полёте» ———
        if self.in_flight {
            let msg = "Dock at a station to buy equipment";
            let scale = ts + 1;
            let mw = msg.len() as i32 * 8 * scale as i32 + 6 * ch;
            let mh = 8 * scale as i32 + 6 * ts as i32;
            let mx = ((f.width as i32 - mw) / 2).max(0);
            let my = (g.list_y + g.content_bottom) / 2 - mh / 2;
            f.fill_rect(mx, my, mw as usize, mh as usize, WARN_BG);
            f.hline(mx, my, mw as usize, WARN_FG);
            f.hline(mx, my + mh - 1, mw as usize, WARN_FG);
            f.draw_text(
                font,
                mx + 3 * ch,
                my + 3 * ts as i32,
                msg,
                WARN_FG,
                scale,
            );
        }

        // ——— Тост ———
        if let Some(msg) = &self.toast {
            let mw = msg.len() as i32 * ch + 6 * ch;
            let mx = ((f.width as i32 - mw) / 2).max(0);
            let my = g.help_y - 2 * line - 2 * ts as i32;
            f.fill_rect(mx, my, mw as usize, (line + 4 * ts as i32) as usize, WARN_BG);
            f.draw_text(font, mx + 3 * ch, my + 2 * ts as i32, msg, WARN_FG, ts);
        }

        // ——— Строка подсказок внизу (как на экране рынка) ———
        self.draw_status_line(f, font, &g);

        // Рамка рисуется последней, как на остальных нативных экранах,
        // чтобы содержимое не перекрывало её линии.
        draw_screen_frame(f, &g);
    }

    /// Нижняя строка подсказок: голубые названия, белые двоеточия и значения,
    /// между полями — два пробела (как на экране рынка).
    fn draw_status_line(&self, f: &mut Frame, font: &[u8], g: &Geo) {
        let fields = [("Select", "Up/Down"), ("Buy", "Enter/Click")];
        let ch = g.ch;
        let mut x = g.x0;
        for (i, (name, value)) in fields.iter().enumerate() {
            if i > 0 {
                x += 2 * ch;
            }
            f.draw_text(font, x, g.help_y, name, LABEL, g.ts);
            let vx = x + name.chars().count() as i32 * ch;
            let tail = format!(": {value}");
            f.draw_text(font, vx, g.help_y, &tail, TEXT, g.ts);
            x = vx + tail.chars().count() as i32 * ch;
        }
    }

    /// Окно выбора крепления (как в оригинале: `0 FRONT .. 3 RIGHT`).
    fn draw_mount_modal(&self, f: &mut Frame, font: &[u8], number: u8) {
        let g = self.geo();
        let ts = g.ts;
        let ch = g.ch;
        let line = g.line;
        let Kind::Laser(laser) = ITEMS[(number - 1) as usize].kind else {
            return;
        };
        let title = format!("Mount {} laser where?", LASER_NAMES[laser as usize]);
        let mw = (44 * ch).max(0) as usize;
        let opt_h = (line + 4 * ts as i32).max(1);
        let mh = (2 * line + 4 * opt_h + line + 4 * ts as i32) as usize;
        let mx = ((f.width as i32 - mw as i32) / 2).max(0);
        let my = ((g.top + g.content_bottom - mh as i32) / 2).max(0);
        f.fill_rect(mx, my, mw, mh, 0xFF00_0000);
        f.hline(mx, my, mw, BORDER);
        f.hline(mx, my + mh as i32 - 1, mw, BORDER);
        f.vline(mx, my, mh, BORDER);
        f.vline(mx + mw as i32 - 1, my, mh, BORDER);
        f.draw_text(
            font,
            mx + 2 * ch,
            my + line / 2 + 2 * ts as i32,
            &title,
            TEXT,
            ts,
        );
        for i in 0..4u8 {
            let r = self.mount_rect(i);
            if self.mount_hover == Some(i) {
                f.fill_rect(r.0, r.1, r.2, r.3, CURSOR_BG);
            }
            let cur = laser_name(self.lasers[i as usize]);
            let s = format!("{}  {:<6} {}", i, MOUNTS[i as usize], cur);
            let ty = r.1 + (r.3 as i32 - line) / 2;
            f.draw_text(font, r.0 + ch, ty, &s, TEXT, ts);
        }
        let hy = my + mh as i32 - line - 2 * ts as i32;
        f.draw_text(
            font,
            mx + 2 * ch,
            hy,
            "0-3 select   Esc cancel",
            TEXT_DIM,
            ts,
        );
    }
}

/// Заголовок из двух цветов: верхняя половина глифов `top`, нижняя — `bottom`
/// (как на экране цен и в оригинальной игре).
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
    let w = s.chars().count() as i32 * 8 * scale as i32;

    f.set_clip(x, y + mid, w.max(0) as usize, (h - mid).max(0) as usize);
    f.draw_text(font, x, y, s, bottom, scale);
    f.clear_clip();

    f.set_clip(x, y, w.max(0) as usize, mid.max(0) as usize);
    f.draw_text(font, x, y, s, top, scale);
    f.clear_clip();
}

/// Обвести экран белой рамкой. Отступ и толщина — как у других нативных
/// экранов, поэтому при переключении F4 рамка не «прыгает».
fn draw_screen_frame(f: &mut Frame, g: &Geo) {
    let w = f.width as i32;
    let h = f.height as i32;
    let m = g.margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = g.frame.clamp(1, fw.min(fh).max(1)) as usize;
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
