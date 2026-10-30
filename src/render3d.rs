//! Общий рендер объектов игры в пространстве.
//!
//! Здесь собрана отрисовка моделей из [`crate::shipdata`] в том же стиле,
//! что и оригинальный Z80-рендер (`CalcVisibleVerges` + `PrintObject`):
//! видимость граней определяется повёрнутой нормалью, обращённые к камере
//! грани заливаются непрозрачным тёмно-серым, поверх них рисуются ярко-белые
//! рёбра, а линии, закрытые передними гранями, вырезаются.
//!
//! Модуль используется и нативной заставкой (`src/splash.rs`), и нативным
//! движком полёта (`src/flight.rs`): заставка задаёт свою калибровку масштаба
//! и сдвиг композиции, полёт — общую перспективную камеру и положение
//! объекта в координатах вида.
//!
//! Конвенция координат — как в оригинале: X вправо, Y вверх, Z вглубь
//! экрана (камера смотрит вдоль +Z), строка экрана растёт вниз.

use crate::display::Frame;
use crate::shipdata::Ship;

/// Цвет рёбер корабля — ярко-белый.
pub const SHIP_COLOR: u32 = 0xFFFF_FFFF;

/// Цвет заливки видимых граней — тёмно-серый.
///
/// Грани непрозрачные: через корпус не должны просвечивать солнце, планета
/// и звёзды. Цвет достаточно тёмный, чтобы белые рёбра читались на нём.
pub const SHIP_FACE_COLOR: u32 = 0xFF20_2832;

/// Радиус, в который должна вписываться модель на заставке (в «пикселях ZX»).
///
/// Используется только заставкой; вынесено сюда вместе с общей геометрией,
/// чтобы не дублировать константы в двух рендерах.
pub const INTRO_SHIP_RADIUS: i32 = 84;

/// Расстояние до интро-объекта в модели (оригинал держит его на Z = `0x200`).
pub const INTRO_DIST: f32 = 512.0;

/// Начальная ориентация интро-объекта — таблица `SKO4Intro` оригинала.
///
/// СКО объекта в памяти Z80 хранится тремя тройками знаковых байт (старший
/// бит — знак, модуль — в младших семи): по адресам `0x6163` — ось Z модели,
/// `0x6169` — ось Y, `0x616F` — ось X. Таблица `SKO4Intro` содержит их
/// именно в таком порядке: `58 44 41 | 98 64 CB | 5A A7 D1`, то есть
/// Z = (88, 68, 65), Y = (−24, 100, −75), X = (90, −39, −81).
///
/// Здесь тройки записаны в порядке «ось X, ось Y, ось Z» модели — так их
/// трактует оригинальный `CalcMatrixRotate`, собирающий из этих байт матрицу
/// поворота (её строки — оси модели в мире).
pub const INTRO_SCO: [[f32; 3]; 3] = [
    [90.0, -39.0, -81.0],  // ось X модели
    [-24.0, 100.0, -75.0], // ось Y модели
    [88.0, 68.0, 65.0],    // ось Z модели
];

/// Матрица поворота «модель → вид» (3x3), как `MatrixRotate` оригинала.
///
/// `m[i][j]` — вклад оси `j` модели в ось `i` вида; столбцы матрицы — оси
/// модели в координатах вида. Оси вида как в оригинале: X вправо, Y вверх,
/// Z вглубь экрана (камера смотрит вдоль +Z).
#[derive(Clone, Copy, Debug)]
pub struct Rotation {
    pub(crate) m: [[f32; 3]; 3],
}

impl Rotation {
    /// Единичная матрица (модель уже в координатах вида).
    pub fn identity() -> Self {
        Rotation {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    /// Исходный ракурс интро-объекта из `SKO4Intro` (столбцы нормируются).
    pub fn intro() -> Self {
        let mut m = [[0.0f32; 3]; 3];
        for (j, c) in INTRO_SCO.iter().enumerate() {
            let n = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
            for (i, row) in m.iter_mut().enumerate() {
                row[j] = c[i] / n;
            }
        }
        Rotation { m }
    }

    /// Матрица из трёх осей модели (X, Y, Z), заданных в координатах вида.
    ///
    /// Оси должны быть ортонормированными: это готовая СКО объекта из памяти
    /// Z80, а не произвольный поворот.
    pub fn from_axes(x: [f32; 3], y: [f32; 3], z: [f32; 3]) -> Self {
        Rotation {
            m: [
                [x[0], y[0], z[0]],
                [x[1], y[1], z[1]],
                [x[2], y[2], z[2]],
            ],
        }
    }

    /// Поворот вокруг `axis` на `angle` (матрица в той же конвенции).
    ///
    /// Ось нормируется: константы задаются округлёнными числами, а
    /// неединичная ось дала бы «утекающую» (не ортогональную) матрицу.
    pub fn axis_angle(axis: [f32; 3], angle: f32) -> Self {
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let [x, y, z] = [axis[0] / len, axis[1] / len, axis[2] / len];
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
    pub fn mul(&self, other: &Rotation) -> Self {
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

    /// Транспонированная матрица (обратный поворот для ортогональной СКО).
    pub fn transpose(&self) -> Self {
        let mut m = [[0.0f32; 3]; 3];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                *v = self.m[j][i];
            }
        }
        Rotation { m }
    }

    /// Перевести точку или вектор модели в координаты вида.
    pub fn apply(&self, v: [f32; 3]) -> [f32; 3] {
        std::array::from_fn(|i| {
            self.m[i][0] * v[0] + self.m[i][1] * v[1] + self.m[i][2] * v[2]
        })
    }
}

/// Экранная проекция: `px = cx + (X/Z - bx)*k`, `py = cy - (Y/Z - by)*k`.
///
/// Так печатает оригинал (`CalcPrnCrdVertex`), только с постоянным сдвигом
/// композиции `(bx, by)` и масштабом `k`. Глубина — `Z` в координатах камеры.
#[derive(Clone, Copy, Debug)]
pub struct Screen {
    pub cx: f32,
    pub cy: f32,
    pub k: f32,
    pub bx: f32,
    pub by: f32,
}

impl Screen {
    /// Экранные координаты точки `(x, y)` на глубине `z` и `1/z`.
    pub fn point(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        (
            self.cx + (x / z - self.bx) * self.k,
            self.cy - (y / z - self.by) * self.k,
            1.0 / z,
        )
    }
}

/// Центр габаритов модели — середина bounding box.
pub fn model_center(ship: &Ship) -> [f32; 3] {
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
pub fn face_visibility(ship: &Ship, rot: &Rotation) -> Vec<bool> {
    ship.faces
        .iter()
        .map(|f| {
            let n = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
            n[2] < 0.0
        })
        .collect()
}

/// Нарисовать объект в координатах вида: тёмные грани и рёбра цветом `color`.
///
/// `pos` — положение начала координат модели в системе камеры (X вправо,
/// Y вверх, Z вглубь экрана). `rot` — ориентация модели относительно камеры.
/// `screen` — перспективная проекция. Объекты целиком за камерой или
/// пересекающие её ближнюю плоскость не рисуются.
pub fn draw_ship(
    frame: &mut Frame,
    ship: &Ship,
    rot: &Rotation,
    pos: [f32; 3],
    screen: &Screen,
    color: u32,
) {
    if ship.vertices.is_empty() || pos[2] <= NEAR {
        return;
    }
    let cen = model_center(ship);
    // Проекция вершин: если объект задел ближнюю плоскость, пропускаем его
    // целиком — линии через камеру всё равно нельзя нарисовать корректно.
    let mut scr = Vec::with_capacity(ship.vertices.len());
    for v in ship.vertices {
        let w = rot.apply([
            v.x as f32 - cen[0],
            v.y as f32 - cen[1],
            v.z as f32 - cen[2],
        ]);
        let (x, y, z) = (w[0] + pos[0], w[1] + pos[1], w[2] + pos[2]);
        if z <= NEAR {
            return;
        }
        scr.push(screen.point(x, y, z));
    }
    let vis = face_visibility(ship, rot);
    let shapes = face_shapes(ship, &scr, &vis);
    // Сначала непрозрачные грани, затем рёбра поверх них.
    fill_faces(frame, &shapes, &scr);
    let occl = occluders(&shapes);
    for e in ship.edges {
        let (a, b) = (e.v[0] as usize, e.v[1] as usize);
        if a >= scr.len() || b >= scr.len() {
            continue;
        }
        let f0 = (e.faces & 0x0F) as usize;
        let f1 = (e.faces >> 4) as usize;
        let seen = |f: usize| f == 0xF || vis.get(f).copied().unwrap_or(false);
        if seen(f0) || seen(f1) {
            line_hidden(frame, &occl, &scr, scr[a], scr[b], color);
        }
    }
}

/// Контур обращённой к камере грани в экранных координатах.
struct FaceShape {
    /// Рёбра контура в индексах вершин модели (петель может быть несколько,
    /// например вырезы двигателей).
    edges: Vec<(usize, usize)>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    /// Плоскость `1/z = aa*x + bb*y + cc` и её ошибка по вершинам.
    plane: Option<(f32, f32, f32)>,
    plane_error: f32,
}

/// Собрать контуры всех видимых граней.
fn face_shapes(ship: &Ship, scr: &[(f32, f32, f32)], vis: &[bool]) -> Vec<FaceShape> {
    let mut face_edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); ship.faces.len()];
    for e in ship.edges {
        let (a, b) = (e.v[0] as usize, e.v[1] as usize);
        if a >= scr.len() || b >= scr.len() {
            continue;
        }
        let f0 = e.faces & 0x0F;
        let f1 = e.faces >> 4;
        // 0xF — признак «грани нет»; одинаковые полубайты не должны
        // добавлять ребро в контур дважды (иначе правило «чёт-нечёт»
        // посчитает его дважды и оставит дырку).
        if f0 != 0xF {
            if let Some(list) = face_edges.get_mut(f0 as usize) {
                list.push((a, b));
            }
        }
        if f1 != f0 && f1 != 0xF {
            if let Some(list) = face_edges.get_mut(f1 as usize) {
                list.push((a, b));
            }
        }
    }
    let mut out = Vec::new();
    for (fi, _) in ship.faces.iter().enumerate() {
        if !vis.get(fi).copied().unwrap_or(false) {
            continue;
        }
        let edges = &face_edges[fi];
        if edges.len() < 3 {
            continue;
        }
        let mut vs: Vec<usize> = edges.iter().flat_map(|&(a, b)| [a, b]).collect();
        vs.sort_unstable();
        vs.dedup();
        if vs.len() < 3 {
            continue;
        }
        let pts: Vec<(f32, f32, f32)> = vs.iter().map(|&i| scr[i]).collect();
        let plane = fit_plane(&pts);
        let plane_error = match plane {
            Some((aa, bb, cc)) => pts.iter().fold(0.0f32, |e, &(x, y, iz)| {
                e.max((aa * x + bb * y + cc - iz).abs() / iz)
            }),
            None => f32::MAX,
        };
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(i, j) in edges {
            for k in [i, j] {
                x0 = x0.min(scr[k].0);
                x1 = x1.max(scr[k].0);
                y0 = y0.min(scr[k].1);
                y1 = y1.max(scr[k].1);
            }
        }
        out.push(FaceShape {
            edges: edges.clone(),
            x0,
            x1,
            y0,
            y1,
            plane,
            plane_error,
        });
    }
    out
}

/// Залить видимые грани непрозрачным цветом.
///
/// Каждый контур заполняется построчно по правилу «чёт-нечёт»: это корректно
/// и для нескольких петель (вырезы двигателей), и для невыпуклых граней.
/// Все грани заливаются одним цветом, поэтому порядок заливки не важен —
/// перекрывающиеся видимые грани всё равно дают один и тот же тёмный тон.
fn fill_faces(frame: &mut Frame, shapes: &[FaceShape], scr: &[(f32, f32, f32)]) {
    let w = frame.width as i32;
    let h = frame.height as i32;
    if w == 0 || h == 0 {
        return;
    }
    for shape in shapes {
        let y_lo = ((shape.y0 - 0.5).ceil() as i32).max(0);
        let y_hi = ((shape.y1 - 0.5).ceil() as i32 - 1).min(h - 1);
        if y_lo > y_hi {
            continue;
        }
        let mut xs: Vec<f32> = Vec::with_capacity(shape.edges.len());
        for y in y_lo..=y_hi {
            let yc = y as f32 + 0.5;
            xs.clear();
            for &(i, j) in &shape.edges {
                let (xi, yi, _) = scr[i];
                let (xj, yj, _) = scr[j];
                if (yi <= yc) != (yj <= yc) {
                    xs.push(xi + (yc - yi) / (yj - yi) * (xj - xi));
                }
            }
            xs.sort_by(f32::total_cmp);
            for pair in xs.chunks_exact(2) {
                let x_lo = ((pair[0] - 0.5).ceil() as i32).max(0);
                let x_hi = ((pair[1] - 0.5).ceil() as i32 - 1).min(w - 1);
                if x_lo <= x_hi {
                    frame.hline(x_lo, y, (x_hi - x_lo + 1) as usize, SHIP_FACE_COLOR);
                }
            }
        }
    }
}

/// Ближняя плоскость отсечения в координатах камеры.
const NEAR: f32 = 0.1;

/// Относительный допуск по глубине при удалении невидимых линий.
///
/// Плоскости граней восстановлены по нормалям из ROM; у части граней
/// вершины отклоняются от своей плоскости на несколько единиц модели,
/// поэтому линия, лежащая **на**
/// передней поверхности, не должна пропадать из-за одной лишь погрешности.
/// Допуск 2 % глубины — это ≈ 10 единиц модели на дистанции интро, то есть
/// заметно меньше перепада между ближней и дальней сторонами корпуса.
const DEPTH_EPS: f32 = 0.02;

/// Предельная относительная ошибка плоскости грани по её вершинам.
///
/// Нормали в данных грубее геометрии, а глубину грани мы берём из её
/// плоскости. У граней, почти параллельных лучу зрения, малейший наклон
/// нормали даёт проценты глубины; такие грани не учитываются при удалении
/// невидимых линий — лучше оставить лишнюю линию, чем закрыть видимую.
const MAX_PLANE_ERROR: f32 = 0.01;

/// Грани-окклюдеры: обращённые к камере грани с экранным контуром и
/// уравнением своей плоскости.
///
/// В данных модели у грани есть только нормаль, а её контур собирается
/// из рёбер, у которых эта грань указана в полубайте. Сама нормаль грубее
/// геометрии, поэтому плоскость для глубины строится по вершинам грани
/// (см. [`fit_plane`]). Удаление невидимых линий идёт точной проверкой
/// «точка ребра против плоскости грани в этой же точке экрана»: у граней,
/// почти параллельных лучу зрения, глубина меняется на проценты за
/// пиксель, и буфер с грубой сеткой закрывал бы видимые линии.
fn occluders(shapes: &[FaceShape]) -> Vec<Occluder> {
    let mut out = Vec::new();
    for shape in shapes {
        let Some((aa, bb, cc)) = shape.plane else {
            continue;
        };
        // Точность важнее полноты: лучше оставить лишнюю линию, чем
        // закрыть видимую.
        if shape.plane_error > MAX_PLANE_ERROR {
            continue;
        }
        out.push(Occluder {
            edges: shape.edges.clone(),
            x0: shape.x0,
            x1: shape.x1,
            y0: shape.y0,
            y1: shape.y1,
            aa,
            bb,
            cc,
        });
    }
    out
}

/// Плоскость `1/z = aa*x + bb*y + cc`, наименее уклоняющаяся от точек.
///
/// Строится по самим вершинам грани (наименьшие квадраты в экранных
/// координатах): записанные в модели нормали грубее геометрии, а для
/// удаления невидимых линий важна именно глубина. Координаты
/// центрируются, чтобы нормальные уравнения были обусловлены.
fn fit_plane(pts: &[(f32, f32, f32)]) -> Option<(f32, f32, f32)> {
    if pts.len() < 3 {
        return None;
    }
    let n = pts.len() as f32;
    let (mut mx, mut my, mut mi) = (0.0f32, 0.0f32, 0.0f32);
    for &(x, y, i) in pts {
        mx += x;
        my += y;
        mi += i;
    }
    mx /= n;
    my /= n;
    mi /= n;
    // Нормальные уравнения для `a*dx + b*dy + c = di`.
    let (mut sxx, mut sxy, mut sx) = (0.0f32, 0.0f32, 0.0f32);
    let (mut syy, mut sy) = (0.0f32, 0.0f32);
    let (mut bx, mut by, mut bi) = (0.0f32, 0.0f32, 0.0f32);
    for &(x, y, i) in pts {
        let (dx, dy, di) = (x - mx, y - my, i - mi);
        sxx += dx * dx;
        sxy += dx * dy;
        sx += dx;
        syy += dy * dy;
        sy += dy;
        bx += dx * di;
        by += dy * di;
        bi += di;
    }
    // Определитель системы 3×3 и правило Крамера.
    let det = sxx * (syy * n - sy * sy) - sxy * (sxy * n - sy * sx) + sx * (sxy * sy - syy * sx);
    if det.abs() < 1e-9 {
        return None;
    }
    let det_a = bx * (syy * n - sy * sy) - sxy * (by * n - sy * bi) + sx * (by * sy - syy * bi);
    let det_b = sxx * (by * n - sy * bi) - bx * (sxy * n - sy * sx) + sx * (sxy * bi - by * sx);
    let det_c = sxx * (syy * bi - by * sy) - sxy * (sxy * bi - by * sx) + bx * (sxy * sy - syy * sx);
    let (a, b, c) = (det_a / det, det_b / det, det_c / det);
    Some((a, b, mi + c - a * mx - b * my))
}

/// Грань, которая может закрывать линии: экранный контур и уравнение
/// плоскости `1/z = aa*x + bb*y + cc`.
pub(crate) struct Occluder {
    /// Рёбра контура в индексах вершин модели (петель может быть несколько,
    /// например вырезы двигателей).
    edges: Vec<(usize, usize)>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    aa: f32,
    bb: f32,
    cc: f32,
}

impl Occluder {
    /// Лежит ли точка экрана внутри контура грани (правило «чёт-нечёт»).
    fn covers(&self, scr: &[(f32, f32, f32)], x: f32, y: f32) -> bool {
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        let mut inside = false;
        for &(i, j) in &self.edges {
            let (xi, yi, _) = scr[i];
            let (xj, yj, _) = scr[j];
            if (yi <= y) != (yj <= y) {
                let xc = xi + (y - yi) / (yj - yi) * (xj - xi);
                if x < xc {
                    inside = !inside;
                }
            }
        }
        inside
    }

    /// Обратная глубина плоскости грани в точке экрана `(x, y)`.
    fn plane_invz(&self, x: f32, y: f32) -> f32 {
        self.aa * x + self.bb * y + self.cc
    }
}

/// Отрезок с удалением невидимых участков.
///
/// Для каждой точки отрезка проверяется, не закрывает ли её какая-нибудь
/// обращённая к камере грань: точка должна лежать внутри её экранного
/// контура и быть глубже её плоскости. Глубина ребра и глубина грани
/// берутся в одной и той же непрерывной точке, поэтому линия на границе
/// своей же грани никогда не пропадает.
pub(crate) fn line_hidden(
    frame: &mut Frame,
    occl: &[Occluder],
    scr: &[(f32, f32, f32)],
    a: (f32, f32, f32),
    b: (f32, f32, f32),
    color: u32,
) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let n = dx.abs().max(dy.abs()).ceil().max(1.0) as i32;
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let x = a.0 + dx * t;
        let y = a.1 + dy * t;
        let invz = a.2 + (b.2 - a.2) * t;
        let hidden = occl
            .iter()
            .any(|o| o.covers(scr, x, y) && o.plane_invz(x, y) > invz * (1.0 + DEPTH_EPS));
        if !hidden {
            frame.plot(x.round() as i32, y.round() as i32, color);
        }
    }
}
