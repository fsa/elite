//! Тесты Spectrum: звук бипера, пейджинг, перехват игровых сообщений.

use super::*;

#[test]
fn paging_switches_upper_bank_and_keeps_writes() {
    let mut m = Spectrum::new(None);
    // В каждом банке — его номер, чтобы сразу видеть, кто отображён.
    let image: Vec<u8> = (0..8u8).flat_map(|b| vec![b; 0x4000]).collect();
    m.load_game_image(&image, 0x10);
    assert_eq!(m.paged_bank, 0);
    assert_eq!(m.mem[0xC000], 0);

    // Запись в отображённую страницу попадает в её банк.
    m.write(0xC000, 0xAA);
    assert_eq!(m.mem[0xC000], 0xAA);

    // Порт 0x7FFD переключает только окно 0xC000; уходящая страница
    // сохраняется в своём банке.
    m.output(0x7FFD, 0x11);
    assert_eq!(m.ram_banks[0][0], 0xAA);
    assert_eq!(m.paged_bank, 1);
    assert_eq!(m.mem[0xC000], 1);
    m.write(0xC000, 0xBB);
    assert_eq!(m.mem[0xC000], 0xBB);

    // Возврат к банку 0 сохраняет сделанную туда запись.
    m.output(0x7FFD, 0x10);
    assert_eq!(m.mem[0xC000], 0xAA);
    assert_eq!(m.ram_banks[1][0], 0xBB);
    assert_eq!(m.paging_writes, 2);
}

#[test]
fn paging_lock_ignores_further_writes() {
    let mut m = Spectrum::new(None);
    let image = vec![0u8; 8 * 0x4000];
    m.load_game_image(&image, 0x10);
    // Бит 5 блокирует порт до следующей перезагрузки.
    m.output(0x7FFD, 0x30);
    assert_eq!(m.paged_bank, 0);
    m.output(0x7FFD, 0x11);
    assert_eq!(m.paged_bank, 0);
    assert_eq!(m.paging_writes, 2); // записи считаются даже при блокировке
    assert_eq!(m.paging_last, 0x11);
}

#[test]
fn hook_bound_to_upper_bank_does_not_fire_in_other_bank() {
    use std::sync::atomic::{AtomicU32, Ordering};
    static CALLS: AtomicU32 = AtomicU32::new(0);
    fn count(_: &mut Z80, _: &mut Spectrum) -> u64 {
        CALLS.fetch_add(1, Ordering::Relaxed);
        0
    }
    fn run_to_halt(m: &mut Spectrum) {
        m.mem[0xC000] = 0xC9; // ret
        m.mem[0xE000] = 0x34;
        m.mem[0xE001] = 0x12; // адрес возврата 0x1234
        m.mem[0x1234] = 0x76; // halt
        m.cpu.sp = 0xE000;
        m.cpu.pc = 0xC000;
        m.cpu.iff1 = false;
        m.run_frame();
    }

    CALLS.store(0, Ordering::Relaxed);
    let mut m = Spectrum::new(None);
    let image = vec![0u8; 8 * 0x4000];
    m.load_game_image(&image, 0x10);
    m.install_hook(0xC000, count, 0);

    run_to_halt(&mut m);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1, "хук банка 0 не сработал");

    m.output(0x7FFD, 0x11); // банк 1 по тому же адресу 0xC000
    run_to_halt(&mut m);
    assert_eq!(
        CALLS.load(Ordering::Relaxed),
        1,
        "хук банка 0 сработал в банке 1"
    );
}

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
    // Бипер — прямоугольный сигнал по тактам Z80: фронты могут быть полными.
    let max_step = out
        .windows(2)
        .map(|w| (w[1] as i32 - w[0] as i32).abs())
        .max()
        .unwrap_or(0);
    assert!(max_step <= 8000, "неожиданный шаг амплитуды: {max_step}");
}

#[test]
fn game_state_is_snapshot_taken_at_pass_end() {
    let mut m = Spectrum::new(None);
    m.game_marker = Some(0x7471);
    m.snapshot_marker = Some(0x81A9);
    m.mem[0x7000] = 0x11;
    assert_eq!(m.game_state()[0x7000], 0x11, "до прохода нужна живая память");

    m.note_pc(0x81A9);
    assert!(m.game_snapshot_valid);
    m.mem[0x7000] = 0x22;
    assert_eq!(m.game_state()[0x7000], 0x11, "снимок должен хранить конец прохода");

    m.note_pc(0x81A9);
    assert_eq!(m.game_state()[0x7000], 0x22, "новый проход обновляет снимок");

    m.intro_range = Some((0x739E, 0x73FD));
    m.note_pc(0x73A0);
    assert!(!m.game_snapshot_valid, "на заставке снимок недействителен");
    assert_eq!(m.game_state()[0x7000], 0x22);
}

#[test]
fn game_state_is_snapshot_taken_at_transition_marker() {
    let mut m = Spectrum::new(None);
    m.transition_snapshot_marker = Some(0x92B3);
    m.mem[0x7000] = 0x11;
    assert_eq!(m.game_state()[0x7000], 0x11, "до ролика нужна живая память");

    m.note_pc(0x92B3);
    assert!(m.game_snapshot_valid);
    m.mem[0x7000] = 0x22;
    assert_eq!(
        m.game_state()[0x7000], 0x11,
        "снимок начала ролика должен хранить состояние перед тоннелем"
    );
}

#[test]
fn invalidated_game_snapshot_returns_live_memory() {
    let mut m = Spectrum::new(None);
    m.snapshot_marker = Some(0x81A9);
    m.mem[0x7000] = 0x11;
    m.note_pc(0x81A9);
    m.mem[0x7000] = 0x22;
    assert_eq!(m.game_state()[0x7000], 0x11, "снимок ещё не сброшен");

    m.invalidate_game_snapshot();
    assert!(!m.game_snapshot_valid);
    assert_eq!(m.game_state()[0x7000], 0x22, "после сброса нужна живая память");

    m.note_pc(0x81A9);
    assert_eq!(m.game_state()[0x7000], 0x22, "следующий проход снимает снимок");
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
    // run_frames(2) даёт 2 кадра Z80 — звук должен покрывать оба кадра,
    // чтобы не терять события при ускорении.
    assert_eq!(out.len(), 2 * m.audio_rate as usize / 50);
    assert!(out.iter().any(|&s| s > 0), "no beeper-high samples");
    assert!(out.iter().any(|&s| s < 0), "no beeper-low samples");
}

/// Завершить кадр перехвата (собираются только события).
fn finish_text_capture(t: &mut GameMsgTap) {
    t.end_text_frame();
}

#[test]
fn text_capture_builds_line() {
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
            // Значок занимает колонку, но не превращается в текст: на его
            // месте остаётся пробел, поэтому слова не слипаются.
            text: "A!_ B".to_string(),
            row: 13,
            col: 6,
        }]
    );
}

#[test]
fn text_capture_keeps_gaps_in_one_line() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    t.capture_text_char(b'A', BOTTOM_MESSAGE_ROW, 5);
    t.capture_text_char(b'B', BOTTOM_MESSAGE_ROW, 7); // разрыв позиции
    t.capture_text_char(b'C', BOTTOM_MESSAGE_ROW, 9); // ещё разрыв
    finish_text_capture(&mut t);

    // Разрывы не делят строку на события: пустые колонки остаются внутри.
    assert_eq!(
        t.text_events(),
        &[TextEvent {
            text: "A B C".to_string(),
            row: BOTTOM_MESSAGE_ROW,
            col: 5,
        }]
    );
}

#[test]
fn text_capture_keeps_parts_from_different_frames() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    let tag = MessageTag {
        number: 0xC0,
        life: 14,
    };
    for (i, c) in b"HYPERSPACE -- ".iter().enumerate() {
        t.capture_text_char_tagged(*c, BOTTOM_MESSAGE_ROW, 5 + i as u8, tag);
    }
    finish_text_capture(&mut t);
    assert_eq!(t.text_events().last().unwrap().text, "HYPERSPACE --");

    // Второй кусок приходит в следующем кадре. Буфер хранит первый, поэтому
    // событие содержит всю строку, а не только хвост.
    let next = MessageTag {
        life: tag.life - 1,
        ..tag
    };
    for (i, c) in b"LAVE".iter().enumerate() {
        t.capture_text_char_tagged(*c, BOTTOM_MESSAGE_ROW, 19 + i as u8, next);
    }
    finish_text_capture(&mut t);
    assert_eq!(
        t.text_events().last().unwrap().text,
        "HYPERSPACE -- LAVE"
    );
}

#[test]
fn text_capture_starts_fresh_for_new_message() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    let old = MessageTag {
        number: 0xC0,
        life: 10,
    };
    for (i, c) in b"OLD".iter().enumerate() {
        t.capture_text_char_tagged(*c, BOTTOM_MESSAGE_ROW, 5 + i as u8, old);
    }
    finish_text_capture(&mut t);
    assert_eq!(t.text_events().last().unwrap().text, "OLD");

    // Новая надпись с тем же номером, но снова взведённым временем жизни.
    let new = MessageTag {
        number: 0xC0,
        life: 14,
    };
    for (i, c) in b"NEW".iter().enumerate() {
        t.capture_text_char_tagged(*c, BOTTOM_MESSAGE_ROW, 5 + i as u8, new);
    }
    finish_text_capture(&mut t);
    assert_eq!(
        t.text_events().last().unwrap().text,
        "NEW",
        "хвост старой строки остался"
    );
}

#[test]
fn text_capture_clears_on_leaving_view() {
    let mut t = GameMsgTap::default();
    t.set_capture(true);
    t.capture_text_char(b'X', BOTTOM_MESSAGE_ROW, 5);
    t.end_text_frame();
    assert!(!t.text_events().is_empty());

    t.set_capture(false);
    assert!(t.text_events().is_empty(), "событие осталось после выхода");

    t.set_capture(true);
    t.capture_text_char(b'Y', BOTTOM_MESSAGE_ROW, 5);
    t.end_text_frame();
    assert_eq!(t.text_events()[0].text, "Y");
}

#[test]
fn text_capture_ignores_all_rows_except_bottom_message() {
    let mut t = GameMsgTap::default();
    t.enable_text_capture_for_test();
    for row in [1, 2, 10, 12, 14, 23] {
        t.capture_text_char(b'X', row, 4);
    }
    finish_text_capture(&mut t);
    assert!(t.text_events().is_empty());
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
