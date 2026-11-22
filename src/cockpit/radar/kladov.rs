//! Радар Кладова.
//!
//! В оригинальной версии Кладова радар — два квадрата. Внешний квадрат сверху
//! и снизу совпадает с линией основной рамки кокпита. Внутренний квадрат
//! находится внутри, отношение сторон внешнего и внутреннего квадратов — 2,455.
//!
//! Положение объекта отображается знаком: `+` если объект впереди, `x` если
//! сзади. Размер знака зависит от удаления объекта.

use crate::display::Frame;

use super::super::geometry::{CockpitGeom, Rect};
use super::super::palette::CockpitStyle;
use super::super::values::CockpitValues;
use super::{RADAR_RANGE, hline, oval, vline};

/// Нарисовать радар Кладова.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    let outer = g.kladov_radar_cell();
    if outer.w < 4 || outer.h < 4 {
        return;
    }
    let color = oval::radar_color(style.line);
    let stroke = (outer.w / 120).clamp(1, 3);

    draw_square(f, outer, stroke, color);
    let inner = inner_square(outer);
    draw_square(f, inner, stroke, color);

    let clip = outer.inset(stroke);
    for contact in values.radar.as_slice() {
        let Some((x, y, size, ahead)) = plot_contact(outer, contact.x, contact.y, contact.z) else {
            continue;
        };
        if ahead {
            draw_plus(f, clip, x, y, size, stroke, color);
        } else {
            draw_cross(f, clip, x, y, size, stroke, color);
        }
    }
}

/// Внутренний квадрат: отношение сторон внешнего к внутреннему 2,455.
fn inner_square(outer: Rect) -> Rect {
    let side = ((outer.w as f32 / 2.455).round() as i32)
        .max(3)
        .min(outer.w - 2);
    Rect {
        x: outer.x + (outer.w - side) / 2,
        y: outer.y + (outer.h - side) / 2,
        w: side,
        h: side,
    }
}

fn draw_square(f: &mut Frame, rect: Rect, stroke: i32, color: u32) {
    if rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let s = stroke.max(1) as usize;
    let x = rect.x;
    let y = rect.y;
    let w = rect.w as usize;
    let h = rect.h as usize;
    f.fill_rect(x, y, w, s, color);
    f.fill_rect(x, y + rect.h - stroke.max(1), w, s, color);
    f.fill_rect(x, y, s, h, color);
    f.fill_rect(x + rect.w - stroke.max(1), y, s, h, color);
}

/// Проекция контакта на квадрат радара.
///
/// X — вправо, Z — вперёд/назад (определяет знак и вертикаль),
/// Y — вверх/вниз. Дальние контакты прижимаются к краю.
fn plot_contact(outer: Rect, cx: i32, cy: i32, cz: i32) -> Option<(i32, i32, i32, bool)> {
    let inner = outer.inset(2);
    if inner.w < 3 || inner.h < 3 {
        return None;
    }
    let nx = cx as f32 / RADAR_RANGE;
    let ny = cy as f32 / RADAR_RANGE;
    let nz = cz as f32 / RADAR_RANGE;

    // X/Y отображаются в пределах большого квадрата; дальние объекты
    // прижимаются к его краю, а не к внутреннему.
    let px = nx.clamp(-1.0, 1.0);
    let py = ny.clamp(-1.0, 1.0);

    let half_w = (inner.w / 2 - 1).max(1) as f32;
    let half_h = (inner.h / 2 - 1).max(1) as f32;
    let mid_x = inner.x + inner.w / 2;
    let mid_y = inner.y + inner.h / 2;
    let x = mid_x + (px * half_w).round() as i32;
    let y = mid_y - (py * half_h).round() as i32;

    let dist = (nx * nx + ny * ny + nz * nz).sqrt().clamp(0.0, 1.5);
    let max_size = (outer.w / 18).max(2);
    let min_size = 1;
    let size = ((1.0 - dist / 1.5) * (max_size - min_size) as f32).round() as i32 + min_size;
    let size = size.clamp(min_size, max_size);

    Some((
        x.clamp(inner.x, inner.x + inner.w - 1),
        y.clamp(inner.y, inner.y + inner.h - 1),
        size,
        cz >= 0,
    ))
}

fn draw_plus(f: &mut Frame, clip: Rect, x: i32, y: i32, size: i32, stroke: i32, color: u32) {
    hline(f, clip, x - size, x + size, y, stroke, color);
    vline(f, clip, x, y - size, y + size, stroke, color);
}

fn draw_cross(f: &mut Frame, clip: Rect, x: i32, y: i32, size: i32, stroke: i32, color: u32) {
    let stroke = stroke.max(1);
    for d in -size..=size {
        plot(f, clip, x + d, y + d, stroke, color);
        plot(f, clip, x + d, y - d, stroke, color);
    }
}

fn plot(f: &mut Frame, clip: Rect, x: i32, y: i32, stroke: i32, color: u32) {
    let off = -(stroke - 1) / 2;
    for dy in 0..stroke {
        for dx in 0..stroke {
            let px = x + off + dx;
            let py = y + off + dy;
            if px >= clip.x && px < clip.x + clip.w && py >= clip.y && py < clip.y + clip.h {
                f.fill_rect(px, py, 1, 1, color);
            }
        }
    }
}
