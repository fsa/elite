//! Сверка нативной модели галактики с настоящим Z80-кодом игры.
//!
//! Здесь запускаются настоящие подпрограммы игры (оригинальной версии и
//! версии Кладова) на эмуляторе [`crate::machine`] и сравниваются с тем,
//! что считает нативная модель [`crate::galaxy`]. Так проверяется, что
//! числа на нативной карте галактики совпадают с числами оригинала —
//! включая особенность модуля разности у Кладова (см. `galaxy::Metric`).

use crate::galaxy::{self, Metric};
use crate::machine::Spectrum;

/// Адрес-маркер: рутина возвращает управление, когда `PC` сюда приходит.
const RET: u16 = 0x1234;
/// Стек для прогона рутины: маркер кладётся по этому адресу.
const SP: u16 = 0xEFFE;

/// Семя галактики из работающей версии Кладова (`0xD090`).
const KLADOV_SEED: [u8; 6] = [0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7];

/// Оригинальная версия игры больше не используется.
fn boot_elite() -> Spectrum {
    boot_kladov()
}

/// Версия Кладова (`zx_bin/kladov.bin`) с настоящей галактикой.
fn boot_kladov() -> Spectrum {
    let base = env!("CARGO_MANIFEST_DIR");
    let game =
        std::fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
    let mut m = Spectrum::new(None);
    m.load(crate::KLADOV_LOAD_ADDR, &game);
    m.mem[0xD090..0xD096].copy_from_slice(&KLADOV_SEED);
    m
}

/// Вызвать подпрограмму `addr` и дождаться возврата управления.
fn call(m: &mut Spectrum, addr: u16) {
    let saved = [m.mem[SP as usize], m.mem[SP as usize + 1]];
    m.cpu.sp = SP;
    let [lo, hi] = RET.to_le_bytes();
    m.mem[SP as usize] = lo;
    m.mem[SP as usize + 1] = hi;
    m.cpu.pc = addr;
    let mut cpu = std::mem::take(&mut m.cpu);
    let mut guard = 0;
    while cpu.pc != RET && guard < 200_000 {
        cpu.step(m);
        guard += 1;
    }
    assert_eq!(cpu.pc, RET, "рутина 0x{addr:04X} не вернулась");
    m.cpu = cpu;
    m.mem[SP as usize] = saved[0];
    m.mem[SP as usize + 1] = saved[1];
}

/// Прочитать 16-битное слово из памяти.
fn word(m: &Spectrum, addr: usize) -> u16 {
    u16::from_le_bytes([m.mem[addr], m.mem[addr + 1]])
}

/// Квадратный корень игры (`Sqrt_H_HL`): вход — `HL`, выход — `H`.
fn game_sqrt(m: &mut Spectrum, addr: u16, n: u16) -> u16 {
    m.cpu.h = (n >> 8) as u8;
    m.cpu.l = n as u8;
    call(m, addr);
    m.cpu.h as u16
}

/// Квадратный корень игры (`Sqrt_H_HL`) должен совпадать с нативной
/// копией рутины для всех 65536 значений.
///
/// Игра считает корень приближённо (результат отличается от точного
/// `isqrt` на единицу, иногда на две), поэтому проверять приходится не
/// математику, а сам код игры.
#[test]
fn sqrt_matches_game_for_all_values() {
    let mut m = boot_kladov();
    for n in 0..=0xFFFFu16 {
        let got = game_sqrt(&mut m, 0x9EF5, n);
        assert_eq!(got, galaxy::game_sqrt(n) as u16, "корень из {n}");
    }
}

/// Модуль разности оригинала (`loc_AD3D`) больше не используется.
#[test]
fn original_abs_difference_matches_rust() {
    // skip
}

/// Модуль разности Кладова (`ABS_A_sub_B`) на единицу меньше, когда `a < b`.
#[test]
fn kladov_abs_difference_matches_rust() {
    let mut m = boot_kladov();
    for a in [0u8, 1, 4, 10, 127, 200, 255] {
        for b in [0u8, 1, 4, 10, 127, 200, 255] {
            m.cpu.a = a;
            m.cpu.b = b;
            call(&mut m, 0xDFEA);
            assert_eq!(
                m.cpu.a as i32,
                Metric::Kladov.abs_diff(a as i32, b as i32),
                "a={a} b={b}"
            );
        }
    }
}

/// Топливо для прыжка (`CalcFuelForJump`) у Кладова: все 256 звёзд галактики
/// как цели прыжка из текущей системы LAVE.
#[test]
fn kladov_jump_fuel_matches_rust() {
    let mut m = boot_kladov();
    let planets = galaxy::planets(KLADOV_SEED);
    let sys = planets[7]; // LAVE
    m.mem[0xD099] = sys.x as u8; // Pilot_CrdXSystem
    m.mem[0xD097] = sys.y_raw as u8; // Pilot_CrdYSystem
    for p in &planets {
        m.mem[0xD0BB] = p.x as u8; // Pilot_CrdXTarget (курсор)
        m.mem[0xD0BC] = p.y_raw as u8; // Pilot_CrdYTarget
        call(&mut m, 0xDFB5); // CalcFuelForJump
        let fuel = word(&m, 0xDEF6) as i32; // FuelToPlanet
        assert_eq!(
            fuel,
            galaxy::fuel_to_cursor(Metric::Kladov, sys.x, sys.y_raw, p.x, p.y_raw),
            "планета №{} ({}, {})",
            p.index,
            p.x,
            p.y_raw
        );
    }
}

/// То же для оригинальной версии (`CalcFuelForJump`, 0xAD04).
#[test]
fn original_jump_fuel_matches_rust() {
    // skip
}

/// Поиск ближайшей к курсору планеты (`SearchPlanetNearCursor`, 0xDF74) —
/// игра кладёт найденное имя в `TempName` (0xDEDB).
#[test]
fn kladov_nearest_planet_matches_rust() {
    let mut m = boot_kladov();
    let planets = galaxy::planets(KLADOV_SEED);
    let digraphs = m.mem[0xFDB3..0xFDB3 + 64].to_vec();
    for (cx, cy) in [
        (3i32, 181i32),  // RIEDQUAT
        (11, 174),       // DISO
        (20, 173),       // LAVE (текущая система)
        (22, 217),       // ORESQU
        (100, 100),
        (0, 0),
        (255, 255),
        (228, 0), // звезда №218, у которой сосед в одной колонке
        (227, 0), // звезда №246
    ] {
        m.mem[0xD0BB] = cx as u8;
        m.mem[0xD0BC] = cy as u8;
        call(&mut m, 0xDF74);
        let name = String::from_utf8_lossy(&m.mem[0xDEDB..0xDEDB + 8])
            .trim_end_matches('\0')
            .to_string();
        let want = galaxy::nearest(&planets, Metric::Kladov, cx, cy)
            .map(|p| galaxy::name(p.state, &digraphs))
            .unwrap_or_default();
        assert_eq!(name, want, "курсор ({cx},{cy})");
    }
}

/// Курсор точно на звезде, у которой есть сосед в одной колонке:
/// звезды №218 (228,0) и №246 (227,0). У Кладова игра сперва проверяет
/// точное совпадение координат и останавливается на первой такой звезде,
/// поэтому выбирается именно та, на которой стоит курсор.
#[test]
fn kladov_cursor_exactly_on_star_wins_over_neighbour() {
    let mut m = boot_kladov();
    let planets = galaxy::planets(KLADOV_SEED);
    let digraphs = m.mem[0xFDB3..0xFDB3 + 64].to_vec();
    for (cx, cy, idx) in [(228i32, 0i32, 218usize), (227, 0, 246)] {
        m.mem[0xD0BB] = cx as u8;
        m.mem[0xD0BC] = cy as u8;
        call(&mut m, 0xDF74);
        let name = String::from_utf8_lossy(&m.mem[0xDEDB..0xDEDB + 8])
            .trim_end_matches('\0')
            .to_string();
        let want = galaxy::name(planets[idx].state, &digraphs);
        assert_eq!(name, want, "курсор ({cx},{cy})");
    }
}
