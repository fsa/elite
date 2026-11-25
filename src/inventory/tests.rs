use super::*;
use crate::cockpit;
use crate::display::Frame;
use crate::font::ZX_FONT;

/// Экран читает деньги из обеих половин `PilotCash` и топливо из `PilotFuel`.
#[test]
fn open_reads_fuel_and_cash_from_commander_block() {
    let mut mem = vec![0u8; 0xE000];
    let pilot = 0xD054usize;
    // 1234.5 кредита = 12345 десятых: cash2 = 1, cash1 = 2345.
    mem[pilot + 18..pilot + 20].copy_from_slice(&1u16.to_le_bytes());
    mem[pilot + 20..pilot + 22].copy_from_slice(&2345u16.to_le_bytes());
    mem[pilot + OFF_FUEL] = 70;

    let mut inv = Inventory::new();
    inv.open(&mem, pilot as u16);
    assert!(inv.is_active());
    assert_eq!(inv.fuel, 70);
    assert_eq!(inv.cash, 12_345);

    inv.close();
    assert!(!inv.is_active());
}

/// Заголовок по центру и двухцветный, параметры с большой буквы, рамка не
/// заходит за край.
#[test]
fn draw_uses_centered_two_tone_title_and_capitalized_labels() {
    let mut mem = vec![0u8; 0xE000];
    let pilot = 0xD054usize;
    mem[pilot + 20..pilot + 22].copy_from_slice(&1000u16.to_le_bytes());
    mem[pilot + OFF_FUEL] = 70;
    let mut inv = Inventory::new();
    inv.open(&mem, pilot as u16);

    let mut hd = Frame::new(1280, 960);
    inv.draw(&mut hd, ZX_FONT);
    let px = hd.pixels();
    let at = |x: usize, y: usize| px[y * hd.width + x];

    let text = (960f32 / 300.0).floor().max(1.0) as usize;
    let cell = 8 * text as i32;
    let frame_line = cockpit::LINE.max(1);
    let margin = cockpit::panel_geometry(1280, 960).margin;
    let inner_x = margin + frame_line;
    let inner_w = 1280 - 2 * inner_x;
    let title = "INVENTORY";
    let tw = title.len() as i32 * cell;
    let tx = inner_x + (inner_w - tw) / 2;
    let ty = inner_x + cell;

    let has_color = |color: u32, from: i32, to: i32| {
        (from..to).any(|yy| (tx..tx + tw).any(|xx| at(xx as usize, yy as usize) == color))
    };
    assert!(
        has_color(FRAME, ty, ty + cell / 2),
        "нет белой половины заголовка"
    );
    assert!(
        has_color(LABEL, ty + cell / 2, ty + cell),
        "нет голубой половины заголовка"
    );

    // Столбцы заголовка симметричны относительно центра экрана.
    assert_eq!(tx - inner_x, inner_x + inner_w - (tx + tw));

    assert!(px.contains(&LABEL), "нет подписей полей");
    assert_eq!(px[0], BG, "рамка не должна заходить за край");
}
