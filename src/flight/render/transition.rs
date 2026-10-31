//! Отрисовка переходов: вылет со станции и тоннель гиперпрыжка/стыковки.

use crate::display::Frame;
use crate::render3d::{self, Rotation, Screen};
use crate::shipdata::{self, Ship};

use super::super::palette::{TRANSITION_TEXT, WARP_RING_COLOR};
use super::super::stars::Star;
use super::super::transition::LAUNCH_TIME;
use super::super::view::{self, ViewDir};
use super::super::Flight;
use super::text::draw_centered_text;

/// Анимация вылета: станция уходит назад, звёзды и кольца разгоняются.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_launch(
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
) {
    let t = (elapsed / LAUNCH_TIME).clamp(0.0, 1.0);
    let forward = view::rotation(ViewDir::Forward);
    draw_star_streaks(
        frame,
        screen,
        &f.stars.stars,
        &forward,
        vx,
        vy,
        vw,
        vh,
        60.0 + t * 520.0,
    );
    draw_tunnel_rings(frame, vx, vy, vw, vh, (t * 1.8).fract(), 4, 0xFF20_5060);
    // Станция видна только в начале: потом она уже далеко за кормой.
    if t < 0.7 {
        if let Some(model) = coriolis_model() {
            let k = (t / 0.7).clamp(0.0, 1.0);
            let z = 260.0 + k * k * 5200.0;
            let rot = Rotation::axis_angle([0.0, 1.0, 0.0], 0.35 + t * 1.1)
                .mul(&Rotation::axis_angle([1.0, 0.0, 0.0], 0.15));
            render3d::draw_ship(
                frame,
                model,
                &rot,
                [0.0, 0.0, z],
                screen,
                render3d::SHIP_COLOR,
            );
        }
    }
    let scale = (s * 2).max(1) as usize;
    draw_centered_text(
        frame,
        font,
        vx,
        vy + 4 * s,
        vw,
        "LAUNCH",
        TRANSITION_TEXT,
        scale,
    );
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
    title: &str,
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

/// Модель станции для анимации вылета.
fn coriolis_model() -> Option<&'static Ship> {
    shipdata::SHIPS.iter().find(|s| s.name == "d_Coriolis")
}
