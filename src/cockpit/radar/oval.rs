//! Классический овальный радар оригинальной Elite.
//!
//! Внутри овала — почти треугольник: его верхние вершины лежат у верхней
//! дуги, а сам он сходится к центру. От вершины вниз идёт линия, а от
//! верхних вершин — две пунктирные линии под 45° влево и вправо. Овал
//! разделён на четыре равные по высоте части тремя горизонтальными
//! пунктирными линиями: верхняя и нижняя — частые, средняя — редкая.

use crate::display::Frame;

use super::super::geometry::Rect;
use super::super::palette::CockpitStyle;
use super::super::values::CockpitValues;
use super::{ellipse, hline, plot_contact, plot_stroke, vline, ContactPlot};

/// Полуширина треугольника в процентах от полуширины овала.
const TRIANGLE_HALF_WIDTH_PCT: i32 = 36;
/// Высота треугольника в процентах от полувысоты овала.
const TRIANGLE_HEIGHT_PCT: i32 = 83;

/// Нарисовать овальный радар в клетке `cell`.
pub(super) fn draw(f: &mut Frame, cell: Rect, style: CockpitStyle, values: &CockpitValues) {
    if cell.w < 4 || cell.h < 4 {
        return;
    }
    let inner = cell.inset(1);
    if inner.is_empty() {
        return;
    }
    let cx = inner.x + inner.w / 2;
    let cy = inner.y + inner.h / 2;
    let rx = (inner.w / 2 - 1).max(1);
    let ry = (inner.h / 2 - 1).max(1);
    let color = radar_color(style.line);
    let stroke = stroke_width(rx);

    ellipse(f, cx, cy, rx, ry, stroke, color);
    draw_graticule(f, inner, cx, cy, rx, ry, stroke, color);

    for contact in values.radar.as_slice() {
        if let Some(plot) = plot_contact(inner, contact) {
            draw_contact(f, inner, plot, stroke, color);
        }
    }
}

/// Немного поднять яркость линий радара: тонкий рисунок иначе выглядит
/// тусклее рамок кокпита.
pub(super) fn radar_color(color: u32) -> u32 {
    let lift = |c: u32| (c + 0x20).min(0xFF);
    (color & 0xFF00_0000)
        | (lift((color >> 16) & 0xFF) << 16)
        | (lift((color >> 8) & 0xFF) << 8)
        | lift(color & 0xFF)
}

/// Толщина линий радара: на крупных клетках рисунок чуть жирнее.
fn stroke_width(rx: i32) -> i32 {
    (rx / 90).clamp(1, 3)
}

/// Разметка сканера: треугольник, вертикаль и три горизонтальные линии.
fn draw_graticule(
    f: &mut Frame,
    cell: Rect,
    cx: i32,
    cy: i32,
    rx: i32,
    ry: i32,
    stroke: i32,
    color: u32,
) {
    let half = (ry / 2).max(1);
    let span = half_span(rx, ry, half);
    let tri_dx = (rx * TRIANGLE_HALF_WIDTH_PCT / 100).max(1);
    let tri_dy = (ry * TRIANGLE_HEIGHT_PCT / 100).max(1);
    let tri_at_half = (tri_dx * half / tri_dy).clamp(0, span);

    let dash = (rx / 40).max(2);
    let dense_gap = (rx / 16).max(3);
    let sparse_gap = (rx / 8).max(6);

    // Верхняя и нижняя линии на четвертях высоты — частый пунктир.
    // Верхняя прерывается внутри треугольника.
    if tri_at_half < span {
        dashed(
            f,
            cell,
            cx - span,
            cx - tri_at_half,
            cy - half,
            dash,
            dense_gap,
            stroke,
            color,
        );
        dashed(
            f,
            cell,
            cx + tri_at_half,
            cx + span,
            cy - half,
            dash,
            dense_gap,
            stroke,
            color,
        );
    }
    dashed(
        f,
        cell,
        cx - span,
        cx + span,
        cy + half,
        dash,
        dense_gap,
        stroke,
        color,
    );

    // Средняя линия — редкий пунктир.
    dashed(
        f,
        cell,
        cx - rx,
        cx + rx,
        cy,
        dash,
        sparse_gap,
        stroke,
        color,
    );

    // Треугольник: сплошные стороны от верхних вершин к центру.
    let top = cy - tri_dy;
    thick_line(f, cx - tri_dx, top, cx, cy, stroke, color);
    thick_line(f, cx + tri_dx, top, cx, cy, stroke, color);

    // От вершины треугольника вниз до нижней дуги.
    vline(f, cell, cx, cy, cy + ry, stroke, color);

    // От верхних вершин — пунктирные линии под 45° влево и вправо.
    // В оригинале они идут до пересечения с нижней половиной овала.
    let diag_dash = (ry / 20).max(2);
    let diag_len = tri_dy + half;
    dashed_diagonal(
        f, cell, cx, cy, rx, ry, cx - tri_dx, top, -1, diag_len, diag_dash, stroke, color,
    );
    dashed_diagonal(
        f, cell, cx, cy, rx, ry, cx + tri_dx, top, 1, diag_len, diag_dash, stroke, color,
    );
}

/// Полуширина эллипса на высоте `dy` от центра.
fn half_span(rx: i32, ry: i32, dy: i32) -> i32 {
    let t = dy as f32 / ry as f32;
    let k = (1.0 - t * t).max(0.0).sqrt();
    (rx as f32 * k).round() as i32
}

/// Пунктирная горизонтальная линия от `x0` до `x1`.
#[allow(clippy::too_many_arguments)]
fn dashed(
    f: &mut Frame,
    cell: Rect,
    x0: i32,
    x1: i32,
    y: i32,
    dash: i32,
    gap: i32,
    stroke: i32,
    color: u32,
) {
    let (x0, x1) = (x0.min(x1), x0.max(x1));
    let dash = dash.max(1);
    let gap = gap.max(1);
    let mut x = x0;
    while x <= x1 {
        let end = (x + dash - 1).min(x1);
        hline(f, cell, x, end, y, stroke, color);
        x += dash + gap;
    }
}

/// Сплошная линия заданной толщины.
fn thick_line(f: &mut Frame, x0: i32, y0: i32, x1: i32, y1: i32, stroke: i32, color: u32) {
    let off = -(stroke - 1) / 2;
    for dx in 0..stroke {
        for dy in 0..stroke {
            f.line(x0 + off + dx, y0 + off + dy, x1 + off + dx, y1 + off + dy, color);
        }
    }
}

/// Пунктирная диагональ: на каждый шаг `step` по X — один шаг вниз.
/// Точки за контуром эллипса не рисуются.
#[allow(clippy::too_many_arguments)]
fn dashed_diagonal(
    f: &mut Frame,
    cell: Rect,
    cx: i32,
    cy: i32,
    rx: i32,
    ry: i32,
    x0: i32,
    y0: i32,
    step: i32,
    len: i32,
    dash: i32,
    stroke: i32,
    color: u32,
) {
    let dash = dash.max(1);
    let nx = rx.max(1) as f32;
    let ny = ry.max(1) as f32;
    for d in 0..=len {
        let x = x0 + step * d;
        let y = y0 + d;
        let ex = (x - cx) as f32 / nx;
        let ey = (y - cy) as f32 / ny;
        if ex * ex + ey * ey > 1.0 {
            break;
        }
        if (d / dash) % 2 != 0 {
            continue;
        }
        if x >= cell.x && x < cell.x + cell.w && y >= cell.y && y < cell.y + cell.h {
            plot_stroke(f, x, y, stroke, color);
        }
    }
}

/// Контакт: буква «Г» — вертикальный штрих высоты объекта и
/// горизонтальная засечка на дальнем от контакта конце.
fn draw_contact(f: &mut Frame, cell: Rect, plot: ContactPlot, stroke: i32, color: u32) {
    let end = plot.y - plot.height;
    if plot.height != 0 {
        vline(f, cell, plot.x, plot.y, end, stroke, color);
    }
    let bar = (cell.w / 60).max(2);
    hline(f, cell, plot.x - bar, plot.x, end, stroke, color);
}
