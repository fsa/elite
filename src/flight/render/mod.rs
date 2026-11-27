//! Отрисовка нативного вида из кабины.
//!
//! Модуль только раскладывает кадр по элементам: звёзды и объекты, надписи,
//! переходы. Вся геометрия и состояние уже подготовлены подсистемами
//! [`super::stars`], [`super::scene`], [`super::transition`] и
//! [`super::messages`].

use crate::cockpit::CockpitGeom;
use crate::display::Frame;
use crate::render3d::Screen;

use super::palette::{BG, VIEW_LABEL_COLOR};
use super::stars::star_count_for;
use super::view;
use super::Flight;

pub(super) mod scene;
mod text;
pub(super) mod transition;

/// Нарисовать вид из кабины в верхней части кадра.
///
/// `geom` — геометрия кокпита: рендер заполняет область над панелью и
/// рисует сцену только внутри главной рамки, которую потом обводит
/// [`crate::cockpit::Cockpit`]. `font` — знакогенератор ZX для подписей
/// вида и обратного отсчёта.
pub(super) fn draw(f: &mut Flight, frame: &mut Frame, geom: &CockpitGeom, font: &[u8]) {
    let w = frame.width as i32;
    let view_bottom = geom.panel_y.clamp(1, frame.height as i32);
    let view_h = view_bottom as usize;
    frame.fill_rect(0, 0, frame.width, view_h, BG);
    if f.docking_blackout {
        // Между концом нативного тоннеля и уходом Z80 на информационный экран
        // оригинал ещё держит кабину с уже пристыкованным кораблём. Обычная
        // сцена в этот момент выглядела бы как вид сквозь станцию.
        return;
    }
    // Плотность звёзд зависит от площади вида: на большом разрешении
    // исходных 16 точек недостаточно, экран выглядит пустым.
    f.stars.resize(star_count_for(frame.width, view_h));

    // Масштаб штрихов как у сетки ZX: вид 256x128 над панелью.
    let s = ((frame.width / 256).min(view_h / 128)).max(1) as i32;
    let cx = w as f32 / 2.0;
    let cy = view_bottom as f32 / 2.0;
    let focal = (view_bottom as f32 * 1.15).max(200.0);
    let screen = Screen {
        cx,
        cy,
        k: focal,
        bx: 0.0,
        by: 0.0,
    };

    let view_rot = view::rotation(f.view_dir);

    // Ограничиваем отрисовку внутренней областью главной рамки.
    let view = geom.view();
    let (vx, vy, vw, vh) = (view.x, view.y, view.w.max(0), view.h.max(0));
    frame.set_clip(vx, vy, vw as usize, vh as usize);

    if let Some(t) = f.transition.station {
        match t.kind {
            super::transition::StationKind::Launch => {
                // Звёзды идут под кольцами трубы. В первых 0x24 проходах оригинал
                // рисует только звёзды, в последних 0x0B добавляет пересчитанные
                // солнце и планету. Центральный выход ограничивает обзор сцены,
                // пока не уйдёт за край экрана.
                scene::draw_stars_view(
                    frame,
                    &screen,
                    &f.render_stars(),
                    &view_rot,
                    f.stars.streak(),
                );
                if t.elapsed >= transition::STATION_REAL_SCENE_TIME {
                    scene::draw_bodies_view(f, frame, &screen, &view_rot, s);
                    if let Some(r) = transition::station_aperture_radius(vw, vh, t.elapsed) {
                        transition::mask_outside_aperture(
                            frame,
                            vx,
                            vy,
                            vw,
                            vh,
                            vx + vw / 2,
                            vy + vh / 2,
                            r,
                        );
                    }
                    // Кольца рисуются последними: планета не должна перекрывать
                    // последнее кольцо, пока оно не ушло за край экрана.
                    transition::draw_station(frame, vx, vy, vw, vh, s, t.elapsed, false, 0);
                } else {
                    transition::draw_station(
                        frame, vx, vy, vw, vh, s, t.elapsed, true, f.planet_color,
                    );
                }
                // Ролик рисует только трубу: подпись Front View уходит вместе с
                // обычной сценой, поэтому её нужно нарисовать поверх анимации.
                draw_view_label(f, frame, font, vx, vy, vw, s);
            }
            super::transition::StationKind::Docking => {
                // Стыковка — тот же белый тоннель, но без планеты в конце.
                transition::draw_station(
                    frame,
                    vx,
                    vy,
                    vw,
                    vh,
                    s,
                    t.elapsed,
                    false,
                    f.planet_color,
                );
                draw_view_label(f, frame, font, vx, vy, vw, s);
            }
        }
    } else if let Some(t) = f.transition.hyperspace {
        // Во время прыжка игра рисует обычное пространство, а тоннель
        // ограничивает вид центральным кольцом. До нуля отсчёта показываем
        // ещё и цифру.
        scene::draw_flight_scene(f, frame, &screen, &view_rot, view_bottom, s);
        if t.warp {
            transition::draw_hyperspace(frame, vx, vy, vw, vh, s, t.warp_elapsed);
            // Звёзды прыжка поверх тоннеля: видны и поверх колец, и вне
            // центрального кольца.
            let stars = f.render_stars();
            scene::draw_stars_view(frame, &screen, &stars, &view_rot, f.stars.streak());
            // Тоннель закрашивает верх строки обычного вида, поэтому
            // подпись рисуется последней, как на экране F1.
            draw_view_label(f, frame, font, vx, vy, vw, s);
        } else {
            // Во время отсчёта верхнюю надпись рисуем сами (подпись вида):
            // перехваченные игра печатает в поле зрения цифры, их здесь не
            // показываем. Само число идёт слева в той же строке, что и
            // подпись вида.
            draw_view_label(f, frame, font, vx, vy, vw, s);
            draw_jump_countdown(frame, font, vx, vy, s, t.countdown);
        }
    } else {
        scene::draw_flight_scene(f, frame, &screen, &view_rot, view_bottom, s);
        if f.game_over {
            draw_game_over(frame, font, vx, vy, vw, vh, s);
        } else {
            draw_view_label(f, frame, font, vx, vy, vw, s);
        }
    }

    // Нижнее сообщение игры — только перехваченная строка 13. Позицию из
    // ZX-экрана не используем: рендер всегда выводит текст по центру вида.
    if let Some(label) = f.messages.bottom_label() {
        draw_bottom_label(frame, font, vx, vy, vw, vh, s, label);
    }
    frame.clear_clip();
}

/// Надпись «Game Over» по центру вида вместо подписи направления.
///
/// После гибели корабля оригинал ещё несколько секунд держит `WhatOnScreen`
/// в виде из кабины; нативный рендер продолжает рисовать сцену, но верхнюю
/// подпись («Front View» и т.п.) заменяет этой надписью.
fn draw_game_over(
    frame: &mut Frame,
    font: &[u8],
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    s: i32,
) {
    let scale = s.max(1) as usize;
    let y = vy + (vh - 8 * scale as i32) / 2;
    text::draw_centered_text(
        frame,
        font,
        vx,
        y,
        vw,
        "Game Over",
        VIEW_LABEL_COLOR,
        scale,
    );
}

/// Верхняя надпись вида: тем же шрифтом и в том же месте, что заголовок
/// «E L I T E» на заставке — одной текстовой строкой ниже верхней рамки,
/// по центру.
///
/// Подпись направления вида. Остальные надписи в верхней зоне рендер
/// показывает сам; перехватывается только нижняя строка сообщений.
fn draw_view_label(
    f: &Flight,
    frame: &mut Frame,
    font: &[u8],
    vx: i32,
    vy: i32,
    vw: i32,
    s: i32,
) {
    let text = super::messages::view_label(f.view_dir);
    let scale = s.max(1) as usize;
    text::draw_centered_text(
        frame,
        font,
        vx,
        vy + 8 * s,
        vw,
        text,
        VIEW_LABEL_COLOR,
        scale,
    );
}

/// Обратный отсчёт гиперпрыжка поверх обычного полёта: только число в левом
/// верхнем углу, в той же строке, что и подпись вида, тем же шрифтом и цветом.
fn draw_jump_countdown(frame: &mut Frame, font: &[u8], vx: i32, vy: i32, s: i32, raw: u8) {
    let number = super::messages::countdown_number(raw).to_string();
    let scale = s.max(1) as usize;
    // Один знак отступа от левой рамки — как строка вида от верхней.
    frame.draw_text(
        font,
        vx + 8 * s,
        vy + 8 * s,
        &number,
        VIEW_LABEL_COLOR,
        scale,
    );
}

/// Нижнее сообщение игры: одна текстовая строка выше нижней рамки. Позиция
/// из ZX-строки не используется — текст всегда центрируется по ширине вида.
#[allow(clippy::too_many_arguments)]
fn draw_bottom_label(
    frame: &mut Frame,
    font: &[u8],
    vx: i32,
    vy: i32,
    vw: i32,
    vh: i32,
    s: i32,
    label: &str,
) {
    let y = vy + vh - 16 * s;
    text::draw_centered_text(
        frame,
        font,
        vx,
        y,
        vw,
        label,
        VIEW_LABEL_COLOR,
        s.max(1) as usize,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::palette::STATION_RING_COLOR;
    use crate::cockpit;
    use crate::display::{Frame, TRANSPARENT};
    use crate::flight::scene::{Body, BodyKind, BODY_SRC_PLANET};
    use crate::flight::TransitionKind;
    use crate::font::ZX_FONT;
    use crate::render3d::{ENGINE_IDLE, Rotation};

    const W: usize = 256;
    const H: usize = 192;
    const PLANET: u32 = 0x00FF00;

    fn launch_frame(elapsed: f32) -> Frame {
        let mut f = Flight::new();
        f.open(0);
        f.planet_color = PLANET;
        f.start_transition_kind(TransitionKind::Launch, 0);
        f.transition.station.as_mut().unwrap().elapsed = elapsed;

        let geom = cockpit::panel_geometry(W as u32, H as u32);
        let mut frame = Frame::new(W, H);
        frame.clear(TRANSPARENT);
        f.draw(&mut frame, &geom, ZX_FONT);
        frame
    }

    fn planet_pixels(frame: &Frame) -> usize {
        frame.pixels().iter().filter(|&&p| p == PLANET).count()
    }

    #[test]
    fn launch_tunnel_starts_with_fake_planet() {
        let frame = launch_frame(1.0);
        assert!(
            planet_pixels(&frame) > 0,
            "в начале тоннеля нет планеты в кольце-выходе"
        );
    }

    #[test]
    fn launch_does_not_end_with_a_full_screen_planet() {
        let frame = launch_frame(transition::STATION_REAL_SCENE_TIME + 0.1);
        assert_eq!(
            planet_pixels(&frame),
            0,
            "на выходе осталась фейковая планета из ролика"
        );
    }

    #[test]
    fn launch_does_not_cover_the_exit_ring_with_a_planet() {
        let elapsed = transition::STATION_REAL_SCENE_TIME + 0.1;
        let mut f = Flight::new();
        f.open(PLANET);
        f.start_transition_kind(TransitionKind::Launch, 0);
        f.transition.station.as_mut().unwrap().elapsed = elapsed;
        // Планета закрывает весь вид: кольцо всё равно должно быть поверх неё.
        f.bodies.push(Body {
            source: BODY_SRC_PLANET,
            kind: BodyKind::Planet,
            model: None,
            pos: [0.0, 0.0, 1000.0],
            rot: Rotation::identity(),
            radius: 1000.0,
            color: PLANET,
            tint: None,
            engine: ENGINE_IDLE,
        });

        let geom = cockpit::panel_geometry(W as u32, H as u32);
        let mut frame = Frame::new(W, H);
        frame.clear(TRANSPARENT);
        f.draw(&mut frame, &geom, ZX_FONT);

        let view = geom.view();
        let r = transition::station_aperture_radius(view.w as i32, view.h as i32, elapsed).unwrap();
        let x = view.x + view.w / 2 + r.round() as i32;
        let y = view.y + view.h / 2;
        assert_eq!(
            frame.pixels()[y as usize * W + x as usize],
            STATION_RING_COLOR,
            "планета перекрыла кольцо-выход"
        );
    }
}
