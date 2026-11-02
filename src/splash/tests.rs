//! Тесты заставки: композиция, анимация и видимость граней.

use crate::display::Frame;
use crate::render3d::{
    INTRO_DIST, INTRO_SCO, INTRO_SHIP_RADIUS, Rotation, SHIP_COLOR, Screen as Proj,
    face_visibility, line_hidden,
};
use crate::shipdata::{self, Ship};

use super::Splash;
use super::cycle::INTRO_CYCLE;
use super::motion::{STEP_ANGLE, STEP_AXIS, STEP_FRAMES};

/// Совпадает ли геометрия двух объектов.
///
/// В таблице Кладова обычный и пиратский варианты Cobra/Python (а также два
/// астероида) хранятся как отдельные записи с одинаковыми вершинами, рёбрами
/// и гранями. Для проверок полезно знать, что это один корпус.
fn same_hull(a: &Ship, b: &Ship) -> bool {
    a.vertices == b.vertices && a.edges == b.edges && a.faces == b.faces
}

/// Заставка: непрозрачный чёрный фон, белый заголовок, корабль с
/// тёмными гранями и белыми рёбрами, жёлтое приглашение.
#[test]
fn splash_draws_title_ship_and_hints() {
    let font = vec![0xFFu8; 0x300]; // шрифт «все биты залиты»
    let mut f = Frame::new(1280, 960);
    let st = Splash::new();
    st.draw(&mut f, &font);
    assert_eq!(
        f.pixels()[0],
        0xFF00_0000,
        "фон должен быть чёрным и непрозрачным"
    );
    assert!(
        f.pixels().iter().any(|&p| p == 0xFFD7_D7D7),
        "заголовок не нарисован"
    );
    assert!(
        f.pixels().iter().any(|&p| p == SHIP_COLOR),
        "рёбра корабля не нарисованы"
    );
    assert!(
        f.pixels()
            .iter()
            .any(|&p| p == crate::render3d::SHIP_FACE_COLOR),
        "грани корабля не залиты"
    );
    assert!(
        f.pixels().iter().any(|&p| p == 0xFFD7_D7D7),
        "приглашение не нарисовано"
    );
    // Лишних надписей нет: серый (подсказка меню) больше не рисуется.
    assert!(
        !f.pixels().iter().any(|&p| p == 0xFF90_9090),
        "на заставке остались лишние надписи"
    );
    if let Ok(path) = std::env::var("ELITE_DUMP_SPLASH") {
        write_ppm(&path, f.pixels(), 1280, 960);
    }
}

/// Сохранить кадр в PPM (R8G8B8) для визуальной проверки.
fn write_ppm(path: &str, buf: &[u32], w: usize, h: usize) {
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for &p in buf {
        out.push((p >> 16) as u8);
        out.push((p >> 8) as u8);
        out.push(p as u8);
    }
    let _ = std::fs::write(path, out);
}

/// Анимация: корабль вращается — кадры через 40 тиков различаются.
#[test]
fn splash_animation_rotates_ship() {
    let font = vec![0u8; 0x300]; // без текста — только корабль
    let mut a = Frame::new(1280, 960);
    let mut st = Splash::new();
    st.draw(&mut a, &font);
    for _ in 0..40 {
        st.tick();
    }
    let mut b = Frame::new(1280, 960);
    st.draw(&mut b, &font);
    assert!(a.pixels() != b.pixels(), "корабль не вращается");
}

/// При не-кратных размерах окна (например, 16:10) композиция заставки
/// центрируется по обеим осям: заголовок не прижат к левому краю.
#[test]
fn splash_is_centered_for_16_10() {
    let font = vec![0xFFu8; 0x300];
    // 1680x1050: масштаб сетки 256x192 — 6, ширина сетки 1536, высота
    // 1152 > 1050, поэтому масштаб 5 (1280x960), отступы ox=200, oy=45.
    let mut f = Frame::new(1680, 1050);
    let st = Splash::new();
    st.draw(&mut f, &font);
    // Левые поля (по 200 px) должны остаться фоном — заголовок не съехал.
    let bg = 0xFF00_0000;
    for y in 0..1050 {
        for x in 0..200 {
            assert_eq!(
                f.pixels()[y * 1680 + x],
                bg,
                "заставка съехала влево: пиксель ({x},{y}) вне центральной области"
            );
        }
    }
    // Справа тоже поля.
    for y in 0..1050 {
        for x in 1480..1680 {
            assert_eq!(f.pixels()[y * 1680 + x], bg, "заставка не по центру справа");
        }
    }
}

/// Надписи заставки внутри новой рамки кокпита: по центру области вида и
/// с отступом в одну текстовую строку от верхней и нижней линии рамки —
/// как в оригинале (заголовок в строке 1, приглашение над панелью).
#[test]
fn splash_texts_are_centered_and_inset_from_frame() {
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1280, 960);
    let g = crate::cockpit::panel_geometry(1280, 960);
    let view = g.view();
    Splash::new().draw_in(&mut f, &font, &g);

    // Габарит белых пикселей в полосе `y0..y1` (рамка кокпита здесь не
    // рисуется, поэтому белым остаются только надписи).
    let bbox = |y0: i32, y1: i32| {
        let (mut x0, mut x1, mut ty0, mut ty1) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for y in y0..y1 {
            for x in view.x..view.x + view.w {
                if f.pixels()[y as usize * 1280 + x as usize] == 0xFFD7_D7D7 {
                    x0 = x0.min(x);
                    x1 = x1.max(x);
                    ty0 = ty0.min(y);
                    ty1 = ty1.max(y);
                }
            }
        }
        assert!(x1 >= x0, "надпись в полосе {y0}..{y1} не нарисована");
        (x0, x1, ty0, ty1)
    };
    let title = bbox(view.y, view.y + view.h / 2);
    let hint = bbox(view.y + view.h / 2, view.y + view.h);
    let center = view.x + view.w / 2;
    for (name, b) in [("заголовок", title), ("приглашение", hint)] {
        let mid = (b.0 + b.1) / 2;
        assert!(
            (mid - center).abs() <= 2,
            "{name} не по центру: {mid} вместо {center}"
        );
    }
    // Отступ от рамки — около одной текстовой строки (8*s пикселей).
    let s = 3; // масштаб сетки при окне 1280x960 и панели 25 %
    assert!(
        title.2 >= view.y + 4 * s,
        "заголовок прижат к верхней рамке: {}",
        title.2
    );
    assert!(
        hint.3 <= view.y + view.h - 4 * s,
        "приглашение прижато к нижней рамке: {}",
        hint.3
    );
    assert!(
        title.0 > view.x && hint.1 < view.x + view.w,
        "надписи выходят за область вида"
    );
}

/// Видимые грани корабля залиты непрозрачным тёмно-серым, а рёбра
/// нарисованы поверх ярко-белым.
///
/// Проверяем область корабля (центр по центру полосы между надписями,
/// радиус `SHIP_RADIUS_S*s`, `s = 5` при окне 1280x960): тёмных пикселей
/// должно быть много (заливка, а не каркас), а белые рёбра — заметны.
#[test]
fn splash_ship_faces_are_filled() {
    let font = vec![0u8; 0x300];
    let mut f = Frame::new(1280, 960);
    let st = Splash::new();
    st.draw(&mut f, &font);
    let (cx, cy, r) = (1280 / 2, 480, INTRO_SHIP_RADIUS * 5);
    let mut total = 0;
    let mut face = 0;
    let mut edge = 0;
    for y in cy - r..cy + r {
        for x in cx - r..cx + r {
            total += 1;
            match f.pixels()[y as usize * 1280 + x as usize] {
                crate::render3d::SHIP_FACE_COLOR => face += 1,
                SHIP_COLOR => edge += 1,
                _ => {}
            }
        }
    }
    assert!(total > 0, "область корабля пуста");
    assert!(
        face * 10 > total,
        "грани корабля не залиты: {face} из {total}"
    );
    assert!(edge > 100, "рёбра корабля не нарисованы: {edge}");
}

/// Корабль на заставке крупнее прежнего и помещается между надписями.
#[test]
fn splash_ship_is_large_and_fits_texts() {
    let font = vec![0u8; 0x300];
    let mut f = Frame::new(1280, 960);
    let st = Splash::new();
    st.draw(&mut f, &font);
    // Полоса между заголовком и приглашением: корабль должен её занимать
    // почти целиком, но не залезать на строки.
    let s = 5;
    let title_bottom = 12 * s;
    let hint_top = 960 - 12 * s;
    let mut top = 960;
    let mut bottom = 0;
    for y in 0..960 {
        for x in 0..1280 {
            if f.pixels()[y * 1280 + x] == SHIP_COLOR {
                top = top.min(y);
                bottom = bottom.max(y);
            }
        }
    }
    assert!(top >= title_bottom, "корабль залез на заголовок: {top}");
    assert!(bottom <= hint_top, "корабль залез на приглашение: {bottom}");
    // Габарит корабля — больше прежних 2*52*s = 520 px по высоте.
    assert!(
        bottom - top > 52 * s,
        "корабль не увеличился: {bottom}..{top}"
    );
}

/// Модель корабля из `shipdata` найдена и не пуста (иначе на заставке
/// рисовался бы запасной куб).
#[test]
fn splash_ship_model_exists() {
    let ships = shipdata::SHIPS;
    let s = ships
        .iter()
        .find(|x| x.name == "d_Cobra1")
        .expect("модель корабля не найдена");
    assert!(!s.vertices.is_empty(), "у модели нет вершин");
    assert!(!s.edges.is_empty(), "у модели нет рёбер");
    assert!(!s.faces.is_empty(), "у модели нет граней");
}

/// Переключение кораблей идёт по циклу оригинала: восемь боевых
/// кораблей, старт — Cobra, порядок как в `loc_71A2`.
#[test]
fn splash_cycle_follows_original_order() {
    let mut st = Splash::new();
    // Старт — Cobra, как у оригинала (`ld a,79h`).
    assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
    let mut seen = Vec::new();
    for _ in 0..INTRO_CYCLE.len() {
        let name = st.ship.expect("нет корабля").name;
        assert!(
            INTRO_CYCLE.contains(&name),
            "корабль {name} вне цикла заставки"
        );
        assert!(!seen.contains(&name), "модель {name} показана дважды");
        seen.push(name);
        // Каждый переключатель возвращает исходный ракурс (как
        // `SetShip4Intro` в оригинале).
        st.tick();
        st.cycle_ship(1);
        let intro = Rotation::intro();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (st.rot.m[i][j] - intro.m[i][j]).abs() < 1e-6,
                    "после смены корабля ракурс не сброшен"
                );
            }
        }
    }
    assert_eq!(
        seen.len(),
        INTRO_CYCLE.len(),
        "круг переключения не замкнулся"
    );
    assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
    // Назад с Cobra — Viper (последний в цикле).
    st.cycle_ship(-1);
    assert_eq!(st.ship.expect("нет корабля").name, "d_Viper");
    // Вперёд с Viper — снова Cobra.
    st.cycle_ship(1);
    assert_eq!(st.ship.expect("нет корабля").name, "d_Cobra1");
    // Все модели цикла есть в таблице и представляют разные корпуса.
    for &name in &INTRO_CYCLE {
        shipdata::find(name).expect("нет модели из цикла");
    }
    for (i, &a) in INTRO_CYCLE.iter().enumerate() {
        for &b in &INTRO_CYCLE[i + 1..] {
            assert!(
                !same_hull(shipdata::find(a).unwrap(), shipdata::find(b).unwrap()),
                "{a} и {b} — один и тот же корпус"
            );
        }
    }
}

/// Модель «Cobra» для проверок видимости.
fn cobra() -> &'static Ship {
    shipdata::SHIPS
        .iter()
        .find(|x| x.name == "d_Cobra1")
        .expect("модель корабля не найдена")
}

/// Вспомогательный поворот вокруг вертикальной оси (для проверок видимости).
fn yaw_rot(angle: f32) -> Rotation {
    Rotation::axis_angle([0.0, 1.0, 0.0], angle)
}

/// Видимость грани — по знаку Z повёрнутой нормали (тест оригинала).
///
/// Раньше в тесте участвовал бит 7 `Face::flags`, который на самом деле
/// задаёт знак X-компоненты нормали, — из-за чего видимая и невидимая
/// стороны менялись местами.
#[test]
fn face_visibility_follows_rotated_normal_z() {
    let ship = cobra();
    for yaw in [0.0f32, 0.7, 1.5, 2.4, 3.0, 4.5] {
        let rot = yaw_rot(yaw);
        let vis = face_visibility(ship, &rot);
        assert_eq!(vis.len(), ship.faces.len());
        for (f, &v) in ship.faces.iter().zip(vis.iter()) {
            let n = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
            assert_eq!(v, n[2] < 0.0, "грань видна не по знаку нормали");
        }
    }
}

/// При нулевом повороте видна «задняя» половина корабля (нормаль хвоста
/// смотрит в -Z, т.е. к наблюдателю) — как в оригинале, где камера
/// смотрит вдоль +Z, а противники в игре видны с кормы.
#[test]
fn face_visibility_shows_tail_side_at_identity() {
    let ship = cobra();
    let rot = yaw_rot(0.0);
    let vis = face_visibility(ship, &rot);
    // Нормаль хвоста (0, 0, -80) — видна.
    let tail = ship
        .faces
        .iter()
        .position(|f| f.nx == 0 && f.ny == 0 && f.nz < 0)
        .expect("нормаль хвоста не найдена");
    assert!(vis[tail], "хвост должен быть виден");
    // Верхние грани (nz > 0) — не видны.
    for (i, f) in ship.faces.iter().enumerate() {
        if f.nz > 0 {
            assert!(!vis[i], "верхняя грань {i} не должна быть видна");
        }
    }
}

/// Видимая половина модели смотрит к наблюдателю: сумма Z-компонент
/// видимых нормалей отрицательна, а невидимых — положительна.
#[test]
fn face_visibility_culls_far_half_only() {
    let ship = cobra();
    for yaw in [0.0f32, 0.9, 2.0, 3.3, 5.0] {
        let rot = yaw_rot(yaw);
        let vis = face_visibility(ship, &rot);
        let mut near = 0.0f32;
        let mut far = 0.0f32;
        for (f, &v) in ship.faces.iter().zip(vis.iter()) {
            let z = rot.apply([f.nx as f32, f.ny as f32, f.nz as f32])[2];
            if v {
                near += z;
            } else {
                far += z;
            }
        }
        assert!(near < 0.0, "видимые грани должны смотреть к наблюдателю");
        assert!(far > 0.0, "невидимые грани должны смотреть от наблюдателя");
    }
}

/// Стартовый ракурс заставки — ровно `SKO4Intro` оригинала.
///
/// Оси модели после поворота должны совпасть с нормированными столбцами
/// таблицы, а нос (локальная +Z) — смотреть от наблюдателя: оригинал
/// показывает корабль с кормы-сверху, а не «носом на камеру».
#[test]
fn splash_starts_from_original_intro_pose() {
    let st = Splash::new();
    for (j, col) in INTRO_SCO.iter().enumerate() {
        let n = (col[0] * col[0] + col[1] * col[1] + col[2] * col[2]).sqrt();
        let mut local = [0.0f32; 3];
        local[j] = 1.0;
        let w = st.rot.apply(local);
        for i in 0..3 {
            let want = col[i] / n;
            assert!(
                (w[i] - want).abs() < 1e-6,
                "ось {j} повёрнута не как в SKO4Intro: {w:?} vs {want}"
            );
        }
    }
    let nose = st.rot.apply([0.0, 0.0, 1.0]);
    assert!(
        nose[2] > 0.0,
        "оригинал показывает корабль с кормы: {nose:?}"
    );
}

/// Угол поворота между двумя ориентациями (радианы).
fn angle_between(a: &Rotation, b: &Rotation) -> f32 {
    // b = a·R, значит R = aᵀ·b; угол — из следа R.
    let mut tr = 0.0;
    for i in 0..3 {
        for k in 0..3 {
            tr += a.m[k][i] * b.m[k][i];
        }
    }
    ((tr - 1.0) / 2.0).clamp(-1.0, 1.0).acos()
}

/// Вращение плавное: за кадр объект поворачивается на небольшой угол.
#[test]
fn splash_rotation_is_smooth() {
    let mut st = Splash::new();
    let mut prev = st.rot;
    for n in 0..100 {
        st.tick();
        let a = angle_between(&prev, &st.rot);
        assert!(
            a > 0.0 && a < 0.03,
            "кадр {n}: неестественный шаг поворота {a}"
        );
        prev = st.rot;
    }
}

/// Шаг вращения повторяет оригинал: за `STEP_FRAMES` кадров накапливается
/// ровно один оригинальный шаг [`STEP_ANGLE`] вокруг оси [`STEP_AXIS`]
/// (измерено по эмулятору: ≈ 10.4° за ~12 кадров, оборот ≈ 8 с).
#[test]
fn splash_rotation_matches_original_step() {
    let mut st = Splash::new();
    let start = st.rot;
    for _ in 0..STEP_FRAMES as usize {
        st.tick();
    }
    let a = angle_between(&start, &st.rot);
    assert!(
        (a - STEP_ANGLE).abs() < 1e-3,
        "за {} кадров повёрнуто на {}рад вместо {}рад",
        STEP_FRAMES,
        a,
        STEP_ANGLE
    );
    // Шаг делается в мире (левое умножение): R = rot·startᵀ. Ось
    // поворота — из антисимметричной части: [n]·sinθ = (R − Rᵀ)/2.
    let mut r = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                r[i][j] += st.rot.m[i][k] * start.m[j][k];
            }
        }
    }
    let sn = 2.0 * a.sin();
    let v = [
        (r[2][1] - r[1][2]) / sn,
        (r[0][2] - r[2][0]) / sn,
        (r[1][0] - r[0][1]) / sn,
    ];
    for i in 0..3 {
        // Допуск 1e-2, а не машинный: стартовая СКО — 7-битные байты
        // оригинала, её триада лишь приближённо ортогональна (≈1%),
        // и это же отклонение вносит погрешность в извлечённую ось.
        assert!(
            (v[i] - STEP_AXIS[i]).abs() < 1e-2,
            "ось шага не совпала с STEP_AXIS: {v:?}"
        );
    }
    // Скорость та же, что у оригинала: полный оборот ≈ 360°/10.4°·12
    // кадров ≈ 415 кадров ≈ 8.3 с при 50 Гц.
    let mut st = Splash::new();
    let start = st.rot;
    let mut per_turn = 0u32;
    while per_turn < 2000 && angle_between(&start, &st.rot) < std::f32::consts::PI {
        st.tick();
        per_turn += 1;
    }
    // Мерится полуповорот (за PI повторений точность выше, чем за
    // полный оборот): оригинальный оборот ≈ 400 кадров, то есть
    // полуповорот ≈ 200 кадров.
    let half = per_turn as f32;
    assert!(
        (190.0..230.0).contains(&half),
        "полуповорот за {half} кадров — скорость не как у оригинала (~200)"
    );
}

/// «Карусель» оригинала: ось вращения близка к направлению взгляда,
/// поэтому нос корабля (+Z модели) остаётся развёрнут от наблюдателя
/// на всём обороте — как в оригинальном интро.
#[test]
fn splash_nose_keeps_pointing_away() {
    let mut st = Splash::new();
    let intro = Rotation::intro();
    assert!(
        intro.apply([0.0, 0.0, 1.0])[2] > 0.0,
        "в стартовой позе нос должен смотреть вглубь экрана"
    );
    for n in 0..(10 * 50) {
        st.tick();
        let nose = st.rot.apply([0.0, 0.0, 1.0]);
        assert!(
            nose[2] > 0.0,
            "кадр {n}: нос развёрнут к наблюдателю ({nose:?}) — это уже не карусель оригинала"
        );
    }
}

/// Матрица поворота не «утекает»: отклонение от ортогональности не
/// растёт со временем. Регрессия к ошибке с неединичной осью в
/// `axis_angle` (там матрица расходилась с каждым кадром).
///
/// Сама стартовая СКО лишь приближённо ортогональна — это 7-битные
/// байты оригинала, поэтому сравнивается не абсолютное значение, а
/// именно отсутствие роста.
#[test]
fn splash_rotation_matrix_stays_orthonormal() {
    /// Максимальное отклонение от ортогональной матрицы.
    fn dev(m: [[f32; 3]; 3]) -> f32 {
        let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        // Столбцы, а не строки: при левом умножении (rot = A·rot)
        // сохраняется именно MᵀM, то есть ортогональность столбцов.
        let cols = [
            [m[0][0], m[1][0], m[2][0]],
            [m[0][1], m[1][1], m[2][1]],
            [m[0][2], m[1][2], m[2][2]],
        ];
        let mut d = 0.0f32;
        for i in 0..3 {
            d = d.max((dot(cols[i], cols[i]) - 1.0).abs());
            d = d.max(dot(cols[i], cols[(i + 1) % 3]).abs());
        }
        let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
        d.max((det - 1.0).abs())
    }
    let mut st = Splash::new();
    let d0 = dev(st.rot.m);
    assert!(d0 < 2e-2, "стартовая СКО слишком неортогональна: {d0}");
    for n in 0..(20 * 50) {
        st.tick();
        if n % 100 == 0 {
            let d = dev(st.rot.m);
            assert!(
                d <= d0 + 1e-3,
                "кадр {}: отклонение выросло {d0} → {d} — матрица расходится",
                n + 1
            );
        }
    }
}

/// Главная ось силуэта корабля по пикселям кадра (градусы).
///
/// Меряется так же, как силуэт оригинала в
/// `dumptitle::dump_intro_silhouette`: ковариация точек силуэта, угол
/// главной оси (определён с точностью до 180°).
fn silhouette_axis(frame: &Frame) -> f64 {
    let (w, h) = (frame.width, frame.height);
    let (mut n, mut sx, mut sy) = (0.0f64, 0.0f64, 0.0f64);
    for y in 0..h {
        for x in 0..w {
            if frame.pixels()[y * w + x] == SHIP_COLOR {
                n += 1.0;
                sx += x as f64;
                sy += y as f64;
            }
        }
    }
    assert!(n > 100.0, "корабль не нарисован: {n}");
    let (cx, cy) = (sx / n, sy / n);
    let (mut sxx, mut syy, mut sxy) = (0.0f64, 0.0f64, 0.0f64);
    for y in 0..h {
        for x in 0..w {
            if frame.pixels()[y * w + x] == SHIP_COLOR {
                let (dx, dy) = (x as f64 - cx, y as f64 - cy);
                sxx += dx * dx;
                syy += dy * dy;
                sxy += dx * dy;
            }
        }
    }
    let (sxx, syy, sxy) = (sxx / n, syy / n, sxy / n);
    0.5 * (2.0 * sxy).atan2(sxx - syy).to_degrees()
}

/// Пиксельные габариты корабля в кадре (ширина, высота).
fn ship_bbox(frame: &Frame) -> (i32, i32) {
    let (w, h) = (frame.width, frame.height);
    let (mut x0, mut x1, mut y0, mut y1) = (w as i32, -1, h as i32, -1);
    for y in 0..h {
        for x in 0..w {
            if frame.pixels()[y * w + x] == SHIP_COLOR {
                x0 = x0.min(x as i32);
                x1 = x1.max(x as i32);
                y0 = y0.min(y as i32);
                y1 = y1.max(y as i32);
            }
        }
    }
    assert!(x1 >= 0, "корабль не нарисован");
    (x1 - x0 + 1, y1 - y0 + 1)
}

/// Корабль не «дышит»: масштаб постоянный, меняется только ракурс.
///
/// Прежде габарит проекции подгонялся под `2r` **каждый кадр**, поэтому
/// при сокращении ракурса фигура раздувалась — корабль пульсировал.
/// Теперь масштаб калибруется один раз по самому широкому ракурсу за
/// оборот (он занимает ровно `2r`), остальные ракурсы меньше — это
/// естественное сокращение, как у оригинала с фиксированным расстоянием
/// до объекта. Тест проверяет, что габарит заметно меняется (то есть
/// нормировки по кадру нет) и при этом не вылезает на надписи.
#[test]
fn splash_ship_size_does_not_breathe() {
    let font = vec![0u8; 0x300];
    let mut st = Splash::new();
    let mut f = Frame::new(1280, 960);
    // Полный оборот (≈415 тиков) с шагом 3 тика.
    let (mut dmin, mut dmax) = (i32::MAX, 0);
    let (mut top, mut bottom) = (960, 0);
    for _ in 0..140 {
        f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
        st.draw(&mut f, &font);
        let (bw, bh) = ship_bbox(&f);
        let d = bw.max(bh);
        dmin = dmin.min(d);
        dmax = dmax.max(d);
        // Вертикальные габариты — для проверки, что надписи не задеты.
        for y in 0..960usize {
            for x in 0..1280usize {
                if f.pixels()[y * 1280 + x] == SHIP_COLOR {
                    top = top.min(y as i32);
                    bottom = bottom.max(y as i32);
                }
            }
        }
        for _ in 0..3 {
            st.tick();
        }
    }
    let s = 5; // масштаб сетки ZX при окне 1280x960
    assert!(
        top >= 12 * s && bottom <= 960 - 12 * s,
        "на широком ракурсе корабль залез на надписи: {top}..{bottom}"
    );
    assert!(
        dmax * 10 > dmin * 11,
        "габарит почти не меняется ({dmin}..{dmax}) — масштаб подгоняется по кадру"
    );
}

/// Наклон силуэта на экране — как в оригинале (и направление вращения).
///
/// Оригинал (замер по эмулятору: кадры 16/28/40/52 — это 1..4-й шаг
/// поворота СКО) даёт главную ось силуэта 8.75°, −11.56°, −29.70°,
/// −48.13°, то есть она **убывает**. Порт вращается плавно, поэтому
/// берутся те же позы (тики 11/23/35/47): значения должны идти в ту же
/// сторону и попадать в допуск на фазу (оригинал шагает «ступенькой»,
/// порт — непрерывно, отсюда разброс в несколько градусов).
///
/// Тест ловит ошибку знака вертикали: при печати `+Y` изображение
/// зеркалится по вертикали и ось растёт (+13°…+51°) — визуально
/// вращение идёт в обратную сторону, хотя СКО поворачивается верно.
#[test]
fn splash_silhouette_turns_like_original() {
    let font = vec![0u8; 0x300];
    let mut st = Splash::new();
    let mut f = Frame::new(1280, 960);
    let mut prev = f64::NAN;
    for (tick, expected) in [(11u64, 8.75f64), (23, -11.56), (35, -29.70), (47, -48.13)] {
        while st.tick < tick {
            st.tick();
        }
        f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
        st.draw(&mut f, &font);
        let mut ang = silhouette_axis(&f);
        // Угол оси определён с точностью до 180°.
        while ang - expected > 90.0 {
            ang -= 180.0;
        }
        while ang - expected < -90.0 {
            ang += 180.0;
        }
        assert!(
            (ang - expected).abs() < 12.0,
            "тик {tick}: ось силуэта {ang:.2}°, в оригинале {expected:.2}°"
        );
        if !prev.is_nan() {
            assert!(
                ang < prev,
                "ось силуэта не убывает, как в оригинале: {prev:.2}° → {ang:.2}°"
            );
        }
        prev = ang;
    }
}

/// Центр габаритов модели — так же, как в [`Splash::draw_ship`].
fn centered(ship: &Ship) -> [f32; 3] {
    let (mut mn, mut mx) = ([i32::MAX; 3], [i32::MIN; 3]);
    for v in ship.vertices {
        for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
            mn[k] = mn[k].min(*c as i32);
            mx[k] = mx[k].max(*c as i32);
        }
    }
    [
        (mn[0] + mx[0]) as f32 / 2.0,
        (mn[1] + mx[1]) as f32 / 2.0,
        (mn[2] + mx[2]) as f32 / 2.0,
    ]
}

/// Эталонный куб: выпуклый корпус. Здесь проверка видимости граней
/// может быть точной (нормали куба заданы ровно), поэтому известно
/// множество заведомо видимых рёбер. Удаление невидимых линий не должно
/// убирать с них ни одного пикселя.
///
/// Регрессия на слишком агрессивное удаление: раньше на гранях, почти
/// параллельных лучу зрения (глубина меняется на проценты за пиксель),
/// видимые линии пропадали.
#[test]
fn convex_cube_keeps_visible_edges() {
    let ship = cube_ship();
    let cen = centered(ship);
    for step in 0..24 {
        let mut st = Splash::new();
        for _ in 0..step * 5 {
            st.tick();
        }
        // Вершины в координатах камеры и точная видимость граней:
        // грань видна, если её нормаль смотрит на камеру.
        let cam: Vec<[f32; 3]> = ship
            .vertices
            .iter()
            .map(|v| {
                let w = st.rot.apply([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ]);
                [w[0], w[1], INTRO_DIST + w[2]]
            })
            .collect();
        let vis: Vec<bool> = (0..ship.faces.len())
            .map(|fi| {
                let f = &ship.faces[fi];
                let n = st.rot.apply([f.nx as f32, f.ny as f32, f.nz as f32]);
                let (mut c, mut cnt) = ([0.0f32; 3], 0.0f32);
                for e in ship.edges {
                    if (e.faces & 0x0F) as usize == fi || (e.faces >> 4) as usize == fi {
                        for i in [e.v[0] as usize, e.v[1] as usize] {
                            for (j, cj) in c.iter_mut().enumerate() {
                                *cj += cam[i][j];
                            }
                            cnt += 1.0;
                        }
                    }
                }
                n[0] * c[0] + n[1] * c[1] + n[2] * c[2] < 0.0 && cnt > 0.0
            })
            .collect();
        // Эталон: только заведомо видимые рёбра, без проверки глубины.
        let k = st.ship_scale(ship, cen, INTRO_DIST, 240);
        let (bx, by) = st.intro_screen_center(ship, cen, INTRO_DIST);
        let pr = Proj {
            cx: 300.0,
            cy: 300.0,
            k,
            bx,
            by,
        };
        let scr = st.project_ship(ship, cen, &pr);
        let mut reference = Frame::new(600, 600);
        for e in ship.edges {
            let (a, b) = (e.v[0] as usize, e.v[1] as usize);
            let seen = |f: usize| f == 0xF || vis.get(f).copied().unwrap_or(false);
            if seen((e.faces & 0x0F) as usize) || seen((e.faces >> 4) as usize) {
                line_hidden(&mut reference, &[], &scr, scr[a], scr[b], SHIP_COLOR);
            }
        }
        let mut f = Frame::new(600, 600);
        st.draw_ship(&mut f, ship, 300, 300, 240);
        let drawn: Vec<bool> = f.pixels().iter().map(|&p| p == SHIP_COLOR).collect();
        let (mut total, mut lost) = (0, 0);
        for (i, &p) in reference.pixels().iter().enumerate() {
            if p == SHIP_COLOR {
                total += 1;
                if !drawn[i] {
                    lost += 1;
                }
            }
        }
        assert!(total > 0, "шаг {step}: куб не нарисован");
        assert_eq!(
            lost, 0,
            "шаг {step}: удаление линий убрало {lost} из {total} пикселей видимых рёбер"
        );
    }
}

/// Куб как модель корабля для [`convex_cube_keeps_visible_edges`].
fn cube_ship() -> &'static Ship {
    use crate::shipdata::{Edge, Face, Ship, Vertex};
    static VERTS: [Vertex; 8] = [
        Vertex {
            x: -50,
            y: -50,
            z: -50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: 50,
            y: -50,
            z: -50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: 50,
            y: 50,
            z: -50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: -50,
            y: 50,
            z: -50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: -50,
            y: -50,
            z: 50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: 50,
            y: -50,
            z: 50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: 50,
            y: 50,
            z: 50,
            lod: 0x1F,
            faces: [0, 0],
        },
        Vertex {
            x: -50,
            y: 50,
            z: 50,
            lod: 0x1F,
            faces: [0, 0],
        },
    ];
    static FACES: [Face; 6] = [
        Face {
            nx: 100,
            ny: 0,
            nz: 0,
            flags: 0x1F,
        },
        Face {
            nx: -100,
            ny: 0,
            nz: 0,
            flags: 0x1F,
        },
        Face {
            nx: 0,
            ny: 100,
            nz: 0,
            flags: 0x1F,
        },
        Face {
            nx: 0,
            ny: -100,
            nz: 0,
            flags: 0x1F,
        },
        Face {
            nx: 0,
            ny: 0,
            nz: 100,
            flags: 0x1F,
        },
        Face {
            nx: 0,
            ny: 0,
            nz: -100,
            flags: 0x1F,
        },
    ];
    static EDGES: [Edge; 12] = [
        Edge {
            lod: 0x1F,
            v: [0, 1],
            faces: 5 | (3 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [1, 2],
            faces: 5 | (0 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [2, 3],
            faces: 5 | (2 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [3, 0],
            faces: 5 | (1 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [4, 5],
            faces: 4 | (3 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [5, 6],
            faces: 4 | (0 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [6, 7],
            faces: 4 | (2 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [7, 4],
            faces: 4 | (1 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [0, 4],
            faces: 3 | (1 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [1, 5],
            faces: 3 | (0 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [2, 6],
            faces: 2 | (0 << 4),
        },
        Edge {
            lod: 0x1F,
            v: [3, 7],
            faces: 2 | (1 << 4),
        },
    ];
    static SHIP: Ship = Ship {
        name: "cube",
        addr: 0,
        category: 0,
        vertices: &VERTS,
        edges: &EDGES,
        faces: &FACES,
        stats: unsafe { std::mem::zeroed() },
    };
    &SHIP
}

/// Дамп кадров анимации заставки (`ELITE_DUMP_SPLASH_FRAMES=prefix` →
/// `prefix_<номер>.ppm`, шаг 12 тиков = один шаг оригинала).
///
/// Шрифт встроен в нативный код, чтобы кадры были с надписями.
#[test]
#[ignore]
fn dump_splash_frames() {
    let Ok(prefix) = std::env::var("ELITE_DUMP_SPLASH_FRAMES") else {
        return;
    };
    let font = crate::font::ZX_FONT;
    let mut st = Splash::new();
    let mut f = Frame::new(1280, 960);
    for i in 0..8 {
        f.fill_rect(0, 0, f.width, f.height, 0xFF00_0000);
        st.draw(&mut f, font);
        write_ppm(&format!("{prefix}_{i}.ppm"), f.pixels(), 1280, 960);
        for _ in 0..12 {
            st.tick();
        }
    }
}

/// Дамп серии кадров вращения (`ELITE_DUMP_ANGLES=prefix` →
/// `prefix_0.ppm`.. ) — для визуальной проверки силуэта и анимации.
#[test]
fn dump_ship_angles() {
    let Ok(prefix) = std::env::var("ELITE_DUMP_ANGLES") else {
        return;
    };
    let mut st = Splash::new();
    for i in 0..8 {
        st.rot = yaw_rot(i as f32 * std::f32::consts::PI / 4.0);
        let mut f = Frame::new(520, 520);
        if let Some(ship) = st.ship {
            st.draw_ship(&mut f, ship, 260, 260, 240);
        }
        write_ppm(&format!("{prefix}_{i}.ppm"), f.pixels(), 520, 520);
    }
}
