//! Тесты нативной карты галактики.

use super::*;

/// Семя и координаты из памяти запущенной версии Кладова (tmp/wf/mem.bin).
fn fake_mem() -> Vec<u8> {
    let mut m = vec![0u8; 0x10000];
    m[0xD090..0xD096].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
    m[0xD097] = 173; // Y системы (LAVE)
    m[0xD099] = 20; // X системы (LAVE)
    m[0xD065] = 2; // номер галактики (PilotNumGalaxy, 0-based)
    m[0xD082] = 70; // топливо (Pilot_Fuel)
    m[0xFDB3..0xFDB3 + 64]
        .copy_from_slice(b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION");
    m
}

fn opened() -> Map {
    let mem = fake_mem();
    let mut map = Map::new();
    map.open(&mem, galaxy::KLADOV_GALAXY);
    map.win_w = 1024;
    map.win_h = 768;
    map
}

#[test]
fn open_builds_galaxy_and_centers_cursor_on_system() {
    let map = opened();
    assert!(map.active);
    assert_eq!(map.planets.len(), galaxy::COUNT);
    assert_eq!((map.cursor_x, map.cursor_y_raw), (20, 173));
    assert_eq!(map.name_of(7), "LAVE");
    assert_eq!(map.name_of(204), "ORESQU");
    assert_eq!(map.nearest_index(), Some(7));
    // Номер галактики показывается как 1..8.
    assert_eq!(map.galaxy_number(), 3);
    // Зум сбрасывается на «вся галактика».
    assert_eq!(map.zoom(), 1.0);
    // Круг дальности прыжка: топливо 70 -> радиус 17 единиц поля.
    assert_eq!(galaxy::jump_radius(map.fuel), 17);
}

#[test]
fn cursor_moves_within_view_and_snaps_to_planet() {
    let mut map = opened();
    assert_eq!(map.nearest_index(), Some(7));
    map.move_by(1000, 1000);
    assert!(map.cursor().0 <= 255 && map.cursor().1 <= 255);
    // Щелчок по ORESQU: курсор встаёт точно на звезду.
    map.cursor_x = 22;
    map.cursor_y_raw = 217;
    assert_eq!(map.nearest_index(), Some(204));
    map.cursor_x = 21;
    map.cursor_y_raw = 216;
    map.snap_to_nearest();
    assert_eq!((map.cursor_x, map.cursor_y_raw), (22, 217));
}

/// Координаты выбранной планеты для чита телепортации (F1): те же числа,
/// что игра хранит в `Pilot_CrdXSystem`/`Pilot_CrdYSystem`.
#[test]
fn selected_coords_follow_cursor() {
    let mut map = opened();
    // Сразу после открытия курсор стоит на текущей системе — LAVE.
    assert_eq!(map.selected_coords(), Some((20, 173)));
    // Наводимся на ORESQU (№204) — координаты меняются вместе с курсором.
    map.cursor_x = 22;
    map.cursor_y_raw = 217;
    map.snap_to_nearest();
    assert_eq!(map.selected_coords(), Some((22, 217)));
}

/// Расстояние до курсора считается так же, как в игре: LAVE -> DISO
/// = 3.2 св. года в версии Кладова (в оригинале Elite — 3.6), а
/// LAVE -> RIEDQUAD = 6.4 (в оригинале — 6.8). Разница — особенность
/// модуля разности у Кладова.
#[test]
fn distance_to_cursor_matches_game() {
    let mut map = opened();
    // Курсор на текущей системе — расстояние ноль.
    assert_eq!(map.distance_to_cursor(), 0.0);
    // Курсор на DISO (№147).
    map.cursor_x = 11;
    map.cursor_y_raw = 174;
    assert_eq!(map.distance_to_cursor(), 3.2);
    // Курсор на RIEDQUAD (№46).
    map.cursor_x = 3;
    map.cursor_y_raw = 181;
    assert_eq!(map.distance_to_cursor(), 6.4);
    assert_eq!(map.nearest_index(), Some(46));
}

/// Поле 256x128 вписывается в окно с сохранением пропорций. Сверху и
/// снизу зарезервированы полосы под заголовок и панель.
#[test]
fn layout_keeps_field_proportions() {
    let map = Map::new();
    let l = map.layout(1024, 768);
    // 1024/256 = 4, доступная высота больше -> узкое место — горизонталь.
    assert_eq!(l.scale, 4.0);
    assert_eq!((l.fx0, l.fw), (0, 1024));
    assert_eq!(l.fh, 512);
    assert_eq!(l.text, 2);
    assert!(l.fy0 >= l.top, "поле заходит под заголовок");
    assert!(l.fy0 + l.fh <= l.panel_y, "поле заходит под панель");
    assert!(l.panel_y < 768);
}

/// На разных соотношениях сторон (16:9, 16:10, 4:3) поле не налезает
/// на заголовок: его верх всегда ниже конца строки заголовка.
#[test]
fn field_never_overlaps_title() {
    let map = Map::new();
    for &(w, h) in &[
        (1280u32, 720u32), // 16:9
        (1920, 1080),      // 16:9 FullHD
        (1440, 900),       // 16:10
        (1680, 1050),      // 16:10
        (1024, 768),       // 4:3
        (3840, 2160),      // 16:9 4K
    ] {
        let l = map.layout(w, h);
        let title_bottom = l.title_y + 8 * l.text as i32;
        assert!(
            l.fy0 >= title_bottom,
            "{w}x{h}: поле ({}) налезает на заголовок (низ {})",
            l.fy0,
            title_bottom
        );
        assert!(l.fy0 >= l.top);
        assert!(
            l.fy0 + l.fh <= l.panel_y,
            "{w}x{h}: поле заходит под панель"
        );
    }
}

/// Наведение мышью: центр поля -> центр карты (курсор не двигает карту).
#[test]
fn map_is_fixed_and_mouse_points_at_field() {
    let mut map = opened();
    let l = map.layout(1024, 768);
    map.point_at(
        l.fx0 as f64 + 128.0 * l.scale as f64,
        l.fy0 as f64 + 64.0 * l.scale as f64,
    );
    assert_eq!((map.cursor_x, map.cursor_y_raw), (128, 128));
    // Карта не сдвинулась от движения курсора.
    let l2 = map.layout(1024, 768);
    assert_eq!((l.fx0, l.fy0), (l2.fx0, l2.fy0));
}

/// Зум всегда идёт вокруг текущей системы, а на максимуме в кадр
/// помещается полный радиус прыжка (7 св. лет) — детализация карты `O`.
#[test]
fn zoom_centers_on_system_and_shows_jump_range() {
    let mut map = opened();
    // Приближаем до предела.
    for _ in 0..80 {
        map.zoom_by(1);
    }
    assert!(map.zoom() > 1.0);
    assert!((map.zoom() - map.max_zoom()).abs() < 1e-3);
    let l = map.layout(1024, 768);
    let (x0, y0, x1, y1) = l.visible_rect();
    // Текущая система видна в кадре.
    let sys = map.system_index().unwrap();
    let p = &map.planets[sys];
    let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
    assert!(
        sx >= x0 && sx < x1 && sy >= y0 && sy < y1,
        "система вне кадра"
    );
    // Полный радиус прыжка помещается в меньшую сторону кадра.
    let range = map.range_px(l.scale);
    let half = l.vw.min(l.vh) / 2;
    assert!(range <= half, "круг дальности {range}px > {half}px");
    // Отдаляем до минимума — вся галактика.
    for _ in 0..200 {
        map.zoom_by(-1);
    }
    assert_eq!(map.zoom(), ZOOM_MIN);
}

/// Планеты, чей центр вне области показа, не рисуются; на зуме 1 видна вся
/// галактика (крайние планеты прижаты внутрь кадра и не пропадают). При
/// сильном зуме в кадре остаются далеко не все 256 звёзд.
#[test]
fn planets_outside_view_are_culled() {
    let mem = fake_mem();
    let mut map = Map::new();
    map.open(&mem, galaxy::KLADOV_GALAXY);
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1280, 720);
    map.draw(&mut f, &font);
    let l = map.layout(1280, 720);
    // При зуме 1 все звёзды центром внутри поля — видны все 256.
    assert_eq!(visible_count(&map, &l), galaxy::COUNT);

    // Приближаем к окрестности LAVE — часть звёзд уходит за кадр.
    map.win_w = 1280;
    map.win_h = 720;
    for _ in 0..8 {
        map.zoom_by(1);
    }
    let l = map.layout(1280, 720);
    assert!(l.scale > 4.0);
    let visible = visible_count(&map, &l);
    assert!(
        0 < visible && visible < galaxy::COUNT / 4,
        "видно слишком много: {visible}"
    );
}

/// Число планет, чей центр попадает в видимую область карты.
fn visible_count(map: &Map, l: &MapLayout) -> usize {
    let (vx0, vy0, vx1, vy1) = l.visible_rect();
    map.planets
        .iter()
        .filter(|p| {
            let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
            sx >= vx0 && sx < vx1 && sy >= vy0 && sy < vy1
        })
        .count()
}

/// Крайние планеты прижимаются внутрь кадра (их кружок не выходит за
/// рамку), а планеты с центром вне области показа не рисуются.
#[test]
fn edge_planets_are_clamped_inside_the_field() {
    let map = opened();
    let l = map.layout(1024, 768);
    let (x0, y0, x1, y1) = l.visible_rect();
    let margin = map.planet_margin(l.scale);
    // Угол поля: центр звезды на самой кромке — кружок сдвинут внутрь.
    let (sx, sy) = map
        .place_planet(&l, 0, 0, margin)
        .expect("звезда на кромке видна");
    assert_eq!((sx, sy), (x0 + margin, y0 + margin));
    let r = planet_px(&map.planets[0], l.scale);
    assert!(
        sx - r >= x0 && sx + r < x1 && sy - r >= y0 && sy + r < y1,
        "кружок вышел за рамку"
    );
    // Центр далеко вне области показа — планета не рисуется.
    assert!(map.place_planet(&l, 400, 400, margin).is_none());
}

/// Карта рисуется во весь слой, планеты раскрашены, есть кольца и курсор.
#[test]
fn draw_fills_layer_and_marks_planets() {
    let mut map = opened();
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1024, 768);
    map.draw(&mut f, &font);
    assert!(f.pixels().iter().all(|&p| p != 0), "layer not opaque");
    let distinct: std::collections::HashSet<u32> = f.pixels().iter().copied().collect();
    assert!(
        distinct.len() > 50,
        "слишком мало цветов: {}",
        distinct.len()
    );
    assert!(f.pixels().iter().any(|&p| p == SYS_RING));
    assert!(f.pixels().iter().any(|&p| p == NEAR_RING));
    assert!(f.pixels().iter().any(|&p| p == CURSOR));
    assert!(
        f.pixels().iter().any(|&p| p == RANGE),
        "нет круга дальности"
    );
}

/// Точные позиции звёзд в HD-слое: звезда попадает в пиксель `star_px`,
/// окрашенный в свой постоянный цвет.
#[test]
fn star_positions_are_scaled_field_coordinates() {
    let mut map = opened();
    // Уводим курсор из системы, чтобы перекрестие не перекрывало LAVE.
    map.move_by(-255, -255);
    let font = vec![0u8; 0x300];
    let mut f = Frame::new(1024, 768);
    map.draw(&mut f, &font);
    let l = map.layout(1024, 768);
    let at = |f: &Frame, x: i32, y: i32| f.pixels()[y as usize * f.width + x as usize];
    let p7 = &map.planets[7];
    let (lx, ly) = l.star_px(p7.x, p7.y() - FIELD_TOP);
    assert_eq!(at(&f, lx, ly), galaxy::color(p7.state));
    let p204 = &map.planets[204];
    let (ox, oy) = l.star_px(p204.x, p204.y() - FIELD_TOP);
    let c = at(&f, ox, oy);
    assert!(
        c == galaxy::color(p204.state)
            || c == NEAR_RING
            || c == SYS_RING
            || c == RANGE
            || c == CURSOR,
        "ORESQU pixel = {c:#010X}"
    );
}

/// Отладочный дамп карты в `tmp/hdmap.ppm` по слепку памяти реальной игры:
/// `cargo test --release dump_map -- --ignored`. Затем, например,
/// `convert tmp/hdmap.ppm tmp/hdmap.png`. В `tmp/wf/mem.bin` уже есть
/// ПЗУ (0x0..0x4000), откуда берём шрифт. Переменная `ELITE_MAP_ZOOM`
/// задаёт зум для дампа.
#[test]
#[ignore]
fn dump_map_to_tmp() {
    let mem = std::fs::read("tmp/wf/mem.bin").expect("tmp/wf/mem.bin");
    let mut map = Map::new();
    map.open(&mem, galaxy::KLADOV_GALAXY);
    map.win_w = 1280;
    map.win_h = 720;
    if let Ok(z) = std::env::var("ELITE_MAP_ZOOM") {
        if let Ok(z) = z.parse::<f32>() {
            let zmax = map.max_zoom();
            while map.zoom() < z && map.zoom() < zmax {
                map.zoom_by(1);
            }
        }
    }
    let font = &mem[0x3D00..0x3D00 + 0x300];
    let mut f = Frame::new(1280, 720);
    map.draw(&mut f, font);
    let mut out = Vec::with_capacity(f.width * f.height * 3 + 32);
    out.extend_from_slice(format!("P6\n{} {}\n255\n", f.width, f.height).as_bytes());
    for &p in f.pixels() {
        out.push((p >> 16) as u8);
        out.push((p >> 8) as u8);
        out.push(p as u8);
    }
    std::fs::write("tmp/hdmap.ppm", out).expect("write tmp/hdmap.ppm");
}
