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
    assert_eq!(status.info.system, "LAVE");
    assert_eq!(status.info.galaxy, 3);
    assert_eq!(status.info.cash, 12345);
    assert_eq!(status.cash_text(), "1234.5 CR");
    assert_eq!(status.info.fuel, 70);
    assert_eq!(status.fuel_text(), "7.0 LY");
    assert_eq!(status.legal_text(), "CLEAN");
    assert_eq!(status.rating_text(), "COMPETENT");
}

/// Пороги правового статуса — как в `Cmd_L_PilotInfo`: `0` — чист,
/// `< 0x32` — нарушитель, дальше — беглец.
#[test]
fn legal_status_names_follow_the_game() {
    let mut status = opened();
    for (value, name) in [
        (0, "CLEAN"),
        (1, "OFFENDER"),
        (0x31, "OFFENDER"),
        (0x32, "FUGITIVE"),
        (0xFF, "FUGITIVE"),
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
        (0, "HARMLESS"),
        (7, "HARMLESS"),
        (8, "MOSTLY HARMLESS"),
        (15, "MOSTLY HARMLESS"),
        (16, "POOR"),
        (32, "AVERAGE"),
        (64, "ABOVE AVERAGE"),
        (128, "COMPETENT"),
        (511, "COMPETENT"),
        (512, "DANGEROUS"),
        (2559, "DANGEROUS"),
        (2560, "DEADLY"),
        (6399, "DEADLY"),
        (6400, "ELITE"),
        (0xFFFF_FF, "ELITE"),
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
        // Строки не заходят на нижнюю панель, панель — в рамку.
        assert!(
            l.panel_y >= l.content_y,
            "{w}x{h}: панель налезает на заголовок"
        );
        assert!(
            l.panel_y + l.frame + l.margin <= h as i32,
            "{w}x{h}: панель выходит за рамку"
        );
    }
}

/// Экран залит непрозрачным фоном, рамка белая, шапка красная, текст есть.
/// `ELITE_DUMP_STATUS=<file>` сохраняет кадр в PPM для визуальной проверки.
#[test]
fn draw_fills_layer_and_draws_header() {
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
    // Внутри шапки — красный фон.
    assert_eq!(
        f.pixels()[(l.header_y + l.header_h / 2) as usize * 1024
            + (l.margin + l.frame + 2) as usize],
        crate::cockpit::BAR_RED,
        "шапка не красная"
    );
    // Белый текст заголовка и строк даёт заметное число пикселей.
    let white = f
        .pixels()
        .iter()
        .filter(|&&p| p == crate::cockpit::FRAME_COLOR)
        .count();
    assert!(white > 500, "текста слишком мало: white={white}");
    if let Ok(path) = std::env::var("ELITE_DUMP_STATUS") {
        crate::headless::write_ppm(&path, f.pixels(), 1024, 768);
    }
}

/// Закрытый экран не мешает игре: `close` только снимает флаг.
#[test]
fn close_deactivates_screen() {
    let mut status = opened();
    status.close();
    assert!(!status.active);
}
