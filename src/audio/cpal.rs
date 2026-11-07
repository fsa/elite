//! Кросс-платформенный бэкенд на cpal (фича `audio-cpal`).
//!
//! Linux — ALSA (в системах с PipeWire обычно маршрутизируется через
//! pipewire-alsa), macOS — CoreAudio, Windows — WASAPI. Частота устройства
//! может отличаться от [`SAMPLE_RATE`], поэтому очередь 44100 Гц прореживается
//! или растягивается прямо в колбэке простым удержанием сэмпла.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, I24, Sample, SampleFormat, SizedSample, StreamConfig};

use super::{SAMPLE_RATE, Shared};

/// Запустить cpal-поток и дождаться результата инициализации.
pub(super) fn start(shared: Arc<Mutex<Shared>>) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("audio-cpal".into())
        .spawn(move || {
            let ready = tx.clone();
            if let Err(e) = run(shared, ready) {
                let _ = tx.send(Err(e));
            }
        })
        .map_err(|e| e.to_string())?;

    rx.recv()
        .unwrap_or_else(|e| Err(format!("поток cpal завершился: {e}")))
}

fn run(shared: Arc<Mutex<Shared>>, ready: mpsc::Sender<Result<(), String>>) -> Result<(), String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| "нет устройства вывода по умолчанию".to_string())?;
    let supported = device.default_output_config().map_err(|e| e.to_string())?;
    let sample_format = supported.sample_format();
    let config = supported.config();

    match sample_format {
        SampleFormat::I8 => build::<i8>(&device, config, shared, ready),
        SampleFormat::I16 => build::<i16>(&device, config, shared, ready),
        SampleFormat::I24 => build::<I24>(&device, config, shared, ready),
        SampleFormat::I32 => build::<i32>(&device, config, shared, ready),
        SampleFormat::I64 => build::<i64>(&device, config, shared, ready),
        SampleFormat::U8 => build::<u8>(&device, config, shared, ready),
        SampleFormat::U16 => build::<u16>(&device, config, shared, ready),
        SampleFormat::U32 => build::<u32>(&device, config, shared, ready),
        SampleFormat::U64 => build::<u64>(&device, config, shared, ready),
        SampleFormat::F32 => build::<f32>(&device, config, shared, ready),
        SampleFormat::F64 => build::<f64>(&device, config, shared, ready),
        other => Err(format!("неподдерживаемый формат сэмплов: {other}")),
    }
}

fn build<T>(
    device: &cpal::Device,
    config: StreamConfig,
    shared: Arc<Mutex<Shared>>,
    ready: mpsc::Sender<Result<(), String>>,
) -> Result<(), String>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    if channels == 0 {
        return Err("устройство вернуло 0 каналов".to_string());
    }
    let rate = config.sample_rate;
    if rate == 0 {
        return Err("устройство вернуло частоту 0 Гц".to_string());
    }

    // Сколько сэмплов очереди (44100 Гц) приходится на один сэмпл устройства.
    let step = SAMPLE_RATE as f64 / rate as f64;
    let mut acc = 0.0f64;

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut shared = shared.lock().unwrap();
                shared.callbacks += 1;
                shared.frames_total += (data.len() / channels) as u64;
                for frame in data.chunks_mut(channels) {
                    acc += step;
                    while acc >= 1.0 {
                        acc -= 1.0;
                        match shared.samples.pop_front() {
                            Some(sample) => shared.last = sample,
                            None => shared.underruns += 1,
                        }
                    }
                    let value = shared.last as f32 / 32768.0;
                    let sample = <T as Sample>::from_sample(value);
                    for slot in frame.iter_mut() {
                        *slot = sample;
                    }
                }
            },
            |err| eprintln!("audio: cpal stream error: {err}"),
            None,
        )
        .map_err(|e| e.to_string())?;

    stream.play().map_err(|e| e.to_string())?;
    let _ = ready.send(Ok(()));

    // `Stream` нельзя перемещать между потоками на некоторых платформах,
    // поэтому держим его здесь и просто не даём потоку завершиться.
    loop {
        std::thread::park();
    }
}
