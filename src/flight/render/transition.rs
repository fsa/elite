//! Отрисовка переходов: тоннель станции и тоннель гиперпрыжка.

use crate::display::Frame;
use crate::render3d::{Rotation, Screen};

use super::super::palette::{BG, STATION_RING_COLOR, TRANSITION_TEXT, WARP_RING_COLOR};
use super::super::stars::Star;
use super::super::view;
use super::super::Flight;
use super::scene::draw_planet;
use super::text::draw_centered_text;

/// Радиус планеты на экране, доля меньшей стороны вида.
///
/// Совпадает с проекцией планеты сразу после вылета: `PLANET_RADIUS` 42000
/// при `z = 0x018000` и фокусе `1.15 * высота` даёт около половины высоты вида.
/// Планета нарисована всё время ролика в том же месте и размере, поэтому при
/// переходе к обычному полёту она не прыгает.
const STATION_PLANET_RADIUS: f32 = 0.50;
/// Начальный радиус выходного отверстия трубы, доля радиуса планеты.
///
/// Сначала стенка трубы закрывает внешнюю часть планеты — виден только её
/// центр. К концу ролика отверстие дорастает до радиуса планеты и открывает
/// её целиком.
const STATION_APERTURE_START: f32 = 0.5;
/// За сколько секунд отверстие трубы дорастает до размеров планеты.
///
/// К этому же моменту последнее кольцо улетает за экран: труба заканчивается,
/// и остаётся только планета (при вылете) или чёрный фон (при стыковке).
const STATION_APERTURE_TIME: f32 = 9.0;
/// За сколько секунд кольцо трубы пролетает путь до края вида.
const STATION_RING_PERIOD: f32 = 2.0;
/// Когда кольца перестают рождаться.
const STATION_RING_STOP: f32 = 7.5;
/// Сколько белых колец рисуется в тоннеле станции.
const STATION_RING_COUNT: usize = 16;

/// Тоннель станции: неподвижная планета за выходным отверстием трубы.
///
/// Планета нарисована сразу в том же месте и размере, как в обычном полёте,
/// но её закрывает непрозрачная стенка трубы: видно только то, что попадает
/// в растущее выходное отверстие. Кольца-рёбра рождаются на кромке отверстия
/// и разлетаются по стенке трубы за экран. К концу ролика отверстие дорастает
/// до планеты, кольца улетают, и остаётся только планета либо чёрный фон при
/// стыковке.
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
    let planet_r = (min_dim * STATION_PLANET_RADIUS).max(4.0);
    let thickness = (s / 2).max(1);
    if launch {
        draw_planet(frame, cx as f32, cy as f32, planet_r, planet_color, s);
        // Стенка трубы непрозрачна: всё вне отверстия закрываем фоном.
        let aperture = station_aperture_radius(planet_r, elapsed);
        mask_outside_aperture(frame, vx, vy, vw, vh, cx, cy, aperture);
        // Кольца живут на стенке трубы, то есть снаружи отверстия.
        draw_station_rings(frame, cx, cy, max_r, aperture, elapsed, s);
        // Кромка трубы: граница между планетой в отверстии и стенкой.
        // Когда отверстие открылось до размера планеты, её ободок уже
        // рисует сама планета, и лишнее белое кольцо не нужно.
        if aperture < planet_r - thickness as f32 {
            draw_thick_circle(frame, cx, cy, aperture, thickness, STATION_RING_COLOR);
        }
    } else {
        // При стыковке за трубой ничего нет, только чёрный фон: кольца
        // рождаются в центре и улетают за экран.
        draw_station_rings(frame, cx, cy, max_r, 0.0, elapsed, s);
    }
}

/// Радиус выходного отверстия трубы на момент `elapsed`.
///
/// Камера летит к выходу с постоянной скоростью, поэтому угловой радиус
/// отверстия растёт как `1 / (1 - t * k)`: в начале почти незаметно, а к концу
/// труба раскрывается всё быстрее — анимация не «замедляется» на финише.
fn station_aperture_radius(planet_r: f32, elapsed: f32) -> f32 {
    let end = planet_r.max(4.0);
    let start = (end * STATION_APERTURE_START).max(1.0);
    let t = (elapsed / STATION_APERTURE_TIME).clamp(0.0, 1.0);
    let k = 1.0 - start / end;
    start / (1.0 - t * k).max(f32::EPSILON)
}

/// Закрыть фоном всё, что не попадает в круг радиуса `r`.
fn mask_outside_aperture(
    frame: &mut Frame,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    cx: i32,
    cy: i32,
    r: f32,
) {
    let r2 = r.max(0.0) * r.max(0.0);
    let right = vx + vw;
    for y in vy..vy + vh {
        let dy = y as f32 - cy as f32;
        let d2 = r2 - dy * dy;
        if d2 <= 0.0 {
            frame.fill_rect(vx, y, vw as usize, 1, BG);
            continue;
        }
        let dx = d2.sqrt() as i32;
        let left_end = cx - dx;
        if left_end > vx {
            frame.fill_rect(vx, y, (left_end - vx) as usize, 1, BG);
        }
        let right_start = cx + dx + 1;
        if right_start < right {
            frame.fill_rect(right_start, y, (right - right_start) as usize, 1, BG);
        }
    }
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

/// Белые кольца трубы.
///
/// При вылете кольца рождаются у края планеты и разлетаются по перспективе:
/// `r = inner_r / d`, где `d` — оставшаяся доля пути до камеры. Поэтому у
/// планеты кольца идут часто и медленно, а у края вида редеют и ускоряются.
/// При стыковке планеты нет, и та же труба сходится к чёрной точке в центре.
#[allow(clippy::too_many_arguments)]
fn draw_station_rings(
    frame: &mut Frame,
    cx: i32,
    cy: i32,
    max_r: f32,
    inner_r: f32,
    elapsed: f32,
    s: i32,
) {
    let thickness = (s / 2).max(1);
    for r in station_ring_radii(inner_r, max_r, elapsed) {
        draw_thick_circle(frame, cx, cy, r as f32, thickness, STATION_RING_COLOR);
    }
}

/// Окружность толщиной `thickness` пикселей, растущая внутрь от `r`.
fn draw_thick_circle(frame: &mut Frame, cx: i32, cy: i32, r: f32, thickness: i32, color: u32) {
    let r = r.round() as i32;
    for k in 0..thickness.max(1) {
        if r - k > 1 {
            frame.circle(cx, cy, r - k, color);
        }
    }
}

/// Радиусы видимых колец на момент `elapsed`, от центра к краю вида.
fn station_ring_radii(inner_r: f32, max_r: f32, elapsed: f32) -> Vec<i32> {
    let mut radii = Vec::with_capacity(STATION_RING_COUNT);
    let phase = (elapsed / STATION_RING_PERIOD).fract();
    if inner_r > 0.5 && max_r > inner_r {
        // Доля пути до камеры, на которой кольцо уходит за край вида.
        let d_min = inner_r / max_r;
        for i in 0..STATION_RING_COUNT {
            // 0 — кольцо только что родилось у края планеты, 1 — у камеры.
            let t = (i as f32 / STATION_RING_COUNT as f32 + phase).fract();
            if !ring_is_alive(t, elapsed) {
                continue;
            }
            let d = 1.0 - t * (1.0 - d_min);
            let r = inner_r / d;
            if r.is_finite() && r <= max_r {
                let r = r.round() as i32;
                // Кольцо ровно на краю планеты только утолщало бы её ободок.
                if (r as f32) > inner_r {
                    radii.push(r);
                }
            }
        }
    } else {
        // Без планеты труба сходится в точку; к краю кольца ускоряются.
        for i in 0..STATION_RING_COUNT {
            let t = (i as f32 / STATION_RING_COUNT as f32 + phase).fract();
            if !ring_is_alive(t, elapsed) {
                continue;
            }
            let r = (t * t * max_r).round() as i32;
            if r > 1 {
                radii.push(r);
            }
        }
    }
    radii.sort_unstable();
    radii
}

/// Живо ли кольцо: после [`STATION_RING_STOP`] новые не рождаются, а старые
/// долетают и уходят за экран.
fn ring_is_alive(t: f32, elapsed: f32) -> bool {
    elapsed - t * STATION_RING_PERIOD <= STATION_RING_STOP
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flight::palette::{BG, planet_rim};

    const PLANET: u32 = 0xFF00_2000;

    fn pixel(frame: &Frame, x: i32, y: i32) -> u32 {
        frame.pixels()[y as usize * frame.width + x as usize]
    }

    fn frame_with_background(w: usize, h: usize) -> Frame {
        let mut frame = Frame::new(w, h);
        frame.fill_rect(0, 0, w, h, BG);
        frame
    }

    #[test]
    fn launch_starts_with_planet_hidden_behind_the_tube_wall() {
        let (w, h) = (200usize, 200usize);
        let planet_r = w as f32 * STATION_PLANET_RADIUS;
        let aperture = station_aperture_radius(planet_r, 0.0);
        // Сначала отверстие показывает только центр планеты.
        assert!(
            aperture < planet_r * 0.75,
            "труба слишком открыта в начале: {aperture}"
        );

        let mut frame = frame_with_background(w, h);
        draw_station(&mut frame, 0, 0, w as i32, h as i32, 1, 0.0, true, PLANET);
        assert_eq!(pixel(&frame, 100, 100), PLANET, "центр не залит планетой");
        // Стенка трубы непрозрачна: внешняя часть планеты закрыта.
        let rim = planet_rim(PLANET);
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let dx = (x - 100) as f32;
                let dy = (y - 100) as f32;
                let d = (dx * dx + dy * dy).sqrt();
                if d > aperture + 1.0 && d < planet_r - 1.0 {
                    let p = pixel(&frame, x, y);
                    assert!(
                        p != PLANET && p != rim,
                        "планета видна сквозь стенку трубы ({x},{y}): {p:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn aperture_opens_to_the_planet_by_the_end() {
        let planet_r = 100.0;
        let start = station_aperture_radius(planet_r, 0.0);
        assert!(
            (start - planet_r * STATION_APERTURE_START).abs() < 0.01,
            "неверное начальное отверстие: {start}"
        );
        let end = station_aperture_radius(planet_r, STATION_APERTURE_TIME);
        assert!(
            (end - planet_r).abs() < 0.01,
            "к концу отверстие не открылось до планеты: {end}"
        );
        // Раскрытие ускоряется к концу: последняя часть пути видна сильнее.
        let mid = station_aperture_radius(planet_r, STATION_APERTURE_TIME * 0.5);
        assert!(
            mid < (start + end) * 0.5,
            "отверстие раскрывается равномерно, нет ускорения: {mid}"
        );
    }

    #[test]
    fn launch_ends_with_whole_planet() {
        let (w, h) = (200usize, 200usize);
        let mut frame = frame_with_background(w, h);
        draw_station(
            &mut frame,
            0,
            0,
            w as i32,
            h as i32,
            1,
            STATION_APERTURE_TIME,
            true,
            PLANET,
        );
        assert_eq!(pixel(&frame, 100, 100), PLANET, "центр не залит планетой");
        // Планета уже того же размера, что и в обычном полёте: полвысоты вида.
        let edge = (w as f32 * STATION_PLANET_RADIUS).round() as i32;
        assert_eq!(
            pixel(&frame, 100, 100 - (edge - 1)),
            planet_rim(PLANET),
            "планета не того размера или ободок не планетный"
        );
    }

    #[test]
    fn rings_reach_the_screen_edges() {
        let (w, h) = (400usize, 400usize);
        let mut frame = frame_with_background(w, h);
        // Планета неподвижна, поэтому кольца заполняют всю оставшуюся площадь
        // уже с первого кадра ролика.
        draw_station(&mut frame, 0, 0, w as i32, h as i32, 1, 0.0, true, PLANET);
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
        let planet_r = 400.0 * STATION_PLANET_RADIUS;
        let inner = station_aperture_radius(planet_r, 0.0);
        let max_r = ((400.0f32 * 400.0 + 400.0 * 400.0).sqrt()) * 0.5;
        let radii = station_ring_radii(inner, max_r, 0.0);
        assert!(
            radii.len() > 8,
            "слишком мало колец в трубе: {}",
            radii.len()
        );
        assert!(radii[0] > inner as i32, "кольцо залезло внутрь отверстия");
        // В перспективе кольца редеют к краю: промежутки растут наружу.
        let gaps: Vec<i32> = radii.windows(2).map(|w| w[1] - w[0]).collect();
        assert!(
            gaps.last() > gaps.first(),
            "кольца расходятся равномерно, без перспективы: {gaps:?}"
        );
        // Последнее кольцо пересекает край вида, а не обрывается внутри.
        assert!(
            *radii.last().unwrap() > 200,
            "кольца не доходят до края вида"
        );
    }

    #[test]
    fn launch_rings_fly_away_leaving_the_planet() {
        let (w, h) = (200usize, 200usize);
        let inner = w as f32 * STATION_PLANET_RADIUS;
        let max_r = ((w * w + h * h) as f32).sqrt() * 0.5;
        // К концу ролика последнее кольцо уже уходит за экран.
        assert!(
            !station_ring_radii(inner, max_r, STATION_RING_STOP + STATION_RING_PERIOD * 0.5)
                .is_empty(),
            "кольца исчезли слишком рано"
        );
        assert!(
            station_ring_radii(inner, max_r, STATION_RING_STOP + STATION_RING_PERIOD).is_empty(),
            "кольца не успевают улететь к концу ролика"
        );

        let mut frame = frame_with_background(w, h);
        let end = STATION_RING_STOP + STATION_RING_PERIOD;
        draw_station(&mut frame, 0, 0, w as i32, h as i32, 1, end, true, PLANET);
        assert_eq!(pixel(&frame, 100, 100), PLANET, "в конце не осталось планеты");
        assert!(
            frame.pixels().iter().all(|&p| p != STATION_RING_COLOR),
            "в конце ролика остались кольца"
        );
    }

    #[test]
    fn docking_ends_on_black_background() {
        let mut frame = frame_with_background(200, 200);
        draw_station(&mut frame, 0, 0, 200, 200, 1, 0.0, false, PLANET);
        assert_eq!(pixel(&frame, 100, 100), BG, "при стыковке в центре планета");

        let end = STATION_RING_STOP + STATION_RING_PERIOD;
        let mut frame = frame_with_background(200, 200);
        draw_station(&mut frame, 0, 0, 200, 200, 1, end, false, PLANET);
        assert_eq!(pixel(&frame, 100, 100), BG, "в конце стыковки в центре не фон");
        assert!(
            frame.pixels().iter().all(|&p| p == BG),
            "в конце стыковки остались кольца"
        );
    }
}
