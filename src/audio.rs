//! Аудиовыход через PipeWire.
//!
//! Игра использует только 1-битный бипер (порт 0xFE, бит 4). Кадры эмулятора
//! кладут готовые моно-сэмплы в очередь методом [`Audio::push`]; фоновый поток
//! с потоком PipeWire забирает их и отдаёт в звуковую подсистему.
//!
//! Число сэмплов, попадающих в очередь, задаётся не частотой кадров, а
//! настенными часами: накопитель `acc` прибавляет `dt * 44100` и отдаёт целую
//! часть. Так вывод не «уплывает» относительно звуковой карты, а небольшой запас
//! (`CUSHION`) не даёт колбэкам PipeWire забирать данные из пустой очереди.

use std::collections::VecDeque;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use pipewire as pw;
use pw::properties::properties;

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

/// Аудиовыход. Держим в `main`, пока работает игра.
pub struct Audio {
    shared: Arc<Mutex<Shared>>,
}

/// Запустить вывод звука, если он не отключён. Возвращает `None` (с
/// предупреждением), если аудиобэкенд не инициализировался.
pub fn start_output(mute: bool) -> Option<Audio> {
    if mute {
        return None;
    }
    match Audio::start() {
        Ok(audio) => Some(audio),
        Err(e) => {
            eprintln!("audio: cannot start PipeWire output: {e}");
            None
        }
    }
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
    pub fn start() -> Result<Audio, std::io::Error> {
        let shared = Arc::new(Mutex::new(Shared {
            samples: VecDeque::with_capacity(MAX_QUEUED),
            last: 0,
            acc: 0.0,
            last_push: None,
            underruns: 0,
            dropped: 0,
            callbacks: 0,
            frames_total: 0,
        }));
        let thread_shared = Arc::clone(&shared);
        std::thread::Builder::new()
            .name("audio".into())
            .spawn(move || {
                if let Err(e) = run_stream(thread_shared) {
                    eprintln!("audio: PipeWire error: {e}");
                }
            })?;
        Ok(Audio { shared })
    }

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

fn run_stream(shared: Arc<Mutex<Shared>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pw::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_rc(None)?;

    let stream = pw::stream::StreamBox::new(
        &core,
        "elite",
        properties! {
            *pw::keys::MEDIA_TYPE => "Audio",
            *pw::keys::MEDIA_ROLE => "Game",
            *pw::keys::MEDIA_CATEGORY => "Playback",
            *pw::keys::AUDIO_CHANNELS => "2",
        },
    )?;

    let _listener = stream
        .add_local_listener_with_user_data(shared)
        .process(|stream, shared| {
            let Some(mut buffer) = stream.dequeue_buffer() else {
                return;
            };
            let requested = buffer.requested() as usize;
            let data = &mut buffer.datas_mut()[0];
            let stride = 4;
            let max_frames = data.data().map(|s| s.len() / stride).unwrap_or(0);
            let frames = if requested > 0 && requested <= max_frames {
                requested
            } else {
                max_frames
            };
            let mut shared = shared.lock().unwrap();
            shared.callbacks += 1;
            shared.frames_total += frames as u64;
            if let Some(slice) = data.data() {
                for i in 0..frames {
                    let s = match shared.samples.pop_front() {
                        Some(s) => {
                            shared.last = s;
                            s
                        }
                        None => {
                            shared.underruns += 1;
                            shared.last
                        }
                    };
                    let bytes = s.to_le_bytes();
                    let off = i * stride;
                    slice[off..off + 2].copy_from_slice(&bytes);
                    slice[off + 2..off + 4].copy_from_slice(&bytes);
                }
            }
            let chunk = data.chunk_mut();
            *chunk.offset_mut() = 0;
            *chunk.stride_mut() = stride as i32;
            *chunk.size_mut() = (frames * stride) as u32;
        })
        .register()?;

    let mut audio_info = pw::spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(pw::spa::param::audio::AudioFormat::S16LE);
    audio_info.set_rate(SAMPLE_RATE);
    audio_info.set_channels(2);
    let mut position = [0; pw::spa::param::audio::MAX_CHANNELS];
    position[0] = pw::spa::sys::SPA_AUDIO_CHANNEL_FL;
    position[1] = pw::spa::sys::SPA_AUDIO_CHANNEL_FR;
    audio_info.set_position(position);
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(pw::spa::pod::Object {
            type_: pw::spa::sys::SPA_TYPE_OBJECT_Format,
            id: pw::spa::sys::SPA_PARAM_EnumFormat,
            properties: audio_info.into(),
        }),
    )?
    .0
    .into_inner();
    let mut params = [pw::spa::pod::Pod::from_bytes(&values).ok_or("invalid audio format pod")?];

    stream.connect(
        pw::spa::utils::Direction::Output,
        None,
        pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
        &mut params,
    )?;

    mainloop.run();
    Ok(())
}
