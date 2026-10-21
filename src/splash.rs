//! Нативная заставка (титульный экран).
//!
//! Заменяет Z80-интро: игра на заставке не выполняется, окно целиком
//! занимает нативный рендер в HD-слой ([`display::Frame`]). Вращающийся
//! объект — настоящая модель корабля из [`crate::shipdata`] (извлечённые из
//! ROM `ShipData`), нарисованная **каркасом** (рёбрами) с отсечением
//! невидимых граней — как это делает оригинальный рендер Z80
//! (`CalcVisibleVerges` + `PrintObject`).
//!
//! С запуском игры заставка не связана напрямую: `SPACE` переводит Z80 на
//! ветку переинициализации (см. [`native::start_game`](crate::native)),
//! а заставка продолжает показываться, пока игра не дойдёт до `GameMain`.

use crate::display::Frame;
use crate::shipdata::{self, Ship};

/// Индекс текущей модели на заставке (для отладки).
static mut SPLASH_SHIP_IDX: usize = 0;

/// Состояние анимации заставки.
pub struct Splash {
    /// Угол поворота корабля вокруг вертикальной оси (радианы).
    yaw: f32,
    /// Счётчик кадров — для мигания приглашения «PRESS SPACE».
    tick: u64,
    /// Модель корабля.
    ship: Option<&'static Ship>,
}

impl Splash {
    pub fn new() -> Self {
        let ships = shipdata::ELITE_SHIPS;
        let mut idx = 0;
        for (i, s) in ships.iter().enumerate() {
            if s.name == "d_Cobra1" {
                idx = i;
                break;
            }
        }
        unsafe {
            SPLASH_SHIP_IDX = idx;
        }
        Splash {
            yaw: std::f32::consts::FRAC_PI_4,
            tick: 0,
            ship: Some(&ships[unsafe { SPLASH_SHIP_IDX }]),
        }
    }

    /// Переключить корабль влево/вправо (для отладки).
    pub fn cycle_ship(&mut self, dir: i32) {
        let ships = shipdata::ELITE_SHIPS;
        if ships.is_empty() {
            return;
        }
        unsafe {
            let mut idx = SPLASH_SHIP_IDX as i32 + dir;
            let len = ships.len() as i32;
            if idx < 0 {
                idx = len - 1;
            }
            if idx >= len {
                idx = 0;
            }
            SPLASH_SHIP_IDX = idx as usize;
            self.ship = Some(&ships[SPLASH_SHIP_IDX]);
        }
    }

    /// Один кадр анимации: поворот корабля и счётчик мигания.
    pub fn tick(&mut self) {
        self.yaw += 0.01;
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
        // Вращающийся корабль (или запасная фигура, если модель не найдена).
        let cy = oy + 82 * s;
        let r = 52 * s;
        if let Some(ship) = self.ship {
            self.draw_ship(frame, ship, cx, cy, r);
        } else {
            self.draw_fallback(frame, cx, cy, r, 0xFF00_FFFF);
        }
        // Приглашение (мигает с периодом ~1 c) и подсказка меню.
        if (self.tick / 25) % 2 == 0 {
            let start = "PRESS SPACE";
            let w = start.len() as i32 * 8 * 2 * s;
            frame.draw_text(font, cx - w / 2, oy + 140 * s, start, HINT, (2 * s) as usize);
        }
        let menu = "ESC - SAVE / LOAD MENU";
        let w = menu.len() as i32 * 8 * s;
        frame.draw_text(font, cx - w / 2, oy + 160 * s, menu, GREY, s as usize);
    }

    /// Нарисовать вращающийся корабль **каркасом** (рёбрами), как в оригинале.
    ///
    /// `cx`,`cy` — центр фигуры, `r` — половина её габарита в пикселях.
    /// Геометрия берётся из [`Ship`]; воспроизводится логика Z80-рендера
    /// (`CalcVisibleVerges` + `PrintObject`): ребро рисуется, если видна хотя
    /// бы одна из назначенных ему граней (`Edge::faces`, полубайты; `0xF` —
    /// сентинел «видна всегда»). Грань видна, если её нормаль после поворота
    /// направлена от наблюдателя (знак скалярного произведения), при этом
    /// бит 7 в `Face::flags` переворачивает фронтальный тест.
    fn draw_ship(&self, frame: &mut Frame, ship: &Ship, cx: i32, cy: i32, r: i32) {
        let verts = ship.vertices;
        if verts.is_empty() {
            return;
        }
        // Центр габаритов и масштаб: модели в знаковых байтах, центрируем по
        // bounding box — как в оригинальном рендере.
        let (mut mn, mut mx) = ([i32::MAX; 3], [i32::MIN; 3]);
        for v in verts {
            for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
                mn[k] = mn[k].min(*c as i32);
                mx[k] = mx[k].max(*c as i32);
            }
        }
        let cen = [
            (mn[0] + mx[0]) as f32 / 2.0,
            (mn[1] + mx[1]) as f32 / 2.0,
            (mn[2] + mx[2]) as f32 / 2.0,
        ];
        // Поворот: вокруг Y (курс) и фиксированный наклон вокруг X — нос к
        // наблюдателю чуть сверху, как в оригинальном интро.
        let (sin_y, cos_y) = (self.yaw.sin(), self.yaw.cos());
        const TILT: f32 = 0.5;
        let (sin_x, cos_x) = (TILT.sin(), TILT.cos());
        let rot = |v: [f32; 3]| -> [f32; 3] {
            let x = v[0] * cos_y + v[2] * sin_y;
            let z = -v[0] * sin_y + v[2] * cos_y;
            let y = v[1] * cos_x - z * sin_x;
            let z = v[1] * sin_x + z * cos_x;
            [x, y, z]
        };
        // Мировые координаты после поворота (без масштаба).
        let rp: Vec<[f32; 3]> = verts
            .iter()
            .map(|v| {
                rot([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ])
            })
            .collect();
        let face_vis: Vec<bool> = ship
            .faces
            .iter()
            .map(|f| {
                let n = rot([f.nx as f32, f.ny as f32, f.nz as f32]);
                let s = if f.flags & 0x80 != 0 { -n[2] } else { n[2] };
                s < 0.0
            })
            .collect();
        // Экранная нормировка: вписываем фигуру в `2r` по проекции.
        // Используем фиксированный масштаб по исходным габаритам модели,
        // чтобы избежать «дёрганья» при вращении.
        let (mut mn2, mut mx2) = ([i32::MAX; 3], [i32::MIN; 3]);
        for v in verts {
            for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
                mn2[k] = mn2[k].min(*c as i32);
                mx2[k] = mx2[k].max(*c as i32);
            }
        }
        let orig_span = ((mx2[0] - mn2[0]) as f32).max((mx2[1] - mn2[1]) as f32).max((mx2[2] - mn2[2]) as f32);
        let k = (2 * r) as f32 / orig_span.max(1.0);
        let sx: Vec<(i32, i32)> = rp
            .iter()
            .map(|p| {
                (
                    cx + (p[0] * k) as i32,
                    cy - (p[1] * k) as i32,
                )
            })
            .collect();
        for e in ship.edges {
            let (a, b) = (e.v[0] as usize, e.v[1] as usize);
            if a >= sx.len() || b >= sx.len() {
                continue;
            }
            let f0 = (e.faces & 0x0F) as usize;
            let f1 = (e.faces >> 4) as usize;
            let vis = |f: usize| f == 0xF || face_vis.get(f).copied().unwrap_or(false);
            if vis(f0) || vis(f1) {
                frame.line(sx[a].0, sx[a].1, sx[b].0, sx[b].1, SHIP_COLOR);
            }
        }
    }

    /// Запасная фигура (куб), если модель корабля не найдена в таблице.
    fn draw_fallback(&self, frame: &mut Frame, cx: i32, cy: i32, r: i32, color: u32) {
        let (sin_y, cos_y) = (self.yaw.sin(), self.yaw.cos());
        const TILT: f32 = 0.45;
        let (sin_x, cos_x) = (TILT.sin(), TILT.cos());
        let rot = |v: [f32; 3]| -> [f32; 3] {
            let x = v[0] * cos_y + v[2] * sin_y;
            let z = -v[0] * sin_y + v[2] * cos_y;
            let y = v[1] * cos_x - z * sin_x;
            [x, y, z]
        };
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
            let p = rot(*v);
            pts[i] = (cx + (p[0] * r as f32) as i32, cy + (p[1] * r as f32) as i32);
        }
        for (a, b) in CUBE_EDGES {
            frame.line(pts[a].0, pts[a].1, pts[b].0, pts[b].1, color);
        }
    }
}

impl Default for Splash {
    fn default() -> Self {
        Self::new()
    }
}

/// Рёбра запасного куба.
const CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0),
    (4, 5), (5, 6), (6, 7), (7, 4),
    (0, 4), (1, 5), (2, 6), (3, 7),
];

/// Цвет линий корабля (циан — как линии объектов в оригинале).
const SHIP_COLOR: u32 = 0xFF00_FFFF;

#[cfg(test)]
mod tests {
    use super::*;

    /// Заставка: непрозрачный чёрный фон, белый заголовок, циановый
    /// корабль, жёлтое приглашение и серая подсказка меню.
    #[test]
    fn splash_draws_title_ship_and_hints() {
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
            "корабль не нарисован"
        );
        assert!(
            f.pixels().iter().any(|&p| p == 0xFFFF_FF00),
            "приглашение не нарисовано"
        );
        assert!(
            f.pixels().iter().any(|&p| p == 0xFF90_9090),
            "подсказка меню не нарисована"
        );
        if let Ok(path) = std::env::var("ELITE_DUMP_SPLASH") {
            write_ppm(&path, f.pixels(), 1280, 960);
        }
    }

    /// Сохранить кадр в PPM (R8G8B8) для визуальной проверки.
    fn write_ppm(path: &str, buf: &[u32], w: usize, h: usize) {
        let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
        for &p in buf {
            out.push((p >> 16) as u8);
            out.push((p >> 8) as u8);
            out.push(p as u8);
        }
        let _ = std::fs::write(path, out);
    }

    /// Анимация: корабль вращается — кадры через 40 тиков различаются.
    #[test]
    fn splash_animation_rotates_ship() {
        let font = vec![0u8; 0x300]; // без текста — только корабль
        let mut a = Frame::new(1280, 960);
        let mut st = Splash::new();
        st.draw(&mut a, &font);
        for _ in 0..40 {
            st.tick();
        }
        let mut b = Frame::new(1280, 960);
        st.draw(&mut b, &font);
        assert!(a.pixels() != b.pixels(), "корабль не вращается");
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

    /// Корабль нарисован каркасом: в области фигуры есть линии, но нет
    /// сплошной заливки (силуэт читается по рёбрам, как в оригинале).
    ///
    /// Проверяем область корабля (центр `82*s`, радиус `52*s`, `s = 5` при
    /// окне 1280x960): должна быть заметная доля «чернил», но она не должна
    /// приближаться к сплошной заливке.
    #[test]
    fn splash_ship_is_wireframe() {
        let font = vec![0u8; 0x300];
        let mut f = Frame::new(1280, 960);
        let st = Splash::new();
        st.draw(&mut f, &font);
        let (cx, cy, r) = (1280 / 2, 82 * 5, 52 * 5);
        let mut ink = 0;
        let mut total = 0;
        for y in cy - r..cy + r {
            for x in cx - r..cx + r {
                total += 1;
                if f.pixels()[y as usize * 1280 + x as usize] != 0xFF00_0000 {
                    ink += 1;
                }
            }
        }
        assert!(total > 0, "область корабля пуста");
        assert!(ink > 0, "корабль не нарисован");
        // Каркас: линии есть, но заливки нет (не больше ~25% площади).
        assert!(
            ink * 4 < total,
            "корабль залит, а не каркас: {ink} из {total}"
        );
    }

    /// Модель корабля из `shipdata` найдена и не пуста (иначе на заставке
    /// рисовался бы запасной куб).
    #[test]
    fn splash_ship_model_exists() {
        let ships = shipdata::ELITE_SHIPS;
        let s = ships.iter().find(|x| x.name == "d_Cobra1").expect("модель корабля не найдена");
        assert!(!s.vertices.is_empty(), "у модели нет вершин");
        assert!(!s.edges.is_empty(), "у модели нет рёбер");
        assert!(!s.faces.is_empty(), "у модели нет граней");
    }

    /// Дамп серии кадров вращения (`ELITE_DUMP_ANGLES=prefix` →
    /// `prefix_0.ppm`.. ) — для визуальной проверки силуэта и анимации.
    #[test]
    fn dump_ship_angles() {
        let Ok(prefix) = std::env::var("ELITE_DUMP_ANGLES") else {
            return;
        };
        let mut st = Splash::new();
        for i in 0..8 {
            st.yaw = i as f32 * std::f32::consts::PI / 4.0;
            let mut f = Frame::new(520, 520);
            if let Some(ship) = st.ship {
                st.draw_ship(&mut f, ship, 260, 260, 240);
            }
            write_ppm(&format!("{prefix}_{i}.ppm"), f.pixels(), 520, 520);
        }
    }
}
