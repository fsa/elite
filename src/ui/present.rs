//! Показ ZX-кадра 256x192 в окне: целочисленное масштабирование по центру.

use crate::display;
use crate::font::ZX_FONT;

/// Размер кадра игры (ZX Spectrum). HD-слой и масштабирование — см. `display`.
pub(crate) const SRC_W: usize = display::ZX_W;
pub(crate) const SRC_H: usize = display::ZX_H;

/// Скопировать ZX-кадр (`src`, 256x192) в буфер окна целым множителем
/// `l.scale` по центру области показа (letterbox).
///
/// Показывается кадр целиком: и вид из кабины (верхние 128 строк), и родная
/// растровая панель оригинала (нижние 64 строки) — нативная панель из
/// `src/cockpit.rs` в игре не рисуется.
pub(crate) fn blit_zx_frame(
    src: &[u32],
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    l: &display::Layout,
) {
    let (dw, dh) = ((l.w as usize).min(out_w), (l.h as usize).min(out_h));
    for row in 0..dh {
        let sy = row / l.scale;
        let src_row = &src[sy * SRC_W..sy * SRC_W + SRC_W];
        let dst_lo = (l.y as usize + row) * out_w + l.x as usize;
        let dst_row = &mut out[dst_lo..dst_lo + dw];
        for (col, d) in dst_row.iter_mut().enumerate() {
            *d = src_row[col / l.scale];
        }
    }
}

/// Отладочная полоса справа: кадр эмулятора 1:1 вверху (ширина ZX-экрана,
/// 256 пикселей), ниже — шестнадцатеричный дамп блока сохранения.
pub(crate) fn draw_debug_panel(
    src: &[u32],
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    save_data: &[u8],
) {
    let panel_w = SRC_W.min(out_w);
    let x0 = out_w - panel_w;
    // Вся полоса чёрная, включая нижнюю часть под миниатюрой.
    for y in 0..out_h {
        out[y * out_w + x0..y * out_w + out_w].fill(0);
    }
    let h = SRC_H.min(out_h);
    for y in 0..h {
        let dst = y * out_w + x0;
        out[dst..dst + panel_w].copy_from_slice(&src[y * SRC_W..y * SRC_W + panel_w]);
    }
    draw_save_dump(out, out_w, out_h, x0, save_data);
}

const CYAN: u32 = 0xFF00_FFFF;
const WHITE: u32 = 0xFFD7_D7D7;
const RED: u32 = 0xFFFF_0000;

/// Дамп `save_data` под миниатюрой эмулятора: адрес от начала блока (голубым),
/// два пробела, 8 байт через пробел. Отступ одно знакоместо от края панели и
/// от основного экрана, с отступом между строками. Под каждым байтом в этом
/// отступе показан соответствующий символ (0x20..0x7F — из ZX-набора,
/// < 0x20 — голубая точка, > 0x7F — красная точка).
fn draw_save_dump(out: &mut [u32], out_w: usize, out_h: usize, x0: usize, save_data: &[u8]) {
    if save_data.is_empty() {
        return;
    }
    let left = x0 + 8;
    let mut y = SRC_H as i32 + 8;
    for (i, chunk) in save_data.chunks(8).enumerate() {
        if y + 16 > out_h as i32 {
            break;
        }
        let mut s = format!("{:04X}  ", i * 8);
        for (j, b) in chunk.iter().enumerate() {
            if j > 0 {
                s.push(' ');
            }
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02X}");
        }
        draw_text(out, out_w, out_h, left as i32, y, &s, CYAN, WHITE);
        for (j, &b) in chunk.iter().enumerate() {
            let cx = left as i32 + (6 + j as i32 * 3) * 8;
            draw_byte_char(out, out_w, out_h, cx, y + 8, b);
        }
        y += 24;
    }
}

fn draw_byte_char(out: &mut [u32], out_w: usize, out_h: usize, x: i32, y: i32, b: u8) {
    match b {
        0x20..=0x7F => draw_char(out, out_w, out_h, x, y, b as char, WHITE),
        0x00..=0x1F => draw_char(out, out_w, out_h, x, y, '.', CYAN),
        _ => draw_char(out, out_w, out_h, x, y, '.', RED),
    }
}

fn draw_text(
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    x: i32,
    y: i32,
    s: &str,
    addr_color: u32,
    data_color: u32,
) {
    let mut cx = x;
    for (i, ch) in s.chars().enumerate() {
        // "ADDR  " — первые 6 символов голубым, остальное белым.
        let color = if i < 6 { addr_color } else { data_color };
        draw_char(out, out_w, out_h, cx, y, ch, color);
        cx += 8;
    }
}

fn draw_char(out: &mut [u32], out_w: usize, out_h: usize, x: i32, y: i32, ch: char, color: u32) {
    let code = ch as u32;
    if !(0x20..=0x7F).contains(&code) {
        return;
    }
    let off = ((code - 0x20) as usize) * 8;
    for (row, bits) in ZX_FONT[off..off + 8].iter().enumerate() {
        let py = y + row as i32;
        if py < 0 || py >= out_h as i32 {
            continue;
        }
        for bit in 0..8 {
            if bits & (0x80 >> bit) != 0 {
                let px = x + bit as i32;
                if px >= 0 && px < out_w as i32 {
                    out[py as usize * out_w + px as usize] = color;
                }
            }
        }
    }
}
