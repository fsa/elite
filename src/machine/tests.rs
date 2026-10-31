//! Тесты Spectrum: звук бипера и перехват игровых сообщений.

use super::*;

#[test]
fn beeper_edges_become_audio() {
    let mut m = Spectrum::new(None);
    // DI ; loop: ld a,0x10 ; out (0xfe),a ; ld a,0 ; out (0xfe),a ; jr loop
    let prog = [
        0xF3, 0x3E, 0x10, 0xD3, 0xFE, 0x3E, 0x00, 0xD3, 0xFE, 0x18, 0xF6,
    ];
    m.load(0x8000, &prog);
    m.cpu.pc = 0x8000;
    m.run_frame();
    let mut out = Vec::new();
    m.render_audio(&mut out);
    assert_eq!(out.len(), m.audio_rate as usize / 50);
    assert!(out.iter().any(|&s| s > 0), "no beeper-high samples");
    assert!(out.iter().any(|&s| s < 0), "no beeper-low samples");
}

use super::*;

#[test]
fn tap_assembles_line_by_position() {
    let mut t = GameMsgTap::default();
    t.enabled = true;
    // Строка печатается посимвольно: колонка следует за хвостом.
    t.push_char(b'F', 1, 0);
    t.push_char(b'R', 1, 1);
    t.push_char(b'O', 1, 2);
    t.push_char(b'N', 1, 3);
    t.push_char(b'T', 1, 4);
    assert_eq!(t.text, "FRONT");
    assert_eq!(t.row, 1);
    assert_eq!(t.col, 0);
    assert_eq!(t.frames, 0);

    // Печать с той же строки, но левее хвоста — новая строка.
    t.push_char(b'V', 1, 6);
    assert_eq!(t.text, "V");

    // Смена строки тоже начинает новую строку.
    t.push_char(b'H', 3, 2);
    t.push_char(b'I', 3, 3);
    assert_eq!(t.text, "HI");
    assert_eq!(t.row, 3);
    assert_eq!(t.col, 2);
}

#[test]
fn tap_ignores_control_codes_and_counts_age() {
    let mut t = GameMsgTap::default();
    t.enabled = true;
    t.push_char(0x0D, 4, 0); // перевод строки не печатается
    t.push_char(0x7F, 4, 0); // не-ASCII тоже
    assert!(t.text.is_empty());

    t.push_char(b'A', 4, 0);
    assert_eq!(t.text, "A");
    assert_eq!(t.frames, 0);

    t.next_frame();
    t.next_frame();
    assert_eq!(
        t.frames, 2,
        "возраст растёт только при включённом перехвате"
    );
    t.enabled = false;
    t.next_frame();
    assert_eq!(t.frames, 2, "выключенный перехват не старит строку");
}

#[test]
fn tap_clear_resets_line() {
    let mut t = GameMsgTap::default();
    t.enabled = true;
    t.push_char(b'X', 2, 0);
    t.clear();
    assert!(t.text.is_empty());
    assert_eq!(t.frames, 0);
}
