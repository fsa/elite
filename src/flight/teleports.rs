//! Эффект ухода корабля или спасательной капсулы «в никуда».
//!
//! И корабли, и выброшенные капсулы уходят через один и тот же счётчик в
//! дополнительном слоте: `CounterShipHyperJump` инкрементирует его, пока
//! значение попадает в `0xA0..0xA7`, и только в конце обнуляет `oEnerg`.
//! `ShipHyperJump` печатает круги всё это время, пока объект ещё в слоте;
//! к моменту очистки `FlgObjNear` уже сброшен, поэтому ждать пустого слота
//! нельзя. Нативный эффект поднимается по самому счётчику и показывает
//! сжатие к точке с короткой вспышкой на последней координате объекта.
//!
//! Облако взрывов живёт в [`super::explosions`] и создаётся только по `0xFF`,
//! поэтому обычное уничтожение сюда не попадает. Слоты станции и ракеты тоже
//! исключены: это не корабли и не капсулы.

use crate::render3d::Rotation;
use crate::shipdata::Ship;

use super::memory::{
    self, ext_slot_base, slot_base, Z80_FLG_OBJ_NEAR, Z80_SLOT_COUNT, Z80_SLOT_OENERG,
};
use super::scene;
use super::view;

/// Частота игрового такта Z80: столько шагов счётчика в секунду.
const GAME_TICKS_PER_SEC: f32 = 5.5;
/// Как долго эффект живёт после исчезновения объекта.
pub(super) const TELEPORT_TRAIL_TIME: f32 = 0.5;
/// Минимальный радиус эффекта в координатах вида, чтобы капсула была видна.
const MIN_RADIUS: f32 = 30.0;
/// Насколько эффект больше радиуса столкновения модели.
const RADIUS_SCALE: f32 = 2.0;

/// Один уход объекта.
pub(super) struct Warp {
    /// Координата объекта: пока слот занят, обновляется вместе с ним.
    pub(super) center: [f32; 3],
    /// Радиус в координатах вида, от которого стягиваются кольца.
    pub(super) radius: f32,
    /// Возраст в секундах (`0..duration`).
    pub(super) age: f32,
    /// Сколько секунд идёт эффект: отсчёт оригинала плюс короткий хвост.
    pub(super) duration: f32,
    /// Слот, из которого вышел объект.
    slot: usize,
    /// Пока слот занят, эффект следует за объектом; после очистки остаётся
    /// на последней координате и движется вместе с миром.
    following: bool,
}

/// Все текущие эффекты и снимок счётчиков, по которому ловится уход.
pub(super) struct Teleports {
    pub(super) warps: Vec<Warp>,
    /// Был ли слот в отсчёте на прошлом опросе: один уход не должен
    /// породить несколько эффектов за время отсчёта.
    prev_warping: [bool; Z80_SLOT_COUNT],
}

impl Teleports {
    pub(super) fn new() -> Self {
        Teleports {
            warps: Vec::new(),
            prev_warping: [false; Z80_SLOT_COUNT],
        }
    }

    /// Забыть эффекты и снимок при открытии вида или начале перехода.
    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    /// Найти уходящие объекты и создать эффекты на их координате.
    ///
    /// `FlgObjNear` — тот же гейт, что оригинал проверяет перед печатью своих
    /// кругов. Дополнительно смотрим, был ли объект перед камерой в текущем
    /// направлении вида. При гибели игрока и в переходе (`ClearAroundObj`)
    /// эффект не показывается: там объекты исчезают все сразу и это не уход.
    pub(super) fn observe(&mut self, mem: &[u8], view_rot: &Rotation) {
        let near = mem.get(Z80_FLG_OBJ_NEAR).copied().unwrap_or(0) != 0
            && !memory::z80_player_dead(mem)
            && !memory::z80_in_transition(mem);
        let cam = view_rot.transpose();

        let mut warping = [false; Z80_SLOT_COUNT];
        let mut track: [Option<[f32; 3]>; Z80_SLOT_COUNT] = [None; Z80_SLOT_COUNT];
        // Слоты 1..=6 — шесть боевых слотов; станция (0) и ракета (7)
        // не участвуют.
        for i in 1..Z80_SLOT_COUNT - 1 {
            let base = slot_base(i);
            let energy = mem.get(base + Z80_SLOT_OENERG).copied().unwrap_or(0);
            if energy == 0 {
                continue;
            }
            let Some((model, pos)) = scene::slot_object(mem, base) else {
                continue;
            };
            if !is_ship_or_pod(model) {
                continue;
            }
            let flags = mem.get(ext_slot_base(i)).copied().unwrap_or(0);
            if flags & 0xF8 != 0xA0 {
                continue;
            }
            // `CounterShipHyperJump` инкрементирует младшие три бита; слот
            // очищается, когда они снова станут нулями.
            let remaining = 8 - (flags & 7);
            track[i] = Some(pos);
            if cam.apply(pos)[2] <= view::NEAR {
                continue;
            }
            warping[i] = true;
            if near && !self.prev_warping[i] {
                let seconds_left = remaining as f32 / GAME_TICKS_PER_SEC;
                self.spawn(i, pos, model.stats.collision as f32, seconds_left);
            }
        }
        // Пока объект в слоте, кольца идут вместе с ним. Когда слот опустел,
        // эффект отпускается и дальше живёт как обычный след.
        for warp in &mut self.warps {
            if !warp.following {
                continue;
            }
            match track[warp.slot] {
                Some(pos) => warp.center = pos,
                None => warp.following = false,
            }
        }
        self.prev_warping = warping;
    }

    /// Старить эффекты и убирать отжившие.
    pub(super) fn tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }
        for warp in &mut self.warps {
            warp.age += dt;
        }
        self.warps.retain(|w| w.age < w.duration);
    }

    /// Игровой такт: повернуть эффект вместе с миром и сдвинуть к игроку.
    ///
    /// Эффект живёт в той же системе координат, что и тела сцены; шаг `delta_z`
    /// совпадает с шагом `CalcNewCrdObj_Pl`.
    pub(super) fn tick_world(&mut self, turn: &Rotation, delta_z: f32) {
        for warp in &mut self.warps {
            warp.center = turn.apply(warp.center);
            warp.center[2] = toward_zero(warp.center[2], delta_z);
        }
    }

    /// Создать сжимающиеся кольца на месте уходящего объекта.
    fn spawn(&mut self, slot: usize, center: [f32; 3], model_radius: f32, seconds_left: f32) {
        if self.warps.len() >= Z80_SLOT_COUNT {
            self.warps.remove(0);
        }
        self.warps.push(Warp {
            center,
            radius: model_radius.max(MIN_RADIUS) * RADIUS_SCALE,
            age: 0.0,
            duration: seconds_left + TELEPORT_TRAIL_TIME,
            slot,
            following: true,
        });
    }
}

/// Корабли и капсула телепортируются, обломки, груз и астероиды — нет.
fn is_ship_or_pod(model: &Ship) -> bool {
    model.name == "d_EscapePod" || (model.name != "d_Alloys" && model.tintable)
}

/// Сдвинуть координату к нулю на `step`, как у облаков взрывов.
fn toward_zero(v: f32, step: f32) -> f32 {
    if v.abs() <= step {
        0.0
    } else if v > 0.0 {
        v - step
    } else {
        v + step
    }
}
