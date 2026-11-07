//! PipeWire-бэкенд (Linux, фича `audio-pipewire`).

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use pipewire as pw;
use pw::properties::properties;

use super::{SAMPLE_RATE, Shared};

/// Запустить PipeWire-поток и дождаться результата инициализации.
///
/// Возврат `Ok` означает, что поток уже создан и `stream.connect` прошёл;
/// ошибка позволяет `start_output` переключиться на cpal.
pub(super) fn start(shared: Arc<Mutex<Shared>>) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("audio-pipewire".into())
        .spawn(move || {
            let ready = tx.clone();
            if let Err(e) = run(shared, ready) {
                let _ = tx.send(Err(e));
            }
        })
        .map_err(|e| e.to_string())?;

    rx.recv()
        .unwrap_or_else(|e| Err(format!("поток PipeWire завершился: {e}")))
}

fn run(shared: Arc<Mutex<Shared>>, ready: mpsc::Sender<Result<(), String>>) -> Result<(), String> {
    pw::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None).map_err(|e| e.to_string())?;
    let context = pw::context::ContextRc::new(&mainloop, None).map_err(|e| e.to_string())?;
    let core = context.connect_rc(None).map_err(|e| e.to_string())?;

    let stream = pw::stream::StreamBox::new(
        &core,
        "elite",
        properties! {
            *pw::keys::MEDIA_TYPE => "Audio",
            *pw::keys::MEDIA_ROLE => "Game",
            *pw::keys::MEDIA_CATEGORY => "Playback",
            *pw::keys::AUDIO_CHANNELS => "2",
        },
    )
    .map_err(|e| e.to_string())?;

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
        .register()
        .map_err(|e| e.to_string())?;

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
    )
    .map_err(|e| e.to_string())?
    .0
    .into_inner();
    let mut params = [pw::spa::pod::Pod::from_bytes(&values)
        .ok_or_else(|| "invalid audio format pod".to_string())?];

    stream
        .connect(
            pw::spa::utils::Direction::Output,
            None,
            pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
            &mut params,
        )
        .map_err(|e| e.to_string())?;

    if ready.send(Ok(())).is_err() {
        return Ok(());
    }

    mainloop.run();
    Ok(())
}
