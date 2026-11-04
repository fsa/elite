//! Элементы обычного вида: звёзды, объекты, облака взрывов, выстрелы и
//! перекрестие.

use crate::display::Frame;
use crate::render3d::{self, Rotation, Screen};

use super::super::explosions::EXPLOSION_TIME;
use super::super::palette::{
    explosion_color, planet_rim, STAR_RIM_COLOR, CYAN, ENEMY_BEAM, RED,
};
use super::super::scene::{Body, BodyKind};
use super::super::stars::Star;
use super::super::view::{self, ViewDir};
use super::super::Flight;

/// Обычный вид из кабины: звёзды, объекты, выстрелы и перекрестие.
pub(super) fn draw_flight_scene(
    f: &Flight,
    frame: &mut Frame,
    screen: &Screen,
    view_rot: &Rotation,
    view_bottom: i32,
    s: i32,
) {
    draw_stars_view(frame, screen, &f.stars.stars, view_rot, f.stars.streak());
    draw_bodies_view(f, frame, screen, view_rot, s);
    draw_explosions_view(f, frame, screen, view_rot, s);
    draw_enemy_shots_view(f, frame, screen, 0, view_bottom, view_rot);
    if f.view_dir == ViewDir::Forward {
        draw_crosshair(frame, screen.cx, screen.cy, s);
    }
    if f.combat.laser_timer > 0.0 && f.view_dir == ViewDir::Forward {
        draw_laser(frame, screen.cx, screen.cy, view_bottom - 1, s);
    }
}

pub(crate) fn draw_stars_view(
    frame: &mut Frame,
    screen: &Screen,
    stars: &[Star],
    view_rot: &Rotation,
    streak: f32,
) {
    let cam = view_rot.transpose();
    let (xmin, ymin) = (0.0, 0.0);
    let (xmax, ymax) = (frame.width as f32 - 1.0, frame.height as f32 - 1.0);
    for star in stars {
        let head = cam.apply(star.pos);
        let head_visible = head[2] > view::NEAR;
        if !head_visible && streak <= 0.0 {
            continue;
        }
        let bright = 0.25 + star.mag as f32 / 255.0 * 0.75;
        let c = 0xFF00_0000
            | ((bright * 210.0) as u32) << 16
            | ((bright * 225.0) as u32) << 8
            | (bright * 255.0) as u32;
        // На J-драйве звезда вытягивается в штрих от прошлого положения к
        // текущему. Хвост лежит дальше по `+Z`: в переднем виде штрих идёт от
        // центра наружу, в заднем — снаружи к центру.
        if streak > 0.0 {
            let tail = cam.apply([star.pos[0], star.pos[1], star.pos[2] + streak]);
            if let Some((a, b)) = view::clip_segment_near(tail, head) {
                let ax = screen.cx + a[0] / a[2] * screen.k;
                let ay = screen.cy - a[1] / a[2] * screen.k;
                let bx = screen.cx + b[0] / b[2] * screen.k;
                let by = screen.cy - b[1] / b[2] * screen.k;
                if let Some((x0, y0, x1, y1)) =
                    view::clip_segment_2d(ax, ay, bx, by, xmin, ymin, xmax, ymax)
                {
                    frame.line(x0, y0, x1, y1, c);
                }
            }
        }
        if !head_visible {
            continue;
        }
        let x = screen.cx + head[0] / head[2] * screen.k;
        let y = screen.cy - head[1] / head[2] * screen.k;
        if x < 0.0 || y < 0.0 || x >= frame.width as f32 || y >= frame.height as f32 {
            continue;
        }
        let (xi, yi) = (x.round() as i32, y.round() as i32);
        frame.plot(xi, yi, c);
        if star.mag > 210 {
            frame.plot(xi + 1, yi, c);
            frame.plot(xi - 1, yi, c);
            frame.plot(xi, yi + 1, c);
            frame.plot(xi, yi - 1, c);
        }
    }
}

fn draw_bodies_view(f: &Flight, frame: &mut Frame, screen: &Screen, view_rot: &Rotation, s: i32) {
    let cam = view_rot.transpose();
    // Дальние объекты рисуются первыми, ближние перекрывают их.
    let mut order: Vec<(f32, usize)> = f
        .bodies
        .iter()
        .enumerate()
        .map(|(i, b)| (cam.apply(b.pos)[2], i))
        .collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (vz, i) in order {
        if vz <= view::NEAR {
            continue;
        }
        let b = &f.bodies[i];
        let vpos = cam.apply(b.pos);
        if vpos[2] <= view::NEAR {
            continue;
        }
        match b.kind {
            BodyKind::Planet => {
                let Some((x, y, r, _)) = project_view(b, screen, &cam) else {
                    continue;
                };
                draw_planet(frame, x, y, r, b.color, s);
            }
            BodyKind::Sun => {
                let Some((x, y, r, _)) = project_view(b, screen, &cam) else {
                    continue;
                };
                draw_star(frame, x, y, r, b.color, s);
            }
            BodyKind::Ship | BodyKind::Station => {
                let Some(model) = b.model else {
                    continue;
                };
                let vrot = cam.mul(&b.rot);
                render3d::draw_ship(frame, model, &vrot, vpos, screen, render3d::SHIP_COLOR);
            }
        }
    }
}

/// Нарисовать облака частиц на месте уничтоженных объектов.
///
/// Частицы живут в координатах вида, поэтому проекция та же, что у тел сцены:
/// облако поворачивается вместе с миром и сжимается по мере удаления. Размер
/// точки привязан к масштабу кадра (`s`), чтобы на большом окне облако не
/// превращалось в еле заметную пыль.
fn draw_explosions_view(
    f: &Flight,
    frame: &mut Frame,
    screen: &Screen,
    view_rot: &Rotation,
    s: i32,
) {
    if f.explosions.clouds.is_empty() {
        return;
    }
    let cam = view_rot.transpose();
    let dot = s.max(1);
    for cloud in &f.explosions.clouds {
        let t = 1.0 - cloud.age / EXPLOSION_TIME;
        if t <= 0.0 {
            continue;
        }
        let color = explosion_color(t);
        for p in &cloud.particles {
            let pos = [
                cloud.center[0] + p.offset[0],
                cloud.center[1] + p.offset[1],
                cloud.center[2] + p.offset[2],
            ];
            let vpos = cam.apply(pos);
            if vpos[2] <= view::NEAR {
                continue;
            }
            let x = screen.cx + vpos[0] / vpos[2] * screen.k;
            let y = screen.cy - vpos[1] / vpos[2] * screen.k;
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            let half = dot / 2;
            frame.fill_rect(
                x.round() as i32 - half,
                y.round() as i32 - half,
                dot as usize,
                dot as usize,
                color,
            );
        }
    }
}

/// Нарисовать планету: тёмно-зелёная заливка и более яркий ободок.
///
/// Толщина ободка масштабируется вместе с видом (`s`), чтобы на большом окне
/// рамка не превращалась в волосяную линию. У очень маленькой планеты
/// ободок занимает весь диск — как в оригинале, где планета рисовалась
/// только контуром.
pub(super) fn draw_planet(frame: &mut Frame, x: f32, y: f32, r: f32, color: u32, s: i32) {
    draw_disc_with_rim(frame, x, y, r, color, planet_rim(color), s);
}

/// Нарисовать звезду: почти белая заливка с чисто белой окантовкой.
///
/// Оттенок заливки — от цвета системы на карте, поэтому звезда остаётся
/// узнаваемой, но не сливается с белыми кораблями и рамкой кабины.
pub(crate) fn draw_star(frame: &mut Frame, x: f32, y: f32, r: f32, color: u32, s: i32) {
    draw_disc_with_rim(frame, x, y, r, color, STAR_RIM_COLOR, s);
}

/// Диск с ободком: внешний круг цвета `rim`, внутренний — цвета `color`.
fn draw_disc_with_rim(
    frame: &mut Frame,
    x: f32,
    y: f32,
    r: f32,
    color: u32,
    rim: u32,
    s: i32,
) {
    let thickness = s.max(1) as f32;
    fill_disc(frame, x, y, r, rim);
    if r > thickness {
        fill_disc(frame, x, y, r - thickness, color);
    }
}

/// Проекция центра объекта: экранные координаты и радиус диска.
fn project_view(b: &Body, screen: &Screen, view_rot: &Rotation) -> Option<(f32, f32, f32, f32)> {
    view::project(view_rot.apply(b.pos), b.radius, screen)
}

/// Нарисовать лучи выстрелов противников.
fn draw_enemy_shots_view(
    f: &Flight,
    frame: &mut Frame,
    screen: &Screen,
    top: i32,
    bottom: i32,
    view_rot: &Rotation,
) {
    let cam = view_rot.transpose();
    let (xmin, ymin) = (0.0, top as f32);
    let (xmax, ymax) = (frame.width as f32 - 1.0, bottom as f32 - 1.0);
    for shot in &f.combat.enemy_shots {
        let Some((a, b)) = view::clip_segment_near(shot.from, shot.to) else {
            continue;
        };
        let va = cam.apply(a);
        let vb = cam.apply(b);
        if va[2] <= view::NEAR && vb[2] <= view::NEAR {
            continue;
        }
        let ax = screen.cx + va[0] / va[2].max(view::NEAR) * screen.k;
        let ay = screen.cy - va[1] / va[2].max(view::NEAR) * screen.k;
        let bx = screen.cx + vb[0] / vb[2].max(view::NEAR) * screen.k;
        let by = screen.cy - vb[1] / vb[2].max(view::NEAR) * screen.k;
        // В оригинале луч не обрывается у перекрестия, а уходит через
        // него до края вида. Продлеваем отрезок в том же направлении и
        // отдаём отсечение прямоугольнику — так выстрел хорошо виден.
        let dx = bx - ax;
        let dy = by - ay;
        let len = (dx * dx + dy * dy).sqrt();
        let (dir_x, dir_y) = if len < 1e-3 {
            (1.0, 0.0)
        } else {
            (dx / len, dy / len)
        };
        let far = 8192.0;
        let Some((x0, y0, x1, y1)) = view::clip_segment_2d(
            ax,
            ay,
            bx + dir_x * far,
            by + dir_y * far,
            xmin,
            ymin,
            xmax,
            ymax,
        ) else {
            continue;
        };
        frame.line(x0, y0, x1, y1, ENEMY_BEAM);
        // Второй пиксель поперёк луча делает выстрел заметнее на фоне
        // тонких белых рёбер корпусов.
        frame.line(x0, y0 + 1, x1, y1 + 1, RED);
    }
}

fn draw_crosshair(frame: &mut Frame, cx: f32, cy: f32, s: i32) {
    let (x, y) = (cx as i32, cy as i32);
    let gap = 3 * s;
    let len = 9 * s;
    frame.hline(x - gap - len, y, len as usize, CYAN);
    frame.hline(x + gap, y, len as usize, CYAN);
    frame.vline(x, y - gap - len, len as usize, CYAN);
    frame.vline(x, y + gap, len as usize, CYAN);
}

fn draw_laser(frame: &mut Frame, cx: f32, cy: f32, bottom: i32, s: i32) {
    let x = cx as i32;
    let y = cy as i32;
    let off = 12 * s;
    frame.line(x - off, bottom, x - 2, y, RED);
    frame.line(x + off, bottom, x + 2, y, RED);
    frame.line(x - off + 2, bottom, x, y, 0xFFFF_A000);
    frame.line(x + off - 2, bottom, x, y, 0xFFFF_A000);
}

/// Залить диск в кадре с отсечением по видимой области.
///
/// Радиус считается в `f64`: вблизи камеры солнце проецируется диском в
/// сотни тысяч пикселей, и `i32`-арифметика [`Frame::disc`] переполнялась.
/// Строки за пределами кадра не перебираются.
fn fill_disc(frame: &mut Frame, x: f32, y: f32, r: f32, color: u32) {
    if !(r >= 0.5) || !x.is_finite() || !y.is_finite() || !r.is_finite() {
        return;
    }
    let (w, h) = (frame.width as f32, frame.height as f32);
    if x + r < 0.0 || y + r < 0.0 || x - r >= w || y - r >= h {
        return;
    }
    let (cx, cy, r) = (x as f64, y as f64, r as f64);
    let y0 = (cy - r).ceil().max(0.0) as i64;
    let y1 = (cy + r).floor().min(h as f64 - 1.0) as i64;
    for yy in y0..=y1 {
        let dy = yy as f64 - cy;
        let d2 = r * r - dy * dy;
        if d2 <= 0.0 {
            continue;
        }
        let dx = d2.sqrt();
        let x0 = ((cx - dx).ceil() as i64).max(0);
        let x1 = ((cx + dx).floor() as i64).min(frame.width as i64 - 1);
        if x0 <= x1 {
            frame.hline(x0 as i32, yy as i32, (x1 - x0 + 1) as usize, color);
        }
    }
}
