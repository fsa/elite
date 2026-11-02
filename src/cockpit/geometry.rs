//! Геометрия кокпита: прямоугольники вида, панели, клеток и разделителей.
//!
//! Геометрия считается один раз на кадр из размера окна. Дальше и экраны,
//! и отрисовка панели пользуются готовыми [`Rect`], а не пересчитывают
//! отступы заново.

use super::values::{MISSILE_ROW, MISSILE_SLOTS, PANEL_ROWS};

/// Толщина линий рамки в пикселях окна.
pub const LINE: i32 = 3;
/// Доля высоты окна под нижнюю часть (панель).
///
/// В оригинале было 64/192 ≈ 33%; 25% оставляют место под заголовок
/// заставки, но делают все семь строк панели одинаково высокими.
const PANEL_FRAC: i32 = 25;

/// Ширина длинной клетки-полосы в процентах от внутренней ширины панели.
///
/// Эта клетка растягивается вместе с шириной окна.
pub const BAR_WIDTH_FRAC: i32 = 15;
/// Ширина клетки подписи/значка в процентах от высоты строки.
///
/// Клетка с текстом не зависит от ширины окна: при изменении соотношения
/// сторон растягиваются только длинные клетки-полосы.
pub const LABEL_WIDTH_PER_ROW_PCT: i32 = 225;

/// Соотношение сторон классического овального радара (ширина/высота).
///
/// В оригинале сканер — плоский овал примерно 90x38 «пикселей» ZX.
const OVAL_RADAR_ASPECT_NUM: i32 = 12;
const OVAL_RADAR_ASPECT_DEN: i32 = 5;

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
            x: if right {
                inner.x + inner.w - w
            } else {
                inner.x
            },
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

    /// Ширина одной позиции запаса ракет.
    ///
    /// Позиция — это белый прямоугольник ракеты и промежуток до следующей:
    /// в оригинале иконка шириной в знакоместо чуть шире своей высоты,
    /// а между иконками остаётся цветной зазор. Поэтому шаг позиции равен
    /// высоте строки плюс [`LINE`].
    pub fn missile_slot_width(&self) -> i32 {
        (self.side_row(MISSILE_ROW).h + LINE).max(1)
    }

    /// Клетка запаса ракет — левая часть длинной клетки.
    ///
    /// Содержит до [`MISSILE_SLOTS`] фиксированных позиций; на узком окне
    /// ужимается по ширине полосы.
    pub fn missile_cell(&self, row: i32, right: bool) -> Rect {
        let bar = self.bar_cell(row, right);
        let w = (self.missile_slot_width() * MISSILE_SLOTS).min(bar.w);
        Rect {
            x: bar.x,
            y: bar.y,
            w,
            h: bar.h,
        }
    }

    /// Клетка режима — правая часть длинной клетки после ракет.
    ///
    /// Пока всегда чёрная; позже её цвет будет зависеть от режима.
    pub fn mode_cell(&self, row: i32, right: bool) -> Rect {
        let bar = self.bar_cell(row, right);
        let missiles = self.missile_cell(row, right);
        let x = missiles.x + missiles.w + LINE;
        Rect {
            x,
            y: bar.y,
            w: (bar.x + bar.w - x).max(0),
            h: bar.h,
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

    /// Квадрат кружка в верхней части центральной клетки.
    ///
    /// `diameter` — желаемый диаметр; он ужимается, чтобы два кружка
    /// поместились в центральную клетку. Кружки прижаты к верхнему краю и
    /// разнесены к левому и правому краям, как в оригинале.
    fn top_circle(&self, right: bool, diameter: i32) -> Rect {
        let center = self.center_cell();
        if center.is_empty() {
            return Rect {
                x: center.x,
                y: center.y,
                w: 0,
                h: 0,
            };
        }
        let gap = (self.side_row(0).h / 2).max(LINE);
        let d = diameter.min(center.w / 3).min(center.h / 2).max(0);
        Rect {
            x: if right {
                center.x + center.w - gap - d
            } else {
                center.x + gap
            },
            y: center.y + gap,
            w: d,
            h: d,
        }
    }

    /// Кружок индикатора опасности (верхний левый в центральной клетке).
    pub fn danger_cell(&self) -> Rect {
        self.top_circle(false, (self.side_row(0).h * 2).max(1))
    }

    /// Кружок компаса (верхний правый в центральной клетке).
    pub fn compass_cell(&self) -> Rect {
        // В оригинале компас чуть больше индикатора опасности: 20x20 против
        // 16x16 «пикселей» ZX.
        self.top_circle(true, (self.side_row(0).h * 5 / 2).max(1))
    }

    /// Свободная область под радар в центральной клетке.
    ///
    /// Классический сканер занимает всю высоту: в оригинале он был почти во
    /// всю высоту панели и даже мог наезжать на кружки по бокам. У нас
    /// центральная клетка достаточно широка, поэтому овал помещается между
    /// ними, не пересекаясь.
    pub fn radar_area(&self) -> Rect {
        let center = self.center_cell();
        if center.is_empty() {
            return center;
        }
        center.inset(self.radar_gap())
    }

    /// Клетка классического овального радара.
    pub fn oval_radar_cell(&self) -> Rect {
        fit_aspect(self.radar_area(), OVAL_RADAR_ASPECT_NUM, OVAL_RADAR_ASPECT_DEN)
    }

    /// Клетка пары индикаторов `E` (ECM) и `S` (станция).
    ///
    /// В оригинале оба символа стоят у правого нижнего края сканера: `E`
    /// слева, `S` справа. Здесь пара прижата к правому нижнему углу
    /// центральной клетки, сразу за овалом радара.
    pub fn status_cell(&self) -> Rect {
        let center = self.center_cell();
        if center.is_empty() {
            return center;
        }
        let gap = self.radar_gap();
        let glyph = (self.side_row(0).h / 8).max(1) * 8;
        let w = (glyph * 2).min((center.w - gap).max(0));
        let h = glyph.min((center.h - gap).max(0));
        Rect {
            x: (center.x + center.w - gap - w).max(center.x),
            y: (center.y + center.h - gap - h).max(center.y),
            w,
            h,
        }
    }

    /// Зазор вокруг радара.
    fn radar_gap(&self) -> i32 {
        (self.side_row(0).h / 3).max(LINE)
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

/// Вписать прямоугольник с соотношением сторон `num/den` в клетку по центру.
fn fit_aspect(cell: Rect, num: i32, den: i32) -> Rect {
    if cell.is_empty() || num <= 0 || den <= 0 {
        return Rect {
            x: cell.x,
            y: cell.y,
            w: 0,
            h: 0,
        };
    }
    // w/h = num/den, значит w = h*num/den. Сначала ограничиваем высотой,
    // затем, если не влезло по ширине, — шириной.
    let mut h = cell.h;
    let mut w = (h * num / den).max(1);
    if w > cell.w {
        w = cell.w;
        h = (w * den / num).max(1);
    }
    Rect {
        x: cell.x + (cell.w - w) / 2,
        y: cell.y + (cell.h - h) / 2,
        w,
        h,
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
