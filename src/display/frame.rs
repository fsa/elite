//! Пиксельная поверхность Frame: примитивы, отсечение и наложение.

use super::*;

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
    fn clamped_rect(
        &self,
        x: i32,
        y: i32,
        w: usize,
        h: usize,
    ) -> Option<(usize, usize, usize, usize)> {
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

    /// Нарисовать строку встроенным шрифтом ZX (8x8, коды 0x20..0x7F),
    /// увеличенной в `scale` раз. Рисуются только установленные биты (фон
    /// прозрачный).
    #[allow(dead_code)] // удобный API для будущих нативных экранов
    pub fn draw_text_zx(&mut self, x: i32, y: i32, s: &str, color: u32, scale: usize) {
        self.draw_text(crate::font::ZX_FONT, x, y, s, color, scale);
    }

    /// Нарисовать строку заданным шрифтом (8x8, коды 0x20..0x7F), увеличенным
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
    pub fn composite_onto(
        &self,
        out: &mut [u32],
        out_w: usize,
        out_h: usize,
        ox: usize,
        oy: usize,
    ) {
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
