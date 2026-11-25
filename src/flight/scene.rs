//! Сцена полёта: объекты, прочитанные из слотов Z80.
//!
//! Z80 уже ведёт координаты в системе игрока, поэтому здесь нет симуляции:
//! модуль только превращает слоты `SlotObj`, координаты солнца и планеты в
//! список [`Body`], готовый для рендера.

use crate::render3d::{ENGINE_IDLE, Rotation, ShipTint};
use crate::shipdata::{self, Ship};

use super::memory::{
    self, z80_in_free_space, Z80_OBJ_CORIOLIS, Z80_OBJ_MISSILE, Z80_OBJ_SLOT0,
    Z80_OENERG_DESTROYED, Z80_PLANET_CRD, Z80_SLOT_OADRDATA, Z80_SLOT_OENERG, Z80_SLOT_SIZE,
    Z80_SUN_CRD,
};
use super::rng::Rng;

pub(super) const BODY_SRC_CORIOLIS: u8 = 0;
pub(super) const BODY_SRC_SLOT0: u8 = 1;
pub(super) const BODY_SRC_MISSILE: u8 = 7;
pub(super) const BODY_SRC_SUN: u8 = 8;
pub(super) const BODY_SRC_PLANET: u8 = 9;
use super::palette::{sun_tint, CYAN, SUN_COLOR};

/// Радиус солнца в единицах координат игры.
///
/// Оригинал считает радиус диска как `0xA00000 / z`: при `z ≈ 96000` это
/// ≈108 px в окне 256x128. Фокус `1.15 * высота_вида` переводит это в
/// `R = (108/128) * 96000 / 1.15 ≈ 70000`.
pub(super) const SUN_RADIUS: f32 = 70_000.0;
/// Радиус планеты. Меньше калиброванного радиуса оригинала: планета
/// рисуется сплошной заливкой, поэтому при том же радиусе заливала бы почти
/// весь вид. Такой диск читается как планета, а не как стена.
pub(super) const PLANET_RADIUS: f32 = 42_000.0;

/// Привязка оттенков к слотам Z80.
///
/// Постоянного идентификатора у объекта в игре нет: пока он существует, его
/// «личность» — номер слота и адрес модели. Если слот освобождается или в нём
/// появляется другая модель, это считается новым объектом и он получает
/// следующий псевдослучайный тон. Пока объект остаётся в слоте, тон не меняется.
pub(super) struct ShipTints {
    slots: [Option<TintSlot>; 8],
    /// Битом отмечены слоты, которые были заняты при текущей сборке сцены.
    seen: u8,
    /// Оттенок только что уничтоженного корабля для его обломков.
    pending_debris: Option<PendingDebris>,
    /// Отдельный генератор: цвета не должны зависеть от звёзд и выстрелов.
    rng: Rng,
}

#[derive(Clone, Copy)]
struct TintSlot {
    model: u16,
    /// `None` у контейнера без известного источника: он остаётся обычным.
    tint: Option<ShipTint>,
}

#[derive(Clone, Copy)]
struct PendingDebris {
    tint: ShipTint,
    /// Сколько сборок сцены ещё ждём появления обломков.
    frames: u32,
}

/// Обломки появляются на следующем игровом такте (~0,18 с), но кадров
/// рендера между ними бывает около десятка. Запас в секунду покрывает и
/// паузу, и замедление эмуляции.
const DEBRIS_TINT_FRAMES: u32 = 60;

impl ShipTints {
    pub(super) fn new() -> Self {
        Self {
            slots: [None; 8],
            seen: 0,
            pending_debris: None,
            rng: Rng::new(0x5EED_1234),
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::new();
    }

    /// Начать очередной обход слотов.
    fn begin(&mut self) {
        self.seen = 0;
        if let Some(pending) = &mut self.pending_debris {
            pending.frames = pending.frames.saturating_sub(1);
            if pending.frames == 0 {
                self.pending_debris = None;
            }
        }
    }

    /// Запомнить оттенок уничтоженного корабля, чтобы красить его обломки
    /// и выпавший груз.
    fn destroyed(&mut self, source: u8) {
        if let Some(slot) = self.slots[source as usize].take() {
            if let Some(tint) = slot.tint {
                self.pending_debris = Some(PendingDebris {
                    tint,
                    frames: DEBRIS_TINT_FRAMES,
                });
            }
        } else if let Some(pending) = &mut self.pending_debris {
            pending.frames = DEBRIS_TINT_FRAMES;
        }
    }

    /// Вернуть оттенок объекта в слоте, назначив новый при необходимости.
    fn assign(&mut self, source: u8, model: &Ship) -> Option<ShipTint> {
        let idx = source as usize;
        self.seen |= 1 << idx;
        // У контейнера флаг `tintable` выключен: на витрине он обычный.
        // В полёте он всё же может получить цвет корабля, из которого выпал.
        if !model.tintable && model.name != "d_Cargo" {
            return None;
        }
        let fresh = self.slots[idx].map_or(true, |prev| prev.model != model.addr);
        if fresh {
            // Обломки (`ShipSet2Alloys`) и выпавший груз сохраняют цвет
            // уничтоженного корабля: это его куски и его груз, а не новые
            // объекты. Если источник неизвестен, груз остаётся обычным.
            let tint = match model.name {
                "d_Alloys" => Some(self.pending_debris.map_or_else(
                    || ShipTint::from_seed(self.rng.next_u32()),
                    |pending| pending.tint,
                )),
                "d_Cargo" => self.pending_debris.map(|pending| pending.tint),
                _ => Some(ShipTint::from_seed(self.rng.next_u32())),
            };
            self.slots[idx] = Some(TintSlot {
                model: model.addr,
                tint,
            });
            tint
        } else {
            self.slots[idx].expect("слот уже проверен").tint
        }
    }

    /// Освободить оттенки слотов, в которых объекта больше нет.
    fn finish(&mut self) {
        for (idx, slot) in self.slots.iter_mut().enumerate() {
            if self.seen & (1 << idx) == 0 {
                *slot = None;
            }
        }
    }
}

/// Тип объекта в сцене.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BodyKind {
    Ship,
    Station,
    Planet,
    Sun,
}

/// Объект сцены в координатах вида.
#[derive(Clone)]
pub(super) struct Body {
    /// Источник в памяти Z80: станция, слоты кораблей, ракета, солнце, планета.
    /// Нужен для интерполяции между игровыми тактами.
    pub(super) source: u8,
    pub(super) kind: BodyKind,
    /// Геометрия (для кораблей и станции).
    pub(super) model: Option<&'static Ship>,
    /// Положение в координатах вида (камера в начале координат).
    pub(super) pos: [f32; 3],
    /// Ориентация модели относительно вида.
    pub(super) rot: Rotation,
    /// Радиус для проекции.
    pub(super) radius: f32,
    /// Цвет диска (планета или солнце); у кораблей не используется.
    pub(super) color: u32,
    /// Оттенок корпуса и рёбер у кораблей, их обломков и выпавшего груза.
    pub(super) tint: Option<ShipTint>,
    /// Интенсивность свечения двигателя: из приращения скорости слота Z80,
    /// [`ENGINE_IDLE`] для объектов без такого показателя.
    pub(super) engine: f32,
}

/// Собрать сцену из текущего состояния памяти игры.
///
/// `system_color` — цвет системы на карте галактики: из него получается
/// лёгкий оттенок почти белой звезды. Ноль означает, что цвет неизвестен.
pub(super) fn build(
    mem: &[u8],
    planet_color: u32,
    system_color: u32,
    tints: &mut ShipTints,
) -> Vec<Body> {
    tints.begin();
    let sun_color = if system_color == 0 {
        SUN_COLOR
    } else {
        sun_tint(system_color)
    };
    let mut bodies: Vec<Body> = Vec::with_capacity(9);
    // Станция, шесть основных слотов и слот ракеты. Пустые слоты
    // обнулены: `oEnerg == 0`, ссылки на модель нет.
    push_z80_slot(
        &mut bodies,
        mem,
        Z80_OBJ_CORIOLIS,
        BODY_SRC_CORIOLIS,
        BodyKind::Station,
        tints,
        false,
    );
    for i in 0..6 {
        push_z80_slot(
            &mut bodies,
            mem,
            Z80_OBJ_SLOT0 + i * Z80_SLOT_SIZE,
            BODY_SRC_SLOT0 + i as u8,
            BodyKind::Ship,
            tints,
            true,
        );
    }
    push_z80_slot(
        &mut bodies,
        mem,
        Z80_OBJ_MISSILE,
        BODY_SRC_MISSILE,
        BodyKind::Ship,
        tints,
        false,
    );

    // Солнце и планета: те же координаты вида, что использует оригинал.
    // В межпланетном пространстве (`InFreeSpace <> 0`) оригинал в `GameMain`
    // вызывает `PrnSunPlanet` только по нулевому флагу, поэтому диски не
    // рисуются. Координаты там всё ещё меняются для компаса, но при перехвате
    // таргонами остаются от покинутой системы: нативный диск заливал бы весь вид.
    if !z80_in_free_space(mem) {
        push_z80_celestial(
            &mut bodies,
            mem,
            Z80_SUN_CRD,
            BODY_SRC_SUN,
            BodyKind::Sun,
            SUN_RADIUS,
            sun_color,
        );
        push_z80_celestial(
            &mut bodies,
            mem,
            Z80_PLANET_CRD,
            BODY_SRC_PLANET,
            BodyKind::Planet,
            PLANET_RADIUS,
            planet_color,
        );
    }

    tints.finish();
    bodies
}

/// Модель по адресу блока данных объекта из памяти Z80.
fn map_ship_ptr(ptr: u16) -> Option<&'static Ship> {
    shipdata::SHIPS.iter().find(|s| s.addr == ptr)
}

/// Модель и положение объекта в слоте Z80.
///
/// Возвращает `None` для пустого (`oEnerg == 0`) слота и для неизвестной
/// модели. Уничтоженный слот (`oEnerg == 0xFF`) ещё хранит модель: по ней
/// [`slot_pose`] находит радиус облака частиц.
fn slot_object(mem: &[u8], base: usize) -> Option<(&'static Ship, [f32; 3])> {
    if base + Z80_SLOT_SIZE > mem.len() || mem[base + Z80_SLOT_OENERG] == 0 {
        return None;
    }
    let ptr = u16::from_le_bytes([
        mem[base + Z80_SLOT_OADRDATA],
        mem[base + Z80_SLOT_OADRDATA + 1],
    ]);
    let model = map_ship_ptr(ptr)?;
    let pos = [
        memory::read_signed24(&mem[base..base + 3]),
        memory::read_signed24(&mem[base + 3..base + 6]),
        memory::read_signed24(&mem[base + 6..base + 9]),
    ];
    Some((model, pos))
}

/// Положение и радиус объекта в слоте (по `collision` модели).
///
/// Нужен эффектам уничтожения: слот уже помечен `oEnerg == 0xFF` и не
/// попадает в сцену, но координаты и размер модели ещё можно прочитать.
pub(super) fn slot_pose(mem: &[u8], base: usize) -> Option<([f32; 3], f32)> {
    let (model, pos) = slot_object(mem, base)?;
    Some((pos, model.stats.collision as f32))
}

/// Добавить объект из слота Z80 (станция, корабль, ракета).
///
/// Слоты пусты, если `oEnerg == 0`; уничтоженный слот (`0xFF`) тоже не
/// рисуется целым кораблём — вместо него показывается облако частиц.
/// Ориентация хранится двумя осями: ось Z модели — по `+0x0A/0C/0E`,
/// ось Y — по `+0x10/12/14` (знаковые байты), ось X достраивается как
/// `Y × Z`. Оттенок получают модели с признаком `tintable`, а также груз,
/// выпавший из уничтоженного корабля. Станция, ракеты и объекты с
/// неизвестным источником остаются исходных цветов.
fn push_z80_slot(
    bodies: &mut Vec<Body>,
    mem: &[u8],
    base: usize,
    source: u8,
    kind: BodyKind,
    tints: &mut ShipTints,
    tintable: bool,
) {
    if mem.get(base + Z80_SLOT_OENERG) == Some(&Z80_OENERG_DESTROYED) {
        if tintable {
            tints.destroyed(source);
        }
        return;
    }
    let Some((model, pos)) = slot_object(mem, base) else {
        return;
    };
    let tint = if tintable {
        tints.assign(source, model)
    } else {
        None
    };
    let z_axis = [
        memory::read_signed8(mem[base + 0x0A]),
        memory::read_signed8(mem[base + 0x0C]),
        memory::read_signed8(mem[base + 0x0E]),
    ];
    let y_axis = [
        memory::read_signed8(mem[base + 0x10]),
        memory::read_signed8(mem[base + 0x12]),
        memory::read_signed8(mem[base + 0x14]),
    ];
    let z_axis = normalize(z_axis);
    let y_axis = normalize(y_axis);
    let x_axis = normalize(cross(y_axis, z_axis));
    let rot = if y_axis == [0.0; 3] || z_axis == [0.0; 3] || x_axis == [0.0; 3] {
        Rotation::identity()
    } else {
        Rotation::from_axes(x_axis, y_axis, z_axis)
    };
    bodies.push(Body {
        source,
        kind,
        model: Some(model),
        pos,
        rot,
        radius: model.stats.collision as f32,
        color: match kind {
            BodyKind::Station => 0xFFA0_FFFF,
            _ => CYAN,
        },
        tint,
        engine: match kind {
            BodyKind::Ship => memory::z80_slot_engine(mem, base),
            _ => ENGINE_IDLE,
        },
    });
}

/// Добавить солнце или планету из координат Z80 (они уже в системе вида).
fn push_z80_celestial(
    bodies: &mut Vec<Body>,
    mem: &[u8],
    addr: usize,
    source: u8,
    kind: BodyKind,
    radius: f32,
    color: u32,
) {
    if addr + 9 > mem.len() {
        return;
    }
    bodies.push(Body {
        source,
        kind,
        model: None,
        pos: [
            memory::read_signed24(&mem[addr..addr + 3]),
            memory::read_signed24(&mem[addr + 3..addr + 6]),
            memory::read_signed24(&mem[addr + 6..addr + 9]),
        ],
        rot: Rotation::identity(),
        radius,
        color,
        tint: None,
        engine: ENGINE_IDLE,
    });
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Нормировать вектор (нулевой остаётся нулевым).
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-6 {
        [0.0; 3]
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}
