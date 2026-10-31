//! `d_Minerals` (адрес 0x623f).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_MINERALS: Ship = Ship {
    name: "d_Minerals",
    addr: 0x623f,
    category: 1,
    stats: Stats {
        cargo: 176, collision: 16, fire_chance: 0,
        max_speed: 10, energy: 20, cash: 0,
        launch: 0, size: 10, flags: 0,
        factor: 5, laser: 0, rating: 30,
    },
    vertices: &[
        Vertex { x: -24, y: -25, z: 16, lod: 0x1f, faces: [18, 51] },
        Vertex { x: 0, y: 12, z: -10, lod: 0x1f, faces: [2, 51] },
        Vertex { x: 11, y: -6, z: 2, lod: 0x1f, faces: [1, 51] },
        Vertex { x: 12, y: 42, z: 7, lod: 0x1f, faces: [1, 34] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [1, 0], faces: 0x32 },
        Edge { lod: 0x1f, v: [2, 1], faces: 0x30 },
        Edge { lod: 0x1f, v: [3, 2], faces: 0x10 },
        Edge { lod: 0x1f, v: [3, 0], faces: 0x21 },
        Edge { lod: 0x1f, v: [2, 0], faces: 0x31 },
        Edge { lod: 0x1f, v: [3, 1], faces: 0x20 },
    ],
    faces: &[
        Face { nx: 177, ny: 12, nz: -148, flags: 0x3f },
        Face { nx: 100, ny: -25, nz: 213, flags: 0x5f },
        Face { nx: -202, ny: 104, nz: -34, flags: 0xbf },
        Face { nx: -2, ny: -51, nz: -74, flags: 0xff },
    ],
};
