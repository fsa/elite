//! Нативная заставка (титульный экран).
//!
//! Заменяет Z80-интро: игра на заставке не выполняется, окно целиком
//! занимает нативный рендер в HD-слой ([`display::Frame`]). Вращающийся
//! объект — настоящая модель корабля из [`crate::shipdata`] (извлечённые из
//! ROM `ShipData`), нарисованная **каркасом** (рёбрами) с отсечением
//! невидимых граней — как это делает оригинальный рендер Z80
//! (`CalcVisibleVerges` + `PrintObject`).
//!
//! Видимость грани повторяет оригинальный тест: грань видна, если её
//! нормаль после поворота СКО направлена к наблюдателю. Камера в оригинале
//! смотрит вдоль +Z (печатные координаты делятся на Z, объект стоит на
//! Z = +0x200), поэтому условие сводится к `нормаль.z < 0` — см.
//! [`face_visibility`]. Старшие биты 4-го байта нормали — знаки её
//! компонентов (бит 7 — X, бит 6 — Y, бит 5 — Z), а не признак
//! «перевёрнутой» грани.
//!
//! Стартовый ракурс берётся из таблицы `SKO4Intro`, но вращение — витринное:
//! объект медленно кувыркается вокруг вертикали и горизонтали экрана, чтобы
//! его можно было осмотреть со всех сторон. Оригинальный интро-цикл
//! (`RotateIntroObj`/`DemoRotateObj` → `RotateCrdNSCO`) крутил объект вокруг
//! его собственной оси, почти совпадающей с носом; из-за этого, например,
//! вход станции `d_Coriolis` (грань +Z) не попадал в кадр никогда.
//!
//! С запуском игры заставка не связана напрямую: `SPACE` переводит Z80 на
//! ветку переинициализации (см. [`native::start_game`](crate::native)),
//! а заставка продолжает показываться, пока игра не дойдёт до `GameMain`.

use crate::display::Frame;
use crate::shipdata::{self, Ship};

/// Индекс текущей модели на заставке (переключается стрелками).
static mut SPLASH_SHIP_IDX: usize = 0;

/// Состояние анимации заставки.
pub struct Splash {
    /// Исходная ориентация (`SKO4Intro`), от которой считаются повороты.
    base: Rotation,
    /// Текущая ориентация корабля.
    rot: Rotation,
    /// Поворот вокруг вертикали экрана (радианы).
    yaw: f32,
    /// Поворот вокруг горизонтали экрана (радианы).
    pitch: f32,
    /// Счётчик кадров — для мигания приглашения «PRESS SPACE».
    tick: u64,
    /// Модель корабля.
    ship: Option<&'static Ship>,
}

impl Splash {
    pub fn new() -> Self {
        let ships = shipdata::SHIPS;
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
        // Стартовый ракурс — ровно тот, что задан в оригинале таблицей
        // `SKO4Intro` (см. [`Rotation::intro`]).
        let base = Rotation::intro();
        Splash {
            base,
            rot: base,
            yaw: 0.0,
            pitch: 0.0,
            tick: 0,
            ship: Some(&ships[unsafe { SPLASH_SHIP_IDX }]),
        }
    }

    /// Переключить корабль влево/вправо (стрелки на заставке).
    ///
    /// Как и оригинальный `SetShip4Intro`, смена корабля заново выставляет
    /// СКО из `SKO4Intro`, поэтому поворот начинается с исходного ракурса.
    pub fn cycle_ship(&mut self, dir: i32) {
        let ships = shipdata::SHIPS;
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
        self.base = Rotation::intro();
        self.rot = self.base;
        self.yaw = 0.0;
        self.pitch = 0.0;
    }

    /// Один кадр анимации: поворот и счётчик мигания.
    ///
    /// Объект поворачивается вокруг двух осей экрана с разными периодами
    /// (см. [`YAW_PER_TICK`] и [`PITCH_PER_TICK`]). Периоды не кратны друг
    /// другу, поэтому ракурс не повторяется и все стороны объекта рано или
    /// поздно попадают в кадр — включая вход станции.
    pub fn tick(&mut self) {
        self.yaw = (self.yaw + YAW_PER_TICK) % std::f32::consts::TAU;
        self.pitch = (self.pitch + PITCH_PER_TICK) % std::f32::consts::TAU;
        let yaw = Rotation::axis_angle([0.0, 1.0, 0.0], self.yaw);
        let pitch = Rotation::axis_angle([1.0, 0.0, 0.0], self.pitch);
        self.rot = yaw.mul(&pitch).mul(&self.base);
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
    /// сентинел «видна всегда»). Видимость грани считает
    /// [`face_visibility`].
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
        // Поворот: та же матрица, что и у оригинала (`CalcMatrixRotate`).
        let rot = &self.rot;
        // Мировые координаты после поворота (без масштаба).
        let rp: Vec<[f32; 3]> = verts
            .iter()
            .map(|v| {
                rot.apply([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ])
            })
            .collect();
        let face_vis = face_visibility(ship, &rot);
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

/// Период полного оборота вокруг вертикали экрана (секунды при 50 Гц).
const YAW_PERIOD_SECS: f32 = 14.0;

/// Период полного оборота вокруг горизонтали экрана (секунды при 50 Гц).
/// Не кратен [`YAW_PERIOD_SECS`] — ракурсы не повторяются.
const PITCH_PERIOD_SECS: f32 = 23.0;

/// Шаг поворота вокруг вертикали за кадр (радианы).
const YAW_PER_TICK: f32 = std::f32::consts::TAU / (YAW_PERIOD_SECS * 50.0);

/// Шаг поворота вокруг горизонтали за кадр (радианы).
const PITCH_PER_TICK: f32 = std::f32::consts::TAU / (PITCH_PERIOD_SECS * 50.0);

/// Начальная ориентация интро-объекта — таблица `SKO4Intro` оригинала:
/// три столбца матрицы поворота (X, Y, Z), записанные знаковыми байтами.
///
/// X = 0x5A,0xA7,0xD1; Y = 0x98,0x64,0xCB; Z = 0x58,0x44,0x41
/// (байт со знаком в старшем бите, модуль — в младших семи).
const INTRO_SCO: [[f32; 3]; 3] = [
    [90.0, -39.0, -81.0],  // ось X модели
    [-24.0, 100.0, -75.0], // ось Y модели
    [88.0, 68.0, 65.0],    // ось Z модели
];

/// Матрица поворота «модель → вид» (3x3), как `MatrixRotate` оригинала.
///
/// `m[i][j]` — вклад оси `j` модели в ось `i` вида; столбцы матрицы — оси
/// модели в координатах вида. Оси вида как в оригинале: X вправо, Y вверх,
/// Z вглубь экрана (камера смотрит вдоль +Z).
#[derive(Clone, Copy)]
struct Rotation {
    m: [[f32; 3]; 3],
}

impl Rotation {
    /// Исходный ракурс интро-объекта из `SKO4Intro` (столбцы нормируются).
    fn intro() -> Self {
        let mut m = [[0.0f32; 3]; 3];
        for (j, c) in INTRO_SCO.iter().enumerate() {
            let n = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
            for (i, row) in m.iter_mut().enumerate() {
                row[j] = c[i] / n;
            }
        }
        Rotation { m }
    }

    /// Поворот вокруг `axis` на `angle` (матрица в той же конвенции).
    fn axis_angle(axis: [f32; 3], angle: f32) -> Self {
        let [x, y, z] = axis;
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        Rotation {
            m: [
                [c + x * x * t, x * y * t - z * s, x * z * t + y * s],
                [y * x * t + z * s, c + y * y * t, y * z * t - x * s],
                [z * x * t - y * s, z * y * t + x * s, c + z * z * t],
            ],
        }
    }

    /// Произведение матриц: `self · other` (сначала `other`, затем `self`).
    fn mul(&self, other: &Rotation) -> Self {
        let mut out = [[0.0f32; 3]; 3];
        for (i, row) in out.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                *v = self.m[i][0] * other.m[0][j]
                    + self.m[i][1] * other.m[1][j]
                    + self.m[i][2] * other.m[2][j];
            }
        }
        Rotation { m: out }
    }

    /// Перевести точку или вектор модели в координаты вида.
    fn apply(&self, v: [f32; 3]) -> [f32; 3] {
        std::array::from_fn(|i| {
            self.m[i][0] * v[0] + self.m[i][1] * v[1] + self.m[i][2] * v[2]
        })
    }
}

/// Видимость граней корабля после поворота СКО.
///
/// Повторяет тест оригинального рендера (`CalcVisibleVerges`): грань видна,
/// если скалярное произведение вектора «нормаль + позиция объекта» на саму
/// нормаль отрицательно, т.е. нормаль направлена к наблюдателю. Камера
/// оригинала смотрит вдоль +Z, а объект стоит перед ней (Z = +0x200),
/// поэтому условие сводится к `нормаль.z < 0`.
///
/// Знаки компонентов нормали уже учтены при разборе ROM
/// (`Face::nx/ny/nz`), так что `Face::flags` в тесте не участвует.
fn face_visibility(ship: &Ship, rot: &Rotation) -> Vec<bool> {
    ship.faces
        .iter()
        .map(|f| {
            let n = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
            n[2] < 0.0
        })
        .collect()
}

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
        let ships = shipdata::SHIPS;
        let s = ships.iter().find(|x| x.name == "d_Cobra1").expect("модель корабля не найдена");
        assert!(!s.vertices.is_empty(), "у модели нет вершин");
        assert!(!s.edges.is_empty(), "у модели нет рёбер");
        assert!(!s.faces.is_empty(), "у модели нет граней");
    }

    /// Модель «Cobra» для проверок видимости.
    fn cobra() -> &'static Ship {
        shipdata::SHIPS
            .iter()
            .find(|x| x.name == "d_Cobra1")
            .expect("модель корабля не найдена")
    }

    /// Вспомогательный поворот вокруг вертикальной оси (для проверок видимости).
    fn yaw_rot(angle: f32) -> Rotation {
        Rotation::axis_angle([0.0, 1.0, 0.0], angle)
    }

    /// Видимость грани — по знаку Z повёрнутой нормали (тест оригинала).
    ///
    /// Раньше в тесте участвовал бит 7 `Face::flags`, который на самом деле
    /// задаёт знак X-компоненты нормали, — из-за чего видимая и невидимая
    /// стороны менялись местами.
    #[test]
    fn face_visibility_follows_rotated_normal_z() {
        let ship = cobra();
        for yaw in [0.0f32, 0.7, 1.5, 2.4, 3.0, 4.5] {
            let rot = yaw_rot(yaw);
            let vis = face_visibility(ship, &rot);
            assert_eq!(vis.len(), ship.faces.len());
            for (f, &v) in ship.faces.iter().zip(vis.iter()) {
                let n = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
                assert_eq!(v, n[2] < 0.0, "грань видна не по знаку нормали");
            }
        }
    }

    /// При нулевом повороте видна «задняя» половина корабля (нормаль хвоста
    /// смотрит в -Z, т.е. к наблюдателю) — как в оригинале, где камера
    /// смотрит вдоль +Z, а противники в игре видны с кормы.
    #[test]
    fn face_visibility_shows_tail_side_at_identity() {
        let ship = cobra();
        let rot = yaw_rot(0.0);
        let vis = face_visibility(ship, &rot);
        // Нормаль хвоста (0, 0, -80) — видна.
        let tail = ship
            .faces
            .iter()
            .position(|f| f.nx == 0 && f.ny == 0 && f.nz < 0)
            .expect("нормаль хвоста не найдена");
        assert!(vis[tail], "хвост должен быть виден");
        // Верхние грани (nz > 0) — не видны.
        for (i, f) in ship.faces.iter().enumerate() {
            if f.nz > 0 {
                assert!(!vis[i], "верхняя грань {i} не должна быть видна");
            }
        }
    }

    /// Видимая половина модели смотрит к наблюдателю: сумма Z-компонент
    /// видимых нормалей отрицательна, а невидимых — положительна.
    #[test]
    fn face_visibility_culls_far_half_only() {
        let ship = cobra();
        for yaw in [0.0f32, 0.9, 2.0, 3.3, 5.0] {
            let rot = yaw_rot(yaw);
            let vis = face_visibility(ship, &rot);
            let mut near = 0.0f32;
            let mut far = 0.0f32;
            for (f, &v) in ship.faces.iter().zip(vis.iter()) {
                let z = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32])[2];
                if v {
                    near += z;
                } else {
                    far += z;
                }
            }
            assert!(near < 0.0, "видимые грани должны смотреть к наблюдателю");
            assert!(far > 0.0, "невидимые грани должны смотреть от наблюдателя");
        }
    }

    /// Стартовый ракурс заставки — ровно `SKO4Intro` оригинала.
    ///
    /// Оси модели после поворота должны совпасть с нормированными столбцами
    /// таблицы, а нос (локальная +Z) — смотреть от наблюдателя: оригинал
    /// показывает корабль с кормы-сверху, а не «носом на камеру».
    #[test]
    fn splash_starts_from_original_intro_pose() {
        let st = Splash::new();
        for (j, col) in INTRO_SCO.iter().enumerate() {
            let n = (col[0] * col[0] + col[1] * col[1] + col[2] * col[2]).sqrt();
            let mut local = [0.0f32; 3];
            local[j] = 1.0;
            let w = st.rot.apply(local);
            for i in 0..3 {
                let want = col[i] / n;
                assert!(
                    (w[i] - want).abs() < 1e-6,
                    "ось {j} повёрнута не как в SKO4Intro: {w:?} vs {want}"
                );
            }
        }
        let nose = st.rot.apply([0.0, 0.0, 1.0]);
        assert!(nose[2] > 0.0, "оригинал показывает корабль с кормы: {nose:?}");
    }

    /// Угол поворота между двумя ориентациями (радианы).
    fn angle_between(a: &Rotation, b: &Rotation) -> f32 {
        // b = a·R, значит R = aᵀ·b; угол — из следа R.
        let mut tr = 0.0;
        for i in 0..3 {
            for k in 0..3 {
                tr += a.m[k][i] * b.m[k][i];
            }
        }
        ((tr - 1.0) / 2.0).clamp(-1.0, 1.0).acos()
    }

    /// Вращение плавное: за кадр объект поворачивается на небольшой угол.
    #[test]
    fn splash_rotation_is_smooth() {
        let mut st = Splash::new();
        let mut prev = st.rot;
        for n in 0..100 {
            st.tick();
            let a = angle_between(&prev, &st.rot);
            assert!(
                a > 0.0 && a < 0.03,
                "кадр {n}: неестественный шаг поворота {a}"
            );
            prev = st.rot;
        }
    }

    /// За полный цикл кувыркания все стороны объекта (включая нос/корму и
    /// верх/низ) оказываются повёрнуты к наблюдателю. Для станции это
    /// означает, что вход (+Z модели) становится виден.
    #[test]
    fn splash_rotation_shows_all_sides() {
        let mut st = Splash::new();
        let axes = [
            [1.0f32, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        let mut seen = [false; 6];
        // Периоды 14 и 23 секунды: за 30 секунд оба проходят с запасом.
        for _ in 0..(30 * 50) {
            st.tick();
            for (i, axis) in axes.iter().enumerate() {
                if st.rot.apply(*axis)[2] < 0.0 {
                    seen[i] = true;
                }
            }
        }
        assert!(seen.iter().all(|&s| s), "не все стороны попали в кадр: {seen:?}");
        // Вход станции — грань +Z модели `d_Coriolis`.
        let station = shipdata::find("d_Coriolis").expect("в таблице нет станции");
        let slot = station
            .faces
            .iter()
            .find(|f| f.nx == 0 && f.ny == 0 && f.nz > 0)
            .expect("у станции нет грани входа");
        let slot_normal = [slot.nx as f32, slot.ny as f32, slot.nz as f32];
        let mut slot_seen = false;
        let mut st = Splash::new();
        for _ in 0..(30 * 50) {
            st.tick();
            if st.rot.apply(slot_normal)[2] < 0.0 {
                slot_seen = true;
                break;
            }
        }
        assert!(slot_seen, "вход станции так и не стал виден");
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
            st.rot = yaw_rot(i as f32 * std::f32::consts::PI / 4.0);
            let mut f = Frame::new(520, 520);
            if let Some(ship) = st.ship {
                st.draw_ship(&mut f, ship, 260, 260, 240);
            }
            write_ppm(&format!("{prefix}_{i}.ppm"), f.pixels(), 520, 520);
        }
    }
}
