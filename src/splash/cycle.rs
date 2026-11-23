//! Цикл кораблей на заставке.
//!
//! Обычный режим повторяет оригинал (`loc_71A2`): тип следующего корабля
//! считается как `((n & 7) + 1)`, для типов < 6 прибавляется 10. Счётчик
//! стартует с значения, дающего Cobra; порядок — Cobra → Python → ASP →
//! Fer-de-Lance → Targon → Sidewinder → Krait → Viper.
//!
//! В режиме отладки (`--debug`) стрелки перебирают все модели из
//! [`shipdata::SHIPS`]. Несколько записей таблицы имеют одну и ту же
//! геометрию (обычный и пиратский варианты Cobra/Python, два астероида):
//! на витрине такой корпус показывается один раз.

use std::sync::OnceLock;

use crate::render3d::Rotation;
use crate::shipdata::{self, Ship};

use super::{Splash, next_tint};

/// Цикл кораблей на заставке (имена моделей в [`shipdata::SHIPS`]).
pub(super) const INTRO_CYCLE: [&str; 8] = [
    "d_Cobra1",
    "d_Python1",
    "d_ASP",
    "d_FerDeLance",
    "d_Targon",
    "dSidewinderH",
    "d_Krait",
    "d_Viper",
];

/// Совпадает ли геометрия двух объектов.
///
/// В таблице Кладова обычный и пиратский варианты Cobra/Python (а также два
/// астероида) хранятся как отдельные записи с одинаковыми вершинами, рёбрами
/// и гранями. Для витрины достаточно показать такой корпус один раз.
fn same_hull(a: &Ship, b: &Ship) -> bool {
    a.vertices == b.vertices && a.edges == b.edges && a.faces == b.faces
}

/// Все уникальные корпуса таблицы моделей, в порядке [`shipdata::SHIPS`].
///
/// Список вычисляется один раз: геометрия моделей статична, а стрелки на
/// заставке могут нажиматься часто.
pub(super) fn debug_cycle() -> &'static [&'static str] {
    static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let ships = shipdata::SHIPS;
        ships
            .iter()
            .enumerate()
            .filter(|(i, ship)| !ships[..*i].iter().any(|prev| same_hull(prev, ship)))
            .map(|(_, ship)| ship.name)
            .collect()
    })
}

impl Splash {
    /// Переключить корабль влево/вправо (стрелки на заставке).
    ///
    /// Обычный цикл повторяет оригинальный `SetShip4Intro`/`loc_71A2`: восемь
    /// боевых кораблей в том же порядке (Cobra → Python → ASP →
    /// Fer-de-Lance → Targon → Sidewinder → Krait → Viper), старт — Cobra.
    /// В режиме отладки добавляются остальные уникальные модели из
    /// [`shipdata::SHIPS`]. Как и в оригинале, смена корабля заново выставляет
    /// СКО из `SKO4Intro`, поэтому поворот начинается с исходного ракурса.
    /// Модель с признаком `tintable` получает следующий псевдослучайный тон;
    /// у остальных моделей цвет остаётся фиксированным.
    pub fn cycle_ship(&mut self, dir: i32) {
        let step = if dir < 0 { -1 } else { 1 };
        if self.cycle.is_empty() {
            return;
        }
        // Текущая позиция в цикле (по имени модели).
        let cur = self
            .ship
            .and_then(|s| self.cycle.iter().position(|&n| n == s.name))
            .unwrap_or(0) as i32;
        let next = (cur + step).rem_euclid(self.cycle.len() as i32) as usize;
        let name = self.cycle[next];
        let idx = shipdata::SHIPS
            .iter()
            .position(|s| s.name == name)
            .unwrap_or(0);
        let model = &shipdata::SHIPS[idx];
        self.index = idx;
        self.ship = Some(model);
        self.tint = model.tintable.then(|| next_tint(&mut self.tint_seed));
        self.rot = Rotation::intro();
    }
}
