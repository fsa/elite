//! Headless-режим: прогон кадров без окна, отладочные дампы и запись WAV.

use std::env;
use std::fs;

use crate::audio;
use crate::cli::Options;
use crate::machine::Spectrum;
use crate::native;
use crate::storage;
use crate::world;

pub fn run(machine: &mut Spectrum, opts: &Options) {
    let trace = env::var("ELITE_TRACE").is_ok();
    let sp_from: Option<u64> = env::var("ELITE_SP").ok().and_then(|s| s.parse().ok());
    // Key injection for tests: ELITE_KEYS="row:bit:down:up[,row:bit:down:up...]"
    // presses keyboard (row,bit) from frame `down` (inclusive) to `up` (exclusive).
    let keys_specs: Vec<(u64, u64, usize, u8)> = env::var("ELITE_KEYS")
        .ok()
        .map(|s| {
            s.split(',')
                .filter_map(|seg| {
                    let n: Vec<u64> = seg.split(':').filter_map(|x| x.parse().ok()).collect();
                    if n.len() == 4 {
                        // row:bit:down:up
                        Some((n[2], n[3], n[0] as usize, n[1] as u8))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    // ELITE_JOY="bit:down:up[,bit:down:up...]" — нажатие бита Kempston (0..4)
    // с кадра `down` (включительно) до `up` (исключительно).
    let joy_specs: Vec<(u64, u64, u8)> = env::var("ELITE_JOY")
        .ok()
        .map(|s| {
            s.split(',')
                .filter_map(|seg| {
                    let n: Vec<u64> = seg.split(':').filter_map(|x| x.parse().ok()).collect();
                    if n.len() == 3 {
                        Some((n[1], n[2], n[0] as u8))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    let mut wav_samples: Option<Vec<i16>> = opts.wav.as_ref().map(|_| Vec::new());
    let audio = if opts.audio {
        audio::start_output(opts.mute)
    } else {
        None
    };
    let frame_time = std::time::Duration::from_micros(1_000_000 / 50);
    for f in 0..opts.frames {
        let frame_started = std::time::Instant::now();
        machine.release_all_keys();
        machine.release_joystick();
        for &(down, up, row, bit) in &keys_specs {
            if f >= down && f < up {
                machine.set_key(row, bit, true);
            }
        }
        for &(down, up, bit) in &joy_specs {
            if f >= down && f < up {
                machine.set_kempston(bit, true);
            }
        }
        if trace && f < 120 {
            println!(
                "frame {:3} pc=0x{:04X} sp=0x{:04X} presskey=0x{:02X}",
                f, machine.cpu.pc, machine.cpu.sp, machine.mem[0xFC13]
            );
        }
        if env::var("ELITE_KEYDBG").is_ok() {
            println!(
                "frame {:3} kb=[{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}] presskey=0x{:02X}",
                f,
                machine.keyboard[0],
                machine.keyboard[1],
                machine.keyboard[2],
                machine.keyboard[3],
                machine.keyboard[4],
                machine.keyboard[5],
                machine.keyboard[6],
                machine.keyboard[7],
                machine.mem[0xFC13]
            );
        }
        if let Some(from) = sp_from {
            if f >= from {
                println!(
                    "frame {:3} pc=0x{:04X} sp=0x{:04X} ix=0x{:04X} iy=0x{:04X}",
                    f, machine.cpu.pc, machine.cpu.sp, machine.cpu.ix, machine.cpu.iy
                );
            }
        }
        machine.run_frame();
        if machine.save_requested {
            machine.save_requested = false;
            let path = opts.save.clone().unwrap_or_else(storage::default_save_name);
            match native::commander_save(&mut *machine, &path, world::save_layout_kladov()) {
                Ok(()) => println!("Сохранено в {}", path),
                Err(e) => eprintln!("Ошибка сохранения: {e}"),
            }
        }
        if audio.is_some() || wav_samples.is_some() {
            let mut frame = Vec::new();
            machine.render_audio(&mut frame);
            if let Some(audio) = &audio {
                audio.push(&frame);
            }
            if let Some(wav) = &mut wav_samples {
                wav.extend_from_slice(&frame);
            }
        }
        if audio.is_some() {
            let elapsed = frame_started.elapsed();
            if frame_time > elapsed {
                std::thread::sleep(frame_time - elapsed);
            }
        }
    }
    println!(
        "frames={} pc=0x{:04X} cycles={} sp=0x{:04X}",
        machine.frames, machine.cpu.pc, machine.cpu.cycles, machine.cpu.sp
    );
    println!(
        "paging(0x7FFD) writes={} last=0x{:02X}",
        machine.paging_writes, machine.paging_last
    );
    for (a, n) in &machine.counts {
        println!("  count pc=0x{:04X} = {}", a, n);
    }
    if !machine.profile.is_empty() {
        let mut top: Vec<(u32, usize)> = machine
            .profile
            .iter()
            .enumerate()
            .filter(|&(_, &n)| n != 0)
            .map(|(a, &n)| (n, a))
            .collect();
        top.sort_unstable_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
        println!(
            "profile: топ-{} адресов по числу исполненных инструкций:",
            machine.profile_top
        );
        for (n, a) in top.iter().take(machine.profile_top) {
            println!("  0x{a:04X} {n}");
        }
    }
    for (a, n) in &machine.wwatch_hist {
        println!("  write-from pc=0x{:04X} = {}", a, n);
    }
    // Debug histogram of the bitmap and attribute regions.
    let bitmap = &machine.mem[0x4000..0x5800];
    let ones: usize = bitmap.iter().map(|b| b.count_ones() as usize).sum();
    println!(
        "bitmap: {} bytes, {} bits set, {:.1}% ink; attr[0x5800]=0x{:02X} attr[0x5A00]=0x{:02X} buf[0xC000]=0x{:02X} buf[0xC800]=0x{:02X}",
        bitmap.len(),
        ones,
        100.0 * ones as f64 / (bitmap.len() as f64 * 8.0),
        machine.mem[0x5800],
        machine.mem[0x5A00],
        machine.mem[0xC000],
        machine.mem[0xC800],
    );
    if let Some(path) = &opts.dump {
        let mut buf = vec![0u32; 256 * 192];
        machine.render_rgba(&mut buf);
        write_ppm(path, &buf, 256, 192);
        println!("wrote {}", path);
    }
    if let Ok(dir) = env::var("ELITE_MEM") {
        let dir = if dir.is_empty() {
            "tmp".to_string()
        } else {
            dir
        };
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(format!("{dir}/screen.bin"), &machine.mem[0x4000..0x5B00]);
        let _ = fs::write(format!("{dir}/buf.bin"), &machine.mem[0xC000..0xD000]);
        let _ = fs::write(format!("{dir}/buf800.bin"), &machine.mem[0xC800..0xD800]);
        let _ = fs::write(
            format!("{dir}/font.bin"),
            &machine.mem[0x5D70..0x5D70 + 0x2D8],
        );
        let _ = fs::write(format!("{dir}/mem.bin"), &machine.mem);
        println!("wrote {dir}/screen.bin buf.bin buf800.bin font.bin mem.bin");
    }
    if let (Some(path), Some(samples)) = (&opts.wav, &wav_samples) {
        audio::write_wav(path, samples, audio::SAMPLE_RATE);
    }
}

/// Записать кадр `buf` (пиксели 0x00RRGGBB) в PPM-файл.
pub(crate) fn write_ppm(path: &str, buf: &[u32], w: usize, h: usize) {
    let mut out = Vec::with_capacity(w * h * 3 + 32);
    out.extend_from_slice(format!("P6\n{} {}\n255\n", w, h).as_bytes());
    for &px in buf {
        out.push(((px >> 16) & 0xFF) as u8);
        out.push(((px >> 8) & 0xFF) as u8);
        out.push((px & 0xFF) as u8);
    }
    let _ = fs::write(path, out);
}
