//! Аудиовыход: PipeWire (Linux) с запасным вариантом cpal.
//!
//! Игра использует только 1-битный бипер (порт 0xFE, бит 4). Кадры эмулятора
//! кладут готовые моно-сэмплы в очередь методом [`Audio::push`]; фоновый поток
//! выбранного бэкенда забирает их и отдаёт в звуковую подсистему.
//!
//! Число сэмплов, попадающих в очередь, задаётся не частотой кадров, а
//! настенными часами: накопитель `acc` прибавляет `dt * 44100` и отдаёт целую
//! часть. Так вывод не «уплывает» относительно звуковой карты, а небольшой запас
//! (`CUSHION`) не даёт колбэкам забирать данные из пустой очереди.
//!
//! Бэкенды:
//! - `audio-pipewire` (Linux, включён по умолчанию) — нативный вывод через
//!   PipeWire, если он есть в системе;
//! - `audio-cpal` (включён по умолчанию, единственный на macOS) — ALSA на
//!   Linux, CoreAudio на macOS, WASAPI на Windows.
//!
//! Если PipeWire собран, он пробуется первым; при ошибке инициализации
//! (например, демон не запущен) `start_output` автоматически переходит на cpal.
//! Мьют и `--wav` от выбора бэкенда не зависят.

use std::collections::VecDeque;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[cfg(all(target_os = "linux", feature = "audio-pipewire"))]
mod pipewire;

#[cfg(feature = "audio-cpal")]
mod cpal;

pub const SAMPLE_RATE: u32 = 44100;
/// Запас (~50 мс), который держим в очереди, чтобы колбэки не голодали.
const CUSHION: usize = SAMPLE_RATE as usize / 20;
/// Верхняя граница очереди (~500 мс) — защита от зависаний вывода.
const MAX_QUEUED: usize = SAMPLE_RATE as usize / 2;

struct Shared {
    samples: VecDeque<i16>,
    /// Последний отданный сэмпл — им добиваем недобор и держим при паузах.
    last: i16,
    /// Накопитель дробной части числа сэмплов, привязанный к часам.
    acc: f64,
    last_push: Option<Instant>,
    underruns: u64,
    dropped: u64,
    callbacks: u64,
    frames_total: u64,
}

impl Shared {
    fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(MAX_QUEUED),
            last: 0,
            acc: 0.0,
            last_push: None,
            underruns: 0,
            dropped: 0,
            callbacks: 0,
            frames_total: 0,
        }
    }
}

/// Аудиовыход. Держим в `main`, пока работает игра.
pub struct Audio {
    shared: Arc<Mutex<Shared>>,
}

/// Запустить вывод звука, если он не отключён. Возвращает `None` (с
/// предупреждением), если ни один аудиобэкенд не инициализировался.
pub fn start_output(mute: bool) -> Option<Audio> {
    if mute {
        return None;
    }

    let shared = Arc::new(Mutex::new(Shared::new()));

    #[cfg(all(target_os = "linux", feature = "audio-pipewire"))]
    {
        match pipewire::start(Arc::clone(&shared)) {
            Ok(()) => return Some(Audio { shared }),
            Err(e) => eprintln!("audio: PipeWire недоступен, пробую cpal: {e}"),
        }
    }

    #[cfg(feature = "audio-cpal")]
    {
        match cpal::start(Arc::clone(&shared)) {
            Ok(()) => return Some(Audio { shared }),
            Err(e) => eprintln!("audio: cpal: {e}"),
        }
    }

    #[cfg(not(any(
        all(target_os = "linux", feature = "audio-pipewire"),
        feature = "audio-cpal"
    )))]
    {
        let _ = &shared;
        eprintln!("audio: сборка без аудиобэкенда (нет фич audio-pipewire/audio-cpal)");
    }

    None
}

/// Write mono `samples` (at `rate` Hz) as a 16-bit PCM WAV file.
pub fn write_wav(path: &str, samples: &[i16], rate: u32) {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + samples.len() * 2);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 2).to_le_bytes()); // byte rate
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    if let Err(e) = fs::write(path, &out) {
        eprintln!("cannot write wav {path}: {e}");
    } else {
        println!("wrote {path}");
    }
}

impl Audio {
    /// Положить моно-сэмплы (44100 Гц, i16) в очередь воспроизведения.
    pub fn push(&self, samples: &[i16]) {
        let now = Instant::now();
        let mut shared = self.shared.lock().unwrap();
        match shared.last_push {
            Some(prev) => {
                let dt = now.duration_since(prev).as_secs_f64();
                shared.acc = (shared.acc + dt * SAMPLE_RATE as f64).min(MAX_QUEUED as f64);
            }
            None => shared.acc = CUSHION as f64,
        }
        shared.last_push = Some(now);

        if shared.acc < 1.0 {
            return;
        }
        let need = shared.acc.floor() as usize;
        shared.acc -= need as f64;
        for i in 0..need {
            let s = samples.get(i).copied().unwrap_or(shared.last);
            if shared.samples.len() >= MAX_QUEUED {
                shared.samples.pop_front();
                shared.dropped += 1;
            }
            shared.samples.push_back(s);
            shared.last = s;
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        if std::env::var_os("ELITE_AUDIO_STATS").is_none() {
            return;
        }
        let shared = self.shared.lock().unwrap();
        eprintln!(
            "audio: underruns={} dropped={} queued={} callbacks={} frames_total={}",
            shared.underruns,
            shared.dropped,
            shared.samples.len(),
            shared.callbacks,
            shared.frames_total
        );
    }
}
