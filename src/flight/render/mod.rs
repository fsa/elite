//! Отрисовка нативного вида из кабины.
//!
//! Модуль только раскладывает кадр по элементам: звёзды и объекты, надписи,
//! переходы. Вся геометрия и состояние уже подготовлены подсистемами
//! [`super::stars`], [`super::scene`] и [`super::transition`].

use crate::cockpit::CockpitGeom;
use crate::display::Frame;
use crate::render3d::Screen;

use super::palette::{BG, VIEW_LABEL_COLOR};
use super::stars::star_count_for;
use super::view;
use super::Flight;

pub(super) mod scene;
mod text;
mod transition;

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

    let current = f.transition.current;
    match current {
        Some(t) if t.kind == super::TransitionKind::Launch => {
            transition::draw_station(frame, vx, vy, vw, vh, s, t.elapsed, true, f.planet_color);
        }
        Some(t) if t.kind == super::TransitionKind::Hyperspace && !t.warp => {
            // До нуля отсчёта показываем обычный полёт: игрок ещё может
            // осмотреться, а цифра говорит, сколько осталось.
            scene::draw_flight_scene(f, frame, &screen, &view_rot, view_bottom, s);
            // Во время отсчёта верхнюю надпись рисуем сами (подпись вида):
            // перехваченные игра печатает в поле зрения цифры, их здесь не
            // показываем. Само число идёт слева в той же строке, что и
            // подпись вида.
            draw_view_label(f, frame, font, vx, vy, vw, s, false);
            draw_jump_countdown(frame, font, vx, vy, s, t.countdown);
            // Строку «HYPERSPACE -- <планета>» печатает сама игра; показываем
            // перехваченный текст внизу вида, как и раньше.
            if let Some(label) = f.messages.jump_label() {
                draw_jump_label(frame, font, vx, vy, vw, vh, s, label);
            }
        }
        Some(t) if t.kind == super::TransitionKind::Docking => {
            // Стыковка — тот же белый тоннель, но без планеты в конце.
            transition::draw_station(
                frame,
                vx,
                vy,
                vw,
                vh,
                s,
                t.warp_elapsed,
                false,
                f.planet_color,
            );
        }
        Some(t) => {
            // Гиперпрыжок: только тоннель и полосы звёзд, без надписей.
            transition::draw_warp(
                f,
                frame,
                font,
                &screen,
                vx,
                vy,
                vw,
                vh,
                s,
                t.warp_elapsed,
                None,
            );
        }
        None => {
            scene::draw_flight_scene(f, frame, &screen, &view_rot, view_bottom, s);
            draw_view_label(f, frame, font, vx, vy, vw, s, true);
        }
    }
    frame.clear_clip();
}

/// Верхняя надпись вида: тем же шрифтом и в том же месте, что заголовок
/// «E L I T E» на заставке — одной текстовой строкой ниже верхней рамки,
/// по центру.
///
/// Пока в верхней зоне вида есть свежее сообщение самой игры (перехват
/// `sub_D499`), показываем его с большой буквы; иначе — подпись
/// направления вида. При `use_tap == false` (отсчёт гиперпрыжка) вывод
/// перехваченного текста отключён: игре печатает в эту зону цифры.
///
/// Переход «сообщение игры → подпись вида» внешне незаметен: у свежих
/// подписей вида текст совпадает («FRONT VIEW» → «Front View», и подпись
/// тоже «Front View»).
fn draw_view_label(
    f: &Flight,
    frame: &mut Frame,
    font: &[u8],
    vx: i32,
    vy: i32,
    vw: i32,
    s: i32,
    use_tap: bool,
) {
    let text = if use_tap {
        f.messages.top_label(f.view_dir)
    } else {
        super::messages::view_label(f.view_dir).to_string()
    };
    let scale = s.max(1) as usize;
    text::draw_centered_text(
        frame,
        font,
        vx,
        vy + 8 * s,
        vw,
        &text,
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

/// Строка гиперпрыжка из игры внизу вида: одна текстовая строка выше нижней
/// рамки, по центру, тем же шрифтом и цветом, что подпись вида.
#[allow(clippy::too_many_arguments)]
fn draw_jump_label(
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
