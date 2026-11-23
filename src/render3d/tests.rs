//! Тесты заливки граней: стили и порядок перекрытия.

use crate::display::Frame;
use crate::shipdata::{self, Edge, Face, FaceStyle, Ship, Vertex};

use super::{Rotation, Screen, SHIP_COLOR, SHIP_FACE_COLOR, draw_ship};

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
