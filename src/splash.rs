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
//! Стартовый ракурс берётся из таблицы `SKO4Intro`. Оригинальный интро-цикл
//! (`RotateIntroObj` → `RotateCrdNSCO`) поворачивал СКО фиксированным шагом:
//! измерено по эмулятору — ≈ 10.4° вокруг постоянной оси мира (≈ (0.23, 0.64,
//! 0.74)), раз в ~12 кадров. Получается «карусель»: корабль кувыркается
//! вокруг этой оси, нос остаётся развёрнут от наблюдателя; полный оборот
//! ≈ 8 секунд. В этой реализации тот же шаг применяется каждый кадр с
//! делением на 12 — угловая скорость сохранена, движение плавное.
//!
//! Переключение кораблей стрелками ◄/► повторяет оригинальный цикл
//! (`loc_71A2`): восемь боевых кораблей (Cobra → Python → ASP →
//! Fer-de-Lance → Targon → Sidewinder → Krait → Viper), старт — Cobra,
//! смена корабля сбрасывает ракурс на `SKO4Intro` — как `SetShip4Intro`.
//!
//! Надписи — как в оригинале: заголовок `---- E L I T E ----` (строка 1,
//! колонка 5) и приглашение `PRESS SPACE` внизу, шрифтом сетки ZX (то есть
//! мелким). Других надписей нет.
//!
//! С запуском игры заставка не связана напрямую: `SPACE` переводит Z80 на
//! ветку переинициализации (см. [`native::start_game`](crate::native)),
//! а заставка продолжает показываться, пока игра не дойдёт до `GameMain`.

use crate::display::Frame;
use crate::render3d::{
    self, Rotation, Screen as Proj, INTRO_DIST, INTRO_SHIP_RADIUS, SHIP_COLOR,
};
use crate::shipdata::{self, Ship};

/// Состояние анимации заставки.
pub struct Splash {
    /// Индекс текущей модели в [`shipdata::SHIPS`] (переключается стрелками).
    index: usize,
    /// Текущая ориентация корабля (в точности `SKO4Intro` при запуске и
    /// после каждой смены корабля — как в оригинале).
    rot: Rotation,
    /// Счётчик кадров — для мигания приглашения «PRESS SPACE».
    tick: u64,
    /// Модель корабля.
    ship: Option<&'static Ship>,
}

impl Splash {
    pub fn new() -> Self {
        let ships = shipdata::SHIPS;
        let idx = ships
            .iter()
            .position(|s| s.name == "d_Cobra1")
            .unwrap_or(0);
        // Стартовый ракурс — ровно тот, что задан в оригинале таблицей
        // `SKO4Intro` (см. [`Rotation::intro`]).
        let rot = Rotation::intro();
        Splash {
            index: idx,
            rot,
            tick: 0,
            ship: Some(&ships[idx]),
        }
    }

    /// Переключить корабль влево/вправо (стрелки на заставке).
    ///
    /// Цикл повторяет оригинальный `SetShip4Intro`/`loc_71A2`: восемь
    /// боевых кораблей в том же порядке (Cobra → Python → ASP →
    /// Fer-de-Lance → Targon → Sidewinder → Krait → Viper), старт — Cobra.
    /// Как и в оригинале, смена корабля заново выставляет СКО из
    /// `SKO4Intro`, поэтому поворот начинается с исходного ракурса.
    pub fn cycle_ship(&mut self, dir: i32) {
        let step = if dir < 0 { -1 } else { 1 };
        let len = INTRO_CYCLE.len() as i32;
        // Текущая позиция в цикле (по имени модели).
        let cur = self
            .ship
            .and_then(|s| INTRO_CYCLE.iter().position(|&n| n == s.name))
            .unwrap_or(0) as i32;
        let next = (cur + step).rem_euclid(len) as usize;
        let name = INTRO_CYCLE[next];
        let idx = shipdata::SHIPS
            .iter()
            .position(|s| s.name == name)
            .unwrap_or(0);
        self.index = idx;
        self.ship = Some(&shipdata::SHIPS[idx]);
        self.rot = Rotation::intro();
    }

    /// Один кадр анимации: шаг оригинального интро-цикла, разнесённый по
    /// кадрам.
    ///
    /// Оригинал (`RotateCrdNSCO`) поворачивал СКО фиксированным шагом
    /// ≈ 10.4° вокруг постоянной оси мира ([`STEP_AXIS`]) и делал это
    /// раз в ~12 кадров ([`STEP_FRAMES`]) — анимация выходила
    /// «ступенчатой». Здесь тот же шаг применяется каждый кадр с делением
    /// на 12: угловая скорость сохранена (полный оборот ≈ 8 с), но
    /// движение плавное.
    pub fn tick(&mut self) {
        let step = Rotation::axis_angle(STEP_AXIS, STEP_ANGLE / STEP_FRAMES);
        self.rot = step.mul(&self.rot);
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
    /// Композиция повторяет оригинальный титульный экран Elite:
    ///
    /// * заголовок `---- E L I T E ----` — в первой строке, колонка 5
    ///   (как `PrnTitTxtY` в оригинале), шрифтом сетки ZX без увеличения;
    /// * в середине — вращающийся корабль (на самом широком ракурсе
    ///   занимает всю свободную полосу между надписями, заметно крупнее
    ///   прежнего; в остальных ракурсах он естественно сокращается, как
    ///   в оригинале);
    /// * внизу — приглашение `PRESS SPACE` (в оригинале на этой строке
    ///   `Load New Commander (Y/N)?`, а в режиме притяжения — `PRESS SPACE,
    ///   COMMANDER`).
    ///
    /// Никаких других надписей нет — ровно как в оригинале.
    ///
    /// Область `0..area_h` — вся композиция: на заставке снизу может быть
    /// показана нативная приборная панель (кокпит), и тогда заголовок с
    /// фигурой не должны под неё заходить.
    pub fn draw_in(&self, frame: &mut Frame, font: &[u8], area_h: i32) {
        const BG: u32 = 0xFF00_0000; // чёрный фон (непрозрачный)
        const FG: u32 = 0xFFD7_D7D7; // белый — заголовок
        const HINT: u32 = 0xFFD7_D7D7; // белый — приглашение
        let area_h = area_h.clamp(1, frame.height as i32);
        // Фон области (снизу может быть своя панель — её не трогаем).
        frame.fill_rect(0, 0, frame.width, area_h as usize, BG);
        // Масштаб сетки ZX 256x192, влезающий в область: им задаётся и размер
        // шрифта, и положение надписей (сетка оригинала — 32x24 знакоместа).
        let s = ((frame.width / 256).min(area_h as usize / 192)).max(1) as i32;
        let ox = (frame.width as i32 - 256 * s) / 2;
        // Заголовок — как в оригинале: первая строка, колонка 5.
        let title = "---- E L I T E ----";
        let title_y = 4 * s;
        frame.draw_text(font, ox + 40 * s, title_y, title, FG, s as usize);
        // Нижняя строка — приглашение нажать SPACE.
        let hint = "Press Space, Commander";
        let hint_y = area_h - 12 * s;
        frame.draw_text(font, ox + 28 * s, hint_y, hint, HINT, s as usize);
        // Вращающийся корабль (или запасная фигура, если модель не найдена) —
        // по центру свободной между надписями полосы, крупнее прежнего.
        let top = title_y + 8 * s;
        let band = (hint_y - top).max(0);
        let cx = ox + 128 * s;
        let cy = top + band / 2;
        let r = (band / 2).min(INTRO_SHIP_RADIUS * s).min(frame.width as i32 * 2 / 5);
        if let Some(ship) = self.ship {
            self.draw_ship(frame, ship, cx, cy, r);
        } else {
            self.draw_fallback(frame, cx, cy, r, SHIP_COLOR);
        }
    }

    /// Множитель подбирается один раз по **самому дальнему выносу вершин за
    /// оборот**: любой ракурс остаётся в квадрате ±`r` вокруг центра, то есть
    /// в отведённой полосе между надписями. При этом масштаб не пересчитывается
    /// по кадру (прежняя ошибка — «дышащий» корабль, который раздувался, когда
    /// ракурс сокращался).
    fn ship_scale(&self, ship: &Ship, cen: [f32; 3], dist: f32, r: i32) -> f32 {
        /// Сколько ракурсов за оборот перебирается при калибровке.
        const SAMPLES: usize = 72;
        let intro = Rotation::intro();
        // Калибровка идёт относительно центра стартового ракурса: к нему же
        // привязан постоянный сдвиг композиции ([`Splash::intro_screen_center`]).
        let (bx, by) = self.intro_screen_center(ship, cen, dist);
        // Максимальное отклонение вершины от этого центра по X или Y (при
        // k = 1), взятое по всем ракурсам оборота: любой ракурс должен
        // оставаться в квадрате ±r, то есть в отведённой полосе.
        let mut reach = 0.0f32;
        for i in 0..SAMPLES {
            let t = i as f32 * std::f32::consts::TAU / SAMPLES as f32;
            let rot = Rotation::axis_angle(STEP_AXIS, t).mul(&intro);
            for v in ship.vertices {
                let w = rot.apply([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ]);
                let z = dist + w[2];
                reach = reach
                    .max((w[0] / z - bx).abs())
                    .max((w[1] / z - by).abs());
            }
        }
        r as f32 / reach.max(1e-3)
    }

    /// Центр габарита проекции на стартовом ракурсе (при масштабе 1).
    ///
    /// Из-за перспективы видимый центр корабля не совпадает с началом
    /// координат модели (у асимметричных корпусов — на десятки пикселей).
    /// Оригинал печатает объект вокруг его позиции, поэтому корабль там
    /// тоже чуть «в стороне»; чтобы композиция заставки была ровной, этот
    /// сдвиг фиксируется один раз по стартовому ракурсу (при вращении он
    /// остаётся постоянным — как фиксированная позиция объекта в оригинале,
    /// а не покадровая подгонка под габарит).
    fn intro_screen_center(&self, ship: &Ship, cen: [f32; 3], dist: f32) -> (f32, f32) {
        let rot = Rotation::intro();
        let (mut mn, mut mx) = ([f32::MAX; 2], [f32::MIN; 2]);
        for v in ship.vertices {
            let w = rot.apply([
                v.x as f32 - cen[0],
                v.y as f32 - cen[1],
                v.z as f32 - cen[2],
            ]);
            let z = dist + w[2];
            for (i, c) in [w[0] / z, w[1] / z].iter().enumerate() {
                mn[i] = mn[i].min(*c);
                mx[i] = mx[i].max(*c);
            }
        }
        ((mn[0] + mx[0]) / 2.0, (mn[1] + mx[1]) / 2.0)
    }

    /// Нарисовать вращающийся корабль **каркасом** (рёбрами), как в оригинале.
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
    fn draw_ship(&self, frame: &mut Frame, ship: &Ship, cx: i32, cy: i32, r: i32) {
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

    /// Экранная проекция вершин (для тестов композиции).
    #[cfg(test)]
    fn project_ship(&self, ship: &Ship, cen: [f32; 3], pr: &Proj) -> Vec<(f32, f32, f32)> {
        ship.vertices
            .iter()
            .map(|v| {
                let w = self.rot.apply([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ]);
                let z = INTRO_DIST + w[2];
                pr.point(w[0], w[1], z)
            })
            .collect()
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

/// Шаг оригинального интро-цикла: суммарный угол поворота СКО за проход
/// (радианы). Измерен по эмулятору на 67 шагах за 800 кадров:
/// 10.1–11.3°, среднее ≈ 10.4°.
const STEP_ANGLE: f32 = 0.1816;

/// Ось шага [`STEP_ANGLE`] **в координатах мира** (нормирована).
///
/// Измерена по эмулятору: вектор поворота `M₂·M₁ᵀ` между соседними
/// изменениями СКО постоянен на всём интервале 800 кадров (среднее
/// (2.4, 6.6, 7.6)°, расхождение ±7° — округления Z80). Шаг — обычный
/// поворот на фиксированный угол вокруг фиксированной оси, поэтому ось
/// постоянна и в мире, и в кадре модели (описания сопряжены через
/// стартовую позу и дают одинаковую последовательность ракурсов).
/// Здесь выбрано мировое описание — левое умножение в [`Splash::tick`].
///
/// Ось близка к направлению взгляда и вертикали: корабль кувыркается
/// вокруг неё, как на карусели; нос остаётся развёрнут от наблюдателя.
const STEP_AXIS: [f32; 3] = [0.2303, 0.6356, 0.7369];

/// Сколько кадров оригиналу требуется на один шаг [`STEP_ANGLE`]
/// (измерено: шаг происходит раз в 11–13 кадров, среднее ≈ 12).
///
/// Шаг применяется не целиком раз в 12 кадров (как в оригинале), а
/// равными долями каждый кадр: угловая скорость сохраняется
/// (полный оборот ≈ 8 с), но движение получается плавным.
const STEP_FRAMES: f32 = 12.0;

/// Цикл кораблей на заставке — как в оригинале (`loc_71A2`: тип
/// `((n & 7) + 1)`, для типов < 6 прибавляется 10). Счётчик стартует
/// с значения, дающего Cobra; порядок — Cobra → Python → ASP →
/// Fer-de-Lance → Targon → Sidewinder → Krait → Viper.
const INTRO_CYCLE: [&str; 8] = [
    "d_Cobra1",
    "d_Python1",
    "d_ASP",
    "d_FerDeLance",
    "d_Targon",
    "dSidewinderH",
    "d_Krait",
    "d_Viper",
];

/// Совпадает ли геометрия двух объектов.
///
/// В таблице Кладова обычный и пиратский варианты Cobra/Python (а также два
/// астероида) хранятся как отдельные записи с одинаковыми вершинами, рёбрами
/// и гранями. Для проверок полезно знать, что это один корпус.
#[cfg(test)]
fn same_hull(a: &Ship, b: &Ship) -> bool {
    a.vertices == b.vertices && a.edges == b.edges && a.faces == b.faces
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render3d::{face_visibility, line_hidden, INTRO_SCO};

    /// Заставка: непрозрачный чёрный фон, белый заголовок, циановый
    /// корабль и жёлтое приглашение.
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
        // Лишних надписей нет: серый (подсказка меню) больше не рисуется.
        assert!(
            !f.pixels().iter().any(|&p| p == 0xFF90_9090),
            "на заставке остались лишние надписи"
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
    /// Проверяем область корабля (центр по центру полосы между надписями,
    /// радиус `SHIP_RADIUS_S*s`, `s = 5` при окне 1280x960): должна быть
    /// заметная доля «чернил», но она не должна приближаться к сплошной
    /// заливке.
    #[test]
    fn splash_ship_is_wireframe() {
        let font = vec![0u8; 0x300];
        let mut f = Frame::new(1280, 960);
        let st = Splash::new();
        st.draw(&mut f, &font);
        let (cx, cy, r) = (1280 / 2, 480, INTRO_SHIP_RADIUS * 5);
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

    /// Корабль на заставке крупнее прежнего и помещается между надписями.
    #[test]
    fn splash_ship_is_large_and_fits_texts() {
        let font = vec![0u8; 0x300];
        let mut f = Frame::new(1280, 960);
        let st = Splash::new();
        st.draw(&mut f, &font);
        // Полоса между заголовком и приглашением: корабль должен её занимать
        // почти целиком, но не залезать на строки.
        let s = 5;
        let title_bottom = 12 * s;
        let hint_top = 960 - 12 * s;
        let mut top = 960;
        let mut bottom = 0;
        for y in 0..960 {
            for x in 0..1280 {
                if f.pixels()[y * 1280 + x] == 0xFF00_FFFF {
                    top = top.min(y);
                    bottom = bottom.max(y);
                }
            }
        }
        assert!(top >= title_bottom, "корабль залез на заголовок: {top}");
        assert!(bottom <= hint_top, "корабль залез на приглашение: {bottom}");
        // Габарит корабля — больше прежних 2*52*s = 520 px по высоте.
        assert!(
            bottom - top > 52 * s,
            "корабль не увеличился: {bottom}..{top}"
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

    /// Переключение кораблей идёт по циклу оригинала: восемь боевых
    /// кораблей, старт — Cobra, порядок как в `loc_71A2`.
    #[test]
    fn splash_cycle_follows_original_order() {
        let mut st = Splash::new();
        // Старт — Cobra, как у оригинала (`ld a,79h`).
        assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
        let mut seen = Vec::new();
        for _ in 0..INTRO_CYCLE.len() {
            let name = st.ship.expect("нет корабля").name;
            assert!(
                INTRO_CYCLE.contains(&name),
                "корабль {name} вне цикла заставки"
            );
            assert!(!seen.contains(&name), "модель {name} показана дважды");
            seen.push(name);
            // Каждый переключатель возвращает исходный ракурс (как
            // `SetShip4Intro` в оригинале).
            st.tick();
            st.cycle_ship(1);
            let intro = Rotation::intro();
            for i in 0..3 {
                for j in 0..3 {
                    assert!(
                        (st.rot.m[i][j] - intro.m[i][j]).abs() < 1e-6,
                        "после смены корабля ракурс не сброшен"
                    );
                }
            }
        }
        assert_eq!(seen.len(), INTRO_CYCLE.len(), "круг переключения не замкнулся");
        assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
        // Назад с Cobra — Viper (последний в цикле).
        st.cycle_ship(-1);
        assert_eq!(st.ship.expect("нет корабля").name, "d_Viper");
        // Вперёд с Viper — снова Cobra.
        st.cycle_ship(1);
        assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
        // Все модели цикла есть в таблице и представляют разные корпуса.
        for &name in &INTRO_CYCLE {
            shipdata::find(name).expect("нет модели из цикла");
        }
        for (i, &a) in INTRO_CYCLE.iter().enumerate() {
            for &b in &INTRO_CYCLE[i + 1..] {
                assert!(
                    !same_hull(shipdata::find(a).unwrap(), shipdata::find(b).unwrap()),
                    "{a} и {b} — один и тот же корпус"
                );
            }
        }
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

    /// Шаг вращения повторяет оригинал: за `STEP_FRAMES` кадров накапливается
    /// ровно один оригинальный шаг [`STEP_ANGLE`] вокруг оси [`STEP_AXIS`]
    /// (измерено по эмулятору: ≈ 10.4° за ~12 кадров, оборот ≈ 8 с).
    #[test]
    fn splash_rotation_matches_original_step() {
        let mut st = Splash::new();
        let start = st.rot;
        for _ in 0..STEP_FRAMES as usize {
            st.tick();
        }
        let a = angle_between(&start, &st.rot);
        assert!(
            (a - STEP_ANGLE).abs() < 1e-3,
            "за {} кадров повёрнуто на {}рад вместо {}рад",
            STEP_FRAMES,
            a,
            STEP_ANGLE
        );
        // Шаг делается в мире (левое умножение): R = rot·startᵀ. Ось
        // поворота — из антисимметричной части: [n]·sinθ = (R − Rᵀ)/2.
        let mut r = [[0.0f32; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    r[i][j] += st.rot.m[i][k] * start.m[j][k];
                }
            }
        }
        let sn = 2.0 * a.sin();
        let v = [
            (r[2][1] - r[1][2]) / sn,
            (r[0][2] - r[2][0]) / sn,
            (r[1][0] - r[0][1]) / sn,
        ];
        for i in 0..3 {
            // Допуск 1e-2, а не машинный: стартовая СКО — 7-битные байты
            // оригинала, её триада лишь приближённо ортогональна (≈1%),
            // и это же отклонение вносит погрешность в извлечённую ось.
            assert!(
                (v[i] - STEP_AXIS[i]).abs() < 1e-2,
                "ось шага не совпала с STEP_AXIS: {v:?}"
            );
        }
        // Скорость та же, что у оригинала: полный оборот ≈ 360°/10.4°·12
        // кадров ≈ 415 кадров ≈ 8.3 с при 50 Гц.
        let mut st = Splash::new();
        let start = st.rot;
        let mut per_turn = 0u32;
        while per_turn < 2000 && angle_between(&start, &st.rot) < std::f32::consts::PI {
            st.tick();
            per_turn += 1;
        }
        // Мерится полуповорот (за PI повторений точность выше, чем за
        // полный оборот): оригинальный оборот ≈ 400 кадров, то есть
        // полуповорот ≈ 200 кадров.
        let half = per_turn as f32;
        assert!(
            (190.0..230.0).contains(&half),
            "полуповорот за {half} кадров — скорость не как у оригинала (~200)"
        );
    }

    /// «Карусель» оригинала: ось вращения близка к направлению взгляда,
    /// поэтому нос корабля (+Z модели) остаётся развёрнут от наблюдателя
    /// на всём обороте — как в оригинальном интро.
    #[test]
    fn splash_nose_keeps_pointing_away() {
        let mut st = Splash::new();
        let intro = Rotation::intro();
        assert!(
            intro.apply([0.0, 0.0, 1.0])[2] > 0.0,
            "в стартовой позе нос должен смотреть вглубь экрана"
        );
        for n in 0..(10 * 50) {
            st.tick();
            let nose = st.rot.apply([0.0, 0.0, 1.0]);
            assert!(
                nose[2] > 0.0,
                "кадр {n}: нос развёрнут к наблюдателю ({nose:?}) — это уже не карусель оригинала"
            );
        }
    }

    /// Матрица поворота не «утекает»: отклонение от ортогональности не
    /// растёт со временем. Регрессия к ошибке с неединичной осью в
    /// `axis_angle` (там матрица расходилась с каждым кадром).
    ///
    /// Сама стартовая СКО лишь приближённо ортогональна — это 7-битные
    /// байты оригинала, поэтому сравнивается не абсолютное значение, а
    /// именно отсутствие роста.
    #[test]
    fn splash_rotation_matrix_stays_orthonormal() {
        /// Максимальное отклонение от ортогональной матрицы.
        fn dev(m: [[f32; 3]; 3]) -> f32 {
            let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            // Столбцы, а не строки: при левом умножении (rot = A·rot)
            // сохраняется именно MᵀM, то есть ортогональность столбцов.
            let cols = [
                [m[0][0], m[1][0], m[2][0]],
                [m[0][1], m[1][1], m[2][1]],
                [m[0][2], m[1][2], m[2][2]],
            ];
            let mut d = 0.0f32;
            for i in 0..3 {
                d = d.max((dot(cols[i], cols[i]) - 1.0).abs());
                d = d.max(dot(cols[i], cols[(i + 1) % 3]).abs());
            }
            let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
            d.max((det - 1.0).abs())
        }
        let mut st = Splash::new();
        let d0 = dev(st.rot.m);
        assert!(d0 < 2e-2, "стартовая СКО слишком неортогональна: {d0}");
        for n in 0..(20 * 50) {
            st.tick();
            if n % 100 == 0 {
                let d = dev(st.rot.m);
                assert!(
                    d <= d0 + 1e-3,
                    "кадр {}: отклонение выросло {d0} → {d} — матрица расходится",
                    n + 1
                );
            }
        }
    }



    /// Главная ось силуэта корабля по пикселям кадра (градусы).
    ///
    /// Меряется так же, как силуэт оригинала в
    /// `dumptitle::dump_intro_silhouette`: ковариация точек силуэта, угол
    /// главной оси (определён с точностью до 180°).
    fn silhouette_axis(frame: &Frame) -> f64 {
        let (w, h) = (frame.width, frame.height);
        let (mut n, mut sx, mut sy) = (0.0f64, 0.0f64, 0.0f64);
        for y in 0..h {
            for x in 0..w {
                if frame.pixels()[y * w + x] == SHIP_COLOR {
                    n += 1.0;
                    sx += x as f64;
                    sy += y as f64;
                }
            }
        }
        assert!(n > 100.0, "корабль не нарисован: {n}");
        let (cx, cy) = (sx / n, sy / n);
        let (mut sxx, mut syy, mut sxy) = (0.0f64, 0.0f64, 0.0f64);
        for y in 0..h {
            for x in 0..w {
                if frame.pixels()[y * w + x] == SHIP_COLOR {
                    let (dx, dy) = (x as f64 - cx, y as f64 - cy);
                    sxx += dx * dx;
                    syy += dy * dy;
                    sxy += dx * dy;
                }
            }
        }
        let (sxx, syy, sxy) = (sxx / n, syy / n, sxy / n);
        0.5 * (2.0 * sxy).atan2(sxx - syy).to_degrees()
    }

    /// Пиксельные габариты корабля в кадре (ширина, высота).
    fn ship_bbox(frame: &Frame) -> (i32, i32) {
        let (w, h) = (frame.width, frame.height);
        let (mut x0, mut x1, mut y0, mut y1) = (w as i32, -1, h as i32, -1);
        for y in 0..h {
            for x in 0..w {
                if frame.pixels()[y * w + x] == SHIP_COLOR {
                    x0 = x0.min(x as i32);
                    x1 = x1.max(x as i32);
                    y0 = y0.min(y as i32);
                    y1 = y1.max(y as i32);
                }
            }
        }
        assert!(x1 >= 0, "корабль не нарисован");
        (x1 - x0 + 1, y1 - y0 + 1)
    }

    /// Корабль не «дышит»: масштаб постоянный, меняется только ракурс.
    ///
    /// Прежде габарит проекции подгонялся под `2r` **каждый кадр**, поэтому
    /// при сокращении ракурса фигура раздувалась — корабль пульсировал.
    /// Теперь масштаб калибруется один раз по самому широкому ракурсу за
    /// оборот (он занимает ровно `2r`), остальные ракурсы меньше — это
    /// естественное сокращение, как у оригинала с фиксированным расстоянием
    /// до объекта. Тест проверяет, что габарит заметно меняется (то есть
    /// нормировки по кадру нет) и при этом не вылезает на надписи.
    #[test]
    fn splash_ship_size_does_not_breathe() {
        let font = vec![0u8; 0x300];
        let mut st = Splash::new();
        let mut f = Frame::new(1280, 960);
        // Полный оборот (≈415 тиков) с шагом 3 тика.
        let (mut dmin, mut dmax) = (i32::MAX, 0);
        let (mut top, mut bottom) = (960, 0);
        for _ in 0..140 {
            f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
            st.draw(&mut f, &font);
            let (bw, bh) = ship_bbox(&f);
            let d = bw.max(bh);
            dmin = dmin.min(d);
            dmax = dmax.max(d);
            // Вертикальные габариты — для проверки, что надписи не задеты.
            for y in 0..960usize {
                for x in 0..1280usize {
                    if f.pixels()[y * 1280 + x] == SHIP_COLOR {
                        top = top.min(y as i32);
                        bottom = bottom.max(y as i32);
                    }
                }
            }
            for _ in 0..3 {
                st.tick();
            }
        }
        let s = 5; // масштаб сетки ZX при окне 1280x960
        assert!(
            top >= 12 * s && bottom <= 960 - 12 * s,
            "на широком ракурсе корабль залез на надписи: {top}..{bottom}"
        );
        assert!(
            dmax * 10 > dmin * 11,
            "габарит почти не меняется ({dmin}..{dmax}) — масштаб подгоняется по кадру"
        );
    }

    /// Наклон силуэта на экране — как в оригинале (и направление вращения).
    ///
    /// Оригинал (замер по эмулятору: кадры 16/28/40/52 — это 1..4-й шаг
    /// поворота СКО) даёт главную ось силуэта 8.75°, −11.56°, −29.70°,
    /// −48.13°, то есть она **убывает**. Порт вращается плавно, поэтому
    /// берутся те же позы (тики 11/23/35/47): значения должны идти в ту же
    /// сторону и попадать в допуск на фазу (оригинал шагает «ступенькой»,
    /// порт — непрерывно, отсюда разброс в несколько градусов).
    ///
    /// Тест ловит ошибку знака вертикали: при печати `+Y` изображение
    /// зеркалится по вертикали и ось растёт (+13°…+51°) — визуально
    /// вращение идёт в обратную сторону, хотя СКО поворачивается верно.
    #[test]
    fn splash_silhouette_turns_like_original() {
        let font = vec![0u8; 0x300];
        let mut st = Splash::new();
        let mut f = Frame::new(1280, 960);
        let mut prev = f64::NAN;
        for (tick, expected) in [(11u64, 8.75f64), (23, -11.56), (35, -29.70), (47, -48.13)] {
            while st.tick < tick {
                st.tick();
            }
            f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
            st.draw(&mut f, &font);
            let mut ang = silhouette_axis(&f);
            // Угол оси определён с точностью до 180°.
            while ang - expected > 90.0 {
                ang -= 180.0;
            }
            while ang - expected < -90.0 {
                ang += 180.0;
            }
            assert!(
                (ang - expected).abs() < 12.0,
                "тик {tick}: ось силуэта {ang:.2}°, в оригинале {expected:.2}°"
            );
            if !prev.is_nan() {
                assert!(
                    ang < prev,
                    "ось силуэта не убывает, как в оригинале: {prev:.2}° → {ang:.2}°"
                );
            }
            prev = ang;
        }
    }



    /// Центр габаритов модели — так же, как в [`Splash::draw_ship`].
    fn centered(ship: &Ship) -> [f32; 3] {
        let (mut mn, mut mx) = ([i32::MAX; 3], [i32::MIN; 3]);
        for v in ship.vertices {
            for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
                mn[k] = mn[k].min(*c as i32);
                mx[k] = mx[k].max(*c as i32);
            }
        }
        [
            (mn[0] + mx[0]) as f32 / 2.0,
            (mn[1] + mx[1]) as f32 / 2.0,
            (mn[2] + mx[2]) as f32 / 2.0,
        ]
    }

    /// Эталонный куб: выпуклый корпус. Здесь проверка видимости граней
    /// может быть точной (нормали куба заданы ровно), поэтому известно
    /// множество заведомо видимых рёбер. Удаление невидимых линий не должно
    /// убирать с них ни одного пикселя.
    ///
    /// Регрессия на слишком агрессивное удаление: раньше на гранях, почти
    /// параллельных лучу зрения (глубина меняется на проценты за пиксель),
    /// видимые линии пропадали.
    #[test]
    fn convex_cube_keeps_visible_edges() {
        let ship = cube_ship();
        let cen = centered(ship);
        for step in 0..24 {
            let mut st = Splash::new();
            for _ in 0..step * 5 {
                st.tick();
            }
            // Вершины в координатах камеры и точная видимость граней:
            // грань видна, если её нормаль смотрит на камеру.
            let cam: Vec<[f32; 3]> = ship
                .vertices
                .iter()
                .map(|v| {
                    let w = st.rot.apply([
                        v.x as f32 - cen[0],
                        v.y as f32 - cen[1],
                        v.z as f32 - cen[2],
                    ]);
                    [w[0], w[1], INTRO_DIST + w[2]]
                })
                .collect();
            let vis: Vec<bool> = (0..ship.faces.len())
                .map(|fi| {
                    let f = &ship.faces[fi];
                    let n = st.rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
                    let (mut c, mut cnt) = ([0.0f32; 3], 0.0f32);
                    for e in ship.edges {
                        if (e.faces & 0x0F) as usize == fi || (e.faces >> 4) as usize == fi {
                            for i in [e.v[0] as usize, e.v[1] as usize] {
                                for (j, cj) in c.iter_mut().enumerate() {
                                    *cj += cam[i][j];
                                }
                                cnt += 1.0;
                            }
                        }
                    }
                    n[0] * c[0] + n[1] * c[1] + n[2] * c[2] < 0.0 && cnt > 0.0
                })
                .collect();
            // Эталон: только заведомо видимые рёбра, без проверки глубины.
            let k = st.ship_scale(ship, cen, INTRO_DIST, 240);
            let (bx, by) = st.intro_screen_center(ship, cen, INTRO_DIST);
            let pr = Proj {
                cx: 300.0,
                cy: 300.0,
                k,
                bx,
                by,
            };
            let scr = st.project_ship(ship, cen, &pr);
            let mut reference = Frame::new(600, 600);
            for e in ship.edges {
                let (a, b) = (e.v[0] as usize, e.v[1] as usize);
                let seen = |f: usize| f == 0xF || vis.get(f).copied().unwrap_or(false);
                if seen((e.faces & 0x0F) as usize) || seen((e.faces >> 4) as usize) {
                    line_hidden(&mut reference, &[], &scr, scr[a], scr[b], SHIP_COLOR);
                }
            }
            let mut f = Frame::new(600, 600);
            st.draw_ship(&mut f, ship, 300, 300, 240);
            let drawn: Vec<bool> = f.pixels().iter().map(|&p| p == SHIP_COLOR).collect();
            let (mut total, mut lost) = (0, 0);
            for (i, &p) in reference.pixels().iter().enumerate() {
                if p == SHIP_COLOR {
                    total += 1;
                    if !drawn[i] {
                        lost += 1;
                    }
                }
            }
            assert!(total > 0, "шаг {step}: куб не нарисован");
            assert_eq!(
                lost, 0,
                "шаг {step}: удаление линий убрало {lost} из {total} пикселей видимых рёбер"
            );
        }
    }

    /// Куб как модель корабля для [`convex_cube_keeps_visible_edges`].
    fn cube_ship() -> &'static Ship {
        use crate::shipdata::{Edge, Face, Ship, Vertex};
        static VERTS: [Vertex; 8] = [
            Vertex { x: -50, y: -50, z: -50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: 50, y: -50, z: -50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: 50, y: 50, z: -50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: -50, y: 50, z: -50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: -50, y: -50, z: 50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: 50, y: -50, z: 50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: 50, y: 50, z: 50, lod: 0x1F, faces: [0, 0] },
            Vertex { x: -50, y: 50, z: 50, lod: 0x1F, faces: [0, 0] },
        ];
        static FACES: [Face; 6] = [
            Face { nx: 100, ny: 0, nz: 0, flags: 0x1F },
            Face { nx: -100, ny: 0, nz: 0, flags: 0x1F },
            Face { nx: 0, ny: 100, nz: 0, flags: 0x1F },
            Face { nx: 0, ny: -100, nz: 0, flags: 0x1F },
            Face { nx: 0, ny: 0, nz: 100, flags: 0x1F },
            Face { nx: 0, ny: 0, nz: -100, flags: 0x1F },
        ];
        static EDGES: [Edge; 12] = [
            Edge { lod: 0x1F, v: [0, 1], faces: 5 | (3 << 4) },
            Edge { lod: 0x1F, v: [1, 2], faces: 5 | (0 << 4) },
            Edge { lod: 0x1F, v: [2, 3], faces: 5 | (2 << 4) },
            Edge { lod: 0x1F, v: [3, 0], faces: 5 | (1 << 4) },
            Edge { lod: 0x1F, v: [4, 5], faces: 4 | (3 << 4) },
            Edge { lod: 0x1F, v: [5, 6], faces: 4 | (0 << 4) },
            Edge { lod: 0x1F, v: [6, 7], faces: 4 | (2 << 4) },
            Edge { lod: 0x1F, v: [7, 4], faces: 4 | (1 << 4) },
            Edge { lod: 0x1F, v: [0, 4], faces: 3 | (1 << 4) },
            Edge { lod: 0x1F, v: [1, 5], faces: 3 | (0 << 4) },
            Edge { lod: 0x1F, v: [2, 6], faces: 2 | (0 << 4) },
            Edge { lod: 0x1F, v: [3, 7], faces: 2 | (1 << 4) },
        ];
        static SHIP: Ship = Ship {
            name: "cube", addr: 0, category: 0,
            vertices: &VERTS, edges: &EDGES, faces: &FACES,
            stats: unsafe { std::mem::zeroed() },
        };
        &SHIP
    }

    /// Дамп кадров анимации заставки (`ELITE_DUMP_SPLASH_FRAMES=prefix` →
    /// `prefix_<номер>.ppm`, шаг 12 тиков = один шаг оригинала).
    ///
    /// Шрифт берётся из ПЗУ `zx_bin/48.rom`, чтобы кадры были с надписями.
    #[test]
    #[ignore]
    fn dump_splash_frames() {
        let Ok(prefix) = std::env::var("ELITE_DUMP_SPLASH_FRAMES") else {
            return;
        };
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).unwrap();
        let font = &rom[0x3D00..0x3D00 + 0x300];
        let mut st = Splash::new();
        let mut f = Frame::new(1280, 960);
        for i in 0..8 {
            f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
            st.draw(&mut f, font);
            write_ppm(&format!("{prefix}_{i}.ppm"), f.pixels(), 1280, 960);
            for _ in 0..12 {
                st.tick();
            }
        }
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
