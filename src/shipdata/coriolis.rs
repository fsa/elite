//! `d_Coriolis` (адрес 0x63c6).
//!
//! Модель из таблицы `AdrDataShips` версии Кладова. Единственное отличие от
//! импортированной геометрии — грань 14: поверх входа (передняя сторона,
//! локальная `+Z`) добавлена чёрная поверхность. В оригинале вход оставался
//! чёрным просто потому, что фон сцены чёрный; нативная заливка корпуса
//! сделала бы его «прозрачным», поэтому вырез получил отдельную грань.

use super::{Edge, Face, FaceStyle, Ship, Stats, Vertex};

pub(super) static SHIP_D_CORIOLIS: Ship = Ship {
    name: "d_Coriolis",
    full_name: "Coriolis Station",
    addr: 0x63c6,
    category: 0,
    tintable: false,
    stats: Stats {
        cargo: 0, collision: 160, fire_chance: 0,
        max_speed: 0, energy: 240, cash: 0,
        launch: 6, size: 120, flags: 0,
        factor: 0, laser: 0, rating: 0,
    },
    vertices: &[
        Vertex { x: 160, y: 0, z: 160, lod: 0x1f, faces: [8, 193] },
        Vertex { x: 160, y: 0, z: -160, lod: 0x1e, faces: [92, 150] },
        Vertex { x: -160, y: 0, z: 160, lod: 0x1e, faces: [40, 211] },
        Vertex { x: -160, y: 0, z: -160, lod: 0x1e, faces: [77, 151] },
        Vertex { x: 160, y: -160, z: 0, lod: 0x1f, faces: [160, 92] },
        Vertex { x: 160, y: 160, z: 0, lod: 0x1e, faces: [193, 107] },
        Vertex { x: -160, y: 160, z: 0, lod: 0x1e, faces: [210, 123] },
        Vertex { x: -160, y: -160, z: 0, lod: 0x1e, faces: [163, 77] },
        Vertex { x: 0, y: 160, z: 160, lod: 0x1f, faces: [24, 178] },
        Vertex { x: 0, y: 160, z: -160, lod: 0x1e, faces: [107, 151] },
        Vertex { x: 0, y: -160, z: 160, lod: 0x1e, faces: [8, 163] },
        Vertex { x: 0, y: -160, z: -160, lod: 0x1e, faces: [74, 149] },
        Vertex { x: 10, y: -30, z: 160, lod: 0x1f, faces: [8, 14] },
        Vertex { x: 10, y: 30, z: 160, lod: 0x1e, faces: [8, 14] },
        Vertex { x: -10, y: 30, z: 160, lod: 0x1e, faces: [8, 14] },
        Vertex { x: -10, y: -30, z: 160, lod: 0x1e, faces: [8, 14] },
    ],
    edges: &[
        Edge { lod: 0x1f, v: [10, 0], faces: 0x80 },
        Edge { lod: 0x1f, v: [8, 0], faces: 0x81 },
        Edge { lod: 0x1f, v: [2, 8], faces: 0x82 },
        Edge { lod: 0x1f, v: [10, 2], faces: 0x83 },
        Edge { lod: 0x1f, v: [10, 4], faces: 0xa },
        Edge { lod: 0x1f, v: [4, 0], faces: 0xc },
        Edge { lod: 0x1f, v: [5, 0], faces: 0x1c },
        Edge { lod: 0x1f, v: [8, 5], faces: 0x1b },
        Edge { lod: 0x1f, v: [6, 8], faces: 0x2b },
        Edge { lod: 0x1f, v: [6, 2], faces: 0x2d },
        Edge { lod: 0x1f, v: [7, 2], faces: 0x3d },
        Edge { lod: 0x1f, v: [10, 7], faces: 0x3a },
        Edge { lod: 0x1f, v: [11, 1], faces: 0x59 },
        Edge { lod: 0x1f, v: [1, 9], faces: 0x69 },
        Edge { lod: 0x1f, v: [9, 3], faces: 0x79 },
        Edge { lod: 0x1f, v: [11, 3], faces: 0x49 },
        Edge { lod: 0x1f, v: [11, 4], faces: 0xa5 },
        Edge { lod: 0x1f, v: [1, 4], faces: 0xc5 },
        Edge { lod: 0x1f, v: [1, 5], faces: 0xc6 },
        Edge { lod: 0x1f, v: [9, 5], faces: 0xb6 },
        Edge { lod: 0x1f, v: [9, 6], faces: 0xb7 },
        Edge { lod: 0x1f, v: [3, 6], faces: 0xd7 },
        Edge { lod: 0x1f, v: [3, 7], faces: 0xd4 },
        Edge { lod: 0x1f, v: [11, 7], faces: 0xa4 },
        Edge { lod: 0x1e, v: [13, 12], faces: 0xe8 },
        Edge { lod: 0x1e, v: [14, 13], faces: 0xe8 },
        Edge { lod: 0x1e, v: [15, 14], faces: 0xe8 },
        Edge { lod: 0x1e, v: [12, 15], faces: 0xe8 },
    ],
    faces: &[
        Face { nx: 107, ny: -107, nz: 107, flags: 0x5f },
        Face { nx: 107, ny: 107, nz: 107, flags: 0x1e },
        Face { nx: -107, ny: 107, nz: 107, flags: 0x9e },
        Face { nx: -107, ny: -107, nz: 107, flags: 0xde },
        Face { nx: -107, ny: -107, nz: -107, flags: 0xfe },
        Face { nx: 107, ny: -107, nz: -107, flags: 0x7e },
        Face { nx: 107, ny: 107, nz: -107, flags: 0x3e },
        Face { nx: -107, ny: 107, nz: -107, flags: 0xbe },
        Face { nx: 0, ny: 0, nz: 160, flags: 0x1f },
        Face { nx: 0, ny: 0, nz: -160, flags: 0x3e },
        Face { nx: 0, ny: -160, nz: 0, flags: 0x5f },
        Face { nx: 0, ny: 160, nz: 0, flags: 0x1e },
        Face { nx: 160, ny: 0, nz: 0, flags: 0x1f },
        Face { nx: -160, ny: 0, nz: 0, flags: 0x9e },
        // Поверхность входа: та же нормаль, что у грани 8; её контур — только
        // внутренний квадрат из вершин 12..15, который у грани 8 является
        // вырезом. Поэтому заливки не перекрывают друг друга.
        Face { nx: 0, ny: 0, nz: 160, flags: 0x1f },
    ],
    face_styles: &[(14, FaceStyle::Color(0xFF00_0000))],
};
