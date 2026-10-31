//! Компоновка кадра заставки: надписи, вращающийся корабль и запасная фигура.

use crate::cockpit::{CockpitGeom, Rect};
use crate::display::Frame;
use crate::render3d::{self, INTRO_DIST, INTRO_SHIP_RADIUS, SHIP_COLOR, Screen as Proj};
use crate::shipdata::Ship;

use super::Splash;

/// Рёбра запасного куба.
const CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

impl Splash {
    /// Нарисовать заставку на слой `frame` во всё окно.
    ///
    /// `font` — знакогенератор ПЗУ ZX (коды 0x20..0x7F), как в меню паузы.
    /// Используется тестами; в игре — [`Splash::draw_in`] с областью вида
    /// кокпита.
    #[allow(dead_code)] // используется в тестах; в игре — `draw_in`
    pub fn draw(&self, frame: &mut Frame, font: &[u8]) {
        let view = Rect {
            x: 0,
            y: 0,
            w: frame.width as i32,
            h: frame.height as i32,
        };
        self.draw_area(frame, font, frame.height as i32, view);
    }

    /// Нарисовать заставку внутри области вида кокпита `geom`.
    ///
    /// Композиция повторяет оригинальный титульный экран Elite:
    ///
    /// * заголовок `---- E L I T E ----` — одной текстовой строкой ниже
    ///   верхней рамки (в оригинале он стоит в строке 1 сетки 32x24), по
    ///   центру области вида;
    /// * в середине — вращающийся корабль (на самом широком ракурсе
    ///   занимает всю свободную полосу между надписями; в остальных
    ///   ракурсах он естественно сокращается, как в оригинале);
    /// * внизу — приглашение `Press Space, Commander` — тоже по центру и
    ///   на одну текстовую строку выше нижней рамки (в оригинале строка 14
    ///   прижата к панели кокпита).
    ///
    /// Отступы от рамки заданы в текстовых строках, поэтому их отношение к
    /// высоте области примерно такое же, как у оригинала (8 из 192 пикселей).
    /// Никаких других надписей нет — ровно как в оригинале.
    ///
    /// Фон закрашивается до `geom.panel_y`: панель рисуется поверх и сама
    /// закрывает нижнюю часть, а поля вокруг главной рамки остаются чёрными,
    /// как и весь титульный экран.
    pub fn draw_in(&self, frame: &mut Frame, font: &[u8], geom: &CockpitGeom) {
        self.draw_area(frame, font, geom.panel_y.max(1), geom.view());
    }

    /// Общая раскладка заставки: `area_h` задаёт масштаб сетки ZX, а `view` —
    /// прямоугольник внутри рамки кокпита, относительно которого ставятся
    /// надписи и корабль.
    fn draw_area(&self, frame: &mut Frame, font: &[u8], area_h: i32, view: Rect) {
        const BG: u32 = 0xFF00_0000; // чёрный фон (непрозрачный)
        const FG: u32 = 0xFFD7_D7D7; // белый — заголовок
        const HINT: u32 = 0xFFD7_D7D7; // белый — приглашение
        let fw = frame.width as i32;
        let fh = frame.height as i32;
        let area_h = area_h.clamp(1, fh);
        // Фон области (снизу может быть своя панель — её не трогаем).
        frame.fill_rect(0, 0, frame.width, area_h as usize, BG);
        // Область вида может быть чуть больше слоя (например, в тестах) —
        // для раскладки используем её пересечение с кадром.
        let x0 = view.x.clamp(0, fw);
        let y0 = view.y.clamp(0, fh);
        let x1 = (view.x + view.w).clamp(x0, fw);
        let y1 = (view.y + view.h).clamp(y0, fh);
        let view = Rect {
            x: x0,
            y: y0,
            w: x1 - x0,
            h: y1 - y0,
        };
        // Масштаб сетки ZX 256x192, влезающий в область: им задаётся и размер
        // шрифта, и отступы от рамки (сетка оригинала — 32x24 знакоместа).
        let s = ((frame.width / 256).min(area_h as usize / 192)).max(1) as i32;
        // Оригинал отступает от рамки на одну текстовую строку (8 пикселей
        // сетки ZX): сверху — заголовок (строка 1), снизу — приглашение.
        let text_center_x = view.x + view.w / 2;
        let centered_x = |text: &str| text_center_x - text.len() as i32 * 4 * s;
        let title = "---- E L I T E ----";
        let title_y = view.y + 8 * s;
        frame.draw_text(font, centered_x(title), title_y, title, FG, s as usize);
        // Нижняя строка — приглашение нажать SPACE.
        let hint = "Press Space, Commander";
        let hint_y = view.y + view.h - 16 * s;
        frame.draw_text(font, centered_x(hint), hint_y, hint, HINT, s as usize);
        // Вращающийся корабль (или запасная фигура, если модель не найдена) —
        // по центру свободной между надписями полосы.
        let top = title_y + 8 * s;
        let band = (hint_y - top).max(0);
        let cx = text_center_x;
        let cy = top + band / 2;
        let r = (band / 2)
            .min(INTRO_SHIP_RADIUS * s)
            .min(view.w.max(0) * 2 / 5)
            .max(0);
        if let Some(ship) = self.ship {
            self.draw_ship(frame, ship, cx, cy, r);
        } else {
            self.draw_fallback(frame, cx, cy, r, SHIP_COLOR);
        }
    }

    /// Нарисовать вращающийся корабль: тёмные грани и белые рёбра.
    ///
    /// `cx`,`cy` — центр фигуры, `r` — радиус, в который вписан корабль
    /// (на самом широком ракурсе фигура касается этой окружности).
    /// Геометрия берётся из [`Ship`]; сама отрисовка — общая с нативным
    /// полётом, см. [`render3d::draw_ship`].
    ///
    /// Экранный масштаб проекции — постоянный для модели и радиуса.
    /// В оригинале масштаб задан расстоянием до объекта и константой печати
    /// (`CalcPrnCrdVertex`: печатная координата = `world / worldZ`), поэтому
    /// при повороте СКО корабль **не масштабируется** — меняется только
    /// ракурс (плюс небольшая перспектива от разной глубины вершин).
    pub(super) fn draw_ship(&self, frame: &mut Frame, ship: &Ship, cx: i32, cy: i32, r: i32) {
        let cen = render3d::model_center(ship);
        let k = self.ship_scale(ship, cen, INTRO_DIST, r);
        let (bx, by) = self.intro_screen_center(ship, cen, INTRO_DIST);
        let screen = Proj {
            cx: cx as f32,
            cy: cy as f32,
            k,
            bx,
            by,
        };
        render3d::draw_ship(
            frame,
            ship,
            &self.rot,
            [0.0, 0.0, INTRO_DIST],
            &screen,
            SHIP_COLOR,
        );
    }

    /// Запасная фигура (куб), если модель корабля не найдена в таблице.
    fn draw_fallback(&self, frame: &mut Frame, cx: i32, cy: i32, r: i32, color: u32) {
        let rot = &self.rot;
        let cube = [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];
        let mut pts = [(0i32, 0i32); 8];
        for (i, v) in cube.iter().enumerate() {
            let p = rot.apply(*v);
            pts[i] = (cx + (p[0] * r as f32) as i32, cy - (p[1] * r as f32) as i32);
        }
        for (a, b) in CUBE_EDGES {
            frame.line(pts[a].0, pts[a].1, pts[b].0, pts[b].1, color);
        }
    }
}
