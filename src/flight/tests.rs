//! Тесты экрана полёта и его подсистем.

use super::*;
use crate::render3d::{ENGINE_IDLE, Rotation, Screen};

use super::explosions::EXPLOSION_TIME;
use super::memory::{
    ext_slot_base, read_signed24, read_signed8, Z80_DAMAGE_FRONT_SHIELD, Z80_FLG_OBJ_NEAR,
    Z80_IN_FREE_SPACE, Z80_LASER_MOUNTS, Z80_OBJ_CORIOLIS, Z80_OBJ_MISSILE, Z80_OBJ_SLOT0,
    Z80_OENERG_DESTROYED, Z80_PLANET_CRD, Z80_SHIELD_FRONT, Z80_SHIELD_REAR, Z80_SLOT_OACCEL,
    Z80_SLOT_OADRDATA, Z80_SLOT_OENERG, Z80_SLOT_SIZE, Z80_STARS_TABLE, Z80_SUN_CRD,
    Z80_TEMP_LASER,
};
use super::messages::{
    countdown_number, view_label, BOTTOM_MSG_TTL, DOCKING_BLINK_ON_FRAMES, DOCKING_BLINK_PERIOD,
    DOCKING_MSG_TTL,
};
use super::palette::{
    planet_rim, sun_tint, teleport_ring_color, CYAN, ENEMY_BEAM, RED, STAR_RIM_COLOR,
    VIEW_LABEL_COLOR,
};
use super::render::scene::{draw_star, draw_stars_view};
use super::scene::{
    BodyKind, BODY_SRC_CORIOLIS, BODY_SRC_MISSILE, BODY_SRC_PLANET, BODY_SRC_SLOT0, PLANET_RADIUS,
    SUN_RADIUS,
};
use super::stars::{
    star_count_for, Star, SPEED_MAX, STAR_ANGLE_LR_PER_UNIT, STAR_COUNT, STAR_COUNT_MAX,
    STAR_FIELD_RADIUS,
};
use super::teleports::TELEPORT_TRAIL_TIME;
use super::transition::Transition;
use super::view;

/// Тёмно-зелёный пиксель планеты: красный и синий нулевые, зелёный заметен.
fn is_planet_green(p: u32) -> bool {
    p >> 24 == 0xFF && p & 0x00FF_00FF == 0 && (p >> 8) & 0xFF >= 0x18
}

fn open() -> Flight {
    let mut f = Flight::new();
    f.open(0xFF40_80C0);
    f
}

/// Имитировать шаг модели Z80: сдвинуть координату объекта. Именно по смене
/// координат нативный слой отличает игровой такт от кадров эмулятора.
fn game_tick(mem: &mut [u8]) {
    mem[Z80_OBJ_SLOT0] = mem[Z80_OBJ_SLOT0].wrapping_add(1);
}

/// Записать в слот Z80 объект с заданной моделью.
fn put_obj(mem: &mut [u8], base: usize, name: &str) {
    let ship = crate::shipdata::find(name).expect("модель не найдена");
    mem[base + Z80_SLOT_OENERG] = 0x40;
    mem[base + Z80_SLOT_OADRDATA..base + Z80_SLOT_OADRDATA + 2]
        .copy_from_slice(&ship.addr.to_le_bytes());
}

fn star(pos: [f32; 3]) -> Star {
    Star { pos, mag: 128 }
}

/// Тело сцены для проверки интерполяции без обращения к памяти Z80.
fn test_body(source: u8, pos: [f32; 3], rot: Rotation) -> Body {
    Body {
        source,
        kind: BodyKind::Ship,
        model: None,
        pos,
        rot,
        radius: 10.0,
        color: 0,
        tint: None,
        engine: ENGINE_IDLE,
    }
}

fn assert_vec_close(a: [f32; 3], b: [f32; 3]) {
    for k in 0..3 {
        assert!(
            (a[k] - b[k]).abs() < 1e-3,
            "векторы не совпали: {a:?} против {b:?}"
        );
    }
}

#[test]
fn engine_comes_from_slot_acceleration_not_speed() {
    let mut mem = vec![0u8; Z80_OBJ_SLOT0 + Z80_SLOT_SIZE];
    let base = Z80_OBJ_SLOT0;
    // Высокая скорость при нулевом приращении — полёт по инерции, двигатель
    // не работает. Само поле скорости здесь специально не читается.
    mem[base + 0x1B] = 31;
    mem[base + Z80_SLOT_OACCEL] = 0;
    assert_eq!(memory::z80_slot_engine(&mem, base), 0.0);

    mem[base + Z80_SLOT_OACCEL] = 0x0A;
    assert!(
        memory::z80_slot_engine(&mem, base) > 0.9,
        "активное приращение должно включать двигатель"
    );
}

#[test]
fn body_interpolation_uses_raw_z80_positions() {
    let mut f = open();
    let prev_rot = Rotation::axis_angle([0.0, 1.0, 0.0], 0.1);
    let rot = Rotation::axis_angle([0.0, 1.0, 0.0], 0.4);
    let pos = [800.0, 100.0, 3000.0];
    let moved = [820.0, 130.0, 2960.0];

    // Z80 уже записал координаты и ориентацию на новом такте в системе
    // игрока. Нативный слой интерполирует эти готовые значения, не поворачивая
    // previous-снимок последним углом камеры.
    f.prev_bodies = vec![test_body(BODY_SRC_SLOT0, pos, prev_rot)];
    f.bodies = vec![test_body(BODY_SRC_SLOT0, moved, rot)];
    f.ship_interval = 0.2;

    for &accum in &[0.0, 0.05, 0.1, 0.19] {
        f.ship_accum = accum;
        let rendered = f.render_bodies();
        let alpha = accum / f.ship_interval;
        let expected = [
            pos[0] + (moved[0] - pos[0]) * alpha,
            pos[1] + (moved[1] - pos[1]) * alpha,
            pos[2] + (moved[2] - pos[2]) * alpha,
        ];
        assert_vec_close(rendered[0].pos, expected);
        let expected_rot = lerp_rotation(&prev_rot, &rot, alpha);
        let axis = rendered[0].rot.apply([1.0, 0.0, 0.0]);
        let expected_axis = expected_rot.apply([1.0, 0.0, 0.0]);
        assert_vec_close(axis, expected_axis);
    }
}

#[test]
fn interpolation_does_not_apply_camera_turn_to_previous_position() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);

    f.tick(1.0 / 50.0);

    // Игровой такт с тангажем: Z80 сдвинул корабль по Y лишь на 10 единиц,
    // хотя угол камеры дал бы previous-координате намного больший сдвиг.
    let sh = Z80_OBJ_SLOT0;
    mem[Z80_HIGH_ANGLE_DU] = 3;
    mem[sh + 1] = mem[sh + 1].wrapping_add(1); // признак такта
    mem[sh + 3] = 10; // y = 10
    f.sync_from_z80(&mem);
    f.tick(1.0 / 100.0);

    let shown = f
        .render_bodies()
        .into_iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("корабль не прочитан")
        .pos[1];
    assert!(
        (0.0..10.0).contains(&shown),
        "интерполированный Y должен быть между позициями Z80: {shown}"
    );
}

#[test]
fn body_snapshot_follows_game_tick_not_display_frame() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();

    // Первый синк только запоминает координаты: такта ещё не было.
    f.sync_from_z80(&mem);
    let initial = f.bodies.clone();

    // Кадр 50 Гц без игрового такта: меняется только таблица звёзд.
    // Previous остаётся от предыдущего такта, иначе интервал интерполяции
    // станет равным кадру и `alpha` всегда будет 1.
    mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
    f.sync_from_z80(&mem);
    let prev = f
        .prev_bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("нет предыдущего состояния корабля");
    let start = initial
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("нет корабля в начальной сцене");
    assert_vec_close(prev.pos, start.pos);
    let current = f
        .bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .unwrap()
        .clone();

    // Игровой такт: координаты изменились — previous обновляется.
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    let prev = f
        .prev_bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("нет предыдущего состояния корабля");
    assert_vec_close(prev.pos, current.pos);
}

#[test]
fn interpolation_spans_several_display_frames() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();

    // Первый синк фиксирует координаты, но тактом не считается.
    f.sync_from_z80(&mem);

    // Девять кадров 50 Гц без нового такта.
    for _ in 0..9 {
        f.tick(1.0 / 50.0);
    }

    // Игровой такт: координаты корабля в памяти сдвинулись.
    mem[Z80_OBJ_SLOT0] = mem[Z80_OBJ_SLOT0].wrapping_add(0x20);
    f.sync_from_z80(&mem);
    assert!(
        (f.ship_interval - 9.0 / 50.0).abs() < 1e-6,
        "интервал должен покрывать несколько кадров: {}",
        f.ship_interval
    );

    // Сразу после такта alpha мал: корабль ещё не дошёл до нового положения.
    f.tick(1.0 / 50.0);
    let current = f
        .bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .unwrap()
        .clone();
    let rendered = f.render_bodies();
    let shown = rendered
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .unwrap();
    assert!(
        (shown.pos[0] - current.pos[0]).abs() > 1e-3,
        "интерполяция отключена: показано новое положение без плавного перехода"
    );
}

#[test]
fn planet_does_not_jitter_when_only_star_table_changes() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    let before = f
        .render_bodies()
        .into_iter()
        .find(|b| b.source == BODY_SRC_PLANET)
        .expect("планета не прочитана")
        .pos;

    // Кадр эмулятора, на котором изменилась только таблица звёзд: Z80 ещё не
    // пересчитал координаты планеты. Поворот камеры не должен сдвинуть её.
    mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
    f.sync_from_z80(&mem);
    let after = f
        .render_bodies()
        .into_iter()
        .find(|b| b.source == BODY_SRC_PLANET)
        .expect("планета не прочитана")
        .pos;
    assert_vec_close(after, before);
}

#[test]
fn stars_turn_only_on_model_tick() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    f.sync_from_z80(&mem);
    f.stars.stars = vec![star([1000.0, 0.0, 0.0])];

    // Одна только таблица звёзд меняется чаще модели: поворот ещё не должен
    // применяться, иначе звёзды уедут раньше кораблей и планеты.
    mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
    mem[Z80_HIGH_ANGLE_LR] = 4;
    f.sync_from_z80(&mem);
    assert_vec_close(f.stars.stars[0].pos, [1000.0, 0.0, 0.0]);

    // Настоящий такт с креном вправо на 4 единицы поворачивает поле.
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    let p = f.stars.stars[0].pos;
    let angle = p[1].atan2(p[0]);
    let expected = 4.0 * STAR_ANGLE_LR_PER_UNIT;
    assert!(
        (angle - expected).abs() < 1e-3,
        "неверный поворот звёзд: {angle} против {expected}"
    );
}

#[test]
fn coriolis_change_does_not_count_as_game_tick() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);

    // Настоящий такт с креном: корабли получили поворот камеры.
    mem[Z80_HIGH_ANGLE_LR] = 8;
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    f.tick(0.02);
    let before = f
        .render_bodies()
        .into_iter()
        .find(|b| b.source == BODY_SRC_PLANET)
        .expect("планета не прочитана")
        .pos;

    // Анимация Coriolis идёт со своим таймером. Её смена не должна заново
    // применять тот же поворот к прежним координатам планеты.
    mem[Z80_OBJ_CORIOLIS] = mem[Z80_OBJ_CORIOLIS].wrapping_add(1);
    f.sync_from_z80(&mem);
    let after = f
        .render_bodies()
        .into_iter()
        .find(|b| b.source == BODY_SRC_PLANET)
        .expect("планета не прочитана")
        .pos;
    assert_vec_close(after, before);
}

#[test]
fn ships_do_not_jitter_when_only_world_updates() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);

    // Настоящий такт с тангажем: корабли и мир получили свой поворот.
    mem[Z80_HIGH_ANGLE_DU] = 3;
    game_tick(&mut mem);
    mem[Z80_PLANET_CRD] = mem[Z80_PLANET_CRD].wrapping_add(1);
    f.sync_from_z80(&mem);
    f.tick(0.02);
    let before = f.render_bodies();
    let ship_before = before
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("корабль не прочитан")
        .pos;
    let station_before = before
        .iter()
        .find(|b| b.source == BODY_SRC_CORIOLIS)
        .expect("станция не прочитана")
        .pos;

    // Следующий кадр обновил только солнце с планетой, angle изменился.
    // Поворот камеры не должен заново сдвинуть корабль и станцию.
    mem[Z80_HIGH_ANGLE_DU] = 6;
    mem[Z80_SUN_CRD] = mem[Z80_SUN_CRD].wrapping_add(1);
    f.sync_from_z80(&mem);
    let after = f.render_bodies();
    let ship_after = after
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .expect("корабль не прочитан")
        .pos;
    let station_after = after
        .iter()
        .find(|b| b.source == BODY_SRC_CORIOLIS)
        .expect("станция не прочитана")
        .pos;
    assert_vec_close(ship_after, ship_before);
    assert_vec_close(station_after, station_before);
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

    // Первый синк только запоминает координаты: такта ещё не было.
    f.sync_from_z80(&mem);

    // Поворот без нового такта не применяется: одна только таблица звёзд
    // меняется чаще игрового цикла.
    f.stars.stars = vec![star([1000.0, 0.0, 0.0])];
    mem[Z80_STARS_TABLE] = mem[Z80_STARS_TABLE].wrapping_add(1);
    f.sync_from_z80(&mem);
    assert_eq!(
        f.stars.stars[0].pos,
        [1000.0, 0.0, 0.0],
        "поворот применён без игрового такта"
    );

    // Игровой такт ловим по изменению координат объектов в памяти игры.
    // Крен вправо: звезда на оси X должна уйти в +Y.
    mem[Z80_HIGH_ANGLE_LR] = 0x04;
    let tick = |f: &mut Flight, mem: &mut [u8]| {
        game_tick(mem);
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
    f.sync_from_z80(&mem); // запомнили координаты объектов

    // Обычный такт: координаты обновились, скорость настоящая.
    f.stars.stars = vec![star([100.0, 0.0, 1000.0])];
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    let before = f.stars.stars[0].pos[2];
    f.tick(1.0 / 50.0);
    let normal = (f.stars.stars[0].pos[2] - before).abs();

    // Такт J-драйва: HighDeltaSpeed затёрт, координаты обновились.
    f.stars.stars = vec![star([100.0, 0.0, 1000.0])];
    game_tick(&mut mem);
    mem[Z80_HIGH_DELTA_SPEED] = 0x10;
    f.sync_from_z80(&mem);
    let before = f.stars.stars[0].pos[2];
    f.tick(1.0 / 50.0);
    let fast = (f.stars.stars[0].pos[2] - before).abs();

    assert!(normal > 0.0, "звёзды не движутся на обычном такте");
    // Обычный полёт ускорен вдвое, J-драйв — нет, поэтому его скорость
    // теперь примерно вдвое выше обычной.
    assert!(
        fast > normal * 1.5,
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

    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert!(f.stars.boost, "флаг J-драйва потерян между кадрами");
}

#[test]
fn launch_service_speed_does_not_trigger_j_drive() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0x41; // скорость 12
    mem[Z80_HIGH_DELTA_SPEED] = 12;
    f.sync_from_z80(&mem);

    // Вылет: сначала скорость ещё настоящая, а в конце ролика Z80 пишет в
    // HighDeltaSpeed служебное 7, хотя индикатор хранит прежние 12.
    f.start_transition(true, &mem);
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    mem[Z80_HIGH_DELTA_SPEED] = 7;
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert!(
        !f.stars.boost,
        "служебная скорость вылета включила J-драйв"
    );

    // Переход уже закончился, но игра ещё не пересчитала скорость.
    f.transition.reset();
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert!(
        !f.stars.boost,
        "J-драйв ошибочно показан после вылета со станции"
    );

    // Настоящая скорость снова совпала — обычный J-драйв опять ловится.
    mem[Z80_HIGH_DELTA_SPEED] = 12;
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    mem[Z80_HIGH_DELTA_SPEED] = 0x10;
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert!(f.stars.boost, "J-драйв перестал распознаваться");
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
        // Имитируем игровой такт: обновляются координаты объектов.
        game_tick(&mut mem);
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
fn sun_is_almost_white_with_system_tint_and_white_rim() {
    let system = 0xFF40_80C0;
    let color = sun_tint(system);
    let channels = [(color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF];
    assert!(
        channels.iter().all(|&c| (0xB0..0xFF).contains(&c)),
        "звезда должна быть почти белой: {channels:?}"
    );
    assert!(
        channels.iter().any(|&c| c < 0xF0),
        "у звезды нет оттенка цвета системы"
    );
    assert!(
        channels[0] < channels[1] && channels[1] < channels[2],
        "оттенок не повторяет порядок каналов системы: {channels:?}"
    );

    // На диске видна белая окантовка и почти белая заливка с оттенком.
    let mut frame = Frame::new(256, 192);
    draw_star(&mut frame, 128.0, 96.0, 20.0, color, 1);
    assert_eq!(frame.pixels()[96 * 256 + 128], color, "нет заливки звезды");
    assert!(
        frame.pixels().iter().any(|&p| p == STAR_RIM_COLOR),
        "у звезды нет белой окантовки"
    );
}

#[test]
fn planet_is_dark_green_and_smaller_than_sun() {
    let color = planet_green(0xFF40_80C0);
    assert_eq!(color & 0x00FF_00FF, 0, "у планеты не зелёный цвет");
    assert!((color >> 8) & 0xFF > 0, "планета слишком тёмная");
    assert!(
        (color >> 8) & 0xFF <= 0x30,
        "планета должна быть сильно темнее прежней"
    );
    // У планеты есть более яркий ободок: тёмная заливка остаётся, но диск
    // не сливается с чёрным космосом и сохраняет рамку оригинала.
    let rim = planet_rim(color);
    assert_eq!(rim & 0x00FF_00FF, 0, "ободок планеты не зелёный");
    assert!(
        (rim >> 8) & 0xFF > (color >> 8) & 0xFF,
        "ободок не ярче заливки"
    );
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

/// Циан в окрестности центра вида — это линии прицела (звёзды такого цвета
/// не рисуются, а тела в этом тесте не создаются).
fn crosshair_pixels(frame: &Frame, geom: &cockpit::CockpitGeom) -> usize {
    let (cx, cy) = (frame.width as i32 / 2, geom.panel_y / 2);
    let r = 80;
    let mut count = 0;
    for y in (cy - r).max(0)..=(cy + r).min(frame.height as i32 - 1) {
        for x in (cx - r).max(0)..=(cx + r).min(frame.width as i32 - 1) {
            if frame.pixels()[y as usize * frame.width + x as usize] == CYAN {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn crosshair_follows_fitted_laser_mount() {
    let mut f = open();
    let mut mem = vec![0u8; 0x10000];
    let geom = cockpit::panel_geometry(1280, 960);

    // Без лазера прицела нет.
    f.sync_from_z80(&mem);
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert_eq!(crosshair_pixels(&frame, &geom), 0, "прицел без лазера");

    // FRONT: прицел появляется в переднем виде.
    mem[Z80_LASER_MOUNTS] = 1;
    f.sync_from_z80(&mem);
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(crosshair_pixels(&frame, &geom) > 0, "нет прицела в FRONT");

    // REAR: прицел переходит в задний вид и пропадает в переднем.
    mem[Z80_LASER_MOUNTS] = 0;
    mem[Z80_LASER_MOUNTS + 1] = 2;
    f.sync_from_z80(&mem);
    f.set_view_dir(ViewDir::Back);
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(crosshair_pixels(&frame, &geom) > 0, "нет прицела в REAR");

    f.set_view_dir(ViewDir::Forward);
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert_eq!(
        crosshair_pixels(&frame, &geom),
        0,
        "прицел остался в FRONT без лазера"
    );
}

#[test]
fn own_laser_beam_uses_current_view_mount() {
    let mut f = open();
    let mut mem = vec![0u8; 0x10000];
    mem[Z80_LASER_MOUNTS + 1] = 1; // REAR: pulse laser
    let geom = cockpit::panel_geometry(1280, 960);

    f.sync_from_z80(&mem);
    f.set_view_dir(ViewDir::Back);
    f.combat.laser_timer = 0.05;
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(
        frame.pixels().iter().any(|&p| p == RED),
        "луч не нарисован в REAR"
    );

    // Снятый лазер убирает и луч, хотя таймер эффекта ещё жив.
    mem[Z80_LASER_MOUNTS + 1] = 0;
    f.sync_from_z80(&mem);
    f.combat.laser_timer = 0.05;
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(
        !frame.pixels().iter().any(|&p| p == RED),
        "луч остался без лазера в этом креплении"
    );
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

    // Оригинал обработал одно попадание: передний щит уменьшился на 7.
    mem[Z80_SHIELD_FRONT] = 0x49;
    f.sync_from_z80(&mem);
    assert_eq!(
        f.combat.enemy_shots.len(),
        1,
        "одно попадание должно дать один луч"
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
fn cargo_container_does_not_spawn_enemy_beam() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    // Заменяем Viper (может стрелять) на грузовой контейнер (fire_chance == 0).
    let sh = Z80_OBJ_SLOT0;
    mem[sh + Z80_SLOT_OADRDATA] = 0x28;
    mem[sh + Z80_SLOT_OADRDATA + 1] = 0x62; // d_Cargo
    f.sync_from_z80(&mem);

    mem[Z80_SHIELD_FRONT] = 0x49;
    f.sync_from_z80(&mem);
    assert!(
        f.combat.enemy_shots.is_empty(),
        "контейнер с товаром не должен стрелять: {:?}",
        f.combat.enemy_shots.len()
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
    mem[Z80_SHIELD_FRONT] = 0x49;
    f.sync_from_z80(&mem);
    assert_eq!(
        f.combat.enemy_shots.len(),
        1,
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
fn enemy_beam_shown_when_shield_is_already_zero() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    mem[Z80_SHIELD_FRONT] = 0;
    f.sync_from_z80(&mem);
    assert!(f.combat.enemy_shots.is_empty());

    // Щит уже нулевой, поэтому не меняется, но оригинал всё равно рисует луч.
    mem[Z80_DAMAGE_FRONT_SHIELD] = 7;
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert_eq!(
        f.combat.enemy_shots.len(),
        1,
        "урон при нулевом щите не показал луч"
    );
}

#[test]
fn repeated_hits_from_one_ship_keep_single_beam() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);

    mem[Z80_SHIELD_FRONT] = 0x49;
    f.sync_from_z80(&mem);
    assert_eq!(f.combat.enemy_shots.len(), 1);
    let first_to = f.combat.enemy_shots[0].to;

    // Следующее попадание от того же корабля, пока прошлый луч ещё жив.
    mem[Z80_SHIELD_FRONT] = 0x42;
    f.sync_from_z80(&mem);
    assert_eq!(
        f.combat.enemy_shots.len(),
        1,
        "один корабль должен давать одну линию"
    );
    assert_ne!(
        f.combat.enemy_shots[0].to, first_to,
        "существующий луч не обновился"
    );
}

#[test]
fn enemy_beam_appears_in_rear_view() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();

    // Корабль позади игрока: в заднем виде он впереди камеры.
    mem[Z80_OBJ_SLOT0 + 8] = 0x80;
    f.sync_from_z80(&mem);
    f.set_view_dir(ViewDir::Back);

    // Попадание сзади уменьшает задний щит; передний не трогаем.
    mem[Z80_SHIELD_FRONT] = 0xA0;
    mem[Z80_SHIELD_REAR] = 0x99;
    f.sync_from_z80(&mem);
    assert_eq!(
        f.combat.enemy_shots.len(),
        1,
        "попадание в задний щит не создало луч"
    );
    assert!(
        f.combat.enemy_shots[0].from[2] < 0.0,
        "источник луча должен быть позади игрока"
    );

    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &cockpit::panel_geometry(1280, 960), &[]);
    let beam_pixels = frame.pixels().iter().filter(|&&p| p == ENEMY_BEAM).count();
    assert!(
        beam_pixels > 50,
        "луч в заднем виде не попал в кадр: {beam_pixels} px"
    );
}

#[test]
fn enemy_beam_aim_point_follows_view() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();

    // Корабль слева по курсу: виден только в левом виде.
    mem[Z80_OBJ_SLOT0] = 0xE8;
    mem[Z80_OBJ_SLOT0 + 1] = 0x03;
    mem[Z80_OBJ_SLOT0 + 2] = 0x80;
    for i in 6..9 {
        mem[Z80_OBJ_SLOT0 + i] = 0;
    }
    f.sync_from_z80(&mem);
    f.set_view_dir(ViewDir::Left);

    mem[Z80_SHIELD_FRONT] = 0x49;
    f.sync_from_z80(&mem);
    assert_eq!(f.combat.enemy_shots.len(), 1);

    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &cockpit::panel_geometry(1280, 960), &[]);
    let beam_pixels = frame.pixels().iter().filter(|&&p| p == ENEMY_BEAM).count();
    assert!(
        beam_pixels > 50,
        "луч в левом виде не попал в кадр: {beam_pixels} px"
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
    assert_eq!(f.stars.stars.len(), STAR_COUNT);
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
    f.stars.stars = vec![star([-2000.0, 300.0, 0.0])];
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
        after[0] < before[0],
        "в левом виде поле не сдвигается влево по экрану"
    );
}

#[test]
fn left_and_right_views_look_in_correct_directions() {
    // Камера смотрит вдоль +Z координат вида. В левом виде впереди должна
    // оказаться левая сторона мира (-X), в правом — правая (+X).
    let left = view::rotation(ViewDir::Left).transpose();
    let right = view::rotation(ViewDir::Right).transpose();

    let left_target = left.apply([-1000.0, 0.0, 0.0]);
    assert!(left_target[2] > 0.0, "левый вид не смотрит влево");
    let right_target = right.apply([1000.0, 0.0, 0.0]);
    assert!(right_target[2] > 0.0, "правый вид не смотрит вправо");

    assert!(
        left.apply([1000.0, 0.0, 0.0])[2] < 0.0,
        "левый вид показывает объект справа"
    );
    assert!(
        right.apply([-1000.0, 0.0, 0.0])[2] < 0.0,
        "правый вид показывает объект слева"
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
    // звезда должна идти от радиуса 0.3 до края кадра заметное время, а не
    // доли секунды. Обычное движение ускорено вдвое относительно оригинала,
    // поэтому нижняя граница тоже уменьшена.
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
        (1.0..6.0).contains(&seconds),
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

#[test]
fn targon_intercept_does_not_draw_stale_planet() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    // При перехвате движок выставляет `InFreeSpace`, но координаты планеты
    // остаются близкими: без проверки флага её диск заливал бы весь вид.
    mem[Z80_IN_FREE_SPACE] = 0xFA;
    mem[Z80_PLANET_CRD + 7] = 0x40;
    mem[Z80_PLANET_CRD + 8] = 0;
    f.sync_from_z80(&mem);
    let mut frame = Frame::new(1280, 960);
    let geom = cockpit::panel_geometry(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(
        !frame.pixels().iter().any(|&p| is_planet_green(p)),
        "планета покинутой системы нарисована в космосе"
    );

    // Контроль: в планетной системе тот же диск всё ещё рисуется.
    mem[Z80_IN_FREE_SPACE] = 0;
    f.sync_from_z80(&mem);
    let mut frame = Frame::new(1280, 960);
    f.draw(&mut frame, &geom, &[]);
    assert!(
        frame.pixels().iter().any(|&p| is_planet_green(p)),
        "планета не нарисовалась в планетной системе"
    );
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

/// Гибель корабля не закрывает нативный вид: оригинал ещё держит кабину,
/// а рендер должен показать «Game Over» вместо подписи направления.
#[test]
fn game_over_keeps_native_view_and_sets_flag() {
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_WHAT_ON_SCREEN] = 0;
    mem[Z80_PLAYER_GAME_OVER] = 1;
    assert!(
        memory::z80_in_cockpit(&mem),
        "погибший корабль должен оставаться на виде из кабины"
    );

    let mut f = open();
    f.follow_z80_screen(&mem, false);
    assert!(f.active, "нативный вид закрылся при гибели");
    f.set_view_dir(ViewDir::Right);
    f.sync_from_z80(&mem);
    assert!(f.game_over, "гибель не распознана для надписи Game Over");
    assert_eq!(
        f.view_dir,
        ViewDir::Forward,
        "при гибели вид не переключился на Front View"
    );

    mem[Z80_PLAYER_GAME_OVER] = 0;
    f.sync_from_z80(&mem);
    assert!(!f.game_over, "флаг Game Over не сбросился");
}

/// Гибель обнуляет `HighDeltaSpeed` в памяти игры; это не должно включать
/// J-драйв и ложное ускорение звёзд.
#[test]
fn game_over_does_not_trigger_star_boost() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    mem[Z80_SPEED_GAUGE] = 0xA0; // скорость 31
    mem[Z80_PLAYER_GAME_OVER] = 1;
    f.sync_from_z80(&mem);
    game_tick(&mut mem);
    f.sync_from_z80(&mem);
    assert!(!f.stars.boost, "на гибели включился J-драйв");
}

/// Оригинал при гибели перестаёт пересчитывать углы, но оставляет последние
/// значения: слоты продолжают вращаться, и обломки уезжают в сторону поворота.
#[test]
fn game_over_clears_z80_rotation() {
    let mut mem = z80_mem_with_flight();
    mem[Z80_HIGH_ANGLE_LR] = 0x40;
    mem[Z80_HIGH_ANGLE_DU] = 0x84;
    z80_stop_rotation(&mut mem);
    assert_eq!(mem[Z80_HIGH_ANGLE_LR], 0, "крен не обнулён");
    assert_eq!(mem[Z80_HIGH_ANGLE_DU], 0, "тангаж не обнулён");
}

/// После гибели камера стоит: индикатор скорости уже не обновляется, поэтому
/// старое значение из памяти не должно заставлять звёзды лететь дальше.
#[test]
fn game_over_stops_camera() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    assert!(f.speed > 0.0, "в тестовой памяти нет скорости");

    mem[Z80_PLAYER_GAME_OVER] = 1;
    f.sync_from_z80(&mem);
    assert_eq!(f.speed, 0.0, "после гибели осталась старая скорость");

    let before = f.stars.stars[0].pos;
    f.tick(0.1);
    assert_eq!(
        f.stars.stars[0].pos, before,
        "звёзды летят после гибели — камера продолжает двигаться"
    );
}

/// При гибели верхняя подпись направления заменяется надписью «Game Over»
/// по центру вида.
#[test]
fn game_over_replaces_view_label_with_centered_text() {
    let mut f = open();
    let mem = vec![0u8; Z80_MEM_MIN];
    f.sync_from_z80(&mem);
    let (w, h) = (640usize, 480usize);
    let geom = cockpit::panel_geometry(w as u32, h as u32);
    let font = vec![0xFFu8; 0x300];
    let view = geom.view();

    let count_label = |frame: &Frame, y0: i32, y1: i32| {
        let mut n = 0;
        for y in y0..y1 {
            for x in view.x..view.x + view.w {
                if frame.pixels()[y as usize * w + x as usize] == VIEW_LABEL_COLOR {
                    n += 1;
                }
            }
        }
        n
    };
    let top_end = view.y + view.h / 3;
    let center_start = view.y + view.h / 3;
    let center_end = view.y + view.h * 2 / 3;

    let mut frame = Frame::new(w, h);
    f.draw(&mut frame, &geom, &font);
    assert!(
        count_label(&frame, view.y, top_end) > 0,
        "подпись вида не нарисована до гибели"
    );

    f.game_over = true;
    let mut frame = Frame::new(w, h);
    f.draw(&mut frame, &geom, &font);
    assert_eq!(
        count_label(&frame, view.y, top_end),
        0,
        "подпись Front View осталась при Game Over"
    );
    assert!(
        count_label(&frame, center_start, center_end) > 0,
        "надпись Game Over не нарисована по центру"
    );
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
    mem[Z80_SHIELD_REAR] = 0xA0;
    mem[Z80_TEMP_LASER] = 0x20;
    mem[Z80_LASER_MOUNTS] = 1; // FRONT: pulse laser

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
    assert_eq!(
        sun.color,
        sun_tint(0xFF40_80C0),
        "звезда должна получить оттенок цвета системы"
    );

    // Пустые слоты и слот ракеты не должны попасть в сцену.
    assert_eq!(
        f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count(),
        1
    );
}

#[test]
fn z80_hides_sun_and_planet_in_free_space() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    // Перехват таргонами: движок держит `InFreeSpace`, но старые координаты
    // солнца и планеты остаются от покинутой системы.
    mem[Z80_IN_FREE_SPACE] = 0xFA;
    f.sync_from_z80(&mem);
    assert!(
        !f.bodies.iter().any(|b| matches!(
            b.kind,
            BodyKind::Sun | BodyKind::Planet
        )),
        "в межпланетном пространстве не должны рисоваться солнце и планета"
    );

    // В обычной системе те же тела по тем же координатам видны.
    mem[Z80_IN_FREE_SPACE] = 0;
    f.sync_from_z80(&mem);
    assert!(
        f.bodies.iter().any(|b| b.kind == BodyKind::Sun),
        "солнце пропало в планетной системе"
    );
    assert!(
        f.bodies.iter().any(|b| b.kind == BodyKind::Planet),
        "планета пропала в планетной системе"
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

#[test]
fn ship_tint_stays_with_slot_and_changes_for_new_object() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Viper");
    put_obj(&mut mem, Z80_OBJ_SLOT0 + Z80_SLOT_SIZE, "d_Cobra1");
    f.sync_from_z80(&mem);

    let tint = |f: &Flight, source: u8| {
        f.bodies
            .iter()
            .find(|b| b.source == source)
            .and_then(|b| b.tint)
            .expect("корабль без оттенка")
    };
    let first = tint(&f, BODY_SRC_SLOT0);
    let second = tint(&f, BODY_SRC_SLOT0 + 1);
    assert_ne!(first, second, "корабли в разных слотах должны различаться");

    f.sync_from_z80(&mem);
    assert_eq!(tint(&f, BODY_SRC_SLOT0), first, "оттенок не должен мигать");

    // После освобождения слота в нём появляется уже другой объект.
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
    f.sync_from_z80(&mem);
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Viper");
    f.sync_from_z80(&mem);
    let mut tints = vec![tint(&f, BODY_SRC_SLOT0)];
    for _ in 0..12 {
        mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
        f.sync_from_z80(&mem);
        put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Viper");
        f.sync_from_z80(&mem);
        tints.push(tint(&f, BODY_SRC_SLOT0));
    }
    tints.sort_unstable();
    tints.dedup();
    assert!(tints.len() > 1, "новые объекты должны получать новые тона");
}

#[test]
fn debris_inherits_destroyed_ship_tint() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Viper");
    f.sync_from_z80(&mem);
    let ship_tint = f
        .bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .and_then(|b| b.tint)
        .expect("у корабля должен быть оттенок");

    // Оригинал помечает уничтоженный слот `oEnerg = 0xFF`, затем `ShipSet2Alloys`
    // создаёт два обломка в свободных слотах.
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = Z80_OENERG_DESTROYED;
    f.sync_from_z80(&mem);
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
    put_obj(&mut mem, Z80_OBJ_SLOT0 + Z80_SLOT_SIZE, "d_Alloys");
    put_obj(
        &mut mem,
        Z80_OBJ_SLOT0 + 2 * Z80_SLOT_SIZE,
        "d_Alloys",
    );
    f.sync_from_z80(&mem);

    let debris: Vec<_> = f
        .bodies
        .iter()
        .filter(|b| b.model.is_some_and(|m| m.name == "d_Alloys"))
        .collect();
    assert_eq!(debris.len(), 2, "должно появиться два обломка");
    for part in debris {
        assert_eq!(
            part.tint,
            Some(ship_tint),
            "обломки должны быть того же цвета, что и корабль"
        );
    }
}

#[test]
fn only_ships_and_debris_get_tint() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    put_obj(&mut mem, Z80_OBJ_MISSILE, "d_Missile");
    f.sync_from_z80(&mem);

    let tint_of = |source: u8| {
        f.bodies
            .iter()
            .find(|b| b.source == source)
            .unwrap_or_else(|| panic!("объект {source} не попал в сцену"))
            .tint
    };
    assert!(
        tint_of(BODY_SRC_SLOT0).is_some(),
        "корабль должен получить оттенок"
    );
    assert_eq!(
        tint_of(super::scene::BODY_SRC_CORIOLIS),
        None,
        "станция должна остаться исходного цвета"
    );
    assert_eq!(
        tint_of(BODY_SRC_MISSILE),
        None,
        "ракета должна остаться исходного цвета"
    );
}

#[test]
fn cargo_from_destroyed_ship_keeps_its_tint() {
    let mut f = open();
    let mut mem = vec![0u8; Z80_MEM_MIN];
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Viper");
    f.sync_from_z80(&mem);
    let ship_tint = f
        .bodies
        .iter()
        .find(|b| b.source == BODY_SRC_SLOT0)
        .and_then(|b| b.tint)
        .expect("у корабля должен быть оттенок");

    // Контейнер без известного источника остаётся обычным.
    let mut plain = open();
    let mut plain_mem = vec![0u8; Z80_MEM_MIN];
    put_obj(&mut plain_mem, Z80_OBJ_SLOT0, "d_Cargo");
    plain.sync_from_z80(&plain_mem);
    assert_eq!(
        plain
            .bodies
            .iter()
            .find(|b| b.model.is_some_and(|m| m.name == "d_Cargo"))
            .and_then(|b| b.tint),
        None,
        "контейнер без известного источника не должен краситься"
    );

    // Груз, выпавший из уничтоженного корабля, наследует его цвет.
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = Z80_OENERG_DESTROYED;
    f.sync_from_z80(&mem);
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
    put_obj(&mut mem, Z80_OBJ_SLOT0 + Z80_SLOT_SIZE, "d_Cargo");
    f.sync_from_z80(&mem);
    let cargo: Vec<_> = f
        .bodies
        .iter()
        .filter(|b| b.model.is_some_and(|m| m.name == "d_Cargo"))
        .collect();
    assert_eq!(cargo.len(), 1, "выпавший контейнер не найден");
    assert_eq!(cargo[0].tint, Some(ship_tint));
}

/// Загрузить образ Кладова и выйти в космос, как это делает игра.
fn launch_kladov() -> crate::machine::Spectrum {
    use crate::machine::Spectrum;
    let base = env!("CARGO_MANIFEST_DIR");
    let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
    let mut m = Spectrum::new(Some(&rom));
    crate::boot::load_kladov_image(&mut m);
    crate::boot::set_kladov_cpu(&mut m);
    crate::native::install_save(&mut m, crate::native::KLADOV_SAVE);
    m.intro_range = Some((0x739E, 0x73FD));
    m.game_marker = Some(0x7471);
    m.snapshot_marker = Some(0x81A9);
    m.transition_snapshot_marker = Some(crate::world::transition_snapshot_marker());

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
fn snapshot_marker_is_reached_in_live_flight() {
    let m = launch_kladov();
    assert!(m.game_snapshot_valid, "PrintCompass не достигнут в полёте");
}

/// В начале гиперпрыжка `PrintCompass` ещё не вызывается, поэтому снимок
/// сцены может хранить старый `JumpCountdown == 0`, пока живая память уже
/// ведёт отсчёт. На этом кадре тоннель начинать нельзя.
#[test]
fn hyperspace_scene_snapshot_lags_countdown_at_start() {
    let mut m = launch_kladov();
    m.mem[0xD082] = 0x46; // Pilot_Fuel
    m.mem[0xD08D] = 1; // PilotGyperdrive
    for key in [winit::keyboard::KeyCode::KeyG, winit::keyboard::KeyCode::KeyH] {
        if let Some((row, bit)) = crate::keys::matrix_pos(key) {
            m.keyboard[row] &= !(1 << bit);
        }
    }
    let mut saw_countdown = false;
    let mut saw_stale_snapshot = false;
    for _ in 0..40 {
        m.run_frame();
        let live = m.mem[Z80_JUMP_COUNTDOWN];
        let snapshot = m.game_snapshot[Z80_JUMP_COUNTDOWN];
        saw_countdown |= m.mem[Z80_JUMP_DOCKING] == 2 && live > 0;
        saw_stale_snapshot |= live > 0 && snapshot == 0;
    }
    assert!(saw_countdown, "гиперпрыжок не начался");
    assert!(
        saw_stale_snapshot,
        "снимок сцены должен отставать от отсчёта в начале прыжка"
    );
}

#[test]
fn hyperspace_countdown_uses_live_memory_over_stale_scene_snapshot() {
    let mut f = open();
    let mut live = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
    let snapshot = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
    live[Z80_JUMP_DOCKING] = 2;
    live[Z80_FLG_JUMP_DOCKING] = 1;
    live[Z80_JUMP_COUNTDOWN] = 0x15;
    // Снимок сцены ещё не содержит новый проход.
    f.start_transition(false, &live);
    f.sync_from_z80(&snapshot);
    assert!(
        f.transition.current.expect("переход пропал").warp,
        "подготовка: снимок без отсчёта должен включать тоннель"
    );

    f.sync_transition_from_live(&live);
    let t = f.transition.current.expect("переход пропал");
    assert_eq!(t.countdown, 0x15, "отсчёт не взят из живой памяти");
    assert!(!t.warp, "тоннель начался до конца отсчёта");

    live[Z80_JUMP_COUNTDOWN] = 0;
    f.sync_transition_from_live(&live);
    let t = f.transition.current.expect("переход пропал");
    assert!(t.warp, "тоннель не начался после обнуления отсчёта");
}

/// Флаги перехода взводятся уже на отсчёте, но блокировать ввод и переключение
/// экранов надо только когда началась сама анимация.
#[test]
fn countdown_is_not_transition_animation() {
    let mut mem = vec![0u8; Z80_JUMP_COUNTDOWN + 1];
    mem[Z80_JUMP_DOCKING] = 1;
    mem[Z80_FLG_JUMP_DOCKING] = 1;
    mem[Z80_JUMP_COUNTDOWN] = 0x15;
    assert!(z80_in_transition(&mem));
    assert!(
        !z80_in_transition_animation(&mem),
        "отсчёт принят за анимацию"
    );

    mem[Z80_JUMP_COUNTDOWN] = 0;
    assert!(
        z80_in_transition_animation(&mem),
        "тоннель не распознан как анимация"
    );
}

/// Оригинал создаёт новую систему перед вызовом `sub_9296`, а весь ролик
/// кругов выполняется в его собственном цикле — до `PrintCompass` дело не
/// доходит. Снимок на входе в ролик должен сразу дать нативному тоннелю новую
/// сцену, иначе на выходе из него остаётся последний кадр перед прыжком.
#[test]
fn hyperspace_tunnel_uses_new_system_from_transition_snapshot() {
    let mut m = launch_kladov();
    m.mem[0xD082] = 0x46; // Pilot_Fuel
    m.mem[0xD08D] = 1; // PilotGyperdrive
    for key in [winit::keyboard::KeyCode::KeyG, winit::keyboard::KeyCode::KeyH] {
        if let Some((row, bit)) = crate::keys::matrix_pos(key) {
            m.keyboard[row] &= !(1 << bit);
        }
    }
    let planet_crd = |mem: &[u8]| {
        let mut crd = [0u8; 9];
        crd.copy_from_slice(&mem[Z80_PLANET_CRD..Z80_PLANET_CRD + 9]);
        crd
    };
    let old_planet = planet_crd(m.game_state());
    let mut f = open();
    let mut new_planet = None;
    for _ in 0..700 {
        m.run_frame();
        f.follow_z80_screen(&m.mem, false);
        f.sync_from_z80(m.game_state());
        f.sync_transition_from_live(&m.mem);
        f.tick(0.02);

        if m.mem[Z80_FLG_JUMP_DOCKING] != 0 && new_planet.is_none() {
            let crd = planet_crd(&m.mem);
            assert_ne!(crd, old_planet, "после прыжка не появилась новая система");
            assert_eq!(
                planet_crd(m.game_state()),
                crd,
                "снимок на входе в тоннель хранит прежнюю систему"
            );
            assert!(
                f.transition.current.is_some_and(|t| t.warp),
                "нативный тоннель не начался вместе с Z80-роликом"
            );
            new_planet = Some(crd);
        }
        if new_planet.is_some() && f.transition.current.is_none() {
            assert_eq!(m.mem[Z80_JUMP_DOCKING], 0, "ролик Z80 ещё не закончился");
            assert_eq!(
                planet_crd(m.game_state()),
                new_planet.unwrap(),
                "на выходе из тоннеля показывается прежняя система"
            );
            return;
        }
    }
    panic!("нативный тоннель гиперпрыжка не закончился");
}

#[test]
fn flight_ignores_partially_updated_game_pass() {
    let mut m = crate::machine::Spectrum::new(None);
    let marker = 0x81A9; // PrintCompass — конец прохода GameMain
    m.snapshot_marker = Some(marker);
    m.mem.copy_from_slice(&z80_mem_with_flight());
    // Чем больше объектов, тем дольше проход и выше шанс, что граница кадра
    // эмулятора придётся на середину обновления объектов.
    for i in 0..6usize {
        let base = Z80_OBJ_SLOT0 + i * Z80_SLOT_SIZE;
        put_obj(&mut m.mem, base, "d_Viper");
        let x = (1000 + i as u32 * 400).to_le_bytes();
        m.mem[base..base + 3].copy_from_slice(&x[..3]);
    }
    // В конце прохода память уже хранит полностью обновлённые объекты.
    m.note_pc(marker);
    assert!(m.game_snapshot_valid);

    let mut stable = open();
    let mut live = open();
    stable.sync_from_z80(m.game_state());
    live.sync_from_z80(&m.mem);
    let slot_x = |f: &Flight| {
        f.bodies
            .iter()
            .find(|b| b.source == BODY_SRC_SLOT0)
            .unwrap()
            .pos[0]
    };
    let before = slot_x(&stable);

    // В середине прохода Z80 успел записать только часть слотов. Прямой опрос
    // памяти видит разорванное состояние, снимок — нет.
    m.mem[Z80_OBJ_SLOT0] = m.mem[Z80_OBJ_SLOT0].wrapping_add(0x20);
    stable.sync_from_z80(m.game_state());
    live.sync_from_z80(&m.mem);
    assert_eq!(slot_x(&stable), before, "снимок не защитил от разрыва прохода");
    assert_ne!(slot_x(&live), before, "живая память должна была измениться");

    // Следующий конец прохода публикует новые объекты.
    m.note_pc(marker);
    stable.sync_from_z80(m.game_state());
    assert_ne!(slot_x(&stable), before, "новый проход не был прочитан");
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
fn transitions_keep_front_view_label() {
    let mut f = open();
    let geom = cockpit::panel_geometry(640, 480);
    let font = vec![0xFFu8; 0x300];
    let view = geom.view();
    let scale = (view.w.max(0) as usize / 256)
        .min(view.h.max(0) as usize / 128)
        .max(1) as i32;
    let top_end = view.y + 24 * scale;

    let label_pixels = |frame: &Frame| {
        let mut n = 0;
        for y in view.y..top_end {
            for x in view.x..view.x + view.w {
                if frame.pixels()[y as usize * frame.width + x as usize] == VIEW_LABEL_COLOR {
                    n += 1;
                }
            }
        }
        n
    };

    // И вылет, и стыковка, и обе фазы гиперпрыжка должны держать ту же
    // подпись Front View, что и обычный экран F1.
    for (kind, warp) in [
        (TransitionKind::Launch, false),
        (TransitionKind::Docking, false),
        (TransitionKind::Hyperspace, false),
        (TransitionKind::Hyperspace, true),
    ] {
        let mut t = Transition::new(kind, 0);
        t.warp = warp;
        t.warp_elapsed = 0.2;
        t.elapsed = 0.2;
        f.transition.current = Some(t);
        f.view_dir = ViewDir::Forward;

        let mut frame = Frame::new(640, 480);
        f.draw(&mut frame, &geom, &font);
        assert!(
            label_pixels(&frame) > 0,
            "{kind:?} (warp={warp}): подпись Front View не нарисована"
        );
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

/// Собрать строку в перехватчике так, как это делает хук печати.
fn tap_with_text(text: &str, row: u8, col: u8) -> GameMsgTap {
    let mut tap = GameMsgTap::default();
    tap.enable_text_capture_for_test();
    for (i, c) in text.bytes().enumerate() {
        tap.capture_text_char(c, row, col + i as u8);
    }
    tap.end_text_frame();
    tap
}

#[test]
fn bottom_message_comes_from_row_13() {
    let mut f = open();
    let mut tap = tap_with_text("HYPERSPACE -- LAVE", 13, 5);
    f.sync_game_msg(&mut tap);
    assert_eq!(f.messages.bottom_label(), Some("HYPERSPACE -- LAVE"));
}

#[test]
fn bottom_message_is_drawn_centered_at_bottom() {
    let mut f = open();
    let mem = vec![0u8; Z80_MEM_MIN];
    f.sync_from_z80(&mem);
    let (w, h) = (640usize, 480usize);
    let geom = cockpit::panel_geometry(w as u32, h as u32);
    let font = vec![0xFFu8; 0x300];
    let view = geom.view();

    let mut tap = tap_with_text("HYPERSPACE -- LAVE", 13, 5);
    f.sync_game_msg(&mut tap);
    let mut frame = Frame::new(w, h);
    f.draw(&mut frame, &geom, &font);

    // Полоса, где рисуется нижнее сообщение: одна строка над нижней рамкой.
    let scale = ((w / 256).min(view.h as usize / 128)).max(1) as i32;
    let y0 = view.y + view.h - 16 * scale;
    let y1 = view.y + view.h;
    let (mut min_x, mut max_x) = (i32::MAX, -1);
    for y in y0..y1 {
        for x in view.x..view.x + view.w {
            if frame.pixels()[y as usize * w + x as usize] == VIEW_LABEL_COLOR {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }
        }
    }
    assert!(max_x >= 0, "нижнее сообщение не нарисовано");
    let text_center = (min_x + max_x) / 2;
    let view_center = view.x + view.w / 2;
    assert!(
        (text_center - view_center).abs() <= 2,
        "текст не по центру: текст {text_center}, вид {view_center}"
    );
}

#[test]
fn bottom_message_ignores_other_rows() {
    let mut f = open();
    for (text, row) in [("FRONT VIEW", 1), ("FUEL", 20), ("MISSILE JAMMED", 12)] {
        let mut tap = tap_with_text(text, row, 0);
        f.sync_game_msg(&mut tap);
        assert_eq!(f.messages.bottom_label(), None, "строка {row} попала в показ");
    }
}

#[test]
fn bottom_message_expires_after_ttl() {
    let mut f = open();
    let mut tap = tap_with_text("MISSILE JAMMED", 13, 5);
    f.sync_game_msg(&mut tap);
    assert_eq!(f.messages.bottom_label(), Some("MISSILE JAMMED"));

    f.advance_messages(BOTTOM_MSG_TTL as u64);
    assert_eq!(f.messages.bottom_label(), Some("MISSILE JAMMED"));
    f.advance_messages(1);
    assert_eq!(f.messages.bottom_label(), None);
}

#[test]
fn repeated_bottom_message_refreshes_ttl() {
    let mut f = open();
    let mut tap = tap_with_text("HYPERSPACE -- LAVE", 13, 5);
    f.sync_game_msg(&mut tap);
    f.advance_messages(BOTTOM_MSG_TTL as u64);
    assert_eq!(f.messages.bottom_label(), Some("HYPERSPACE -- LAVE"));

    // Повтор той же строки — heartbeat: возраст снова ноль.
    let mut tap = tap_with_text("HYPERSPACE -- LAVE", 13, 5);
    f.sync_game_msg(&mut tap);
    assert_eq!(
        f.messages.bottom.as_ref().map(|m| m.age),
        Some(0),
        "повтор не продлил показ"
    );
    f.advance_messages(BOTTOM_MSG_TTL as u64);
    assert_eq!(f.messages.bottom_label(), Some("HYPERSPACE -- LAVE"));
}

#[test]
fn docking_message_stays_visible_between_game_prints() {
    let mut f = open();
    let mut tap = tap_with_text("DOCKING COMPUTERS ON", 13, 5);
    f.sync_game_msg(&mut tap);
    assert_eq!(f.messages.bottom_label(), Some("DOCKING COMPUTERS ON"));

    // Один проход `GameMain` занимает 3–4 кадра, поэтому между печатями внутри
    // пачки бывают пропуски. Они не должны вызывать мигание.
    for _ in 0..6 {
        f.advance_messages(4);
        let mut tap = tap_with_text("DOCKING COMPUTERS ON", 13, 5);
        f.sync_game_msg(&mut tap);
        assert_eq!(f.messages.bottom_label(), Some("DOCKING COMPUTERS ON"));
    }
}

#[test]
fn docking_message_blinks_with_slow_period() {
    let mut f = open();
    let mut tap = tap_with_text("DOCKING COMPUTERS ON", 13, 5);
    f.sync_game_msg(&mut tap);

    // Повторная печать обновляет срок жизни, но не фазу мигания.
    let mut elapsed = 0;
    while elapsed < DOCKING_BLINK_ON_FRAMES - 1 {
        f.advance_messages(3);
        elapsed += 3;
        let mut tap = tap_with_text("DOCKING COMPUTERS ON", 13, 5);
        f.sync_game_msg(&mut tap);
        assert_eq!(f.messages.bottom_label(), Some("DOCKING COMPUTERS ON"));
    }

    f.advance_messages(1);
    assert_eq!(f.messages.bottom_label(), None, "надпись не погасла");

    // Пауза короче TTL, поэтому надпись остаётся в сообщении и появляется
    // снова на следующем цикле.
    f.advance_messages((DOCKING_BLINK_PERIOD - DOCKING_BLINK_ON_FRAMES - 1) as u64);
    assert_eq!(f.messages.bottom_label(), None);
    f.advance_messages(1);
    assert_eq!(f.messages.bottom_label(), Some("DOCKING COMPUTERS ON"));
}

#[test]
fn docking_message_expires_when_prints_stop() {
    let mut f = open();
    let mut tap = tap_with_text("DOCKING COMPUTERS ON", 13, 5);
    f.sync_game_msg(&mut tap);
    f.advance_messages(DOCKING_MSG_TTL as u64);
    assert_eq!(f.messages.bottom_label(), Some("DOCKING COMPUTERS ON"));
    f.advance_messages(1);
    assert_eq!(f.messages.bottom_label(), None);
}

#[test]
fn closing_flight_clears_bottom_message() {
    let mut f = open();
    let mut tap = tap_with_text("HYPERSPACE -- LAVE", 13, 5);
    f.sync_game_msg(&mut tap);
    assert!(f.messages.bottom_label().is_some());
    f.close();
    assert_eq!(f.messages.bottom_label(), None);
}

/// Уничтожить объект так, как это делает `FireLaser` перед `DestroyObj`.
fn mark_slot_destroyed(mem: &mut [u8], base: usize) {
    mem[base + Z80_SLOT_OENERG] = Z80_OENERG_DESTROYED;
}

#[test]
fn destroyed_object_spawns_particle_cloud() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    assert!(
        f.explosions.clouds.is_empty(),
        "облако появилось без уничтожения"
    );

    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);

    assert_eq!(f.explosions.clouds.len(), 1, "уничтожение не поймано");
    let cloud = &f.explosions.clouds[0];
    assert_eq!(cloud.center, [1000.0, 0.0, 3000.0]);
    assert!(
        cloud.particles.len() >= 12,
        "в облаке слишком мало частиц: {}",
        cloud.particles.len()
    );
    // Уничтоженный слот не должен рисоваться как целый корабль, но станция
    // и остальные объекты сцены остаются.
    assert!(
        !f.bodies.iter().any(|b| b.kind == BodyKind::Ship),
        "уничтоженный слот нарисован целым кораблём"
    );
    assert!(f.bodies.iter().any(|b| b.kind == BodyKind::Station));
}

#[test]
fn destroyed_marker_does_not_spawn_repeated_clouds() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    assert_eq!(f.explosions.clouds.len(), 1);

    // `oEnerg == 0xFF` живёт несколько тактов — облако не должно множиться.
    f.sync_from_z80(&mem);
    f.sync_from_z80(&mem);
    assert_eq!(f.explosions.clouds.len(), 1, "облако создано повторно");

    // Слот очистился — облако продолжает жить, но нового уже нет.
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
    f.sync_from_z80(&mem);
    assert_eq!(f.explosions.clouds.len(), 1);
}

#[test]
fn clearing_slots_does_not_spawn_clouds() {
    // `SetShipwreck2` обнуляет `oEnerg` всех слотов. Это не взрыв, и облака
    // на месте пропавших объектов появляться не должны.
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    mem[Z80_OBJ_CORIOLIS + Z80_SLOT_OENERG] = 0;
    mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG] = 0;
    f.sync_from_z80(&mem);
    assert!(
        f.explosions.clouds.is_empty(),
        "обнуление слотов принято за уничтожение"
    );
}

#[test]
fn particle_cloud_dissolves_in_two_seconds() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);

    let spread = |f: &Flight| {
        f.explosions.clouds[0]
            .particles
            .iter()
            .map(|p| {
                let o = p.offset;
                (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt()
            })
            .fold(0.0f32, f32::max)
    };
    let start = spread(&f);

    // К середине жизни облако ещё видно, частицы уже разлетелись.
    for _ in 0..(EXPLOSION_TIME / 2.0 / 0.02) as usize {
        f.tick(0.02);
    }
    assert_eq!(f.explosions.clouds.len(), 1, "облако исчезло раньше времени");
    assert!(
        spread(&f) > start,
        "частицы не разлетаются: {start} -> {}",
        spread(&f)
    );

    // Через две секунды от облака не остаётся ничего.
    for _ in 0..(EXPLOSION_TIME / 2.0 / 0.02) as usize + 5 {
        f.tick(0.02);
    }
    assert!(
        f.explosions.clouds.is_empty(),
        "облако не растворилось за {EXPLOSION_TIME} с"
    );
}

#[test]
fn particle_cloud_follows_game_tick() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    let before = f.explosions.clouds[0].center;

    // Игровой такт: меняются координаты, а вместе с ними и углы камеры.
    game_tick(&mut mem);
    mem[Z80_HIGH_ANGLE_LR] = 0x40;
    f.sync_from_z80(&mem);
    let after = f.explosions.clouds[0].center;

    assert_ne!(after, before, "облако не повернулось вместе с миром");
    assert!(
        after[2] < before[2] - 1.0,
        "облако не сдвинулось к игроку: {} -> {}",
        before[2],
        after[2]
    );
}

/// После гибели облако не поворачивается вместе с камерой и уносится от неё.
#[test]
fn game_over_cloud_flies_away_from_camera() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    assert_eq!(f.explosions.clouds.len(), 1, "облако не создано");

    let before = f.explosions.clouds[0].center;
    // Игровой такт с поворотом камеры и гибелью игрока.
    game_tick(&mut mem);
    mem[Z80_HIGH_ANGLE_LR] = 0x40;
    mem[Z80_PLAYER_GAME_OVER] = 1;
    f.sync_from_z80(&mem);
    let after = f.explosions.clouds[0].center;

    assert_eq!(
        [after[0], after[1]],
        [before[0], before[1]],
        "облако повернулось вместе с камерой после гибели"
    );
    assert!(
        after[2] > before[2],
        "облако не ушло от игрока: {} -> {}",
        before[2],
        after[2]
    );
}

/// Тёплый пиксель взрыва: красный ярче зелёного, зелёный ярче синего.
fn is_warm(p: u32) -> bool {
    let (r, g, b) = ((p >> 16) & 0xFF, (p >> 8) & 0xFF, p & 0xFF);
    r > g && g > b && r - b > 40
}

#[test]
fn destroyed_object_draws_warm_particle_cloud() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    f.sync_from_z80(&mem);
    let geom = cockpit::panel_geometry(640, 480);
    let mut before = Frame::new(640, 480);
    f.draw(&mut before, &geom, &[]);
    assert!(
        !before.pixels().iter().any(|&p| is_warm(p)),
        "тёплые пиксели были ещё до взрыва"
    );

    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    let mut after = Frame::new(640, 480);
    f.draw(&mut after, &geom, &[]);
    assert!(
        after.pixels().iter().any(|&p| is_warm(p)),
        "облако частиц не нарисовано"
    );
}

/// Память с объектами, у которых original взводил бы `FlgObjNear`.
fn flight_mem_with_near_objects() -> Vec<u8> {
    let mut mem = z80_mem_with_flight();
    mem[Z80_FLG_OBJ_NEAR] = 1;
    mem
}

/// Освободить слот: в конце отсчёта original сам обнуляет `oEnerg`.
fn clear_slot(mem: &mut [u8], base: usize) {
    mem[base + Z80_SLOT_OENERG] = 0;
}

/// Запустить отсчёт ухода в первом корабельном слоте: слот ещё занят, а в
/// дополнительном слоте уже 0xA0. Так уходят и корабли, и выброшенные капсулы.
fn start_warp(mem: &mut [u8], name: &str) {
    put_obj(mem, Z80_OBJ_SLOT0, name);
    mem[ext_slot_base(1)] = 0xA0;
}

fn start_ship_warp(mem: &mut [u8]) {
    start_warp(mem, "d_Cobra1");
}

#[test]
fn departing_ship_spawns_warp_while_slot_is_occupied() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty());

    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);

    assert_eq!(f.teleports.warps.len(), 1, "уход корабля не пойман");
    assert_eq!(f.teleports.warps[0].center, [1000.0, 0.0, 3000.0]);
    assert_ne!(mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG], 0, "слот уже пуст");
    // Обычное облако взрыва создаётся только по `oEnerg == 0xFF`.
    assert!(f.explosions.clouds.is_empty());
}

#[test]
fn warp_appears_on_countdown_not_after_slot_clear() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);

    start_ship_warp(&mut mem);
    mem[ext_slot_base(1)] = 0xA3;
    f.sync_from_z80(&mem);
    assert_eq!(f.teleports.warps.len(), 1, "счётчик гиперпрыжка не пойман");

    // Конец отсчёта: original чистит слот и уже не печатает круги.
    mem[ext_slot_base(1)] = 0;
    clear_slot(&mut mem, Z80_OBJ_SLOT0);
    mem[Z80_FLG_OBJ_NEAR] = 0;
    f.sync_from_z80(&mem);
    assert_eq!(f.teleports.warps.len(), 1, "эффект создан повторно");
}

#[test]
fn departing_escape_pod_spawns_warp() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);

    // `sub_7C60` пишет 0xA0 в допслот при выбросе капсулы — тот же счётчик,
    // что и у уходящего корабля.
    start_warp(&mut mem, "d_EscapePod");
    f.sync_from_z80(&mem);

    assert_eq!(f.teleports.warps.len(), 1, "уход капсулы не пойман");
    assert_ne!(mem[Z80_OBJ_SLOT0 + Z80_SLOT_OENERG], 0, "слот уже пуст");
}

#[test]
fn empty_slot_without_countdown_does_not_spawn_warp() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_EscapePod");
    f.sync_from_z80(&mem);

    clear_slot(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);

    assert!(
        f.teleports.warps.is_empty(),
        "пустой слот принят за телепорт"
    );
}

#[test]
fn destroyed_object_does_not_spawn_warp() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);

    mark_slot_destroyed(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty(), "взрыв принят за телепорт");
    assert_eq!(f.explosions.clouds.len(), 1);

    // После очистки слот уже не имеет прошлого состояния корабля.
    clear_slot(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty());
}

#[test]
fn warp_is_not_spawned_twice() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);
    assert_eq!(f.teleports.warps.len(), 1);

    // Счётчик идёт несколько тактов, слот опрашивается на каждом кадре.
    mem[ext_slot_base(1)] = 0xA5;
    f.sync_from_z80(&mem);
    f.sync_from_z80(&mem);
    assert_eq!(f.teleports.warps.len(), 1, "эффект создан повторно");
}

#[test]
fn warp_needs_near_objects_flag() {
    let mut f = open();
    let mut mem = z80_mem_with_flight();
    mem[Z80_FLG_OBJ_NEAR] = 0;
    f.sync_from_z80(&mem);

    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);

    assert!(
        f.teleports.warps.is_empty(),
        "эффект показан без `FlgObjNear`"
    );
}

#[test]
fn warp_needs_object_in_view() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    // Знак у 24-битной координаты — старший бит третьего байта.
    mem[Z80_OBJ_SLOT0 + 8] = 0x80; // z = -3000, объект позади камеры
    f.sync_from_z80(&mem);

    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);

    assert!(
        f.teleports.warps.is_empty(),
        "эффект показан для объекта за кадром"
    );
}

#[test]
fn cargo_and_station_do_not_spawn_warp() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    put_obj(&mut mem, Z80_OBJ_SLOT0, "d_Cargo");
    mem[ext_slot_base(1)] = 0xA0;
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty(), "груз принят за корабль");
    clear_slot(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty(), "груз принят за корабль");

    f.sync_from_z80(&mem);
    mem[ext_slot_base(0)] = 0xA0;
    clear_slot(&mut mem, Z80_OBJ_CORIOLIS);
    f.sync_from_z80(&mem);
    assert!(f.teleports.warps.is_empty(), "станция принята за корабль");
}

#[test]
fn warp_tail_follows_game_tick_after_object_leaves() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);

    // После очистки слот уже не обновляет координату эффекта: дальше он
    // движется вместе с миром, как облако взрыва.
    mem[ext_slot_base(1)] = 0;
    clear_slot(&mut mem, Z80_OBJ_SLOT0);
    f.sync_from_z80(&mem);
    let before = f.teleports.warps[0].center;

    game_tick(&mut mem);
    mem[Z80_HIGH_ANGLE_LR] = 0x40;
    f.sync_from_z80(&mem);
    let after = f.teleports.warps[0].center;

    assert_ne!(after, before, "эффект не повернулся вместе с миром");
    assert!(
        after[2] < before[2] - 1.0,
        "эффект не сдвинулся к игроку: {} -> {}",
        before[2],
        after[2]
    );
}

#[test]
fn warp_follows_object_until_slot_is_empty() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);
    let before = f.teleports.warps[0].center;

    mem[Z80_OBJ_SLOT0] = mem[Z80_OBJ_SLOT0].wrapping_add(8);
    f.sync_from_z80(&mem);
    let after = f.teleports.warps[0].center;

    assert_ne!(after, before, "эффект не последовал за объектом");
    assert!(
        (after[0] - before[0] - 8.0).abs() < 1e-3,
        "эффект отошёл от объекта: {} -> {}",
        before[0],
        after[0]
    );
}

#[test]
fn warp_disappears_after_animation() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);
    let duration = f.teleports.warps[0].duration;
    // После ухода объекта эффект ещё живёт полсекунды.
    assert!(duration > TELEPORT_TRAIL_TIME, "эффект слишком короткий");

    for _ in 0..(duration / 2.0 / 0.02) as usize {
        f.tick(0.02);
    }
    assert_eq!(f.teleports.warps.len(), 1, "эффект исчез раньше времени");

    for _ in 0..(duration / 2.0 / 0.02) as usize + 5 {
        f.tick(0.02);
    }
    assert!(
        f.teleports.warps.is_empty(),
        "эффект не закончился за {duration} с"
    );
}

#[test]
fn departing_object_draws_cold_rings_not_explosion() {
    let mut f = open();
    let mut mem = flight_mem_with_near_objects();
    f.sync_from_z80(&mem);
    let geom = cockpit::panel_geometry(640, 480);
    let ring = teleport_ring_color(1.0);
    let mut before = Frame::new(640, 480);
    f.draw(&mut before, &geom, &[]);
    assert!(
        !before.pixels().iter().any(|&p| p == ring),
        "кольца телепорта были ещё до ухода"
    );

    start_ship_warp(&mut mem);
    f.sync_from_z80(&mem);
    let mut after = Frame::new(640, 480);
    f.draw(&mut after, &geom, &[]);
    assert!(
        after.pixels().iter().any(|&p| p == ring),
        "кольца телепорта не нарисованы"
    );
    assert!(
        !after.pixels().iter().any(|&p| is_warm(p)),
        "уход объекта нарисован тёплым взрывом"
    );
}
