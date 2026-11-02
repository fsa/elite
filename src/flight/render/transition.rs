//! Отрисовка переходов: тоннель станции и тоннель гиперпрыжка.

use crate::display::Frame;
use crate::render3d::{Rotation, Screen};

use super::super::palette::{STATION_RING_COLOR, TRANSITION_TEXT, WARP_RING_COLOR};
use super::super::stars::Star;
use super::super::view;
use super::super::Flight;
use super::scene::draw_planet;
use super::text::draw_centered_text;

/// Скорость движения колец тоннеля станции (доля цикла в секунду).
const STATION_TUNNEL_SPEED: f32 = 1.35;
/// Сколько белых колец рисуется в тоннеле станции.
const STATION_RING_COUNT: usize = 8;

/// Тоннель станции: концентрические белые окружности, как в оригинале.
///
/// При вылете в центре тоннеля видна планета — та самая, к которой ведёт
/// труба. При стыковке планеты нет: кольца просто уходят в точку.
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
    let max_r = ((vw * vw + vh * vh) as f32).sqrt() * 0.5;
    let planet_r = if launch {
        (vw.min(vh) as f32 * 0.16).max(4.0)
    } else {
        0.0
    };
    draw_station_rings(
        frame,
        cx,
        cy,
        max_r,
        planet_r,
        (elapsed * STATION_TUNNEL_SPEED).fract(),
        s,
    );
    // Планета закрывает начало тоннеля: кольца как будто выходят из неё.
    if launch {
        draw_planet(frame, cx as f32, cy as f32, planet_r, planet_color, s);
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

/// Белые кольца тоннеля станции.
///
/// Кольца стартуют у края планеты (`inner_r`) и расходятся к краям вида.
/// Так центральная окружность читается как конец трубы, а не как её
/// продолжение.
fn draw_station_rings(
    frame: &mut Frame,
    cx: i32,
    cy: i32,
    max_r: f32,
    inner_r: f32,
    phase: f32,
    s: i32,
) {
    let span = (max_r - inner_r).max(0.0);
    let thickness = (s / 2).max(1);
    for i in 0..STATION_RING_COUNT {
        let t = (i as f32 / STATION_RING_COUNT as f32 + phase).fract();
        // Кольца ускоряются к краю экрана — эффект полёта сквозь трубу.
        let r = inner_r + span * t * t;
        let r = r.round() as i32;
        for k in 0..thickness {
            if r - k > 1 {
                frame.circle(cx, cy, r - k, STATION_RING_COLOR);
            }
        }
    }
}
