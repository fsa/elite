//! 3D-модели объектов игры из таблицы `AdrDataShips` версии Кладова
//! (`zx_bin/kladov.bin`). Данные других версий игры сюда не попадают:
//! у общих моделей геометрия та же, а характеристики и порядок данных
//! берутся из версии Кладова.
//!
//! Каждая модель лежит в отдельном модуле (`src/shipdata/<имя>.rs`),
//! список [`SHIPS`] сохраняет порядок таблицы `AdrDataShips`.
//!
//! Координаты — знаковые байты локальной СКО корабля; `lod` — байт
//! детализации (младшие 5 бит = максимальная дальность отрисовки в
//! единицах 1/256, `0x1F` = всегда); `faces` — два сдвоенных индекса
//! граней (полубайт), видимость которых означает видимость элемента.
//!
//! В формате Кладова вершины и нормали сжаты: общая координата хранится
//! один раз, а признак начала новой группы — младший бит первого байта
//! (у вершины это байт lod, у грани — flags). Знаки компонентов — в
//! старших трёх битах того же байта. Смещения блоков могут указывать
//! на общую геометрию (`d_Minerals`, `dTargletH`) — она читается из
//! того же образа Кладова.
//!
//! У граней (`Face`) 4-й байт — младшие 5 бит: порог дальности,
//! старшие три: знаки компонентов нормали (бит 7 — X, бит 6 — Y,
//! бит 5 — Z); сами знаковые компоненты уже в `nx`/`ny`/`nz`.
//! Признака «перевёрнутой» грани в формате нет — видимость считается
//! по знаку Z повёрнутой нормали (см. `src/splash.rs`).

mod coriolis;
mod missile;
mod escape_pod;
mod alloys;
mod cargo;
mod minerals;
mod asteroid2;
mod asteroid1;
mod viper;
mod cobra1;
mod python1;
mod python2;
mod cobra2;
mod asp;
mod fer_de_lance;
mod targon;
mod sidewinder;
mod adder;
mod targlet;
mod stru_6381;
mod stru_6398;
mod stru_63af;
mod krait;

/// Вершина модели (6 байт в ROM).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vertex {
    pub x: i16, pub y: i16, pub z: i16,
    pub lod: u8,
    pub faces: [u8; 2],
}

/// Ребро модели (4 байта в ROM): две вершины и сдвоенные индексы граней.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub lod: u8,
    pub v: [u8; 2],
    pub faces: u8,
}

/// Грань модели: нормаль и байт детализации/знаков (4 байта в ROM).
///
/// В 4-м байте младшие 5 бит — порог дальности отрисовки, старшие три —
/// знаки компонентов нормали (бит 7 — X, бит 6 — Y, бит 5 — Z). Знаковые
/// компоненты уже разобраны в `nx`/`ny`/`nz`, а `flags` хранит их
/// вместе с порогом дальности.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Face {
    pub nx: i16, pub ny: i16, pub nz: i16,
    pub flags: u8,
}

/// Лётно-тактические характеристики из шапки `ShipData`.
///
/// Поля повторяют формат таблицы; пока нативная логика читает только
/// `collision` (радиус столкновения), остальные нужны для будущих механик
/// и сверки с образом.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Stats {
    pub cargo: u8, pub collision: u8, pub fire_chance: u8,
    pub max_speed: u8, pub energy: u8, pub cash: u16,
    pub launch: u8, pub size: u8, pub flags: u8,
    pub factor: u8, pub laser: u8, pub rating: u16,
}

/// Один объект игры: геометрия плюс характеристики.
#[derive(Clone, Copy, Debug)]
pub struct Ship {
    pub name: &'static str,
    /// Адрес блока в ROM версии Кладова.
    pub addr: u16,
    /// 4-й байт записи таблицы (категория).
    #[allow(dead_code)]
    pub category: u8,
    pub stats: Stats,
    pub vertices: &'static [Vertex],
    pub edges: &'static [Edge],
    pub faces: &'static [Face],
}

/// Модели версии Кладова (`zx_bin/kladov.bin`, 23 шт.).
pub static SHIPS: &[Ship] = &[
    coriolis::SHIP_D_CORIOLIS,
    missile::SHIP_D_MISSILE,
    escape_pod::SHIP_D_ESCAPEPOD,
    alloys::SHIP_D_ALLOYS,
    cargo::SHIP_D_CARGO,
    minerals::SHIP_D_MINERALS,
    asteroid2::SHIP_D_ASTEROID2,
    asteroid1::SHIP_D_ASTEROID1,
    viper::SHIP_D_VIPER,
    cobra1::SHIP_D_COBRA1,
    python1::SHIP_D_PYTHON1,
    python2::SHIP_D_PYTHON2,
    cobra2::SHIP_D_COBRA2,
    asp::SHIP_D_ASP,
    fer_de_lance::SHIP_D_FERDELANCE,
    targon::SHIP_D_TARGON,
    sidewinder::SHIP_DSIDEWINDERH,
    adder::SHIP_D_ADDER,
    targlet::SHIP_DTARGLETH,
    stru_6381::SHIP_STRU_6381,
    stru_6398::SHIP_STRU_6398,
    stru_63af::SHIP_STRU_63AF,
    krait::SHIP_D_KRAIT,
];

/// Найти модель по внутреннему имени.
///
/// Используется тестами заставки.
#[allow(dead_code)]
pub fn find(name: &str) -> Option<&'static Ship> {
    SHIPS.iter().find(|s| s.name == name)
}
