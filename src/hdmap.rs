//! Отдельная (нативная) карта галактики — клавиша F10.
//!
//! Это независимый от эмуляции просмотрщик: при включении окно целиком
//! занимает нативный рендер (ZX-кадр не показывается), при выключении
//! возвращается обычный экран игры. Старые карты игры (клавиши I/O и
//! локальная карта) не затрагиваются.
//!
//! Модель галактики берётся из [`crate::galaxy`] — те же координаты и имена,
//! что строит Z80-код игры. Важная особенность: поле карты по горизонтали
//! `0..255`, а по вертикали — вдвое меньше (`(y_raw >> 1)`, 128 строк),
//! поэтому галактика вытянута по горизонтали. Поле `256x128` вписывается в
//! окно с сохранением пропорций (круги планет остаются круглыми), то есть по
//! осям получаются разные масштабы.
//!
//! Карта «фиксированная»: положение поля не зависит от курсора, а зум всегда
//! идёт вокруг текущей системы (планеты, где стоит корабль). Колесо мыши
//! (или `+`/`-`) меняет масштаб `1..max`; максимум подобран так, чтобы в кадр
//! помещался полный радиус прыжка на бак топлива (7.0 св. года) — это уровень
//! детализации карты по `O`. Планеты, чей центр выходит за видимую область,
//! не рисуются; оставшиеся вписаны внутрь кадра, чтобы не налезать на рамку.
//! Вокруг текущей системы рисуется круг дальности прыжка (как в карте `I`:
//! радиус `fuel >> 2` единиц поля).

use crate::display::Frame;
use crate::galaxy::{self, GalaxyLayout, Planet, FIELD_HEIGHT, FIELD_TOP};

/// Фон карты.
const BG: u32 = 0xFF00_0008;
/// Фон поля галактики.
const FIELD_BG: u32 = 0xFF00_1018;
/// Рамка поля.
const BORDER: u32 = 0xFF30_6070;
/// Радиус планеты: доля игрового радиуса (`3..5`) на единицу масштаба поля.
const PLANET_RADIUS: f32 = 0.16;
/// Кольцо вокруг текущей системы (ЛАВЕ и т.п.).
const SYS_RING: u32 = 0xFFFF_FF80;
/// Кольцо вокруг планеты, ближайшей к курсору.
const NEAR_RING: u32 = 0xFF40_FFC0;
/// Перекрестие курсора.
const CURSOR: u32 = 0xFF00_FF00;
/// Круг дальности прыжка.
const RANGE: u32 = 0xFF60_A0C0;
/// Текст.
const TEXT: u32 = 0xFFE0_E8F0;
/// Приглушённый текст.
const TEXT_DIM: u32 = 0xFF80_9098;

/// Минимальный зум — вся галактика.
const ZOOM_MIN: f32 = 1.0;
/// Шаг зума (множитель за одну ступень колеса).
const ZOOM_STEP: f32 = 1.25;
/// Полуразмер видимой области поля на максимальном зуме (в единицах поля).
/// Взят так, чтобы на максимуме помещался круг дальности прыжка на полный бак
/// (7.0 св. года ~ 17.5 единиц) — это соответствует детальной карте по `O`.
const DETAIL_HALF_UNITS: f32 = 20.0;

/// Радиус кружка планеты в пикселях при масштабе поля `scale`.
fn planet_px(p: &Planet, scale: f32) -> i32 {
    ((p.radius() as f32 * scale * PLANET_RADIUS).round() as i32).max(1)
}

/// Геометрия окна, не зависящая от зума и положения камеры.
#[derive(Clone, Copy)]
struct View {
    line: i32,
    top: i32,
    panel_y: i32,
    vx0: i32,
    vy0: i32,
    vw: i32,
    vh: i32,
    /// Масштаб «вся галактика» (зум = 1).
    base: f32,
    /// Центр области показа в пикселях окна.
    vcx: f32,
    vcy: f32,
    text: usize,
}

/// Сосчитать геометрию окна `w` x `h`.
fn view(w: u32, h: u32) -> View {
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    // Полоса под заголовок: строка текста плюс отступы сверху и снизу,
    // чтобы заголовок не налезал на поле (важно при крупном шрифте —
    // на 16:9/16:10 окнах `text` = 3 и больше).
    let top = 12 * text as i32 + 8;
    let panel_h = line * 3 + 8 * text as i32;
    let panel_y = h as i32 - panel_h;
    let vx0 = 0;
    let vy0 = top;
    let vw = w as i32;
    let vh = (panel_y - top).max(1);
    let base = (w as f32 / 256.0)
        .min(vh as f32 / FIELD_HEIGHT as f32)
        .max(1.0);
    View {
        line,
        top,
        panel_y,
        vx0,
        vy0,
        vw,
        vh,
        base,
        vcx: vw as f32 / 2.0,
        vcy: vy0 as f32 + vh as f32 / 2.0,
        text,
    }
}

/// Ограничить центр камеры так, чтобы поле не отрывалось от краёв окна
/// (когда поле меньше окна — оно просто по центру).
fn clamp_center(v: &View, scale: f32, cx: f32, cr: f32) -> (f32, f32) {
    let half_w = (v.vw as f32 / (2.0 * scale)).min(128.0);
    let half_h = (v.vh as f32 / (2.0 * scale)).min(64.0);
    (cx.clamp(half_w, 256.0 - half_w), cr.clamp(half_h, 128.0 - half_h))
}

/// Максимальный зум для окна: масштаб, при котором меньшая сторона области
/// показа покрывает [`DETAIL_HALF_UNITS`] единиц поля с каждой стороны от
/// текущей системы. Так на максимуме виден полный радиус прыжка (7 св. лет),
/// что соответствует детальной карте по `O`.
fn max_zoom(v: &View) -> f32 {
    let target = v.vw.min(v.vh) as f32 / (2.0 * DETAIL_HALF_UNITS);
    (target / v.base).max(ZOOM_MIN)
}

/// Зажать `v` в `lo..=hi`. Если диапазон пуст (`lo > hi`), вернуть `v`
/// без изменений (такое случается в очень маленьком окне).
fn clamp_inside(v: i32, lo: i32, hi: i32) -> i32 {
    if lo > hi {
        v
    } else {
        v.clamp(lo, hi)
    }
}

/// Раскладка поля галактики `256x128` в окне (с учётом зума).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapLayout {
    /// Пикселей на единицу поля (одинаково по обеим осям).
    pub scale: f32,
    /// Прямоугольник всего поля в пикселях окна (может выходить за окно).
    pub fx0: i32,
    pub fy0: i32,
    pub fw: i32,
    pub fh: i32,
    /// Область показа (viewport) — сюда обрезается рисование поля.
    pub vx0: i32,
    pub vy0: i32,
    pub vw: i32,
    pub vh: i32,
    /// Масштаб шрифта интерфейса (заголовок и панель).
    pub text: usize,
    /// Верхняя граница нижней панели.
    pub panel_y: i32,
    /// Высота верхней полосы (под заголовок).
    pub top: i32,
    /// Верх заголовка (по вертикали он центрируется в верхней полосе).
    pub title_y: i32,
}

impl MapLayout {
    /// Пиксель центра звезды на экранной строке `row` (`0..127`).
    pub fn star_px(&self, x: i32, row: i32) -> (i32, i32) {
        (
            (self.fx0 as f32 + x as f32 * self.scale).round() as i32,
            (self.fy0 as f32 + row as f32 * self.scale).round() as i32,
        )
    }

    /// Обратное преобразование пикселя окна в координаты поля `(x, строка)`.
    pub fn field_at(&self, win_x: f64, win_y: f64) -> (f64, f64) {
        (
            (win_x - self.fx0 as f64) / self.scale as f64,
            (win_y - self.fy0 as f64) / self.scale as f64,
        )
    }

    /// Видимая часть поля: пересечение поля и области показа `(x0, y0, x1, y1)`.
    pub fn visible_rect(&self) -> (i32, i32, i32, i32) {
        (
            self.fx0.max(self.vx0),
            self.fy0.max(self.vy0),
            (self.fx0 + self.fw).min(self.vx0 + self.vw),
            (self.fy0 + self.fh).min(self.vy0 + self.vh),
        )
    }
}

/// Состояние и отрисовка карты.
pub struct Map {
    /// Показывается ли карта сейчас.
    pub active: bool,
    layout: GalaxyLayout,
    planets: Vec<Planet>,
    digraphs: Vec<u8>,
    cursor_x: i32,
    cursor_y_raw: i32,
    sys_x: i32,
    sys_y_raw: i32,
    /// Номер галактики (`PilotNumGalaxy`, 0..7).
    galaxy_num: u8,
    /// Топливо (`Pilot_Fuel`) — задаёт круг дальности прыжка.
    fuel: u8,
    /// Множитель зума (1.0 — вся галактика).
    zoom: f32,
    /// Размер области рисования прошлого кадра (для попадания мышью).
    win_w: u32,
    win_h: u32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            active: false,
            layout: galaxy::KLADOV_GALAXY,
            planets: Vec::new(),
            digraphs: Vec::new(),
            cursor_x: 0,
            cursor_y_raw: 0,
            sys_x: 0,
            sys_y_raw: 0,
            galaxy_num: 0,
            fuel: 0,
            zoom: ZOOM_MIN,
            win_w: 0,
            win_h: 0,
        }
    }

    /// Открыть карту: собрать галактику из памяти запущенной игры.
    pub fn open(&mut self, mem: &[u8], layout: GalaxyLayout) {
        self.layout = layout;
        let seed: [u8; 6] = mem[layout.seed..layout.seed + 6].try_into().unwrap();
        self.digraphs = mem[layout.digraphs..layout.digraphs + 64].to_vec();
        self.planets = galaxy::planets(seed);
        self.sys_x = mem[layout.sys_x] as i32;
        self.sys_y_raw = mem[layout.sys_y] as i32;
        self.galaxy_num = mem[layout.galaxy] & 7;
        self.fuel = mem[layout.fuel];
        self.cursor_x = self.sys_x;
        self.cursor_y_raw = self.sys_y_raw;
        self.zoom = ZOOM_MIN;
        self.active = true;
    }

    /// Закрыть карту (возврат к экрану игры).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Запомнить размер окна (до первого кадра), чтобы попадание мышью и
    /// зум работали сразу после открытия.
    pub fn resize(&mut self, w: u32, h: u32) {
        self.win_w = w;
        self.win_h = h;
    }

    /// Раскладка поля `256x128` в окне `w` x `h` с сохранением пропорций.
    /// Камера всегда центрирована на текущей системе (с зажимом у краёв поля).
    pub fn layout(&self, w: u32, h: u32) -> MapLayout {
        let v = view(w, h);
        let scale = v.base * self.zoom;
        let sys_row = (self.sys_y_raw >> 1) as f32;
        let (cam_x, cam_row) =
            clamp_center(&v, scale, self.sys_x as f32, sys_row);
        MapLayout {
            scale,
            fx0: (v.vcx - cam_x * scale).round() as i32,
            fy0: (v.vcy - cam_row * scale).round() as i32,
            fw: (256.0 * scale).round() as i32,
            fh: (FIELD_HEIGHT as f32 * scale).round() as i32,
            vx0: v.vx0,
            vy0: v.vy0,
            vw: v.vw,
            vh: v.vh,
            text: v.text,
            panel_y: v.panel_y,
            top: v.top,
            title_y: (v.top - v.line) / 2,
        }
    }

    /// Сдвинуть курсор (координаты поля: `x` 0..255, `y_raw` 0..255).
    /// Курсор остаётся в видимой области карты.
    pub fn move_by(&mut self, dx: i32, dy_raw: i32) {
        self.cursor_x = (self.cursor_x + dx).clamp(0, 255);
        self.cursor_y_raw = (self.cursor_y_raw + dy_raw).clamp(0, 255);
        self.clamp_cursor_to_view();
    }

    /// Число звёзд в построенной галактике (всегда [`galaxy::COUNT`]).
    #[allow(dead_code)] // используется тестами и внешними просмотрщиками
    pub fn planet_count(&self) -> usize {
        self.planets.len()
    }

    /// Номер текущей галактики для показа (`1..8`).
    pub fn galaxy_number(&self) -> u8 {
        (self.galaxy_num & 7) + 1
    }

    /// Текущий множитель зума (1.0 — вся галактика).
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Радиус круга дальности прыжка (в пикселях) для текущего топлива.
    pub fn range_px(&self, scale: f32) -> i32 {
        (galaxy::jump_radius(self.fuel) as f32 * scale).round() as i32
    }

    /// Поле вокруг планеты (в пикселях) — кружок плюс кольца. Планеты
    /// прижимаются внутрь кадра с этим запасом, чтобы не налезать на рамку.
    fn planet_margin(&self, scale: f32) -> i32 {
        self.planets
            .iter()
            .map(|p| planet_px(p, scale))
            .max()
            .unwrap_or(0)
            + 3
    }

    /// Экранная позиция планеты для отрисовки. `None`, если её центр вне
    /// области показа; иначе позиция, зажатая внутрь кадра с полем `margin`.
    fn place_planet(
        &self,
        l: &MapLayout,
        x: i32,
        row: i32,
        margin: i32,
    ) -> Option<(i32, i32)> {
        let (vx0, vy0, vx1, vy1) = l.visible_rect();
        let (sx, sy) = l.star_px(x, row);
        if sx < vx0 || sx >= vx1 || sy < vy0 || sy >= vy1 {
            return None;
        }
        Some((
            clamp_inside(sx, vx0 + margin, vx1 - 1 - margin),
            clamp_inside(sy, vy0 + margin, vy1 - 1 - margin),
        ))
    }

    /// Максимальный зум для текущего размера окна: масштаб, при котором в кадр
    /// помещается полный радиус прыжка (детализация карты `O`).
    pub fn max_zoom(&self) -> f32 {
        max_zoom(&view(self.win_w.max(1), self.win_h.max(1)))
    }

    /// Изменить зум на `steps` шагов колеса. Карта всегда центрирована на
    /// текущей системе, поэтому меняется только масштаб поля.
    pub fn zoom_by(&mut self, steps: i32) {
        let zmax = self.max_zoom();
        self.zoom = (self.zoom * ZOOM_STEP.powi(steps)).clamp(ZOOM_MIN, zmax);
        self.clamp_cursor_to_view();
    }

    /// Текущее положение курсора карты `(x, y_raw)`.
    pub fn cursor(&self) -> (i32, i32) {
        (self.cursor_x, self.cursor_y_raw)
    }

    /// Звезда, ближайшая к курсору (метрика игры).
    pub fn nearest_index(&self) -> Option<usize> {
        galaxy::nearest(&self.planets, self.layout.metric, self.cursor_x, self.cursor_y_raw)
            .map(|p| p.index)
    }

    /// Текущая звезда (система, в которой находится корабль).
    pub fn system_index(&self) -> Option<usize> {
        galaxy::nearest(&self.planets, self.layout.metric, self.sys_x, self.sys_y_raw)
            .map(|p| p.index)
    }

    /// Имя планеты по её номеру.
    pub fn name_of(&self, index: usize) -> String {
        galaxy::name(self.planets[index].state, &self.digraphs)
    }

    /// Координаты системы `(x, y_raw)` выбранной курсором планеты.
    ///
    /// Это те же числа, что игра хранит в `Pilot_CrdXSystem`/`Pilot_CrdYSystem`
    /// (поле карты `x` `0..255`, `y_raw` `0..255`), — их и нужно записать для
    /// телепорта (см. `native::request_teleport`).
    pub fn selected_coords(&self) -> Option<(u8, u8)> {
        self.nearest_index()
            .map(|i| (self.planets[i].x as u8, self.planets[i].y_raw as u8))
    }

    /// Расстояние (св. годы) от текущей системы до курсора — ровно то число,
    /// которое игра показывает на карте галактики (`CalcFuelForJump`:
    /// `FuelToPlanet`, печатается как «6.4»).
    ///
    /// Расстояние считается от координат системы (`Pilot_CrdXSystem`/
    /// `Pilot_CrdYSystem`), а не от координат ближайшей звезды, и по
    /// модулю разности той версии игры, что запущена (см.
    /// [`galaxy::Metric`]). У Кладова модуль разности на единицу меньше,
    /// когда курсор левее системы, — из-за этого расстояние до звёзд
    /// слева от курсора на 0.4 св. года меньше, чем в оригинале.
    pub fn distance_to_cursor(&self) -> f32 {
        galaxy::distance_to_cursor_ly(
            self.layout.metric,
            self.sys_x,
            self.sys_y_raw,
            self.cursor_x,
            self.cursor_y_raw,
        )
    }

    /// Поставить курсор по координатам окна (наведение мышью).
    pub fn point_at(&mut self, win_x: f64, win_y: f64) {
        let l = self.layout(self.win_w.max(1), self.win_h.max(1));
        let (fx, frow) = l.field_at(win_x, win_y);
        self.cursor_x = fx.round().clamp(0.0, 255.0) as i32;
        // Поле вертикали: строка = (y_raw >> 1), значит y_raw = 2 * строка.
        self.cursor_y_raw = (frow * 2.0).round().clamp(0.0, 255.0) as i32;
        self.clamp_cursor_to_view();
    }

    /// Поставить курсор точно на ближайшую звезду (клик ЛКМ).
    pub fn snap_to_nearest(&mut self) {
        if let Some(p) =
            galaxy::nearest(&self.planets, self.layout.metric, self.cursor_x, self.cursor_y_raw)
        {
            self.cursor_x = p.x;
            self.cursor_y_raw = p.y_raw;
        }
        self.clamp_cursor_to_view();
    }

    /// Удержать курсор в видимой части поля.
    fn clamp_cursor_to_view(&mut self) {
        // Размер окна ещё неизвестен (например, в тестах без окна) — не трогаем.
        if self.win_w == 0 || self.win_h == 0 {
            return;
        }
        let l = self.layout(self.win_w, self.win_h);
        let (x0, r0, x1, r1) = l.visible_rect();
        let fx0 = ((x0 - l.fx0) as f32 / l.scale).floor() as i32;
        let fx1 = ((x1 - l.fx0) as f32 / l.scale).ceil() as i32;
        let fr0 = ((r0 - l.fy0) as f32 / l.scale).floor() as i32;
        let fr1 = ((r1 - l.fy0) as f32 / l.scale).ceil() as i32;
        // «Сырая» вертикаль вдвое больше строки.
        let y0 = 2 * fr0;
        let y1 = 2 * fr1;
        self.cursor_x = self.cursor_x.clamp(fx0.max(0), fx1.min(255));
        self.cursor_y_raw = self.cursor_y_raw.clamp(y0.max(0), y1.min(255));
    }

    /// Нарисовать карту во весь слой.
    pub fn draw(&mut self, f: &mut Frame, font: &[u8]) {
        self.win_w = f.width as u32;
        self.win_h = f.height as u32;
        let l = self.layout(self.win_w, self.win_h);
        f.clear(BG);

        let ts = l.text;
        let line = 8 * ts as i32;
        let near = self.nearest_index();
        let sys = self.system_index();

        // Видимая часть поля; сюда обрезаем всё рисование поля.
        let (vx0, vy0, vx1, vy1) = l.visible_rect();
        let vw = (vx1 - vx0).max(0) as usize;
        let vh = (vy1 - vy0).max(0) as usize;
        f.set_clip(vx0, vy0, vw, vh);
        // Планета видна, если её центр попадает в область показа. Позиция при
        // этом зажимается внутрь кадра: на минимальном зуме так остаются видны
        // все планеты, включая крайние (их кружок не налезает на рамку), а при
        // зуме за кадром оказываются только те, чей центр вне области показа.
        let margin = self.planet_margin(l.scale);
        let place = |x: i32, row: i32| -> Option<(i32, i32)> {
            self.place_planet(&l, x, row, margin)
        };

        // Поле галактики и его рамка.
        f.fill_rect(l.fx0, l.fy0, l.fw as usize, l.fh as usize, FIELD_BG);
        f.hline(l.fx0, l.fy0, l.fw as usize, BORDER);
        f.hline(l.fx0, l.fy0 + l.fh - 1, l.fw as usize, BORDER);
        f.vline(l.fx0, l.fy0, l.fh as usize, BORDER);
        f.vline(l.fx0 + l.fw - 1, l.fy0, l.fh as usize, BORDER);

        // Круг дальности прыжка вокруг текущей системы (как в карте I).
        let range = self.range_px(l.scale);
        if range > 0 {
            if let Some(i) = sys {
                let p = &self.planets[i];
                let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
                f.circle(sx, sy, range, RANGE);
            }
        }

        // Планеты: у каждой свой постоянный цвет; вне области показа — пропуск.
        for p in &self.planets {
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.disc(sx, sy, planet_px(p, l.scale), galaxy::color(p.state));
            }
        }
        // Кольцо ближайшей к курсору планеты.
        if let Some(i) = near {
            let p = &self.planets[i];
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.circle(sx, sy, planet_px(p, l.scale) + 3, NEAR_RING);
            }
        }
        // Кольцо текущей системы.
        if let Some(i) = sys {
            let p = &self.planets[i];
            if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                f.circle(sx, sy, planet_px(p, l.scale) + 1, SYS_RING);
            }
        }

        // Названия планет (как на детальной карте по O) — только при крупном зуме,
        // когда планет в кадре немного, иначе надписи сливаются.
        let mut visible_count: usize = 0;
        for p in &self.planets {
            if place(p.x, p.y() - FIELD_TOP).is_some() {
                visible_count += 1;
            }
        }
        if visible_count < 40 {
            for p in &self.planets {
                if let Some((sx, sy)) = place(p.x, p.y() - FIELD_TOP) {
                    let name = self.name_of(p.index);
                    let px = planet_px(p, l.scale);
                    let tx = sx + px + 3 * ts as i32;
                    let ty = sy - 3 * ts as i32;
                    f.draw_text(font, tx, ty, &name, TEXT, ts);
                }
            }
        }

        // Курсор — перекрестие.
        let (cur_x, cur_y_raw) = self.cursor();
        let (cx, cy) = l.star_px(cur_x, cur_y_raw >> 1);
        let arm = ((3.0 * l.scale).round() as i32).clamp(3, 14);
        f.hline(cx - arm, cy, (2 * arm + 1) as usize, CURSOR);
        f.vline(cx, cy - arm, (2 * arm + 1) as usize, CURSOR);

        f.clear_clip();

        // Заголовок: как в оригинале — «GALACTIC CHART <номер галактики>».
        let title = format!("GALACTIC CHART {}", self.galaxy_number());
        let tw = title.len() as i32 * 8 * ts as i32;
        let tx0 = ((f.width as i32 - tw) / 2).max(8 * ts as i32);
        f.draw_text(font, tx0, l.title_y, &title, TEXT, ts);
        // Индикатор зума справа.
        let zs = format!("x{:.2}", self.zoom());
        let zx = (f.width as i32 - 8 * ts as i32 - zs.len() as i32 * 8 * ts as i32).max(0);
        f.draw_text(font, zx, l.title_y, &zs, TEXT_DIM, ts);

        // Панель внизу: текущая система и выбранная планета с расстоянием.
        let panel_h = f.height as i32 - l.panel_y;
        let py = l.panel_y;
        f.fill_rect(0, py, f.width, panel_h as usize, 0xFF00_0000);
        f.hline(0, py, f.width, BORDER);
        let tx = 8 * ts as i32;
        let mut ty = py + 4 * ts as i32;
        if let Some(i) = sys {
            let s = format!("SYSTEM   {:<8}", self.name_of(i));
            f.draw_text(font, tx, ty, &s, SYS_RING, ts);
        }
        ty += line + 2;
        if let Some(i) = near {
            let p = &self.planets[i];
            // Цветной маркер планеты рядом с именем.
            f.fill_rect(
                tx,
                ty,
                (6 * ts) as usize,
                line as usize,
                galaxy::color(p.state),
            );
            let s = format!(
                "SELECTED {:<8} {:>4.1} LY",
                self.name_of(i),
                self.distance_to_cursor()
            );
            f.draw_text(font, tx + 8 * ts as i32, ty, &s, NEAR_RING, ts);
        }
        ty += line + 2;
        f.draw_text(
            font,
            tx,
            ty,
            "F10/Esc back   mouse: point/pick   wheel or +/-: zoom",
            TEXT_DIM,
            ts,
        );
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Семя и координаты из памяти запущенной версии Кладова (tmp/wf/mem.bin).
    fn fake_mem() -> Vec<u8> {
        let mut m = vec![0u8; 0x10000];
        m[0xD090..0xD096].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
        m[0xD097] = 173; // Y системы (LAVE)
        m[0xD099] = 20; // X системы (LAVE)
        m[0xD065] = 2; // номер галактики (PilotNumGalaxy, 0-based)
        m[0xD082] = 70; // топливо (Pilot_Fuel)
        m[0xFDB3..0xFDB3 + 64].copy_from_slice(
            b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION",
        );
        m
    }

    fn opened() -> Map {
        let mem = fake_mem();
        let mut map = Map::new();
        map.open(&mem, galaxy::KLADOV_GALAXY);
        map.win_w = 1024;
        map.win_h = 768;
        map
    }

    #[test]
    fn open_builds_galaxy_and_centers_cursor_on_system() {
        let map = opened();
        assert!(map.active);
        assert_eq!(map.planets.len(), galaxy::COUNT);
        assert_eq!((map.cursor_x, map.cursor_y_raw), (20, 173));
        assert_eq!(map.name_of(7), "LAVE");
        assert_eq!(map.name_of(204), "ORESQU");
        assert_eq!(map.nearest_index(), Some(7));
        // Номер галактики показывается как 1..8.
        assert_eq!(map.galaxy_number(), 3);
        // Зум сбрасывается на «вся галактика».
        assert_eq!(map.zoom(), 1.0);
        // Круг дальности прыжка: топливо 70 -> радиус 17 единиц поля.
        assert_eq!(galaxy::jump_radius(map.fuel), 17);
    }

    #[test]
    fn cursor_moves_within_view_and_snaps_to_planet() {
        let mut map = opened();
        assert_eq!(map.nearest_index(), Some(7));
        map.move_by(1000, 1000);
        assert!(map.cursor().0 <= 255 && map.cursor().1 <= 255);
        // Щелчок по ORESQU: курсор встаёт точно на звезду.
        map.cursor_x = 22;
        map.cursor_y_raw = 217;
        assert_eq!(map.nearest_index(), Some(204));
        map.cursor_x = 21;
        map.cursor_y_raw = 216;
        map.snap_to_nearest();
        assert_eq!((map.cursor_x, map.cursor_y_raw), (22, 217));
    }

    /// Координаты выбранной планеты для чита телепортации (F1): те же числа,
    /// что игра хранит в `Pilot_CrdXSystem`/`Pilot_CrdYSystem`.
    #[test]
    fn selected_coords_follow_cursor() {
        let mut map = opened();
        // Сразу после открытия курсор стоит на текущей системе — LAVE.
        assert_eq!(map.selected_coords(), Some((20, 173)));
        // Наводимся на ORESQU (№204) — координаты меняются вместе с курсором.
        map.cursor_x = 22;
        map.cursor_y_raw = 217;
        map.snap_to_nearest();
        assert_eq!(map.selected_coords(), Some((22, 217)));
    }

    /// Расстояние до курсора считается так же, как в игре: LAVE -> DISO
    /// = 3.2 св. года в версии Кладова (в оригинале Elite — 3.6), а
    /// LAVE -> RIEDQUAD = 6.4 (в оригинале — 6.8). Разница — особенность
    /// модуля разности у Кладова.
    #[test]
    fn distance_to_cursor_matches_game() {
        let mut map = opened();
        // Курсор на текущей системе — расстояние ноль.
        assert_eq!(map.distance_to_cursor(), 0.0);
        // Курсор на DISO (№147).
        map.cursor_x = 11;
        map.cursor_y_raw = 174;
        assert_eq!(map.distance_to_cursor(), 3.2);
        // Курсор на RIEDQUAD (№46).
        map.cursor_x = 3;
        map.cursor_y_raw = 181;
        assert_eq!(map.distance_to_cursor(), 6.4);
        assert_eq!(map.nearest_index(), Some(46));
    }

    /// Поле 256x128 вписывается в окно с сохранением пропорций. Сверху и
    /// снизу зарезервированы полосы под заголовок и панель.
    #[test]
    fn layout_keeps_field_proportions() {
        let map = Map::new();
        let l = map.layout(1024, 768);
        // 1024/256 = 4, доступная высота больше -> узкое место — горизонталь.
        assert_eq!(l.scale, 4.0);
        assert_eq!((l.fx0, l.fw), (0, 1024));
        assert_eq!(l.fh, 512);
        assert_eq!(l.text, 2);
        assert!(l.fy0 >= l.top, "поле заходит под заголовок");
        assert!(l.fy0 + l.fh <= l.panel_y, "поле заходит под панель");
        assert!(l.panel_y < 768);
    }

    /// На разных соотношениях сторон (16:9, 16:10, 4:3) поле не налезает
    /// на заголовок: его верх всегда ниже конца строки заголовка.
    #[test]
    fn field_never_overlaps_title() {
        let map = Map::new();
        for &(w, h) in &[
            (1280u32, 720u32), // 16:9
            (1920, 1080),      // 16:9 FullHD
            (1440, 900),       // 16:10
            (1680, 1050),      // 16:10
            (1024, 768),       // 4:3
            (3840, 2160),      // 16:9 4K
        ] {
            let l = map.layout(w, h);
            let title_bottom = l.title_y + 8 * l.text as i32;
            assert!(
                l.fy0 >= title_bottom,
                "{w}x{h}: поле ({}) налезает на заголовок (низ {})",
                l.fy0,
                title_bottom
            );
            assert!(l.fy0 >= l.top);
            assert!(l.fy0 + l.fh <= l.panel_y, "{w}x{h}: поле заходит под панель");
        }
    }

    /// Наведение мышью: центр поля -> центр карты (курсор не двигает карту).
    #[test]
    fn map_is_fixed_and_mouse_points_at_field() {
        let mut map = opened();
        let l = map.layout(1024, 768);
        map.point_at(
            l.fx0 as f64 + 128.0 * l.scale as f64,
            l.fy0 as f64 + 64.0 * l.scale as f64,
        );
        assert_eq!((map.cursor_x, map.cursor_y_raw), (128, 128));
        // Карта не сдвинулась от движения курсора.
        let l2 = map.layout(1024, 768);
        assert_eq!((l.fx0, l.fy0), (l2.fx0, l2.fy0));
    }

    /// Зум всегда идёт вокруг текущей системы, а на максимуме в кадр
    /// помещается полный радиус прыжка (7 св. лет) — детализация карты `O`.
    #[test]
    fn zoom_centers_on_system_and_shows_jump_range() {
        let mut map = opened();
        // Приближаем до предела.
        for _ in 0..80 {
            map.zoom_by(1);
        }
        assert!(map.zoom() > 1.0);
        assert!((map.zoom() - map.max_zoom()).abs() < 1e-3);
        let l = map.layout(1024, 768);
        let (x0, y0, x1, y1) = l.visible_rect();
        // Текущая система видна в кадре.
        let sys = map.system_index().unwrap();
        let p = &map.planets[sys];
        let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
        assert!(sx >= x0 && sx < x1 && sy >= y0 && sy < y1, "система вне кадра");
        // Полный радиус прыжка помещается в меньшую сторону кадра.
        let range = map.range_px(l.scale);
        let half = l.vw.min(l.vh) / 2;
        assert!(range <= half, "круг дальности {range}px > {half}px");
        // Отдаляем до минимума — вся галактика.
        for _ in 0..200 {
            map.zoom_by(-1);
        }
        assert_eq!(map.zoom(), ZOOM_MIN);
    }

    /// Планеты, чей центр вне области показа, не рисуются; на зуме 1 видна вся
    /// галактика (крайние планеты прижаты внутрь кадра и не пропадают). При
    /// сильном зуме в кадре остаются далеко не все 256 звёзд.
    #[test]
    fn planets_outside_view_are_culled() {
        let mem = fake_mem();
        let mut map = Map::new();
        map.open(&mem, galaxy::KLADOV_GALAXY);
        let font = vec![0xFFu8; 0x300];
        let mut f = Frame::new(1280, 720);
        map.draw(&mut f, &font);
        let l = map.layout(1280, 720);
        // При зуме 1 все звёзды центром внутри поля — видны все 256.
        assert_eq!(visible_count(&map, &l), galaxy::COUNT);

        // Приближаем к окрестности LAVE — часть звёзд уходит за кадр.
        map.win_w = 1280;
        map.win_h = 720;
        for _ in 0..8 {
            map.zoom_by(1);
        }
        let l = map.layout(1280, 720);
        assert!(l.scale > 4.0);
        let visible = visible_count(&map, &l);
        assert!(
            0 < visible && visible < galaxy::COUNT / 4,
            "видно слишком много: {visible}"
        );
    }

    /// Число планет, чей центр попадает в видимую область карты.
    fn visible_count(map: &Map, l: &MapLayout) -> usize {
        let (vx0, vy0, vx1, vy1) = l.visible_rect();
        map.planets
            .iter()
            .filter(|p| {
                let (sx, sy) = l.star_px(p.x, p.y() - FIELD_TOP);
                sx >= vx0 && sx < vx1 && sy >= vy0 && sy < vy1
            })
            .count()
    }

    /// Крайние планеты прижимаются внутрь кадра (их кружок не выходит за
    /// рамку), а планеты с центром вне области показа не рисуются.
    #[test]
    fn edge_planets_are_clamped_inside_the_field() {
        let map = opened();
        let l = map.layout(1024, 768);
        let (x0, y0, x1, y1) = l.visible_rect();
        let margin = map.planet_margin(l.scale);
        // Угол поля: центр звезды на самой кромке — кружок сдвинут внутрь.
        let (sx, sy) = map
            .place_planet(&l, 0, 0, margin)
            .expect("звезда на кромке видна");
        assert_eq!((sx, sy), (x0 + margin, y0 + margin));
        let r = planet_px(&map.planets[0], l.scale);
        assert!(
            sx - r >= x0 && sx + r < x1 && sy - r >= y0 && sy + r < y1,
            "кружок вышел за рамку"
        );
        // Центр далеко вне области показа — планета не рисуется.
        assert!(map.place_planet(&l, 400, 400, margin).is_none());
    }

    /// Карта рисуется во весь слой, планеты раскрашены, есть кольца и курсор.
    #[test]
    fn draw_fills_layer_and_marks_planets() {
        let mut map = opened();
        let font = vec![0xFFu8; 0x300];
        let mut f = Frame::new(1024, 768);
        map.draw(&mut f, &font);
        assert!(f.pixels().iter().all(|&p| p != 0), "layer not opaque");
        let distinct: std::collections::HashSet<u32> = f.pixels().iter().copied().collect();
        assert!(distinct.len() > 50, "слишком мало цветов: {}", distinct.len());
        assert!(f.pixels().iter().any(|&p| p == SYS_RING));
        assert!(f.pixels().iter().any(|&p| p == NEAR_RING));
        assert!(f.pixels().iter().any(|&p| p == CURSOR));
        assert!(f.pixels().iter().any(|&p| p == RANGE), "нет круга дальности");
    }

    /// Точные позиции звёзд в HD-слое: звезда попадает в пиксель `star_px`,
    /// окрашенный в свой постоянный цвет.
    #[test]
    fn star_positions_are_scaled_field_coordinates() {
        let mut map = opened();
        // Уводим курсор из системы, чтобы перекрестие не перекрывало LAVE.
        map.move_by(-255, -255);
        let font = vec![0u8; 0x300];
        let mut f = Frame::new(1024, 768);
        map.draw(&mut f, &font);
        let l = map.layout(1024, 768);
        let at = |f: &Frame, x: i32, y: i32| f.pixels()[y as usize * f.width + x as usize];
        let p7 = &map.planets[7];
        let (lx, ly) = l.star_px(p7.x, p7.y() - FIELD_TOP);
        assert_eq!(at(&f, lx, ly), galaxy::color(p7.state));
        let p204 = &map.planets[204];
        let (ox, oy) = l.star_px(p204.x, p204.y() - FIELD_TOP);
        let c = at(&f, ox, oy);
        assert!(
            c == galaxy::color(p204.state)
                || c == NEAR_RING
                || c == SYS_RING
                || c == RANGE
                || c == CURSOR,
            "ORESQU pixel = {c:#010X}"
        );
    }

    /// Отладочный дамп карты в `tmp/hdmap.ppm` по слепку памяти реальной игры:
    /// `cargo test --release dump_map -- --ignored`. Затем, например,
    /// `convert tmp/hdmap.ppm tmp/hdmap.png`. В `tmp/wf/mem.bin` уже есть
    /// ПЗУ (0x0..0x4000), откуда берём шрифт. Переменная `ELITE_MAP_ZOOM`
    /// задаёт зум для дампа.
    #[test]
    #[ignore]
    fn dump_map_to_tmp() {
        let mem = std::fs::read("tmp/wf/mem.bin").expect("tmp/wf/mem.bin");
        let mut map = Map::new();
        map.open(&mem, galaxy::KLADOV_GALAXY);
        map.win_w = 1280;
        map.win_h = 720;
        if let Ok(z) = std::env::var("ELITE_MAP_ZOOM") {
            if let Ok(z) = z.parse::<f32>() {
                let zmax = map.max_zoom();
                while map.zoom() < z && map.zoom() < zmax {
                    map.zoom_by(1);
                }
            }
        }
        let font = &mem[0x3D00..0x3D00 + 0x300];
        let mut f = Frame::new(1280, 720);
        map.draw(&mut f, font);
        let mut out = Vec::with_capacity(f.width * f.height * 3 + 32);
        out.extend_from_slice(format!("P6\n{} {}\n255\n", f.width, f.height).as_bytes());
        for &p in f.pixels() {
            out.push((p >> 16) as u8);
            out.push((p >> 8) as u8);
            out.push(p as u8);
        }
        std::fs::write("tmp/hdmap.ppm", out).expect("write tmp/hdmap.ppm");
    }
}
