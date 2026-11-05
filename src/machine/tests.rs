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

#[test]
fn run_frames_advances_double_and_keeps_audio_rate() {
    let mut m = Spectrum::new(None);
    let prog = [
        0xF3, 0x3E, 0x10, 0xD3, 0xFE, 0x3E, 0x00, 0xD3, 0xFE, 0x18, 0xF6,
    ];
    m.load(0x8000, &prog);
    m.cpu.pc = 0x8000;
    m.run_frames(2);
    assert_eq!(m.frame_start, 2 * CYCLES_PER_FRAME);
    assert_eq!(m.frames, 2);
    let mut out = Vec::new();
    m.render_audio(&mut out);
    // Ускорение не раздувает звуковой буфер: на кадр показа — один кадр
    // звука, просто сжатый вдвое по времени.
    assert_eq!(out.len(), m.audio_rate as usize / 50);
    assert!(out.iter().any(|&s| s > 0), "no beeper-high samples");
    assert!(out.iter().any(|&s| s < 0), "no beeper-low samples");
}

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
    assert!(t.lines.is_empty());
}

#[test]
fn tap_keeps_last_line_of_each_row() {
    let mut t = GameMsgTap::default();
    t.enabled = true;
    for (i, c) in b"HYPERSPACE".iter().enumerate() {
        t.push_char(*c, 13, 5 + i as u8);
    }
    for (i, c) in b"FRONT".iter().enumerate() {
        t.push_char(*c, 1, i as u8);
    }
    // «Последняя» строка — верхняя подпись, но строка прыжка осталась в истории.
    assert_eq!(t.text, "FRONT");
    let jump = t
        .lines
        .iter()
        .find(|l| l.row == 13)
        .expect("строка гиперпрыжка потерялась");
    assert_eq!(jump.text, "HYPERSPACE");
    assert_eq!(jump.col, 5);

    // Повторная печать в ту же строку начинает её заново, не трогая другие.
    t.push_char(b'X', 13, 0);
    assert_eq!(
        t.lines
            .iter()
            .find(|l| l.row == 13)
            .map(|l| l.text.as_str()),
        Some("X")
    );
    assert_eq!(
        t.lines.iter().find(|l| l.row == 1).map(|l| l.text.as_str()),
        Some("FRONT")
    );
}

/// Завершить кадр перехвата (в тестах файл не создаётся — только события).
fn finish_text_capture(t: &mut GameMsgTap) {
    t.end_text_frame();
}

#[test]
fn text_capture_builds_lines_for_debug_log() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    for (i, c) in b"Hyperspace -- Lave".iter().enumerate() {
        t.capture_text_char(*c, 13, 5 + i as u8);
    }
    finish_text_capture(&mut t);

    assert_eq!(
        t.text_events(),
        &[TextEvent {
            text: "Hyperspace -- Lave".to_string(),
            row: 13,
            col: 5,
        }]
    );
}

#[test]
fn text_capture_maps_quirks_and_skips_icons() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    t.capture_text_char(0x21, 13, 5); // служебный значок
    t.capture_text_char(b'A', 13, 6);
    t.capture_text_char(0x2A, 13, 7); // на экране `!`
    t.capture_text_char(0x40, 13, 8); // на экране `_`
    t.capture_text_char(0x26, 13, 9); // ещё значок
    t.capture_text_char(b'B', 13, 10);
    finish_text_capture(&mut t);

    assert_eq!(
        t.text_events(),
        &[TextEvent {
            text: "A!_B".to_string(),
            row: 13,
            col: 5,
        }]
    );
}

#[test]
fn text_capture_starts_new_line_on_gap() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    t.capture_text_char(b'A', 13, 5);
    t.capture_text_char(b'B', 13, 7); // разрыв позиции
    t.capture_text_char(b'C', 1, 2); // другая строка
    finish_text_capture(&mut t);

    assert_eq!(
        t.text_events(),
        &[
            TextEvent {
                text: "A".to_string(),
                row: 13,
                col: 5,
            },
            TextEvent {
                text: "B".to_string(),
                row: 13,
                col: 7,
            },
            TextEvent {
                text: "C".to_string(),
                row: 1,
                col: 2,
            },
        ]
    );
}

#[test]
fn text_capture_ignores_control_codes() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    t.capture_text_char(0x0D, 13, 5); // управляющий код
    t.capture_text_char(0x7F, 13, 5); // игра не печатает
    finish_text_capture(&mut t);
    assert!(t.text_events().is_empty());
}

#[test]
fn text_capture_is_off_until_started() {
    let mut t = GameMsgTap::default();
    t.capture_text_char(b'A', 1, 1);
    finish_text_capture(&mut t);
    assert!(t.text_events().is_empty());
}
