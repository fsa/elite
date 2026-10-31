//! Цикл кораблей на заставке — как в оригинале (`loc_71A2`).
//!
//! Тип следующего корабля считается как `((n & 7) + 1)`, для типов < 6
//! прибавляется 10. Счётчик стартует с значения, дающего Cobra; порядок —
//! Cobra → Python → ASP → Fer-de-Lance → Targon → Sidewinder → Krait →
//! Viper.

use crate::render3d::Rotation;
use crate::shipdata;

use super::Splash;

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

impl Splash {
    /// Переключить корабль влево/вправо (стрелки на заставке).
    ///
    /// Цикл повторяет оригинальный `SetShip4Intro`/`loc_71A2`: восемь
    /// боевых кораблей в том же порядке (Cobra → Python → ASP →
    /// Fer-de-Lance → Targon → Sidewinder → Krait → Viper), старт — Cobra.
    /// Как и в оригинале, смена корабля заново выставляет СКО из
    /// `SKO4Intro`, поэтому поворот начинается с исходного ракурса.
    pub fn cycle_ship(&mut self, dir: i32) {
        let step = if dir < 0 { -1 } else { 1 };
        let len = INTRO_CYCLE.len() as i32;
        // Текущая позиция в цикле (по имени модели).
        let cur = self
            .ship
            .and_then(|s| INTRO_CYCLE.iter().position(|&n| n == s.name))
            .unwrap_or(0) as i32;
        let next = (cur + step).rem_euclid(len) as usize;
        let name = INTRO_CYCLE[next];
        let idx = shipdata::SHIPS
            .iter()
            .position(|s| s.name == name)
            .unwrap_or(0);
        self.index = idx;
        self.ship = Some(&shipdata::SHIPS[idx]);
        self.rot = Rotation::intro();
    }
}
