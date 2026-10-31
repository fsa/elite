//! `d_Alloys` (адрес 0x6211).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_ALLOYS: Ship = Ship {
    name: "d_Alloys",
    addr: 0x6211,
    category: 1,
    stats: Stats {
        cargo: 128, collision: 10, fire_chance: 0,
        max_speed: 16, energy: 16, cash: 0,
        launch: 0, size: 12, flags: 0,
        factor: 3, laser: 0, rating: 32,
    },
    vertices: &[
        Vertex { x: -15, y: -22, z: -9, lod: 0x1f, faces: [255, 255] },
        Vertex { x: -15, y: 38, z: -9, lod: 0x1f, faces: [255, 255] },
        Vertex { x: 19, y: 32, z: 11, lod: 0x15, faces: [255, 255] },
        Vertex { x: 10, y: -46, z: 6, lod: 0x15, faces: [255, 255] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [1, 0], faces: 0xff },
        Edge { lod: 0x11, v: [2, 1], faces: 0xff },
        Edge { lod: 0x15, v: [3, 2], faces: 0xff },
        Edge { lod: 0x11, v: [3, 0], faces: 0xff },
    ],
    faces: &[
    ],
};
