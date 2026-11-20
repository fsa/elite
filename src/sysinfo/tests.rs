//! Тесты экрана информации о планете.

use super::*;

/// Память с данными планеты (адреса версии Кладова).
fn fake_mem() -> Vec<u8> {
    let mut m = vec![0u8; 0x10000];
    m[ADDR_FUEL_TO_PLANET] = 5;
    m[ADDR_PLANET_ECONOMY] = 0;
    m[ADDR_PLANET_GOVERNMENT] = 6;
    m[ADDR_PLANET_TECH_LEVEL] = 4;
    m[ADDR_PLANET_POPULATION] = 0x19;
    m[ADDR_PLANET_GROSS_PRODUCT..ADDR_PLANET_GROSS_PRODUCT + 2]
        .copy_from_slice(&750u16.to_le_bytes());
    m[ADDR_PLANET_DATA1..ADDR_PLANET_DATA1 + 2].copy_from_slice(&0x149Cu16.to_le_bytes());
    m[ADDR_PLANET_DATA2..ADDR_PLANET_DATA2 + 2].copy_from_slice(&0x151Du16.to_le_bytes());
    m
}

#[test]
fn reads_planet_info_from_prepared_memory() {
    let info = read_info(&fake_mem(), "Lave");
    assert_eq!(info.name, "Lave");
    assert_eq!(info.distance_tenths, 5);
    assert_eq!(info.economy, "Rich industrial");
    assert_eq!(info.government, "Democracy");
    assert_eq!(info.tech_level, 5);
    assert_eq!(info.population_tenths, 0x19);
    assert_eq!(info.species, "Human colonials");
    assert_eq!(info.gross_product, 750);
    assert_eq!(info.radius_km, 4116);
}

#[test]
fn economy_and_government_mapping() {
    assert_eq!(economy_name(0), "Rich industrial");
    assert_eq!(economy_name(1), "Average industrial");
    assert_eq!(economy_name(2), "Poor industrial");
    assert_eq!(economy_name(3), "Mainly industrial");
    assert_eq!(economy_name(4), "Mainly agricultural");
    assert_eq!(economy_name(5), "Rich agricultural");
    assert_eq!(economy_name(6), "Average agricultural");
    assert_eq!(economy_name(7), "Poor agricultural");

    assert_eq!(government_name(0), "Anarchy");
    assert_eq!(government_name(1), "Feudal");
    assert_eq!(government_name(2), "Multi-government");
    assert_eq!(government_name(3), "Dictatorship");
    assert_eq!(government_name(4), "Communist");
    assert_eq!(government_name(5), "Confederacy");
    assert_eq!(government_name(6), "Democracy");
    assert_eq!(government_name(7), "Corporate state");
}

#[test]
fn humanoid_species_when_bit_set() {
    let mut m = fake_mem();
    m[ADDR_PLANET_DATA2] |= 0x80;
    let info = read_info(&m, "Lave");
    assert_eq!(info.species, "Humanoids");
}

#[test]
fn formatting_helpers() {
    assert_eq!(format_distance(0), "");
    assert_eq!(format_distance(5), "0.5");
    assert_eq!(format_distance(73), "7.3");
    assert_eq!(format_population(25), "2.5");
}

#[test]
fn open_close_draw_smoke() {
    let mem = vec![0u8; 0x10000];
    let mut s = SysInfo::new();
    assert!(!s.active);
    s.open(&mem, galaxy::KLADOV_GALAXY);
    assert!(s.active);
    let mut hd = Frame::new(640, 480);
    s.draw(&mut hd, crate::font::ZX_FONT);
    assert!(hd.pixels().iter().any(|&p| p != 0));
    s.close();
    assert!(!s.active);
}
