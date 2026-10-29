//! Нативный кокпит (приборная панель) корабля.
//!
//! Кокпит используется на двух экранах:
//!
//! * в полёте (`F1`) — вокруг вида из кабины и под ним;
//! * на заставке — вокруг поля с вращающимся кораблём и под ним.
//!
//! Раньше рамку вокруг вида рисовал [`crate::flight`], а нижнюю часть —
//! `Cockpit::draw`; из-за этого кокпит на заставке выглядел иначе (у неё не
//! было главной рамки). Теперь вся «обвязка» кокпита — главная рамка и рамка
//! нижней части — живёт здесь, а экраны только рисуют своё содержимое.
//!
//! В оригинале панель — статический растр в видеопамяти ZX (`DataCocpit`,
//! оригинал `0xC800`, Кладов `0xC000`), нижние 64 строки экрана; индикаторы
//! уложены в 7 строк по высоте, на которые поверх допечатываются значения:
//! шесть полос (щиты, топливо, температура, лазер, высота), скорость и четыре
//! энергоблока, компас и радиолокатор (`UpdateAllIndicators`, `PrintCompass`,
//! `PrintObjScaner`).
//!
//! Сейчас панель — таблица с рамками: слева и справа по семь строк, в центре
//! одна большая клетка на всю высоту. В каждой боковой строке две клетки:
//! короткая подпись/значок на чёрном фоне и длинная клетка-полоса. Полосы
//! залиты цветом типа корабля игрока: у Cobra Mk III красным, у Cobra Mk IV
//! фиолетовым; на заставке всегда красным. Подписи повторяют обозначения
//! оригинала: слева `FS`, `AS`, `FU`, `CT`, `LT`, `AL` и значок скорости,
//! справа `SP`, `RL`, `DC` и номера энергоблоков `1`..`4`. Они нарисованы
//! тем же шрифтом ПЗУ ZX, что и заголовок `ELITE` на заставке.
//!
//! Длинная клетка-полоса задаётся в процентах от ширины панели
//! ([`BAR_WIDTH_FRAC`]) и растягивается вместе с окном. Клетка подписи
//! фиксирована по высоте строки ([`LABEL_WIDTH_PER_ROW_PCT`]) и при смене
//! соотношения сторон не растягивается.
//!
//! Геометрия (`CockpitGeom`) отдаёт прямоугольники вида, панели, клеток и
//! разделителей, чтобы следующие шаги рисовали внутри неё, а не заново
//! считали отступы. Адреса переменных панели сохранены в [`CockpitLayout`]:
//! значения читаются из живой памяти Z80 (`mem`) и понадобятся, когда полосы
//! начнут показывать реальные показания.

use crate::display::Frame;

/// Цвет линий кокпита — «белый» ZX (ink 7), как в оригинальном растре.
pub const FRAME_COLOR: u32 = 0xFFD7_D7D7;
/// Цвет рамки при попадании (мигание опасности).
pub const DANGER_COLOR: u32 = 0xFFFF_4040;
/// Цвет фона кокпита (paper 0).
pub const BACKGROUND: u32 = 0xFF00_0000;

/// Толщина линий рамки в пикселях окна.
pub const LINE: i32 = 3;
/// Доля высоты окна под нижнюю часть (панель).
///
/// В оригинале было 64/192 ≈ 33%; 25% оставляют место под заголовок
/// заставки, но делают все семь строк панели одинаково высокими.
const PANEL_FRAC: i32 = 25;
/// Высота нижней части в знакоместах — 7 видимых строк, как в оригинале.
pub const PANEL_ROWS: i32 = 7;

/// Ширина длинной клетки-полосы в процентах от внутренней ширины панели.
///
/// Эта клетка растягивается вместе с шириной окна.
pub const BAR_WIDTH_FRAC: i32 = 15;
/// Ширина клетки подписи/значка в процентах от высоты строки.
///
/// Клетка с текстом не зависит от ширины окна: при изменении соотношения
/// сторон растягиваются только длинные клетки-полосы.
pub const LABEL_WIDTH_PER_ROW_PCT: i32 = 225;
/// Цвет длинных клеток у Cobra Mk III — красный (ZX paper 2).
pub const BAR_RED: u32 = 0xFFD7_0000;
/// Цвет длинных клеток у Cobra Mk IV — фиолетовый (ZX paper 3).
pub const BAR_MAGENTA: u32 = 0xFFD7_00D7;

/// Подписи левой колонки сверху вниз.
///
/// Последняя строка — не текст, а значок скорости ([`SPEED_ICON`]), поэтому
/// для неё указан `None`.
pub const LEFT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("FS"),
    Some("AS"),
    Some("FU"),
    Some("CT"),
    Some("LT"),
    Some("AL"),
    None,
];
/// Подписи правой колонки сверху вниз: `SP`, `RL`, `DC` и номера энергоблоков.
pub const RIGHT_LABELS: [Option<&str>; PANEL_ROWS as usize] = [
    Some("SP"),
    Some("RL"),
    Some("DC"),
    Some("1"),
    Some("2"),
    Some("3"),
    Some("4"),
];
/// Растр значка скорости из оригинальной панели: 12×5 пикселей.
///
/// Горизонтальная шкала со стрелкой влево; старший бит — левый пиксель.
const SPEED_ICON: [u16; 5] = [
    0b011000000000,
    0b010111111111,
    0b011111111111,
    0b010111111111,
    0b011000000000,
];

/// Адреса переменных панели в памяти игры (текущее значение показания).
///
/// Каждый индикатор-полоса — байт `0..0xA0` (160 т.е. 100%), полоса занимает
/// 5x5 «пикселей» ZX от `AdrScrIndicators` (значение сдвигается вправо на 2).
/// Адреса различаются у версий на постоянный сдвиг (Кладов +0xA2).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
#[derive(Clone, Copy)]
pub struct CockpitLayout {
    /// Передний щит (`ShieldFront`).
    pub shield_front: u16,
    /// Задний щит (`ShieldRear`).
    pub shield_rear: u16,
    /// Топливо (`Fuel`).
    pub fuel: u16,
    /// Температура кабины (`Temperature`).
    pub temperature: u16,
    /// Температура лазера (`TempLaser`).
    pub laser_temp: u16,
    /// Высота/дальность до поверхности (`Altitude`).
    pub altitude: u16,
    /// Скорость (`Speed`).
    pub speed: u16,
    /// Четыре энергоблока (`ShieldUnit1..4`), идут подряд с шагом 2.
    pub shield_unit1: u16,
    /// Индикатор опасности (`IndDanger`) — задаёт цвет рамки.
    pub ind_danger: u16,
}

/// Раскладка переменных панели для оригинальной версии (`elite.bin`).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const ELITE_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x7046,
    shield_rear: 0x7048,
    fuel: 0x704A,
    temperature: 0x704C,
    laser_temp: 0x704E,
    altitude: 0x7050,
    speed: 0x7052,
    shield_unit1: 0x7054,
    ind_danger: 0x7062,
};

/// Раскладка переменных панели для версии Кладова (`kladov.bin`), +0xA2.
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const KLADOV_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x70E8,
    shield_rear: 0x70EA,
    fuel: 0x70EC,
    temperature: 0x70EE,
    laser_temp: 0x70F0,
    altitude: 0x70F2,
    speed: 0x70F4,
    shield_unit1: 0x70F6,
    ind_danger: 0x7104,
};

/// Прямоугольник в пикселях окна.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    /// Прямоугольник, сжатый на `d` пикселей со всех сторон.
    pub fn inset(&self, d: i32) -> Rect {
        let d = d.max(0);
        Rect {
            x: self.x + d,
            y: self.y + d,
            w: (self.w - 2 * d).max(0),
            h: (self.h - 2 * d).max(0),
        }
    }

    /// Координата правого края (включительно).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn right(&self) -> i32 {
        self.x + self.w - 1
    }

    /// Координата нижнего края (включительно).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn bottom(&self) -> i32 {
        self.y + self.h - 1
    }

    /// Пуст ли прямоугольник (рисовать нечего).
    pub fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }
}

/// Геометрия кокпита для текущего окна.
///
/// Кокпит занимает всю ширину окна. Главная рамка окружает область вида
/// (верхнюю часть окна), нижняя часть (панель) прижата к низу окна.
/// Внутри панели координаты считаются в «единицах знакомест» шириной `cell`:
/// 32 знакоместа по горизонтали, [`PANEL_ROWS`] по вертикали — как в ZX-версии.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CockpitGeom {
    /// Высота окна.
    pub height: i32,
    /// Ширина окна.
    pub width: i32,
    /// Экранная Y-координата нижней части (сверху).
    pub panel_y: i32,
    /// Высота нижней части в пикселях.
    pub panel_h: i32,
    /// Ширина одного знакоместа (`width / 32`).
    pub cell: i32,
    /// Общий отступ: от краёв окна до обеих рамок и между рамками.
    pub margin: i32,
}

impl CockpitGeom {
    /// Экранная X-координата знакоместа `col` (0..32).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn col_x(&self, col: i32) -> i32 {
        self.cell * col
    }

    /// Экранная Y-координата знакоместной строки `row` (0..[`PANEL_ROWS`]).
    #[allow(dead_code)] // пригодится при раскладке индикаторов
    pub fn row_y(&self, row: i32) -> i32 {
        self.panel_y + self.panel_h * row / PANEL_ROWS
    }

    /// Внешняя граница главной рамки (вокруг области вида).
    ///
    /// Нижняя линия рамки касается верхней границы панели
    /// (`panel_y - 1`), как в оригинале, где панель продолжает кадр.
    /// Боковые поля и поле сверху равны [`CockpitGeom::margin`].
    pub fn main_frame(&self) -> Rect {
        let m = self.margin.max(LINE);
        Rect {
            x: m,
            y: m,
            w: (self.width - 2 * m).max(0),
            h: (self.panel_y - m).max(0),
        }
    }

    /// Внутренняя область вида — сюда рисуется 3D-сцена или заставка.
    pub fn view(&self) -> Rect {
        self.main_frame().inset(LINE)
    }

    /// Внешняя граница белой рамки нижней части.
    ///
    /// Отступ от краёв окна — тот же [`CockpitGeom::margin`], поэтому и
    /// зазор до главной рамки получается таким же.
    pub fn panel_frame(&self) -> Rect {
        let m = self.margin.max(LINE);
        let y = self.panel_y + m;
        Rect {
            x: m,
            y,
            w: (self.width - 2 * m).max(0),
            h: (self.height - m - y).max(0),
        }
    }

    /// Внутренняя область панели — без линий рамки.
    pub fn panel_inner(&self) -> Rect {
        self.panel_frame().inset(LINE)
    }

    /// Высота строки боковой колонки без горизонтального разделителя.
    ///
    /// Все семь строк одинаковой высоты (разница не больше пикселя), а
    /// остаток от деления распределяется по верхним строкам. Последняя
    /// строка упирается в нижнюю линию рамки панели, поэтому разделителя
    /// под ней нет.
    pub fn side_row(&self, row: i32) -> Rect {
        let inner = self.panel_inner();
        let row = row.clamp(0, PANEL_ROWS - 1);
        let available = (inner.h - (PANEL_ROWS - 1) * LINE).max(PANEL_ROWS);
        let base = available / PANEL_ROWS;
        let extra = available % PANEL_ROWS;
        let h = base + if row < extra { 1 } else { 0 };
        let y = inner.y + row * (base + LINE) + row.min(extra);
        Rect {
            x: inner.x,
            y,
            w: inner.w,
            h,
        }
    }

    /// Горизонтальный разделитель под строкой `row` (только боковые колонки).
    pub fn row_separator(&self, row: i32) -> Rect {
        let row = row.clamp(0, PANEL_ROWS - 2);
        let band = self.side_row(row);
        Rect {
            x: band.x,
            y: band.y + band.h,
            w: band.w,
            h: LINE,
        }
    }

    /// Ширина клетки под подпись/значок.
    ///
    /// Считается от высоты строки, а не от ширины панели, поэтому при смене
    /// соотношения сторон клетка с текстом сохраняет размер.
    pub fn label_width(&self) -> i32 {
        (self.side_row(0).h * LABEL_WIDTH_PER_ROW_PCT / 100).max(1)
    }

    /// Ширина длинной клетки-полосы.
    pub fn bar_width(&self) -> i32 {
        self.panel_inner().w * BAR_WIDTH_FRAC / 100
    }

    /// Клетка подписи/значка строки `row` (слева или справа).
    pub fn label_cell(&self, row: i32, right: bool) -> Rect {
        let inner = self.panel_inner();
        let band = self.side_row(row);
        let w = self.label_width();
        Rect {
            x: if right { inner.x + inner.w - w } else { inner.x },
            y: band.y,
            w,
            h: band.h,
        }
    }

    /// Длинная клетка-полоса строки `row` (слева или справа).
    pub fn bar_cell(&self, row: i32, right: bool) -> Rect {
        let inner = self.panel_inner();
        let band = self.side_row(row);
        let lw = self.label_width();
        let bw = self.bar_width();
        Rect {
            x: if right {
                inner.x + inner.w - lw - LINE - bw
            } else {
                inner.x + lw + LINE
            },
            y: band.y,
            w: bw,
            h: band.h,
        }
    }

    /// Большая центральная клетка на всю высоту панели.
    pub fn center_cell(&self) -> Rect {
        let inner = self.panel_inner();
        let side = self.label_width() + LINE + self.bar_width() + LINE;
        Rect {
            x: inner.x + side,
            y: inner.y,
            w: (inner.w - 2 * side).max(0),
            h: inner.h,
        }
    }

    /// X-координаты четырёх вертикальных разделителей таблицы.
    pub fn column_separators(&self) -> [i32; 4] {
        let inner = self.panel_inner();
        let lw = self.label_width();
        let bw = self.bar_width();
        [
            inner.x + lw,
            inner.x + lw + LINE + bw,
            inner.x + inner.w - lw - LINE - bw - LINE,
            inner.x + inner.w - lw - LINE,
        ]
    }
}

/// Разложение виртуальной сетки знакомест по окну `WxH`.
///
/// Панель прижимается к нижнему краю окна и занимает всю его ширину.
pub fn panel_geometry(w: u32, h: u32) -> CockpitGeom {
    let w = w as i32;
    let h = h as i32;
    let panel_h = (h * PANEL_FRAC / 100).clamp(48.min(h), h).max(1);
    let cell = (w / 32).max(1);
    // Один «шаг» отступа — четверть знакоместа: на большом окне обе рамки
    // заметно отодвинуты от краёв, на маленьком — хотя бы две толщины линии.
    let margin = (cell / 4).clamp(LINE * 2, 32);
    CockpitGeom {
        height: h,
        width: w,
        panel_y: h - panel_h,
        panel_h,
        cell,
        margin,
    }
}

/// Нативное оформление кокпита. Состояния нет — рамки статичны, а экраны
/// рисуют своё содержимое до вызова [`Cockpit::draw`].
pub struct Cockpit;

impl Cockpit {
    pub fn new() -> Self {
        Cockpit
    }

    /// Нарисовать кокпит белыми линиями; длинные клетки — красные (Cobra
    /// Mk III, а также заставка).
    ///
    /// `font` — знакогенератор ПЗУ ZX (коды 0x20..0x7F) для подписей.
    pub fn draw(&self, f: &mut Frame, font: &[u8]) {
        self.draw_themed(f, FRAME_COLOR, BAR_RED, font);
    }

    /// Нарисовать кокпит линиями цвета `color` (например, красными при
    /// попадании по кораблю); длинные клетки остаются красными.
    pub fn draw_colored(&self, f: &mut Frame, color: u32, font: &[u8]) {
        self.draw_themed(f, color, BAR_RED, font);
    }

    /// Нарисовать кокпит с заданными цветом линий и цветом длинных клеток.
    ///
    /// `bar_color` — цвет полос по типу корабля игрока: [`BAR_RED`] для
    /// Cobra Mk III и [`BAR_MAGENTA`] для Cobra Mk IV.
    pub fn draw_themed(&self, f: &mut Frame, line_color: u32, bar_color: u32, font: &[u8]) {
        let g = panel_geometry(f.width as u32, f.height as u32);
        self.draw_with_geom(f, &g, line_color, bar_color, font);
    }

    /// Нарисовать кокпит по заранее вычисленной геометрии.
    ///
    /// Содержимое вида (космос, заставка) должно быть уже нарисовано:
    /// кокпит только заливает фон нижней части, рисует две рамки и таблицу
    /// с индикаторами в нижней части.
    pub fn draw_with_geom(
        &self,
        f: &mut Frame,
        g: &CockpitGeom,
        line_color: u32,
        bar_color: u32,
        font: &[u8],
    ) {
        // Нижняя часть непрозрачна и закрывает всё, что под ней.
        if g.panel_h > 0 && g.width > 0 {
            f.fill_rect(
                0,
                g.panel_y,
                g.width as usize,
                g.panel_h as usize,
                BACKGROUND,
            );
        }
        // Главная рамка вокруг вида и отдельная рамка нижней части с
        // отступом от неё.
        stroke_rect(f, g.main_frame(), LINE, line_color);
        stroke_rect(f, g.panel_frame(), LINE, line_color);
        draw_panel(f, g, line_color, bar_color, font);
    }
}

/// Нарисовать таблицу панели: длинные клетки, разделители и подписи.
fn draw_panel(f: &mut Frame, g: &CockpitGeom, line_color: u32, bar_color: u32, font: &[u8]) {
    let inner = g.panel_inner();
    if inner.is_empty() {
        return;
    }
    // Длинные клетки с будущими индикаторами — цвет типа корабля.
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            let bar = g.bar_cell(row, right);
            if !bar.is_empty() {
                f.fill_rect(
                    bar.x,
                    bar.y,
                    bar.w as usize,
                    bar.h as usize,
                    bar_color,
                );
            }
        }
    }
    // Горизонтальные разделители только в боковых колонках: центральная
    // клетка одна на все семь строк.
    let center = g.center_cell();
    for row in 0..PANEL_ROWS - 1 {
        let sep = g.row_separator(row);
        let left_w = (center.x - inner.x).max(0);
        if left_w > 0 {
            f.fill_rect(
                inner.x,
                sep.y,
                left_w as usize,
                sep.h as usize,
                line_color,
            );
        }
        let right_x = center.x + center.w;
        let right_w = (inner.x + inner.w - right_x).max(0);
        if right_w > 0 {
            f.fill_rect(
                right_x,
                sep.y,
                right_w as usize,
                sep.h as usize,
                line_color,
            );
        }
    }
    // Вертикальные разделители между всеми клетками таблицы.
    for x in g.column_separators() {
        f.fill_rect(x, inner.y, LINE as usize, inner.h as usize, line_color);
    }
    // Подписи/значки на чёрном фоне коротких клеток.
    for right in [false, true] {
        for row in 0..PANEL_ROWS {
            draw_indicator_label(f, g, row, right, line_color, font);
        }
    }
}

/// Подпись индикатора строки `row` (слева или справа).
///
/// `None` означает, что в клетке вместо текста рисуется значок скорости.
fn indicator_label(row: i32, right: bool) -> Option<&'static str> {
    let table = if right { &RIGHT_LABELS } else { &LEFT_LABELS };
    table.get(row as usize).copied().flatten()
}

/// Нарисовать подпись индикатора (или значок скорости) по центру короткой клетки.
fn draw_indicator_label(
    f: &mut Frame,
    g: &CockpitGeom,
    row: i32,
    right: bool,
    line_color: u32,
    font: &[u8],
) {
    let area = g.label_cell(row, right);
    if area.is_empty() {
        return;
    }
    let Some(label) = indicator_label(row, right) else {
        draw_speed_icon(f, area, line_color);
        return;
    };
    if font.is_empty() {
        return;
    }
    let chars = label.len() as i32;
    let scale = (area.h / 8).min(area.w / (8 * chars).max(1)).max(1);
    let text_w = 8 * scale * chars;
    let text_h = 8 * scale;
    let x = area.x + (area.w - text_w) / 2;
    let y = area.y + (area.h - text_h) / 2;
    // На очень маленьких окнах подпись может не поместиться в клетку —
    // обрезаем её, чтобы не залезть на соседние строки и полосы.
    f.set_clip(area.x, area.y, area.w.max(0) as usize, area.h.max(0) as usize);
    f.draw_text(font, x, y, label, line_color, scale as usize);
    f.clear_clip();
}

/// Нарисовать значок скорости по центру короткой клетки.
///
/// Масштаб тот же, что у подписей: один пиксель растра — [`LINE`] и более
/// пикселей окна, в зависимости от высоты строки.
fn draw_speed_icon(f: &mut Frame, area: Rect, color: u32) {
    const W: i32 = 12;
    const H: i32 = SPEED_ICON.len() as i32;
    let scale = (area.h / 8).min(area.w / W).max(1);
    let w = W * scale;
    let h = H * scale;
    let x = area.x + (area.w - w) / 2;
    let y = area.y + (area.h - h) / 2;
    f.set_clip(area.x, area.y, area.w.max(0) as usize, area.h.max(0) as usize);
    for (row, bits) in SPEED_ICON.iter().enumerate() {
        for col in 0..W {
            if bits & (1 << (W - 1 - col)) != 0 {
                f.fill_rect(
                    x + col * scale,
                    y + row as i32 * scale,
                    scale as usize,
                    scale as usize,
                    color,
                );
            }
        }
    }
    f.clear_clip();
}

impl Default for Cockpit {
    fn default() -> Self {
        Self::new()
    }
}

/// Обвести прямоугольник `r` рамкой толщиной `t`.
fn stroke_rect(f: &mut Frame, r: Rect, t: i32, color: u32) {
    if r.is_empty() {
        return;
    }
    let t = t.min(r.w).min(r.h).max(1);
    // Верхняя и нижняя полосы.
    f.fill_rect(r.x, r.y, r.w as usize, t as usize, color);
    f.fill_rect(r.x, r.y + r.h - t, r.w as usize, t as usize, color);
    // Боковые полосы между ними.
    let mid_h = (r.h - 2 * t).max(0) as usize;
    if mid_h > 0 {
        f.fill_rect(r.x, r.y + t, t as usize, mid_h, color);
        f.fill_rect(r.x + r.w - t, r.y + t, t as usize, mid_h, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Шрифт-заглушка: все знаки — сплошной прямоугольник.
    fn font() -> Vec<u8> {
        vec![0xFFu8; 0x300]
    }

    fn draw(w: usize, h: usize) -> (Frame, CockpitGeom) {
        let mut f = Frame::new(w, h);
        let font = font();
        Cockpit::new().draw(&mut f, &font);
        (f, panel_geometry(w as u32, h as u32))
    }

    /// Кокпит рисует главную рамку и рамку нижней части.
    #[test]
    fn cockpit_draws_main_and_panel_frames() {
        let (f, g) = draw(1280, 960);
        let main = g.main_frame();
        let panel = g.panel_frame();
        let px = |x: i32, y: i32| f.pixels()[y as usize * 1280 + x as usize];

        assert_eq!(px(main.x, main.y), FRAME_COLOR, "главная рамка не нарисована");
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
                assert_eq!(
                    px(bar.x + bar.w / 2, bar.y + bar.h / 2),
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
        // В центре горизонтальных линий нет.
        for row in 0..PANEL_ROWS - 1 {
            let sep = g.row_separator(row);
            assert_eq!(
                px(center.x + center.w / 2, sep.y + LINE / 2),
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
        Cockpit::new().draw(&mut f, &font);
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
        Cockpit::new().draw(&mut splash, &font);
        assert_eq!(
            splash.pixels()[(bar.y + bar.h / 2) as usize * 1280
                + (bar.x + bar.w / 2) as usize],
            BAR_RED,
            "на заставке полосы должны быть красными"
        );
        let mut mk4 = Frame::new(1280, 960);
        Cockpit::new().draw_themed(&mut mk4, FRAME_COLOR, BAR_MAGENTA, &font);
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
        assert_eq!(
            g.panel_y + g.panel_h,
            1050,
            "панель не прижата к низу окна"
        );
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
        Cockpit::new().draw_colored(&mut f, DANGER_COLOR, &font);
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
}
