//! `d_EscapePod` (адрес 0x61fa).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_ESCAPEPOD: Ship = Ship {
    name: "d_EscapePod",
    addr: 0x61fa,
    category: 1,
    stats: Stats {
        cargo: 32, collision: 16, fire_chance: 0,
        max_speed: 8, energy: 17, cash: 0,
        launch: 0, size: 8, flags: 1,
        factor: 4, laser: 0, rating: 50,
    },
    vertices: &[
        Vertex { x: -7, y: 0, z: 36, lod: 0x1f, faces: [18, 51] },
        Vertex { x: -7, y: -14, z: -12, lod: 0x1f, faces: [2, 51] },
        Vertex { x: -7, y: 14, z: -12, lod: 0x1e, faces: [1, 51] },
        Vertex { x: 21, y: 0, z: 0, lod: 0x1f, faces: [1, 34] },
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
        Face { nx: 52, ny: 0, nz: -122, flags: 0x3f },
        Face { nx: 39, ny: 103, nz: 30, flags: 0x1f },
        Face { nx: 39, ny: -103, nz: 30, flags: 0x5e },
        Face { nx: -112, ny: 0, nz: 0, flags: 0x9f },
    ],
};
