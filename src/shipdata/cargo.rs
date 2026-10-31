//! `d_Cargo` (адрес 0x6228).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_CARGO: Ship = Ship {
    name: "d_Cargo",
    addr: 0x6228,
    category: 1,
    stats: Stats {
        cargo: 0, collision: 20, fire_chance: 0,
        max_speed: 15, energy: 17, cash: 0,
        launch: 0, size: 12, flags: 0,
        factor: 2, laser: 0, rating: 31,
    },
    vertices: &[
        Vertex { x: -24, y: -13, z: 9, lod: 0x1f, faces: [35, 17] },
        Vertex { x: -24, y: -13, z: -9, lod: 0x1e, faces: [66, 17] },
        Vertex { x: 24, y: -13, z: 9, lod: 0x1e, faces: [48, 34] },
        Vertex { x: 24, y: -13, z: -9, lod: 0x1e, faces: [32, 68] },
        Vertex { x: 24, y: 16, z: 0, lod: 0x1f, faces: [96, 85] },
        Vertex { x: -24, y: 16, z: 0, lod: 0x1e, faces: [86, 17] },
        Vertex { x: -24, y: 5, z: 15, lod: 0x1f, faces: [54, 17] },
        Vertex { x: -24, y: 5, z: -15, lod: 0x1e, faces: [84, 17] },
        Vertex { x: 24, y: 5, z: 15, lod: 0x1e, faces: [96, 51] },
        Vertex { x: 24, y: 5, z: -15, lod: 0x1e, faces: [64, 85] },
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
        Face { nx: 96, ny: 0, nz: 0, flags: 0x1f },
        Face { nx: -96, ny: 0, nz: 0, flags: 0x9e },
        Face { nx: 0, ny: -51, nz: 0, flags: 0x5f },
        Face { nx: 0, ny: -18, nz: 48, flags: 0x5f },
        Face { nx: 0, ny: -18, nz: -48, flags: 0x7e },
        Face { nx: 0, ny: 41, nz: -30, flags: 0x3f },
        Face { nx: 0, ny: 41, nz: 30, flags: 0x1e },
    ],
};
