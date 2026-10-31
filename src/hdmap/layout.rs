//! Геометрия окна и раскладка поля галактики на карте.

use super::*;

/// Фон карты.
pub(super) const BG: u32 = 0xFF00_0008;
/// Фон поля галактики.
pub(super) const FIELD_BG: u32 = 0xFF00_1018;
/// Рамка поля.
pub(super) const BORDER: u32 = 0xFF30_6070;
/// Радиус планеты: доля игрового радиуса (`3..5`) на единицу масштаба поля.
pub(super) const PLANET_RADIUS: f32 = 0.16;
/// Кольцо вокруг текущей системы (ЛАВЕ и т.п.).
pub(super) const SYS_RING: u32 = 0xFFFF_FF80;
/// Кольцо вокруг планеты, ближайшей к курсору.
pub(super) const NEAR_RING: u32 = 0xFF40_FFC0;
/// Перекрестие курсора.
pub(super) const CURSOR: u32 = 0xFF00_FF00;
/// Круг дальности прыжка.
pub(super) const RANGE: u32 = 0xFF60_A0C0;
/// Текст.
pub(super) const TEXT: u32 = 0xFFE0_E8F0;
/// Приглушённый текст.
pub(super) const TEXT_DIM: u32 = 0xFF80_9098;

/// Минимальный зум — вся галактика.
pub(super) const ZOOM_MIN: f32 = 1.0;
/// Шаг зума (множитель за одну ступень колеса).
pub(super) const ZOOM_STEP: f32 = 1.25;
/// Полуразмер видимой области поля на максимальном зуме (в единицах поля).
/// Взят так, чтобы на максимуме помещался круг дальности прыжка на полный бак
/// (7.0 св. года ~ 17.5 единиц) — это соответствует детальной карте по `O`.
pub(super) const DETAIL_HALF_UNITS: f32 = 20.0;

/// Радиус кружка планеты в пикселях при масштабе поля `scale`.
pub(super) fn planet_px(p: &Planet, scale: f32) -> i32 {
    ((p.radius() as f32 * scale * PLANET_RADIUS).round() as i32).max(1)
}

/// Геометрия окна, не зависящая от зума и положения камеры.
#[derive(Clone, Copy)]
pub(super) struct View {
    pub(super) line: i32,
    pub(super) top: i32,
    pub(super) panel_y: i32,
    pub(super) vx0: i32,
    pub(super) vy0: i32,
    pub(super) vw: i32,
    pub(super) vh: i32,
    /// Масштаб «вся галактика» (зум = 1).
    pub(super) base: f32,
    /// Центр области показа в пикселях окна.
    pub(super) vcx: f32,
    pub(super) vcy: f32,
    pub(super) text: usize,
}

/// Сосчитать геометрию окна `w` x `h`.
pub(super) fn view(w: u32, h: u32) -> View {
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    // Полоса под заголовок: строка текста плюс отступы сверху и снизу,
    // чтобы заголовок не налезал на поле (важно при крупном шрифте —
    // на 16:9/16:10 окнах `text` = 3 и больше).
    let top = 12 * text as i32 + 8;
    let panel_h = line * 3 + 8 * text as i32;
    let panel_y = h as i32 - panel_h;
    let vx0 = 0;
    let vy0 = top;
    let vw = w as i32;
    let vh = (panel_y - top).max(1);
    let base = (w as f32 / 256.0)
        .min(vh as f32 / FIELD_HEIGHT as f32)
        .max(1.0);
    View {
        line,
        top,
        panel_y,
        vx0,
        vy0,
        vw,
        vh,
        base,
        vcx: vw as f32 / 2.0,
        vcy: vy0 as f32 + vh as f32 / 2.0,
        text,
    }
}

/// Ограничить центр камеры так, чтобы поле не отрывалось от краёв окна
/// (когда поле меньше окна — оно просто по центру).
pub(super) fn clamp_center(v: &View, scale: f32, cx: f32, cr: f32) -> (f32, f32) {
    let half_w = (v.vw as f32 / (2.0 * scale)).min(128.0);
    let half_h = (v.vh as f32 / (2.0 * scale)).min(64.0);
    (
        cx.clamp(half_w, 256.0 - half_w),
        cr.clamp(half_h, 128.0 - half_h),
    )
}

/// Максимальный зум для окна: масштаб, при котором меньшая сторона области
/// показа покрывает [`DETAIL_HALF_UNITS`] единиц поля с каждой стороны от
/// текущей системы. Так на максимуме виден полный радиус прыжка (7 св. лет),
/// что соответствует детальной карте по `O`.
pub(super) fn max_zoom(v: &View) -> f32 {
    let target = v.vw.min(v.vh) as f32 / (2.0 * DETAIL_HALF_UNITS);
    (target / v.base).max(ZOOM_MIN)
}

/// Зажать `v` в `lo..=hi`. Если диапазон пуст (`lo > hi`), вернуть `v`
/// без изменений (такое случается в очень маленьком окне).
pub(super) fn clamp_inside(v: i32, lo: i32, hi: i32) -> i32 {
    if lo > hi { v } else { v.clamp(lo, hi) }
}

/// Раскладка поля галактики `256x128` в окне (с учётом зума).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapLayout {
    /// Пикселей на единицу поля (одинаково по обеим осям).
    pub scale: f32,
    /// Прямоугольник всего поля в пикселях окна (может выходить за окно).
    pub fx0: i32,
    pub fy0: i32,
    pub fw: i32,
    pub fh: i32,
    /// Область показа (viewport) — сюда обрезается рисование поля.
    pub vx0: i32,
    pub vy0: i32,
    pub vw: i32,
    pub vh: i32,
    /// Масштаб шрифта интерфейса (заголовок и панель).
    pub text: usize,
    /// Верхняя граница нижней панели.
    pub panel_y: i32,
    /// Высота верхней полосы (под заголовок).
    pub top: i32,
    /// Верх заголовка (по вертикали он центрируется в верхней полосе).
    pub title_y: i32,
}

impl MapLayout {
    /// Пиксель центра звезды на экранной строке `row` (`0..127`).
    pub fn star_px(&self, x: i32, row: i32) -> (i32, i32) {
        (
            (self.fx0 as f32 + x as f32 * self.scale).round() as i32,
            (self.fy0 as f32 + row as f32 * self.scale).round() as i32,
        )
    }

    /// Обратное преобразование пикселя окна в координаты поля `(x, строка)`.
    pub fn field_at(&self, win_x: f64, win_y: f64) -> (f64, f64) {
        (
            (win_x - self.fx0 as f64) / self.scale as f64,
            (win_y - self.fy0 as f64) / self.scale as f64,
        )
    }

    /// Видимая часть поля: пересечение поля и области показа `(x0, y0, x1, y1)`.
    pub fn visible_rect(&self) -> (i32, i32, i32, i32) {
        (
            self.fx0.max(self.vx0),
            self.fy0.max(self.vy0),
            (self.fx0 + self.fw).min(self.vx0 + self.vw),
            (self.fy0 + self.fh).min(self.vy0 + self.vh),
        )
    }
}
