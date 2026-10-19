//! Нативная модель галактики Elite: 256 звёзд из 6-байтного семени.
//!
//! Это независимая реализация — те же числа, что даёт Z80, но без эмуляции.
//! Семя галактики лежит в памяти игры (`PilotGalaxyData0`, у Кладова `0xD090`);
//! каждая следующая звезда получается из предыдущей четырьмя шагами
//! `ModifyPlanetData` (у Кладова `0xE063`):
//!
//! ```text
//! (w0, w1, w2) <- (w1, w2, w0 + w1 + w2)   (мод 0x10000)
//! ```
//!
//! Три 16-битных слова `w0..w2` — по два байта (младший, старший). Из них
//! берутся координаты звезды на карте галактики:
//!
//! * `x = старший байт w1` — горизонталь, `0..255`;
//! * `y_raw = старший байт w0` — «сырая» вертикаль `0..255`, экранная строка
//!   `y = (y_raw >> 1) + 24` (поле `24..151`).
//!
//! Из-за деления пополам вертикаль занимает вдвое меньше строк, чем значений
//! `y_raw`: на экране карта сжата по вертикали вдвое относительно горизонтали.
//! Именно поэтому при показе галактики масштабы по осям разные (см. `hdmap`).

/// Число звёзд в галактике.
pub const COUNT: usize = 256;

/// Поле звезды по вертикали: строки `24..=151` (128 строк).
pub const FIELD_TOP: i32 = 24;
pub const FIELD_HEIGHT: i32 = 128;

/// Одна звезда галактики.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Planet {
    /// Порядковый номер в последовательности галактики (`0..255`).
    pub index: usize,
    /// Экранная горизонталь `0..255` — старший байт `w1`.
    pub x: i32,
    /// Сырая вертикаль `0..255` — старший байт `w0`.
    pub y_raw: i32,
    /// Шесть байт состояния звезды: `w0`, `w1`, `w2` (мл., ст.).
    pub state: [u8; 6],
}

impl Planet {
    /// Экранная строка карты `24..151`.
    pub fn y(&self) -> i32 {
        (self.y_raw >> 1) + FIELD_TOP
    }

    /// Радиус кружка планеты (как на малой карте игры, `PrnPlanetOnSmallMap`):
    /// класс `(hi(w2) & 0x0C) >> 2` отображается в `3..5`.
    pub fn radius(&self) -> i32 {
        let c = ((self.state[5] as i32) & 0x0C) >> 2;
        (if c == 0 { 1 } else { c }) + 2
    }
}

/// Раскладка адресов галактики в памяти конкретной версии игры.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GalaxyLayout {
    /// Семя галактики (`PilotGalaxyData0`), 6 байт.
    pub seed: usize,
    /// Таблица диграфов имён (`TxtPairOfChr`/`byte_FDB3`), 64 байта.
    pub digraphs: usize,
    /// Номер текущей галактики (`PilotNumGalaxy`, 0..7).
    pub galaxy: usize,
    /// Координаты текущей системы: X и «сырой» Y.
    pub sys_x: usize,
    pub sys_y: usize,
    /// Топливо (`Pilot_Fuel`): задаёт радиус достижимости на карте.
    pub fuel: usize,
}

/// Версия Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_GALAXY: GalaxyLayout = GalaxyLayout {
    seed: 0xD090,
    digraphs: 0xFDB3,
    galaxy: 0xD065,
    sys_x: 0xD099,
    sys_y: 0xD097,
    fuel: 0xD082,
};

/// Оригинальная версия (`zx_bin/elite.bin`).
pub const ELITE_GALAXY: GalaxyLayout = GalaxyLayout {
    seed: 0xA852,
    digraphs: 0xFBC3,
    galaxy: 0xA827,
    sys_x: 0xA85B,
    sys_y: 0xA859,
    fuel: 0xA844,
};

/// Шаг `ModifyPlanetData`.
fn twist(s: [u8; 6]) -> [u8; 6] {
    let w0 = u16::from_le_bytes([s[0], s[1]]);
    let w1 = u16::from_le_bytes([s[2], s[3]]);
    let w2 = u16::from_le_bytes([s[4], s[5]]);
    let n0 = w1;
    let n1 = w2;
    let n2 = w0.wrapping_add(w1).wrapping_add(w2);
    [
        n0 as u8,
        (n0 >> 8) as u8,
        n1 as u8,
        (n1 >> 8) as u8,
        n2 as u8,
        (n2 >> 8) as u8,
    ]
}

/// Построить все звёзды галактики из 6-байтного семени.
pub fn planets(seed: [u8; 6]) -> Vec<Planet> {
    let mut s = seed;
    let mut out = Vec::with_capacity(COUNT);
    for index in 0..COUNT {
        out.push(Planet {
            index,
            x: s[3] as i32,
            y_raw: s[1] as i32,
            state: s,
        });
        for _ in 0..4 {
            s = twist(s);
        }
    }
    out
}

/// Имя планеты — порт `GeneratePlanetName` (у Кладова `0xE016`).
///
/// Имя собирается из диграфов таблицы `byte_FDB3` (у Кладова `0xFDB3`).
/// Таблица читается по индексу `hi(w2) & 0x1F`; на каждом шаге состояние
/// прокручивается `ModifyPlanetData`. Шагов три, а если младший байт `w0`
/// имеет бит 6 — четыре. Второй символ диграфа `?` не выводится.
pub fn name(state: [u8; 6], digraphs: &[u8]) -> String {
    let mut s = state;
    let calls = if s[0] & 0x40 != 0 { 4 } else { 3 };
    let mut out = String::new();
    for _ in 0..calls {
        let a = (s[5] & 0x1F) as usize;
        let i = a * 2;
        if a != 0 && i + 1 < digraphs.len() {
            out.push(digraphs[i] as char);
            if digraphs[i + 1] != b'?' {
                out.push(digraphs[i + 1] as char);
            }
        }
        s = twist(s);
    }
    out
}

/// Постоянный цвет планеты на карте галактики.
///
/// Цвет выводится из тех же данных, что и имя: на каждом шаге прокрутки
/// `ModifyPlanetData` в хеш подмешивается индекс диграфа (`hi(w2) & 0x1F`)
/// и байт состояния. Поэтому цвет постоянен для одной и той же звезды, но
/// различается от звезды к звезде — так же, как различаются имена.
pub fn color(state: [u8; 6]) -> u32 {
    // FNV-1a по «имени» звезды.
    let mut s = state;
    let mut h: u32 = 0x811C_9DC5;
    for _ in 0..4 {
        h ^= (s[5] & 0x1F) as u32;
        h = h.wrapping_mul(0x0100_0193);
        h ^= s[0] as u32;
        h = h.wrapping_mul(0x0100_0193);
        s = twist(s);
    }
    h ^= h >> 13;
    h = h.wrapping_mul(0x5BD1_E995);
    h ^= h >> 15;

    let hue = (h % 360) as f32;
    let sat = 0.45 + ((h >> 9) & 0x7F) as f32 / 127.0 * 0.40;
    let light = 0.60 + ((h >> 17) & 0x3F) as f32 / 63.0 * 0.18;
    hsl_to_rgb(hue, sat, light)
}

/// HSL (оттенок `0..360`, насыщенность и светлота `0..1`) в `0xFFRRGGBB`.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> u32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let ch = |v: f32| (((v + m) * 255.0).round().clamp(0.0, 255.0)) as u32;
    0xFF00_0000 | (ch(r) << 16) | (ch(g) << 8) | ch(b)
}

/// Квадрат расстояния от точки до звезды — та же метрика, что в игре
/// (`CalcDistanceToCursor2`): `dx^2 + ((dy_raw / 2))^2`. Так как вертикаль
/// карты сжата вдвое, «сырая» разница по вертикали тоже делится пополам.
pub fn distance2(px: i32, py_raw: i32, p: &Planet) -> i32 {
    let dx = px - p.x;
    let dy = (py_raw - p.y_raw).abs() >> 1;
    dx * dx + dy * dy
}

/// Ближайшая к точке звезда (первая при равенстве расстояний).
pub fn nearest<'a>(planets: &'a [Planet], px: i32, py_raw: i32) -> Option<&'a Planet> {
    planets
        .iter()
        .min_by_key(|p| distance2(px, py_raw, p))
}

/// Гиперпрыжковое расстояние между двумя системами в десятых долях светового
/// года — величина `FuelToPlanet` из игры (`CalcFuelForDistance`):
/// `4 * floor(sqrt(dist2))`, где `dist2 = dx^2 + (dy/2)^2`.
pub fn distance_tenths(a: &Planet, b: &Planet) -> i32 {
    let d2 = distance2(a.x, a.y_raw, b);
    let mut r = 0i32;
    while (r + 1) * (r + 1) <= d2 {
        r += 1;
    }
    4 * r
}

/// Гиперпрыжковое расстояние между двумя системами в световых годах.
/// Шаг — 0.4 св. года, как в игре.
pub fn distance_ly(a: &Planet, b: &Planet) -> f32 {
    distance_tenths(a, b) as f32 / 10.0
}

/// Радиус достижимости на карте галактики (в единицах поля `x`/строка) для
/// запаса топлива `fuel`: в игре `RadiusCircle = Pilot_Fuel >> 2`
/// (`Cmd_I_GlobalMap` / `CalcFuelForDistance`). Это окружность в метрике
/// [`distance2`], то есть обычный круг в координатах `(x, строка)`.
pub fn jump_radius(fuel: u8) -> i32 {
    (fuel >> 2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Семя галактики из запущенной версии Кладова (`tmp/wf/mem.bin`, `0xD090`).
    const SEED: [u8; 6] = [0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7];

    /// Таблица диграфов имён (`byte_FDB3`, 32 пары символов).
    const DIGRAPHS: &[u8] =
        b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION";

    #[test]
    fn planets_match_known_galaxy() {
        let p = planets(SEED);
        assert_eq!(p.len(), COUNT);
        // Текущая система Джеймсона — LAVE, звезда №7: (x=20, y_raw=173).
        assert_eq!((p[7].x, p[7].y_raw), (20, 173));
        assert_eq!(p[7].state.to_vec(), b"\x38\xad\x9c\x14\x1d\x15".to_vec());
        // Система ORESQU — звезда №204: (x=22, y_raw=217).
        assert_eq!((p[204].x, p[204].y_raw), (22, 217));
        assert_eq!(p[204].state.to_vec(), b"\xba\xd9\x90\x16\xb3\xf9".to_vec());
    }

    #[test]
    fn planet_names_match_game() {
        let p = planets(SEED);
        assert_eq!(name(p[7].state, DIGRAPHS), "LAVE");
        assert_eq!(name(p[204].state, DIGRAPHS), "ORESQU");
    }

    #[test]
    fn planet_color_is_stable_and_varied() {
        let p = planets(SEED);
        // Постоянство: одинаковые данные дают одинаковый цвет.
        assert_eq!(color(p[7].state), color(p[7].state));
        assert_ne!(color(p[7].state), color(p[204].state));
        // Цвет непрозрачный и не чёрный.
        for pl in &p {
            let c = color(pl.state);
            assert_eq!(c >> 24, 0xFF, "планета {}", pl.index);
            assert_ne!(c & 0x00FF_FFFF, 0, "планета {}", pl.index);
        }
        // Разные звёзды — разные цвета.
        let distinct: std::collections::HashSet<u32> =
            p.iter().map(|pl| color(pl.state)).collect();
        assert!(distinct.len() > 200, "слишком мало цветов: {}", distinct.len());
    }

    #[test]
    fn radius_is_three_to_five() {
        let p = planets(SEED);
        for pl in &p {
            let r = pl.radius();
            assert!((3..=5).contains(&r), "radius {r} for planet {}", pl.index);
        }
    }

    #[test]
    fn nearest_uses_squeezed_vertical_metric() {
        let p = planets(SEED);
        // Точка ровно на звезде №7 — она и ближайшая.
        assert_eq!(nearest(&p, 20, 173).unwrap().index, 7);
        // Сдвиг по «сырой» вертикали на 4 — это 2 экранные строки.
        assert_eq!(distance2(20, 177, &p[7]), 4);
        // Горизонталь не делится: сдвиг на 4 даёт 16.
        assert_eq!(distance2(24, 173, &p[7]), 16);
    }

    /// Гиперпрыжковое расстояние в световых годах: LAVE (№7) -> DISO (№147)
    /// равно 3.6 св. года, как в оригинале Elite.
    #[test]
    fn distance_in_light_years_matches_game() {
        let p = planets(SEED);
        assert_eq!(p[147].x, 11);
        assert_eq!(p[147].y_raw, 174);
        assert_eq!(distance_tenths(&p[7], &p[147]), 36);
        assert_eq!(distance_ly(&p[7], &p[147]), 3.6);
        assert_eq!(distance_ly(&p[7], &p[7]), 0.0);
    }

    /// Радиус достижимости: топливо 70 -> 17 единиц поля (как в карте I).
    #[test]
    fn jump_radius_matches_game() {
        assert_eq!(jump_radius(70), 17);
        assert_eq!(jump_radius(7), 1);
        assert_eq!(jump_radius(0), 0);
    }
}
