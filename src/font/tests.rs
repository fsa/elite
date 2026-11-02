//! Тесты встроенного знакогенератора.

use std::fs;

use super::*;

/// Встроенные байты совпадают со знакогенератором из ПЗУ 48K.
#[test]
fn embedded_font_matches_rom() {
    let base = env!("CARGO_MANIFEST_DIR");
    let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
    let expected = &rom[ZX_FONT_ROM_ADDR..ZX_FONT_ROM_ADDR + ZX_FONT_LEN];
    assert_eq!(ZX_FONT.as_slice(), expected);
}

/// Таблица покрывает печатные символы ASCII 0x20..0x7F.
#[test]
fn font_covers_printable_ascii() {
    assert_eq!(ZX_FONT.len(), 96 * GLYPH_H);
    assert_eq!(FIRST_CHAR, 0x20);
    assert_eq!(GLYPH_W * GLYPH_H, 8 * 8);
    // 'A' — непустой глиф.
    let a = (b'A' - FIRST_CHAR) as usize * GLYPH_H;
    assert!(ZX_FONT[a..a + GLYPH_H].iter().any(|&row| row != 0));
}
