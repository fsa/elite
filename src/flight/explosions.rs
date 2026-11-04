//! Облака частиц после уничтожения объекта — функция, которой нет в оригинале.
//!
//! Оригинал печатает взрыв одним кадром (`PrnExplodeObj` по биту 4 `oFlags`),
//! поэтому нативно его не поймать. Зато `FireLaser` перед `DestroyObj` пишет в
//! `oEnerg` слота `0xFF`, и это состояние живёт до следующего игрового такта.
//! По переходу `не 0xFF → 0xFF` модуль создаёт облако частиц в координатах
//! вида; два секунды оно расширяется и плавно гаснет.
//!
//! Облако живёт в тех же координатах, что и тела сцены, поэтому на игровом
//! такте оно поворачивается вместе со звёздами и сдвигается к игроку на шаг
//! `CalcNewCrdObj_Pl` (см. [`Explosions::tick_world`]).

use crate::render3d::Rotation;

use super::memory::{
    Z80_OBJ_CORIOLIS, Z80_OBJ_MISSILE, Z80_OBJ_SLOT0, Z80_OENERG_DESTROYED, Z80_SLOT_OENERG,
    Z80_SLOT_SIZE,
};
use super::rng::Rng;
use super::scene;

/// Сколько секунд живёт облако: за это время частицы разлетаются и гаснут.
pub(super) const EXPLOSION_TIME: f32 = 2.0;
/// Число слотов `SlotObj`: станция, шесть кораблей и ракета.
const SLOT_COUNT: usize = 8;
/// Минимальный радиус облака в единицах координат (для мелких объектов).
const MIN_RADIUS: f32 = 12.0;
/// Пределы числа частиц в одном облаке.
const MIN_PARTICLES: usize = 12;
const MAX_PARTICLES: usize = 40;
/// Затухание скорости частиц, 1/с.
const DRAG: f32 = 1.6;

/// Частица облака.
pub(super) struct Particle {
    /// Смещение от центра облака в координатах вида.
    pub(super) offset: [f32; 3],
    /// Скорость разлёта в единицах координат в секунду.
    velocity: [f32; 3],
}

/// Облако частиц на месте уничтоженного объекта.
pub(super) struct Cloud {
    /// Центр в координатах вида на момент уничтожения.
    pub(super) center: [f32; 3],
    /// Возраст в секундах (`0..EXPLOSION_TIME`).
    pub(super) age: f32,
    pub(super) particles: Vec<Particle>,
}

/// Все облака и состояние слотов, по которому ловится уничтожение.
pub(super) struct Explosions {
    pub(super) clouds: Vec<Cloud>,
    /// `oEnerg` каждого слота на прошлом опросе.
    prev_energ: Option<[u8; SLOT_COUNT]>,
    rng: Rng,
}

impl Explosions {
    pub(super) fn new() -> Self {
        Explosions {
            clouds: Vec::new(),
            prev_energ: None,
            rng: Rng::new(0x0E4F_1051),
        }
    }

    /// Забыть всё при открытии вида или начале перехода.
    pub(super) fn reset(&mut self) {
        self.clouds.clear();
        self.prev_energ = None;
    }

    /// Найти только что уничтоженные слоты и создать на их месте облака.
    ///
    /// Слот считается уничтоженным при переходе `oEnerg` в `0xFF`. Повторный
    /// опрос того же состояния облака не дублирует. Простое обнуление слотов
    /// (например, `SetShipwreck2` при гибели игрока) взрывом не считается.
    pub(super) fn observe(&mut self, mem: &[u8]) {
        let mut energy = [0u8; SLOT_COUNT];
        for (i, e) in energy.iter_mut().enumerate() {
            *e = mem
                .get(slot_base(i) + Z80_SLOT_OENERG)
                .copied()
                .unwrap_or(0);
        }
        if let Some(prev) = self.prev_energ {
            for i in 0..SLOT_COUNT {
                if prev[i] != Z80_OENERG_DESTROYED && energy[i] == Z80_OENERG_DESTROYED {
                    if let Some((pos, radius)) = scene::slot_pose(mem, slot_base(i)) {
                        self.spawn(pos, radius);
                    }
                }
            }
        }
        self.prev_energ = Some(energy);
    }

    /// Двигать частицы и убирать отжившие облака.
    pub(super) fn tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }
        let drag = (1.0 - DRAG * dt).max(0.0);
        for cloud in &mut self.clouds {
            cloud.age += dt;
            for p in &mut cloud.particles {
                for k in 0..3 {
                    p.offset[k] += p.velocity[k] * dt;
                    p.velocity[k] *= drag;
                }
            }
        }
        self.clouds.retain(|c| c.age < EXPLOSION_TIME);
    }

    /// Игровой такт: повернуть облака вместе с миром и сдвинуть их к игроку.
    ///
    /// `delta_z` — тот же шаг, что `CalcNewCrdObj_Pl` вычитает из координаты
    /// `Z` каждого объекта (`b = 2 * (HighDeltaSpeed + 1)`), поэтому облако
    /// не отстаёт от системы координат, в которой живут тела сцены.
    pub(super) fn tick_world(&mut self, turn: &Rotation, delta_z: f32) {
        for cloud in &mut self.clouds {
            cloud.center = turn.apply(cloud.center);
            cloud.center[2] = toward_zero(cloud.center[2], delta_z);
            for p in &mut cloud.particles {
                p.offset = turn.apply(p.offset);
            }
        }
    }

    /// Создать облако из разлетающихся частиц.
    fn spawn(&mut self, center: [f32; 3], model_radius: f32) {
        // Облако заметно больше корпуса: взрыв должен читаться даже у мелкого
        // корабля, но оставаться пропорциональным станции или астероиду.
        let radius = model_radius.max(MIN_RADIUS) * 1.2;
        let count = (radius / 4.0) as usize;
        let count = count.clamp(MIN_PARTICLES, MAX_PARTICLES);
        let mut particles = Vec::with_capacity(count);
        for _ in 0..count {
            let dir = random_dir(&mut self.rng);
            // Рождение не в самом центре: облако сразу выглядит объёмным.
            let start = radius * (0.15 + 0.35 * self.rng.next_f32());
            let speed = radius * (0.6 + 0.8 * self.rng.next_f32()) / EXPLOSION_TIME;
            particles.push(Particle {
                offset: [dir[0] * start, dir[1] * start, dir[2] * start],
                velocity: [dir[0] * speed, dir[1] * speed, dir[2] * speed],
            });
        }
        if self.clouds.len() >= SLOT_COUNT {
            self.clouds.remove(0);
        }
        self.clouds.push(Cloud {
            center,
            age: 0.0,
            particles,
        });
    }
}

/// Адрес слота по индексу: станция, шесть кораблей, ракета.
fn slot_base(i: usize) -> usize {
    match i {
        0 => Z80_OBJ_CORIOLIS,
        i if i == SLOT_COUNT - 1 => Z80_OBJ_MISSILE,
        i => Z80_OBJ_SLOT0 + (i - 1) * Z80_SLOT_SIZE,
    }
}

/// Сдвинуть координату к нулю на `step` (объекты всегда летят на игрока).
fn toward_zero(v: f32, step: f32) -> f32 {
    if v.abs() <= step {
        0.0
    } else if v > 0.0 {
        v - step
    } else {
        v + step
    }
}

/// Случайное единичное направление (равномерно по шару).
fn random_dir(rng: &mut Rng) -> [f32; 3] {
    loop {
        let v = [
            rng.next_f32() * 2.0 - 1.0,
            rng.next_f32() * 2.0 - 1.0,
            rng.next_f32() * 2.0 - 1.0,
        ];
        let len2 = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
        if len2 > 1e-4 && len2 <= 1.0 {
            let inv = 1.0 / len2.sqrt();
            return [v[0] * inv, v[1] * inv, v[2] * inv];
        }
    }
}
