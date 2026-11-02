//! Наложение меню паузы в HD-слой.

use crate::display;
use crate::ui::present::SRC_W;

/// Наложение меню паузы в HD-слой: рамка и пункты «1 продолжить /
/// 2 сохранить / 3 загрузить / 4 выйти». Панель задана в пикселях ZX
/// (256x192) и увеличивается до целого масштаба слоя — текст чёткий в любом
/// окне. `sel` — выбранный пункт (0..3) для подсветки, `intro` — заставка
/// (сохранение отключено, поэтому пункт «сохранить» затемнён).
pub(crate) fn draw_pause_menu(frame: &mut display::Frame, font: &[u8], sel: usize, intro: bool) {
    const FG: u32 = 0xFFD7_D7D7; // белый (непрозрачный)
    const BG: u32 = 0xFF00_0000; // чёрный (непрозрачный)
    const DIM: u32 = 0x0080_8080; // затемнённый текст (прозрачный)
    const MARK: u32 = 0xFFFF_FF00; // жёлтый маркер выбора
    let scale = (frame.width / SRC_W).max(1);
    let s = scale as i32;
    let (x0, y0) = (40 * s, 52 * s);
    let (w, h) = (177 * scale, 89 * scale);
    frame.fill_rect(x0, y0, w, h, BG);
    // Рамка: внешняя и внутренняя линии.
    frame.hline(x0, y0, w, FG);
    frame.hline(x0, y0 + h as i32 - s, w, FG);
    frame.vline(x0, y0, h, FG);
    frame.vline(x0 + w as i32 - s, y0, h, FG);
    let lines = [
        (52 + 8, "ELITE - PAUSED"),
        (52 + 24, "1  RESUME"),
        (52 + 36, "2  SAVE"),
        (52 + 48, "3  LOAD"),
        (52 + 60, "4  QUIT GAME"),
    ];
    for (i, (y, text)) in lines.iter().enumerate() {
        // Пункты 1..4 идут после заголовка (i == 0).
        let item = i as isize - 1;
        let dim = intro && item == 1;
        let color = if dim { DIM } else { FG };
        let tx = x0 + 12 * s;
        let ty = *y as i32 * s;
        frame.draw_text(font, tx, ty, text, color, scale);
        if item >= 0 && item as usize == sel {
            frame.fill_rect(x0 + 4 * s, ty, (6 * scale) as usize, 8 * scale, MARK);
        }
    }
}
