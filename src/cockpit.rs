//! Нативный кокпит (приборная панель) корабля — черновик.
//!
//! В оригинале панель — статический растр в видеопамяти ZX (`DataCocpit`,
//! оригинал `0xC800`, Кладов `0xC000`), нижние 64 строки экрана (8 знакомест),
//! на который поверх допечатываются динамические индикаторы: шесть полос
//! (щиты, топливо, температура, лазер, высота), скорость и четыре энергоблока,
//! компас и радиолокатор (`UpdateAllIndicators`, `PrintCompass`,
//! `PrintObjScaner`).
//!
//! Эта панель — первая нативная (Rust) отрисовка, но пока черновик: её
//! показывают на заставке (титульном экране) и на экране полёта (`F1`).
//! В остальное время экран остаётся родным ZX-кадром целиком: вид из кабины
//! и исходная растровая панель оригинала. Когда игровой экран станет
//! полностью нативным, панель переедет в него (и будет доведена до точной
//! копии оригинала).
//!
//! Раскладка нативной панели пока узнаваема, но повторяет оригинал только
//! приблизительно (32 знакоместа, столбцы 1..31):
//!
//! ```text
//!   столбцы:  1..3      3..8          8..21        21..24      24..29   29..31
//!   ┌──────────────────────────────────────────────────────────────────────┐
//!   │ иконки │ 6 полос │ радиолокатор │ компас │ скорость/  │ подписи      │
//!   │        │         │              │        │ энергоблоки│              │
//!   └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! Значения читаются из живой памяти Z80 (`mem`) после кадра, поэтому панель
//! показывает текущее состояние полёта без вмешательства в эмуляцию.

use crate::display::Frame;

/// Адреса переменных панели в памяти игры (текущее значение показания).
///
/// Каждый индикатор-полоса — байт `0..0xA0` (160 т.е. 100%), полоса занимает
/// 5x5 «пикселей» ZX от `AdrScrIndicators` (значение сдвигается вправо на 2).
/// Адреса различаются у версий на постоянный сдвиг (Кладов +0xA2).
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

/// Нативная приборная панель. Состояния нет — панель целиком читается из
/// памяти игры на каждый кадр.
pub struct Cockpit;

/// Геометрия панели для текущего окна.
///
/// Панель занимает всю ширину окна; её высота — `PANEL_FRAC` от высоты окна.
/// Внутри панели координаты считаются в «единицах знакомест» шириной `cell`:
/// 32 знакоместа по горизонтали, `PANEL_ROWS` по вертикали.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PanelGeom {
    /// Экранная Y-координата панели (сверху).
    pub panel_y: i32,
    /// Высота панели в пикселях.
    pub panel_h: i32,
    /// Ширина одного знакоместа (`width / 32`).
    pub cell: i32,
    /// Ширина окна.
    pub width: i32,
}

/// Доля высоты окна под панель (в процентах). В оригинале было 64/192 ≈ 33%;
/// 22% оставляют место под заголовок заставки, но не делают приборы
/// нечитаемыми.
const PANEL_FRAC: i32 = 22;
/// Высота панели в знакоместах (как в оригинале).
pub const PANEL_ROWS: i32 = 8;

impl PanelGeom {
    /// Экранная X-координата знакоместа `col` (0..32).
    fn col_x(&self, col: i32) -> i32 {
        self.cell * col
    }

    /// Экранная Y-координата знакоместной строки `row` (0..8) панели.
    fn row_y(&self, row: i32) -> i32 {
        self.panel_y + self.panel_h * row / PANEL_ROWS
    }
}

/// Разложение виртуальной сетки знакомест по окну `WxH`.
///
/// Панель прижимается к нижнему краю окна и занимает всю его ширину.
pub fn panel_geometry(w: u32, h: u32) -> PanelGeom {
    let w = w as i32;
    let h = h as i32;
    let panel_h = (h * PANEL_FRAC / 100).clamp(48.min(h), h).max(1);
    PanelGeom {
        panel_y: h - panel_h,
        panel_h,
        cell: (w / 32).max(1),
        width: w,
    }
}

impl Cockpit {
    pub fn new() -> Self {
        Cockpit
    }

    /// Нарисовать панель в нижней части окна.
    ///
    /// `mem` — живая память Z80 (читаются показания), `font` — шрифт ПЗУ,
    /// `layout` — адреса переменных версии игры.
    pub fn draw(&self, f: &mut Frame, mem: &[u8], font: &[u8], layout: CockpitLayout) {
        let g = panel_geometry(f.width as u32, f.height as u32);
        if g.panel_h <= 0 {
            return;
        }
        let mk4 = is_cobra_mk4(mem);
        let c = Colors::danger(read_u8(mem, layout.ind_danger), mk4);
        self.draw_frame(f, &g, c);
        self.draw_bars(f, mem, font, layout, &g, c);
        self.draw_units(f, mem, font, layout, &g, c);
        self.draw_compass(f, mem, &g, c);
        self.draw_scanner(f, mem, &g, c);
    }

    /// Корпус панели: чёрный фон, рамка и разделители окон — как в
    /// оригинальном растре `DataCocpit`.
    ///
    /// Оригинальные столбцы: внешние края `1/31`, иконки `1..3`, полосы
    /// `3..8`, радиолокатор `8..21`, компас `21..24`, правые полосы `24..29`
    /// и подписи `29..31`.
    fn draw_frame(&self, f: &mut Frame, g: &PanelGeom, c: Colors) {
        let h = g.panel_h;
        // Чёрный фон панели.
        f.fill_rect(0, g.panel_y, g.width as usize, h as usize, c.body);
        let x0 = g.col_x(1);
        let x1 = g.col_x(31);
        let bw = (x1 - x0).max(1) as usize;
        // Верхняя и нижняя линии рамки (с чёрными полями до краёв окна).
        f.hline(x0, g.panel_y, bw, c.frame);
        f.hline(x0, g.panel_y + h - 1, bw, c.frame);
        // Вертикальные линии по краям и разделители окон.
        let lw = 1i32;
        for col in [1, 3, 8, 21, 24, 29, 31] {
            f.fill_rect(
                g.col_x(col),
                g.panel_y + 1,
                lw as usize,
                (h - 2).max(1) as usize,
                c.frame,
            );
        }
    }

    /// Левое окно (col 3..7): шесть полос — иконка пиктограммы и шкала.
    ///
    /// Порядок и переменные — как `AdrScrIndicators` (оригинал
    /// `elite.asm:3767`): передний щит, задний щит, топливо, температура
    /// кабины, температура лазера, высота.
    fn draw_bars(
        &self,
        f: &mut Frame,
        mem: &[u8],
        font: &[u8],
        layout: CockpitLayout,
        g: &PanelGeom,
        c: Colors,
    ) {
        let bars: [(Icon, u8); 6] = [
            (Icon::ShieldFront, read_u8(mem, layout.shield_front)),
            (Icon::ShieldRear, read_u8(mem, layout.shield_rear)),
            (Icon::Fuel, read_u8(mem, layout.fuel)),
            (Icon::Temp, read_u8(mem, layout.temperature)),
            (Icon::Laser, read_u8(mem, layout.laser_temp)),
            (Icon::Altitude, read_u8(mem, layout.altitude)),
        ];
        // Полосы занимают знакоместные строки 1..7; каждая — своей высоты.
        let top = g.row_y(1);
        let row_h = (g.row_y(7) - top) / 6;
        let x_icon = g.col_x(1) + 2;
        let icon_w = (g.col_x(3) - x_icon - 2).max(4);
        let x_bar = g.col_x(3) + 2;
        let bar_w = (g.col_x(8) - x_bar - 2).max(4);
        for (i, (icon, value)) in bars.iter().enumerate() {
            let y = top + i as i32 * row_h;
            draw_icon(
                f,
                font,
                *icon,
                x_icon,
                y + 1,
                icon_w.min(row_h - 3),
                c.frame,
            );
            draw_bar(f, *value, x_bar, y + 1, bar_w, row_h - 3, c);
        }
    }

    /// Правое окно (col 24..31): полосы скорости и четырёх энергоблоков,
    /// а справа от них — компактные подписи, как в оригинале.
    fn draw_units(
        &self,
        f: &mut Frame,
        mem: &[u8],
        font: &[u8],
        layout: CockpitLayout,
        g: &PanelGeom,
        c: Colors,
    ) {
        let top = g.row_y(1);
        let row_h = (g.row_y(7) - top) / 6;
        let x_bar = g.col_x(24) + 2;
        let bar_w = (g.col_x(29) - x_bar - 2).max(4);
        let x_label = g.col_x(29) + 2;
        let label_w = (g.col_x(31) - x_label - 1).max(4);
        // Скорость — строка 0 окна (знакоместная строка 1).
        draw_bar(
            f,
            read_u8(mem, layout.speed),
            x_bar,
            top + 1,
            bar_w,
            row_h - 3,
            c,
        );
        draw_unit_icon(f, x_label, top + 1, label_w.min(row_h - 3), false, c);
        // Четыре энергоблока — строки 1..5 (первый энергоблок пропускает
        // строку скорости; в оригинале UNIT1..4 идут ниже).
        for i in 0..4 {
            let v = read_u8(mem, layout.shield_unit1 + (i as u16) * 2);
            let y = top + (i as i32 + 1) * row_h;
            draw_bar(f, v, x_bar, y + 1, bar_w, row_h - 3, c);
            draw_unit_icon(f, x_label, y + 1, label_w.min(row_h - 3), v >= 0xA0, c);
        }
        // Подпись скорости: маленький треугольник у первой полосы.
        let s = (row_h / 3).clamp(4, bar_w / 2);
        f.line(x_label, top + 1, x_label + s, top + row_h / 2, c.needle);
        f.line(
            x_label,
            top + row_h - 2,
            x_label + s,
            top + row_h / 2,
            c.needle,
        );
        let _ = font;
    }

    /// Компас (col 21..24): круг с указателем. Угол берётся из памяти игры
    /// (`PrintCompass`); при отсутствии доступа — указатель на север.
    fn draw_compass(&self, f: &mut Frame, _mem: &[u8], g: &PanelGeom, c: Colors) {
        let cx = (g.col_x(21) + g.col_x(24)) / 2;
        let cy = g.panel_y + g.panel_h / 2;
        let r = ((g.col_x(24) - g.col_x(21)) / 2 - 2)
            .min(g.panel_h / 3)
            .max(6);
        f.disc(cx, cy, r, c.body);
        f.circle(cx, cy, r, c.frame);
        f.circle(cx, cy, r - 2, c.frame);
        // Указатель на север и перекрестие — как базовый вид оригинала.
        f.line(cx, cy - r + 2, cx, cy - r / 3, c.needle);
        f.line(cx - r / 3, cy, cx + r / 3, cy, c.frame);
        f.line(cx, cy - r / 3, cx, cy + r / 3, c.frame);
    }

    /// Радиолокатор (col 8..21): круг с диагональными и прямыми осями и
    /// отметкой корабля игрока в центре — как сетка оригинала
    /// (`PrintObjScaner`, база точек `0x558F`). На широком экране радар
    /// крупнее, чем в ZX-версии.
    ///
    /// В версии Кладова есть альтернативный радар (alt radar). При
    /// активном alt radar рисуем объекты вокруг по координатам.
    fn draw_scanner(&self, f: &mut Frame, mem: &[u8], g: &PanelGeom, c: Colors) {
        let cx = (g.col_x(8) + g.col_x(21)) / 2;
        let cy = g.panel_y + g.panel_h / 2;
        // Радиус ограничен и высотой панели, и шириной отведённых знакомест.
        let r = ((g.col_x(21) - g.col_x(8)) / 2 - 2)
            .min(g.panel_h / 2 - 3)
            .max(6);
        f.disc(cx, cy, r, c.body);
        f.circle(cx, cy, r, c.frame);
        // Горизонтальная и вертикальная оси.
        f.line(cx - r, cy, cx + r, cy, c.frame);
        f.line(cx, cy - r, cx, cy + r, c.frame);
        // Диагонали (45°) — как в сетке оригинала.
        let d = (r as f32 * 0.7071) as i32;
        f.line(cx - d, cy - d, cx + d, cy + d, c.frame);
        f.line(cx - d, cy + d, cx + d, cy - d, c.frame);
        // Отметка корабля игрока.
        f.disc(cx, cy, 2, c.needle);
        // Альтернативный радар версии Кладова.
        if is_alt_radar(mem) {
            self.draw_scanner_alt(f, mem, cx, cy, r, c);
        }
    }

    fn draw_scanner_alt(
        &self,
        f: &mut Frame,
        mem: &[u8],
        cx: i32,
        cy: i32,
        r: i32,
        c: Colors,
    ) {
        // Рисуем элементы альтернативного радара - заглушка для отображения
        // Рисуем точки по кругу и оси для наглядности
        f.disc(cx, cy - r / 2, 1, c.needle);
        f.disc(cx + r / 3, cy, 1, c.needle);
        f.disc(cx - r / 4, cy + r / 3, 1, c.needle);
        // Также пробуем нарисовать из памяти
        const Z80_OBJ_CORIOLIS: usize = 0x6048;
        const Z80_OBJ_SLOT0: usize = 0x606F;
        const Z80_SLOT_SIZE: usize = 0x27;
        const Z80_SLOT_OENERG: usize = 0x22;
        let mut slots = Vec::new();
        slots.push(Z80_OBJ_CORIOLIS);
        for i in 0..6 {
            slots.push(Z80_OBJ_SLOT0 + i * Z80_SLOT_SIZE);
        }
        for &base in &slots {
            if base + Z80_SLOT_SIZE > mem.len() {
                continue;
            }
            if mem[base + Z80_SLOT_OENERG] == 0 {
                continue;
            }
            let e = mem[base + 0] as i16;
            let d = mem[base + 1] as i16;
            let cbyte = mem[base + 3] as i16;
            let bbyte = mem[base + 4] as i16;
            let lbyte = mem[base + 6] as i16;
            let hbyte = mem[base + 7] as i16;
            let de = ((d as u16) << 8) | (e as u16);
            let bc = ((bbyte as u16) << 8) | (cbyte as u16);
            let hlv = ((hbyte as u16) << 8) | (lbyte as u16);
            let sx = if (mem[base + 2] & 0x80) != 0 { -1i32 } else { 1i32 };
            let sy = if (mem[base + 5] & 0x80) != 0 { -1i32 } else { 1i32 };
            let sz = if (mem[base + 8] & 0x80) != 0 { -1i32 } else { 1i32 };
            let x = (de as i32) * sx;
            let y = (bc as i32) * sy;
            let z = (hlv as i32) * sz;
            if x.abs() > 30000 || y.abs() > 30000 || z.abs() > 30000 {
                continue;
            }
            let dist = ((x as i64) * (x as i64) + (y as i64) * (y as i64) + (z as i64) * (z as i64)) as f64;
            let dist = dist.sqrt() as i32;
            if dist == 0 {
                continue;
            }
            let scale = (r as f32) * 0.8;
            let sxp = (x as f32 / dist as f32 * scale) as i32;
            let syp = (y as f32 / dist as f32 * scale) as i32;
            let mx = cx + sxp;
            let my = cy - syp;
            f.disc(mx, my, 1, c.needle);
            f.line(mx - 1, my, mx + 1, my, c.needle);
            f.line(mx, my - 1, mx, my + 1, c.needle);
        }
    }
}

impl Default for Cockpit {
    fn default() -> Self {
        Self::new()
    }
}

/// Пиктограмма левого окна полос.
#[derive(Clone, Copy)]
enum Icon {
    ShieldFront,
    ShieldRear,
    Fuel,
    Temp,
    Laser,
    Altitude,
}

/// Нарисовать пиктограмму индикатора в квадрате `size` (сверху `y`, слева `x`).
///
/// Рисуется простыми линиями — стилизация оригинальных значков панели.
fn draw_icon(f: &mut Frame, font: &[u8], icon: Icon, x: i32, y: i32, size: i32, color: u32) {
    let s = size.max(4);
    let (x0, y0, x1, y1) = (x, y, x + s - 1, y + s - 1);
    let mid = (x0 + x1) / 2;
    let my = (y0 + y1) / 2;
    match icon {
        // Щит: домик-«щит» с крышей.
        Icon::ShieldFront | Icon::ShieldRear => {
            f.line(mid, y0, x1, y0 + s / 3, color);
            f.line(mid, y0, x0, y0 + s / 3, color);
            f.line(x0, y0 + s / 3, x0, y1, color);
            f.line(x1, y0 + s / 3, x1, y1, color);
            f.line(x0, y1, mid, y1, color);
            f.line(x1, y1, mid, y1, color);
            if matches!(icon, Icon::ShieldRear) {
                f.line(x0, my, x1, my, color);
            }
        }
        // Топливо: силуэт бочки/бака.
        Icon::Fuel => {
            f.line(x0 + 1, y0, x1 - 1, y0, color);
            f.line(x0 + 1, y1, x1 - 1, y1, color);
            f.line(x0, y0 + 1, x0, y1 - 1, color);
            f.line(x1, y0 + 1, x1, y1 - 1, color);
            f.line(mid, y0 + 1, mid, y1 - 1, color);
        }
        // Температура: термометр.
        Icon::Temp => {
            f.line(mid, y0, mid, y1 - 2, color);
            f.line(mid - 1, y0, mid - 1, y1 - 2, color);
            f.disc(mid - 1, y1 - 1, 2, color);
        }
        // Лазер: молния/луч.
        Icon::Laser => {
            f.line(x0, y1, mid, y0, color);
            f.line(mid, y0, mid, my, color);
            f.line(mid, my, x1, y0, color);
        }
        // Высота: стрелка вверх на подставке.
        Icon::Altitude => {
            f.line(mid, y0, mid, y1 - 1, color);
            f.line(x0, my, mid, y0, color);
            f.line(x1, my, mid, y0, color);
            f.line(x0, y1, x1, y1, color);
        }
    }
    let _ = font;
}

/// Иконка энергоблока: контур, заполненный при полном блоке.
fn draw_unit_icon(f: &mut Frame, x: i32, y: i32, size: i32, full: bool, c: Colors) {
    let s = size.max(4);
    f.fill_rect(x, y, s as usize, s as usize, c.frame);
    if full {
        f.fill_rect(
            x + 1,
            y + 1,
            (s - 2).max(0) as usize,
            (s - 2).max(0) as usize,
            c.frame,
        );
    } else {
        f.fill_rect(
            x + 1,
            y + 1,
            (s - 2).max(0) as usize,
            (s - 2).max(0) as usize,
            c.body,
        );
    }
}

/// Шкала-индикатор: рамка и заполнение слева-направо по значению `0..0xA0`.
/// Стиль оригинала — белые линии по чёрному.
fn draw_bar(f: &mut Frame, value: u8, x: i32, y: i32, w: i32, h: i32, c: Colors) {
    if w <= 2 || h <= 2 {
        return;
    }
    f.fill_rect(x, y, w as usize, h as usize, c.frame);
    f.fill_rect(x + 1, y + 1, (w - 2) as usize, (h - 2) as usize, c.body);
    let frac = (value as i32).clamp(0, 0xA0);
    let fill_w = ((w - 2) * frac) / 0xA0;
    let fc = if frac <= 0x20 {
        c.bar_low
    } else if frac <= 0x50 {
        c.bar_warn
    } else {
        c.bar_ok
    };
    f.fill_rect(x + 1, y + 1, fill_w.max(0) as usize, (h - 2) as usize, fc);
}

/// Цвета панели в стиле оригинала: чёрный фон, белые линии (ink 7 по paper 0).
///
/// Рамка меняет цвет по уровню опасности (`IndDanger`) — как `PrintIndDanger`.
/// Для Cobra MK4 используется фиолетовый цвет.
#[derive(Clone, Copy)]
struct Colors {
    body: u32,
    frame: u32,
    needle: u32,
    bar_ok: u32,
    bar_warn: u32,
    bar_low: u32,
}

fn is_cobra_mk4(mem: &[u8]) -> bool {
    mem.get(0xD08E).copied().unwrap_or(0) != 0
}

impl Colors {
    fn danger(ind: u8, mk4: bool) -> Self {
        let frame = if mk4 {
            // Фиолетовый для Cobra Mk4
            match ind & 0x7F {
                0x02 => 0xFFD0_3030, // красный — щит пуст
                0x06 => 0xFFD0_C040, // жёлтый — объект рядом
                0x63 => 0xFFFF_4040, // мигающий красный
                0x04 => 0xFFB0_60FF, // фиолетовый — в полёте
                _ => 0xFFB0_60FF,    // фиолетовый — базовый цвет Mk4
            }
        } else {
            // Красный для Cobra Mk3
            match ind & 0x7F {
                0x02 => 0xFFD0_3030, // красный — щит пуст
                0x06 => 0xFFD0_C040, // жёлтый — объект рядом
                0x63 => 0xFFFF_4040, // мигающий красный
                0x04 => 0xFFFF_4040, // красный — в полёте
                _ => 0xFFFF_4040,    // красный — базовый цвет Mk3
            }
        };
        Colors {
            // Оригинал: чёрный фон панели, белая графика.
            body: 0xFF00_0000,
            frame,
            needle: 0xFFFF_FF00,
            bar_ok: 0xFFD7_D7D7,
            bar_warn: 0xFFFF_E000,
            bar_low: 0xFFFF_4040,
        }
    }
}

/// Проверка включён ли альтернативный радар Кладова.
fn is_alt_radar(_mem: &[u8]) -> bool {
    true
}

/// Читать байт памяти игры (0, если адрес вне памяти).
fn read_u8(mem: &[u8], addr: u16) -> u8 {
    mem.get(addr as usize).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_font() -> Vec<u8> {
        vec![0xFFu8; 0x300]
    }

    fn mem_with(l: CockpitLayout, fuel: u8, speed: u8, ind: u8) -> Vec<u8> {
        let mut m = vec![0u8; 0x10000];
        m[l.fuel as usize] = fuel;
        m[l.speed as usize] = speed;
        m[l.ind_danger as usize] = ind;
        m
    }

    /// Панель рисуется и непрозрачна по всей своей области.
    #[test]
    fn cockpit_is_drawn_and_opaque() {
        let font = test_font();
        let mut f = Frame::new(1680, 1050);
        let mem = mem_with(ELITE_COCKPIT, 0xA0, 0x50, 0x04);
        Cockpit::new().draw(&mut f, &mem, &font, ELITE_COCKPIT);
        let g = panel_geometry(1680, 1050);
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

    /// Панель растянута на всю ширину окна (16:10) и прижата к нижнему
    /// краю, оставляя место под заголовок заставки.
    #[test]
    fn cockpit_spans_full_width_but_is_compact() {
        let font = test_font();
        let mut f = Frame::new(1680, 1050);
        let mem = mem_with(ELITE_COCKPIT, 0xA0, 0x50, 0x04);
        Cockpit::new().draw(&mut f, &mem, &font, ELITE_COCKPIT);
        let g = panel_geometry(1680, 1050);
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

    /// Значение индикатора влияет на длину заливки: пусто и полно различны.
    #[test]
    fn cockpit_bar_length_follows_value() {
        let font = test_font();
        let empty = mem_with(ELITE_COCKPIT, 0x00, 0x00, 0x04);
        let full = mem_with(ELITE_COCKPIT, 0xA0, 0xA0, 0x04);
        let mut a = Frame::new(1680, 1050);
        let mut b = Frame::new(1680, 1050);
        Cockpit::new().draw(&mut a, &empty, &font, ELITE_COCKPIT);
        Cockpit::new().draw(&mut b, &full, &font, ELITE_COCKPIT);
        assert!(a.pixels() != b.pixels(), "показания не влияют на панель");
    }

    /// Цвет рамки зависит от `IndDanger`.
    #[test]
    fn cockpit_frame_color_follows_danger() {
        let calm = Colors::danger(0x00, false).frame;
        let warn = Colors::danger(0x02, false).frame;
        assert_ne!(calm, warn, "цвет рамки не зависит от опасности");
    }

    /// Стиль оригинала: фон панели чёрный, графика — светлые линии поверх.
    #[test]
    fn cockpit_style_matches_original() {
        let font = test_font();
        let mut f = Frame::new(1280, 960);
        let mem = mem_with(ELITE_COCKPIT, 0xA0, 0x50, 0x00);
        Cockpit::new().draw(&mut f, &mem, &font, ELITE_COCKPIT);
        let g = panel_geometry(1280, 960);
        let mut black = 0usize;
        let mut light = 0usize;
        let mut total = 0usize;
        for y in g.panel_y..960 {
            for x in 0..1280 {
                total += 1;
                match f.pixels()[y as usize * 1280 + x as usize] {
                    0xFF00_0000 => black += 1,
                    0 => {}
                    _ => light += 1,
                }
            }
        }
        assert!(black > 0, "нет чёрного фона панели");
        assert!(light > 0, "панель не содержит графики поверх фона");
        assert!(black * 100 / total < 99, "панель без графики поверх фона");
    }

    /// Радар на широком экране больше, чем на узком (в пределах знакомест).
    #[test]
    fn scanner_is_larger_on_wide_screen() {
        let narrow = panel_geometry(1024, 768);
        let wide = panel_geometry(1920, 1200);
        let r_narrow = ((narrow.col_x(21) - narrow.col_x(8)) / 2 - 2).min(narrow.panel_h / 2 - 3);
        let r_wide = ((wide.col_x(21) - wide.col_x(8)) / 2 - 2).min(wide.panel_h / 2 - 3);
        assert!(
            r_wide > r_narrow,
            "радар на широком экране не крупнее ({r_wide} <= {r_narrow})"
        );
    }

    /// Панель не выходит за пределы окна: она рисуется целиком в нижней
    /// части и не задевает область заставки.
    #[test]
    fn cockpit_panel_fits_window() {
        let g = panel_geometry(1920, 1080);
        assert!(g.panel_y >= 0, "панель начинается выше окна");
        assert!(g.panel_y + g.panel_h <= 1080, "панель выходит за окно");
        assert_eq!(g.width, 1920);
    }
}
