//! Отдельная (нативная) карта галактики — клавиша F6.
//!
//! Оформление — как у карты оригинала: белая рамка вокруг экрана и красная
//! ячейка с белым заголовком сверху.
//!
//! Это независимый от эмуляции просмотрщик: при включении окно целиком
//! занимает нативный рендер (ZX-кадр не показывается), при выключении
//! возвращается обычный экран игры. Отдельных карт эмулятора (клавиши I/O)
//! больше не требуется: та же нативная карта открывается в двух масштабах —
//! `I` показывает всю галактику, `O` — детальный фрагмент вокруг системы.
//!
//! Модель галактики берётся из [`crate::galaxy`] — те же координаты и имена,
//! что строит Z80-код игры. Важная особенность: поле карты по горизонтали
//! `0..255`, а по вертикали — вдвое меньше (`(y_raw >> 1)`, 128 строк),
//! поэтому галактика вытянута по горизонтали. Поле `256x128` вписывается в
//! окно с сохранением пропорций (круги планет остаются круглыми), то есть по
//! осям получаются разные масштабы.
//!
//! Карта «фиксированная»: положение поля не зависит от курсора, а зум всегда
//! идёт вокруг текущей системы (планеты, где стоит корабль). Есть два пресета:
//! `I` — вся галактика (зум `1.0`) и `O` — максимум, при котором в кадр
//! помещается полный радиус прыжка на бак топлива (7.0 св. года). Колесо мыши
//! и `+`/`-` меняют масштаб внутри выбранного режима. Планеты, чей центр
//! выходит за видимую область, не рисуются; оставшиеся вписаны внутрь кадра,
//! чтобы не налезать на рамку. Вокруг текущей системы рисуется круг дальности
//! прыжка (радиус `fuel >> 2` единиц поля).

use crate::display::Frame;
use crate::galaxy::{self, FIELD_HEIGHT, FIELD_TOP, GalaxyLayout, Planet};

mod draw;
mod layout;

pub use layout::MapLayout;

use layout::*;

/// Масштаб, с которым открыта нативная карта.
///
/// Соответствует двум родным картам Кладова: `I` показывает всю галактику,
/// `O` — детальный фрагмент вокруг текущей системы.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapMode {
    /// Вся галактика: зум `1.0`, как по клавише `I`.
    Galaxy,
    /// Фрагмент вокруг текущей системы: предельный зум, как по `O`.
    Local,
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
    /// Режим, в котором карта была открыта (`I` или `O`).
    mode: MapMode,
    /// Размер области рисования прошлого кадра (для попадания мышью).
    win_w: u32,
    win_h: u32,
    /// Последняя позиция указателя мыши в координатах окна. Обновляется и
    /// когда карта закрыта, чтобы первый же клик после открытия попал в цель.
    /// Само движение мыши курсор карты не двигает — позиция нужна только ЛКМ.
    pointer: Option<(f64, f64)>,
    /// Активен ли режим поиска планеты по имени (клавиша `R`).
    searching: bool,
    /// Текст запроса поиска планеты.
    search_query: String,
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
            mode: MapMode::Galaxy,
            win_w: 0,
            win_h: 0,
            pointer: None,
            searching: false,
            search_query: String::new(),
        }
    }

    /// Открыть карту: собрать галактику из памяти запущенной игры.
    ///
    /// Курсор берётся из `Pilot_CrdXTarget`/`Pilot_CrdYTarget` — это актуальная
    /// цель гиперперехода, которую игра обновляет при движении по своей карте.
    /// Ставить курсор заново на текущую систему нельзя: при повторном входе
    /// карта забывала бы, куда игрок нацелился в прошлый раз.
    #[allow(dead_code)] // используется тестами
    pub fn open(&mut self, mem: &[u8], layout: GalaxyLayout) {
        self.open_mode(mem, layout, MapMode::Galaxy);
    }

    /// Открыть карту в заданном масштабе: `I` — вся галактика, `O` — фрагмент
    /// вокруг текущей системы.
    ///
    /// Размер окна должен быть известен заранее (см. [`Self::resize`]): пресет
    /// `Local` вычисляется из текущей геометрии, чтобы в кадр гарантированно
    /// помещался круг дальности прыжка.
    pub fn open_mode(&mut self, mem: &[u8], layout: GalaxyLayout, mode: MapMode) {
        self.layout = layout;
        let seed: [u8; 6] = mem[layout.seed..layout.seed + 6].try_into().unwrap();
        self.digraphs = mem[layout.digraphs..layout.digraphs + 64].to_vec();
        self.planets = galaxy::planets(seed);
        self.sys_x = mem[layout.sys_x] as i32;
        self.sys_y_raw = mem[layout.sys_y] as i32;
        self.galaxy_num = mem[layout.galaxy] & 7;
        self.fuel = mem[layout.fuel];
        self.cursor_x = mem[layout.target_x] as i32;
        self.cursor_y_raw = mem[layout.target_y] as i32;
        self.searching = false;
        self.search_query.clear();
        self.active = true;
        self.set_mode(mode);
    }

    /// Переключить масштаб карты, не перечитывая галактику и не сбрасывая
    /// курсор. Курсор лишь зажимается в новую видимую область, как и при
    /// обычном зуме колесом.
    pub fn set_mode(&mut self, mode: MapMode) {
        self.mode = mode;
        self.zoom = match mode {
            MapMode::Galaxy => ZOOM_MIN,
            MapMode::Local => self.max_zoom(),
        };
        self.clamp_cursor_to_view();
    }

    /// Текущий масштаб карты (`I` или `O`).
    #[allow(dead_code)] // используется тестами и внешними наблюдателями
    pub fn mode(&self) -> MapMode {
        self.mode
    }

    /// Заголовок ячейки: родные названия двух карт Кладова.
    pub fn title(&self) -> String {
        let name = match self.mode {
            MapMode::Galaxy => "GALACTIC CHART",
            MapMode::Local => "SHORT RANGE CHART",
        };
        format!("{} {}", name, self.galaxy_number())
    }

    /// Закрыть карту (возврат к экрану игры).
    pub fn close(&mut self) {
        self.active = false;
        self.searching = false;
        self.search_query.clear();
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
            frame: v.frame,
            margin: v.margin,
            header_y: v.header_y,
            header_h: v.header_h,
            top: v.top,
            title_y: v.header_y + (v.header_h - v.line) / 2,
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

    /// Запомнить позицию указателя мыши (в координатах окна). Сама по себе
    /// мышь курсор карты не двигает — это делает только ЛКМ (см. [`Self::click`]).
    pub fn pointer_moved(&mut self, win_x: f64, win_y: f64) {
        self.pointer = Some((win_x, win_y));
    }

    /// ЛКМ: перепрыгнуть курсором на звезду под указателем мыши. Клик вне
    /// видимой области поля (заголовок, панель, рамка) игнорируется.
    pub fn click(&mut self) {
        let Some((win_x, win_y)) = self.pointer else {
            return;
        };
        let l = self.layout(self.win_w.max(1), self.win_h.max(1));
        let (x0, y0, x1, y1) = l.visible_rect();
        let (px, py) = (win_x.round() as i32, win_y.round() as i32);
        if px < x0 || px >= x1 || py < y0 || py >= y1 {
            return;
        }
        let (fx, frow) = l.field_at(win_x, win_y);
        self.cursor_x = fx.round().clamp(0.0, 255.0) as i32;
        // Поле вертикали: строка = (y_raw >> 1), значит y_raw = 2 * строка.
        self.cursor_y_raw = (frow * 2.0).round().clamp(0.0, 255.0) as i32;
        self.clamp_cursor_to_view();
        self.snap_to_nearest();
    }

    /// Поставить курсор точно на ближайшую звезду (ЛКМ, `Enter`/`Space`, `D`).
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

    /// Вернуть курсор на текущую планету (клавиша `B`).
    pub fn cursor_to_current_system(&mut self) {
        self.cursor_x = self.sys_x;
        self.cursor_y_raw = self.sys_y_raw;
        self.snap_to_nearest();
    }

    /// Активен ли режим поиска планеты.
    pub fn is_searching(&self) -> bool {
        self.searching
    }

    /// Текущий текст запроса поиска.
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Включить поиск планеты (клавиша `R`).
    pub fn start_search(&mut self) {
        self.searching = true;
        self.search_query.clear();
    }

    /// Отменить поиск без изменения курсора.
    pub fn cancel_search(&mut self) {
        self.searching = false;
        self.search_query.clear();
    }

    /// Подтвердить поиск: выйти из режима, оставив курсор на найденной планете.
    pub fn confirm_search(&mut self) {
        self.searching = false;
        self.search_query.clear();
    }

    /// Добавить символ в запрос поиска и перейти к совпадению.
    pub fn search_push(&mut self, ch: char) {
        self.search_query.push(ch);
        self.apply_search();
    }

    /// Удалить последний символ запроса поиска.
    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        if !self.search_query.is_empty() {
            self.apply_search();
        }
    }

    /// Найти планету по запросу: точное имя, затем префикс, затем вхождение.
    fn find_planet(&self, query: &str) -> Option<usize> {
        if query.is_empty() {
            return None;
        }
        let q = query.to_ascii_uppercase();
        let mut prefix = None;
        let mut contains = None;
        for p in &self.planets {
            let name = galaxy::name(p.state, &self.digraphs);
            if name == q {
                return Some(p.index);
            }
            if prefix.is_none() && name.starts_with(&q) {
                prefix = Some(p.index);
            }
            if contains.is_none() && name.contains(&q) {
                contains = Some(p.index);
            }
        }
        prefix.or(contains)
    }

    /// Переместить курсор на планету, совпадающую с текущим запросом.
    fn apply_search(&mut self) {
        if let Some(i) = self.find_planet(&self.search_query) {
            let p = &self.planets[i];
            self.cursor_x = p.x;
            self.cursor_y_raw = p.y_raw;
            self.clamp_cursor_to_view();
        }
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
