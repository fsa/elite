//! Отрисовка переходов: тоннель станции и тоннель гиперпрыжка.

use crate::display::Frame;

use super::super::palette::{BG, STATION_RING_COLOR, WARP_RING_COLOR};
use super::scene::draw_planet;

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

/// Насколько далеко от камеры выход трубы гиперпрыжка в начале ролика.
///
/// Вдвое дальше, чем у станции: начальное кольцо получается примерно вдвое
/// меньше по диаметру, и игровое пространство открывается постепенно.
const HYPERSPACE_EXIT_DISTANCE_START: f32 = 14.0;
/// Сколько длится тоннель гиперпрыжка.
///
/// Оригинал ведёт трубу счётчиком `byte_93D2` (0x42 = 66 шагов), но шаг
/// приходится примерно на 7.5 кадров основного цикла, поэтому весь переход
/// занимает около 500 кадров (~10 с). Ролик должен тянуться до самого конца
/// перехода, иначе труба заканчивается задолго до смены системы.
const HYPERSPACE_EXIT_TIME: f32 = 10.0;
/// За сколько секунд кольцо гиперпрыжка пролетает путь до края вида.
const HYPERSPACE_RING_PERIOD: f32 = 0.6;
/// Когда в тоннеле гиперпрыжка перестают рождаться кольца.
///
/// Кольца идут весь ролик и пропадают только когда выходное кольцо уходит
/// за край вида, открывая сцену новой системы.
const HYPERSPACE_RING_STOP: f32 = HYPERSPACE_EXIT_TIME;
/// Сколько колец рисуется в тоннеле гиперпрыжка.
const HYPERSPACE_RING_COUNT: usize = 8;

/// Геометрия трубы перехода: кольца на стенке и центральный выход.
struct Tunnel {
    /// Сколько колец рисуется на стенке.
    ring_count: usize,
    /// За сколько секунд кольцо пролетает путь до края вида.
    ring_period: f32,
    /// Когда кольца перестают рождаться.
    ring_stop: f32,
    /// Во сколько раз выход дальше камеры в начале ролика.
    distance_start: f32,
    /// За сколько секунд камера долетает до выхода.
    time: f32,
    /// Цвет колец трубы.
    color: u32,
}

const STATION_TUNNEL: Tunnel = Tunnel {
    ring_count: STATION_RING_COUNT,
    ring_period: STATION_RING_PERIOD,
    ring_stop: STATION_RING_STOP,
    distance_start: STATION_EXIT_DISTANCE_START,
    time: STATION_EXIT_TIME,
    color: STATION_RING_COLOR,
};

const HYPERSPACE_TUNNEL: Tunnel = Tunnel {
    ring_count: HYPERSPACE_RING_COUNT,
    ring_period: HYPERSPACE_RING_PERIOD,
    ring_stop: HYPERSPACE_RING_STOP,
    distance_start: HYPERSPACE_EXIT_DISTANCE_START,
    time: HYPERSPACE_EXIT_TIME,
    color: WARP_RING_COLOR,
};

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
    tunnel_gate_radius(planet_r, &STATION_TUNNEL, elapsed)
}

/// Радиус выхода трубы на момент `elapsed` для любой трубы.
fn tunnel_gate_radius(base_r: f32, tunnel: &Tunnel, elapsed: f32) -> Option<f32> {
    let t = (elapsed / tunnel.time).clamp(0.0, 1.0);
    let distance = tunnel.distance_start * (1.0 - t);
    if distance <= f32::EPSILON {
        return None;
    }
    Some((base_r.max(4.0) / distance).max(1.0))
}

/// Тоннель гиперпрыжка: игровая сцена видна только в центральном кольце.
///
/// Сцена уже нарисована вызывающей стороной. Всё снаружи выходного кольца
/// закрывается фоном, а на стенке трубы рисуются кольца. В начале кольцо
/// маленькое, поэтому виден лишь узкий тоннель; к концу оно проносится мимо
/// камеры и открывает всю сцену.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_hyperspace(
    frame: &mut Frame,
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    s: i32,
    elapsed: f32,
) {
    if vw <= 0 || vh <= 0 {
        return;
    }
    let cx = vx + vw / 2;
    let cy = vy + vh / 2;
    let min_dim = vw.min(vh) as f32;
    let max_r = ((vw * vw + vh * vh) as f32).sqrt() * 0.5;
    let base_r = (min_dim * STATION_PLANET_RADIUS).max(4.0);
    let gate_r = tunnel_gate_radius(base_r, &HYPERSPACE_TUNNEL, elapsed);
    let ring_ratio = tunnel_ring_ratio(base_r, max_r, &HYPERSPACE_TUNNEL);
    if let Some(gate) = gate_r {
        mask_outside_aperture(frame, vx, vy, vw, vh, cx, cy, gate);
    }
    let inner_r = gate_r.unwrap_or(max_r * 2.0);
    draw_tunnel_rings(
        frame,
        cx,
        cy,
        max_r,
        inner_r,
        ring_ratio,
        elapsed,
        s,
        &HYPERSPACE_TUNNEL,
    );
    if let Some(r) = gate_r.filter(|&r| r <= max_r) {
        draw_thick_circle(frame, cx, cy, r, (s / 2).max(1), WARP_RING_COLOR);
    }
}

/// Во сколько раз растёт радиус кольца от выхода к краю вида.
///
/// Считается один раз по начальному положению выхода, чтобы кольца уходили
/// за экран, а не растягивались вместе с ним.
fn station_ring_ratio(planet_r: f32, max_r: f32) -> f32 {
    tunnel_ring_ratio(planet_r, max_r, &STATION_TUNNEL)
}

fn tunnel_ring_ratio(base_r: f32, max_r: f32, tunnel: &Tunnel) -> f32 {
    if tunnel.ring_count == 0 {
        return 1.0;
    }
    let start = (base_r / tunnel.distance_start).max(1.0);
    let span = tunnel.ring_count as f32 - 0.5;
    // Чуть не доводим внешнее кольцо до угла вида, чтобы оно гарантированно
    // пересекало край и не отбрасывалось проверкой `r <= max_r`.
    (max_r * 0.98 / start)
        .max(1.0)
        .powf(tunnel.ring_count as f32 / span)
}

/// Белые кольца трубы станции.
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
    draw_tunnel_rings(
        frame,
        cx,
        cy,
        max_r,
        inner_r,
        ring_ratio,
        elapsed,
        s,
        &STATION_TUNNEL,
    );
}

/// Кольца на стенке трубы: от выхода до камеры.
///
/// Кольца равномерно распределены по стенке (геометрическая прогрессия
/// радиусов). Камера летит вперёд, поэтому каждое кольцо растёт и уходит за
/// экран, а у выхода рождается новое. В начале выход далеко и колец много,
/// к концу ролика их становится всё меньше.
#[allow(clippy::too_many_arguments)]
fn draw_tunnel_rings(
    frame: &mut Frame,
    cx: i32,
    cy: i32,
    max_r: f32,
    inner_r: f32,
    ring_ratio: f32,
    elapsed: f32,
    s: i32,
    tunnel: &Tunnel,
) {
    let thickness = (s / 2).max(1);
    for r in tunnel_ring_radii(inner_r, max_r, ring_ratio, elapsed, tunnel) {
        draw_thick_circle(frame, cx, cy, r as f32, thickness, tunnel.color);
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

/// Радиусы видимых колец станции на момент `elapsed`.
#[cfg(test)]
fn station_ring_radii(inner_r: f32, max_r: f32, ring_ratio: f32, elapsed: f32) -> Vec<i32> {
    tunnel_ring_radii(inner_r, max_r, ring_ratio, elapsed, &STATION_TUNNEL)
}

/// Радиусы видимых колец на момент `elapsed`, от центра к краю вида.
fn tunnel_ring_radii(
    inner_r: f32,
    max_r: f32,
    ring_ratio: f32,
    elapsed: f32,
    tunnel: &Tunnel,
) -> Vec<i32> {
    if inner_r <= 1.0 || max_r <= inner_r + 1.0 || tunnel.ring_count == 0 {
        return Vec::new();
    }
    let phase = (elapsed / tunnel.ring_period).fract();
    let mut radii = Vec::with_capacity(tunnel.ring_count);
    for i in 0..tunnel.ring_count {
        // 0 — кольцо только что родилось у выхода, 1 — у края вида.
        let t = ((i as f32 + 0.5) / tunnel.ring_count as f32 + phase).fract();
        // После `ring_stop` новые кольца не рождаются, а старые долетают.
        if elapsed - t * tunnel.ring_period > tunnel.ring_stop {
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

    #[test]
    fn hyperspace_starts_with_smaller_ring_than_station() {
        let base_r = 100.0;
        let hyper = tunnel_gate_radius(base_r, &HYPERSPACE_TUNNEL, 0.0).unwrap();
        let station = station_gate_radius(base_r, 0.0).unwrap();
        assert!(
            hyper < station,
            "кольцо гиперпрыжка не меньше станционного: {hyper} >= {station}"
        );
        assert!(
            (station / hyper - 2.0).abs() < 0.01,
            "кольцо гиперпрыжка не вдвое меньше станционного: {hyper} vs {station}"
        );
    }

    #[test]
    fn hyperspace_gate_flies_past_by_the_end() {
        let base_r = 100.0;
        let max_r = 141.4;
        let late = tunnel_gate_radius(base_r, &HYPERSPACE_TUNNEL, HYPERSPACE_EXIT_TIME * 0.99)
            .unwrap();
        assert!(
            late > max_r,
            "кольцо гиперпрыжка не улетело за экран к концу: {late}"
        );
        assert!(
            tunnel_gate_radius(base_r, &HYPERSPACE_TUNNEL, HYPERSPACE_EXIT_TIME).is_none(),
            "кольцо гиперпрыжка осталось перед камерой в конце"
        );
    }

    #[test]
    fn hyperspace_rings_last_the_whole_transition() {
        let base_r = 64.0;
        let max_r = ((256.0f32 * 256.0 + 128.0 * 128.0).sqrt()) * 0.5;
        let ratio = tunnel_ring_ratio(base_r, max_r, &HYPERSPACE_TUNNEL);
        // Кольца должны быть видны почти до самого конца ролика, а не
        // заканчиваться на его середине.
        for part in [0.1, 0.3, 0.5, 0.7, 0.9, 0.95] {
            let elapsed = HYPERSPACE_EXIT_TIME * part;
            let gate = tunnel_gate_radius(base_r, &HYPERSPACE_TUNNEL, elapsed).unwrap();
            let radii = tunnel_ring_radii(gate, max_r, ratio, elapsed, &HYPERSPACE_TUNNEL);
            assert!(
                !radii.is_empty(),
                "на {elapsed} с (доля {part}) в тоннеле гиперпрыжка нет колец"
            );
        }
    }

    #[test]
    fn hyperspace_tunnel_covers_the_real_transition() {
        // Реальный переход Z80 длится около 500 кадров (~10 с). Ролик не
        // должен заканчиваться заметно раньше, иначе труба пропадает до
        // смены системы.
        assert!(
            HYPERSPACE_EXIT_TIME >= 9.5,
            "тоннель гиперпрыжка короче настоящего перехода: {HYPERSPACE_EXIT_TIME} с"
        );
    }

    #[test]
    fn hyperspace_shows_scene_only_inside_the_ring() {
        const SCENE: u32 = 0xFF00_8040;
        let (w, h) = (200usize, 200usize);
        let mut frame = Frame::new(w, h);
        frame.fill_rect(0, 0, w, h, SCENE);
        draw_hyperspace(&mut frame, 0, 0, w as i32, h as i32, 1, 0.0);
        let gate = tunnel_gate_radius(100.0, &HYPERSPACE_TUNNEL, 0.0).unwrap();
        assert_eq!(pixel(&frame, 100, 100), SCENE, "в центре нет сцены");
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let dx = (x - 100) as f32;
                let dy = (y - 100) as f32;
                if (dx * dx + dy * dy).sqrt() > gate + 2.0 {
                    let p = pixel(&frame, x, y);
                    assert_ne!(p, SCENE, "сцена видна за кольцом ({x},{y})");
                }
            }
        }
    }
}
