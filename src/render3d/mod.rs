//! Общий рендер объектов игры в пространстве.
//!
//! Здесь собрана отрисовка моделей из [`crate::shipdata`] в том же стиле,
//! что и оригинальный Z80-рендер (`CalcVisibleVerges` + `PrintObject`):
//! видимость граней определяется повёрнутой нормалью, обращённые к камере
//! грани заливаются непрозрачным тёмным металлическим цветом, поверх
//! них рисуются ярко-белые рёбра, а линии, закрытые передними гранями, вырезаются.
//! Грань может задать свой стиль ([`crate::shipdata::FaceStyle`]): выделенный
//! цвет или эффект; перед заливкой грани сортируются от дальней к ближней,
//! чтобы выделенная поверхность перекрывала корпус позади неё.
//!
//! По желанию на весь корабль накладывается небольшой оттенок ([`ShipTint`]):
//! он меняет рёбра и все грани, но не трогает двигатели (`Flame`, `EngineGlow`).
//!
//! Модуль используется и нативной заставкой (`src/splash.rs`), и нативным
//! движком полёта (`src/flight.rs`): заставка задаёт свою калибровку масштаба
//! и сдвиг композиции, полёт — общую перспективную камеру и положение
//! объекта в координатах вида.
//!
//! Конвенция координат — как в оригинале: X вправо, Y вверх, Z вглубь
//! экрана (камера смотрит вдоль +Z), строка экрана растёт вниз.

use crate::display::Frame;
use crate::shipdata::{FaceStyle, Ship};
use std::sync::OnceLock;
use std::time::Instant;

mod faces;
mod occlusion;
#[cfg(test)]
mod tests;

use faces::{face_shapes, fill_faces};
use occlusion::{NEAR, occluders};
pub(crate) use occlusion::line_hidden;

/// Цвет рёбер корабля — ярко-белый.
pub const SHIP_COLOR: u32 = 0xFFFF_FFFF;

/// Базовый цвет заливки видимых граней — очень тёмный металл.
///
/// Грани непрозрачные: через корпус не должны просвечивать солнце, планета
/// и звёзды. Оттенок [`ShipTint`] добавляет к этому основанию цвет, не меняя
/// суммарную яркость; белые рёбра остаются заметно светлее заливки.
/// Каналы не опускаются до нуля, чтобы сдвиг оттенка не обрезался.
pub const SHIP_FACE_COLOR: u32 = 0xFF14_1820;

/// Число различных оттенков корпуса.
const SHIP_TINT_COUNT: u32 = 12;
/// Амплитуда цветного сдвига каждого канала.
const TINT_AMPLITUDE: i32 = 18;

/// Небольшой цветной оттенок металлической поверхности корабля.
///
/// Хранится номером тона, а не готовым цветом: рендер накладывает его на
/// базовый цвет каждой грани и на цвет рёбер, сохраняя их относительную
/// яркость. Двигатели (`Flame`, `EngineGlow`) при этом остаются без изменений.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShipTint(u8);

impl ShipTint {
    /// Выбрать тон по произвольному числу (номеру слота, счётчику появлений).
    pub fn from_seed(seed: u32) -> Self {
        Self((seed % SHIP_TINT_COUNT) as u8)
    }

    /// Наложить оттенок на RGBA-цвет, сохранив альфу.
    ///
    /// К трём каналам прибавляются сдвиги `a·cos(φ + phase)` с фазой 120°.
    /// Их сумма равна нулю: один канал светлеет, два других гаснут на ту же
    /// величину, поэтому заливка не становится ярче. Так оттенок виден даже
    /// на тёмном исходном металле. Чёрный остаётся чёрным — это важно для
    /// входа в станцию.
    pub fn apply(self, color: u32) -> u32 {
        if color & 0x00FF_FFFF == 0 {
            return color;
        }
        let tau = std::f32::consts::TAU;
        let angle = self.0 as f32 * tau / SHIP_TINT_COUNT as f32;
        let d0 = (TINT_AMPLITUDE as f32 * angle.cos()).round() as i32;
        let d1 = (TINT_AMPLITUDE as f32 * (angle + tau / 3.0).cos()).round() as i32;
        let d2 = -d0 - d1;
        let channel = |value: u32, delta: i32| (value as i32 + delta).clamp(0, 255) as u32;
        (color & 0xFF00_0000)
            | (channel((color >> 16) & 0xFF, d0) << 16)
            | (channel((color >> 8) & 0xFF, d1) << 8)
            | channel(color & 0xFF, d2)
    }
}

/// Цвет грани со стилем [`FaceStyle::Flame`].
///
/// Пока это сплошная заливка: стиль задаёт место для будущей анимации
/// двигателей, не меняя формат геометрии.
pub const FLAME_COLOR: u32 = 0xFFF0_7020;

/// Цвет сияния двигателя.
const ENGINE_GLOW_BASE: (f32, f32, f32) = (0.20, 0.80, 1.0);

fn engine_glow_phase() -> f32 {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_secs_f32() * 4.0
}

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
            m: [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]],
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
        std::array::from_fn(|i| self.m[i][0] * v[0] + self.m[i][1] * v[1] + self.m[i][2] * v[2])
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
/// если скалярное произведение повёрнутой нормали и вектора позиции объекта
/// отрицательно, т.е. грань обращена к камере. Для объекта на оси взгляда
/// это эквивалентно `n.z < 0`, но для объектов сбоку/сверху важна полная
/// позиция, иначе грани пропадают «дырками».
///
/// Знаки компонентов нормали уже учтены при разборе ROM
/// (`Face::nx/ny/nz`), так что `Face::flags` в тесте не участвует.
pub fn face_visibility(ship: &Ship, rot: &Rotation, pos: [f32; 3]) -> Vec<bool> {
    ship.faces
        .iter()
        .map(|f| {
            let n = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
            n[0] * pos[0] + n[1] * pos[1] + n[2] * pos[2] < 0.0
        })
        .collect()
}

/// Нарисовать объект в координатах вида без дополнительного оттенка.
///
/// `pos` — положение начала координат модели в системе камеры (X вправо,
/// Y вверх, Z вглубь экрана). `rot` — ориентация модели относительно камеры.
/// `screen` — перспективная проекция. `color` — цвет рёбер.
pub fn draw_ship(
    frame: &mut Frame,
    ship: &Ship,
    rot: &Rotation,
    pos: [f32; 3],
    screen: &Screen,
    color: u32,
) {
    draw_ship_tinted(frame, ship, rot, pos, screen, color, None);
}

/// Нарисовать объект с лёгким цветным оттенком корпуса.
///
/// Оттенок применяется к рёбрам и ко всем граням, кроме двигателей. `None`
/// оставляет нейтральный металлический цвет (используется заставкой).
/// Объекты целиком за камерой или пересекающие её ближнюю плоскость
/// не рисуются.
pub fn draw_ship_tinted(
    frame: &mut Frame,
    ship: &Ship,
    rot: &Rotation,
    pos: [f32; 3],
    screen: &Screen,
    color: u32,
    tint: Option<ShipTint>,
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
    let vis = face_visibility(ship, rot, pos);
    let shapes = face_shapes(ship, &scr, &vis);
    let glow_phase = engine_glow_phase();
    // Сначала непрозрачные грани, затем рёбра поверх них.
    fill_faces(frame, &shapes, &scr, glow_phase, tint);
    let occl = occluders(&shapes);
    let edge_color = tint.map_or(color, |t| t.apply(color));
    for e in ship.edges {
        let (a, b) = (e.v[0] as usize, e.v[1] as usize);
        if a >= scr.len() || b >= scr.len() {
            continue;
        }
        // lod==0 — конструктивное ребро: участвует в контурах граней для
        // заливки, но не рисуется как видимая линия.
        if e.lod & 0x1F == 0 {
            continue;
        }
        let f0 = (e.faces & 0x0F) as usize;
        let f1 = (e.faces >> 4) as usize;
        let seen = |f: usize| f == 0xF || vis.get(f).copied().unwrap_or(false);
        if seen(f0) || seen(f1) {
            line_hidden(frame, &occl, &scr, scr[a], scr[b], edge_color);
        }
    }
}
