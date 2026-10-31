//! `dTargletH` (адрес 0x636a).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_DTARGLETH: Ship = Ship {
    name: "dTargletH",
    addr: 0x636a,
    category: 4,
    stats: Stats {
        cargo: 240, collision: 40, fire_chance: 120,
        max_speed: 30, energy: 20, cash: 50,
        launch: 1, size: 20, flags: 4,
        factor: 2, laser: 5, rating: 100,
    },
    vertices: &[
        Vertex { x: -9, y: -24, z: -32, lod: 0x1f, faces: [19, 34] },
        Vertex { x: -9, y: 24, z: -32, lod: 0x1e, faces: [18, 68] },
        Vertex { x: 9, y: -6, z: -26, lod: 0x1f, faces: [50, 0] },
        Vertex { x: 9, y: 6, z: -26, lod: 0x1e, faces: [36, 255] },
        Vertex { x: 9, y: 0, z: -8, lod: 0x1f, faces: [101, 0] },
        Vertex { x: -9, y: 0, z: 40, lod: 0x1f, faces: [22, 255] },
        Vertex { x: -9, y: -38, z: 12, lod: 0x1f, faces: [22, 51] },
        Vertex { x: -9, y: 38, z: 12, lod: 0x1e, faces: [20, 85] },
        Vertex { x: 9, y: -10, z: -15, lod: 0x1f, faces: [99, 0] },
        Vertex { x: 9, y: 10, z: -15, lod: 0x1e, faces: [69, 0] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [8, 4], faces: 0x60 },
        Edge { lod: 0x1f, v: [8, 2], faces: 0x30 },
        Edge { lod: 0x1f, v: [3, 2], faces: 0x20 },
        Edge { lod: 0x1f, v: [9, 3], faces: 0x40 },
        Edge { lod: 0x1f, v: [9, 4], faces: 0x50 },
        Edge { lod: 0x1f, v: [5, 4], faces: 0x65 },
        Edge { lod: 0x1f, v: [8, 6], faces: 0x63 },
        Edge { lod: 0x1f, v: [2, 0], faces: 0x32 },
        Edge { lod: 0x1f, v: [3, 1], faces: 0x42 },
        Edge { lod: 0x1f, v: [9, 7], faces: 0x54 },
        Edge { lod: 0x1f, v: [6, 5], faces: 0x61 },
        Edge { lod: 0x1f, v: [6, 0], faces: 0x31 },
        Edge { lod: 0x1f, v: [1, 0], faces: 0x21 },
        Edge { lod: 0x1f, v: [7, 1], faces: 0x41 },
        Edge { lod: 0x1f, v: [7, 5], faces: 0x51 },
    ],
    faces: &[
        Face { nx: 36, ny: 0, nz: 0, flags: 0x1f },
        Face { nx: -36, ny: 0, nz: 0, flags: 0x9e },
        Face { nx: 36, ny: 0, nz: -104, flags: 0x3f },
        Face { nx: 46, ny: -42, nz: -14, flags: 0x7f },
        Face { nx: 46, ny: 42, nz: -14, flags: 0x3e },
        Face { nx: 20, ny: 5, nz: 7, flags: 0x1f },
        Face { nx: 20, ny: -5, nz: 7, flags: 0x5e },
    ],
};
