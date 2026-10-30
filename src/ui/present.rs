//! Показ ZX-кадра 256x192 в окне: целочисленное масштабирование по центру.

use crate::display;

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
