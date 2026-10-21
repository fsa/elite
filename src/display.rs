//! Полноэкранная (HD) поверхность отрисовки.
//!
//! Кадр игры рисуется старым кодом по-прежнему в 256x192 (видеопамять
//! ZX Spectrum). В игре показываются только верхние 128 строк (вид из
//! кабины) целым множителем (nearest neighbour), а нижнюю часть окна
//! занимает нативная панель. Рядом существует HD-слой [`Frame`] — поверхность
//! в разрешении всего окна, в которую нативные (Rust) реализации могут
//! рисовать в полном разрешении (панель, рамка, гладкие линии, крупный текст).
//!
//! Слой очищается перед каждым кадром, нативные хуки рисуют в него во
//! время эмуляции ([`Spectrum::hd`](crate::machine::Spectrum)), а при
//! показе непрозрачные пиксели слоя (старший байт != 0) накладываются
//! поверх масштабированного ZX-кадра.

/// Ширина кадра ZX Spectrum.
pub const ZX_W: usize = 256;
/// Высота кадра ZX Spectrum.
pub const ZX_H: usize = 192;

/// Прозрачный пиксель HD-слоя: накладывается только непрозрачный цвет
/// (со старшим байтом != 0, например `0xFFRRGGBB`).
pub const TRANSPARENT: u32 = 0;

/// Область показа ZX-кадра в окне: целый масштаб и положение по центру.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Layout {
    /// Целый масштаб (>= 1): пиксели ZX строго квадратные и чёткие.
    pub scale: usize,
    /// Левый верхний угол области в пикселях окна.
    pub x: u32,
    pub y: u32,
    /// Размер области (256*scale x 192*scale, обрезается по окну).
    pub w: u32,
    pub h: u32,
}

/// Рассчитать область показа для окна `win_w` x `win_h`.
///
/// Масштаб — наибольшее целое, влезающее по обеим осям (в полноэкранном
/// режиме 1920x1080 — это x5, картинка 1280x960 по центру). Если окно
/// меньше кадра ZX, масштаб принудительно 1, а область обрезается по окну.
pub fn layout(win_w: u32, win_h: u32) -> Layout {
    let scale = ((win_w as usize / ZX_W).min(win_h as usize / ZX_H)).max(1);
    let w = (ZX_W * scale).min(win_w as usize) as u32;
    let h = (ZX_H * scale).min(win_h as usize) as u32;
    Layout {
        scale,
        x: (win_w - w) / 2,
        y: (win_h - h) / 2,
        w,
        h,
    }
}

/// Пиксельная поверхность произвольного разрешения (RGBA, `0xFFRRGGBB`).
pub struct Frame {
    pub width: usize,
    pub height: usize,
    px: Vec<u32>,
    /// Необязательная область отсечения `(x0, y0, x1, y1)` (правый/нижний
    /// край исключается). `None` — рисовать во всю поверхность.
    clip: Option<(i32, i32, i32, i32)>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            width,
            height,
            px: vec![TRANSPARENT; width * height],
            clip: None,
        }
    }

    /// Изменить размер поверхности (все пиксели обнуляются).
    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.px.clear();
            self.px.resize(width * height, TRANSPARENT);
        }
    }

    /// Очистить слой (по умолчанию — прозрачным).
    pub fn clear(&mut self, color: u32) {
        self.px.fill(color);
    }

    /// Ограничить рисование прямоугольником `(x, y)` размером `w` x `h`.
    ///
    /// Действует на `plot`/`fill_rect` и всё, что через них построено
    /// (линии, круги, текст). Нужно, например, чтобы поле карты галактики
    /// при увеличении не вылезало на заголовок и панель.
    pub fn set_clip(&mut self, x: i32, y: i32, w: usize, h: usize) {
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x.max(0) as usize + w).min(self.width) as i32;
        let y1 = (y.max(0) as usize + h).min(self.height) as i32;
        self.clip = Some((x0, y0, x1, y1));
    }

    /// Снять ограничение рисования (рисовать во всю поверхность).
    pub fn clear_clip(&mut self) {
        self.clip = None;
    }

    /// Пересечение запрошенного прямоугольника с границей отсечения и слоя.
    fn clamped_rect(&self, x: i32, y: i32, w: usize, h: usize) -> Option<(usize, usize, usize, usize)> {
        let mut x0 = x.max(0);
        let mut y0 = y.max(0);
        let mut x1 = (x.max(0) as usize + w).min(self.width) as i32;
        let mut y1 = (y.max(0) as usize + h).min(self.height) as i32;
        if let Some((cx0, cy0, cx1, cy1)) = self.clip {
            x0 = x0.max(cx0);
            y0 = y0.max(cy0);
            x1 = x1.min(cx1);
            y1 = y1.min(cy1);
        }
        if x0 >= x1 || y0 >= y1 {
            None
        } else {
            Some((x0 as usize, y0 as usize, x1 as usize, y1 as usize))
        }
    }

    /// Пиксели слоя (только для чтения).
    #[allow(dead_code)] // читают тесты и будущие нативные реализации
    pub fn pixels(&self) -> &[u32] {
        &self.px
    }

    /// Одиночный пиксель с отсечением по границам.
    pub fn plot(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            if let Some((cx0, cy0, cx1, cy1)) = self.clip {
                if x < cx0 || y < cy0 || x >= cx1 || y >= cy1 {
                    return;
                }
            }
            self.px[y as usize * self.width + x as usize] = color;
        }
    }

    /// Залитый прямоугольник `(x, y)` размером `w` x `h` (с отсечением).
    pub fn fill_rect(&mut self, x: i32, y: i32, w: usize, h: usize, color: u32) {
        let Some((x0, y0, x1, y1)) = self.clamped_rect(x, y, w, h) else {
            return;
        };
        for yy in y0..y1 {
            self.px[yy * self.width + x0..yy * self.width + x1].fill(color);
        }
    }

    /// Горизонтальная линия из `len` пикселей от `(x, y)`.
    pub fn hline(&mut self, x: i32, y: i32, len: usize, color: u32) {
        self.fill_rect(x, y, len, 1, color);
    }

    /// Вертикальная линия из `len` пикселей от `(x, y)`.
    pub fn vline(&mut self, x: i32, y: i32, len: usize, color: u32) {
        self.fill_rect(x, y, 1, len, color);
    }

    /// Отрезок произвольного направления (алгоритм Брезенхэма).
    #[allow(dead_code)] // API для нативных реализаций и тестов
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let (mut x, mut y) = (x0, y0);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.plot(x, y, color);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Залитый круг радиуса `r` с центром `(cx, cy)`.
    pub fn disc(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        if r < 0 {
            return;
        }
        for dy in -r..=r {
            let dx = isqrt(r * r - dy * dy);
            self.hline(cx - dx, cy + dy, (2 * dx + 1) as usize, color);
        }
    }

    /// Контур окружности радиуса `r` (алгоритм средней точки).
    pub fn circle(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        if r < 0 {
            return;
        }
        let (mut x, mut y, mut d) = (r, 0, 1 - r);
        while x >= y {
            self.plot(cx + x, cy + y, color);
            self.plot(cx + y, cy + x, color);
            self.plot(cx - y, cy + x, color);
            self.plot(cx - x, cy + y, color);
            self.plot(cx - x, cy - y, color);
            self.plot(cx - y, cy - x, color);
            self.plot(cx + y, cy - x, color);
            self.plot(cx + x, cy - y, color);
            y += 1;
            if d <= 0 {
                d += 2 * y + 1;
            } else {
                x -= 1;
                d += 2 * (y - x) + 1;
            }
        }
    }

    /// Наложить отснятый ZX-кадр 256x192 целым множителем `scale` со
    /// смещением `(ox, oy)`; выход за границы обрезается.
    #[allow(dead_code)] // API для нативных реализаций и тестов
    pub fn blit_zx(&mut self, src: &[u32], scale: usize, ox: usize, oy: usize) {
        debug_assert_eq!(src.len(), ZX_W * ZX_H);
        let scale = scale.max(1);
        for sy in 0..ZX_H {
            for sx in 0..ZX_W {
                let color = src[sy * ZX_W + sx];
                for dy in 0..scale {
                    let y = oy + sy * scale + dy;
                    if y >= self.height {
                        break;
                    }
                    for dx in 0..scale {
                        let x = ox + sx * scale + dx;
                        if x >= self.width {
                            break;
                        }
                        self.px[y * self.width + x] = color;
                    }
                }
            }
        }
    }

    /// Нарисовать строку шрифтом ПЗУ ZX (8x8, коды 0x20..0x7F), увеличенной
    /// в `scale` раз. Рисуются только установленные биты (фон прозрачный).
    pub fn draw_text(&mut self, font: &[u8], x: i32, y: i32, s: &str, color: u32, scale: usize) {
        let scale = scale.max(1);
        let mut cx = x;
        for b in s.bytes() {
            let ch = b.clamp(0x20, 0x7F) as usize;
            let off = (ch - 0x20) * 8;
            if off + 8 > font.len() {
                break;
            }
            for (row, bits) in font[off..off + 8].iter().enumerate() {
                for col in 0..8 {
                    if bits & (0x80 >> col) != 0 {
                        self.fill_rect(
                            cx + col as i32 * scale as i32,
                            y + row as i32 * scale as i32,
                            scale,
                            scale,
                            color,
                        );
                    }
                }
            }
            cx += 8 * scale as i32;
        }
    }

    /// Наложить слой на буфер `out` (ширина `out_w`, высота `out_h`) со
    /// смещением `(ox, oy)`: непрозрачные пиксели слоя заменяют подложку.
    pub fn composite_onto(&self, out: &mut [u32], out_w: usize, out_h: usize, ox: usize, oy: usize) {
        let dw = self.width.min(out_w.saturating_sub(ox));
        let dh = self.height.min(out_h.saturating_sub(oy));
        for y in 0..dh {
            let src_row = &self.px[y * self.width..y * self.width + dw];
            let dst_lo = (oy + y) * out_w + ox;
            let dst_row = &mut out[dst_lo..dst_lo + dw];
            for (d, &s) in dst_row.iter_mut().zip(src_row) {
                if s & 0xFF00_0000 != 0 {
                    *d = s;
                }
            }
        }
    }
}

/// Целочисленный квадратный корень (для радиусов кругов).
fn isqrt(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut x = 0;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::Spectrum;
    use crate::z80::Z80;

    /// Целый масштаб и положение области по центру окна.
    #[test]
    fn layout_uses_largest_integer_scale() {
        // FullHD 1920x1080: масштаб x5 (1080/192 = 5.625), 1280x960 по центру.
        let l = layout(1920, 1080);
        assert_eq!(l.scale, 5);
        assert_eq!((l.w, l.h), (1280, 960));
        assert_eq!((l.x, l.y), (320, 60));
        // HD 1280x720: масштаб x3 (720/192 = 3.75).
        let l = layout(1280, 720);
        assert_eq!(l.scale, 3);
        assert_eq!((l.w, l.h), (768, 576));
        // Окно меньше кадра ZX: масштаб 1, область обрезается по окну.
        let l = layout(100, 100);
        assert_eq!(l.scale, 1);
        assert_eq!((l.w, l.h), (100, 100));
    }

    /// ZX-кадр масштабируется целым множителем: каждый пиксель -> квадрат.
    #[test]
    fn blit_zx_scales_by_integer_factor() {
        // Источник 256x192: (0,0) белый, (0,1) красный, остальные чёрные.
        let mut src = vec![0u32; ZX_W * ZX_H];
        src[0] = 0xFF_FF_FF;
        src[ZX_W] = 0xFF_00_00;
        let mut f = Frame::new(4, 4);
        f.blit_zx(&src, 1, 0, 0);
        assert_eq!(f.pixels()[0], 0xFF_FF_FF);
        assert_eq!(f.pixels()[1], 0);
        assert_eq!(f.pixels()[4], 0xFF_00_00);
        // Масштаб x2: пиксель (0,0) -> блок 2x2, (0,1) -> блок ниже.
        let mut f = Frame::new(4, 4);
        f.blit_zx(&src, 2, 0, 0);
        assert_eq!(f.pixels()[0], 0xFF_FF_FF);
        assert_eq!(f.pixels()[1], 0xFF_FF_FF);
        assert_eq!(f.pixels()[2], 0);
        assert_eq!(f.pixels()[2 * 4 + 0], 0xFF_00_00);
        assert_eq!(f.pixels()[2 * 4 + 1], 0xFF_00_00);
        assert_eq!(f.pixels()[2 * 4 + 2], 0);
    }

    /// Композитинг: непрозрачные пиксели HD-слоя заменяют подложку,
    /// прозрачные её оставляют.
    #[test]
    fn composite_takes_only_opaque_pixels() {
        let mut out = vec![0xAA_BB_CC; 4];
        let mut f = Frame::new(2, 2);
        f.plot(0, 0, 0xFF11_2233); // непрозрачный
        f.plot(1, 1, 0x0011_2233); // прозрачный (альфа = 0)
        f.composite_onto(&mut out, 2, 2, 0, 0);
        assert_eq!(out[0], 0xFF11_2233);
        assert_eq!(out[1], 0xAA_BB_CC);
        assert_eq!(out[2], 0xAA_BB_CC);
        assert_eq!(out[3], 0xAA_BB_CC);
        // Смещение и обрезка по границам буфера.
        let mut out = vec![0u32; 8];
        let f = Frame::new(4, 4);
        f.composite_onto(&mut out, 4, 2, 3, 1);
    }

    /// Текст шрифтом ПЗУ рисуется увеличенным: каждый бит — квадрат scale x scale.
    #[test]
    fn draw_text_scales_glyph_pixels() {
        // Шрифт, где залиты все биты всех глифов: «A» = 64 пикселя.
        let font = vec![0xFFu8; 0x300];
        let mut f = Frame::new(64, 8);
        f.draw_text(&font, 0, 0, "A", 0xFF_FF_FF, 1);
        let lit_1x = f.pixels().iter().filter(|&&p| p != 0).count();
        assert_eq!(lit_1x, 64, "glyph not drawn");
        let mut f = Frame::new(128, 16);
        f.draw_text(&font, 0, 0, "A", 0xFF_FF_FF, 2);
        let lit_2x = f.pixels().iter().filter(|&&p| p != 0).count();
        assert_eq!(lit_2x, lit_1x * 4, "scale=2 must give 4x pixels");
    }

    /// Круг и его контур: залитый диск — все точки внутри радиуса,
    /// контур — границы (не меньше 2*r*... точек).
    #[test]
    fn disc_and_circle_cover_expected_pixels() {
        let mut f = Frame::new(32, 32);
        f.disc(16, 16, 5, 0xFF_FF_FF);
        assert_eq!(f.pixels()[16 * 32 + 16], 0xFF_FF_FF, "center");
        assert_eq!(f.pixels()[16 * 32 + 21], 0xFF_FF_FF, "right edge");
        assert_eq!(f.pixels()[16 * 32 + 22], 0, "outside");
        assert_ne!(f.pixels()[16 * 32 + 11], 0, "left edge");
        let mut g = Frame::new(32, 32);
        g.circle(16, 16, 5, 0xFF_FF_FF);
        assert_eq!(g.pixels()[16 * 32 + 16], 0, "outline has hollow center");
        assert_ne!(g.pixels()[16 * 32 + 21], 0, "outline on right edge");
        // Радиус 0 — один пиксель.
        let mut h = Frame::new(4, 4);
        h.disc(1, 1, 0, 0xFF_FF_FF);
        assert_eq!(h.pixels().iter().filter(|&&p| p != 0).count(), 1);
    }

    /// Отсечение: рисование ограничивается прямоугольником `set_clip`.
    #[test]
    fn clip_limits_drawing() {
        let mut f = Frame::new(16, 16);
        f.set_clip(4, 4, 8, 8);
        f.fill_rect(0, 0, 16, 16, 0xFF_FF_FF); // во всю поверхность
        assert_eq!(f.pixels()[4 * 16 + 4], 0xFF_FF_FF, "inside clip");
        assert_eq!(f.pixels()[11 * 16 + 11], 0xFF_FF_FF, "clip corner");
        assert_eq!(f.pixels()[3 * 16 + 4], 0, "left of clip");
        assert_eq!(f.pixels()[12 * 16 + 4], 0, "right of clip");
        assert_eq!(f.pixels()[4 * 16 + 3], 0, "above clip");
        // plot тоже уважает отсечение.
        f.plot(2, 2, 0xFF_00_00);
        assert_eq!(f.pixels()[2 * 16 + 2], 0, "plot outside clip");
        // Снятие отсечения возвращает рисование во всю поверхность.
        f.clear_clip();
        f.plot(2, 2, 0xFF_00_00);
        assert_eq!(f.pixels()[2 * 16 + 2], 0xFF_00_00, "plot after clear_clip");
    }

    /// Нативный хук рисует в HD-слой машины в полном разрешении —
    /// проверка связки «хук -> Spectrum::hd -> композитинг».
    #[test]
    fn native_hook_draws_into_hd_layer() {
        fn draw_hook(_cpu: &mut Z80, s: &mut Spectrum) -> u64 {
            // Жёлтый непрозрачный: 0xFF в старшем байте.
            s.hd.hline(10, 20, 100, 0xFF_FF_FF00);
            13
        }
        let mut m = Spectrum::new(None);
        m.hd = Frame::new(640, 480);
        m.hooks.insert(0x8000, draw_hook);
        m.cpu.pc = 0x8000;
        m.cpu.sp = 0x9000;
        m.mem[0x9000] = 0x00; // адрес возврата 0x4000 (NOP-ы)
        m.mem[0x9001] = 0x40;
        m.run_frame();
        assert_eq!(m.hd.pixels()[20 * 640 + 10], 0xFF_FF_FF00, "line start");
        assert_eq!(m.hd.pixels()[20 * 640 + 109], 0xFF_FF_FF00, "line end");
        assert_eq!(m.hd.pixels()[20 * 640 + 110], TRANSPARENT, "no overrun");
        assert_eq!(m.hd.pixels()[19 * 640 + 10], TRANSPARENT, "single row");
        // Композитинг поверх подложки.
        let mut out = vec![7u32; 640 * 480];
        m.hd.composite_onto(&mut out, 640, 480, 0, 0);
        assert_eq!(out[20 * 640 + 10], 0xFF_FF_FF00);
        assert_eq!(out[21 * 640 + 10], 7);
    }
}
