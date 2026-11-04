//! Компас: кольцо и указатель на выбранную цель.

use crate::display::Frame;

use super::geometry::CockpitGeom;
use super::palette::CockpitStyle;
use super::values::CockpitValues;

/// Растр указателя компаса: заполненный ромб 4x4.
const COMPASS_POINTER_FILLED: [u8; 4] = [0x18, 0x3C, 0x3C, 0x18];
/// Растр указателя компаса: полый квадрат 4x4.
const COMPASS_POINTER_EMPTY: [u8; 4] = [0x3C, 0x24, 0x24, 0x3C];

/// Нарисовать компас: окружность и указатель на цель.
///
/// Размеры повторяют оригинал: круг 20x20 «пикселей» ZX и указатель 4x4,
/// смещённый от центра на `-8..=8`. При увеличении окна один пиксель ZX
/// превращается в восьмую часть диаметра.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    let cell = g.compass_cell();
    if cell.is_empty() {
        return;
    }
    let d = cell.w.min(cell.h);
    if d <= 0 {
        return;
    }
    let unit = (d / 20).max(1);
    let cx = cell.x + cell.w / 2;
    let cy = cell.y + cell.h / 2;
    let radius = (d / 2 - 1).max(1);
    // Толщина кольца — один пиксель оригинала.
    for k in 0..unit {
        f.circle(cx, cy, (radius - k).max(0), style.line);
    }
    // Указатель: положительный `y` — вверх, как `CompasPointerCrdY`.
    // На титульном экране ZX рисуется только кольцо.
    if !values.compass.visible {
        return;
    }
    let x0 = cx + values.compass.x as i32 * unit - 2 * unit;
    let y0 = cy - values.compass.y as i32 * unit - 2 * unit;
    let sprite = if values.compass.filled {
        &COMPASS_POINTER_FILLED
    } else {
        &COMPASS_POINTER_EMPTY
    };
    for (row, bits) in sprite.iter().enumerate() {
        for col in 0..4 {
            if bits & (0x20 >> col) != 0 {
                f.fill_rect(
                    x0 + col * unit,
                    y0 + row as i32 * unit,
                    unit as usize,
                    unit as usize,
                    style.line,
                );
            }
        }
    }
}
