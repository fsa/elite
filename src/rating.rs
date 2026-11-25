//! Боевой рейтинг версии Кладова и его печатное название.
//!
//! Оригинал (`Cmd_L_PilotInfo`, `RAM:EFE0`) не складывает `PilotRating0..2`
//! в одно число. Он смотрит на третий байт (`PilotRating2`): если он нулевой,
//! пороги сравниваются со вторым байтом (`PilotRating1`); иначе пороги
//! сравниваются уже с `PilotRating2`. Начальный индекс названия в первом
//! случае `Harmless`, во втором — `Competent`. Поэтому очки из
//! `PilotRating0..1` сами по себе не определяют звание: радикально
//! неверно трактовать их как младшие 16 бит числа и сравнивать с
//! порогами `8/16/32/…/6400` — так безобидный пилот быстро становится
//! «Dangerous».
//!
//! Таблицы порогов из оригинала: `byte_F0AE = 08 10 20 40 80 FF`,
//! `byte_F0B4 = 02 0A 19 FF`. Индексы названий — сообщения `0x87..0x8F`
//! таблицы `word_F11A`.

/// Очки боя из `PilotRating0..1` (`u16`, little-endian).
///
/// `PilotRating2` в них не входит: оригинал использует этот байт отдельно
/// и не как обычный старший байт счётчика.
pub fn score(bytes: &[u8]) -> u32 {
    u16::from_le_bytes([byte(bytes, 0), byte(bytes, 1)]) as u32
}

/// Индекс названия рейтинга, который печатает `Cmd_L_PilotInfo`.
///
/// `0` — `Harmless`, `1` — `Mostly Harmless`, …, `8` — `Elite`.
pub fn tier(bytes: &[u8]) -> u8 {
    const R1_THRESHOLDS: [u8; 6] = [8, 16, 32, 64, 128, 255];
    const R2_THRESHOLDS: [u8; 4] = [2, 10, 25, 255];

    let r2 = byte(bytes, 2);
    if r2 == 0 {
        let r1 = byte(bytes, 1);
        R1_THRESHOLDS
            .iter()
            .take_while(|&&threshold| r1 >= threshold)
            .count() as u8
    } else {
        5 + R2_THRESHOLDS
            .iter()
            .take_while(|&&threshold| r2 >= threshold)
            .count() as u8
    }
}

/// Название рейтинга по индексу из [`tier`].
pub fn name(tier: u8) -> &'static str {
    match tier {
        0 => "Harmless",
        1 => "Mostly Harmless",
        2 => "Poor",
        3 => "Average",
        4 => "Above Average",
        5 => "Competent",
        6 => "Dangerous",
        7 => "Deadly",
        _ => "Elite",
    }
}

fn byte(bytes: &[u8], index: usize) -> u8 {
    bytes.get(index).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(r0: u8, r1: u8, r2: u8) -> &'static str {
        name(tier(&[r0, r1, r2]))
    }

    /// При нулевом `PilotRating2` оригинал сравнивает с порогами только
    /// `PilotRating1`. Младший байт в звание не входит.
    #[test]
    fn r1_thresholds_come_from_byte_f0ae() {
        for (r1, expected) in [
            (0, "Harmless"),
            (7, "Harmless"),
            (8, "Mostly Harmless"),
            (15, "Mostly Harmless"),
            (16, "Poor"),
            (31, "Poor"),
            (32, "Average"),
            (63, "Average"),
            (64, "Above Average"),
            (127, "Above Average"),
            (128, "Competent"),
            (254, "Competent"),
            (255, "Dangerous"),
        ] {
            assert_eq!(text(0, r1, 0), expected, "R1={r1}");
            assert_eq!(text(0xFF, r1, 0), expected, "R0 не влияет: R1={r1}");
        }
    }

    /// При ненулевом `PilotRating2` пороги сравниваются с ним, а отсчёт
    /// названий начинается с `Competent` (`byte_F0B4`).
    #[test]
    fn r2_thresholds_come_from_byte_f0b4() {
        for (r2, expected) in [
            (1, "Competent"),
            (2, "Dangerous"),
            (9, "Dangerous"),
            (10, "Deadly"),
            (24, "Deadly"),
            (25, "Elite"),
            (0xFF, "Elite"),
        ] {
            assert_eq!(text(0, 0, r2), expected, "R2={r2}");
            assert_eq!(text(0, 0xFF, r2), expected, "R1 не влияет: R2={r2}");
        }
    }

    /// Несколько перелётов дают очки в младших байтах; звание при этом
    /// остаётся безобидным, пока `PilotRating1` не перешагнёт порог.
    #[test]
    fn a_few_flights_do_not_make_pilot_dangerous() {
        assert_eq!(score(&[0xC8, 0x00, 0x00]), 200);
        assert_eq!(text(0xC8, 0x00, 0x00), "Harmless");
        assert_eq!(text(0xFF, 0x07, 0x00), "Harmless");
    }
}
