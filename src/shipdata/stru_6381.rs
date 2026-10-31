//! `stru_6381` (адрес 0x6381).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_STRU_6381: Ship = Ship {
    name: "stru_6381",
    addr: 0x6381,
    category: 3,
    stats: Stats {
        cargo: 0, collision: 99, fire_chance: 255,
        max_speed: 47, energy: 240, cash: 600,
        launch: 54, size: 55, flags: 12,
        factor: 2, laser: 6, rating: 2448,
    },
    vertices: &[
        Vertex { x: 32, y: 0, z: 80, lod: 0x1f, faces: [36, 102] },
        Vertex { x: 32, y: 0, z: -80, lod: 0x1e, faces: [53, 102] },
        Vertex { x: 32, y: 69, z: 40, lod: 0x1f, faces: [2, 102] },
        Vertex { x: 32, y: 69, z: -40, lod: 0x1e, faces: [3, 255] },
        Vertex { x: 32, y: -69, z: 40, lod: 0x1e, faces: [20, 102] },
        Vertex { x: 32, y: -69, z: -40, lod: 0x1e, faces: [21, 102] },
        Vertex { x: -24, y: 0, z: 160, lod: 0x1f, faces: [36, 255] },
        Vertex { x: -24, y: 0, z: -160, lod: 0x1e, faces: [53, 119] },
        Vertex { x: -24, y: 139, z: 80, lod: 0x1f, faces: [2, 119] },
        Vertex { x: -24, y: 139, z: -80, lod: 0x1e, faces: [3, 119] },
        Vertex { x: -24, y: -139, z: 80, lod: 0x1e, faces: [20, 119] },
        Vertex { x: -24, y: -139, z: -80, lod: 0x1e, faces: [21, 119] },
        Vertex { x: -24, y: 0, z: 64, lod: 0x1d, faces: [119, 119] },
        Vertex { x: -24, y: 0, z: -64, lod: 0x1c, faces: [119, 119] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [2, 0], faces: 0x62 },
        Edge { lod: 0x1f, v: [4, 0], faces: 0x64 },
        Edge { lod: 0x1f, v: [6, 0], faces: 0x42 },
        Edge { lod: 0x1f, v: [3, 1], faces: 0x63 },
        Edge { lod: 0x1f, v: [5, 1], faces: 0x65 },
        Edge { lod: 0x1f, v: [7, 1], faces: 0x53 },
        Edge { lod: 0x1f, v: [3, 2], faces: 0x60 },
        Edge { lod: 0x1f, v: [8, 2], faces: 0x20 },
        Edge { lod: 0x1f, v: [9, 3], faces: 0x30 },
        Edge { lod: 0x1f, v: [5, 4], faces: 0x61 },
        Edge { lod: 0x1f, v: [10, 4], faces: 0x41 },
        Edge { lod: 0x1f, v: [11, 5], faces: 0x51 },
        Edge { lod: 0x1f, v: [8, 6], faces: 0x72 },
        Edge { lod: 0x1f, v: [10, 6], faces: 0x74 },
        Edge { lod: 0x1f, v: [9, 7], faces: 0x73 },
        Edge { lod: 0x1f, v: [11, 7], faces: 0x75 },
        Edge { lod: 0x1f, v: [9, 8], faces: 0x70 },
        Edge { lod: 0x1f, v: [11, 10], faces: 0x71 },
        Edge { lod: 0x1d, v: [13, 12], faces: 0x77 },
    ],
    faces: &[
        Face { nx: 75, ny: 43, nz: 0, flags: 0x1f },
        Face { nx: 75, ny: -43, nz: 0, flags: 0x5e },
        Face { nx: 75, ny: 22, nz: 37, flags: 0x1f },
        Face { nx: 75, ny: 22, nz: -37, flags: 0x3e },
        Face { nx: 75, ny: -22, nz: 37, flags: 0x5e },
        Face { nx: 75, ny: -22, nz: -37, flags: 0x7e },
        Face { nx: 32, ny: 0, nz: 0, flags: 0x1f },
        Face { nx: -32, ny: 0, nz: 0, flags: 0x9e },
    ],
};
