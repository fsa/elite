//! `dSidewinderH` (адрес 0x633c).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_DSIDEWINDERH: Ship = Ship {
    name: "dSidewinderH",
    addr: 0x633c,
    category: 4,
    stats: Stats {
        cargo: 0, collision: 65, fire_chance: 130,
        max_speed: 37, energy: 70, cash: 50,
        launch: 1, size: 22, flags: 12,
        factor: 2, laser: 0, rating: 256,
    },
    vertices: &[
        Vertex { x: -32, y: 0, z: 36, lod: 0x1f, faces: [22, 255] },
        Vertex { x: 32, y: 0, z: 36, lod: 0x1e, faces: [38, 53] },
        Vertex { x: 64, y: 0, z: -28, lod: 0x1f, faces: [2, 51] },
        Vertex { x: -64, y: 0, z: -28, lod: 0x1e, faces: [1, 255] },
        Vertex { x: 0, y: 16, z: -28, lod: 0x1f, faces: [22, 2] },
        Vertex { x: 0, y: -16, z: -28, lod: 0x1e, faces: [64, 53] },
        Vertex { x: -12, y: 6, z: -28, lod: 0xf, faces: [0, 0] },
        Vertex { x: 12, y: 6, z: -28, lod: 0xe, faces: [0, 0] },
        Vertex { x: 12, y: -6, z: -28, lod: 0xc, faces: [0, 0] },
        Vertex { x: -12, y: -6, z: -28, lod: 0xc, faces: [0, 0] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [1, 0], faces: 0x65 },
        Edge { lod: 0x1f, v: [3, 0], faces: 0x41 },
        Edge { lod: 0x1f, v: [4, 0], faces: 0x61 },
        Edge { lod: 0x1f, v: [5, 0], faces: 0x54 },
        Edge { lod: 0x1f, v: [2, 1], faces: 0x32 },
        Edge { lod: 0x1f, v: [4, 1], faces: 0x62 },
        Edge { lod: 0x1f, v: [5, 1], faces: 0x53 },
        Edge { lod: 0x1f, v: [4, 2], faces: 0x20 },
        Edge { lod: 0x1f, v: [5, 2], faces: 0x30 },
        Edge { lod: 0x1f, v: [4, 3], faces: 0x10 },
        Edge { lod: 0x1f, v: [5, 3], faces: 0x40 },
        Edge { lod: 0xf, v: [7, 6], faces: 0x0 },
        Edge { lod: 0xb, v: [9, 6], faces: 0x0 },
        Edge { lod: 0xb, v: [8, 7], faces: 0x0 },
        Edge { lod: 0xb, v: [9, 8], faces: 0x0 },
    ],
    faces: &[
        Face { nx: 0, ny: 0, nz: -112, flags: 0x3f },
        Face { nx: -12, ny: 47, nz: 6, flags: 0x9f },
        Face { nx: 12, ny: 47, nz: 6, flags: 0x1e },
        Face { nx: 12, ny: -47, nz: 6, flags: 0x5e },
        Face { nx: -12, ny: -47, nz: 6, flags: 0xde },
        Face { nx: 0, ny: -32, nz: 8, flags: 0x5f },
        Face { nx: 0, ny: 32, nz: 8, flags: 0x1e },
    ],
};
