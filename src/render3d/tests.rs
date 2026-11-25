//! Тесты заливки граней: стили и порядок перекрытия.

use crate::display::Frame;
use crate::shipdata::{self, Edge, Face, FaceStyle, Ship, Vertex};

use super::faces::{engine_core_color, face_color};
use super::{
    FLAME_COLOR, Rotation, SHIP_COLOR, SHIP_FACE_COLOR, SHIP_TINT_COUNT, Screen, ShipTint,
    draw_ship_tinted,
};

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
    draw_ship_tinted(&mut f, ship, &rot, [0.0, 0.0, 512.0], &screen, SHIP_COLOR, None);

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
        draw_ship_tinted(&mut f, ship, &rot, [0.0, 0.0, 512.0], &screen, SHIP_COLOR, None);
        assert_eq!(
            f.pixels()[200 * 400 + 200],
            SHIP_FACE_COLOR,
            "центр обломка должен быть залит цветом корабля"
        );
    }
}

/// Два треугольника в основании Viper — двигатели: они заливаются сиянием,
/// а не цветом корпуса. Вид сзади задан единичным поворотом: нормаль `-Z`
/// обращена к камере, объект стоит на `+Z`.
#[test]
fn viper_base_triangles_are_engines() {
    let ship = shipdata::find("d_Viper").expect("модель Viper не найдена");
    assert_eq!(ship.face_style(7), FaceStyle::EngineGlow);
    assert_eq!(ship.face_style(8), FaceStyle::EngineGlow);

    let mut f = Frame::new(400, 400);
    f.clear(0xFFFF_0000); // контрастный фон: непрозрачность заливки видна сразу
    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 1000.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_ship_tinted(
        &mut f,
        ship,
        &Rotation::identity(),
        [0.0, 0.0, 512.0],
        &screen,
        SHIP_COLOR,
        None,
    );

    // Цвет сияния пульсирует, но всегда остаётся сине-голубым; корпус и фон
    // таким не станут. Точки лежат внутри левого и правого треугольников.
    let is_glow = |x: usize, y: usize| {
        let c = f.pixels()[y * 400 + x];
        let (r, g, b) = ((c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF);
        (c >> 24) == 0xFF && b > g + 10 && g > r + 10 && b > 50
    };
    assert!(is_glow(166, 200), "левый двигатель не светится");
    assert!(is_glow(234, 200), "правый двигатель не светится");
}

/// Прямоугольник в основании Sidewinder — двигатель: он заливается сиянием,
/// а задняя грань вокруг него остаётся обычным корпусом. Контур выреза
/// сохраняется: рёбра принадлежат и задней грани, и грани двигателя.
#[test]
fn sidewinder_base_rectangle_is_engine() {
    let ship = shipdata::find("dSidewinderH").expect("модель Sidewinder не найдена");
    assert_eq!(ship.face_style(7), FaceStyle::EngineGlow);

    let mut f = Frame::new(400, 400);
    f.clear(0xFFFF_0000);
    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 1000.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_ship_tinted(
        &mut f,
        ship,
        &Rotation::identity(),
        [0.0, 0.0, 512.0],
        &screen,
        SHIP_COLOR,
        None,
    );

    let c = f.pixels()[200 * 400 + 200];
    let (r, g, b) = ((c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF);
    assert!(
        (c >> 24) == 0xFF && b > g + 10 && g > r + 10 && b > 50,
        "двигатель не светится: {c:08X}"
    );
    assert_eq!(
        f.pixels()[175 * 400 + 200],
        SHIP_FACE_COLOR,
        "задняя грань вокруг двигателя должна остаться корпусом"
    );
}

/// Двигатель — тёмная грань с маленькой яркой сердцевиной, а не однотонное
/// светящееся пятно. Ядро всегда ярче грани, поэтому читается как работа
/// сопла даже на тёмном фоне.
#[test]
fn engine_glow_has_small_bright_core() {
    let base = face_color(FaceStyle::EngineGlow, None, 0.0);
    let core = engine_core_color(0.0);
    let blue = |c: u32| c & 0xFF;
    assert!(
        blue(core) > blue(base) + 80,
        "сердцевина должна быть заметно ярче грани: {core:08X} vs {base:08X}"
    );

    let ship = shipdata::find("d_Viper").expect("модель Viper не найдена");
    let mut f = Frame::new(400, 400);
    f.clear(0xFFFF_0000);
    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 1000.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_ship_tinted(
        &mut f,
        ship,
        &Rotation::identity(),
        [0.0, 0.0, 512.0],
        &screen,
        SHIP_COLOR,
        None,
    );
    let is_core = |c: u32| {
        let (r, g, b) = ((c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF);
        // грань даёт не больше 108 в синем канале, ядро — не меньше 140;
        // белые рёбра тоже яркие, но у них все каналы равны
        b > 130 && b > g + 10 && g > r + 10
    };
    let bright = (180..=220).any(|y| {
        (140..=260).any(|x| is_core(f.pixels()[y * 400 + x]))
    });
    assert!(bright, "яркая сердцевина двигателя не отрисовалась");
}

/// Оперение ракеты — отдельные грани, а не прозрачные рёбра чужих граней.
/// Два противоположных пера лежат в одной плоскости, поэтому проверяем
/// заливку с обеих сторон этой плоскости.
#[test]
fn missile_fins_are_filled() {
    let ship = shipdata::find("d_Missile").expect("модель ракеты не найдена");
    let screen = Screen {
        cx: 200.0,
        cy: 200.0,
        k: 1000.0,
        bx: 0.0,
        by: 0.0,
    };
    let render = |angle: f32| {
        let rot = Rotation::axis_angle([0.0, 1.0, 0.0], angle);
        let mut f = Frame::new(400, 400);
        f.clear(0xFFFF_0000);
        draw_ship_tinted(&mut f, ship, &rot, [0.0, 0.0, 512.0], &screen, SHIP_COLOR, None);
        f
    };

    // Маленькие треугольники за задним срезом корпуса: без нормалей оперения
    // там оставался бы фон.
    let front = render(std::f32::consts::FRAC_PI_4);
    for &(x, y) in &[(110, 179), (110, 220)] {
        assert_eq!(
            front.pixels()[y * 400 + x],
            SHIP_FACE_COLOR,
            "перо ({x},{y}) не залито"
        );
    }

    let back = render(-std::f32::consts::FRAC_PI_4);
    let fin_pixels = (0..180)
        .flat_map(|y| (270..400).map(move |x| (x, y)))
        .filter(|&(x, y)| back.pixels()[y * 400 + x] == SHIP_FACE_COLOR)
        .count();
    assert!(fin_pixels > 0, "перо не залито с другой стороны плоскости");
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
        tintable: true,
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
    draw_ship_tinted(
        &mut f,
        &SHIP,
        &Rotation::identity(),
        [0.0, 0.0, 512.0],
        &screen,
        SHIP_COLOR,
        None,
    );

    assert_eq!(
        f.pixels()[100 * 200 + 100],
        NEAR,
        "ближняя грань должна перекрывать дальную"
    );
}

#[test]
fn tint_keeps_original_brightness_and_changes_all_hull_faces() {
    assert_eq!(
        SHIP_FACE_COLOR, 0xFF14_1820,
        "базовая заливка должна остаться тёмной"
    );

    let tint = ShipTint::from_seed(5);
    let hull = tint.apply(SHIP_FACE_COLOR);
    assert_eq!(hull >> 24, 0xFF, "альфа не должна меняться");
    let sum = |c: u32| ((c >> 16) & 0xFF) + ((c >> 8) & 0xFF) + (c & 0xFF);
    assert!(
        sum(hull) <= sum(SHIP_FACE_COLOR) + 1,
        "оттенок не должен делать корпус ярче: {} → {}",
        sum(SHIP_FACE_COLOR),
        sum(hull)
    );
    let max_delta = [16, 8, 0]
        .iter()
        .map(|&shift| {
            ((SHIP_FACE_COLOR >> shift) & 0xFF).abs_diff((hull >> shift) & 0xFF)
        })
        .max()
        .expect("три канала есть всегда");
    assert!(
        max_delta >= 10,
        "оттенок должен быть заметен на тёмном металле: {hull:08X}"
    );
    assert_ne!(hull, SHIP_FACE_COLOR, "корпус должен получить оттенок");

    let edges = tint.apply(SHIP_COLOR);
    assert_ne!(edges, SHIP_COLOR, "рёбра тоже должны получить оттенок");
    assert!(
        [16, 8, 0]
            .iter()
            .all(|&shift| ((edges >> shift) & 0xFF) >= 200),
        "рёбра должны остаться почти белыми: {edges:08X}"
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
fn tints_use_only_red_green_and_blue_hues() {
    let channels = |c: u32| [(c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF];
    let base = channels(SHIP_FACE_COLOR);
    let mut raised = [false; 3];
    for id in 0..SHIP_TINT_COUNT {
        let tinted = channels(ShipTint::from_seed(id).apply(SHIP_FACE_COLOR));
        let up: Vec<usize> = (0..3).filter(|&i| tinted[i] > base[i]).collect();
        assert_eq!(
            up.len(),
            1,
            "тон {id} смешивает несколько каналов: {tinted:?}"
        );
        raised[up[0]] = true;
        assert!(
            tinted.iter().sum::<u32>() <= base.iter().sum::<u32>(),
            "тон {id} делает корпус ярче"
        );
    }
    assert!(
        raised.iter().all(|&seen| seen),
        "палитра должна содержать красный, зелёный и синий: {raised:?}"
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
