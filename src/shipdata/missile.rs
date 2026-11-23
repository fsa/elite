//! `d_Missile` (адрес 0x61e3).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова. Отличие от импортированной
//! геометрии — грани 9..12 и конструктивные рёбра для их контуров: в ROM рёбра
//! оперения отнесены к боковым (4..7) и задней (8) граням корпуса, а нормалей
//! самих плоскостей нет. Поэтому оперение не заливалось. Четыре грани описывают
//! две диагональные плоскости, в которых лежат противоположные перья, по две
//! нормали на каждую плоскость: так перо заливается с любой стороны. Исходные
//! рёбра и их видимость не меняются — конструктивные рёбра (lod == 0) нужны
//! только для заливки и не рисуются.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_MISSILE: Ship = Ship {
    name: "d_Missile",
    full_name: "Missile",
    addr: 0x61e3,
    category: 0,
    tintable: false,
    stats: Stats {
        cargo: 0, collision: 40, fire_chance: 0,
        max_speed: 44, energy: 2, cash: 0,
        launch: 0, size: 14, flags: 0,
        factor: 2, laser: 0, rating: 336,
    },
    vertices: &[
        Vertex { x: 0, y: 0, z: 68, lod: 0x1f, faces: [32, 49] },
        Vertex { x: 8, y: -8, z: 36, lod: 0x1f, faces: [18, 100] },
        Vertex { x: 8, y: 8, z: 36, lod: 0x1e, faces: [49, 116] },
        Vertex { x: -8, y: 8, z: 36, lod: 0x1e, faces: [48, 118] },
        Vertex { x: -8, y: -8, z: 36, lod: 0x1e, faces: [32, 86] },
        Vertex { x: 8, y: 8, z: -44, lod: 0x1f, faces: [116, 136] },
        Vertex { x: 8, y: -8, z: -44, lod: 0x1e, faces: [100, 136] },
        Vertex { x: -8, y: -8, z: -44, lod: 0x1e, faces: [86, 136] },
        Vertex { x: -8, y: 8, z: -44, lod: 0x1e, faces: [117, 136] },
        Vertex { x: 12, y: 12, z: -44, lod: 0x9, faces: [116, 136] },
        Vertex { x: 12, y: -12, z: -44, lod: 0x8, faces: [100, 136] },
        Vertex { x: -12, y: -12, z: -44, lod: 0x8, faces: [86, 136] },
        Vertex { x: -12, y: 12, z: -44, lod: 0x8, faces: [117, 136] },
        Vertex { x: -8, y: 8, z: -12, lod: 0x9, faces: [117, 119] },
        Vertex { x: -8, y: -8, z: -12, lod: 0x8, faces: [86, 85] },
        Vertex { x: 8, y: 8, z: -12, lod: 0x8, faces: [116, 119] },
        Vertex { x: 8, y: -8, z: -12, lod: 0x8, faces: [100, 102] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [1, 0], faces: 0x21 },
        Edge { lod: 0x1f, v: [2, 0], faces: 0x31 },
        Edge { lod: 0x1f, v: [3, 0], faces: 0x30 },
        Edge { lod: 0x1f, v: [4, 0], faces: 0x20 },
        Edge { lod: 0x1f, v: [2, 1], faces: 0x41 },
        Edge { lod: 0x1f, v: [4, 1], faces: 0x62 },
        Edge { lod: 0x1f, v: [6, 1], faces: 0x64 },
        Edge { lod: 0x1f, v: [3, 2], faces: 0x73 },
        Edge { lod: 0x1f, v: [5, 2], faces: 0x74 },
        Edge { lod: 0x1f, v: [4, 3], faces: 0x50 },
        Edge { lod: 0x1f, v: [7, 4], faces: 0x65 },
        Edge { lod: 0x1f, v: [6, 5], faces: 0x84 },
        Edge { lod: 0x1f, v: [8, 5], faces: 0x87 },
        Edge { lod: 0x1f, v: [8, 3], faces: 0x75 },
        Edge { lod: 0x1f, v: [7, 6], faces: 0x86 },
        Edge { lod: 0x1f, v: [8, 7], faces: 0x85 },
        // Оперение из ROM: рёбра отнесены к боковым (4..7) и задней (8) граням
        // корпуса. Их видимость сохраняется как в оригинале, в том числе
        // силуэт в виде звёздочки при взгляде строго сзади.
        Edge { lod: 0x9, v: [9, 5], faces: 0x87 },
        Edge { lod: 0x9, v: [10, 6], faces: 0x86 },
        Edge { lod: 0x9, v: [11, 7], faces: 0x86 },
        Edge { lod: 0x9, v: [12, 8], faces: 0x87 },
        Edge { lod: 0x9, v: [15, 9], faces: 0x74 },
        Edge { lod: 0x9, v: [14, 11], faces: 0x65 },
        Edge { lod: 0x9, v: [13, 12], faces: 0x57 },
        Edge { lod: 0x9, v: [10, 16], faces: 0x64 },
        // Контуры четырёх треугольников оперения для заливки. Замыкающее ребро
        // идёт по поверхности корпуса (вершины 5/6/7/8 — задний срез, 13..16 —
        // переднее основание пера); в ROM его нет: ребро 2..5 проходит через
        // вершину 15, не разбиваясь. Оба полубайта отведены двум сторонам своей
        // плоскости (x = y — грани 9/10, x = -y — грани 11/12).
        Edge { lod: 0x0, v: [5, 9], faces: 0xA9 },
        Edge { lod: 0x0, v: [9, 15], faces: 0xA9 },
        Edge { lod: 0x0, v: [5, 15], faces: 0xA9 },
        Edge { lod: 0x0, v: [6, 10], faces: 0xCB },
        Edge { lod: 0x0, v: [10, 16], faces: 0xCB },
        Edge { lod: 0x0, v: [6, 16], faces: 0xCB },
        Edge { lod: 0x0, v: [7, 11], faces: 0xA9 },
        Edge { lod: 0x0, v: [11, 14], faces: 0xA9 },
        Edge { lod: 0x0, v: [7, 14], faces: 0xA9 },
        Edge { lod: 0x0, v: [8, 12], faces: 0xCB },
        Edge { lod: 0x0, v: [12, 13], faces: 0xCB },
        Edge { lod: 0x0, v: [8, 13], faces: 0xCB },
    ],
    faces: &[
        Face { nx: -64, ny: 0, nz: 16, flags: 0x9f },
        Face { nx: 64, ny: 0, nz: 16, flags: 0x1e },
        Face { nx: 0, ny: -64, nz: 16, flags: 0x5f },
        Face { nx: 0, ny: 64, nz: 16, flags: 0x1e },
        Face { nx: 32, ny: 0, nz: 0, flags: 0x1f },
        Face { nx: -32, ny: 0, nz: 0, flags: 0x9e },
        Face { nx: 0, ny: -32, nz: 0, flags: 0x5f },
        Face { nx: 0, ny: 32, nz: 0, flags: 0x1e },
        Face { nx: 0, ny: 0, nz: -176, flags: 0x3f },
        // Оперение. Два противоположных пера лежат в одной плоскости, поэтому
        // им хватает четырёх граней: по две нормали на плоскость x = y и
        // x = -y. Контур каждой грани — два треугольника, не пересекающиеся
        // на экране (кроме ракурса строго вдоль плоскости, когда заливки нет).
        Face { nx: 32, ny: -32, nz: 0, flags: 0x5f },
        Face { nx: -32, ny: 32, nz: 0, flags: 0x9f },
        Face { nx: 32, ny: 32, nz: 0, flags: 0x1f },
        Face { nx: -32, ny: -32, nz: 0, flags: 0xdf },
    ],
    face_styles: &[],
};
