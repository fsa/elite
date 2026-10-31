//! Отрисовка карты галактики: поле, планеты, курсор и панели.

use super::layout::*;
use super::*;

impl Map {
    /// Нарисовать карту во весь слой.
    pub fn draw(&mut self, f: &mut Frame, font: &[u8]) {
        self.win_w = f.width as u32;
        self.win_h = f.height as u32;
        let l = self.layout(self.win_w, self.win_h);
        f.clear(BG);

        let ts = l.text;
        let line = 8 * ts as i32;
        let near = self.nearest_index();
        let sys = self.system_index();

        // Видимая часть поля; сюда обрезаем всё рисование поля.
        let (vx0, vy0, vx1, vy1) = l.visible_rect();
        let vw = (vx1 - vx0).max(0) as usize;
        let vh = (vy1 - vy0).max(0) as usize;
        f.set_clip(vx0, vy0, vw, vh);
        // Планета видна, если её центр попадает в область показа. Позиция при
        // этом зажимается внутрь кадра: на минимальном зуме так остаются видны
        // все планеты, включая крайние (их кружок не налезает на рамку), а при
        // зуме за кадром оказываются только те, чей центр вне области показа.
        let margin = self.planet_margin(l.scale);
        let place =
            |x: i32, row: i32| -> Option<(i32, i32)> { self.place_planet(&l, x, row, margin) };

        // Поле галактики и его рамка.
        f.fill_rect(l.fx0, l.fy0, l.fw as usize, l.fh as usize, FIELD_BG);
        f.hline(l.fx0, l.fy0, l.fw as usize, BORDER);
        f.hline(l.fx0, l.fy0 + l.fh - 1, l.fw as usize, BORDER);
        f.vline(l.fx0, l.fy0, l.fh as usize, BORDER);
        f.vline(l.fx0 + l.fw - 1, l.fy0, l.fh as usize, BORDER);

        // Круг дальности прыжка вокруг текущей системы (как в карте I).
        let range = self.range_px(l.scale);
        if range > 0 {
            if let Some(i) = sys {
                let p = &self.planets[i];
                let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
                f.circle(sx, sy, range, RANGE);
            }
        }

        // Планеты: у каждой свой постоянный цвет; вне области показа — пропуск.
        for p in &self.planets {
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.disc(sx, sy, planet_px(p, l.scale), galaxy::color(p.state));
            }
        }
        // Кольцо ближайшей к курсору планеты.
        if let Some(i) = near {
            let p = &self.planets[i];
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.circle(sx, sy, planet_px(p, l.scale) + 3, NEAR_RING);
            }
        }
        // Кольцо текущей системы.
        if let Some(i) = sys {
            let p = &self.planets[i];
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.circle(sx, sy, planet_px(p, l.scale) + 1, SYS_RING);
            }
        }

        // Названия планет (как на детальной карте по O) — только при крупном зуме,
        // когда планет в кадре немного, иначе надписи сливаются.
        let mut visible_count: usize = 0;
        for p in &self.planets {
            if place(p.x, p.y() - FIELD_TOP).is_some() {
                visible_count += 1;
            }
        }
        if visible_count < 40 {
            for p in &self.planets {
                if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                    let name = self.name_of(p.index);
                    let px = planet_px(p, l.scale);
                    let tx = sx + px + 3 * ts as i32;
                    let ty = sy - 3 * ts as i32;
                    f.draw_text(font, tx, ty, &name, TEXT, ts);
                }
            }
        }

        // Курсор — перекрестие.
        let (cur_x, cur_y_raw) = self.cursor();
        let (cx, cy) = l.star_px(cur_x, cur_y_raw >> 1);
        let arm = ((3.0 * l.scale).round() as i32).clamp(3, 14);
        f.hline(cx - arm, cy, (2 * arm + 1) as usize, CURSOR);
        f.vline(cx, cy - arm, (2 * arm + 1) as usize, CURSOR);

        f.clear_clip();

        // Заголовок: как в оригинале — «GALACTIC CHART <номер галактики>».
        let title = format!("GALACTIC CHART {}", self.galaxy_number());
        let tw = title.len() as i32 * 8 * ts as i32;
        let tx0 = ((f.width as i32 - tw) / 2).max(8 * ts as i32);
        f.draw_text(font, tx0, l.title_y, &title, TEXT, ts);
        // Индикатор зума справа.
        let zs = format!("x{:.2}", self.zoom());
        let zx = (f.width as i32 - 8 * ts as i32 - zs.len() as i32 * 8 * ts as i32).max(0);
        f.draw_text(font, zx, l.title_y, &zs, TEXT_DIM, ts);

        // Панель внизу: текущая система и выбранная планета с расстоянием.
        let panel_h = f.height as i32 - l.panel_y;
        let py = l.panel_y;
        f.fill_rect(0, py, f.width, panel_h as usize, 0xFF00_0000);
        f.hline(0, py, f.width, BORDER);
        let tx = 8 * ts as i32;
        let mut ty = py + 4 * ts as i32;
        if let Some(i) = sys {
            let s = format!("SYSTEM   {:<8}", self.name_of(i));
            f.draw_text(font, tx, ty, &s, SYS_RING, ts);
        }
        ty += line + 2;
        if let Some(i) = near {
            let p = &self.planets[i];
            // Цветной маркер планеты рядом с именем.
            f.fill_rect(
                tx,
                ty,
                (6 * ts) as usize,
                line as usize,
                galaxy::color(p.state),
            );
            let s = format!(
                "SELECTED {:<8} {:>4.1} LY",
                self.name_of(i),
                self.distance_to_cursor()
            );
            f.draw_text(font, tx + 8 * ts as i32, ty, &s, NEAR_RING, ts);
        }
        ty += line + 2;
        f.draw_text(
            font,
            tx,
            ty,
            "F10/Esc back   mouse: point/pick   wheel or +/-: zoom",
            TEXT_DIM,
            ts,
        );
    }
}
