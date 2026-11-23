//! Боевые эффекты вида: лазер игрока, ответные лучи и вспышка попадания.
//!
//! Оригинал не хранит отдельный флаг выстрела противника: луч живёт один
//! кадр, а бит `oFlg_1` сбрасывается сразу после печати. Зато урон от выстрела
//! остаётся в `DamageFrontShield`/`DamageRearShield` до следующего игрового
//! такта, а сами щиты показывают, с какой стороны был корабль. Модуль ловит
//! эти изменения в памяти Z80 и превращает их в короткоживущие визуальные
//! эффекты.

use crate::render3d;

use super::memory::{
    Z80_DAMAGE_FRONT_SHIELD, Z80_DAMAGE_REAR_SHIELD, Z80_SHIELD_FRONT, Z80_SHIELD_REAR,
    Z80_TEMP_LASER,
};
use super::rng::Rng;
use super::scene::{Body, BodyKind};
use super::view::{self, ViewDir, NEAR};

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
/// Один выстрел противника добавляет 7 к урону щита.
const HIT_DAMAGE: i32 = 7;
/// Сколько лучей противника может появиться за один игровой такт.
const MAX_ENEMY_SHOTS: usize = 2;

/// Выстрел противника: отрезок от корабля к окрестности перекрестия.
///
/// Координаты — в системе игрока (как слоты Z80), а не в координатах текущего
/// вида. Поэтому линия правильно поворачивается вместе с полем, пока живёт;
/// проекция в конкретный вид делается уже при отрисовке.
pub(super) struct EnemyShot {
    /// Индекс корабля в текущей сцене: один источник даёт только один луч.
    pub(super) source: usize,
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
    /// Предыдущие щиты Z80 — по их падению ловим атаку.
    prev_front_shield: Option<u8>,
    prev_rear_shield: Option<u8>,
}

impl Combat {
    pub(super) fn new() -> Self {
        Combat {
            enemy_shots: Vec::new(),
            damage_flash: 0.0,
            laser_timer: 0.0,
            prev_laser_temp: None,
            prev_front_shield: None,
            prev_rear_shield: None,
        }
    }

    pub(super) fn reset(&mut self) {
        self.enemy_shots.clear();
        self.damage_flash = 0.0;
        self.laser_timer = 0.0;
        self.prev_laser_temp = None;
        self.prev_front_shield = None;
        self.prev_rear_shield = None;
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

    /// Поймать попадание по урону щитов и показать ответные лучи.
    ///
    /// Оригинал не оставляет флаг выстрела противника в памяти: бит 3
    /// `oFlg_1` сбрасывается сразу после печати луча. Зато `DamageFrontShield`
    /// и `DamageRearShield` держат весь урон за такт, даже когда щит уже нулевой
    /// и сам по себе больше не меняется.
    pub(super) fn observe_shield(
        &mut self,
        mem: &[u8],
        bodies: &[Body],
        view_dir: ViewDir,
        ticked: bool,
        rng: &mut Rng,
    ) {
        let front = mem[Z80_SHIELD_FRONT];
        let rear = mem[Z80_SHIELD_REAR];
        let front_drop = self
            .prev_front_shield
            .map_or(0, |prev| prev.saturating_sub(front) as i32);
        let rear_drop = self
            .prev_rear_shield
            .map_or(0, |prev| prev.saturating_sub(rear) as i32);
        self.prev_front_shield = Some(front);
        self.prev_rear_shield = Some(rear);

        let pulse = if ticked {
            mem.get(Z80_DAMAGE_FRONT_SHIELD).copied().unwrap_or(0) as i32
                + mem.get(Z80_DAMAGE_REAR_SHIELD).copied().unwrap_or(0) as i32
        } else {
            0
        };
        let damage = pulse.max(front_drop + rear_drop);
        if damage <= 0 {
            return;
        }

        self.damage_flash = DAMAGE_FLASH_TIME;
        let hits = ((damage + HIT_DAMAGE - 1) / HIT_DAMAGE)
            .clamp(1, MAX_ENEMY_SHOTS as i32) as usize;
        self.spawn_enemy_shots(bodies, view_dir, hits, rng);
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

    /// Показать выстрел противника: луч от видимых кораблей к окрестности
    /// перекрестия — так же, как это делает оригинал.
    fn spawn_enemy_shots(
        &mut self,
        bodies: &[Body],
        view_dir: ViewDir,
        hits: usize,
        rng: &mut Rng,
    ) {
        let rot = view::rotation(view_dir);
        let cam = rot.transpose();
        let mut targets: Vec<(f32, usize)> = bodies
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                b.kind == BodyKind::Ship
                    && b.model.is_some_and(|m| m.stats.fire_chance != 0)
            })
            .filter_map(|(i, b)| {
                let depth = cam.apply(b.pos)[2];
                (depth > NEAR).then_some((depth, i))
            })
            .collect();
        targets.sort_by(|a, b| a.0.total_cmp(&b.0));
        let active: Vec<usize> = self.enemy_shots.iter().map(|shot| shot.source).collect();
        for (_, i) in targets.into_iter().take(hits) {
            let from = laser_origin(&bodies[i]);
            let jx = (rng.next_u32() % 161) as f32 - 80.0;
            let jy = (rng.next_u32() % 121) as f32 - 60.0;
            // Точка прицела задана в координатах вида; переводим её в систему
            // игрока, чтобы после поворота камеры при отрисовке луч снова
            // оказался у перекрестия, а не за камерой.
            let to = rot.apply([jx, jy, 8000.0]);
            // Один корабль может выстрелить несколько раз, пока след прошлого
            // выстрела ещё жив. Не умножаем линии от него, а обновляем уже
            // существующий луч: новая цель и новый срок жизни.
            if active.contains(&i) {
                if let Some(shot) = self.enemy_shots.iter_mut().find(|s| s.source == i) {
                    shot.from = from;
                    shot.to = to;
                    shot.life = ENEMY_LASER_TIME;
                }
                continue;
            }
            self.enemy_shots.push(EnemyShot {
                source: i,
                from,
                to,
                life: ENEMY_LASER_TIME,
            });
        }
    }
}

/// Вершина, из которой корабль оригинала выпускает лазер (`dLaser` в
/// `ShipData`). В нативном рендере модели центрированы, поэтому берём тот же
/// повёрнутый локальный вектор, что и при отрисовке корпуса.
fn laser_origin(body: &Body) -> [f32; 3] {
    let Some(model) = body.model else {
        return body.pos;
    };
    let Some(v) = model.vertices.get(model.stats.laser as usize) else {
        return body.pos;
    };
    let center = render3d::model_center(model);
    let local = [
        v.x as f32 - center[0],
        v.y as f32 - center[1],
        v.z as f32 - center[2],
    ];
    let shifted = body.rot.apply(local);
    [
        body.pos[0] + shifted[0],
        body.pos[1] + shifted[1],
        body.pos[2] + shifted[2],
    ]
}
