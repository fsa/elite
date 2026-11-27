//! Воспроизведение звуковых эффектов игры.
//!
//! Биперные эффекты описываются компактными последовательностями фронтов.
//! Звуки General Sound — это настоящие 8-битные сэмплы, извлечённые из
//! `Elite3GS.trd` в [`crate::gs_samples`]: команда `0x39` из порта `0xBB`
//! приходит вместе с номером сэмпла из защёлки `0xB3`, и этот номер напрямую
//! выбирает сэмпл. Готовые `i16`-сэмплы смешиваются в общий поток вывода
//! (PipeWire / cpal) через [`crate::audio::Audio`].

use crate::audio::SAMPLE_RATE;
use crate::gs_samples;

/// Тактовая частота процессора ZX Spectrum (Гц). Периоды бипера в оригинале
/// задаются в тактах Z80.
const CPU_HZ: u32 = 3_500_000;

/// Амплитуда, как у бипера эмулятора (см. `machine::audio`).
const AMP: i16 = 4000;

/// Громкость 8-битного сэмпла GS: `(sample - 128) * GS_GAIN`.
const GS_GAIN: i16 = 32;

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
    /// Команда General Sound: номер сэмпла из защёлки порта `0xB3`, переданный
    /// вместе с командой `0x39`. Номер 1..23 выбирает сэмпл из `Elite3GS.trd`;
    /// неизвестный номер даёт короткий гудок.
    Gs(u8),
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

/// Резервный гудок для неизвестного номера GS.
const GS_FALLBACK: [Tone; 1] = [Tone {
    half_period_tstates: 1600,
    edges: 40,
    start_high: true,
}];

impl Sfx {
    fn seq(self) -> ToneSeq {
        match self {
            Sfx::DockedBeep => ToneSeq {
                tones: &DOCKED_BEEP,
            },
            Sfx::PauseKey => ToneSeq {
                tones: &PAUSE_KEY,
            },
            Sfx::Gs(_) => ToneSeq {
                tones: &GS_FALLBACK,
            },
        }
    }
}

/// Проигрыватель звуковых эффектов.
///
/// Хранит текущий воспроизводимый звук и умеет подмешивать его в буфер
/// сэмплов текущего кадра. Повтор той же команды GS, пока сэмпл играет,
/// игнорируется: так непрерывно посылаемые игрой звуки не перезапускаются
/// каждый кадр, а звучат до конца.
#[derive(Default)]
pub struct SfxPlayer {
    playing: Option<Playing>,
}

/// Активный звук: либо бипер, либо сэмпл General Sound.
#[derive(Clone, Copy, Debug)]
enum Playing {
    Tone(TonePlaying),
    Sample(SamplePlaying),
}

/// Состояние проигрываемого тона с масштабированием времени в фиксированной точке.
#[derive(Clone, Copy, Debug)]
struct TonePlaying {
    tones: &'static [Tone],
    tone_idx: usize,
    edge_index: u32,
    /// Накопитель: сколько "такто-сэмплов" прошло в текущем полупериоде.
    accum: u64,
}

/// Состояние проигрываемого сэмпла General Sound.
#[derive(Clone, Copy, Debug)]
struct SamplePlaying {
    id: u8,
    data: &'static [u8],
    /// Позиция в сэмпле (в его родной частоте `GS_SAMPLE_RATE`).
    pos: usize,
    /// Накопитель пересчёта `GS_SAMPLE_RATE` в `SAMPLE_RATE`.
    accum: u32,
}

impl TonePlaying {
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
        let Some(tone) = self.tone() else {
            return false;
        };
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

impl SamplePlaying {
    fn new(id: u8, data: &'static [u8]) -> Self {
        Self {
            id,
            data,
            pos: 0,
            accum: 0,
        }
    }

    fn finished(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn render(&mut self, out: &mut [i16]) {
        for sample in out.iter_mut() {
            if self.finished() {
                break;
            }
            let raw = self.data[self.pos] as i16;
            *sample = sample.saturating_add((raw - 128).saturating_mul(GS_GAIN));

            self.accum += gs_samples::GS_SAMPLE_RATE;
            while self.accum >= SAMPLE_RATE && !self.finished() {
                self.accum -= SAMPLE_RATE;
                self.pos += 1;
            }
        }
    }
}

impl Playing {
    fn gs_id(&self) -> Option<u8> {
        match self {
            Playing::Sample(p) => Some(p.id),
            Playing::Tone(_) => None,
        }
    }

    fn finished(&self) -> bool {
        match self {
            Playing::Tone(p) => p.finished(),
            Playing::Sample(p) => p.finished(),
        }
    }

    fn render(&mut self, out: &mut [i16]) {
        match self {
            Playing::Tone(p) => p.render(out),
            Playing::Sample(p) => p.render(out),
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
        if let Sfx::Gs(id) = sfx {
            if self.playing.as_ref().and_then(Playing::gs_id) == Some(id) {
                return;
            }
            if let Some(data) = gs_samples::sample(id) {
                self.playing = Some(Playing::Sample(SamplePlaying::new(id, data)));
                return;
            }
        }
        self.playing = Some(Playing::Tone(TonePlaying::new(sfx.seq())));
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
    fn all_gs_samples_are_available() {
        let count = gs_samples::GS_SAMPLE_OFFSETS.len() - 1;
        assert_eq!(count, 23);
        for id in 1..=count as u8 {
            let data = gs_samples::sample(id).expect("сэмпл GS должен быть в образе");
            assert!(!data.is_empty(), "пустой сэмпл GS-{id}");
        }
        // Номер из защёлки 1 — это первая запись каталога, а не вторая.
        assert_eq!(gs_samples::sample(1).unwrap().len(), 1024);
        assert_eq!(gs_samples::sample(13).unwrap().len(), 768);
        assert!(gs_samples::sample(0).is_none());
        assert!(gs_samples::sample(count as u8 + 1).is_none());
    }

    #[test]
    fn gs_command_plays_real_sample() {
        let mut player = SfxPlayer::new();
        // Звук 13 — самый короткий сэмпл (3 сектора = 768 байт).
        player.play(Sfx::Gs(13));
        let mut buf = vec![0i16; 4096];
        player.mix(&mut buf);
        assert!(
            buf.iter().any(|&s| s != 0),
            "сэмпл GS-12 не дал звука"
        );
        assert!(!player.is_playing(), "короткий сэмпл GS должен закончиться");
        // Сэмпл — не прямоугольник: между соседними значениями есть перепады.
        assert!(
            buf.windows(2).any(|w| w[0] != w[1]),
            "сэмпл GS должен быть немодулированным сигналом"
        );
    }

    #[test]
    fn gs_sample_survives_several_frames() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::Gs(14));
        let mut frame = vec![0i16; 882]; // 20 мс кадра
        player.mix(&mut frame);
        assert!(player.is_playing(), "длинный сэмпл должен идти несколько кадров");
        let mut total = 0;
        for _ in 0..500 {
            frame.fill(0);
            player.mix(&mut frame);
            total += frame.iter().filter(|&&s| s != 0).count();
            if !player.is_playing() {
                break;
            }
        }
        assert!(!player.is_playing(), "сэмпл GS-14 должен закончиться");
        assert!(total > 1000, "сэмпл GS-14 слишком короткий: {total}");
    }

    #[test]
    fn repeated_gs_command_does_not_restart_sample() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::Gs(14));
        let mut buf = vec![0i16; 882];
        player.mix(&mut buf);
        player.play(Sfx::Gs(14));
        let mut after = vec![0i16; 882];
        player.mix(&mut after);
        assert!(
            after.iter().any(|&s| s != 0),
            "повтор команды не должен обрывать звук"
        );
    }

    #[test]
    fn unknown_gs_id_uses_fallback() {
        let mut player = SfxPlayer::new();
        player.play(Sfx::Gs(99));
        let mut buf = vec![0i16; 2048];
        player.mix(&mut buf);
        assert!(buf.iter().any(|&s| s != 0), "резервный гудок не звучит");
        assert!(!player.is_playing());
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
