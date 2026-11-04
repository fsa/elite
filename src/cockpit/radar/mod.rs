//! Классический овальный радар оригинальной Elite.
//!
//! Компонент независим: получает свою клетку и рисует только в ней. Данные
//! приходят одним снимком [`CockpitValues`], поэтому радар не знает ни о
//! полётной подсистеме, ни о памяти Z80.
//!
//! Контакты хранятся в системе координат игрока (`+X` вправо, `+Y` вверх,
//! `+Z` вперёд). Сканер оригинала отображает `X` по горизонтали, `Z` по
//! вертикали, а `Y` — длиной вертикального штриха.
//!
//! Альтернативный радар Кладова временно отключён: его нужно переделать.

use crate::display::Frame;

use super::geometry::{CockpitGeom, Rect};
use super::palette::CockpitStyle;
use super::values::{CockpitValues, RadarContact};

mod oval;
#[cfg(test)]
mod tests;

/// Дальность радара в единицах координат игры.
///
/// В оригинале сканер показывает объекты, у которых старший байт координаты
/// меньше `0x40`, то есть примерно до `0x4000` единиц.
pub(super) const RADAR_RANGE: f32 = 0x4000 as f32;

/// Положение контакта на радаре и высота его вертикального штриха.
pub(super) struct ContactPlot {
    pub x: i32,
    pub y: i32,
    /// Высота в пикселях; положительная — вверх.
    pub height: i32,
}

/// Нарисовать радар в свободной области центральной клетки.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    oval::draw(f, g.oval_radar_cell(), style, values);
}

/// Перевести контакт в пиксели внутри эллипса радара.
///
/// Контакт не пропадает: если объект дальше дальности радара или его
/// положение выходит за контур эллипса, направление на объект прижимается
/// к краю овала. Иначе цели, ушедшие в сторону или назад, внезапно
/// исчезали бы с радара.
pub(super) fn plot_contact(cell: Rect, contact: &RadarContact) -> Option<ContactPlot> {
    if cell.w < 4 || cell.h < 4 {
        return None;
    }
    let nx = contact.x as f32 / RADAR_RANGE;
    let nz = contact.z as f32 / RADAR_RANGE;
    // Единичная окружность в этих координатах совпадает с контуром овала.
    let len = (nx * nx + nz * nz).sqrt();
    let (nx, nz) = if len > 1.0 {
        (nx / len, nz / len)
    } else {
        (nx, nz)
    };
    let rx = (cell.w / 2 - 1).max(1) as f32;
    let ry = (cell.h / 2 - 1).max(1) as f32;
    let ny = (contact.y as f32 / RADAR_RANGE).clamp(-1.0, 1.0);
    let x = cell.x + cell.w / 2 + (nx * rx).round() as i32;
    let y = cell.y + cell.h / 2 - (nz * ry).round() as i32;
    Some(ContactPlot {
        x: x.clamp(cell.x, cell.x + cell.w - 1),
        y: y.clamp(cell.y, cell.y + cell.h - 1),
        height: (ny * ry).round() as i32,
    })
}

/// Контур эллипса, вписанного в клетку.
pub(super) fn ellipse(
    f: &mut Frame,
    cx: i32,
    cy: i32,
    rx: i32,
    ry: i32,
    stroke: i32,
    color: u32,
) {
    let rx = rx.max(1) as f64;
    let ry = ry.max(1) as f64;
    let mut steps = (rx.max(ry) * 4.0).ceil() as usize;
    steps = (steps / 4 * 4).max(16);
    for i in 0..steps {
        let t = std::f64::consts::TAU * i as f64 / steps as f64;
        let x = cx + (rx * t.cos()).round() as i32;
        let y = cy + (ry * t.sin()).round() as i32;
        plot_stroke(f, x, y, stroke, color);
    }
}

/// Квадратный мазок толщиной `stroke` пикселей.
pub(super) fn plot_stroke(f: &mut Frame, x: i32, y: i32, stroke: i32, color: u32) {
    let stroke = stroke.max(1);
    let off = -(stroke - 1) / 2;
    f.fill_rect(x + off, y + off, stroke as usize, stroke as usize, color);
}

/// Горизонтальная линия, обрезанная клеткой радара.
#[allow(clippy::too_many_arguments)]
pub(super) fn hline(
    f: &mut Frame,
    cell: Rect,
    x0: i32,
    x1: i32,
    y: i32,
    stroke: i32,
    color: u32,
) {
    let (x0, x1) = (x0.min(x1), x0.max(x1));
    let x0 = x0.max(cell.x);
    let x1 = x1.min(cell.x + cell.w - 1);
    if x0 > x1 || y < cell.y || y >= cell.y + cell.h {
        return;
    }
    let stroke = stroke.max(1);
    let off = -(stroke - 1) / 2;
    f.fill_rect(x0, y + off, (x1 - x0 + 1) as usize, stroke as usize, color);
}

/// Вертикальная линия, обрезанная клеткой радара.
pub(super) fn vline(
    f: &mut Frame,
    cell: Rect,
    x: i32,
    y0: i32,
    y1: i32,
    stroke: i32,
    color: u32,
) {
    if x < cell.x || x >= cell.x + cell.w {
        return;
    }
    let (y0, y1) = (y0.min(y1), y0.max(y1));
    let y0 = y0.max(cell.y);
    let y1 = y1.min(cell.y + cell.h - 1);
    if y0 > y1 {
        return;
    }
    let stroke = stroke.max(1);
    let off = -(stroke - 1) / 2;
    f.fill_rect(x + off, y0, stroke as usize, (y1 - y0 + 1) as usize, color);
}
