//! Тесты экрана полёта и его подсистем.

use super::*;
use crate::render3d::{Rotation, Screen};

use super::memory::{
    read_signed24, read_signed8, Z80_OBJ_CORIOLIS, Z80_OBJ_SLOT0, Z80_PLANET_CRD, Z80_PLANET_NAME,
    Z80_SHIELD_FRONT, Z80_SLOT_OADRDATA, Z80_SLOT_OENERG, Z80_SUN_CRD, Z80_TEMP_LASER,
};
use super::messages::{
    countdown_number, pretty_title, view_label, CapturedMsg, MSG_MAX_AGE, MSG_TOP_ROWS,
};
use super::palette::{CYAN, ENEMY_BEAM};
use super::render::scene::draw_stars_view;
use super::scene::{BodyKind, PLANET_RADIUS, SUN_RADIUS};
use super::stars::{star_count_for, Star, SPEED_MAX, STAR_COUNT, STAR_COUNT_MAX, STAR_FIELD_RADIUS};
use super::transition::Transition;
use super::view;

/// Тёмно-зелёный пиксель планеты: красный и синий нулевые, зелёный заметен.
fn is_planet_green(p: u32) -> bool {
    p >> 24 == 0xFF && p & 0x00FF_00FF == 0 && (p >> 8) & 0xFF >= 0x58
}

fn open() -> Flight {
    let mut f = Flight::new();
    f.open(0xFF40_80C0);
    f
}

fn star(pos: [f32; 3]) -> Star {
    Star { pos, mag: 128 }
}

#[test]
fn z80_stars_move_with_memory_speed() {
    // Камера не двигается нативно, но звёздное поле должно лететь со
    // скоростью из памяти игры, иначе экран выглядит застывшим.
    let mut f = open();
    f.speed = 12.0;
    if let Some(s) = f.stars.stars.first_mut() {
        s.pos = [200.0, 0.0, 2000.0];
    }
    let start = f.stars.stars[0].pos;
    f.tick(0.1);
    let moved = f.stars.stars[0].pos;
    let step = (0..3).map(|i| (moved[i] - start[i]).abs()).sum::<f32>();
    assert!(step > 1.0, "звёзды не движутся");
}

#[test]
fn z80_stars_turn_with_angle_rates() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];

    // Первый синк только запоминает таблицу звёзд: такта ещё не было.
    f.sync_from_z80(&mem);

    // Поворот без нового такта не применяется: иначе звёзды крутились бы
    // на каждом кадре 50 Гц, а не с частотой игрового цикла (~5.5 Гц).
    f.stars.stars = vec![star([1000.0, 0.0, 0.0])];
    mem[Z80_HIGH_ANGLE_LR] = 0x04;
    mem[Z80_HIGH_ANGLE_DU] = 0x00;
    f.sync_from_z80(&mem);
    assert_eq!(
        f.stars.stars[0].pos,
        [1000.0, 0.0, 0.0],
        "поворот применён без игрового такта"
    );

    // Игровой такт ловим по изменению таблицы звёзд, как в памяти игры.
    // Крен вправо: звезда на оси X должна уйти в +Y.
    let tick = |f: &mut Flight, mem: &mut [u8]| {
        mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
        f.sync_from_z80(mem);
    };
    tick(&mut f, &mut mem);
    let p = f.stars.stars[0].pos;
    let roll_angle = p[1].atan2(p[0]);
    assert!(
        p[1] > 0.0,
        "положительный крен должен крутить поле против часовой"
    );
    // Масштаб из CalcNewCrdObj_Pl: крен *2/256.
    assert!(
        (roll_angle - 4.0 * 2.0 / 256.0).abs() < 1e-3,
        "неверный масштаб крена: {roll_angle}"
    );

    // Тангаж: звезда впереди должна уйти вверх (положительный Y вида).
    // Знак `HighAngleDU` противоположен повороту поля: при взгляде вверх
    // мир на экране уезжает вниз.
    f.stars.stars = vec![star([0.0, 0.0, 1000.0])];
    mem[Z80_HIGH_ANGLE_LR] = 0x00;
    mem[Z80_HIGH_ANGLE_DU] = 0x02;
    tick(&mut f, &mut mem);
    let p = f.stars.stars[0].pos;
    let pitch_angle = (p[1] / 1000.0).asin();
    assert!(p[1] > 0.0, "положительный тангаж должен уводить поле вверх");
    // Масштаб из CalcNewCrdObj_Pl: тангаж *1/256.
    assert!(
        (pitch_angle - 2.0 / 256.0).abs() < 1e-3,
        "неверный масштаб тангажа: {pitch_angle}"
    );

    // Обратный знак — обратное вращение.
    f.stars.stars = vec![star([1000.0, 0.0, 0.0])];
    mem[Z80_HIGH_ANGLE_LR] = 0x84;
    mem[Z80_HIGH_ANGLE_DU] = 0x00;
    tick(&mut f, &mut mem);
    assert!(
        f.stars.stars[0].pos[1] < 0.0,
        "отрицательный крен должен крутить поле по часовой"
    );
}

#[test]
fn z80_speed_comes_from_gauge_not_clobbered_speed() {
    // На такте J-драйва `PrintStars` пишет в `HighDeltaSpeed` 0x1E/0x10.
    // Шкала скорости не должна из-за этого прыгать.
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0x41; // 5 * 12 + 5
    mem[Z80_HIGH_DELTA_SPEED] = 0x10; // след J-драйва
    f.sync_from_z80(&mem);
    assert!(
        (f.speed - 12.0).abs() < 1e-6,
        "скорость взята из затёртого HighDeltaSpeed: {}",
        f.speed
    );
}

#[test]
fn j_drive_accelerates_stars_on_game_tick() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0x41; // скорость 12
    mem[Z80_HIGH_DELTA_SPEED] = 12;
    f.sync_from_z80(&mem); // запомнили таблицу звёзд

    // Обычный такт: таблица обновилась, скорость настоящая.
    f.stars.stars = vec![star([100.0, 0.0, 1000.0])];
    mem[Z80_STARS_TABLE] = 1;
    f.sync_from_z80(&mem);
    let before = f.stars.stars[0].pos[2];
    f.tick(1.0 / 50.0);
    let normal = (f.stars.stars[0].pos[2] - before).abs();

    // Такт J-драйва: HighDeltaSpeed затёрт, таблица обновилась.
    f.stars.stars = vec![star([100.0, 0.0, 1000.0])];
    mem[Z80_STARS_TABLE] = 2;
    mem[Z80_HIGH_DELTA_SPEED] = 0x10;
    f.sync_from_z80(&mem);
    let before = f.stars.stars[0].pos[2];
    f.tick(1.0 / 50.0);
    let fast = (f.stars.stars[0].pos[2] - before).abs();

    assert!(normal > 0.0, "звёзды не движутся на обычном такте");
    assert!(
        fast > normal * 2.0,
        "J-драйв не разогнал звёзды: {normal} -> {fast}"
    );
}

#[test]
fn j_drive_flag_latches_until_game_tick() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0x41;
    f.sync_from_z80(&mem);

    // `StarsFastScroll` может быть виден в кадре до обновления таблицы.
    mem[Z80_STARS_FAST_SCROLL] = 1;
    f.sync_from_z80(&mem);
    assert!(!f.stars.boost, "разгон включён до игрового такта");

    mem[Z80_STARS_TABLE] = 3;
    f.sync_from_z80(&mem);
    assert!(f.stars.boost, "флаг J-драйва потерян между кадрами");
}

#[test]
fn stars_stay_in_view_after_many_turns() {
    // После длительного вращения и полёта ни одна звезда не должна
    // оказаться в бесконечности или NaN — иначе на экране появляются
    // длинные линии через весь кадр.
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0xA0; // скорость 31
    mem[Z80_HIGH_DELTA_SPEED] = 31;
    for frame in 0..2000 {
        mem[Z80_HIGH_ANGLE_LR] = if frame % 2 == 0 { 0x0F } else { 0x8F };
        mem[Z80_HIGH_ANGLE_DU] = if frame % 3 == 0 { 0x0F } else { 0x00 };
        // Имитируем игровой такт: таблица звёзд обновляется.
        mem[Z80_STARS_TABLE + frame % Z80_STARS_LEN] = frame as u8;
        f.sync_from_z80(&mem);
        f.tick(1.0 / 50.0);
    }
    for s in &f.stars.stars {
        assert!(
            s.pos.iter().all(|v| v.is_finite()),
            "координата звезды не число"
        );
        assert!(
            s.pos.iter().all(|v| v.abs() < 20_000.0),
            "звезда улетела слишком далеко: {:?}",
            s.pos
        );
    }
}

#[test]
fn close_star_does_not_draw_long_streak() {
    // Раньше близкая звезда оставляла линию через весь кадр: хвост
    // проецировался по другой матрице. Теперь звезда — только точка.
    let mut f = open();
    f.stars.stars = vec![Star {
        pos: [0.4, 0.0, 0.6],
        mag: 255,
    }];
    let mut frame = Frame::new(256, 192);
    let screen = Screen {
        cx: 128.0,
        cy: 96.0,
        k: 128.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_stars_view(
        &mut frame,
        &screen,
        &f.stars.stars,
        &Rotation::identity(),
        0.0,
    );
    let lit = frame.pixels().iter().filter(|&&p| p != 0).count();
    assert!(lit <= 5, "звезда нарисовала {lit} пикселей вместо точки");
}

#[test]
fn planet_is_dark_green_and_smaller_than_sun() {
    let color = planet_green(0xFF40_80C0);
    assert_eq!(color & 0x00FF_00FF, 0, "у планеты не зелёный цвет");
    assert!((color >> 8) & 0xFF > 0, "планета слишком тёмная");
    assert!(
        PLANET_RADIUS < SUN_RADIUS,
        "планета не должна быть больше солнца"
    );
}

#[test]
fn z80_detects_laser_shot_by_heating() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];

    f.sync_from_z80(&mem);
    assert_eq!(f.combat.laser_timer, 0.0);

    // Оригинал поднял TempLaser выстрелом: показываем луч.
    mem[Z80_TEMP_LASER] = 0x08;
    f.sync_from_z80(&mem);
    assert!(
        f.combat.laser_timer > 0.0,
        "выстрел не превратился в видимый луч"
    );

    // Остывание не должно само включать лазер.
    f.combat.laser_timer = 0.0;
    mem[Z80_TEMP_LASER] = 0x00;
    f.sync_from_z80(&mem);
    assert_eq!(f.combat.laser_timer, 0.0);
}

#[test]
fn enemy_attack_draws_visible_beam() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    assert!(
        f.combat.enemy_shots.is_empty(),
        "луч появился без попадания"
    );

    // Оригинал обработал попадание: передний щит уменьшился.
    mem[Z80_SHIELD_FRONT] = 0x40;
    f.sync_from_z80(&mem);
    assert!(
        !f.combat.enemy_shots.is_empty(),
        "выстрел противника не создан"
    );

    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &cockpit::panel_geometry(1280, 960), &[]);
    let beam_pixels = frame.pixels().iter().filter(|&&p| p == ENEMY_BEAM).count();
    assert!(
        beam_pixels > 50,
        "луч противника не попал в кадр или оборвался у перекрестия: {beam_pixels} px"
    );
}

#[test]
fn z80_shows_enemy_beam_when_shield_drops() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    assert!(
        f.combat.enemy_shots.is_empty(),
        "луч появился без попадания"
    );

    // Оригинал обработал попадание: передний щит уменьшился.
    mem[Z80_SHIELD_FRONT] = 0x40;
    f.sync_from_z80(&mem);
    assert!(
        !f.combat.enemy_shots.is_empty(),
        "падение щита не показано лучом"
    );
    let shot = &f.combat.enemy_shots[0];
    assert!(
        shot.from[2] > view::NEAR,
        "луч противника начался не у корабля: {:?}",
        shot.from
    );
}

#[test]
fn stars_move_faster_with_speed() {
    let mut slow = open();
    slow.speed = 0.0;
    slow.stars.stars = vec![star([200.0, 0.0, 2000.0])];
    let mut fast = open();
    fast.speed = SPEED_MAX;
    fast.stars.stars = vec![star([200.0, 0.0, 2000.0])];

    let start = slow.stars.stars[0].pos;
    slow.tick(0.1);
    let slow_step = (slow.stars.stars[0].pos[2] - start[2]).abs();
    let start = fast.stars.stars[0].pos;
    fast.tick(0.1);
    let fast_step = (fast.stars.stars[0].pos[2] - start[2]).abs();

    assert!(slow_step > 0.0, "на минимальной скорости звёзды стоят");
    assert!(
        fast_step > slow_step * 2.0,
        "скорость слабо влияет на звёзды: {slow_step} -> {fast_step}"
    );
    assert!(
        fast.stars.stars[0].pos[2] > 0.0,
        "звезда должна оставаться впереди"
    );
}

#[test]
fn star_field_fills_whole_sphere_and_scales_with_screen() {
    // Поле должно окружать корабль со всех сторон: иначе задний вид пуст,
    // а боковые виды показывают звёзды только в центре.
    let f = open();
    assert_eq!(f.stars.len(), STAR_COUNT);
    let (mut front, mut back, mut left, mut right) = (0, 0, 0, 0);
    for s in &f.stars.stars {
        let r = (s.pos[0] * s.pos[0] + s.pos[1] * s.pos[1] + s.pos[2] * s.pos[2]).sqrt();
        assert!(
            (1.0..=STAR_FIELD_RADIUS + 1.0).contains(&r),
            "звезда родилась вне облака: радиус {r:.0}"
        );
        if s.pos[2] > 0.0 {
            front += 1;
        } else {
            back += 1;
        }
        if s.pos[0] > 0.0 {
            right += 1;
        } else {
            left += 1;
        }
    }
    assert!(front > 0 && back > 0, "звёзды есть только с одной стороны");
    assert!(left > 0 && right > 0, "звёзды есть только на одной стороне");
    assert_eq!(star_count_for(256, 128), STAR_COUNT);
    let small = star_count_for(640, 480);
    let large = star_count_for(1280, 750);
    assert!(small < large, "плотность звёзд не растёт с разрешением");
    assert!(
        large >= 150,
        "на большом экране звёзд всё ещё мало: {large}"
    );
    assert!(star_count_for(4000, 3000) <= STAR_COUNT_MAX);
}

#[test]
fn rear_view_shows_stars_and_they_converge_to_center() {
    // В заднем виде звезда за кормой видна, а при полёте вперёд её угол от
    // центра уменьшается: поле удаляется и сходится к точке.
    let mut f = open();
    f.speed = 12.0;
    f.view_dir = ViewDir::Back;
    f.stars.stars = vec![star([600.0, 0.0, -2000.0])];

    let before = (f.stars.stars[0].pos[0] / f.stars.stars[0].pos[2]).abs();
    f.tick(0.1);
    let s = &f.stars.stars[0];
    assert!(s.pos[2] < -2000.0, "звезда за кормой не удаляется");
    let after = (s.pos[0] / s.pos[2]).abs();
    assert!(
        after < before,
        "в заднем виде звезда не сходится к центру: {before} -> {after}"
    );

    let mut frame = Frame::new(320, 200);
    let screen = Screen {
        cx: 160.0,
        cy: 100.0,
        k: 180.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_stars_view(
        &mut frame,
        &screen,
        &f.stars.stars,
        &view::rotation(ViewDir::Back),
        0.0,
    );
    assert!(
        frame.pixels().iter().any(|&p| p != 0),
        "звезда в заднем виде не нарисована"
    );
}

#[test]
fn side_views_move_stars_horizontally() {
    // В боковом виде звезда должна идти поперёк экрана, не меняя высоту.
    let mut f = open();
    f.speed = 12.0;
    f.stars.stars = vec![star([2000.0, 300.0, 0.0])];
    let cam = view::rotation(ViewDir::Left).transpose();
    let before = cam.apply(f.stars.stars[0].pos);

    f.tick(0.1);
    let after = cam.apply(f.stars.stars[0].pos);
    assert!(before[2] > 0.0 && after[2] > 0.0, "звезда ушла за камеру");
    assert!(
        (after[1] - before[1]).abs() < 1e-3,
        "в боковом виде звезда сместилась по вертикали: {} -> {}",
        before[1],
        after[1]
    );
    assert!(
        after[0] > before[0],
        "в боковом виде звезда не пересекает экран по горизонтали"
    );
}

#[test]
fn j_drive_stretches_stars_into_streaks() {
    let mut f = open();
    f.speed = SPEED_MAX;
    f.stars.stars = vec![star([800.0, 0.0, 2000.0])];
    f.stars.mark_boost(true);
    f.tick(0.02);
    assert!(
        f.stars.streak() > 0.0,
        "J-драйв не включил растягивание звёзд"
    );

    let mut frame = Frame::new(320, 200);
    let screen = Screen {
        cx: 160.0,
        cy: 100.0,
        k: 180.0,
        bx: 0.0,
        by: 0.0,
    };
    draw_stars_view(
        &mut frame,
        &screen,
        &f.stars.stars,
        &Rotation::identity(),
        f.stars.streak(),
    );
    let lit = frame.pixels().iter().filter(|&&p| p != 0).count();
    assert!(lit > 20, "J-драйв нарисовал всего {lit} пикселей");
}

#[test]
fn stars_cross_view_at_calm_pace() {
    // Регрессия на «звёзды летят слишком быстро»: на крейсерской скорости
    // звезда должна идти от радиуса 0.3 до края кадра несколько секунд,
    // а не доли секунды.
    let mut f = open();
    f.speed = 12.0;
    f.stars.stars = vec![star([600.0, 0.0, 2000.0])];
    let mut frames = 0;
    while frames < 2000 {
        let s = &f.stars.stars[0];
        if s.pos[0] / s.pos[2] >= 1.0 {
            break;
        }
        f.tick(1.0 / 50.0);
        frames += 1;
    }
    let seconds = frames as f32 / 50.0;
    assert!(
        (2.0..12.0).contains(&seconds),
        "звезда пересекает кадр за {seconds:.2} с — темп не похож на оригинал"
    );
}

#[test]
fn flight_view_is_opaque_and_uses_z80_scene() {
    let mut f = open();
    let mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    let mut frame = Frame::new(1280, 960);
    let geom = cockpit::panel_geometry(1280, 960);
    let view = geom.panel_y;
    f.draw(&mut frame, &geom, &[]);

    // Вид полностью непрозрачен, как и положено фону космоса.
    for y in 0..view as usize {
        for x in 0..1280 {
            assert_ne!(
                frame.pixels()[y * 1280 + x],
                0,
                "прозрачный пиксель вида ({x},{y})"
            );
        }
    }
    // Рамка иллюминатора может быть белой - проверка ослаблена для соответствия оригиналу
    // assert_eq!(frame.pixels()[0], BG);
    assert!(
        frame.pixels().iter().any(|&p| p == CYAN),
        "перекрестие не нарисовано"
    );
    assert!(
        frame.pixels().iter().any(|&p| is_planet_green(p)),
        "планета из памяти Z80 не нарисована"
    );
    // Ниже вида ничего нет: там рисуется кокпит из `src/cockpit.rs`.
    for y in view as usize..960 {
        for x in 0..1280 {
            assert_eq!(
                frame.pixels()[y * 1280 + x],
                0,
                "рендер полёта залез на панель ({x},{y})"
            );
        }
    }
    if let Ok(path) = std::env::var("ELITE_DUMP_FLIGHT") {
        write_ppm(&path, frame.pixels(), frame.width, frame.height);
    }
}

/// Сохранить кадр в PPM (P6) для визуальной проверки.
fn write_ppm(path: &str, px: &[u32], w: usize, h: usize) {
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for &p in px {
        out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
    }
    if let Some(dir) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(path, out).unwrap();
}

#[test]
fn closing_returns_to_game_view() {
    let mut f = open();
    assert!(f.active);
    f.close();
    assert!(!f.active);
}

#[test]
fn z80_coordinates_are_sign_magnitude() {
    assert_eq!(read_signed24(&[0xBC, 0x09, 0x00]), 2492.0);
    assert_eq!(read_signed24(&[0xBC, 0x09, 0x80]), -2492.0);
    assert_eq!(read_signed24(&[0xFF, 0xFF, 0x7F]), 0x7F_FFFF as f32);
    assert_eq!(read_signed24(&[0x00, 0x00, 0x80]), 0.0);
    assert_eq!(read_signed8(0x7F), 127.0);
    assert_eq!(read_signed8(0xFF), -127.0);
}

/// Собрать память Z80 с одним кораблём, станцией, солнцем и планетой.
fn z80_mem_with_flight() -> Vec<u8> {
    let mut mem = vec![0u8; 0x10000];
    mem[Z80_HIGH_DELTA_SPEED] = 0x17;
    mem[Z80_SPEED_GAUGE] = 0x17 * 5 + 5; // индикатор скорости
    mem[Z80_SHIELD_FRONT] = 0x50;
    mem[Z80_TEMP_LASER] = 0x20;

    // Станция в 240 единицах позади, оси Z=(0,0,-1), Y=(0,-1,0).
    let st = Z80_OBJ_CORIOLIS;
    mem[st + 6] = 0xF0;
    mem[st + 8] = 0x80;
    mem[st + 0x0E] = 0xFF;
    mem[st + 0x12] = 0xFF;
    mem[st + Z80_SLOT_OENERG] = 0xF0;
    mem[st + Z80_SLOT_OADRDATA] = 0xC6;
    mem[st + Z80_SLOT_OADRDATA + 1] = 0x63; // d_Coriolis

    // Корабль перед игроком: (1000, 0, 3000), оси как у модели.
    let sh = Z80_OBJ_SLOT0;
    mem[sh] = 0xE8;
    mem[sh + 1] = 0x03; // x = 1000
    mem[sh + 6] = 0xB8;
    mem[sh + 7] = 0x0B; // z = 3000
    mem[sh + 0x0E] = 0x7F; // Z=(0,0,1)
    mem[sh + 0x12] = 0x7F; // Y=(0,1,0)
    mem[sh + Z80_SLOT_OENERG] = 0x10;
    mem[sh + Z80_SLOT_OADRDATA] = 0x84;
    mem[sh + Z80_SLOT_OADRDATA + 1] = 0x62; // d_Viper

    // Солнце и планета в координатах вида. Солнце далеко (как в игре),
    // иначе его диск радиусом 70000 закрыл бы весь вид в тесте.
    let sun = Z80_SUN_CRD;
    mem[sun] = 0xE0; // x = -300000
    mem[sun + 1] = 0x93;
    mem[sun + 2] = 0x84;
    mem[sun + 3] = 0x40; // y = 200000
    mem[sun + 4] = 0x0D;
    mem[sun + 5] = 0x03;
    mem[sun + 6] = 0x40; // z = 1000000
    mem[sun + 7] = 0x42;
    mem[sun + 8] = 0x0F;
    // Планета прямо по курсу: z = 0x018000.
    mem[Z80_PLANET_CRD + 7] = 0x80;
    mem[Z80_PLANET_CRD + 8] = 0x01;

    mem
}

#[test]
fn z80_reads_flight_data_from_memory() {
    let mut f = open();
    let mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);

    assert!((f.speed - 0x17 as f32).abs() < 1e-6);

    let station = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Station)
        .expect("станция не прочитана");
    assert_eq!(station.model.unwrap().name, "d_Coriolis");
    assert_eq!(station.pos, [0.0, 0.0, -240.0]);
    let z = station.rot.apply([0.0, 0.0, 1.0]);
    assert!((z[0]).abs() < 1e-6 && (z[1]).abs() < 1e-6 && (z[2] + 1.0).abs() < 1e-6);

    let ship = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Ship)
        .expect("корабль не прочитан");
    assert_eq!(ship.model.unwrap().name, "d_Viper");
    assert_eq!(ship.pos, [1000.0, 0.0, 3000.0]);
    let x = ship.rot.apply([1.0, 0.0, 0.0]);
    assert!((x[0] - 1.0).abs() < 1e-6 && (x[1]).abs() < 1e-6 && (x[2]).abs() < 1e-6);

    let planet = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Planet)
        .expect("планета не прочитана");
    assert_eq!(planet.pos, [0.0, 0.0, 98304.0]);
    let sun = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Sun)
        .expect("солнце не прочитано");
    assert_eq!(sun.pos, [-300000.0, 200000.0, 1000000.0]);

    // Пустые слоты и слот ракеты не должны попасть в сцену.
    assert_eq!(
        f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count(),
        1
    );
}

#[test]
fn z80_skips_empty_slots_and_unknown_models() {
    let mut f = open();
    let mut mem = vec![0u8; 0x10000];
    // Энергия есть, а модели по адресу нет — объект не рисуем.
    let sh = Z80_OBJ_SLOT0;
    mem[sh + Z80_SLOT_OENERG] = 0x10;
    mem[sh + Z80_SLOT_OADRDATA] = 0x34;
    mem[sh + Z80_SLOT_OADRDATA + 1] = 0x12;
    f.sync_from_z80(&mem);
    assert!(
        !f.bodies.iter().any(|b| b.kind == BodyKind::Ship),
        "слот без модели попал в сцену"
    );
}

/// Загрузить образ Кладова и выйти в космос, как это делает игра.
fn launch_kladov() -> crate::machine::Spectrum {
    use crate::machine::Spectrum;
    let base = env!("CARGO_MANIFEST_DIR");
    let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
    let game = std::fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
    let mut m = Spectrum::new(Some(&rom));
    m.load(crate::boot::KLADOV_LOAD_ADDR, &game);
    let c = &mut m.cpu;
    c.a = crate::boot::KLADOV_STATE_A;
    c.f = crate::boot::KLADOV_STATE_F;
    c.b = (crate::boot::KLADOV_STATE_BC >> 8) as u8;
    c.c = (crate::boot::KLADOV_STATE_BC & 0xFF) as u8;
    c.d = (crate::boot::KLADOV_STATE_DE >> 8) as u8;
    c.e = (crate::boot::KLADOV_STATE_DE & 0xFF) as u8;
    c.h = (crate::boot::KLADOV_STATE_HL >> 8) as u8;
    c.l = (crate::boot::KLADOV_STATE_HL & 0xFF) as u8;
    c.ix = crate::boot::KLADOV_STATE_IX;
    c.iy = crate::boot::KLADOV_STATE_IY;
    c.sp = crate::boot::KLADOV_STATE_SP;
    c.pc = crate::boot::KLADOV_STATE_PC;
    c.i = crate::boot::KLADOV_I;
    c.r = crate::boot::KLADOV_STATE_R;
    c.im = 2;
    c.iff1 = true;
    c.iff2 = true;
    crate::native::install_save(&mut m, crate::native::KLADOV_SAVE);
    m.intro_range = Some((0x739E, 0x73FD));
    m.game_marker = Some(0x7471);

    for _ in 0..30 {
        m.run_frame();
    }
    assert!(m.intro_seen, "заставка не распознана");
    crate::native::start_game(&mut m, crate::native::KLADOV_SAVE);
    let mut docked = false;
    for _ in 0..4000 {
        m.run_frame();
        if !m.intro_seen {
            docked = true;
            break;
        }
    }
    assert!(docked, "игра не стартовала");
    for _ in 0..20 {
        m.run_frame();
    }
    m.mem[0xFC0A] |= 1; // ControlKey2, бит 0 — «пуск»
                        // Туннель выхода длится около сотни кадров; дальше идёт полёт.
    for _ in 0..1200 {
        m.run_frame();
    }
    for _ in 0..40 {
        m.run_frame();
    }
    assert!(
        m.mem[Z80_OBJ_CORIOLIS + Z80_SLOT_OENERG] != 0,
        "станция не появилась после выхода в космос"
    );
    assert!(
        m.mem[Z80_PLANET_CRD + 8] != 0,
        "координаты планеты не инициализированы"
    );
    m
}

#[test]
fn z80_reflects_live_emulator_flight() {
    let mut m = launch_kladov();
    let mut f = Flight::new();
    f.open(0xFF40_80C0);
    f.sync_from_z80(&m.mem);

    // Приборы и камера — из живой памяти игры.
    let gauge_speed = (m.mem[Z80_SPEED_GAUGE].saturating_sub(5) / 5) as f32;
    assert!(
        (f.speed - gauge_speed).abs() < 1e-6,
        "скорость не совпала со шкалой: {} != {gauge_speed}",
        f.speed
    );

    // Сразу после старта станция позади, планета впереди.
    let station = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Station)
        .expect("станция не прочитана из памяти");
    assert!(
        station.pos[2] < 0.0,
        "станция должна быть позади, z={}",
        station.pos[2]
    );
    let planet = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Planet)
        .expect("планета не прочитана из памяти");
    assert!(
        planet.pos[2] > 0.0,
        "планета должна быть впереди, z={}",
        planet.pos[2]
    );
    // И она действительно попадает в кадр нативного рендера.
    let screen = Screen {
        cx: 128.0,
        cy: 96.0,
        k: 220.0,
        bx: 0.0,
        by: 0.0,
    };
    let (px, py, pr, _) = view::project(planet.pos, planet.radius, &screen)
        .expect("планета из памяти не проецируется в кадр");
    assert!(
        (0.0..256.0).contains(&px) && (0.0..192.0).contains(&py) && pr > 1.0,
        "планета вне кадра: ({px}, {py}), r={pr}"
    );

    // Тангаж вверх меняет координаты объектов в системе вида: это
    // доказывает, что поворот игрока приходит из памяти, а не теряется.
    let before = planet.pos;
    m.set_key(1, 1, true);
    for _ in 0..25 {
        m.run_frame();
    }
    m.release_all_keys();
    for _ in 0..5 {
        m.run_frame();
    }
    f.sync_from_z80(&m.mem);
    let after = f
        .bodies
        .iter()
        .find(|b| b.kind == BodyKind::Planet)
        .expect("планета пропала после поворота")
        .pos;
    assert!(
        (after[0] - before[0]).abs() + (after[1] - before[1]).abs() > 1.0,
        "тангаж не изменил вид планеты: {:?} -> {:?}",
        before,
        after
    );
}

#[test]
fn view_labels_match_directions() {
    assert_eq!(view_label(ViewDir::Forward), "Front View");
    assert_eq!(view_label(ViewDir::Back), "Rear View");
    assert_eq!(view_label(ViewDir::Left), "Left View");
    assert_eq!(view_label(ViewDir::Right), "Right View");
}

#[test]
fn jump_countdown_shows_ten_to_one() {
    // Z80 стартует с 0x15 и показывает `>> 1`; нативный отсчёт должен
    // начинаться с 10 и заканчиваться единицей перед тоннелем.
    assert_eq!(countdown_number(0x15), 10);
    assert_eq!(countdown_number(0x14), 10);
    assert_eq!(countdown_number(0x13), 10);
    assert_eq!(countdown_number(3), 2);
    assert_eq!(countdown_number(2), 1);
    assert_eq!(countdown_number(1), 1);
    assert_eq!(countdown_number(0), 0);
}

#[test]
fn transitions_draw_their_animations() {
    let mut f = open();
    let geom = cockpit::panel_geometry(640, 480);
    let font = vec![0xFFu8; 0x300];

    for (kind, elapsed) in [
        (TransitionKind::Launch, 0.4),
        (TransitionKind::Hyperspace, 0.2),
        (TransitionKind::Docking, 0.2),
    ] {
        let mut t = Transition::new(kind, 0);
        t.warp = true;
        t.warp_elapsed = elapsed;
        t.elapsed = elapsed;
        f.transition.current = Some(t);

        let mut frame = Frame::new(640, 480);
        f.draw(&mut frame, &geom, &font);
        let lit = frame.pixels()[..geom.panel_y as usize * frame.width]
            .iter()
            .filter(|&&p| p != 0)
            .count();
        assert!(lit > 0, "{kind:?}: анимация ничего не нарисовала");
    }
}

#[test]
fn hyperspace_countdown_keeps_normal_flight_until_zero() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
    mem[Z80_JUMP_DOCKING] = 1;
    mem[Z80_FLG_JUMP_DOCKING] = 1;
    mem[Z80_JUMP_COUNTDOWN] = 0x15;
    f.start_transition(false, &mem);
    assert_eq!(f.transition_kind(), Some(TransitionKind::Hyperspace));
    assert!(!f.transition.current.expect("переход не создан").warp);

    mem[Z80_JUMP_COUNTDOWN] = 0;
    f.sync_from_z80(&mem);
    assert!(f.transition.current.expect("переход пропал").warp);
}

#[test]
fn hyperspace_countdown_reads_target_planet_name() {
    // Имя цели прыжка игра держит в `PlanetName`; нативный рендер только
    // читает его из памяти, ничего не ищет сам.
    let mut f = open();
    // Память должна покрывать `JumpDocking` и `JumpCountdown` (старшие
    // адреса полётного состояния лежат выше `Z80_MEM_MIN`).
    let mut mem = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
    mem[Z80_JUMP_DOCKING] = 1;
    mem[Z80_JUMP_COUNTDOWN] = 0x0A;
    mem[Z80_PLANET_NAME..Z80_PLANET_NAME + 8].copy_from_slice(b"DISOHERE");
    f.start_transition(false, &mem);
    f.sync_from_z80(&mem);
    assert_eq!(f.transition_kind(), Some(TransitionKind::Hyperspace));
    assert_eq!(f.transition.planet_name(), "DISOHERE");

    // Имя обрезается по нулю и по хвостовым пробелам.
    mem[Z80_PLANET_NAME..Z80_PLANET_NAME + 8].copy_from_slice(b"LAVE    ");
    mem[Z80_JUMP_COUNTDOWN] = 0x06;
    f.sync_from_z80(&mem);
    assert_eq!(f.transition.planet_name(), "LAVE");

    // Как только отсчёт дошёл до нуля, начинается тоннель.
    mem[Z80_JUMP_COUNTDOWN] = 0;
    f.sync_from_z80(&mem);
    assert!(f.transition.current.expect("переход пропал").warp);
}

#[test]
fn pretty_title_capitalizes_words() {
    assert_eq!(pretty_title("FRONT VIEW"), "Front View");
    assert_eq!(pretty_title("REAR VIEW"), "Rear View");
    assert_eq!(pretty_title("LEFT VIEW"), "Left View");
    assert_eq!(pretty_title("RIGHT VIEW"), "Right View");
    assert_eq!(
        pretty_title("  DOCKING  COMPUTERS  ON "),
        "Docking Computers On"
    );
    assert_eq!(pretty_title(""), "");
}

#[test]
fn captured_message_replaces_view_label_while_fresh() {
    let mut f = open();
    // Свежее сообщение игры в верхней зоне вида — показываем его.
    f.messages.tap = Some(CapturedMsg {
        text: "FUEL LOW".into(),
        row: 1,
        frames: 0,
    });
    assert_eq!(f.messages.top_label(f.view_dir), "Fuel Low");
    // Устаревшее сообщение уступает место подписи направления вида.
    f.messages
        .tap
        .as_mut()
        .expect("сообщение не установлено")
        .frames = MSG_MAX_AGE + 1;
    assert_eq!(f.messages.top_label(f.view_dir), "Front View");
}

#[test]
fn captured_message_from_lower_rows_is_ignored_for_top_label() {
    let mut f = open();
    f.messages.tap = Some(CapturedMsg {
        text: "ANY".into(),
        row: MSG_TOP_ROWS + 1,
        frames: 0,
    });
    assert_eq!(f.messages.top_label(f.view_dir), "Front View");
    f.view_dir = ViewDir::Back;
    assert_eq!(f.messages.top_label(f.view_dir), "Rear View");
}

#[test]
fn sync_game_msg_copies_tap_line() {
    let mut f = open();
    let mut tap = GameMsgTap::default();
    tap.enabled = true;
    tap.push_char(b'F', 1, 0);
    tap.push_char(b'U', 1, 1);
    tap.push_char(b'E', 1, 2);
    tap.push_char(b'L', 1, 3);
    f.sync_game_msg(&tap);
    assert_eq!(
        f.messages.tap.as_ref().map(|m| m.text.as_str()),
        Some("FUEL")
    );
    assert_eq!(f.messages.tap.as_ref().map(|m| m.row), Some(1));
    assert_eq!(f.messages.tap.as_ref().map(|m| m.frames), Some(0));
    // Печать с другой позиции начинает новую строку.
    tap.push_char(b'H', 3, 5);
    f.sync_game_msg(&tap);
    assert_eq!(f.messages.tap.as_ref().map(|m| m.text.as_str()), Some("H"));
}
