//! Нативная заставка (титульный экран).
//!
//! Заменяет Z80-интро: игра на заставке не выполняется, окно целиком
//! занимает нативный рендер в HD-слой ([`display::Frame`]). Вращение
//! корабля из оригинального интро пока заменено вращающейся тестовой
//! фигурой (куб) с ортографической проекцией. Грани куба залиты и
//! отсекаются по видимости, поэтому фигура непрозрачная и объёмная —
//! как и полагается будущим кораблям (см. требования проекта).
//!
//! С запуском игры заставка не связана напрямую: `SPACE` переводит Z80 на
//! ветку переинициализации (см. [`native::start_game`](crate::native)),
//! а заставка продолжает показываться, пока игра не дойдёт до `GameMain`.

use crate::display::Frame;

/// Вершины куба (единичный куб с центром в начале координат).
const VERTS: [[f32; 3]; 8] = [
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
];

/// Грани куба: четыре индекса вершин (по кругу) и внешняя нормаль.
const FACES: [([usize; 4], [f32; 3]); 6] = [
    ([0, 1, 2, 3], [0.0, 0.0, -1.0]),
    ([4, 5, 6, 7], [0.0, 0.0, 1.0]),
    ([0, 1, 5, 4], [0.0, -1.0, 0.0]),
    ([3, 2, 6, 7], [0.0, 1.0, 0.0]),
    ([0, 3, 7, 4], [-1.0, 0.0, 0.0]),
    ([1, 2, 6, 5], [1.0, 0.0, 0.0]),
];

/// Состояние анимации заставки.
pub struct Splash {
    /// Угол поворота фигуры вокруг вертикальной оси (радианы).
    yaw: f32,
    /// Счётчик кадров — для мигания приглашения «PRESS SPACE».
    tick: u64,
}

impl Splash {
    pub fn new() -> Self {
        Splash { yaw: 0.0, tick: 0 }
    }

    /// Один кадр анимации: поворот фигуры и счётчик мигания.
    pub fn tick(&mut self) {
        self.yaw += 0.02;
        self.tick += 1;
    }

    /// Нарисовать заставку на слой `frame` во всё окно.
    ///
    /// `font` — знакогенератор ПЗУ ZX (коды 0x20..0x7F), как в меню паузы.
    /// Координаты задаются в «пикселях ZX» (сетка 256x192), масштабируются
    /// целым множителем `s` и центрируются в окне по обеим осям — поэтому
    /// композиция одинакова при любом размере окна и соотношении сторон.
    #[allow(dead_code)] // используется в тестах; в игре — `draw_in`
    pub fn draw(&self, frame: &mut Frame, font: &[u8]) {
        frame.fill_rect(0, 0, frame.width, frame.height, 0xFF00_0000);
        self.draw_in(frame, font, frame.height as i32);
    }
    /// Нарисовать заставку в верхней части слоя высотой `area_h` пикселей.
    ///
    /// Композиция центрируется в области `0..area_h` — на заставке снизу
    /// может быть показана нативная приборная панель (кокпит), и тогда
    /// заголовок с фигурой не должны под неё заходить.
    pub fn draw_in(&self, frame: &mut Frame, font: &[u8], area_h: i32) {
        const BG: u32 = 0xFF00_0000; // чёрный фон (непрозрачный)
        const FG: u32 = 0xFFD7_D7D7; // белый — заголовок
        const SHAPE: u32 = 0xFF00_FFFF; // циан — вращающаяся фигура
        const HINT: u32 = 0xFFFF_FF00; // жёлтый — приглашение
        const GREY: u32 = 0xFF90_9090; // серый — подсказка меню
        let area_h = area_h.clamp(1, frame.height as i32);
        // Фон области (снизу может быть своя панель — её не трогаем).
        frame.fill_rect(0, 0, frame.width, area_h as usize, BG);
        // Масштаб сетки 256x192, влезающий в область; смещение центрирует её.
        let s = ((frame.width / 256).min(area_h as usize / 192)).max(1) as i32;
        let ox = (frame.width as i32 - 256 * s) / 2;
        let oy = (area_h - 192 * s) / 2;
        let cx = ox + 128 * s;
        // Заголовок увеличен в 4 раза (относительно сетки).
        let title = "ELITE";
        let tw = title.len() as i32 * 8 * 4 * s;
        frame.draw_text(font, cx - tw / 2, oy + 14 * s, title, FG, (4 * s) as usize);
        // Вращающаяся фигура.
        self.draw_shape(frame, cx, oy + 88 * s, 34 * s, SHAPE);
        // Приглашение (мигает с периодом ~1 c) и подсказка меню.
        if (self.tick / 25) % 2 == 0 {
            let start = "PRESS SPACE";
            let w = start.len() as i32 * 8 * 2 * s;
            frame.draw_text(font, cx - w / 2, oy + 134 * s, start, HINT, (2 * s) as usize);
        }
        let menu = "ESC - SAVE / LOAD MENU";
        let w = menu.len() as i32 * 8 * s;
        frame.draw_text(font, cx - w / 2, oy + 158 * s, menu, GREY, s as usize);
    }

    /// Вращающаяся фигура: поворот вокруг Y плюс фиксированный наклон вокруг
    /// X, ортографическая проекция на плоскость экрана, `r` — половина
    /// размера фигуры в пикселях.
    ///
    /// Грани, обращённые к наблюдателю (по знаку нормали после поворота),
    /// заливаются плоским цветом: каждая грань чуть разной яркости, поэтому
    /// фигура выглядит объёмной и непрозрачной. Задние грани не рисуются,
    /// но их всё равно перекрывают передние.
    fn draw_shape(&self, frame: &mut Frame, cx: i32, cy: i32, r: i32, color: u32) {
        let (sin_y, cos_y) = (self.yaw.sin(), self.yaw.cos());
        const TILT: f32 = 0.45;
        let (sin_x, cos_x) = (TILT.sin(), TILT.cos());
        // Поворот: сначала вокруг Y, затем вокруг X.
        let rot = |v: [f32; 3]| -> [f32; 3] {
            let x = v[0] * cos_y + v[2] * sin_y;
            let z = -v[0] * sin_y + v[2] * cos_y;
            let y = v[1] * cos_x - z * sin_x;
            [x, y, z]
        };
        let mut pts = [(0i32, 0i32); 8];
        for (i, v) in VERTS.iter().enumerate() {
            let p = rot(*v);
            pts[i] = (cx + (p[0] * r as f32) as i32, cy + (p[1] * r as f32) as i32);
        }
        // Сначала все грани: сортируем по глубине (средняя Z после поворота),
        // дальние рисуем первыми, чтобы передние их перекрыли (painter's
        // algorithm). Камера смотрит вдоль -Z, поэтому «к наблюдателю» — это
        // бо́льшая Z.
        let (fr, fg, fb) = ((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF);
        let shade = |k: f32| -> u32 {
            let c = |v: u32| ((v as f32 * k).min(255.0)) as u32;
            0xFF00_0000 | (c(fr) << 16) | (c(fg) << 8) | c(fb)
        };
        // Направление на источник света (верх-слева-спереди) — для объёма.
        let light = [-0.4f32, -0.5, 0.76];
        let mut order: Vec<(f32, usize)> = FACES
            .iter()
            .enumerate()
            .map(|(i, (f, _))| {
                let z = f.iter().map(|&j| rot(VERTS[j])[2]).sum::<f32>() / 4.0;
                (z, i)
            })
            .collect();
        order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for (z, fi) in order {
            let (f, n) = FACES[fi];
            // Плоское «освещение»: яркость по нормали грани (нормаль тоже
            // поворачиваем). Грань, обращённая к свету, светлее; отвёрнутая —
            // темнее. Так каждая из видимых граней куба имеет свою яркость.
            let nn = rot(n);
            let dot = (nn[0] * light[0] + nn[1] * light[1] + nn[2] * light[2]).max(0.0);
            let k = (0.4 + 0.6 * dot).clamp(0.4, 1.0);
            // Дальние грани чуть гасим — они и так перекрыты, но по кромкам
            // это добавляет глубины.
            let k = if z < 0.0 { k * 0.7 } else { k };
            let col = shade(k);
            let quad = [pts[f[0]], pts[f[1]], pts[f[2]], pts[f[3]]];
            fill_quad(frame, &quad, col);
        }
        // Контур куба поверх заливки — чётче читаются рёбра.
        for (a, b) in EDGES {
            frame.line(pts[a].0, pts[a].1, pts[b].0, pts[b].1, color);
        }
    }
}

/// Рёбра куба (пары индексов вершин) — рисуются контуром поверх заливки.
const EDGES: [(usize, usize); 12] = [
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

/// Залить выпуклый четырёхугольник `quad` (по кругу) сплошным цветом.
///
/// Разбиваем на два треугольника и заливаем каждый по сканлайнам — простая
/// и достаточная для выпуклого куба растеризация (без сглаживания).
fn fill_quad(frame: &mut Frame, quad: &[(i32, i32); 4], color: u32) {
    fill_tri(frame, quad[0], quad[1], quad[2], color);
    fill_tri(frame, quad[0], quad[2], quad[3], color);
}

/// Залить треугольник `(a, b, c)` сплошным цветом по сканлайнам.
fn fill_tri(frame: &mut Frame, a: (i32, i32), b: (i32, i32), c: (i32, i32), color: u32) {
    let (mut lo, mut mid, mut hi) = (a, b, c);
    if lo.1 > mid.1 {
        std::mem::swap(&mut lo, &mut mid);
    }
    if mid.1 > hi.1 {
        std::mem::swap(&mut mid, &mut hi);
    }
    if lo.1 > mid.1 {
        std::mem::swap(&mut lo, &mut mid);
    }
    if lo.1 == hi.1 {
        return; // вырожденный по вертикали
    }
    // Интерполяция x по левой и правой кромкам на строке y.
    let interp = |y: i32, p0: (i32, i32), p1: (i32, i32)| -> f32 {
        let dy = (p1.1 - p0.1) as f32;
        if dy == 0.0 {
            p0.0 as f32
        } else {
            p0.0 as f32 + (y - p0.1) as f32 * (p1.0 - p0.0) as f32 / dy
        }
    };
    let mut scan = |y0: i32, y1: i32, xl: &dyn Fn(i32) -> f32, xr: &dyn Fn(i32) -> f32| {
        for y in y0..y1 {
            let (mut x0, mut x1) = (xl(y), xr(y));
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
            }
            let xi0 = x0.ceil() as i32;
            let xi1 = x1.floor() as i32;
            if xi1 >= xi0 {
                frame.hline(xi0, y, (xi1 - xi0 + 1) as usize, color);
            }
        }
    };
    let left = |y: i32| interp(y, lo, hi);
    let right_lo = |y: i32| interp(y, lo, mid);
    let right_hi = |y: i32| interp(y, mid, hi);
    scan(lo.1, mid.1, &left, &right_lo);
    scan(mid.1, hi.1, &left, &right_hi);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Заставка: непрозрачный чёрный фон, белый заголовок, циановая
    /// вращающаяся фигура, жёлтое приглашение и серая подсказка меню.
    #[test]
    fn splash_draws_title_shape_and_hints() {
        let font = vec![0xFFu8; 0x300]; // шрифт «все биты залиты»
        let mut f = Frame::new(1280, 960);
        let st = Splash::new();
        st.draw(&mut f, &font);
        assert_eq!(f.pixels()[0], 0xFF00_0000, "фон должен быть чёрным и непрозрачным");
        assert!(
            f.pixels().iter().any(|&p| p == 0xFFD7_D7D7),
            "заголовок не нарисован"
        );
        assert!(
            f.pixels().iter().any(|&p| p == 0xFF00_FFFF),
            "фигура не нарисована"
        );
        assert!(
            f.pixels().iter().any(|&p| p == 0xFFFF_FF00),
            "приглашение не нарисовано"
        );
        assert!(
            f.pixels().iter().any(|&p| p == 0xFF90_9090),
            "подсказка меню не нарисована"
        );
    }

    /// Анимация: фигура вращается — кадры через 40 тиков различаются.
    #[test]
    fn splash_animation_rotates_shape() {
        let font = vec![0u8; 0x300]; // без текста — только фигура
        let mut a = Frame::new(1280, 960);
        let mut st = Splash::new();
        st.draw(&mut a, &font);
        for _ in 0..40 {
            st.tick();
        }
        let mut b = Frame::new(1280, 960);
        st.draw(&mut b, &font);
        assert!(a.pixels() != b.pixels(), "фигура не вращается");
    }

    /// При не-кратных размерах окна (например, 16:10) композиция заставки
    /// центрируется по обеим осям: заголовок не прижат к левому краю.
    #[test]
    fn splash_is_centered_for_16_10() {
        let font = vec![0xFFu8; 0x300];
        // 1680x1050: масштаб сетки 256x192 — 6, ширина сетки 1536, высота
        // 1152 > 1050, поэтому масштаб 5 (1280x960), отступы ox=200, oy=45.
        let mut f = Frame::new(1680, 1050);
        let st = Splash::new();
        st.draw(&mut f, &font);
        // Левые поля (по 200 px) должны остаться фоном — заголовок не съехал.
        let bg = 0xFF00_0000;
        for y in 0..1050 {
            for x in 0..200 {
                assert_eq!(
                    f.pixels()[y * 1680 + x],
                    bg,
                    "заставка съехала влево: пиксель ({x},{y}) вне центральной области"
                );
            }
        }
        // Справа тоже поля.
        for y in 0..1050 {
            for x in 1480..1680 {
                assert_eq!(f.pixels()[y * 1680 + x], bg, "заставка не по центру справа");
            }
        }
    }

    /// Фигура непрозрачная: её грани залиты, а не нарисованы контуром.
    ///
    /// Проверяем область вокруг центра фигуры (256x192 → масштаб 5 при
    /// окне 1280x960): подавляющая часть пикселей внутри силуэта должна
    /// быть закрашена (не фоном).
    #[test]
    fn splash_shape_is_opaque() {
        let font = vec![0u8; 0x300];
        let mut f = Frame::new(1280, 960);
        let st = Splash::new();
        st.draw(&mut f, &font);
        // Область фигуры: центр (88*s) и радиус (34*s), s = 5.
        let (cx, cy, r) = (1280 / 2, 88 * 5, 34 * 5);
        let mut filled = 0;
        let mut total = 0;
        for y in cy - r..cy + r {
            for x in cx - r..cx + r {
                // Только внутри вписанного круга — гарантированно внутри куба.
                let (dx, dy) = (x - cx, y - cy);
                if dx * dx + dy * dy > (r / 2) * (r / 2) {
                    continue;
                }
                total += 1;
                if f.pixels()[y as usize * 1280 + x as usize] != 0xFF00_0000 {
                    filled += 1;
                }
            }
        }
        assert!(total > 0, "область фигуры пуста");
        assert!(
            filled * 10 >= total * 9,
            "фигура прозрачная: закрашено {filled} из {total}"
        );
    }

    /// На всём обороте фигура остаётся непрозрачной (нет «дыр» между
    /// гранями): проверяем заполнение внутри вписанного круга для многих
    /// углов поворота.
    #[test]
    fn splash_shape_stays_opaque_while_rotating() {
        let font = vec![0u8; 0x300];
        let (cx, cy, r) = (1280 / 2, 88 * 5, 34 * 5);
        let mut st = Splash::new();
        for _ in 0..150 {
            let mut f = Frame::new(1280, 960);
            st.draw(&mut f, &font);
            let mut filled = 0;
            let mut total = 0;
            for y in cy - r..cy + r {
                for x in cx - r..cx + r {
                    let (dx, dy) = (x - cx, y - cy);
                    if dx * dx + dy * dy > (r / 2) * (r / 2) {
                        continue;
                    }
                    total += 1;
                    if f.pixels()[y as usize * 1280 + x as usize] != 0xFF00_0000 {
                        filled += 1;
                    }
                }
            }
            assert!(
                filled * 5 >= total * 4,
                "фигура прозрачная при повороте: {filled} из {total}"
            );
            st.tick();
        }
    }
}
