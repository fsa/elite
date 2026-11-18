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

/// Что показывать под миниатюрой эмулятора. По умолчанию — сокращённая
/// сводка по полям блока командира; дамп всех байтов включается второй
/// кнопкой.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum DebugView {
    #[default]
    Summary,
    Dump,
}

/// Отступ кнопок от края полосы и промежуток между ними.
const BUTTON_MARGIN: usize = 8;
/// Высота кнопки в пикселях.
const BUTTON_H: usize = 24;
/// Шаг строк сокращённой сводки (шрифт 8 пикселей + просвет).
const SUMMARY_LINE: i32 = 12;

/// Прямоугольники двух кнопок переключения режима под миниатюрой:
/// `(x, y, ширина, высота)` в координатах окна.
pub(crate) fn debug_button_rects(out_w: usize) -> [(i32, i32, usize, usize); 2] {
    let x0 = crate::display::debug_panel_x(out_w as u32) as i32;
    let w = (SRC_W - 3 * BUTTON_MARGIN) / 2;
    let y = SRC_H as i32 + BUTTON_MARGIN as i32;
    [
        (x0 + BUTTON_MARGIN as i32, y, w, BUTTON_H),
        (
            x0 + (2 * BUTTON_MARGIN + w) as i32,
            y,
            w,
            BUTTON_H,
        ),
    ]
}

/// Режим, на кнопку которого пришёлся клик в координатах окна.
pub(crate) fn debug_button_at(out_w: usize, x: f64, y: f64) -> Option<DebugView> {
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let (px, py) = (x as i32, y as i32);
    let views = [DebugView::Summary, DebugView::Dump];
    for (i, &(bx, by, bw, bh)) in debug_button_rects(out_w).iter().enumerate() {
        if px >= bx && px < bx + bw as i32 && py >= by && py < by + bh as i32 {
            return Some(views[i]);
        }
    }
    None
}

/// Отладочная полоса справа: кадр эмулятора 1:1 вверху (ширина ZX-экрана,
/// 256 пикселей), под ним — две кнопки выбора режима и содержимое выбранного
/// режима: сводка по полям сейва или полный шестнадцатеричный дамп.
pub(crate) fn draw_debug_panel(
    src: &[u32],
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    save_data: &[u8],
    view: DebugView,
) {
    let x0 = crate::display::debug_panel_x(out_w as u32) as usize;
    let panel_w = out_w - x0;
    // Вся полоса чёрная, включая нижнюю часть под миниатюрой.
    for y in 0..out_h {
        out[y * out_w + x0..y * out_w + out_w].fill(0);
    }
    let h = SRC_H.min(out_h);
    for y in 0..h {
        let dst = y * out_w + x0;
        out[dst..dst + panel_w].copy_from_slice(&src[y * SRC_W..y * SRC_W + panel_w]);
    }
    draw_debug_buttons(out, out_w, out_h, view);
    let top = SRC_H as i32 + (BUTTON_MARGIN + BUTTON_H + BUTTON_MARGIN) as i32;
    match view {
        DebugView::Summary => draw_save_summary(out, out_w, out_h, x0, save_data, top),
        DebugView::Dump => draw_save_dump(out, out_w, out_h, x0, save_data, top),
    }
}

const CYAN: u32 = 0xFF00_FFFF;
const WHITE: u32 = 0xFFD7_D7D7;
const GREY: u32 = 0xFF80_8080;
const RED: u32 = 0xFFFF_0000;

/// Две кнопки под миниатюрой. Активная обведена голубым, неактивная — серым.
fn draw_debug_buttons(out: &mut [u32], out_w: usize, out_h: usize, view: DebugView) {
    let buttons = [
        ("1 SUMMARY", DebugView::Summary),
        ("2 HEX DUMP", DebugView::Dump),
    ];
    for (i, &(label, button_view)) in buttons.iter().enumerate() {
        let (x, y, w, h) = debug_button_rects(out_w)[i];
        let active = view == button_view;
        fill_rect(out, out_w, out_h, x, y, w, h, 0xFF10_1010);
        let color = if active { CYAN } else { GREY };
        // Рамка толщиной 1 пиксель по периметру кнопки.
        fill_rect(out, out_w, out_h, x, y, w, 1, color);
        fill_rect(out, out_w, out_h, x, y + h as i32 - 1, w, 1, color);
        fill_rect(out, out_w, out_h, x, y, 1, h, color);
        fill_rect(out, out_w, out_h, x + w as i32 - 1, y, 1, h, color);
        let tx = x + ((w as i32 - label.len() as i32 * 8) / 2).max(0);
        let ty = y + (h as i32 - 8) / 2;
        let text = if active { WHITE } else { GREY };
        draw_text(out, out_w, out_h, tx, ty, label, text, text);
    }
}

/// Дамп `save_data` под миниатюрой эмулятора: адрес от начала блока (голубым),
/// два пробела, 8 байт через пробел. Отступ одно знакоместо от края панели и
/// от основного экрана, с отступом между строками. Под каждым байтом в этом
/// отступе показан соответствующий символ (0x20..0x7F — из ZX-набора,
/// < 0x20 — голубая точка, > 0x7F — красная точка).
fn draw_save_dump(
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    x0: usize,
    save_data: &[u8],
    top: i32,
) {
    if save_data.is_empty() {
        return;
    }
    let left = x0 + 8;
    let mut y = top;
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

/// Сокращённая сводка по блоку сохранения: поля из `docs/kladov_save.md` с
/// адресом от начала блока. Известные поля показаны разобранными, области с
/// неустановленной раскладкой — сгруппированными байтами.
fn draw_save_summary(
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    x0: usize,
    save_data: &[u8],
    top: i32,
) {
    if save_data.is_empty() {
        return;
    }
    let left = x0 + 8;
    let mut y = top;
    let b = |off: usize| save_data.get(off).copied().unwrap_or(0);
    let u32_at = |off: usize| {
        u32::from_le_bytes([b(off), b(off + 1), b(off + 2), b(off + 3)])
    };

    let name = summary_name(save_data);
    let legal = b(0x0B);
    let rating = u32::from_le_bytes([b(0x0C), b(0x0D), b(0x0E), 0]);
    let cash = u32_at(0x14);

    let rows: [(usize, &str, String); 26] = [
        (0x00, "NAME", name),
        (0x0B, "LEGAL", format!("{legal} {}", legal_text(legal))),
        (0x0C, "RATING", format!("{rating} {}", rating_text(rating))),
        (0x0F, "SERVICE", hex_at(save_data, 0x0F, 5)),
        (
            0x14,
            "CASH",
            format!("{}.{} Cr", cash / 10, cash % 10),
        ),
        (0x18, "CARGO", hex_at(save_data, 0x18, 6)),
        (0x1E, "CARGO", hex_at(save_data, 0x1E, 6)),
        (0x24, "CARGO", hex_at(save_data, 0x24, 5)),
        (0x28, "BAY", format!("{} t", b(0x28))),
        (0x29, "FRONT", format!("{}", b(0x29))),
        (0x2A, "SLOTS", hex_at(save_data, 0x2A, 4)),
        (
            0x2E,
            "FUEL",
            format!("{}.{} LY", b(0x2E) / 10, b(0x2E) % 10),
        ),
        (0x2F, "MISSILES", format!("{}", b(0x2F))),
        (0x30, "EQUIP", hex_at(save_data, 0x30, 6)),
        (0x36, "EQUIP", hex_at(save_data, 0x36, 6)),
        (0x3C, "GALAXY", hex_at(save_data, 0x3C, 6)),
        (0x42, "PLANET", hex_at(save_data, 0x42, 1)),
        (0x43, "SYS-Y", format!("{}", b(0x43))),
        (0x44, "SERVICE", hex_at(save_data, 0x44, 1)),
        (0x45, "SYS-X", format!("{}", b(0x45))),
        (0x46, "PDATA", hex_at(save_data, 0x46, 6)),
        (0x4C, "PDATA", hex_at(save_data, 0x4C, 6)),
        (0x52, "PDATA", hex_at(save_data, 0x52, 6)),
        (0x58, "PDATA", hex_at(save_data, 0x58, 5)),
        (0x5D, "SUN", hex_at(save_data, 0x5D, 6)),
        (0x63, "SUN", hex_at(save_data, 0x63, 3)),
    ];
    for (addr, label, value) in &rows {
        if !summary_line(out, out_w, out_h, left as i32, &mut y, *addr, label, value) {
            break;
        }
    }
}

/// Одна строка сводки: адрес и имя поля голубым, значение белым.
fn summary_line(
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    x: i32,
    y: &mut i32,
    addr: usize,
    label: &str,
    value: &str,
) -> bool {
    if *y + 8 > out_h as i32 {
        return false;
    }
    let head = format!("{addr:02X} {label}: ");
    draw_text(out, out_w, out_h, x, *y, &head, CYAN, CYAN);
    let vx = x + head.chars().count() as i32 * 8;
    draw_text(out, out_w, out_h, vx, *y, value, WHITE, WHITE);
    *y += SUMMARY_LINE;
    true
}

/// Имя командира: строка до `NUL`; непечатаемые байты заменяются на `?`.
fn summary_name(data: &[u8]) -> String {
    let end = data[..data.len().min(11)]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or_else(|| data.len().min(11));
    data[..end]
        .iter()
        .map(|&b| if (0x20..0x7F).contains(&b) { b as char } else { '?' })
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Правовой статус как в родном `Cmd_L_PilotInfo`: `0` — чист,
/// `< 0x32` — нарушитель, остальное — беглец.
fn legal_text(legal: u8) -> &'static str {
    match legal {
        0 => "Clean",
        1..=0x31 => "Offender",
        _ => "Fugitive",
    }
}

/// Боевой рейтинг по таблицам `byte_F0AE`/`byte_F0B4`.
fn rating_text(rating: u32) -> &'static str {
    match rating {
        0..=7 => "Harmless",
        8..=15 => "Mostly Harmless",
        16..=31 => "Poor",
        32..=63 => "Average",
        64..=127 => "Above Average",
        128..=511 => "Competent",
        512..=2559 => "Dangerous",
        2560..=6399 => "Deadly",
        _ => "Elite",
    }
}

/// `len` байт с `start` через пробел (`00 01 02`). Недостающие байты
/// (обрезанный блок) пропускаются.
fn hex_at(data: &[u8], start: usize, len: usize) -> String {
    data.iter()
        .skip(start)
        .take(len)
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Залить прямоугольник цветом с обрезкой по границам буфера.
fn fill_rect(
    out: &mut [u32],
    out_w: usize,
    out_h: usize,
    x: i32,
    y: i32,
    w: usize,
    h: usize,
    color: u32,
) {
    let x0 = x.max(0) as usize;
    let y0 = y.max(0) as usize;
    let x1 = x.saturating_add(w as i32).clamp(0, out_w as i32) as usize;
    let y1 = y.saturating_add(h as i32).clamp(0, out_h as i32) as usize;
    for py in y0..y1 {
        for px in x0..x1 {
            out[py * out_w + px] = color;
        }
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
