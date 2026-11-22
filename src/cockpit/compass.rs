//! Компас: кольцо и указатель на выбранную цель.

use crate::display::Frame;

use super::geometry::CockpitGeom;
use super::palette::CockpitStyle;
use super::values::CockpitValues;

/// Растры указателя компаса из таблицы `CompassInd*` оригинала.
///
/// Это 4 строки по 8 пикселей: `RenewCompass` сдвигает байт строки в
/// трёхбайтовом окне, поэтому знак может уходить за центр кольца на 8 точек.
/// Первые два растра — обычный ромб и полый квадрат, остальные четыре
/// выбирает `sub_6F29` в межпланетном пространстве для солнца и планеты.
const COMPASS_GLYPHS: [[u8; 4]; 6] = [
    [0x18, 0x3C, 0x3C, 0x18], // CompassIndFill
    [0x3C, 0x24, 0x24, 0x3C], // CompassIndEmpty
    [0x09, 0x1A, 0x2C, 0x48], // CompassInd_N — солнце впереди
    [0x0E, 0x10, 0x08, 0x70], // CompassInd_S — солнце позади
    [0x41, 0x82, 0xB4, 0x58], // CompassInd_W — планета впереди
    [0x0E, 0x1C, 0x20, 0x1C], // CompassInd_E — планета позади
];

/// Режим круглого компаса в нативном кокпите.
///
/// Это чисто нативный переключатель: он не посылает клавиши Z80 и не меняет
/// режим сканера в эмуляторе. Клик по кружку в правом верхнем углу панели
/// включает мигающий режим «солнце + планета» внутри планетной системы.
/// В космосе он включается сам, а внутри системы станция перебивает мигание.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompassView {
    /// Один указатель на планету или станцию, как в обычном полёте.
    PlanetStation,
    /// Мигание: солнце и планета по очереди, каждый со своим значком.
    PlanetSun,
}

impl Default for CompassView {
    fn default() -> Self {
        CompassView::PlanetStation
    }
}

/// Нарисовать компас: окружность и указатели выбранного режима.
///
/// Размеры повторяют оригинал: круг 20x20 «пикселей» ZX и указатель 4x4,
/// смещённый от центра на `-8..=8`. При увеличении окна один пиксель ZX
/// превращается в восьмую часть диаметра.
pub(super) fn draw(
    f: &mut Frame,
    g: &CockpitGeom,
    style: CockpitStyle,
    values: &CockpitValues,
    view: CompassView,
) {
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
    // Как и в оригинале, вне системы режим включается сам, а внутри системы
    // пользователь задаёт его кликом. `sub_FD02` в космосе сбрасывает
    // `CoriolisNear`, поэтому там станция режим не перебивает.
    let station = values.station_near && !values.in_free_space;
    let sun_planet = !station && (values.in_free_space || view == CompassView::PlanetSun);
    if sun_planet {
        let pointer = if values.compass_phase {
            &values.compass_sun
        } else {
            &values.compass_planet
        };
        draw_pointer(f, cx, cy, unit, style, pointer);
    } else {
        draw_pointer(f, cx, cy, unit, style, &values.compass);
    }
}

/// Один указатель. Положительный `y` — вверх, как `CompasPointerCrdY`.
fn draw_pointer(
    f: &mut Frame,
    cx: i32,
    cy: i32,
    unit: i32,
    style: CockpitStyle,
    pointer: &super::values::Compass,
) {
    if !pointer.visible {
        return;
    }
    // В оригинале байт строки занимает 8 пикселей, а смещение `-8..=8`
    // сдвигает его в трёхбайтовом окне. Нативный компас рисует те же 8
    // колонок, поэтому левый край знака отстоит от центра на 4 точки.
    let x0 = cx + pointer.x as i32 * unit - 4 * unit;
    let y0 = cy - pointer.y as i32 * unit - 2 * unit;
    let sprite = &COMPASS_GLYPHS[pointer.glyph.index()];
    for (row, bits) in sprite.iter().enumerate() {
        for col in 0..8 {
            if bits & (0x80 >> col) != 0 {
                f.fill_rect(
                    x0 + col as i32 * unit,
                    y0 + row as i32 * unit,
                    unit as usize,
                    unit as usize,
                    style.line,
                );
            }
        }
    }
}
