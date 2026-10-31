//! Тесты нативной модели галактики.

use super::*;

/// Семя галактики из запущенной версии Кладова (`tmp/wf/mem.bin`, `0xD090`).
const SEED: [u8; 6] = [0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7];

/// Таблица диграфов имён (`byte_FDB3`, 32 пары символов).
const DIGRAPHS: &[u8] = b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION";

#[test]
fn planets_match_known_galaxy() {
    let p = planets(SEED);
    assert_eq!(p.len(), COUNT);
    // Текущая система Джеймсона — LAVE, звезда №7: (x=20, y_raw=173).
    assert_eq!((p[7].x, p[7].y_raw), (20, 173));
    assert_eq!(p[7].state.to_vec(), b"\x38\xad\x9c\x14\x1d\x15".to_vec());
    // Система ORESQU — звезда №204: (x=22, y_raw=217).
    assert_eq!((p[204].x, p[204].y_raw), (22, 217));
    assert_eq!(p[204].state.to_vec(), b"\xba\xd9\x90\x16\xb3\xf9".to_vec());
}

#[test]
fn planet_names_match_game() {
    let p = planets(SEED);
    assert_eq!(name(p[7].state, DIGRAPHS), "LAVE");
    assert_eq!(name(p[204].state, DIGRAPHS), "ORESQU");
}

#[test]
fn planet_color_is_stable_and_varied() {
    let p = planets(SEED);
    // Постоянство: одинаковые данные дают одинаковый цвет.
    assert_eq!(color(p[7].state), color(p[7].state));
    assert_ne!(color(p[7].state), color(p[204].state));
    // Цвет непрозрачный и не чёрный.
    for pl in &p {
        let c = color(pl.state);
        assert_eq!(c >> 24, 0xFF, "планета {}", pl.index);
        assert_ne!(c & 0x00FF_FFFF, 0, "планета {}", pl.index);
    }
    // Разные звёзды — разные цвета.
    let distinct: std::collections::HashSet<u32> = p.iter().map(|pl| color(pl.state)).collect();
    assert!(
        distinct.len() > 200,
        "слишком мало цветов: {}",
        distinct.len()
    );
}

#[test]
fn radius_is_three_to_five() {
    let p = planets(SEED);
    for pl in &p {
        let r = pl.radius();
        assert!((3..=5).contains(&r), "radius {r} for planet {}", pl.index);
    }
}

#[test]
fn nearest_uses_squeezed_vertical_metric() {
    let p = planets(SEED);
    // Точка ровно на звезде №7 — она и ближайшая.
    assert_eq!(nearest(&p, Metric::Kladov, 20, 173).unwrap().index, 7);
    // Сдвиг по «сырой» вертикали на 4 — это 2 экранные строки.
    assert_eq!(distance2(Metric::Kladov, 20, 177, &p[7]), 4);
    // Горизонталь не делится: сдвиг на 4 даёт 16.
    assert_eq!(distance2(Metric::Kladov, 24, 173, &p[7]), 16);
}

/// Расстояние на карте галактики: LAVE (№7) -> DISO (№147). В версии
/// Кладова 3.2 св. года, в оригинале Elite — 3.6: модуль разности у
/// Кладова на единицу меньше, когда курсор левее системы.
#[test]
fn distance_in_light_years_matches_game() {
    let p = planets(SEED);
    assert_eq!((p[147].x, p[147].y_raw), (11, 174));
    assert_eq!(
        fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, p[147].x, p[147].y_raw),
        32
    );
    assert_eq!(
        distance_to_cursor_ly(Metric::Kladov, p[7].x, p[7].y_raw, p[147].x, p[147].y_raw),
        3.2
    );
    // До самой системы — ноль.
    assert_eq!(
        fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, 20, 173),
        0
    );
}

/// LAVE (№7) -> RIEDQUAD (№46): в версии Кладова 6.4 св. года, в
/// оригинале Elite — 6.8 (разница ровно в модуле разности).
#[test]
fn distance_to_riedquad_matches_game() {
    let p = planets(SEED);
    assert_eq!((p[46].x, p[46].y_raw), (3, 181));
    assert_eq!(
        fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, p[46].x, p[46].y_raw),
        64
    );
    assert_eq!(
        distance_to_cursor_ly(Metric::Kladov, p[7].x, p[7].y_raw, p[46].x, p[46].y_raw),
        6.4
    );
}

/// Радиус достижимости: топливо 70 -> 17 единиц поля (как в карте I).
#[test]
fn jump_radius_matches_game() {
    assert_eq!(jump_radius(70), 17);
    assert_eq!(jump_radius(7), 1);
    assert_eq!(jump_radius(0), 0);
}

/// Корень игры считается приближённо: `Sqrt_H_HL` возвращает не точный
/// `isqrt`, а число, отличающееся на единицу (иногда на две). Опорные
/// значения сняты с настоящего кода игры (см. `probe`).
#[test]
fn game_sqrt_matches_game_values() {
    assert_eq!(game_sqrt(0), 2);
    assert_eq!(game_sqrt(1), 2);
    assert_eq!(game_sqrt(64), 8);
    assert_eq!(game_sqrt(100), 10);
    assert_eq!(game_sqrt(256), 16);
    assert_eq!(game_sqrt(7690), 88);
    assert_eq!(game_sqrt(0xFFFF), 255);
    // Точное значение для сравнения: 7690 -> 87.69 -> 88.
    assert_eq!(game_sqrt(7690), 88);
}

/// Топливо для прыжка считается так же, как в игре: четыре корня.
#[test]
fn fuel_for_distance_matches_game() {
    assert_eq!(fuel_for_distance(0), 0);
    assert_eq!(fuel_for_distance(64), 32); // 3.2 св. года
    assert_eq!(fuel_for_distance(272), 64); // 6.4 св. года
    assert_eq!(fuel_for_distance(305), 68); // 6.8 св. года
    // Переполнение 16 бит: игра подставляет 0xFFFF -> 1020.
    assert_eq!(fuel_for_distance(0x10000), 1020);
}
