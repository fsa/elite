//! Геометрия вида: направления кабины, камера и отсечение отрезков.
//!
//! Координаты объектов приходят из Z80 уже в системе игрока (X вправо,
//! Y вверх, Z вглубь экрана). Здесь только преобразование «вид → экран»
//! и вспомогательная математика, общая для всех элементов сцены.

use crate::render3d::{Rotation, Screen};

/// Ближняя плоскость отсечения.
pub(super) const NEAR: f32 = 0.5;

/// Направление вида из кабины.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewDir {
    /// Вид вперёд (+Z)
    Forward,
    /// Вид назад (-Z)
    Back,
    /// Вид влево (-X)
    Left,
    /// Вид вправо (+X)
    Right,
}

impl Default for ViewDir {
    fn default() -> Self {
        ViewDir::Forward
    }
}

/// Поворот камеры для выбранного направления вида.
pub(super) fn rotation(dir: ViewDir) -> Rotation {
    match dir {
        ViewDir::Forward => Rotation::identity(),
        ViewDir::Back => Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::PI),
        ViewDir::Left => Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2),
        ViewDir::Right => Rotation::axis_angle([0.0, 1.0, 0.0], -std::f32::consts::FRAC_PI_2),
    }
}

/// Проекция точки в координатах вида: экранные координаты и радиус диска.
///
/// `radius` — радиус объекта в единицах координат игры. Четвёртым элементом
/// возвращается глубина `z`, чтобы вызывающий мог отсечь объект по камере.
pub(super) fn project(pos: [f32; 3], radius: f32, screen: &Screen) -> Option<(f32, f32, f32, f32)> {
    if pos[2] <= NEAR {
        return None;
    }
    let x = screen.cx + pos[0] / pos[2] * screen.k;
    let y = screen.cy - pos[1] / pos[2] * screen.k;
    let r = radius / pos[2] * screen.k;
    Some((x, y, r, pos[2]))
}

/// Отсечь отрезок по ближней плоскости в координатах вида.
///
/// Возвращает точки уже с `z > NEAR`, чтобы проекция не улетала в
/// бесконечность.
pub(super) fn clip_segment_near(mut a: [f32; 3], mut b: [f32; 3]) -> Option<([f32; 3], [f32; 3])> {
    if a[2] <= NEAR && b[2] <= NEAR {
        return None;
    }
    let t = |a: [f32; 3], b: [f32; 3]| (NEAR - a[2]) / (b[2] - a[2]);
    if a[2] <= NEAR {
        let t = t(a, b);
        a = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, NEAR];
    } else if b[2] <= NEAR {
        let t = t(b, a);
        b = [b[0] + (a[0] - b[0]) * t, b[1] + (a[1] - b[1]) * t, NEAR];
    }
    Some((a, b))
}

/// Отсечь 2D-отрезок прямоугольником (Liang–Barsky).
///
/// Без этого `Frame::line` честно шёл бы от координаты в миллионы пикселей,
/// что для выстрела сбоку от камеры заметно тормозит кадр.
pub(super) fn clip_segment_2d(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    xmin: f32,
    ymin: f32,
    xmax: f32,
    ymax: f32,
) -> Option<(i32, i32, i32, i32)> {
    if ![x0, y0, x1, y1].iter().all(|v| v.is_finite()) {
        return None;
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let p = [-dx, dx, -dy, dy];
    let q = [x0 - xmin, xmax - x0, y0 - ymin, ymax - y0];
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    for i in 0..4 {
        if p[i].abs() < f32::EPSILON {
            if q[i] < 0.0 {
                return None;
            }
            continue;
        }
        let r = q[i] / p[i];
        if p[i] < 0.0 {
            if r > t1 {
                return None;
            }
            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return None;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }
    Some((
        (x0 + t0 * dx).round() as i32,
        (y0 + t0 * dy).round() as i32,
        (x0 + t1 * dx).round() as i32,
        (y0 + t1 * dy).round() as i32,
    ))
}
