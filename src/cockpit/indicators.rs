//! Значения длинных клеток: шкалы, отклонения и запас ракет.
//!
//! Каждая функция рисует одну клетку целиком и сама заливает её фон, поэтому
//! старую шкалу не нужно стирать отдельно.

use crate::display::Frame;

use super::geometry::{CockpitGeom, LINE, Rect};
use super::palette::{BACKGROUND, CockpitStyle};
use super::values::{
    CockpitValues, DEFLECTION_MAX, GAUGE_MAX, Indicator, MISSILE_ROW, MISSILE_SLOTS, PANEL_ROWS,
};

/// Нарисовать значения всех длинных клеток.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    for right in [false, true] {
        let cells = if right { &values.right } else { &values.left };
        for row in 0..PANEL_ROWS {
            let value = cells[row as usize];
            if !right && row == MISSILE_ROW {
                let missiles = match value {
                    Indicator::Missiles(n) => n,
                    _ => 0,
                };
                draw_missiles(f, g, style, missiles);
                continue;
            }
            draw_indicator(f, g, row, right, value, style);
        }
    }
}

/// Нарисовать значение одной длинной клетки.
fn draw_indicator(
    f: &mut Frame,
    g: &CockpitGeom,
    row: i32,
    right: bool,
    value: Indicator,
    style: CockpitStyle,
) {
    let cell = g.bar_cell(row, right);
    if cell.is_empty() {
        return;
    }
    match value {
        Indicator::Gauge(v) => draw_gauge(f, cell, style, v),
        Indicator::Deflection(v) => draw_deflection(f, cell, style, v),
        Indicator::Missiles(_) => {
            // Запас ракет живёт только в нижней левой клетке; в остальных
            // местах показываем пустую клетку.
            fill_cell(f, cell, BACKGROUND);
        }
        Indicator::Empty => fill_cell(f, cell, BACKGROUND),
    }
}

/// Шкала: цветной фон и белая заливка слева направо, как в оригинале.
///
/// Заливка не касается верхней и нижней границы клетки: сверху и снизу
/// остаётся цветной отступ толщиной [`LINE`] — столько же, сколько у рамки.
fn draw_gauge(f: &mut Frame, cell: Rect, style: CockpitStyle, value: u8) {
    fill_cell(f, cell, style.bar);
    let inner = indicator_inner(cell);
    if inner.is_empty() {
        return;
    }
    let filled = inner.w * value.min(GAUGE_MAX) as i32 / GAUGE_MAX as i32;
    if filled > 0 {
        f.fill_rect(
            inner.x,
            inner.y,
            filled.min(inner.w) as usize,
            inner.h as usize,
            style.line,
        );
    }
}

/// Область значения внутри клетки: без цветного отступа [`LINE`] сверху и
/// снизу. По горизонтали значение занимает всю ширину клетки, как в
/// оригинале.
fn indicator_inner(cell: Rect) -> Rect {
    let gap = LINE.min((cell.h - 1).max(0) / 2);
    Rect {
        x: cell.x,
        y: cell.y + gap,
        w: cell.w,
        h: (cell.h - 2 * gap).max(0),
    }
}

/// Индикатор влево/вправо: маркер на цветном фоне, 0 — по центру.
fn draw_deflection(f: &mut Frame, cell: Rect, style: CockpitStyle, value: i8) {
    fill_cell(f, cell, style.bar);
    let inner = indicator_inner(cell);
    if inner.is_empty() {
        return;
    }
    let marker_w = (LINE * 2).clamp(1, inner.w.max(1));
    let half = inner.w / 2;
    let max_shift = (half - marker_w / 2).max(0);
    let shift =
        value.clamp(-DEFLECTION_MAX, DEFLECTION_MAX) as i32 * max_shift / DEFLECTION_MAX as i32;
    let x = (inner.x + half + shift - marker_w / 2).clamp(inner.x, inner.x + inner.w - marker_w);
    f.fill_rect(x, inner.y, marker_w as usize, inner.h as usize, style.line);
}

/// Прямоугольник одной ракеты внутри ракетной клетки.
///
/// Высота совпадает с заливкой индикаторов, ширина чуть больше высоты на
/// толщину линии. Левый отступ и промежутки между ракетами остаются
/// цветными — это фон клетки.
pub(super) fn missile_icon_rect(cell: Rect, pitch: i32, slot: i32) -> Rect {
    let inner = indicator_inner(cell);
    let gap = (inner.y - cell.y).max(0);
    let icon_h = inner.h;
    Rect {
        x: cell.x + gap + slot * pitch,
        y: inner.y,
        w: (icon_h + LINE).max(1),
        h: icon_h,
    }
}

/// Нарисовать запас ракет и клетку режима в нижней левой длинной клетке.
///
/// Фон ракетной части всегда цвет полосы (в оригинале — красный). Ракеты —
/// белые прямоугольники с цветной серединой: максимум четыре, слева
/// направо. Если ракет меньше, правые прямоугольники не рисуются, но фон
/// остаётся цветным. Прямоугольник чуть шире своей высоты, а по высоте
/// совпадает с заливкой индикаторов. Справа после разделителя — клетка
/// режима; пока она всегда чёрная.
fn draw_missiles(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, missiles: u8) {
    let left = g.bar_cell(MISSILE_ROW, false);
    if left.is_empty() {
        return;
    }
    let cell = g.missile_cell(MISSILE_ROW, false);
    if cell.is_empty() {
        return;
    }
    fill_cell(f, cell, style.bar);
    let filled = (missiles as i32).clamp(0, MISSILE_SLOTS);
    let pitch = g.missile_slot_width();
    for slot in 0..filled {
        let icon = missile_icon_rect(cell, pitch, slot);
        if icon.x + icon.w > cell.x + cell.w {
            break;
        }
        super::frame::stroke_rect(f, icon, LINE, style.line);
    }
    // Разделитель между ракетами и клеткой режима.
    let sep_x = cell.x + cell.w;
    let sep_w = (left.x + left.w - sep_x).clamp(0, LINE);
    if sep_w > 0 {
        f.fill_rect(sep_x, left.y, sep_w as usize, left.h as usize, style.line);
    }
    // Клетка режима: по умолчанию чёрная.
    let mode = g.mode_cell(MISSILE_ROW, false);
    if !mode.is_empty() {
        fill_cell(f, mode, BACKGROUND);
    }
}

/// Залить клетку сплошным цветом.
fn fill_cell(f: &mut Frame, cell: Rect, color: u32) {
    f.fill_rect(cell.x, cell.y, cell.w as usize, cell.h as usize, color);
}
