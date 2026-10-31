//! Имена и постоянные цвета планет.

use super::model::twist;

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
