//! `d_Alloys` (адрес 0x6211).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова. Это обломки (`d_Alloys`),
//! которые оригинал создаёт при гибели корабля (`ShipSet2Alloys` — две штуки).
//! В ROM у модели четыре вершины, четыре ребра и **ни одной нормали**, поэтому
//! нативный рендер не мог залить её — обломки оставались пустым каркасом.
//! Добавлены две грани с противоположными нормалями: плоский обломок виден
//! с любой стороны и заливается обычным цветом кораблей.

use super::{Edge, Face, Ship, Stats, Vertex};

pub(super) static SHIP_D_ALLOYS: Ship = Ship {
    name: "d_Alloys",
    full_name: "Alloy Plate",
    addr: 0x6211,
    category: 1,
    stats: Stats {
        cargo: 128, collision: 10, fire_chance: 0,
        max_speed: 16, energy: 16, cash: 0,
        launch: 0, size: 12, flags: 0,
        factor: 3, laser: 0, rating: 32,
    },
    vertices: &[
        Vertex { x: -15, y: -22, z: -9, lod: 0x1f, faces: [0, 1] },
        Vertex { x: -15, y: 38, z: -9, lod: 0x1f, faces: [0, 1] },
        Vertex { x: 19, y: 32, z: 11, lod: 0x15, faces: [0, 1] },
        Vertex { x: 10, y: -46, z: 6, lod: 0x15, faces: [0, 1] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [1, 0], faces: 0x10 },
        Edge { lod: 0x11, v: [2, 1], faces: 0x10 },
        Edge { lod: 0x15, v: [3, 2], faces: 0x10 },
        Edge { lod: 0x11, v: [3, 0], faces: 0x10 },
    ],
    faces: &[
        // Нормаль посчитана по контуру 0→1→2→3 (Newell); вторая грань —
        // та же поверхность с противоположной нормалью, чтобы обломок был
        // залит и при повороте на 180°.
        Face { nx: 82, ny: 5, nz: -138, flags: 0x3f },
        Face { nx: -82, ny: -5, nz: 138, flags: 0xdf },
    ],
    face_styles: &[],
};
