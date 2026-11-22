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
    /// Короткий отклик на клавишу паузы Кладова. Звук нативный: замороженный
    /// Z80 не может подтвердить нажатие своим бипером.
    PauseKey,
}

/// Описание звука: последовательность тонов бипера.
#[derive(Clone, Copy, Debug)]
struct ToneSeq {
    tones: &'static [Tone],
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

const DOCKED_BEEP: [Tone; 1] = [Tone {
    half_period_tstates: 630,
    edges: 194,
    start_high: true,
}];

/// ~15 мс тона 875 Гц — короткий, но отчётливый отклик на нажатие.
const PAUSE_KEY: [Tone; 1] = [Tone {
    half_period_tstates: 2000,
    edges: 26,
    start_high: true,
}];

impl Sfx {
    fn seq(self) -> ToneSeq {
        match self {
            Sfx::DockedBeep => ToneSeq { tones: &DOCKED_BEEP },
            Sfx::PauseKey => ToneSeq { tones: &PAUSE_KEY },
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
    tones: &'static [Tone],
    tone_idx: usize,
    edge_index: u32,
    /// Накопитель: сколько "такто-сэмплов" прошло в текущем полупериоде.
    accum: u64,
}

impl Playing {
    fn new(seq: ToneSeq) -> Self {
        Self {
            tones: seq.tones,
            tone_idx: 0,
            edge_index: 0,
            accum: 0,
        }
    }

    fn tone(&self) -> Option<Tone> {
        self.tones.get(self.tone_idx).copied()
    }

    fn level(&self) -> bool {
        let Some(tone) = self.tone() else { return false };
        if self.edge_index.is_multiple_of(2) {
            tone.start_high
        } else {
            !tone.start_high
        }
    }

    fn finished(&self) -> bool {
        self.tone_idx >= self.tones.len()
    }

    fn render(&mut self, out: &mut [i16]) {
        for sample in out.iter_mut() {
            let Some(tone) = self.tone() else {
                break;
            };
            if tone.half_period_tstates == 0 || tone.edges == 0 {
                self.tone_idx += 1;
                self.edge_index = 0;
                self.accum = 0;
                continue;
            }
            let amp = if self.level() { AMP } else { -AMP };
            *sample = sample.saturating_add(amp);

            let edge_len = tone.half_period_tstates as u64 * SAMPLE_RATE as u64;
            self.accum += CPU_HZ as u64;
            while self.accum >= edge_len && !self.finished() {
                self.accum -= edge_len;
                self.edge_index += 1;
                if let Some(tone) = self.tone() {
                    if self.edge_index >= tone.edges {
                        self.tone_idx += 1;
                        self.edge_index = 0;
                    }
                }
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
        self.playing = Some(Playing::new(sfx.seq()));
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
    fn pause_key_is_short_and_audible() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::PauseKey);
        let mut buf = vec![0i16; 2048];
        player.mix(&mut buf);
        assert!(!player.is_playing(), "отклик паузы должен быть коротким");
        assert!(buf.iter().any(|&s| s > 0), "нет положительных сэмплов");
        assert!(buf.iter().any(|&s| s < 0), "нет отрицательных сэмплов");
        let nonzero = buf.iter().filter(|&&s| s != 0).count();
        assert!(
            (500..=800).contains(&nonzero),
            "неожиданная длительность отклика: {nonzero} сэмплов"
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
