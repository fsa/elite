//! Компас: кольцо и указатель на выбранную цель.

use crate::display::Frame;

use super::geometry::CockpitGeom;
use super::palette::{
    BACKGROUND, COMPASS_PLANET, COMPASS_STATION, COMPASS_SUN, CockpitStyle,
};
use super::values::{
    COMPASS_CIRCLE_MAX_OFFSET, COMPASS_CIRCLE_RADIUS_X2, CockpitValues, Compass, CompassCircle,
    CompassGlyph,
};

/// Растры указателя компаса из таблицы `CompassInd*` оригинала.
///
/// Это 4 строки по 8 пикселей. `RenewCompass` сдвигает байт строки в
/// трёхбайтовом окне, поэтому знак может уходить за центр кольца на 8 точек.
/// В открытом космосе `sub_6F29` выбирает один из четырёх последних растров:
/// `N` — солнце впереди, `S` — солнце позади, `W` — планета впереди,
/// `E` — планета позади. Первые два — обычный ромб и полый квадрат.
const COMPASS_GLYPHS: [[u8; 4]; 6] = [
    [0x18, 0x3C, 0x3C, 0x18], // CompassIndFill
    [0x3C, 0x24, 0x24, 0x3C], // CompassIndEmpty
    [0x09, 0x1A, 0x2C, 0x48], // CompassInd_N
    [0x0E, 0x10, 0x08, 0x70], // CompassInd_S
    [0x41, 0x82, 0xB4, 0x58], // CompassInd_W
    [0x0E, 0x1C, 0x20, 0x1C], // CompassInd_E
];

/// Режим круглого компаса в нативном кокпите.
///
/// Это чисто нативный переключатель: он не посылает клавиши Z80 и не меняет
/// режим сканера в эмуляторе. Клавиша `C` в паузе включает режим цветных
/// кругов: зелёная планета, синеватая станция и белое солнце. В обычном
/// режиме внутри системы показывается один круг цели, окрашенный по тому же
/// правилу: зелёный для планеты, синеватый для станции. В межпланетном
/// пространстве печатаются оригинальные растры «стороны света» (`N/S/W/E`) и
/// круги тел не показываются; по фазе [`CockpitValues::compass_blink_phase`]
/// чередуются пары знаков солнца и планеты. Внутри системы пока солнце и
/// планета не накладываются, они просто видны одновременно; при наложении
/// меняется порядок отрисовки, чтобы оба тела оставались различимы.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompassView {
    /// Один указатель на планету или станцию, как в обычном полёте.
    Pointer,
    /// Дополнительный режим: станция, если она есть, иначе планета и солнце.
    Bodies,
}

impl Default for CompassView {
    fn default() -> Self {
        CompassView::Pointer
    }
}

/// Нарисовать компас: кольцо и круг цели.
///
/// Размеры повторяют оригинал: кольцо 20x20 «пикселей» ZX, смещение цели
/// `-8..=8`. Радиус круга согласован с порогом наложения в
/// [`CockpitValues::compass_bodies_overlap`].
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
    if values.in_free_space {
        // Межпланетное пространство: исходные растры «стороны света». Фаза
        // переключает пары солнца и планеты, как это делает оригинальный
        // `sub_FD02`, но с фиксированным интервалом по настенным часам.
        draw_free_space_glyph(
            f,
            cx,
            cy,
            unit,
            style.line,
            &values.compass,
            values.compass_blink_phase,
        );
        return;
    }
    if view == CompassView::Bodies {
        // При наличии станции дополнительный режим показывает только её: планета
        // и солнце в этом режиме больше не нужны и только засоряют компас.
        if values.compass_station.visible {
            draw_circle(f, cx, cy, unit, COMPASS_STATION, &values.compass_station);
        } else if values.compass_bodies_overlap() && !values.compass_blink_phase {
            // Наложение: меняем порядок, чтобы в общей области попеременно
            // был виден то зелёный круг, то белый. Это заменяет старый хак с
            // переключением цели `PrintCompass`: тела больше не пропадают.
            draw_circle(f, cx, cy, unit, COMPASS_SUN, &values.compass_sun);
            draw_circle(f, cx, cy, unit, COMPASS_PLANET, &values.compass_planet);
        } else {
            draw_circle(f, cx, cy, unit, COMPASS_PLANET, &values.compass_planet);
            draw_circle(f, cx, cy, unit, COMPASS_SUN, &values.compass_sun);
        }
    } else {
        // Обычный режим: внутри системы — круг цели с той же расцветкой, что
        // и круги тел в дополнительном режиме.
        let pointer = &values.compass;
        let color = if values.station_near {
            COMPASS_STATION
        } else {
            COMPASS_PLANET
        };
        draw_circle(
            f,
            cx,
            cy,
            unit,
            color,
            &CompassCircle {
                x: pointer.x.clamp(-COMPASS_CIRCLE_MAX_OFFSET, COMPASS_CIRCLE_MAX_OFFSET),
                y: pointer.y.clamp(-COMPASS_CIRCLE_MAX_OFFSET, COMPASS_CIRCLE_MAX_OFFSET),
                behind: pointer.glyph.is_behind(),
                visible: pointer.visible,
            },
        );
    }
}

/// Мигающий знак стороны света в межпланетном пространстве.
///
/// Оригинал переключает цель `sub_FD02` каждый кадр Z80, поэтому знаки солнца
/// (`N/S`) и планеты (`W/E`) сменяются слишком часто. Здесь пары чередуются по
/// [`CockpitValues::compass_blink_phase`], а признак «позади» берётся из
/// текущего растра `PrintCompass`.
fn draw_free_space_glyph(
    f: &mut Frame,
    cx: i32,
    cy: i32,
    unit: i32,
    color: u32,
    pointer: &Compass,
    blink_phase: bool,
) {
    let behind = pointer.glyph.is_behind();
    let glyph = if blink_phase {
        if behind {
            CompassGlyph::SunBack
        } else {
            CompassGlyph::SunFront
        }
    } else if behind {
        CompassGlyph::PlanetBack
    } else {
        CompassGlyph::PlanetFront
    };
    draw_glyph(f, cx, cy, unit, color, &Compass { glyph, ..*pointer });
}

/// Растр указателя из `CompassInd*` в открытом космосе.
///
/// В оригинале байт строки занимает 8 пикселей, а смещение `-8..=8` сдвигает
/// его в трёхбайтовом окне. Нативный компас рисует те же 8 колонок, поэтому
/// левый край знака отстоит от центра на 4 точки.
fn draw_glyph(f: &mut Frame, cx: i32, cy: i32, unit: i32, color: u32, pointer: &Compass) {
    if !pointer.visible {
        return;
    }
    let index = match pointer.glyph {
        CompassGlyph::Filled => 0,
        CompassGlyph::Empty => 1,
        CompassGlyph::SunFront => 2,
        CompassGlyph::SunBack => 3,
        CompassGlyph::PlanetFront => 4,
        CompassGlyph::PlanetBack => 5,
    };
    let x0 = cx + pointer.x as i32 * unit - 4 * unit;
    let y0 = cy - pointer.y as i32 * unit - 2 * unit;
    for (row, bits) in COMPASS_GLYPHS[index].iter().enumerate() {
        for col in 0..8 {
            if bits & (0x80 >> col) != 0 {
                f.fill_rect(
                    x0 + col as i32 * unit,
                    y0 + row as i32 * unit,
                    unit as usize,
                    unit as usize,
                    color,
                );
            }
        }
    }
}

/// Круг тела: заливка для цели впереди, непрозрачный контур для сзади.
fn draw_circle(
    f: &mut Frame,
    cx: i32,
    cy: i32,
    unit: i32,
    color: u32,
    circle: &CompassCircle,
) {
    if !circle.visible {
        return;
    }
    let x = cx + circle.x as i32 * unit;
    let y = cy - circle.y as i32 * unit;
    let r = unit * COMPASS_CIRCLE_RADIUS_X2 / 2;
    if circle.behind {
        // Фон внутри контура не прозрачен: он перекрывает тела, нарисованные
        // раньше, поэтому при наложении верхний круг всегда читается.
        f.disc(x, y, r, BACKGROUND);
        for k in 0..unit {
            f.circle(x, y, (r - k).max(0), color);
        }
    } else {
        f.disc(x, y, r, color);
    }
}

#[cfg(test)]
mod tests {
    use super::super::palette::{COMPASS_PLANET, COMPASS_STATION};
    use super::super::values::{COMPASS_CIRCLE_RADIUS_X2, CompassCircle};
    use super::*;

    /// Контур перекрывает круг, нарисованный под ним, а не просвечивает.
    #[test]
    fn empty_circle_is_opaque() {
        let unit = 4;
        let center = (50, 50);
        let filled = CompassCircle {
            x: 0,
            y: 0,
            behind: false,
            visible: true,
        };
        let outline = CompassCircle {
            behind: true,
            ..filled
        };
        let mut f = Frame::new(100, 100);
        draw_circle(&mut f, center.0, center.1, unit, COMPASS_PLANET, &filled);
        assert_eq!(f.pixels()[50 * 100 + 50], COMPASS_PLANET);

        draw_circle(&mut f, center.0, center.1, unit, COMPASS_STATION, &outline);
        assert_eq!(f.pixels()[50 * 100 + 50], BACKGROUND);
        let rim = 50 + unit * COMPASS_CIRCLE_RADIUS_X2 / 2;
        assert_eq!(f.pixels()[50 * 100 + rim as usize], COMPASS_STATION);
    }
}
