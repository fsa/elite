//! Отрисовка переходов: тоннель станции и тоннель гиперпрыжка.

use crate::display::Frame;
use crate::render3d::{Rotation, Screen};

use super::super::palette::{BG, STATION_RING_COLOR, TRANSITION_TEXT, WARP_RING_COLOR};
use super::super::stars::Star;
use super::super::view;
use super::super::Flight;
use super::scene::draw_disc_with_rim;
use super::text::draw_centered_text;

/// Время, за которое центральная окружность вырастает до конечного размера.
/// Примерно столько же длится ролик вылета/стыковки.
const STATION_GROW_TIME: f32 = 10.0;
/// Начальный радиус центральной окружности (доля меньшей стороны вида).
const STATION_CORE_START: f32 = 0.01;
/// Конечный радиус центральной окружности.
///
/// Совпадает с проекцией планеты сразу после вылета: `PLANET_RADIUS` 42000
/// при `z = 0x018000` и фокусе `1.15 * высота` даёт около половины высоты вида.
const STATION_CORE_END: f32 = 0.50;
/// За сколько секунд кольцо проходит по глубине путь от края центра до камеры.
/// За край вида оно уходит немного раньше.
const STATION_RING_PERIOD: f32 = 3.0;
/// Сколько белых колец рисуется в тоннеле станции.
const STATION_RING_COUNT: usize = 16;

/// Тоннель станции: растущая центральная окружность и расходящиеся от неё
/// белые кольца.
///
/// При вылете в центре тоннеля видна планета — та самая, к которой ведёт
/// труба; её заливка остаётся планетной, а ободок белый, как у колец. При
/// стыковке планеты нет, внутри окружности остаётся фон.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_station(
    frame: &mut Frame,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    s: i32,
    elapsed: f32,
    launch: bool,
    planet_color: u32,
) {
    if vw <= 0 || vh <= 0 {
        return;
    }
    let cx = vx + vw / 2;
    let cy = vy + vh / 2;
    let min_dim = vw.min(vh) as f32;
    let max_r = ((vw * vw + vh * vh) as f32).sqrt() * 0.5;
    let core_r = station_core_radius(min_dim, elapsed);
    draw_station_rings(frame, cx, cy, max_r, core_r, elapsed, s);
    // Центральная окружность растёт весь ролик; кольца выходят из её края.
    let fill = if launch { planet_color } else { BG };
    draw_disc_with_rim(
        frame,
        cx as f32,
        cy as f32,
        core_r,
        fill,
        STATION_RING_COLOR,
        (s / 2).max(1),
    );
}

/// Радиус центральной окружности на момент `elapsed`.
///
/// Растёт линейно от [`STATION_CORE_START`] до [`STATION_CORE_END`] за
/// [`STATION_GROW_TIME`] секунд, то есть практически весь ролик.
fn station_core_radius(min_dim: f32, elapsed: f32) -> f32 {
    let start = (min_dim * STATION_CORE_START).max(4.0);
    let end = (min_dim * STATION_CORE_END).max(start);
    let t = (elapsed / STATION_GROW_TIME).clamp(0.0, 1.0);
    start + (end - start) * t
}

/// Тоннель гиперпрыжка/стыковки: полосы звёзд и расходящиеся кольца.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_warp(
    f: &Flight,
    frame: &mut Frame,
    font: &[u8],
    screen: &Screen,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    s: i32,
    elapsed: f32,
    title: Option<&str>,
) {
    let view_rot = view::rotation(f.view_dir);
    draw_star_streaks(
        frame,
        screen,
        &f.stars.stars,
        &view_rot,
        vx,
        vy,
        vw,
        vh,
        300.0 + elapsed * 1400.0,
    );
    draw_tunnel_rings(
        frame,
        vx,
        vy,
        vw,
        vh,
        (elapsed * 1.7).fract(),
        6,
        WARP_RING_COLOR,
    );
    if let Some(title) = title {
        let scale = (s * 2).max(1) as usize;
        draw_centered_text(
            frame,
            font,
            vx,
            vy + 4 * s,
            vw,
            title,
            TRANSITION_TEXT,
            scale,
        );
    }
}

/// Полосы звёзд, летящих на камеру.
///
/// Для каждой звезды строим отрезок из её чуть более далёкого положения в
/// текущее: в перспективе он расходится от центра, как след разгона.
#[allow(clippy::too_many_arguments)]
fn draw_star_streaks(
    frame: &mut Frame,
    screen: &Screen,
    stars: &[Star],
    view_rot: &Rotation,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    streak: f32,
) {
    if vw <= 0 || vh <= 0 {
        return;
    }
    let cam = view_rot.transpose();
    let (xmin, ymin) = (vx as f32, vy as f32);
    let (xmax, ymax) = ((vx + vw - 1) as f32, (vy + vh - 1) as f32);
    for star in stars {
        let v = cam.apply(star.pos);
        if v[2] <= view::NEAR {
            continue;
        }
        let near = v[2];
        let far = near + streak.max(1.0);
        let x1 = screen.cx + v[0] / near * screen.k;
        let y1 = screen.cy - v[1] / near * screen.k;
        let x0 = screen.cx + v[0] / far * screen.k;
        let y0 = screen.cy - v[1] / far * screen.k;
        let Some((ax, ay, bx, by)) = view::clip_segment_2d(x0, y0, x1, y1, xmin, ymin, xmax, ymax)
        else {
            continue;
        };
        let bright = 0.35 + star.mag as f32 / 255.0 * 0.65;
        let c = 0xFF00_0000
            | ((bright * 210.0) as u32) << 16
            | ((bright * 235.0) as u32) << 8
            | (bright * 255.0) as u32;
        frame.line(ax, ay, bx, by, c);
    }
}

/// Расходящиеся кольца тоннеля. `phase` — 0..1, сдвиг между кольцами.
#[allow(clippy::too_many_arguments)]
fn draw_tunnel_rings(
    frame: &mut Frame,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    phase: f32,
    count: usize,
    color: u32,
) {
    if vw <= 0 || vh <= 0 || count == 0 {
        return;
    }
    let cx = vx + vw / 2;
    let cy = vy + vh / 2;
    let max_r = ((vw * vw + vh * vh) as f32).sqrt() * 0.5;
    for i in 0..count {
        let t = (i as f32 / count as f32 + phase).fract();
        // В перспективе кольцо ускоряется по мере приближения к камере.
        let r = (t * t * max_r).round() as i32;
        if r > 1 {
            frame.circle(cx, cy, r, color);
        }
    }
}

/// Белые кольца тоннеля станции.
///
/// Тоннель — это набор сечений трубы на разной глубине. Чем ближе сечение к
/// камере, тем больше его видимый радиус, поэтому радиусы колец подчиняются
/// перспективе: `r = core_r / d`, где `d` — оставшаяся доля пути от камеры
/// до планеты (1 — у края планеты, 0 — у камеры). Глубина `d` у каждого кольца
/// убывает линейно, так что к центру кольца стоят часто, а к краю вида —
/// редко и летят быстро: получается полёт сквозь трубу, а не равномерное
/// расхождение окружностей.
///
/// Новое кольцо входит в кадр ровно у края растущей центральной окружности
/// (`d = 1`) и уходит за край вида, когда `d` опускается до `core_r / max_r`.
#[allow(clippy::too_many_arguments)]
fn draw_station_rings(
    frame: &mut Frame,
    cx: i32,
    cy: i32,
    max_r: f32,
    core_r: f32,
    elapsed: f32,
    s: i32,
) {
    let thickness = (s / 2).max(1);
    for r in station_ring_radii(core_r, max_r, elapsed) {
        for k in 0..thickness {
            if r - k > 1 {
                frame.circle(cx, cy, r - k, STATION_RING_COLOR);
            }
        }
    }
}

/// Радиусы видимых колец тоннеля на момент `elapsed`, от центра к краю вида.
fn station_ring_radii(core_r: f32, max_r: f32, elapsed: f32) -> Vec<i32> {
    let mut radii = Vec::with_capacity(STATION_RING_COUNT);
    if !(core_r > 0.5) || !(max_r > core_r) {
        return radii;
    }
    // Доля пути до планеты, на которой кольцо уходит за край вида.
    let d_min = core_r / max_r;
    let phase = (elapsed / STATION_RING_PERIOD).fract();
    for i in 0..STATION_RING_COUNT {
        // 0 — кольцо только что родилось у края центра, 1 — ушло за край.
        let t = (i as f32 / STATION_RING_COUNT as f32 + phase).fract();
        let d = 1.0 - t * (1.0 - d_min);
        let r = core_r / d;
        if r.is_finite() && r <= max_r {
            let r = r.round() as i32;
            // Кольцо ровно на ободке центра только утолщало бы его.
            if (r as f32) > core_r {
                radii.push(r);
            }
        }
    }
    radii.sort_unstable();
    radii
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLANET: u32 = 0xFF00_2000;

    fn pixel(frame: &Frame, x: i32, y: i32) -> u32 {
        frame.pixels()[y as usize * frame.width + x as usize]
    }

    #[test]
    fn core_grows_from_tiny_to_planet_size() {
        let start = station_core_radius(400.0, 0.0);
        let mid = station_core_radius(400.0, STATION_GROW_TIME / 2.0);
        let end = station_core_radius(400.0, STATION_GROW_TIME);
        assert!(
            start < mid && mid < end,
            "центральная окружность не растёт: {start} {mid} {end}"
        );
        assert!(start < 0.02 * 400.0, "центр стартует не с крошечной окружности");
        assert!((end - 400.0 * STATION_CORE_END).abs() < 0.5);
        // После окончания ролика радиус не продолжает расти.
        assert_eq!(
            station_core_radius(400.0, STATION_GROW_TIME * 2.0),
            end
        );
    }

    #[test]
    fn rings_reach_the_screen_edges() {
        let (w, h) = (400usize, 400usize);
        let mut frame = Frame::new(w, h);
        // В конце ролика центр уже большой: кольца должны заполнять оставшуюся
        // площадь вплоть до краёв, а не обрываться на полпути.
        draw_station(
            &mut frame,
            0,
            0,
            w as i32,
            h as i32,
            1,
            STATION_GROW_TIME,
            true,
            PLANET,
        );
        let white_at = |x: usize, y: usize| pixel(&frame, x as i32, y as i32) == STATION_RING_COLOR;
        for y in 0..5 {
            assert!((0..w).any(|x| white_at(x, y)), "верх вида без колец");
            assert!((0..w).any(|x| white_at(x, h - 1 - y)), "низ вида без колец");
        }
        for x in 0..5 {
            assert!((0..h).any(|y| white_at(x, y)), "левый край вида без колец");
            assert!((0..h).any(|y| white_at(w - 1 - x, y)), "правый край вида без колец");
        }
    }

    #[test]
    fn rings_spread_out_toward_the_edge() {
        let core = station_core_radius(400.0, STATION_GROW_TIME);
        let max_r = ((400.0f32 * 400.0 + 400.0 * 400.0).sqrt()) * 0.5;
        let radii = station_ring_radii(core, max_r, STATION_GROW_TIME);
        assert!(
            radii.len() > 8,
            "слишком мало колец в тоннеле: {}",
            radii.len()
        );
        assert!(radii[0] > core as i32, "кольцо залезло внутрь центра");
        assert!(
            *radii.last().unwrap() as f32 > 0.95 * max_r,
            "кольца обрываются до края вида"
        );
        // В перспективе кольца редеют к краю: промежутки растут наружу.
        let gaps: Vec<i32> = radii.windows(2).map(|w| w[1] - w[0]).collect();
        assert!(
            gaps.last() > gaps.first(),
            "кольца расходятся равномерно, без перспективы: {gaps:?}"
        );
    }

    #[test]
    fn launch_core_has_planet_fill_and_white_rim() {
        let mut frame = Frame::new(200, 200);
        draw_station(&mut frame, 0, 0, 200, 200, 1, 0.0, true, PLANET);
        let core = station_core_radius(200.0, 0.0);
        let edge = core.round() as i32;
        assert_eq!(pixel(&frame, 100, 100), PLANET, "центр не залит планетой");
        assert_eq!(
            pixel(&frame, 100, 100 - (edge - 1)),
            STATION_RING_COLOR,
            "ободок центральной окружности не белый"
        );
        // Внутри центра нет колец: они появляются только у его края.
        for y in 0..200 {
            for x in 0..200 {
                let dx = (x - 100) as f32;
                let dy = (y - 100) as f32;
                if (dx * dx + dy * dy).sqrt() + 0.5 < core {
                    let p = pixel(&frame, x, y);
                    assert!(
                        p == PLANET || p == STATION_RING_COLOR,
                        "внутри центра ({x},{y}) лишний цвет {p:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn docking_core_stays_on_background() {
        let mut frame = Frame::new(200, 200);
        draw_station(&mut frame, 0, 0, 200, 200, 1, 0.0, false, PLANET);
        assert_eq!(pixel(&frame, 100, 100), BG, "при стыковке в центре планета");
    }
}
