
use super::indicators::missile_icon_rect;
use super::labels::{SPEED_ICON, indicator_label};
use super::layout::DANGER_ATTR;
use super::*;

/// Шрифт-заглушка: все знаки — сплошной прямоугольник.
fn font() -> Vec<u8> {
    vec![0xFFu8; 0x300]
}

/// Записать координаты объекта (X, Y, Z) в формате знак-величина.
fn put_crd(mem: &mut [u8], at: u16, x: i32, y: i32, z: i32) {
    let put = |mem: &mut [u8], at: u16, v: i32| {
        let sign = if v < 0 { 0x80 } else { 0 };
        let mag = v.unsigned_abs();
        mem[at as usize] = mag as u8;
        mem[at as usize + 1] = (mag >> 8) as u8;
        mem[at as usize + 2] = ((mag >> 16) as u8 & 0x7F) | sign;
    };
    put(mem, at, x);
    put(mem, at + 3, y);
    put(mem, at + 6, z);
}

/// Значения для тестов: пустые шкалы и полный запас ракет.
fn values(missiles: u8) -> CockpitValues {
    CockpitValues {
        left: [
            Indicator::Gauge(0),
            Indicator::Gauge(0),
            Indicator::Gauge(0),
            Indicator::Gauge(0),
            Indicator::Gauge(0),
            Indicator::Gauge(0),
            Indicator::Missiles(missiles),
        ],
        right: [Indicator::Gauge(0); PANEL_ROWS as usize],
        danger: Danger::Off,
        compass: Compass::centered(),
        radar: RadarContacts::empty(),
        blink: true,
    }
}

fn draw(w: usize, h: usize) -> (Frame, CockpitGeom) {
    let mut f = Frame::new(w, h);
    let font = font();
    Cockpit::new().draw(
        &mut f,
        &font,
        CockpitStyle::COBRA_MK3,
        &values(MISSILE_SLOTS as u8),
    );
    (f, panel_geometry(w as u32, h as u32))
}

/// Кокпит рисует главную рамку и рамку нижней части.
#[test]
fn cockpit_draws_main_and_panel_frames() {
    let (f, g) = draw(1280, 960);
    let main = g.main_frame();
    let panel = g.panel_frame();
    let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

    assert_eq!(
        px(main.x, main.y),
        FRAME_COLOR,
        "главная рамка не нарисована"
    );
    assert_eq!(
        px(main.x + main.w / 2, main.bottom()),
        FRAME_COLOR,
        "нижняя линия главной рамки не нарисована"
    );
    assert_eq!(
        px(panel.x, panel.y),
        FRAME_COLOR,
        "рамка нижней части не нарисована"
    );
    assert_eq!(
        px(panel.right(), panel.bottom()),
        FRAME_COLOR,
        "правый нижний угол нижней рамки не нарисован"
    );
}

/// Все отступы одинаковые: от краёв окна и между рамками.
#[test]
fn all_margins_are_equal() {
    let g = panel_geometry(1280, 960);
    let main = g.main_frame();
    let panel = g.panel_frame();
    // Отступ от краёв окна у обеих рамок.
    assert_eq!(main.x, g.margin, "главная рамка не на общем отступе слева");
    assert_eq!(main.y, g.margin, "главная рамка не на общем отступе сверху");
    assert_eq!(panel.x, g.margin, "нижняя рамка не на общем отступе слева");
    assert_eq!(
        g.width - 1 - panel.right(),
        g.margin,
        "нижняя рамка не на общем отступе справа"
    );
    assert_eq!(
        g.height - 1 - panel.bottom(),
        g.margin,
        "нижняя рамка не на общем отступе снизу"
    );
    // Зазор между главной рамкой и нижней — тот же отступ.
    let gap_rows = panel.y - main.bottom() - 1;
    assert_eq!(gap_rows, g.margin, "зазор между рамками не равен отступу");
    // И этот зазор действительно чёрный, а не общая линия.
    let (f, _) = draw(1280, 960);
    let mid_x = g.width / 2;
    for y in main.bottom() + 1..panel.y {
        assert_eq!(
            f.pixels()[y as usize * g.width as usize + mid_x as usize],
            BACKGROUND,
            "зазор между рамками не чёрный в строке {y}"
        );
    }
}

/// В панели по семь строк слева и справа: длинные клетки залиты, между
/// строками — белые разделители, у нижней строки разделитель — сама рамка.
#[test]
fn panel_has_seven_indicator_rows_on_both_sides() {
    let (f, g) = draw(1280, 960);
    let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            let bar = g.bar_cell(row, right);
            assert!(!bar.is_empty(), "пустая клетка {row} (right={right})");
            // Нижняя левая клетка разделена: центр полосы попадает в
            // чёрную клетку режима, поэтому проверяем позицию ракеты.
            let (px_x, px_y) = if !right && row == MISSILE_ROW {
                let slot = g.missile_cell(row, right);
                (slot.x + slot.w / 2, slot.y + slot.h / 2)
            } else {
                (bar.x + bar.w / 2, bar.y + bar.h / 2)
            };
            assert_eq!(
                px(px_x, px_y),
                BAR_RED,
                "клетка {row} (right={right}) не залита"
            );
            // Между строками — белая линия шириной с рамку.
            if row + 1 < PANEL_ROWS {
                let sep = g.row_separator(row);
                assert_eq!(
                    px(bar.x + bar.w / 2, sep.y + LINE / 2),
                    FRAME_COLOR,
                    "нет линии под строкой {row} (right={right})"
                );
            }
        }
    }
    // Нижняя строка опирается на нижнюю линию рамки панели.
    let inner = g.panel_inner();
    let last = g.side_row(PANEL_ROWS - 1);
    assert_eq!(
        last.y + last.h,
        inner.y + inner.h,
        "нижняя строка не доходит до рамки панели"
    );
}

/// Все семь строк боковых колонок одинаковой высоты, включая нижнюю.
#[test]
fn side_rows_have_equal_height() {
    let g = panel_geometry(1280, 960);
    let heights: Vec<i32> = (0..PANEL_ROWS).map(|r| g.side_row(r).h).collect();
    let min = *heights.iter().min().unwrap();
    let max = *heights.iter().max().unwrap();
    assert!(max - min <= 1, "строки разной высоты: {heights:?}");
    let inner = g.panel_inner();
    assert_eq!(g.side_row(0).y, inner.y, "первая строка не сверху панели");
    let last = g.side_row(PANEL_ROWS - 1);
    assert_eq!(
        last.y + last.h,
        inner.y + inner.h,
        "нижняя строка не доходит до рамки панели"
    );
    // Разделители стоят ровно между строками.
    for row in 0..PANEL_ROWS - 1 {
        let sep = g.row_separator(row);
        let next = g.side_row(row + 1);
        assert_eq!(sep.y, g.side_row(row).y + g.side_row(row).h);
        assert_eq!(sep.y + sep.h, next.y, "разрыв перед строкой {}", row + 1);
    }
}

/// Таблица с рамками: вертикальные разделители между всеми клетками,
/// горизонтальные — только в боковых колонках, центр — одна большая клетка.
#[test]
fn panel_is_a_table_with_borders() {
    let (f, g) = draw(1280, 960);
    let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
    let inner = g.panel_inner();
    let center = g.center_cell();
    assert!(!center.is_empty(), "центральная клетка пуста");
    assert_eq!(center.y, inner.y, "центр не начинается сверху панели");
    assert_eq!(
        center.y + center.h,
        inner.y + inner.h,
        "центр не занимает всю высоту панели"
    );
    // Вертикальные линии отделяют подпись, полосу и центр.
    for x in g.column_separators() {
        assert_eq!(
            px(x + LINE / 2, center.y + center.h / 2),
            FRAME_COLOR,
            "нет вертикального разделителя в x={x}"
        );
    }
    // В центре горизонтальных линий нет: радар рисует только свою клетку.
    let sample_x = (center.x + g.oval_radar_cell().x) / 2;
    for row in 0..PANEL_ROWS - 1 {
        let sep = g.row_separator(row);
        assert_eq!(
            px(sample_x, sep.y + LINE / 2),
            BACKGROUND,
            "горизонтальная линия пересекает центр (строка {row})"
        );
    }
}

/// Ширина и высота клеток задаются долями панели и легко меняются.
#[test]
fn indicator_size_follows_panel_fractions() {
    let g = panel_geometry(1280, 960);
    let inner = g.panel_inner();
    let bar = g.bar_cell(0, false);
    let label = g.label_cell(0, false);
    assert_eq!(bar.w, inner.w * BAR_WIDTH_FRAC / 100);
    assert_eq!(label.w, g.side_row(0).h * LABEL_WIDTH_PER_ROW_PCT / 100);
    assert!(bar.h > 0 && bar.h == g.side_row(0).h);
    // Клетки разделены вертикальной линией.
    assert_eq!(bar.x, label.x + label.w + LINE);
    // Справа — зеркально: подпись правее полосы.
    let rbar = g.bar_cell(0, true);
    let rlabel = g.label_cell(0, true);
    assert_eq!(rlabel.x, rbar.x + rbar.w + LINE);
    assert_eq!(rlabel.x + rlabel.w, inner.x + inner.w);
}

/// Нижняя левая клетка: четыре позиции ракет фиксированной ширины,
/// разделитель и клетка режима справа.
#[test]
fn missile_cell_has_four_fixed_slots_and_mode_cell() {
    let g = panel_geometry(1280, 960);
    let bar = g.bar_cell(MISSILE_ROW, false);
    let cell = g.missile_cell(MISSILE_ROW, false);
    let slot = g.missile_slot_width();
    assert_eq!(
        slot,
        g.side_row(MISSILE_ROW).h + LINE,
        "неверный шаг позиции ракеты"
    );
    assert_eq!(cell.w, slot * MISSILE_SLOTS, "не четыре позиции");
    assert_eq!(cell.x, bar.x);
    assert_eq!(cell.y, bar.y);
    assert_eq!(cell.h, bar.h);
    let mode = g.mode_cell(MISSILE_ROW, false);
    assert_eq!(
        mode.x,
        cell.x + cell.w + LINE,
        "клетка режима не сразу за ракетами"
    );
    assert_eq!(
        mode.x + mode.w,
        bar.x + bar.w,
        "клетка режима не доходит до края полосы"
    );
    assert!(mode.w > 0, "клетка режима пуста");
}

/// Ракеты — белые прямоугольники на красном фоне: максимум четыре,
/// слева направо. При уменьшении запаса правые пропадают, а фон остаётся.
#[test]
fn missiles_are_white_rectangles_on_red_background() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let cell = g.missile_cell(MISSILE_ROW, false);
    let pitch = g.missile_slot_width();
    let mode = g.mode_cell(MISSILE_ROW, false);
    for count in 0..=MISSILE_SLOTS as u8 + 1 {
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &values(count));
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];
        let filled = (count as i32).min(MISSILE_SLOTS);
        let mid_y = cell.y + cell.h / 2;
        // Фон ракетной части красный, включая пустые позиции.
        assert_eq!(px(cell.x, mid_y), BAR_RED, "нет фона слева от ракет");
        assert_eq!(
            px(cell.x + cell.w - 1, mid_y),
            BAR_RED,
            "нет фона справа от ракет"
        );
        for i in 0..MISSILE_SLOTS {
            let icon = missile_icon_rect(cell, pitch, i);
            let whites = (icon.x..icon.x + icon.w)
                .flat_map(|x| (icon.y..icon.y + icon.h).map(move |y| (x, y)))
                .filter(|&(x, y)| px(x, y) == FRAME_COLOR)
                .count();
            if i < filled {
                assert!(icon.w > icon.h, "ракета не шире своей высоты");
                assert_eq!(
                    icon.h,
                    cell.h - 2 * LINE,
                    "высота ракеты не как у индикатора"
                );
                assert!(whites > 0, "ракета {i} не нарисована");
                assert_eq!(
                    px(icon.x, icon.y),
                    FRAME_COLOR,
                    "у ракеты {i} нет белой рамки"
                );
                assert_eq!(
                    px(icon.x + icon.w / 2, icon.y + icon.h / 2),
                    BAR_RED,
                    "середина ракеты {i} не красная"
                );
            } else {
                assert_eq!(whites, 0, "лишняя ракета {i} при запасе {count}");
            }
        }
        // Между ракетами и режимом — белый разделитель.
        assert_eq!(
            px(cell.x + cell.w + LINE / 2, mid_y),
            FRAME_COLOR,
            "нет разделителя перед клеткой режима"
        );
        // Клетка режима чёрная по всей площади.
        for y in mode.y..mode.y + mode.h {
            for x in mode.x..mode.x + mode.w {
                assert_eq!(
                    px(x, y),
                    BACKGROUND,
                    "клетка режима не чёрная ({x},{y}), ракет {count}"
                );
            }
        }
    }
}

/// Шкала заполняется слева направо: пустая — весь фон полосы, полная —
/// вся ширина клетки.
#[test]
fn gauge_fills_bar_from_left() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let cell = g.bar_cell(0, false);
    let y = cell.y + cell.h / 2;
    let sample = |f: &Frame, x: i32| f.pixels()[y as usize * 1280 + x as usize];

    let mut v = values(MISSILE_SLOTS as u8);
    let mut f = Frame::new(1280, 960);
    Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
    assert_eq!(sample(&f, cell.x + cell.w / 2), BAR_RED, "0% не пустая");

    v.left[0] = Indicator::Gauge(GAUGE_MAX / 2);
    f = Frame::new(1280, 960);
    Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
    assert_eq!(
        sample(&f, cell.x + cell.w / 4),
        FRAME_COLOR,
        "нет заливки 50%"
    );
    assert_eq!(
        sample(&f, cell.x + cell.w * 3 / 4),
        BAR_RED,
        "заливка 50% ушла дальше середины"
    );

    v.left[0] = Indicator::Gauge(GAUGE_MAX);
    f = Frame::new(1280, 960);
    Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
    assert_eq!(
        sample(&f, cell.x + cell.w - 1),
        FRAME_COLOR,
        "100% не заполняет клетку"
    );
}

/// Значения не касаются верхней и нижней границы клетки: сверху и снизу
/// остаётся цветной отступ толщиной [`LINE`], как у рамки.
#[test]
fn indicators_keep_gap_from_cell_borders() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let mut v = values(MISSILE_SLOTS as u8);
    v.left[0] = Indicator::Gauge(GAUGE_MAX);
    v.right[1] = Indicator::Deflection(0);
    let mut f = Frame::new(1280, 960);
    Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
    let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

    let gauge = g.bar_cell(0, false);
    assert!(gauge.h > 2 * LINE, "клетка слишком низкая для отступа");
    let gx = gauge.x + gauge.w / 2;
    for d in 0..LINE {
        assert_eq!(px(gx, gauge.y + d), BAR_RED, "нет отступа сверху шкалы");
        assert_eq!(
            px(gx, gauge.y + gauge.h - 1 - d),
            BAR_RED,
            "нет отступа снизу шкалы"
        );
    }
    assert_eq!(
        px(gx, gauge.y + LINE),
        FRAME_COLOR,
        "заливка не начинается после отступа"
    );

    let mark = g.bar_cell(1, true);
    let mx = mark.x + mark.w / 2;
    for d in 0..LINE {
        assert_eq!(px(mx, mark.y + d), BAR_RED, "нет отступа сверху маркера");
        assert_eq!(
            px(mx, mark.y + mark.h - 1 - d),
            BAR_RED,
            "нет отступа снизу маркера"
        );
    }
    assert_eq!(
        px(mx, mark.y + LINE),
        FRAME_COLOR,
        "маркер не начинается после отступа"
    );
    assert_eq!(
        px(mx, mark.y + mark.h - 1 - LINE),
        FRAME_COLOR,
        "маркер не заканчивается до отступа"
    );
}

/// Индикатор влево/вправо ставит маркер по центру или сдвигает его.
#[test]
fn deflection_marker_moves_left_and_right() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let cell = g.bar_cell(1, true);
    let y = cell.y + cell.h / 2;
    let marker_center = |f: &Frame| {
        let xs: Vec<i32> = (cell.x..cell.x + cell.w)
            .filter(|&x| f.pixels()[y as usize * 1280 + x as usize] == FRAME_COLOR)
            .collect();
        assert!(!xs.is_empty(), "маркер не нарисован");
        (xs[0] + xs[xs.len() - 1]) / 2
    };
    let draw = |value: i8| {
        let mut v = values(MISSILE_SLOTS as u8);
        v.right[1] = Indicator::Deflection(value);
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        f
    };
    let center = cell.x + cell.w / 2;
    let neutral = draw(0);
    assert!(
        (marker_center(&neutral) - center).abs() <= 1,
        "нейтральное положение не по центру"
    );
    assert!(
        marker_center(&draw(-DEFLECTION_MAX)) < center,
        "маркер не ушёл влево"
    );
    assert!(
        marker_center(&draw(DEFLECTION_MAX)) > center,
        "маркер не ушёл вправо"
    );
}

/// Раскладка читает все значения из памяти и ограничивает их диапазоны.
#[test]
fn layout_reads_all_indicators_from_memory() {
    let l = KLADOV_COCKPIT;
    let mut mem = vec![0u8; 0x10000];
    mem[l.shield_front as usize] = 10;
    mem[l.shield_rear as usize] = 20;
    mem[l.fuel as usize] = 30;
    mem[l.temperature as usize] = 40;
    mem[l.laser_temp as usize] = 50;
    mem[l.altitude as usize] = 0xFF; // больше максимума — обрезается
    mem[l.speed as usize] = 65;
    mem[l.rotate_lr as usize] = 0x15;
    mem[l.rotate_du as usize] = 0x10;
    mem[l.shield_unit1 as usize] = 0xA0;
    mem[l.shield_unit1 as usize + 2] = 0x80;
    mem[l.shield_unit1 as usize + 4] = 0x40;
    mem[l.shield_unit1 as usize + 6] = 0;
    mem[l.missiles as usize] = 9; // больше четырёх — обрезается
    let v = l.read(&mem);
    assert_eq!(
        v.left,
        [
            Indicator::Gauge(10),
            Indicator::Gauge(20),
            Indicator::Gauge(30),
            Indicator::Gauge(40),
            Indicator::Gauge(50),
            Indicator::Gauge(GAUGE_MAX),
            Indicator::Missiles(MISSILE_SLOTS as u8),
        ]
    );
    assert_eq!(
        v.right,
        [
            Indicator::Gauge(65),
            Indicator::Deflection(0),
            Indicator::Deflection(-5),
            Indicator::Gauge(0xA0),
            Indicator::Gauge(0x80),
            Indicator::Gauge(0x40),
            Indicator::Gauge(0),
        ]
    );
}

/// Верхние кружки стоят в центральной клетке: опасность слева, компас
/// справа, оба прижаты к верхнему краю.
#[test]
fn danger_and_compass_are_in_top_corners_of_center() {
    let g = panel_geometry(1280, 960);
    let center = g.center_cell();
    let danger = g.danger_cell();
    let compass = g.compass_cell();
    for (name, cell) in [("опасность", danger), ("компас", compass)] {
        assert!(!cell.is_empty(), "{name}: пустая клетка");
        assert_eq!(cell.w, cell.h, "{name}: клетка не квадратная");
        assert!(cell.x >= center.x, "{name}: левее центра");
        assert!(
            cell.x + cell.w <= center.x + center.w,
            "{name}: правее центра"
        );
        assert!(cell.y >= center.y, "{name}: выше центра");
    }
    assert!(
        danger.x + danger.w < compass.x,
        "кружки перекрываются или стоят не по краям"
    );
}

/// Индикатор опасности — залитый круг цвета `IndDanger`; мигающий
/// красный пропадает в выключенной фазе.
#[test]
fn danger_indicator_uses_original_colors() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let cell = g.danger_cell();
    let sample = |f: &Frame| {
        f.pixels()[(cell.y + cell.h / 2) as usize * 1280 + (cell.x + cell.w / 2) as usize]
    };
    let draw = |danger: Danger, blink: bool| {
        let mut v = values(MISSILE_SLOTS as u8);
        v.danger = danger;
        v.blink = blink;
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        f
    };
    assert_eq!(sample(&draw(Danger::Off, true)), BACKGROUND);
    assert_eq!(sample(&draw(Danger::Green, true)), DANGER_GREEN);
    assert_eq!(sample(&draw(Danger::Yellow, true)), DANGER_YELLOW);
    assert_eq!(sample(&draw(Danger::Red, true)), DANGER_COLOR);
    assert_eq!(sample(&draw(Danger::BlinkingRed, true)), DANGER_COLOR);
    assert_eq!(
        sample(&draw(Danger::BlinkingRed, false)),
        BACKGROUND,
        "мигающий красный не погас"
    );
}

/// Компас: кольцо и указатель, который смещается по координатам цели.
#[test]
fn compass_ring_and_pointer_follow_target() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let cell = g.compass_cell();
    let unit = (cell.w.min(cell.h) / 20).max(1);
    let cx = cell.x + cell.w / 2;
    let cy = cell.y + cell.h / 2;
    let radius = (cell.w.min(cell.h) / 2 - 1).max(1);
    let draw = |compass: Compass| {
        let mut v = values(MISSILE_SLOTS as u8);
        v.compass = compass;
        let mut f = Frame::new(1280, 960);
        Cockpit::new().draw(&mut f, &font, CockpitStyle::COBRA_MK3, &v);
        f
    };
    let px = |f: &Frame, x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

    // Кольцо нарисовано по краям кружка.
    let ring = draw(Compass::centered());
    assert_eq!(px(&ring, cx + radius, cy), FRAME_COLOR, "нет кольца");
    assert_eq!(px(&ring, cx - radius, cy), FRAME_COLOR, "нет кольца");

    // Заполненный указатель: в центре белый ромб.
    let filled = draw(Compass::centered());
    assert_eq!(
        px(&filled, cx, cy),
        FRAME_COLOR,
        "заполненный указатель не нарисован"
    );
    // Полый указатель: в центре дырка.
    let empty = draw(Compass {
        x: 0,
        y: 0,
        filled: false,
    });
    assert_eq!(
        px(&empty, cx, cy),
        BACKGROUND,
        "полый указатель не имеет дырки"
    );

    // Смещение вправо/влево и вверх.
    let right = draw(Compass {
        x: 4,
        y: 0,
        filled: true,
    });
    assert_eq!(px(&right, cx + 4 * unit, cy), FRAME_COLOR);
    assert_eq!(
        px(&right, cx - 4 * unit, cy),
        BACKGROUND,
        "указатель не ушёл вправо"
    );
    let left = draw(Compass {
        x: -4,
        y: 0,
        filled: true,
    });
    assert_eq!(px(&left, cx - 4 * unit, cy), FRAME_COLOR);
    let up = draw(Compass {
        x: 0,
        y: 4,
        filled: true,
    });
    assert_eq!(
        px(&up, cx, cy - 4 * unit),
        FRAME_COLOR,
        "положительный y должен уводить указатель вверх"
    );
    assert_eq!(px(&up, cx, cy + 4 * unit), BACKGROUND);
}

/// Компас читает цель по логике оригинала: вне системы — фиктивные
/// координаты, рядом с планетой — станция, иначе планета.
#[test]
fn compass_reads_target_from_memory() {
    let l = KLADOV_COCKPIT;
    let mut mem = vec![0u8; 0x10000];
    let crd = |mem: &mut [u8], addr: u16, x: i32, y: i32, z: i32| {
        let put = |mem: &mut [u8], at: u16, v: i32| {
            let sign = if v < 0 { 0x80 } else { 0 };
            let mag = v.unsigned_abs();
            mem[at as usize] = mag as u8;
            mem[at as usize + 1] = (mag >> 8) as u8;
            mem[at as usize + 2] = ((mag >> 16) as u8 & 0x7F) | sign;
        };
        put(mem, addr, x);
        put(mem, addr + 3, y);
        put(mem, addr + 6, z);
    };

    // Планета впереди справа: указатель сдвинут вправо и заполнен.
    // J-драйв у станции недоступен, поэтому цель — сама планета.
    mem[l.j_drive_coriolis as usize] = 0xFF;
    crd(&mut mem, l.planet_crd, 1000, 0, 1000);
    let v = l.read(&mem);
    assert_eq!(v.compass.x, 6);
    assert_eq!(v.compass.y, 0);
    assert!(v.compass.filled);

    // Планета за спиной: указатель полый.
    crd(&mut mem, l.planet_crd, 1000, 0, -1000);
    assert!(!l.read(&mem).compass.filled);

    // Рядом с планетой цель — станция.
    mem[l.in_free_space as usize] = 0;
    mem[l.j_drive_coriolis as usize] = 0;
    mem[l.planet_crd as usize + 2] = 0;
    mem[l.planet_crd as usize + 5] = 0;
    mem[l.planet_crd as usize + 8] = 0;
    crd(&mut mem, l.coriolis_slot, 0, 1000, 1000);
    let v = l.read(&mem);
    assert_eq!(v.compass.y, 6, "компас не переключился на станцию");

    // Вне системы — фиктивные координаты, а не планета.
    mem[l.in_free_space as usize] = 1;
    crd(&mut mem, l.fake_planet_crd, 0, -1000, 1000);
    let v = l.read(&mem);
    assert_eq!(v.compass.x, 0);
    assert_eq!(v.compass.y, -6);

    // Цвет опасности берётся с экрана: `PrintIndDanger` пишет в атрибут
    // текущую фазу (мигающий красный — это чередование 0x02 и 0x00).
    mem[DANGER_ATTR as usize] = 0x06;
    assert_eq!(l.read(&mem).danger, Danger::Yellow);
    mem[DANGER_ATTR as usize] = 0x02;
    assert_eq!(l.read(&mem).danger, Danger::Red);
    mem[DANGER_ATTR as usize] = 0x00;
    assert_eq!(l.read(&mem).danger, Danger::Off);

    // Переменная `IndDanger` к концу кадра уже сброшена в «зелёный»
    // (`PrintIndDanger` делает это после печати), поэтому она не должна
    // перекрывать реально показанный на экране красный.
    mem[0x7104] = 0x04;
    mem[DANGER_ATTR as usize] = 0x02;
    assert_eq!(l.read(&mem).danger, Danger::Red);
}

/// Радар читает тот же набор слотов, что и оригинальный сканер:
/// станция, шесть основных слотов и ракета. Пустые слоты пропускаются.
#[test]
fn layout_reads_radar_contacts_from_slots() {
    const OENERG: usize = 0x22;
    const SLOT: u16 = 0x27;
    let l = KLADOV_COCKPIT;
    let mut mem = vec![0u8; 0x10000];

    // Пустая память — ни одного контакта.
    assert!(
        l.read(&mem).radar.as_slice().is_empty(),
        "радар не пуст без объектов"
    );

    // Станция справа и впереди.
    mem[l.coriolis_slot as usize + OENERG] = 0x50;
    put_crd(&mut mem, l.coriolis_slot, 1000, 0, 2000);

    // Заняты нулевой и пятый слоты кораблей, остальные пусты.
    mem[l.ship_slot0 as usize + OENERG] = 1;
    put_crd(&mut mem, l.ship_slot0, -3000, 500, 100);
    let fifth = l.ship_slot0 + 5 * SLOT;
    mem[fifth as usize + OENERG] = 1;
    put_crd(&mut mem, fifth, 0, 0, -4000);

    // Ракета.
    mem[l.missile_slot as usize + OENERG] = 1;
    put_crd(&mut mem, l.missile_slot, 100, -200, 300);

    let radar = l.read(&mem).radar;
    let c = radar.as_slice();
    assert_eq!(c.len(), 4, "прочитаны не все контакты");
    assert_eq!(c[0].kind, RadarContactKind::Station);
    assert_eq!((c[0].x, c[0].y, c[0].z), (1000, 0, 2000));
    assert_eq!(c[1].kind, RadarContactKind::Ship);
    assert_eq!(
        (c[1].x, c[1].y, c[1].z),
        (-3000, 500, 100),
        "неверные координаты или знак"
    );
    assert_eq!(c[2].kind, RadarContactKind::Ship);
    assert_eq!((c[2].x, c[2].y, c[2].z), (0, 0, -4000));
    assert_eq!(c[3].kind, RadarContactKind::Missile);
    assert_eq!((c[3].x, c[3].y, c[3].z), (100, -200, 300));
}

/// В снимок радара помещаются все восемь слотов без потерь.
#[test]
fn radar_holds_all_eight_slots() {
    const OENERG: usize = 0x22;
    const SLOT: u16 = 0x27;
    let l = KLADOV_COCKPIT;
    let mut mem = vec![0u8; 0x10000];
    mem[l.coriolis_slot as usize + OENERG] = 1;
    for i in 0..6u16 {
        mem[(l.ship_slot0 + i * SLOT) as usize + OENERG] = 1;
    }
    mem[l.missile_slot as usize + OENERG] = 1;
    assert_eq!(l.read(&mem).radar.as_slice().len(), 8);
}

/// Кокпит рисует овальный радар и показывает на нём контакты.
#[test]
fn cockpit_draws_radar_with_contacts() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let count = |f: &Frame, cell: Rect| {
        let mut n = 0;
        for y in cell.y..cell.y + cell.h {
            for x in cell.x..cell.x + cell.w {
                if f.pixels()[y as usize * 1280 + x as usize] != BACKGROUND {
                    n += 1;
                }
            }
        }
        n
    };
    let mut v = values(MISSILE_SLOTS as u8);
    let mut empty = Frame::new(1280, 960);
    Cockpit::new().draw(&mut empty, &font, CockpitStyle::COBRA_MK3, &v);
    let oval = g.oval_radar_cell();
    let oval_before = count(&empty, oval);
    assert!(oval_before > 0, "радар не нарисован");

    v.radar.push(RadarContact {
        x: 0x2000,
        y: 0,
        z: 0,
        kind: RadarContactKind::Ship,
    });
    let mut full = Frame::new(1280, 960);
    Cockpit::new().draw(&mut full, &font, CockpitStyle::COBRA_MK3, &v);
    assert!(
        count(&full, oval) > oval_before,
        "радар не показал контакт"
    );
}

/// Случайные значения заставки детерминированы и не выходят из диапазонов.
#[test]
fn splash_values_are_deterministic_and_in_range() {
    let a = CockpitValues::for_splash();
    assert_eq!(a, CockpitValues::for_splash(), "заставка будет мерцать");
    for indicator in a.left.iter().chain(a.right.iter()) {
        match *indicator {
            Indicator::Gauge(v) => assert!(v <= GAUGE_MAX),
            Indicator::Deflection(v) => {
                assert!(v.abs() <= DEFLECTION_MAX)
            }
            Indicator::Missiles(v) => assert!(v <= MISSILE_SLOTS as u8),
            Indicator::Empty => {}
        }
    }
}

/// При смене соотношения сторон клетка с текстом не меняется, а полосы
/// и центральная клетка растягиваются по ширине окна.
#[test]
fn label_cell_width_does_not_follow_aspect_ratio() {
    let narrow = panel_geometry(1280, 960);
    let wide = panel_geometry(1706, 960);
    assert_eq!(
        narrow.label_width(),
        wide.label_width(),
        "клетка с текстом растянулась вместе с окном"
    );
    assert!(
        wide.bar_width() > narrow.bar_width(),
        "длинная клетка-полоса не растянулась"
    );
    assert!(
        wide.center_cell().w > narrow.center_cell().w,
        "центральная клетка не растянулась"
    );
    // При изменении высоты окна клетка с текстом масштабируется вместе
    // со строками.
    let taller = panel_geometry(1280, 1080);
    assert!(taller.label_width() > narrow.label_width());
}

/// Подписи повторяют оригинальную панель: слева шесть обозначений и
/// значок скорости, справа `SP`, `RL`, `DC` и номера энергоблоков.
#[test]
fn indicator_labels_match_original() {
    assert_eq!(
        LEFT_LABELS,
        [
            Some("FS"),
            Some("AS"),
            Some("FU"),
            Some("CT"),
            Some("LT"),
            Some("AL"),
            None
        ]
    );
    assert_eq!(
        RIGHT_LABELS,
        [
            Some("SP"),
            Some("RL"),
            Some("DC"),
            Some("1"),
            Some("2"),
            Some("3"),
            Some("4")
        ]
    );
    assert_eq!(
        indicator_label(PANEL_ROWS - 1, false),
        None,
        "слева снизу должен быть значок скорости"
    );
    assert!(
        !LEFT_LABELS
            .iter()
            .flatten()
            .chain(RIGHT_LABELS.iter().flatten())
            .any(|s| *s == "XX"),
        "в панели осталась заглушка XX"
    );
}

/// Подписи и значок скорости рисуются белым по чёрному фону, а не по полосе.
#[test]
fn indicator_labels_are_on_black_background() {
    // Шрифт с единственным горящим пикселем в каждом знаке: так легко
    // посчитать, сколько пикселей занимает каждая подпись.
    let mut font = vec![0u8; 0x300];
    for ch in 0x20..0x80u8 {
        font[(ch - 0x20) as usize * 8] = 0x80;
    }
    let mut f = Frame::new(1280, 960);
    Cockpit::new().draw(
        &mut f,
        &font,
        CockpitStyle::COBRA_MK3,
        &values(MISSILE_SLOTS as u8),
    );
    let g = panel_geometry(1280, 960);
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            let area = g.label_cell(row, right);
            let mut lit = 0;
            for y in area.y..area.y + area.h {
                for x in area.x..area.x + area.w {
                    let p = f.pixels()[y as usize * 1280 + x as usize];
                    if p == FRAME_COLOR {
                        lit += 1;
                    } else {
                        assert_eq!(p, BACKGROUND, "подпись не на чёрном фоне");
                    }
                }
            }
            match indicator_label(row, right) {
                Some(label) => {
                    let chars = label.len() as i32;
                    let scale = (area.h / 8).min(area.w / (8 * chars).max(1)).max(1);
                    assert_eq!(
                        lit as i32,
                        chars * scale * scale,
                        "подпись {label} не нарисована (right={right}, row={row})"
                    );
                }
                None => {
                    let scale = (area.h / 8).min(area.w / 12).max(1);
                    let ink: i32 = SPEED_ICON.iter().map(|b| b.count_ones() as i32).sum();
                    assert_eq!(
                        lit as i32,
                        ink * scale * scale,
                        "значок скорости не нарисован (right={right}, row={row})"
                    );
                }
            }
        }
    }
}

/// Цвет длинных клеток — по типу корабля: заставка (Cobra Mk III) красная,
/// Cobra Mk IV — фиолетовая.
#[test]
fn bar_color_follows_ship_type() {
    let font = font();
    let g = panel_geometry(1280, 960);
    let bar = g.bar_cell(0, false);
    let mut splash = Frame::new(1280, 960);
    Cockpit::new().draw(
        &mut splash,
        &font,
        CockpitStyle::COBRA_MK3,
        &values(MISSILE_SLOTS as u8),
    );
    assert_eq!(
        splash.pixels()[(bar.y + bar.h / 2) as usize * 1280 + (bar.x + bar.w / 2) as usize],
        BAR_RED,
        "на заставке полосы должны быть красными"
    );
    let mut mk4 = Frame::new(1280, 960);
    Cockpit::new().draw(
        &mut mk4,
        &font,
        CockpitStyle::COBRA_MK4,
        &values(MISSILE_SLOTS as u8),
    );
    assert_eq!(
        mk4.pixels()[(bar.y + bar.h / 2) as usize * 1280 + (bar.x + bar.w / 2) as usize],
        BAR_MAGENTA,
        "у Cobra Mk IV полосы должны быть фиолетовыми"
    );
}

/// Нижняя часть непрозрачна по всей своей площади (закрывает подложку).
#[test]
fn cockpit_is_drawn_and_opaque() {
    let (f, g) = draw(1680, 1050);
    for y in g.panel_y..1050 {
        for x in 0..1680 {
            assert_ne!(
                f.pixels()[y as usize * 1680 + x as usize],
                0,
                "прозрачный пиксель панели ({x},{y})"
            );
        }
    }
}

/// Кокпит растянут на всю ширину окна и прижат к нижнему краю.
#[test]
fn cockpit_spans_full_width_but_is_compact() {
    let (f, g) = draw(1680, 1050);
    let mid = (g.panel_y + g.panel_h / 2) as usize * 1680;
    assert_ne!(f.pixels()[mid], 0, "левый край панели пуст");
    assert_ne!(f.pixels()[mid + 1679], 0, "правый край панели пуст");
    assert!(g.panel_h < 1050 / 3, "панель всё ещё занимает треть экрана");
    assert!(g.panel_h > 0);
    assert_eq!(g.panel_y + g.panel_h, 1050, "панель не прижата к низу окна");
}

/// Панель занимает только часть окна: над ней остаётся область заставки.
#[test]
fn cockpit_leaves_room_for_splash_title() {
    let g = panel_geometry(1920, 1080);
    assert!(g.panel_y > 1080 / 2, "под панель не осталось места");
    assert_eq!(g.panel_y + g.panel_h, 1080);
    assert!(g.cell >= 1, "знакомест должен быть не нулевым");
}

/// Цвет линий можно переопределить (красная рамка при попадании).
#[test]
fn cockpit_line_color_can_change() {
    let mut f = Frame::new(1280, 960);
    let font = font();
    Cockpit::new().draw(
        &mut f,
        &font,
        CockpitStyle {
            line: DANGER_COLOR,
            bar: BAR_RED,
        },
        &values(MISSILE_SLOTS as u8),
    );
    let g = panel_geometry(1280, 960);
    let main = g.main_frame();
    let panel = g.panel_frame();
    assert_eq!(
        f.pixels()[main.y as usize * 1280 + main.x as usize],
        DANGER_COLOR
    );
    assert_eq!(
        f.pixels()[panel.y as usize * 1280 + panel.x as usize],
        DANGER_COLOR
    );
}

/// Стиль оригинала: фон панели чёрный, линии светлые, длинные клетки
/// красные (заставка и Cobra Mk III).
#[test]
fn cockpit_style_matches_original() {
    let (f, g) = draw(1280, 960);
    let (mut black, mut light, mut red) = (0usize, 0usize, 0usize);
    for y in g.panel_y..960 {
        for x in 0..1280 {
            match f.pixels()[y as usize * 1280 + x as usize] {
                BACKGROUND => black += 1,
                FRAME_COLOR => light += 1,
                BAR_RED => red += 1,
                _ => {}
            }
        }
    }
    assert!(black > 0, "нет чёрного фона панели");
    assert!(light > 0, "панель не содержит белых линий");
    assert!(red > 0, "нет красных клеток");
    assert!(black > light + red, "панель слишком заполнена");
}

/// Геометрия не выходит за пределы окна.
#[test]
fn cockpit_panel_fits_window() {
    let g = panel_geometry(1920, 1080);
    assert!(g.panel_y >= 0, "панель начинается выше окна");
    assert!(g.panel_y + g.panel_h <= 1080, "панель выходит за окно");
    assert_eq!(g.width, 1920);
    let view = g.view();
    assert!(!view.is_empty(), "область вида пуста");
    assert_eq!(view.x, g.margin + LINE);
    assert_eq!(view.bottom(), g.main_frame().bottom() - LINE);
}
