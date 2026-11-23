//! Тесты заливки граней: стили и порядок перекрытия.

use crate::display::Frame;
use crate::shipdata::{self, Edge, Face, FaceStyle, Ship, Vertex};

use super::faces::face_color;
use super::{FLAME_COLOR, Rotation, SHIP_COLOR, SHIP_FACE_COLOR, Screen, ShipTint, draw_ship};

/// Вход в станцию — чёрная поверхность, а не дыра, сквозь которую виден фон.
///
/// Грань входа (`+Z` модели) показана поворотом на 180°: в камере оригинала
/// объект стоит на `+Z`, поэтому к наблюдателю обращена нормаль `-Z`.
#[test]
fn station_opening_is_black() {
    let ship = shipdata::find("d_Coriolis").expect("модель станции не найдена");
    assert_eq!(ship.face_style(0), FaceStyle::Hull, "обычный стиль грани");
    assert_eq!(
        ship.face_style(14),
        FaceStyle::Color(0xFF00_0000),
        "вход должен иметь чёрный стиль"
    );

    let mut f = Frame::new(400, 400);
    f.clear(0xFFFF_0000); // контрастный фон: если дыра осталась, будет виден он
    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 400.0,
        bx: 0.0,
        by: 0.0,
    };
    let rot = Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::PI);
    draw_ship(&mut f, ship, &rot, [0.0, 0.0, 512.0], &screen, SHIP_COLOR);

    assert_eq!(
        f.pixels()[200 * 400 + 200],
        0xFF00_0000,
        "центр входа должен быть чёрным"
    );
    assert_eq!(
        f.pixels()[200 * 400 + 250],
        SHIP_FACE_COLOR,
        "корпус вокруг входа должен остаться обычным"
    );
}

/// Обломки корабля (`d_Alloys`) в ROM не имеют нормалей, поэтому без
/// добавленных граней они оставались белым каркасом. Теперь они заливаются
/// обычным цветом корпуса — в том числе после поворота на 180°.
#[test]
fn ship_debris_is_filled_with_hull_color() {
    let ship = shipdata::find("d_Alloys").expect("модель обломков не найдена");
    assert_eq!(ship.faces.len(), 2, "у обломков должны быть две стороны");
    assert_eq!(ship.face_style(0), FaceStyle::Hull, "стиль по умолчанию");

    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 1000.0,
        bx: 0.0,
        by: 0.0,
    };
    for rot in [
        Rotation::identity(),
        Rotation::axis_angle([0.0, 1.0, 0.0], std::f32::consts::PI),
    ] {
        let mut f = Frame::new(400, 400);
        f.clear(0xFFFF_0000);
        draw_ship(&mut f, ship, &rot, [0.0, 0.0, 512.0], &screen, SHIP_COLOR);
        assert_eq!(
            f.pixels()[200 * 400 + 200],
            SHIP_FACE_COLOR,
            "центр обломка должен быть залит цветом корабля"
        );
    }
}

/// Ближняя грань со своим цветом перекрывает дальную, даже если в списке
/// граней она идёт первой.
#[test]
fn nearer_face_color_wins_over_farther_one() {
    const NEAR: u32 = 0xFF0000FF;
    static VERTS: [Vertex; 8] = [
        Vertex { x: -20, y: -20, z: -100, lod: 0x1F, faces: [0, 0] },
        Vertex { x: 20, y: -20, z: -100, lod: 0x1F, faces: [0, 0] },
        Vertex { x: 20, y: 20, z: -100, lod: 0x1F, faces: [0, 0] },
        Vertex { x: -20, y: 20, z: -100, lod: 0x1F, faces: [0, 0] },
        Vertex { x: -30, y: -30, z: 100, lod: 0x1F, faces: [1, 1] },
        Vertex { x: 30, y: -30, z: 100, lod: 0x1F, faces: [1, 1] },
        Vertex { x: 30, y: 30, z: 100, lod: 0x1F, faces: [1, 1] },
        Vertex { x: -30, y: 30, z: 100, lod: 0x1F, faces: [1, 1] },
    ];
    static FACES: [Face; 2] = [
        Face { nx: 0, ny: 0, nz: -100, flags: 0 },
        Face { nx: 0, ny: 0, nz: -100, flags: 0 },
    ];
    static EDGES: [Edge; 8] = [
        Edge { lod: 0x1F, v: [0, 1], faces: 0x00 },
        Edge { lod: 0x1F, v: [1, 2], faces: 0x00 },
        Edge { lod: 0x1F, v: [2, 3], faces: 0x00 },
        Edge { lod: 0x1F, v: [3, 0], faces: 0x00 },
        Edge { lod: 0x1F, v: [4, 5], faces: 0x11 },
        Edge { lod: 0x1F, v: [5, 6], faces: 0x11 },
        Edge { lod: 0x1F, v: [6, 7], faces: 0x11 },
        Edge { lod: 0x1F, v: [7, 4], faces: 0x11 },
    ];
    static SHIP: Ship = Ship {
        name: "two faces",
        full_name: "Two Faces",
        addr: 0,
        category: 0,
        stats: unsafe { std::mem::zeroed() },
        vertices: &VERTS,
        edges: &EDGES,
        faces: &FACES,
        face_styles: &[(0, FaceStyle::Color(NEAR))],
    };

    let mut f = Frame::new(200, 200);
    f.clear(0xFFFF_0000);
    let screen = Screen {
        cx: 100.0,
        cy: 100.0,
        k: 2000.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_ship(&mut f, &SHIP, &Rotation::identity(), [0.0, 0.0, 512.0], &screen, SHIP_COLOR);

    assert_eq!(
        f.pixels()[100 * 200 + 100],
        NEAR,
        "ближняя грань должна перекрывать дальную"
    );
}

#[test]
fn tint_keeps_metal_moderate_and_changes_all_hull_faces() {
    let channels = [
        (SHIP_FACE_COLOR >> 16) & 0xFF,
        (SHIP_FACE_COLOR >> 8) & 0xFF,
        SHIP_FACE_COLOR & 0xFF,
    ];
    assert!(
        channels.iter().all(|&c| c <= 160),
        "базовый корпус слишком яркий: {channels:?}"
    );
    assert!(
        channels.iter().all(|&c| c >= 100),
        "базовый корпус слишком тёмный: {channels:?}"
    );

    let tint = ShipTint::from_seed(5);
    let hull = tint.apply(SHIP_FACE_COLOR);
    assert_eq!(hull >> 24, 0xFF, "альфа не должна меняться");
    for shift in [16, 8, 0] {
        let base = (SHIP_FACE_COLOR >> shift) & 0xFF;
        let tinted = (hull >> shift) & 0xFF;
        assert!(
            tinted + 24 >= base && base + 24 >= tinted,
            "оттенок слишком сильно меняет яркость металла: {base} → {tinted}"
        );
    }
    assert_ne!(hull, SHIP_FACE_COLOR, "корпус должен получить оттенок");
    assert_ne!(
        tint.apply(SHIP_COLOR),
        SHIP_COLOR,
        "рёбра тоже должны получить оттенок"
    );
    assert_eq!(
        tint.apply(0xFF00_0000),
        0xFF00_0000,
        "чёрный вход в станцию должен остаться чёрным"
    );
    assert_ne!(
        ShipTint::from_seed(0).apply(SHIP_FACE_COLOR),
        ShipTint::from_seed(4).apply(SHIP_FACE_COLOR),
        "разные номера тона должны давать разные оттенки"
    );
}

#[test]
fn tint_does_not_touch_engines() {
    let tint = ShipTint::from_seed(3);
    assert_eq!(
        face_color(FaceStyle::Hull, Some(tint), 0.0),
        tint.apply(SHIP_FACE_COLOR)
    );
    assert_eq!(
        face_color(FaceStyle::Color(0xFF12_3456), Some(tint), 0.0),
        tint.apply(0xFF12_3456)
    );
    assert_eq!(
        face_color(FaceStyle::Flame, Some(tint), 0.0),
        FLAME_COLOR,
        "пламя двигателя не красится"
    );
    assert_eq!(
        face_color(FaceStyle::EngineGlow, Some(tint), 0.7),
        face_color(FaceStyle::EngineGlow, None, 0.7),
        "сияние двигателя не красится"
    );
}
