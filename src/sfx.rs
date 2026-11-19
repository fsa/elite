//! Воспроизведение звуковых эффектов игры.
//!
//! Компонент хранит компактные определения звуков (параметры оригинального
//! бипера ZX Spectrum) и синтезирует из них моно-сэмплы `i16`, которые
//! смешиваются в поток вывода (PipeWire / cpal) через [`crate::audio::Audio`].
//!
//! Формат определения рассчитан на воспроизведение в коде: звук описывается
//! несколькими числами, а не записанными сэмплами.

use crate::audio::SAMPLE_RATE;

/// Тактовая частота процессора ZX Spectrum (Гц). Периоды бипера в оригинале
/// задаются в тактах Z80.
const CPU_HZ: u32 = 3_500_000;

/// Амплитуда, как у бипера эмулятора (см. `machine::audio`).
const AMP: i16 = 4000;

/// Звуковой эффект игры.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfx {
    /// Короткий гудок на станции при нажатии `G` или `H`: экран `DOCKED`.
    ///
    /// Соответствует вызову ROM BEEPER (`0x03B5`) с `HL=0x0080`, `DE=0x0060`
    /// (подпрограмма `sub_E1E8` оригинала). 194 фронта, полупериод 630 тактов.
    DockedBeep,
}

/// Компактное описание прямоугольного сигнала бипера.
#[derive(Clone, Copy, Debug)]
struct Tone {
    /// Длительность полупериода в тактах Z80.
    half_period_tstates: u32,
    /// Число полупериодов (фронтов/переключений).
    edges: u32,
    /// Начальный уровень: `true` — высокий.
    start_high: bool,
}

impl Sfx {
    fn tone(self) -> Tone {
        match self {
            // Параметры сняты с ROM BEEPER 48K (0x03B5) для HL=0x0080, DE=0x0060.
            Sfx::DockedBeep => Tone {
                half_period_tstates: 630,
                edges: 194,
                start_high: true,
            },
        }
    }
}

/// Проигрыватель звуковых эффектов.
///
/// Хранит текущий воспроизводимый звук и умеет подмешивать его в буфер
/// сэмплов текущего кадра.
#[derive(Default)]
pub struct SfxPlayer {
    playing: Option<Playing>,
}

/// Состояние проигрываемого звука с масштабированием времени в фиксированной точке.
#[derive(Clone, Copy, Debug)]
struct Playing {
    tone: Tone,
    edge_index: u32,
    /// Накопитель: сколько "такто-сэмплов" прошло в текущем полупериоде.
    /// Порог перехода к следующему полупериоду: `half_period_tstates * SAMPLE_RATE`.
    accum: u64,
}

impl Playing {
    fn new(tone: Tone) -> Self {
        Self {
            tone,
            edge_index: 0,
            accum: 0,
        }
    }

    fn level(&self) -> bool {
        if self.edge_index.is_multiple_of(2) {
            self.tone.start_high
        } else {
            !self.tone.start_high
        }
    }

    fn finished(&self) -> bool {
        self.edge_index >= self.tone.edges
    }

    fn render(&mut self, out: &mut [i16]) {
        if self.finished() || self.tone.half_period_tstates == 0 {
            return;
        }
        let edge_len = self.tone.half_period_tstates as u64 * SAMPLE_RATE as u64;
        for sample in out.iter_mut() {
            if self.finished() {
                break;
            }
            let amp = if self.level() { AMP } else { -AMP };
            *sample = sample.saturating_add(amp);

            self.accum += CPU_HZ as u64;
            while self.accum >= edge_len && !self.finished() {
                self.accum -= edge_len;
                self.edge_index += 1;
            }
        }
    }
}

impl SfxPlayer {
    /// Новый проигрыватель без активных звуков.
    pub fn new() -> Self {
        Self::default()
    }

    /// Начать воспроизведение эффекта, прерывая текущий, если он есть.
    pub fn play(&mut self, sfx: Sfx) {
        self.playing = Some(Playing::new(sfx.tone()));
    }

    /// Подмешать звук в буфер моно-сэмплов (44100 Гц, `i16`).
    ///
    /// Сэмплы эффекта складываются с уже записанными значениями с насыщением.
    /// Если звук закончился, буфер не меняется.
    pub fn mix(&mut self, out: &mut [i16]) {
        if let Some(ref mut p) = self.playing {
            p.render(out);
            if p.finished() {
                self.playing = None;
            }
        }
    }

    /// Идёт ли сейчас воспроизведение.
    pub fn is_playing(&self) -> bool {
        self.playing.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docked_beep_has_expected_duration_and_shape() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::DockedBeep);
        assert!(player.is_playing());

        // ~34.7 мс при 44100 Гц -> ~1530 сэмплов; генерим с запасом.
        let mut buf = vec![0i16; 2000];
        player.mix(&mut buf);

        assert!(!player.is_playing(), "звук должен закончиться");

        // Должны быть и положительные, и отрицательные сэмплы (прямоугольник).
        assert!(buf.iter().any(|&s| s > 0), "нет положительных сэмплов");
        assert!(buf.iter().any(|&s| s < 0), "нет отрицательных сэмплов");

        // После окончания сигнала хвост буфера остаётся нулевым.
        assert!(
            buf[1800..].iter().all(|&s| s == 0),
            "после окончания звука должны быть тишина"
        );

        // Длительность: 194 полупериода * 630 тактов = 122220 тактов = ~1540 сэмплов.
        let nonzero = buf.iter().filter(|&&s| s != 0).count();
        assert!(
            (1400..=1700).contains(&nonzero),
            "неожиданная длительность: {nonzero} ненулевых сэмплов"
        );
    }

    #[test]
    fn mix_adds_to_existing_audio() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::DockedBeep);
        let mut buf = vec![100i16; 256];
        player.mix(&mut buf);
        assert!(buf.iter().any(|&s| s != 100), "звук не подмешан");
    }

    #[test]
    fn silence_when_idle() {
        let mut player = SfxPlayer::new();
        let mut buf = vec![0i16; 256];
        player.mix(&mut buf);
        assert!(buf.iter().all(|&s| s == 0));
    }
}
