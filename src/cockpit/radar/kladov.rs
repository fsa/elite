//! Радар Кладова.
//!
//! В оригинальной версии Кладова радар — два квадрата. Внешний квадрат сверху
//! и снизу совпадает с линией основной рамки кокпита. Его сторона занимает 55
//! экранных пикселей, внутренний квадрат — 23 пикселя.
//!
//! Положение знака задаётся не абсолютной координатой, а направлением на
//! объект: отношения `x / |v|` и `y / |v|` квантуются с шагом в три пикселя
//! оригинального экрана. Размер знака задаёт старший байт длины вектора,
//! поэтому близкие объекты показываются заметно крупнее дальних.
//!
//! Знак `+` означает объект впереди, `x` — сзади.

use crate::display::Frame;

use super::super::geometry::{CockpitGeom, Rect};
use super::super::palette::CockpitStyle;
use super::super::values::{CockpitValues, RadarContact};
use super::{hline, oval, vline};

/// Сторона внешнего квадрата на экране оригинала, в пикселях.
const ORIGINAL_SIDE: i32 = 55;
/// Левый верхний угол внешнего квадрата на экране оригинала.
const ORIGINAL_X: i32 = 96;
const ORIGINAL_Y: i32 = 136;
/// Нейтральное положение знака по X: `0x7C`. По Y нейтральное значение —
/// `0x23 + 0x80 = 0xA3`, то есть на 27 пикселей ниже верха квадрата.
const CENTER_X: i32 = 124;
/// Сдвиг знака за один шаг квантования направления.
const STEP: i32 = 3;
/// Поправка к вертикали из младших двух битов адреса слота Z80.
const SLOT_DY: [i32; 8] = [0, 3, 2, 1, 0, 3, 2, 1];

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
    // В оригинале знак состоит из одиночных пикселей: масштабируется только
    // длина луча, толщина штриха остаётся одной точкой.
    let pen = 1;
    for contact in values.radar.as_slice() {
        let Some((x, y, arm, ahead)) = plot_contact(outer, contact) else {
            continue;
        };
        if ahead {
            draw_plus(f, clip, x, y, arm, pen, color);
        } else {
            draw_cross(f, clip, x, y, arm, pen, color);
        }
    }
}

/// Внутренний квадрат: 23 пикселя на 55 пикселей внешнего.
fn inner_square(outer: Rect) -> Rect {
    let side = ((outer.w as f32 * 23.0 / ORIGINAL_SIDE as f32).round() as i32)
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

/// Проекция контакта в пиксели оригинального экрана, затем в клетку радара.
///
/// Повторяет `loc_B2EF`: сначала `CalcLenVectorCrd` даёт длину вектора, затем
/// `CompasCrdNnear` делит на неё `x` и `y`; результат квантуется на 8 уровней
/// (максимум 7) и умножается на три пикселя. Старший байт длины задаёт число
/// шагов луча через `sub_B440`.
pub(super) fn plot_contact(outer: Rect, contact: &RadarContact) -> Option<(i32, i32, i32, bool)> {
    let ax = contact.x.unsigned_abs() as i64;
    let ay = contact.y.unsigned_abs() as i64;
    let az = contact.z.unsigned_abs() as i64;

    // sub_96DC отбрасывает объекты, у которых старший байт X или Z больше 0x3F.
    let xh = ax >> 8;
    let zh = az >> 8;
    if xh >= 0x40 || zh >= 0x40 || xh * xh + zh * zh >= 0x1600 {
        return None;
    }

    let len = isqrt(ax * ax + ay * ay + az * az);
    if len == 0 {
        return None;
    }

    let qx = ((ax * 8 / len).min(7)) as i32;
    let qy = ((ay * 8 / len).min(7)) as i32;
    let x_sign = if contact.x < 0 { -1 } else { 1 };
    let y_sign = if contact.y < 0 { -1 } else { 1 };

    // C = 0x7C ± 3*qx, B = 0x23 ∓ 3*qy - поправка слота.
    let mut c = CENTER_X + x_sign * STEP * qx;
    let b = 35 - y_sign * STEP * qy - SLOT_DY[(contact.slot as usize) & 7];

    // D — старший байт длины. Младшие биты задают длину луча; если биты
    // 4..6 нулевые, знак дополнительно сдвигается влево на пиксель.
    let d = (len >> 8) as i32;
    let mut arm = 1;
    if d & 0x70 == 0 {
        c -= 1;
        arm += 4;
    }
    if d & 0x0C == 0 {
        arm += 2;
    }
    if d & 0x03 == 0 {
        arm += 1;
    }

    let scale = outer.w as f32 / ORIGINAL_SIDE as f32;
    let x = outer.x + ((c - ORIGINAL_X) as f32 * scale).round() as i32;
    // B в оригинале — экранная координата Y минус 0x80.
    let y = outer.y + ((b + 128 - ORIGINAL_Y) as f32 * scale).round() as i32;
    let arm = ((arm as f32 * scale).round() as i32).max(1);

    Some((x, y, arm, contact.z >= 0))
}

/// Целочисленный квадратный корень для координат радара.
fn isqrt(value: i64) -> i64 {
    if value <= 0 {
        return 0;
    }
    let mut root = (value as f64).sqrt() as i64;
    while root * root > value {
        root -= 1;
    }
    while (root + 1) * (root + 1) <= value {
        root += 1;
    }
    root
}

fn draw_plus(f: &mut Frame, clip: Rect, x: i32, y: i32, arm: i32, stroke: i32, color: u32) {
    hline(f, clip, x - arm, x + arm, y, stroke, color);
    vline(f, clip, x, y - arm, y + arm, stroke, color);
}

fn draw_cross(f: &mut Frame, clip: Rect, x: i32, y: i32, arm: i32, stroke: i32, color: u32) {
    let stroke = stroke.max(1);
    for d in -arm..=arm {
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
