//! Тесты экрана-заглушки `DOCKED`.

use std::time::{Duration, Instant};

use super::*;

const WHITE: u32 = crate::cockpit::FRAME_COLOR;
const BG: u32 = 0xFF00_0000;

fn rendered() -> (Frame, usize, usize) {
    let (w, h) = (1280usize, 960usize);
    let mut hd = Frame::new(w, h);
    Docked {
        active: true,
        until: None,
    }
    .draw(&mut hd, crate::font::ZX_FONT);
    (hd, w, h)
}

#[test]
fn draws_standard_frame_and_docked_text() {
    let (hd, w, _h) = rendered();
    let px = hd.pixels();

    // Чёрный фон вместо игры.
    assert_eq!(px[0], BG);
    assert_eq!(px[(600 * w) + 600], BG);
    // Белая рамка: отступ `panel_geometry` (10 px) и толщина `LINE` (3 px).
    assert_eq!(px[10 * w + 10], WHITE);
    assert_eq!(px[12 * w + 12], WHITE);
    assert_eq!(px[10 * w + 200], WHITE);

    // Надпись — на седьмой строке сверху и по центру.
    let is_white = |x: usize, y: usize| px[y * w + x] == WHITE;
    let left = (20..w - 20)
        .find(|&x| (157..181).any(|y| is_white(x, y)))
        .expect("DOCKED not drawn");
    let right = (20..w - 20)
        .rev()
        .find(|&x| (157..181).any(|y| is_white(x, y)))
        .expect("DOCKED not drawn");
    assert!(left > 500 && right < 780, "text is not centered: {left}..{right}");
}

#[test]
fn expires_after_show_time() {
    let mut docked = Docked::new();
    assert!(!docked.is_active());
    docked.show();
    assert!(docked.is_active());
    docked.expire(Instant::now());
    assert!(docked.is_active(), "closed too early");
    docked.expire(Instant::now() + SHOW_TIME + Duration::from_millis(1));
    assert!(!docked.is_active(), "did not close in time");
}
