//! Минимальная заглушка ROM для запуска без образа ПЗУ.

use super::*;

/// Minimal substitute ROM implementing only the entry points Elite needs.
/// Used when a real ROM image is not supplied. Addresses match the standard
/// ZX Spectrum 48K ROM layout.
pub fn install_stub_rom(rom: &mut [u8]) {
    // Reset vector: DI / JR to itself is not desired; start of ROM just spins.
    rom[0x0000] = 0xF3; // DI
    rom[0x0001] = 0x76; // HALT
    rom[0x0002] = 0x18; // JR
    rom[0x0003] = 0xFD; // -3

    // Interrupt handler at 0x0038: EI / RET
    rom[0x0038] = 0xFB; // EI
    rom[0x0039] = 0xC9; // RET

    // BEEPER at 0x03B5: RET (silent)
    rom[0x03B5] = 0xC9;

    // PIXEL_ADD exact bytes at 0x22AA (contract: B=y, C=x -> HL=addr, A=x&7).
    const PIXEL_ADD: [u8; 33] = [
        0x3E, 0xAF, 0x90, 0xDA, 0xF9, 0x24, 0x47, 0xA7, 0x1F, 0x37, 0x1F, 0xA7, 0x1F, 0xA8, 0xE6,
        0xF8, 0xA8, 0x67, 0x79, 0x07, 0x07, 0x07, 0xA8, 0xE6, 0xC7, 0xA8, 0x07, 0x07, 0x6F, 0x79,
        0xE6, 0x07, 0xC9,
    ];
    rom[0x22AA..0x22AA + PIXEL_ADD.len()].copy_from_slice(&PIXEL_ADD);

    // Target of the "out of range" branch: just RET.
    rom[0x24F9] = 0xC9;

    // Tape load/save stubs (LD-BYTES 0x0562, SA-BYTES 0x04C6): RET.
    rom[0x0562] = 0xC9;
    rom[0x04C6] = 0xC9;
}
