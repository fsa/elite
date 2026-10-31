//! Тесты HD-слоя: раскладка, масштабирование, отсечение и текст.

use super::*;

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
