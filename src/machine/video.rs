//! Отрисовка ZX-кадра в RGBA и палитра Spectrum.

use super::*;

impl Spectrum {
    /// Decode the bitmap and attributes into a 256x192 RGBA pixel buffer.
    ///
    /// Это «старый» путь отрисовки: кадр всегда 256x192 независимо от
    /// разрешения окна — масштабирование до HD делает оконный слой
    /// (`main.rs`, целое множитель). В полном разрешении рисует HD-слой
    /// [`Spectrum::hd`] (нативные реализации).
    pub fn render_rgba(&self, out: &mut [u32]) {
        for y in 0..192u16 {
            for x in 0..256u16 {
                let (addr, bit) = pixel_addr(x, y);
                let on = self.mem[addr as usize] & bit != 0;
                let attr = self.mem[0x5800 + (y as usize / 8) * 32 + x as usize / 8];
                let ink = attr & 7;
                let paper = (attr >> 3) & 7;
                let bright = if attr & 0x40 != 0 { 8 } else { 0 };
                let palette = if on { ink } else { paper };
                out[y as usize * 256 + x as usize] = ZX_PALETTE[(palette | bright) as usize];
            }
        }
    }
}

/// Compute the bitmap byte address and bit mask for a screen pixel.
/// (0,0) is the top-left corner.
#[inline]
pub fn pixel_addr(x: u16, y: u16) -> (u16, u8) {
    let addr = 0x4000 + ((y & 0xC0) << 5) + ((y & 0x07) << 8) + ((y & 0x38) << 2) + (x >> 3);
    (addr, 0x80 >> (x & 7))
}

pub const ZX_PALETTE: [u32; 16] = [
    0x0000_0000, // black
    0x0000_00D7, // blue
    0x00D7_0000, // red
    0x00D7_00D7, // magenta
    0x0000_D700, // green
    0x0000_D7D7, // cyan
    0x00D7_D700, // yellow
    0x00D7_D7D7, // white
    0x0000_0000, // bright black
    0x0000_00FF, // bright blue
    0x00FF_0000, // bright red
    0x00FF_00FF, // bright magenta
    0x0000_FF00, // bright green
    0x0000_FFFF, // bright cyan
    0x00FF_FF00, // bright yellow
    0x00FF_FFFF, // bright white
];
