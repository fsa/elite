//! Тесты экрана статуса пилота.

use super::*;

/// Память с данными командира и галактикой (те же адреса, что у Кладова).
fn fake_mem() -> Vec<u8> {
    let mut m = vec![0u8; 0x10000];
    // Имя: «KRYTEN» и хвостовые пробелы, как в блоке `PilotName`.
    m[0xD054..0xD054 + 11].copy_from_slice(b"KRYTEN    \0");
    m[0xD05F] = 0; // PilotLegalStatus: чист
    m[0xD060..0xD063].copy_from_slice(&[0xC8, 0x00, 0x00]); // рейтинг 200
    m[0xD065] = 2; // номер галактики (0-based)
    m[0xD066..0xD068].copy_from_slice(&1u16.to_le_bytes()); // cash2
    m[0xD068..0xD06A].copy_from_slice(&2345u16.to_le_bytes()); // cash1
    m[0xD082] = 70; // топливо: 7.0 LY
    m[0xD090..0xD096].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
    m[0xD097] = 173; // Y системы (LAVE)
    m[0xD099] = 20; // X системы (LAVE)
    m[0xFDB3..0xFDB3 + 64]
        .copy_from_slice(b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION");
    // Цель гиперпрыжка (DISO) — координаты берём из той же галактики.
    let planets = galaxy::planets([0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
    let diso = planets
        .iter()
        .find(|p| galaxy::name(p.state, &m[0xFDB3..0xFDB3 + 64]) == "DISO")
        .expect("DISO в галактике");
    m[0xD0BB] = diso.x as u8;
    m[0xD0BC] = diso.y_raw as u8;
    m
}

fn opened() -> Status {
    let mem = fake_mem();
    let mut status = Status::new();
    status.open(&mem, KLADOV_STATUS);
    status
}

#[test]
fn open_reads_commander_fields() {
    let status = opened();
    assert!(status.active);
    assert_eq!(status.info.name, "KRYTEN");
    assert_eq!(status.info.system, "Lave");
    assert_eq!(status.info.target, "Diso");
    assert_eq!(status.info.cash, 12345);
    assert_eq!(status.cash_text(), "1234.5 Cr");
    assert_eq!(status.info.fuel, 70);
    assert_eq!(status.fuel_text(), "7.0 Light Years");
    assert_eq!(status.legal_text(), "Clean");
    assert_eq!(status.rating_text(), "Competent");
}

/// Список оборудования — в порядке `Cmd_L_PilotInfo`: шесть флагов,
/// лазеры по креплениям, компьютер и устройство невидимости.
#[test]
fn open_reads_equipment_in_original_order() {
    let mut mem = fake_mem();
    for addr in [0xD085, 0xD088, 0xD08A, 0xD08B, 0xD08C, 0xD08D] {
        mem[addr] = 0xFF;
    }
    mem[0xD07D] = 1; // FRONT: pulse
    mem[0xD07E] = 2; // REAR: beam
    mem[0xD07F] = 0; // LEFT: нет лазера
    mem[0xD080] = 4; // RIGHT: mining
    mem[0xD086] = 1; // alien computer
    mem[0xD0B0] = 0x40; // cloaking device
    let mut status = Status::new();
    status.open(&mem, KLADOV_STATUS);
    assert_eq!(
        status.info.equipment,
        [
            "E.C.M.System",
            "Fuel Scoops",
            "Energy Bomb",
            "Energy Unit",
            "Docking Computer",
            "Galactic Hyperspace",
            "Front Pulse Laser",
            "Rear Beam Laser",
            "Right Mining Laser",
            "Alien Computer",
            "Cloaking Device",
        ]
    );
}

/// Без оборудования список пуст, но заголовок на экране остаётся.
#[test]
fn no_equipment_gives_empty_list() {
    assert!(opened().info.equipment.is_empty());
}

/// Блок оборудования белый и начинается через пустую строку после рейтинга.
#[test]
fn equipment_block_is_white_after_blank_line() {
    let mut mem = fake_mem();
    mem[0xD085] = 0xFF;
    mem[0xD07D] = 1;
    let mut status = Status::new();
    status.open(&mem, KLADOV_STATUS);
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1024, 768);
    status.draw(&mut f, &font);

    let l = draw::layout(1024, 768);
    let x = l.margin + l.frame + 8 * l.text as i32;
    let row_h = l.line + l.line / 2;
    let blank_y = l.first_row_y + 6 * row_h;
    let heading_y = l.first_row_y + 7 * row_h;
    let item_y = l.first_row_y + 8 * row_h;
    let width = f.width as usize;
    let px = |x: i32, y: i32| f.pixels()[y as usize * width + x as usize];
    // Строка между рейтингом и заголовком пустая.
    assert_eq!(px(x, blank_y), 0xFF00_0000, "нет пустой строки");
    // Заголовок и список — белые, предметы с отступом в 5 символов.
    assert_eq!(px(x, heading_y), crate::cockpit::FRAME_COLOR, "заголовок");
    assert_eq!(
        px(x + 5 * 8 * l.text as i32, item_y),
        crate::cockpit::FRAME_COLOR,
        "предмет списка"
    );
}

/// Пороги правового статуса — как в `Cmd_L_PilotInfo`: `0` — чист,
/// `< 0x32` — нарушитель, дальше — беглец.
#[test]
fn legal_status_names_follow_the_game() {
    let mut status = opened();
    for (value, name) in [
        (0, "Clean"),
        (1, "Offender"),
        (0x31, "Offender"),
        (0x32, "Fugitive"),
        (0xFF, "Fugitive"),
    ] {
        status.info.legal = value;
        assert_eq!(status.legal_text(), name, "legal={value}");
    }
}

/// Пороги рейтинга — из таблиц `byte_F0AE`/`byte_F0B4`.
#[test]
fn rating_names_follow_the_game_tables() {
    let mut status = opened();
    for (rating, name) in [
        (0, "Harmless"),
        (7, "Harmless"),
        (8, "Mostly Harmless"),
        (15, "Mostly Harmless"),
        (16, "Poor"),
        (32, "Average"),
        (64, "Above Average"),
        (128, "Competent"),
        (511, "Competent"),
        (512, "Dangerous"),
        (2559, "Dangerous"),
        (2560, "Deadly"),
        (6399, "Deadly"),
        (6400, "Elite"),
        (0xFFFF_FF, "Elite"),
    ] {
        status.info.rating = rating;
        assert_eq!(status.rating_text(), name, "rating={rating}");
    }
}

/// Повреждённый блок командира не должен ломать экран: имя обрезается по
/// `NUL`, непечатаемые байты заменяются.
#[test]
fn broken_name_is_safe_to_show() {
    let mut mem = fake_mem();
    mem[0xD054..0xD054 + 11].copy_from_slice(b"AB\x01CD\0  \0  ");
    let mut status = Status::new();
    status.open(&mem, KLADOV_STATUS);
    assert_eq!(status.info.name, "AB?CD");
}

/// Рамка и её отступы совпадают с кокпитом и картой галактики.
#[test]
fn frame_matches_cockpit_geometry() {
    for &(w, h) in &[
        (640u32, 480u32),
        (1024, 768),
        (1280, 720),
        (1920, 1080),
        (3840, 2160),
    ] {
        let l = draw::layout(w, h);
        let g = crate::cockpit::panel_geometry(w, h);
        let main = g.main_frame();
        assert_eq!(l.frame, crate::cockpit::LINE, "{w}x{h}: толщина линии");
        assert_eq!(l.margin, g.margin, "{w}x{h}: отступ от края");
        assert_eq!(l.margin, main.x, "{w}x{h}: левая рамка кокпита");
        assert_eq!(l.margin, main.y, "{w}x{h}: верхняя рамка кокпита");
        assert_eq!(main.w, w as i32 - 2 * l.margin, "{w}x{h}: ширина рамки");
        // Шесть полей, пустая строка, заголовок и 12 строк оборудования
        // не заходят за нижнюю рамку.
        let row_h = l.line + l.line / 2;
        assert!(
            l.first_row_y + 19 * row_h + l.line < h as i32 - l.margin - l.frame,
            "{w}x{h}: содержимое выходит за рамку"
        );
    }
}

/// Экран залит непрозрачным фоном, рамка белая, заголовок по центру.
/// `ELITE_DUMP_STATUS=<file>` сохраняет кадр в PPM для визуальной проверки.
#[test]
fn draw_fills_layer_and_centers_title() {
    let status = opened();
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1024, 768);
    status.draw(&mut f, &font);
    assert!(f.pixels().iter().all(|&p| p != 0), "layer not opaque");

    let l = draw::layout(1024, 768);
    // Угол внешней рамки — белый.
    assert_eq!(
        f.pixels()[(l.margin as usize) * 1024 + l.margin as usize],
        crate::cockpit::FRAME_COLOR,
        "рамка не нарисована"
    );
    // Заголовок — голубой, по центру.
    let title = "COMMANDER KRYTEN";
    let inner_x = l.margin + l.frame;
    let inner_w = 1024 - 2 * inner_x;
    let tw = title.len() as i32 * 8 * l.text as i32;
    let tx = inner_x + (inner_w - tw) / 2;
    assert_eq!(
        f.pixels()[l.title_y as usize * 1024 + tx as usize],
        0xFF00_D7D7,
        "заголовок не по центру или не голубой"
    );
    // Красной шапки у экрана больше нет.
    assert!(
        !f.pixels().iter().any(|&p| p == crate::cockpit::BAR_RED),
        "лишняя красная шапка"
    );
    // Текст даёт заметное число пикселей.
    let lit = f.pixels().iter().filter(|&&p| p != 0xFF00_0000).count();
    assert!(lit > 500, "текста слишком мало: {lit}");
    if let Ok(path) = std::env::var("ELITE_DUMP_STATUS") {
        let mut g = Frame::new(1024, 768);
        status.draw(&mut g, crate::font::ZX_FONT);
        crate::headless::write_ppm(&path, g.pixels(), 1024, 768);
    }
}

/// Закрытый экран не мешает игре: `close` только снимает флаг.
#[test]
fn close_deactivates_screen() {
    let mut status = opened();
    status.close();
    assert!(!status.active);
}
