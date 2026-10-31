//! Боевые эффекты вида: лазер игрока, ответные лучи и вспышка попадания.
//!
//! Оригинал не хранит флаг выстрела противника: луч живёт один кадр, а
//! попадание видно по падению `ShieldFront`. Модуль ловит эти изменения в
//! памяти Z80 и превращает их в короткоживущие визуальные эффекты.

use super::memory::{Z80_SHIELD_FRONT, Z80_TEMP_LASER};
use super::rng::Rng;
use super::scene::{Body, BodyKind};
use super::view::NEAR;

/// Сколько секунд виден луч собственного лазера.
const LASER_TIME: f32 = 0.10;
/// Сколько секунд виден луч выстрела противника.
///
/// В оригинале луч живёт один кадр (флаг `oFlags` сбрасывается сразу после
/// печати), но при 50 Гц это почти незаметно. Чуть более долгий след делает
/// атаку читаемой, не превращая её в постоянную линию.
const ENEMY_LASER_TIME: f32 = 0.14;
/// Время красной рамки вида после попадания.
const DAMAGE_FLASH_TIME: f32 = 0.25;

/// Выстрел противника: отрезок от корабля к окрестности перекрестия.
///
/// Храним координаты вида, чтобы линия правильно поворачивалась вместе с
/// полем, пока живёт.
pub(super) struct EnemyShot {
    pub(super) from: [f32; 3],
    pub(super) to: [f32; 3],
    pub(super) life: f32,
}

/// Состояние боевых эффектов.
pub(super) struct Combat {
    /// Лучи выстрелов противников.
    pub(super) enemy_shots: Vec<EnemyShot>,
    /// Красная рамка вида после попадания.
    pub(super) damage_flash: f32,
    /// Сколько ещё виден луч собственного лазера.
    pub(super) laser_timer: f32,
    /// Предыдущий `TempLaser` из Z80 — по его росту ловим выстрел.
    prev_laser_temp: Option<u8>,
    /// Предыдущий передний щит Z80 — по его падению ловим атаку.
    prev_z80_shield: Option<u8>,
}

impl Combat {
    pub(super) fn new() -> Self {
        Combat {
            enemy_shots: Vec::new(),
            damage_flash: 0.0,
            laser_timer: 0.0,
            prev_laser_temp: None,
            prev_z80_shield: None,
        }
    }

    pub(super) fn reset(&mut self) {
        self.enemy_shots.clear();
        self.damage_flash = 0.0;
        self.laser_timer = 0.0;
        self.prev_laser_temp = None;
        self.prev_z80_shield = None;
    }

    /// Поймать выстрел игрока по нагреву лазера.
    ///
    /// В оригинале выстрел виден только по нагреву лазера: `TempLaser`
    /// подскакивает на несколько единиц, затем остывает.
    pub(super) fn observe_laser(&mut self, mem: &[u8]) {
        let temp = mem[Z80_TEMP_LASER];
        if let Some(prev) = self.prev_laser_temp {
            if temp > prev.saturating_add(2) {
                self.laser_timer = LASER_TIME;
            }
        }
        self.prev_laser_temp = Some(temp);
    }

    /// Поймать попадание по падению переднего щита и показать ответный луч.
    ///
    /// Оригинал не оставляет флаг выстрела противника в памяти: бит 3
    /// сбрасывается сразу после печати луча. Зато попадание уменьшает
    /// `ShieldFront` — по этому падению и показываем ответный луч.
    pub(super) fn observe_shield(&mut self, mem: &[u8], bodies: &[Body], rng: &mut Rng) {
        let shield_raw = mem[Z80_SHIELD_FRONT];
        let shield_dropped = self.prev_z80_shield.is_some_and(|prev| shield_raw < prev);
        self.prev_z80_shield = Some(shield_raw);
        if shield_dropped {
            self.damage_flash = DAMAGE_FLASH_TIME;
            self.spawn_enemy_shots(bodies, rng);
        }
    }

    /// Погасить отжившие лучи противников и таймеры.
    pub(super) fn tick(&mut self, dt: f32) {
        self.laser_timer = (self.laser_timer - dt).max(0.0);
        for shot in &mut self.enemy_shots {
            shot.life -= dt;
        }
        self.enemy_shots.retain(|shot| shot.life > 0.0);
        self.damage_flash = (self.damage_flash - dt).max(0.0);
    }

    /// Показать выстрел противника: луч от ближайших кораблей впереди к
    /// окрестности перекрестия — так же, как это делает оригинал.
    fn spawn_enemy_shots(&mut self, bodies: &[Body], rng: &mut Rng) {
        let mut targets: Vec<usize> = bodies
            .iter()
            .enumerate()
            .filter(|(_, b)| b.kind == BodyKind::Ship && b.model.is_some() && b.pos[2] > NEAR)
            .map(|(i, _)| i)
            .collect();
        targets.sort_by(|&a, &b| {
            bodies[a].pos[2]
                .partial_cmp(&bodies[b].pos[2])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for i in targets.into_iter().take(2) {
            let origin = bodies[i].pos;
            let jx = (rng.next_u32() % 161) as f32 - 80.0;
            let jy = (rng.next_u32() % 121) as f32 - 60.0;
            self.enemy_shots.push(EnemyShot {
                from: origin,
                to: [jx, jy, 8000.0],
                life: ENEMY_LASER_TIME,
            });
        }
    }
}
