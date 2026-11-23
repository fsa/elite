//! Экранная проекция корабля: калибровка масштаба и постоянный центр.

use crate::render3d::Rotation;
use crate::shipdata::Ship;

#[cfg(test)]
use crate::render3d::{INTRO_DIST, Screen as Proj};

use super::Splash;
use super::motion::STEP_AXIS;

impl Splash {
    /// Множитель подбирается один раз по **самому дальнему выносу вершин за
    /// оборот**: любой ракурс остаётся в квадрате ±`r` вокруг центра, то есть
    /// в отведённой полосе между надписями. При этом масштаб не пересчитывается
    /// по кадру (прежняя ошибка — «дышащий» корабль, который раздувался, когда
    /// ракурс сокращался).
    ///
    /// В отладочном режиме стрелки ▲/▼ добавляют поворот вокруг оси X, поэтому
    /// калибровка перебирает и его. Без этого наклоненная модель вылезала за
    /// полосу и перекрывала подпись. Обычная заставка по-прежнему калибруется
    /// только по витринному обороту и остаётся такой же крупной.
    pub(super) fn ship_scale(&self, ship: &Ship, cen: [f32; 3], dist: f32, r: i32) -> f32 {
        /// Сколько ракурсов за оборот перебирается при калибровке.
        const SPIN_SAMPLES: usize = 72;
        /// Сколько наклонов ▲/▼ перебирается в отладочном режиме.
        const TILT_SAMPLES: usize = 24;
        let intro = Rotation::intro();
        // Калибровка идёт относительно центра стартового ракурса: к нему же
        // привязан постоянный сдвиг композиции ([`Splash::intro_screen_center`]).
        let (bx, by) = self.intro_screen_center(ship, cen, dist);
        // Максимальное отклонение вершины от этого центра по X или Y (при
        // k = 1), взятое по всем ракурсам оборота: любой ракурс должен
        // оставаться в квадрате ±r, то есть в отведённой полосе.
        let mut reach = 0.0f32;
        for i in 0..SPIN_SAMPLES {
            let t = i as f32 * std::f32::consts::TAU / SPIN_SAMPLES as f32;
            let spin = Rotation::axis_angle(STEP_AXIS, t).mul(&intro);
            for j in 0..if self.debug { TILT_SAMPLES } else { 1 } {
                let tilt = Rotation::axis_angle(
                    [1.0, 0.0, 0.0],
                    j as f32 * std::f32::consts::TAU / TILT_SAMPLES as f32,
                );
                let rot = tilt.mul(&spin);
                for v in ship.vertices {
                    let w = rot.apply([
                        v.x as f32 - cen[0],
                        v.y as f32 - cen[1],
                        v.z as f32 - cen[2],
                    ]);
                    let z = dist + w[2];
                    reach = reach.max((w[0] / z - bx).abs()).max((w[1] / z - by).abs());
                }
            }
        }
        r as f32 / reach.max(1e-3)
    }

    /// Центр габарита проекции на стартовом ракурсе (при масштабе 1).
    ///
    /// Из-за перспективы видимый центр корабля не совпадает с началом
    /// координат модели (у асимметричных корпусов — на десятки пикселей).
    /// Оригинал печатает объект вокруг его позиции, поэтому корабль там
    /// тоже чуть «в стороне»; чтобы композиция заставки была ровной, этот
    /// сдвиг фиксируется один раз по стартовому ракурсу (при вращении он
    /// остаётся постоянным — как фиксированная позиция объекта в оригинале,
    /// а не покадровая подгонка под габарит).
    pub(super) fn intro_screen_center(&self, ship: &Ship, cen: [f32; 3], dist: f32) -> (f32, f32) {
        let rot = Rotation::intro();
        let (mut mn, mut mx) = ([f32::MAX; 2], [f32::MIN; 2]);
        for v in ship.vertices {
            let w = rot.apply([
                v.x as f32 - cen[0],
                v.y as f32 - cen[1],
                v.z as f32 - cen[2],
            ]);
            let z = dist + w[2];
            for (i, c) in [w[0] / z, w[1] / z].iter().enumerate() {
                mn[i] = mn[i].min(*c);
                mx[i] = mx[i].max(*c);
            }
        }
        ((mn[0] + mx[0]) / 2.0, (mn[1] + mx[1]) / 2.0)
    }

    /// Экранная проекция вершин (для тестов композиции).
    #[cfg(test)]
    pub(super) fn project_ship(
        &self,
        ship: &Ship,
        cen: [f32; 3],
        pr: &Proj,
    ) -> Vec<(f32, f32, f32)> {
        ship.vertices
            .iter()
            .map(|v| {
                let w = self.rot.apply([
                    v.x as f32 - cen[0],
                    v.y as f32 - cen[1],
                    v.z as f32 - cen[2],
                ]);
                let z = INTRO_DIST + w[2];
                pr.point(w[0], w[1], z)
            })
            .collect()
    }
}
