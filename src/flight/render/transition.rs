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
/// Планета нарисована сразу в этом размере, но кольцо-выход открывает её
/// постепенно, поэтому при переходе к обычному полёту она не прыгает.
const STATION_PLANET_RADIUS: f32 = 0.50;
/// Во сколько раз выход трубы дальше камеры в начале ролика.
///
/// В начале выход — маленькая точка в центре вида, к концу камера подлетает
/// к нему вплотную: `r = planet_r / distance`.
const STATION_EXIT_DISTANCE_START: f32 = 7.0;
/// За сколько секунд камера долетает до выхода трубы.
///
/// К этому моменту кольцо-выход улетает за экран, и остаётся только планета
/// (при вылете) или чёрный фон (при стыковке).
const STATION_EXIT_TIME: f32 = 9.0;
/// За сколько секунд кольцо трубы пролетает путь до края вида.
const STATION_RING_PERIOD: f32 = 2.0;
/// Когда кольца перестают рождаться.
///
/// Кольца успевают улететь за экран раньше, чем кольцо-выход пройдёт камеру.
const STATION_RING_STOP: f32 = 7.0;
/// Сколько белых колец рисуется в тоннеле станции.
const STATION_RING_COUNT: usize = 6;

/// Тоннель станции: камера летит к выходу трубы.
///
/// Планета нарисована сразу в обычном размере, но стенка трубы закрывает всё
/// снаружи центрального кольца-выхода. Кольцо приближается и растёт, открывая
/// планету всё больше, а в конце пролетает мимо камеры и уходит за экран.
/// Кольца-рёбра на стенке трубы живут снаружи выходного кольца и по одному
/// улетают за экран. При стыковке за кольцом остаётся только чёрный фон.
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
    let gate_r = station_gate_radius(planet_r, elapsed);
    let ring_ratio = station_ring_ratio(planet_r, max_r);
    if launch {
        // Планета всегда обычного размера; видно только то, что попадает
        // в центральное кольцо-выход.
        draw_planet(frame, cx as f32, cy as f32, planet_r, planet_color, s);
        if let Some(gate) = gate_r.filter(|&r| r < planet_r) {
            mask_outside_aperture(frame, vx, vy, vw, vh, cx, cy, gate);
        }
    }
    // Кольца живут снаружи выхода: при стыковке внутри него остаётся
    // только чёрный фон.
    let inner_r = gate_r.unwrap_or(max_r * 2.0);
    draw_station_rings(frame, cx, cy, max_r, inner_r, ring_ratio, elapsed, s);
    // Белое кольцо-выход рисуется последним: в начале оно обводит выход,
    // а к концу труба заканчивается, и кольцо пролетает мимо камеры.
    if let Some(r) = gate_r.filter(|&r| r <= max_r) {
        draw_thick_circle(frame, cx, cy, r, (s / 2).max(1), STATION_RING_COLOR);
    }
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

/// Радиус белого кольца-выхода на момент `elapsed`.
///
/// Кольцо висит в конце трубы и в начале совпадает с выходом. Камера летит
/// сквозь него, поэтому расстояние до кольца падает до нуля, а радиус растёт
/// как `1 / distance`: в конце кольцо проносится мимо и уходит за экран.
/// `None` — кольцо уже за камерой.
fn station_gate_radius(planet_r: f32, elapsed: f32) -> Option<f32> {
    let t = (elapsed / STATION_EXIT_TIME).clamp(0.0, 1.0);
    let distance = STATION_EXIT_DISTANCE_START * (1.0 - t);
    if distance <= f32::EPSILON {
        return None;
    }
    Some((planet_r.max(4.0) / distance).max(1.0))
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

/// Во сколько раз растёт радиус кольца от выхода к краю вида.
///
/// Считается один раз по начальному положению выхода, чтобы кольца уходили
/// за экран, а не растягивались вместе с ним.
fn station_ring_ratio(planet_r: f32, max_r: f32) -> f32 {
    let start = (planet_r / STATION_EXIT_DISTANCE_START).max(1.0);
    let span = STATION_RING_COUNT as f32 - 0.5;
    // Чуть не доводим внешнее кольцо до угла вида, чтобы оно гарантированно
    // пересекало край и не отбрасывалось проверкой `r <= max_r`.
    (max_r * 0.98 / start)
        .max(1.0)
        .powf(STATION_RING_COUNT as f32 / span)
}

/// Белые кольца трубы.
///
/// Кольца равномерно распределены по стенке трубы от выхода до камеры
/// (геометрическая прогрессия радиусов). Камера летит вперёд, поэтому каждое
/// кольцо растёт и уходит за экран, а у выхода рождается новое. В начале
/// выход далеко и колец много, к концу ролика их становится всё меньше.
///
/// `inner_r` — радиус выхода: при вылете в нём планета, при стыковке —
/// чёрный фон.
#[allow(clippy::too_many_arguments)]
fn draw_station_rings(
    frame: &mut Frame,
    cx: i32,
    cy: i32,
    max_r: f32,
    inner_r: f32,
    ring_ratio: f32,
    elapsed: f32,
    s: i32,
) {
    let thickness = (s / 2).max(1);
    for r in station_ring_radii(inner_r, max_r, ring_ratio, elapsed) {
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
fn station_ring_radii(inner_r: f32, max_r: f32, ring_ratio: f32, elapsed: f32) -> Vec<i32> {
    if inner_r <= 1.0 || max_r <= inner_r + 1.0 {
        return Vec::new();
    }
    let phase = (elapsed / STATION_RING_PERIOD).fract();
    let mut radii = Vec::with_capacity(STATION_RING_COUNT);
    for i in 0..STATION_RING_COUNT {
        // 0 — кольцо только что родилось у выхода, 1 — у края вида.
        let t = ((i as f32 + 0.5) / STATION_RING_COUNT as f32 + phase).fract();
        if !ring_is_alive(t, elapsed) {
            continue;
        }
        let r = inner_r * ring_ratio.powf(t);
        if r.is_finite() && r <= max_r && r > inner_r + 0.5 {
            radii.push(r.round() as i32);
        }
    }
    radii.sort_unstable();
    radii.dedup();
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
    use crate::flight::palette::planet_rim;

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
    fn planet_is_hidden_outside_the_exit_ring() {
        let (w, h) = (200usize, 200usize);
        let planet_r = w as f32 * STATION_PLANET_RADIUS;
        let gate = station_gate_radius(planet_r, 0.0).unwrap();
        // В начале кольцо-выход далеко и маленькое.
        assert!(
            gate < planet_r * 0.2,
            "кольцо-выход слишком большое в начале: {gate}"
        );

        let mut frame = frame_with_background(w, h);
        draw_station(&mut frame, 0, 0, w as i32, h as i32, 1, 0.0, true, PLANET);
        assert_eq!(pixel(&frame, 100, 100), PLANET, "центр не залит планетой");
        // Планета уже обычного размера, но за кольцом её не видно.
        let rim = planet_rim(PLANET);
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let dx = (x - 100) as f32;
                let dy = (y - 100) as f32;
                let d = (dx * dx + dy * dy).sqrt();
                if d > gate + 1.0 && d < planet_r - 1.0 {
                    let p = pixel(&frame, x, y);
                    assert!(
                        p != PLANET && p != rim,
                        "планета видна за кольцом-выходом ({x},{y}): {p:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn white_exit_ring_starts_at_the_exit() {
        let (w, h) = (200usize, 200usize);
        let planet_r = w as f32 * STATION_PLANET_RADIUS;
        let start = station_gate_radius(planet_r, 0.0).unwrap();
        assert!(
            (start - planet_r / STATION_EXIT_DISTANCE_START).abs() < 0.01,
            "кольцо-выход не совпадает с выходом: {start}"
        );

        let mut frame = frame_with_background(w, h);
        draw_station(&mut frame, 0, 0, w as i32, h as i32, 1, 0.0, true, PLANET);
        let r = start.round() as i32;
        assert_eq!(
            pixel(&frame, 100, 100 - r),
            STATION_RING_COLOR,
            "в начале нет белого кольца-выхода"
        );
    }

    #[test]
    fn white_exit_ring_flies_past_by_the_end() {
        let planet_r = 100.0;
        let max_r = 200.0;
        let late = station_gate_radius(planet_r, STATION_EXIT_TIME * 0.95).unwrap();
        assert!(
            late > planet_r,
            "кольцо-выход не обогнало планету: {late}"
        );
        assert!(
            late > max_r,
            "кольцо-выход не улетело за экран к концу: {late}"
        );
        assert!(
            station_gate_radius(planet_r, STATION_EXIT_TIME).is_none(),
            "кольцо-выход осталось перед камерой в конце"
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
            STATION_EXIT_TIME,
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
        let inner = station_gate_radius(planet_r, 0.0).unwrap();
        let max_r = ((400.0f32 * 400.0 + 400.0 * 400.0).sqrt()) * 0.5;
        let ratio = station_ring_ratio(planet_r, max_r);
        let radii = station_ring_radii(inner, max_r, ratio, 0.0);
        assert!(radii.len() > 4, "слишком мало колец в трубе: {}", radii.len());
        assert!(radii[0] > inner as i32, "кольцо залезло внутрь выхода");
        // Промежутки растут наружу — кольца идут по стенке в перспективе.
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
        let planet_r = w as f32 * STATION_PLANET_RADIUS;
        let max_r = ((w * w + h * h) as f32).sqrt() * 0.5;
        let ratio = station_ring_ratio(planet_r, max_r);
        // К концу ролика последнее кольцо уже уходит за экран.
        let mid = STATION_RING_STOP;
        let inner = station_gate_radius(planet_r, mid).unwrap();
        assert!(
            !station_ring_radii(inner, max_r, ratio, mid).is_empty(),
            "кольца исчезли слишком рано"
        );
        let end = STATION_RING_STOP + STATION_RING_PERIOD;
        let inner = station_gate_radius(planet_r, end).unwrap_or(max_r * 2.0);
        assert!(
            station_ring_radii(inner, max_r, ratio, end).is_empty(),
            "кольца не успевают улететь к концу ролика"
        );

        let mut frame = frame_with_background(w, h);
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
