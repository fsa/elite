//! Сцена полёта: объекты, прочитанные из слотов Z80.
//!
//! Z80 уже ведёт координаты в системе игрока, поэтому здесь нет симуляции:
//! модуль только превращает слоты `SlotObj`, координаты солнца и планеты в
//! список [`Body`], готовый для рендера.

use crate::render3d::Rotation;
use crate::shipdata::{self, Ship};

use super::memory::{
    self, Z80_OBJ_CORIOLIS, Z80_OBJ_MISSILE, Z80_OBJ_SLOT0, Z80_PLANET_CRD, Z80_SLOT_OADRDATA,
    Z80_SLOT_OENERG, Z80_SLOT_SIZE, Z80_SUN_CRD,
};
use super::palette::{CYAN, SUN_COLOR};

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
    pub(super) kind: BodyKind,
    /// Геометрия (для кораблей и станции).
    pub(super) model: Option<&'static Ship>,
    /// Положение в координатах вида (камера в начале координат).
    pub(super) pos: [f32; 3],
    /// Ориентация модели относительно вида.
    pub(super) rot: Rotation,
    /// Радиус для проекции.
    pub(super) radius: f32,
    /// Цвет диска (планета или солнце); корабли рисуются общим цветом.
    pub(super) color: u32,
}

/// Собрать сцену из текущего состояния памяти игры.
pub(super) fn build(mem: &[u8], planet_color: u32) -> Vec<Body> {
    let mut bodies: Vec<Body> = Vec::with_capacity(9);
    // Станция, шесть основных слотов и слот ракеты. Пустые слоты
    // обнулены: `oEnerg == 0`, ссылки на модель нет.
    push_z80_slot(&mut bodies, mem, Z80_OBJ_CORIOLIS, BodyKind::Station);
    for i in 0..6 {
        push_z80_slot(
            &mut bodies,
            mem,
            Z80_OBJ_SLOT0 + i * Z80_SLOT_SIZE,
            BodyKind::Ship,
        );
    }
    push_z80_slot(&mut bodies, mem, Z80_OBJ_MISSILE, BodyKind::Ship);

    // Солнце и планета: те же координаты вида, что использует оригинал.
    push_z80_celestial(
        &mut bodies,
        mem,
        Z80_SUN_CRD,
        BodyKind::Sun,
        SUN_RADIUS,
        SUN_COLOR,
    );
    push_z80_celestial(
        &mut bodies,
        mem,
        Z80_PLANET_CRD,
        BodyKind::Planet,
        PLANET_RADIUS,
        planet_color,
    );

    bodies
}

/// Модель по адресу блока данных объекта из памяти Z80.
fn map_ship_ptr(ptr: u16) -> Option<&'static Ship> {
    shipdata::SHIPS.iter().find(|s| s.addr == ptr)
}

/// Добавить объект из слота Z80 (станция, корабль, ракета).
///
/// Слоты пусты, если `oEnerg == 0`; ориентация хранится двумя осями:
/// ось Z модели — по `+0x0A/0C/0E`, ось Y — по `+0x10/12/14`
/// (знаковые байты), ось X достраивается как `Y × Z`.
fn push_z80_slot(bodies: &mut Vec<Body>, mem: &[u8], base: usize, kind: BodyKind) {
    if base + Z80_SLOT_SIZE > mem.len() || mem[base + Z80_SLOT_OENERG] == 0 {
        return;
    }
    let ptr = u16::from_le_bytes([
        mem[base + Z80_SLOT_OADRDATA],
        mem[base + Z80_SLOT_OADRDATA + 1],
    ]);
    let Some(model) = map_ship_ptr(ptr) else {
        return;
    };
    let pos = [
        memory::read_signed24(&mem[base..base + 3]),
        memory::read_signed24(&mem[base + 3..base + 6]),
        memory::read_signed24(&mem[base + 6..base + 9]),
    ];
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
        kind,
        model: Some(model),
        pos,
        rot,
        radius: model.stats.collision as f32,
        color: match kind {
            BodyKind::Station => 0xFFA0_FFFF,
            _ => CYAN,
        },
    });
}

/// Добавить солнце или планету из координат Z80 (они уже в системе вида).
fn push_z80_celestial(
    bodies: &mut Vec<Body>,
    mem: &[u8],
    addr: usize,
    kind: BodyKind,
    radius: f32,
    color: u32,
) {
    if addr + 9 > mem.len() {
        return;
    }
    bodies.push(Body {
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
