//! Альтернативный локатор Кладова.
//!
//! В версиях Elite 2/3 Кладова новый локатор включается дополнительной
//! клавишей и дополняет классический сканер. В нативном кокпите он занимает
//! ту же клетку: овал размечается кольцами дальности и крестом, а контакты
//! показываются квадратными метками. Всё рисование по-прежнему идёт только
//! внутри переданной клетки.

use crate::display::Frame;

use super::super::geometry::Rect;
use super::super::palette::CockpitStyle;
use super::super::values::{CockpitValues, RadarContactKind};
use super::{ellipse, hline, oval, plot_contact, vline};

/// Нарисовать альтернативный локатор в клетке `cell`.
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
    let color = oval::radar_color(style.line);
    let stroke = (rx / 90).clamp(1, 3);

    ellipse(f, cx, cy, rx, ry, stroke, color);
    draw_rings(f, inner, cx, cy, rx, ry, color);
    draw_cross(f, inner, cx, cy, rx, ry, color);

    for contact in values.radar.as_slice() {
        let Some(plot) = plot_contact(inner, contact) else {
            continue;
        };
        let half = (rx / 30).clamp(1, 3);
        // Вертикальный штрих сохраняет высоту объекта из классического
        // сканера, но укорочен, чтобы метка не сливалась с соседними.
        let tip = plot.y - plot.height / 2;
        vline(f, inner, plot.x, plot.y, tip, 1, color);
        match contact.kind {
            RadarContactKind::Station => hollow_marker(f, inner, plot.x, plot.y, half, color),
            RadarContactKind::Missile => cross_marker(f, inner, plot.x, plot.y, half, color),
            RadarContactKind::Ship => filled_marker(f, inner, plot.x, plot.y, half, color),
        }
    }
}

/// Два кольца дальности внутри внешнего овала.
fn draw_rings(f: &mut Frame, cell: Rect, cx: i32, cy: i32, rx: i32, ry: i32, color: u32) {
    for (num, den) in [(1, 3), (2, 3)] {
        let ring_rx = (rx * num / den).max(1);
        let ring_ry = (ry * num / den).max(1);
        // Кольца рисуются по точкам, поэтому обрезаем их вручную.
        for point in ellipse_points(cx, cy, ring_rx, ring_ry) {
            if point.0 >= cell.x && point.0 < cell.x + cell.w && point.1 >= cell.y && point.1 < cell.y + cell.h {
                super::plot_stroke(f, point.0, point.1, 1, color);
            }
        }
    }
}

/// Точки эллипса, чтобы не дублировать код отрисовки колец.
fn ellipse_points(cx: i32, cy: i32, rx: i32, ry: i32) -> Vec<(i32, i32)> {
    let rx = rx.max(1) as f64;
    let ry = ry.max(1) as f64;
    let steps = ((rx.max(ry) * 4.0).ceil() as usize).max(16);
    (0..steps)
        .map(|i| {
            let t = std::f64::consts::TAU * i as f64 / steps as f64;
            (
                cx + (rx * t.cos()).round() as i32,
                cy + (ry * t.sin()).round() as i32,
            )
        })
        .collect()
}

/// Пунктирный крест по осям овала.
fn draw_cross(f: &mut Frame, cell: Rect, cx: i32, cy: i32, rx: i32, ry: i32, color: u32) {
    let dash = (rx / 40).max(2);
    let gap = (rx / 12).max(3);
    dashed_h(f, cell, cx - rx, cx + rx, cy, dash, gap, color);
    dashed_v(f, cell, cx, cy - ry, cy + ry, dash, gap, color);
}

fn dashed_h(
    f: &mut Frame,
    cell: Rect,
    x0: i32,
    x1: i32,
    y: i32,
    dash: i32,
    gap: i32,
    color: u32,
) {
    let mut x = x0.min(x1);
    let end_max = x0.max(x1);
    while x <= end_max {
        hline(f, cell, x, (x + dash - 1).min(end_max), y, 1, color);
        x += dash + gap;
    }
}

fn dashed_v(
    f: &mut Frame,
    cell: Rect,
    x: i32,
    y0: i32,
    y1: i32,
    dash: i32,
    gap: i32,
    color: u32,
) {
    let mut y = y0.min(y1);
    let end_max = y0.max(y1);
    while y <= end_max {
        vline(f, cell, x, y, (y + dash - 1).min(end_max), 1, color);
        y += dash + gap;
    }
}

fn filled_marker(f: &mut Frame, cell: Rect, x: i32, y: i32, half: i32, color: u32) {
    for dy in -half..=half {
        hline(f, cell, x - half, x + half, y + dy, 1, color);
    }
}

fn hollow_marker(f: &mut Frame, cell: Rect, x: i32, y: i32, half: i32, color: u32) {
    hline(f, cell, x - half, x + half, y - half, 1, color);
    hline(f, cell, x - half, x + half, y + half, 1, color);
    vline(f, cell, x - half, y - half, y + half, 1, color);
    vline(f, cell, x + half, y - half, y + half, 1, color);
}

fn cross_marker(f: &mut Frame, cell: Rect, x: i32, y: i32, half: i32, color: u32) {
    hline(f, cell, x - half, x + half, y, 1, color);
    vline(f, cell, x, y - half, y + half, 1, color);
}
