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
use crate::galaxy::{self, FIELD_HEIGHT, FIELD_TOP, GalaxyLayout, Planet};

mod draw;
mod layout;

pub use layout::MapLayout;

use layout::*;

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
        let (cam_x, cam_row) = clamp_center(&v, scale, self.sys_x as f32, sys_row);
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
    fn place_planet(&self, l: &MapLayout, x: i32, row: i32, margin: i32) -> Option<(i32, i32)> {
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
        galaxy::nearest(
            &self.planets,
            self.layout.metric,
            self.cursor_x,
            self.cursor_y_raw,
        )
        .map(|p| p.index)
    }

    /// Текущая звезда (система, в которой находится корабль).
    pub fn system_index(&self) -> Option<usize> {
        galaxy::nearest(
            &self.planets,
            self.layout.metric,
            self.sys_x,
            self.sys_y_raw,
        )
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
        if let Some(p) = galaxy::nearest(
            &self.planets,
            self.layout.metric,
            self.cursor_x,
            self.cursor_y_raw,
        ) {
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
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

#[cfg(test)]
mod tests;
