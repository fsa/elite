//! Тесты геометрии и отрисовки овального радара.

use crate::display::Frame;

use super::super::geometry::{Rect, panel_geometry};
use super::super::palette::{CockpitStyle, FRAME_COLOR};
use super::super::values::{CockpitValues, RadarContact, RadarContactKind, RadarContacts};
use super::{draw, oval, plot_contact, RADAR_RANGE};

const W: usize = 1280;
const H: usize = 960;

/// Цвет линий радара (чуть ярче линий кокпита).
fn ink() -> u32 {
    oval::radar_color(FRAME_COLOR)
}

fn contact(x: i32, y: i32, z: i32) -> RadarContact {
    RadarContact {
        x,
        y,
        z,
        kind: RadarContactKind::Ship,
    }
}

fn values(contacts: &[RadarContact]) -> CockpitValues {
    let mut v = CockpitValues::for_splash();
    let mut radar = RadarContacts::empty();
    for c in contacts {
        radar.push(*c);
    }
    v.radar = radar;
    v
}

/// Нарисовать радар и вернуть кадр с его клеткой.
fn draw_radar(contacts: &[RadarContact]) -> (Frame, Rect) {
    let mut f = Frame::new(W, H);
    let g = panel_geometry(W as u32, H as u32);
    draw(&mut f, &g, CockpitStyle::COBRA_MK3, &values(contacts));
    (f, g.oval_radar_cell())
}

fn px(f: &Frame, x: i32, y: i32) -> u32 {
    f.pixels()[y as usize * W + x as usize]
}

fn lit_in(f: &Frame, cell: Rect) -> usize {
    let mut lit = 0;
    for y in cell.y..cell.y + cell.h {
        for x in cell.x..cell.x + cell.w {
            if px(f, x, y) == ink() {
                lit += 1;
            }
        }
    }
    lit
}

fn assert_only_in(f: &Frame, cell: Rect) {
    for y in 0..H as i32 {
        for x in 0..W as i32 {
            let inside = x >= cell.x && x < cell.x + cell.w && y >= cell.y && y < cell.y + cell.h;
            if !inside {
                assert_eq!(px(f, x, y), 0, "рисование вне клетки радара ({x},{y})");
            }
        }
    }
}

/// Параметры эллипса так же, как их считает отрисовка.
fn ellipse_params(cell: Rect) -> (i32, i32, i32, i32) {
    let inner = cell.inset(1);
    (
        inner.x + inner.w / 2,
        inner.y + inner.h / 2,
        (inner.w / 2 - 1).max(1),
        (inner.h / 2 - 1).max(1),
    )
}

fn half_span(rx: i32, ry: i32, dy: i32) -> i32 {
    let t = dy as f32 / ry as f32;
    (rx as f32 * (1.0 - t * t).max(0.0).sqrt()).round() as i32
}

/// Непрерывные отрезки горящих пикселей в строке.
fn lit_runs(f: &Frame, y: i32, x0: i32, x1: i32) -> Vec<(i32, i32)> {
    let mut runs = Vec::new();
    let mut start = None;
    for x in x0..=x1 {
        match (px(f, x, y) == ink(), start) {
            (true, None) => start = Some(x),
            (false, Some(s)) => {
                runs.push((s, x - 1));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        runs.push((s, x1));
    }
    runs
}

/// Самый длинный разрыв между отрезками.
fn max_gap(runs: &[(i32, i32)]) -> i32 {
    runs.windows(2)
        .map(|w| w[1].0 - w[0].1 - 1)
        .max()
        .unwrap_or(0)
}

/// Радар занимает всю свободную высоту центра и не задевает кружки.
#[test]
fn radar_uses_free_height_and_stays_between_circles() {
    for (w, h) in [(1024u32, 768u32), (1280, 960), (1706, 960), (1920, 1080)] {
        let g = panel_geometry(w, h);
        let area = g.radar_area();
        let cell = g.oval_radar_cell();
        assert!(!cell.is_empty(), "{w}x{h}: пустая клетка радара");
        assert_eq!(cell.h, area.h, "{w}x{h}: радар не занял высоту");
        let aspect = cell.w as f64 / cell.h as f64;
        assert!(
            (aspect - 12.0 / 5.0).abs() < 0.05,
            "{w}x{h}: овал не 12:5, а {aspect}"
        );
        assert!(
            cell.x > g.danger_cell().right(),
            "{w}x{h}: овал наезжает на кружок опасности"
        );
        assert!(
            cell.right() < g.compass_cell().x,
            "{w}x{h}: овал наезжает на компас"
        );
        // Овал по центру свободной области.
        let left = cell.x - area.x;
        let right = area.right() - cell.right();
        assert!((left - right).abs() <= 1, "{w}x{h}: овал не по центру");
    }
}

/// Контакт переводится в пиксели: X — вправо, Z — вперёд (вверх),
/// Y — высота вертикального штриха.
#[test]
fn plot_contact_maps_player_coordinates() {
    let cell = Rect {
        x: 100,
        y: 50,
        w: 240,
        h: 100,
    };
    let center = plot_contact(cell, &contact(0, 0, 0)).unwrap();
    assert_eq!((center.x, center.y, center.height), (220, 100, 0));

    let right = plot_contact(cell, &contact(RADAR_RANGE as i32 / 2, 0, 0)).unwrap();
    assert_eq!(right.x, 280);
    let forward = plot_contact(cell, &contact(0, 0, RADAR_RANGE as i32 / 2)).unwrap();
    assert_eq!(forward.y, 75, "передний контакт должен быть выше");

    let above = plot_contact(cell, &contact(0, RADAR_RANGE as i32 / 2, 0)).unwrap();
    assert_eq!(above.height, 25);
    let below = plot_contact(cell, &contact(0, -(RADAR_RANGE as i32 / 2), 0)).unwrap();
    assert_eq!(below.height, -25);
}

/// Контакты вне дальности и за контуром овала не показываются.
#[test]
fn contacts_outside_radar_are_hidden() {
    let cell = Rect {
        x: 0,
        y: 0,
        w: 200,
        h: 100,
    };
    let far = contact(RADAR_RANGE as i32 * 2, 0, 0);
    assert!(plot_contact(cell, &far).is_none());

    let edge = contact(RADAR_RANGE as i32, 0, 0);
    let plot = plot_contact(cell, &edge).unwrap();
    assert_eq!(plot.x, cell.x + cell.w - 1, "контакт вышел за край");

    let diag = (RADAR_RANGE * 0.9) as i32;
    assert!(
        plot_contact(cell, &contact(diag, 0, diag)).is_none(),
        "контакт за контуром эллипса должен быть скрыт"
    );
}

/// Радар рисует только в своей клетке.
#[test]
fn radar_draws_only_inside_its_cell() {
    let (f, cell) = draw_radar(&[]);
    assert!(lit_in(&f, cell) > 0, "радар ничего не нарисовал");
    assert_only_in(&f, cell);
}

/// Пустой радар: контур овала и вся разметка.
#[test]
fn empty_radar_draws_oval_and_graticule() {
    let (f, cell) = draw_radar(&[]);
    let (cx, cy, rx, ry) = ellipse_params(cell);
    for (name, x, y) in [
        ("верх", cx, cy - ry),
        ("низ", cx, cy + ry),
        ("лево", cx - rx, cy),
        ("право", cx + rx, cy),
    ] {
        assert_eq!(px(&f, x, y), ink(), "нет контура овала: {name}");
    }
    // Три горизонтальные линии на четвертях высоты.
    for y in [cy - ry / 2, cy, cy + ry / 2] {
        let runs = lit_runs(&f, y, cx - rx + 3, cx + rx - 3);
        assert!(runs.len() >= 3, "горизонтальная линия y={y} не пунктирная");
    }
    // Вертикаль от вершины треугольника вниз.
    assert_eq!(px(&f, cx, cy + ry / 3), ink(), "нет вертикали вниз");
}

/// Средняя линия заметно реже верхней и нижней.
#[test]
fn middle_line_is_sparser_than_outer_lines() {
    let (f, cell) = draw_radar(&[]);
    let (cx, cy, rx, ry) = ellipse_params(cell);
    let half = ry / 2;
    let span = half_span(rx, ry, half);

    let bottom = lit_runs(&f, cy + half, cx - span + 3, cx + span - 3);
    let middle = lit_runs(&f, cy, cx - rx + 3, cx + rx - 3);
    let top_left = lit_runs(&f, cy - half, cx - span + 3, cx - rx / 3);
    let top_right = lit_runs(&f, cy - half, cx + rx / 3, cx + span - 3);

    assert!(!bottom.is_empty(), "нет нижней линии");
    assert!(!middle.is_empty(), "нет средней линии");
    assert!(
        max_gap(&middle) > max_gap(&bottom),
        "средняя линия не реже нижней: {} против {}",
        max_gap(&middle),
        max_gap(&bottom)
    );
    assert!(!top_left.is_empty(), "нет верхней линии слева");
    assert!(!top_right.is_empty(), "нет верхней линии справа");
    // Внутри треугольника верхняя линия не рисуется.
    let inside = lit_runs(&f, cy - half, cx - rx / 8, cx + rx / 8);
    assert!(inside.is_empty(), "верхняя линия пересекает треугольник");
}

/// Треугольник сверху: сплошные стороны и пунктирные лучи под 45°.
#[test]
fn triangle_and_diagonals_match_original() {
    let (f, cell) = draw_radar(&[]);
    let (cx, cy, rx, ry) = ellipse_params(cell);
    let tri_dx = (rx * 36 / 100).max(1);
    let tri_dy = (ry * 83 / 100).max(1);
    let top = cy - tri_dy;
    let diag_dash = (ry / 20).max(2);

    // Верхние вершины треугольника.
    assert_eq!(px(&f, cx - tri_dx, top), ink(), "нет левой вершины");
    assert_eq!(px(&f, cx + tri_dx, top), ink(), "нет правой вершины");
    // Стороны сходятся в центре.
    assert_eq!(px(&f, cx, cy), ink(), "нет вершины треугольника");

    // Лучи от верхних вершин идут вниз-влево и вниз-вправо и пунктирны.
    let on = diag_dash * 2;
    // Сдвиг на пиксель от начала пропуска: линия толще одного пикселя.
    let off = diag_dash * 3 + 1;
    assert!(on < tri_dy && off < tri_dy, "лучи короче проверяемых точек");
    assert_eq!(
        px(&f, cx - tri_dx - on, top + on),
        ink(),
        "нет левого луча"
    );
    assert_eq!(
        px(&f, cx + tri_dx + on, top + on),
        ink(),
        "нет правого луча"
    );
    assert_eq!(
        px(&f, cx - tri_dx - off, top + off),
        0,
        "левый луч не пунктирный"
    );
    assert_eq!(
        px(&f, cx + tri_dx + off, top + off),
        0,
        "правый луч не пунктирный"
    );
}

/// Один и тот же контакт меняет картинку радара.
#[test]
fn contacts_are_drawn_on_radar() {
    let c = contact(RADAR_RANGE as i32 / 2, 0, 0);
    let (empty, cell) = draw_radar(&[]);
    let (full, _) = draw_radar(&[c]);
    assert!(
        lit_in(&full, cell) > lit_in(&empty, cell),
        "радар не показал контакт"
    );
}

/// Контакт вне дальности не меняет картинку.
#[test]
fn far_contacts_do_not_change_radar() {
    let far = contact(RADAR_RANGE as i32 * 2, 0, 0);
    let (empty, cell) = draw_radar(&[]);
    let (full, _) = draw_radar(&[far]);
    assert_eq!(lit_in(&full, cell), lit_in(&empty, cell));
}

/// Контакт — буква «Г»: засечка на дальнем конце штриха высоты,
/// а не у самого контакта.
#[test]
fn contact_looks_like_gamma_not_tee() {
    let (f, cell) = draw_radar(&[contact(RADAR_RANGE as i32 / 4, RADAR_RANGE as i32 / 2, RADAR_RANGE as i32 / 8)]);
    let (cx, cy, rx, ry) = ellipse_params(cell);
    let x = cx + rx / 4;
    let base = cy - ry / 8;
    let end = base - ry / 2;
    let bar = (cell.w / 60).max(2);

    assert_eq!(px(&f, x, base - ry / 4), ink(), "нет штриха высоты");
    assert_eq!(px(&f, x, end), ink(), "штрих не дошёл до высоты");
    assert_eq!(
        px(&f, x - bar, end),
        ink(),
        "нет засечки на дальнем конце"
    );
    assert_eq!(
        px(&f, x - bar, base),
        0,
        "засечка осталась у контакта — получилась буква Т"
    );
}

/// Высота объекта — вертикальный штрих от контакта.
#[test]
fn vertical_stroke_shows_object_height() {
    let high = contact(0, RADAR_RANGE as i32 / 2, 0);
    let (empty, cell) = draw_radar(&[]);
    let (full, _) = draw_radar(&[high]);
    let (cx, cy, _, ry) = ellipse_params(cell);
    let y = cy - ry / 4;
    assert_eq!(px(&empty, cx, y), 0, "штрих нарисован без контакта");
    assert_eq!(px(&full, cx, y), ink(), "нет штриха высоты");
}

/// Слишком маленькая клетка не ломает отрисовку.
#[test]
fn tiny_cells_are_skipped() {
    let mut f = Frame::new(4, 4);
    let v = values(&[contact(0, 0, 0)]);
    for cell in [
        Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        },
        Rect {
            x: 0,
            y: 0,
            w: 2,
            h: 2,
        },
    ] {
        oval::draw(&mut f, cell, CockpitStyle::COBRA_MK3, &v);
    }
}



