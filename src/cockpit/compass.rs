//! Компас: кольцо и указатель на выбранную цель.

use crate::display::Frame;

use super::geometry::CockpitGeom;
use super::palette::{
    BACKGROUND, COMPASS_PLANET, COMPASS_STATION, COMPASS_SUN, CockpitStyle,
};
use super::values::{
    COMPASS_CIRCLE_MAX_OFFSET, COMPASS_CIRCLE_RADIUS_X2, CockpitValues, CompassCircle,
};

/// Режим круглого компаса в нативном кокпите.
///
/// Это чисто нативный переключатель: он не посылает клавиши Z80 и не меняет
/// режим сканера в эмуляторе. Клавиша `C` в паузе включает режим цветных
/// кругов: зелёная планета, серый станция и белое солнце. В обычном режиме
/// один круг тоже окрашен: зелёный для планеты и обычный белый для станции,
/// чтобы было видно, что компас перестроился. Пока солнце и планета не
/// накладываются, они просто видны одновременно; при наложении меняется
/// порядок отрисовки, чтобы оба тела оставались различимы.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompassView {
    /// Один указатель на планету или станцию, как в обычном полёте.
    Pointer,
    /// Цветные круги тел: планета, станция и солнце.
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
    if view == CompassView::Bodies {
        // Станцию рисуем первой: при совпадении круг планеты или солнца
        // должен оставаться сверху.
        draw_circle(f, cx, cy, unit, COMPASS_STATION, &values.compass_station);
        if values.compass_bodies_overlap() && !values.compass_overlap_phase {
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
        // Обычный режим: тот же круг, но зелёный для планеты и белый для
        // станции. Так сразу видно, на что перестроился компас.
        let pointer = &values.compass;
        let color = if values.station_near && !values.in_free_space {
            style.line
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
