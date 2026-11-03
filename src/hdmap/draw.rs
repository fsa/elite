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

        // Поле галактики на чёрном фоне, без собственной рамки: единственная
        // рамка — белая рамка экрана.

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

        // Верхняя ячейка таблицы: красная шапка во всю ширину, отделённая
        // белой рамкой. Заголовок — как в оригинале, «GALACTIC CHART <n>».
        let inner_x = l.margin + l.frame;
        let inner_w = (f.width as i32 - 2 * inner_x).max(0);
        f.fill_rect(
            inner_x,
            l.header_y,
            inner_w as usize,
            l.header_h.max(0) as usize,
            HEADER_BG,
        );
        let title = format!("GALACTIC CHART {}", self.galaxy_number());
        let tw = title.len() as i32 * 8 * ts as i32;
        let tx0 = inner_x + ((inner_w - tw) / 2).max(0);
        f.draw_text(font, tx0, l.title_y, &title, HEADER_TEXT, ts);

        // Панель внизу — ячейка таблицы с чёрным фоном: сверху её отделяет
        // белый разделитель, остальные края — внешняя рамка экрана.
        let panel_bottom = (f.height as i32 - l.margin - l.frame).max(l.panel_y);
        let panel_h = panel_bottom - l.panel_y;
        let py = l.panel_y;
        f.fill_rect(inner_x, py, inner_w as usize, panel_h.max(0) as usize, BG);
        let tx = inner_x + 8 * ts as i32;
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
        let help = format!(
            "F10/Esc back   LMB: pick   D: nearest   wheel or +/-: zoom x{:.2}",
            self.zoom()
        );
        f.draw_text(font, tx, ty, &help, TEXT_DIM, ts);

        // Белая рамка вокруг всего экрана и разделитель под шапкой. Рисуется
        // последней, чтобы линии не перекрывались содержимым.
        draw_screen_frame(f, &l);
    }
}

/// Обвести экран белой рамкой и отделить верхнюю ячейку с заголовком.
///
/// Толщина линий и отступ от краёв взяты с кокпита, чтобы при переключении
/// между экранами внешняя рамка не меняла положение и толщину.
fn draw_screen_frame(f: &mut Frame, l: &MapLayout) {
    let w = f.width as i32;
    let h = f.height as i32;
    let m = l.margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = l.frame.clamp(1, fw.min(fh).max(1)) as usize;
    if fw <= 0 || fh <= 0 {
        return;
    }
    // Верхняя и нижняя линии рамки.
    f.fill_rect(m, m, fw as usize, t, FRAME);
    f.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    // Боковые линии между ними.
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        f.fill_rect(m, m + t as i32, t, side_h, FRAME);
        f.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
    // Нижняя окантовка ячейки заголовка — общая горизонтальная линия.
    let y = l.header_y + l.header_h;
    f.fill_rect(m, y, fw as usize, t, FRAME);
    // Верхняя окантовка нижней ячейки: панель тоже читается как ячейка
    // таблицы, нижние и боковые края ей даёт внешняя рамка.
    let py = l.panel_y - l.frame;
    f.fill_rect(m, py, fw as usize, t, FRAME);
}
