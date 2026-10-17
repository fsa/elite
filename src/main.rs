mod audio;
mod machine;
mod native;
mod z80;

use audio::Audio;
use machine::Spectrum;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Load address of the assembled game image.
const GAME_LOAD_ADDR: u16 = 0x6048;
/// Entry point (label `Start`).
const GAME_ENTRY: u16 = 0x7000;
/// Initial stack pointer for the original image.
const GAME_SP: u16 = 0x6000;

/// Версия Кладова (с дискеты `ELITE3HL.TRD`).  Полный 48K-образ памяти
/// (0x4000-0xFFFF) готовит `tools/make_kladov_image.py` из снапшота
/// `zx_src/kladov.snapshot` (в `zx_bin/kladov.bin`).  Файл `ELITE3  C` на
/// дискете — это слепок уже работающей игры, без согласованной точки
/// холодного старта, поэтому мы не проходим загрузчик, а возобновляем
/// исполнение ровно там, где был прерван обработчик прерывания в снапшоте:
/// IM2, `I=0xFD`, обработчик 0xFEFE (таблица векторов уже в образе).
const KLADOV_LOAD_ADDR: u16 = 0x4000;
const KLADOV_I: u8 = 0xFD;
const KLADOV_STATE_PC: u16 = 0xFEFE;
const KLADOV_STATE_SP: u16 = 0x4114;
const KLADOV_STATE_AF: u16 = 0x0045;
const KLADOV_STATE_BC: u16 = 0x0012;
const KLADOV_STATE_DE: u16 = 0x482E;
const KLADOV_STATE_HL: u16 = 0xC80E;
const KLADOV_STATE_IX: u16 = 0x6159;
const KLADOV_STATE_IY: u16 = 0x629B;
const KLADOV_STATE_R: u8 = 0x3E;
const KLADOV_STATE_A: u8 = (KLADOV_STATE_AF >> 8) as u8;
const KLADOV_STATE_F: u8 = (KLADOV_STATE_AF & 0xFF) as u8;

struct Options {
    game: String,
    rom: Option<String>,
    headless: bool,
    frames: u64,
    dump: Option<String>,
    /// Отключить звук (PipeWire).
    mute: bool,
    /// Включить звук и в headless-режиме (реальное время).
    audio: bool,
    /// Сохранить сведённый звук в WAV-файл.
    wav: Option<String>,
    /// Базовая версия проекта — Кладова. `--elite` переключает на оригинал.
    kladov: bool,
    /// Включить нативные (Rust) замены подпрограмм игры вместо Z80.
    native: bool,
    /// Фиксированный путь сохранения командира. `None` — спрашивать через
    /// нативный диалог выбора файла (по умолчанию, расширение `.zxelite`).
    save: Option<String>,
}

fn parse_args() -> Options {
    let mut opts = Options {
        game: String::new(),
        rom: None,
        headless: false,
        frames: 600,
        dump: None,
        mute: false,
        audio: false,
        wav: None,
        kladov: true,
        native: false,
        save: None,
    };
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--kladov" => opts.kladov = true,
            "--elite" => opts.kladov = false,
            "--rom" => opts.rom = args.next(),
            "--frames" => opts.frames = args.next().and_then(|s| s.parse().ok()).unwrap_or(600),
            "--dump" => {
                opts.dump = args.next();
                opts.headless = true;
            }
            "--headless" => opts.headless = true,
            "--mute" => opts.mute = true,
            "--audio" => opts.audio = true,
            "--native" => opts.native = true,
            "--wav" => opts.wav = args.next(),
            "--save" => opts.save = args.next(),
            "-h" | "--help" => {
                println!(
                    "usage: elite [game.bin] [--elite] [--rom rom.bin] [--headless] [--frames N] [--dump out.ppm] [--mute] [--audio] [--wav out.wav] [--native] [--save FILE]"
                );
                println!("  по умолчанию запускается версия Кладова (zx_bin/kladov.bin)");
                println!("  --mute     отключить вывод звука через PipeWire");
                println!("  --audio    выводить звук и в headless-режиме (реальное время)");
                println!("  --wav FILE сохранить звук в WAV-файл");
                println!("  --native   включить нативные (Rust) замены подпрограмм");
                println!("  --save FILE фиксированный файл командира вместо диалога");
                exit(0);
            }
            other => opts.game = other.to_string(),
        }
    }
    if opts.game.is_empty() {
        opts.game = if opts.kladov {
            "zx_bin/kladov.bin".to_string()
        } else {
            "zx_bin/elite.bin".to_string()
        };
    }
    if opts.rom.is_none() {
        for cand in [
            "zx_bin/48.rom",
            "/usr/share/fuse/48.rom",
            "/usr/share/fuse/48.ZX.rom",
        ] {
            if fs::metadata(cand).is_ok() {
                opts.rom = Some(cand.to_string());
                break;
            }
        }
    }
    opts
}

/// The interrupt handler routine (`Interrupts` in the disassembly).
const INTERRUPT_HANDLER: u16 = 0xFF25;
/// Interrupt mode-2 table base; the game sets I=0xFD in `Interrupts_ON`.
const IM2_TABLE: u16 = 0xFD00;

/// The original tape loader patches the IM2 vector table (which ships filled
/// with 0xFE placeholders) so every interrupt vector points at the handler.
/// We reproduce that step here.
fn install_im2_table(machine: &mut Spectrum) {
    let lo = (INTERRUPT_HANDLER & 0xFF) as u8;
    let hi = (INTERRUPT_HANDLER >> 8) as u8;
    for i in 0..128u16 {
        let a = IM2_TABLE + i * 2;
        machine.mem[a as usize] = lo;
        machine.mem[a as usize + 1] = hi;
    }
}

fn main() {
    let opts = parse_args();

    let game = match fs::read(&opts.game) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read game image {:?}: {}", opts.game, e);
            if opts.kladov {
                eprintln!("hint: run python3 tools/make_kladov_image.py to build the Kladov RAM image");
            } else if opts.game == "elite.bin" {
                eprintln!("hint: run ./build.sh to assemble elite.bin from elite.asm");
            }
            exit(1);
        }
    };
    let rom = opts.rom.as_ref().map(|p| match fs::read(p) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read ROM {:?}: {}", p, e);
            exit(1);
        }
    });

    let mut machine = Spectrum::new(rom.as_deref());
    if opts.kladov {
        // Версия Кладова: полный 48K-образ (0x4000..0xFFFF) загружается как есть.
        // Игра настраивает IM2 сама (I=0xFD, обработчик 0xFEFE), таблица векторов
        // и рабочие данные уже находятся в образе. Образ снят с работающей игры
        // (см. tools/make_kladov_image.py), поэтому стартуем не с холодной заставки,
        // а возобновляем выполнение с сохранённого прерванного кадра.
        machine.load(KLADOV_LOAD_ADDR, &game);
        machine.cpu.a = KLADOV_STATE_A;
        machine.cpu.f = KLADOV_STATE_F;
        machine.cpu.b = (KLADOV_STATE_BC >> 8) as u8;
        machine.cpu.c = (KLADOV_STATE_BC & 0xFF) as u8;
        machine.cpu.d = (KLADOV_STATE_DE >> 8) as u8;
        machine.cpu.e = (KLADOV_STATE_DE & 0xFF) as u8;
        machine.cpu.h = (KLADOV_STATE_HL >> 8) as u8;
        machine.cpu.l = (KLADOV_STATE_HL & 0xFF) as u8;
        machine.cpu.ix = KLADOV_STATE_IX;
        machine.cpu.iy = KLADOV_STATE_IY;
        machine.cpu.sp = KLADOV_STATE_SP;
        machine.cpu.pc = KLADOV_STATE_PC;
        machine.cpu.i = KLADOV_I;
        machine.cpu.r = KLADOV_STATE_R;
        machine.cpu.im = 2;
        machine.cpu.iff1 = true;
        machine.cpu.iff2 = true;
    } else {
        machine.load(GAME_LOAD_ADDR, &game);
        install_im2_table(&mut machine);
        machine.cpu.pc = GAME_ENTRY;
        machine.cpu.sp = GAME_SP;

        // Выбор устройства управления. Игра определяет его эвристически при
        // старте, опрашивая порт Kempston (0x1F). По умолчанию включаем Kempston,
        // чтобы курсорные клавиши работали как джойстик: подменяем `in a,(c)`
        // (ED 78) на `ld a,2` (3E 02) в BeginGame, из-за чего выбирается
        // Kempston. `ELITE_CTRL=keyboard` возвращает штатное поведение игры.
        let ctrl = env::var("ELITE_CTRL").unwrap_or_default();
        if ctrl != "keyboard" {
            machine.mem[0x71F8] = 0x3E;
            machine.mem[0x71F9] = 0x02;
        }
    }

    if opts.native || env::var("ELITE_NATIVE").is_ok() {
        native::install(&mut machine);
    }

    // Нативный сейв/лоад командира: перехват `MenuSaveLoad` выставляет флаг,
    // а главный цикл показывает диалог выбора файла (`.zxelite`). Раскладка
    // адресов зависит от версии игры.
    native::install_save(&mut machine, save_layout(&opts));

    if let Ok(w) = env::var("ELITE_WATCH") {
        if let Some((a, b)) = w.split_once('-') {
            let lo = u16::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
            let hi = u16::from_str_radix(b.trim_start_matches("0x"), 16).unwrap_or(0xFFFF);
            machine.watch = Some((lo, hi));
        }
    }

    if let Ok(w) = env::var("ELITE_WWATCH") {
        if let Some((a, b)) = w.split_once('-') {
            let lo = u16::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
            let hi = u16::from_str_radix(b.trim_start_matches("0x"), 16).unwrap_or(0xFFFF);
            machine.wwatch = Some((lo, hi));
        }
    }

    if let Ok(list) = env::var("ELITE_AT") {
        for part in list.split(',') {
            if let Ok(a) = u16::from_str_radix(part.trim().trim_start_matches("0x"), 16) {
                machine.dump_at.push(a);
            }
        }
    }

    if let Ok(list) = env::var("ELITE_SPAT") {
        for part in list.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Ok(a) = u16::from_str_radix(part.trim_start_matches("0x"), 16) {
                machine.spat.insert(a, 0);
            }
        }
    }

    if let Ok(list) = env::var("ELITE_COUNT") {
        for part in list.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Ok(a) = u16::from_str_radix(part.trim_start_matches("0x"), 16) {
                machine.counts.push((a, 0));
            }
        }
    }

    if opts.headless {
        run_headless(&mut machine, &opts);
        return;
    }
    run_window(machine, &opts);
}

/// Раскладка адресов сейва/лоада выбранной версии игры.
fn save_layout(opts: &Options) -> native::SaveLayout {
    if opts.kladov {
        native::KLADOV_SAVE
    } else {
        native::ELITE_SAVE
    }
}

/// Start the PipeWire audio output unless muted. Returns `None` (with a
/// warning) if the audio backend cannot be initialized.
fn start_audio(opts: &Options) -> Option<Audio> {
    if opts.mute {
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
fn write_wav(path: &str, samples: &[i16], rate: u32) {
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

fn run_headless(machine: &mut Spectrum, opts: &Options) {
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
    let audio = if opts.audio { start_audio(opts) } else { None };
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
                machine.keyboard[0], machine.keyboard[1], machine.keyboard[2], machine.keyboard[3],
                machine.keyboard[4], machine.keyboard[5], machine.keyboard[6], machine.keyboard[7],
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
            let path = opts
                .save
                .clone()
                .unwrap_or_else(default_save_name);
            match native::commander_save(&machine, &path, save_layout(opts)) {
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
        let dir = if dir.is_empty() { "tmp".to_string() } else { dir };
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(format!("{dir}/screen.bin"), &machine.mem[0x4000..0x5B00]);
        let _ = fs::write(format!("{dir}/buf.bin"), &machine.mem[0xC000..0xD000]);
        let _ = fs::write(format!("{dir}/buf800.bin"), &machine.mem[0xC800..0xD800]);
        let _ = fs::write(format!("{dir}/font.bin"), &machine.mem[0x5D70..0x5D70 + 0x2D8]);
        println!("wrote {dir}/screen.bin buf.bin buf800.bin font.bin");
    }
    if let (Some(path), Some(samples)) = (&opts.wav, &wav_samples) {
        write_wav(path, samples, audio::SAMPLE_RATE);
    }
}

fn write_ppm(path: &str, buf: &[u32], w: usize, h: usize) {
    let mut out = Vec::with_capacity(w * h * 3 + 32);
    out.extend_from_slice(format!("P6\n{} {}\n255\n", w, h).as_bytes());
    for &px in buf {
        out.push(((px >> 16) & 0xFF) as u8);
        out.push(((px >> 8) & 0xFF) as u8);
        out.push((px & 0xFF) as u8);
    }
    let _ = fs::write(path, out);
}

/// Авто-имя файла сохранения: `elite-ГГГГ-ММ-ДД_ЧЧ-ММ-СС.zxelite`.
fn default_save_name() -> String {
    let now = chrono::Local::now();
    format!("elite-{}.zxelite", now.format("%Y-%m-%d_%H-%M-%S"))
}

/// Диалог сохранения командира. `last` — прошлый путь (задаёт каталог).
fn save_dialog(last: &str) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .add_filter("Elite (ZX)", &["zxelite"])
        .set_file_name(default_save_name());
    if let Some(dir) = Path::new(last).parent().filter(|p| !p.as_os_str().is_empty()) {
        d = d.set_directory(dir);
    }
    d.save_file()
}

/// Путь сохранения: фиксированный `--save` или интерактивный диалог.
fn choose_save_path(opts: &Options, last: &str) -> Option<PathBuf> {
    match &opts.save {
        Some(p) => Some(PathBuf::from(p)),
        None => save_dialog(last),
    }
}

/// Сохранить блок командира; при успехе запомнить путь в `last`.
fn do_save(machine: &Spectrum, opts: &Options, last: &mut String) {
    let Some(path) = choose_save_path(opts, last) else {
        return;
    };
    match native::commander_save(machine, &path.to_string_lossy(), save_layout(opts)) {
        Ok(()) => {
            println!("Сохранено в {}", path.display());
            *last = path.to_string_lossy().into_owned();
        }
        Err(e) => eprintln!("Ошибка сохранения: {e}"),
    }
}

fn run_window(mut machine: Spectrum, opts: &Options) {
    let mut window = Window::new(
        "Elite (ZX Spectrum)",
        256,
        192,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("cannot open window: {}", e);
        exit(1);
    });
    window.set_target_fps(50);

    let audio = start_audio(opts);
    let mut wav_samples: Option<Vec<i16>> = opts.wav.as_ref().map(|_| Vec::new());
    let mut audio_buf = Vec::new();
    let mut buf = vec![0u32; 256 * 192];
    let mut last_path = String::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        machine.release_all_keys();
        machine.release_joystick();
        apply_keys(&mut machine, &window, opts.kladov);
        machine.run_frame();
        if machine.save_requested {
            machine.save_requested = false;
            do_save(&machine, opts, &mut last_path);
        }
        machine.render_audio(&mut audio_buf);
        if let Some(audio) = &audio {
            audio.push(&audio_buf);
        }
        if let Some(wav) = &mut wav_samples {
            wav.extend_from_slice(&audio_buf);
        }
        machine.render_rgba(&mut buf);
        let _ = window.update_with_buffer(&buf, 256, 192);
    }
    if let (Some(path), Some(samples)) = (&opts.wav, &wav_samples) {
        write_wav(path, samples, audio::SAMPLE_RATE);
    }
}

fn apply_keys(machine: &mut Spectrum, window: &Window, kempston_as_keys: bool) {
    // (minifb key, half-row index, bit)
    const MAP: &[(Key, usize, u8)] = &[
        (Key::LeftShift, 0, 0), (Key::Z, 0, 1), (Key::X, 0, 2), (Key::C, 0, 3), (Key::V, 0, 4),
        (Key::A, 1, 0), (Key::S, 1, 1), (Key::D, 1, 2), (Key::F, 1, 3), (Key::G, 1, 4),
        (Key::Q, 2, 0), (Key::W, 2, 1), (Key::E, 2, 2), (Key::R, 2, 3), (Key::T, 2, 4),
        (Key::Key1, 3, 0), (Key::Key2, 3, 1), (Key::Key3, 3, 2), (Key::Key4, 3, 3), (Key::Key5, 3, 4),
        (Key::Key0, 4, 0), (Key::Key9, 4, 1), (Key::Key8, 4, 2), (Key::Key7, 4, 3), (Key::Key6, 4, 4),
        (Key::P, 5, 0), (Key::O, 5, 1), (Key::I, 5, 2), (Key::U, 5, 3), (Key::Y, 5, 4),
        (Key::Enter, 6, 0), (Key::L, 6, 1), (Key::K, 6, 2), (Key::J, 6, 3), (Key::H, 6, 4),
        (Key::Space, 7, 0), (Key::RightShift, 7, 1), (Key::M, 7, 2), (Key::N, 7, 3), (Key::B, 7, 4),
    ];
    for &(key, row, bit) in MAP {
        machine.set_key(row, bit, window.is_key_down(key));
    }

    // Kempston (порт 0x1F): курсорные клавиши = направления, RightCtrl = огонь.
    // Скорость (Space / SymbolShift) в режиме Kempston всё ещё читается игрой
    // с клавиатуры, поэтому Space здесь не трогаем.
    machine.set_kempston(0, window.is_key_down(Key::Right));
    machine.set_kempston(1, window.is_key_down(Key::Left));
    machine.set_kempston(2, window.is_key_down(Key::Down));
    machine.set_kempston(3, window.is_key_down(Key::Up));
    machine.set_kempston(4, window.is_key_down(Key::RightCtrl));

    // В версии Кладова опрос порта Kempston в игре убран: направления она читает
    // только с клавиатуры (N/M/X/S), а огонь — по клавише A. Поэтому дублируем
    // джойстик на эти же клавиши, чтобы стрелки и RightCtrl работали.
    if kempston_as_keys {
        if window.is_key_down(Key::Right) {
            machine.set_key(7, 2, true);
        }
        if window.is_key_down(Key::Left) {
            machine.set_key(7, 3, true);
        }
        if window.is_key_down(Key::Up) {
            machine.set_key(1, 1, true);
        }
        if window.is_key_down(Key::Down) {
            machine.set_key(0, 2, true);
        }
        if window.is_key_down(Key::RightCtrl) {
            machine.set_key(1, 0, true);
        }
    }
}

#[cfg(test)]
mod intro_tests {
    use super::*;

    /// Boot the real game image with the real ROM, as `main` does.
    fn boot() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = fs::read(format!("{base}/zx_bin/elite.bin")).expect("zx_bin/elite.bin");
        let mut m = Spectrum::new(Some(&rom));
        m.load(GAME_LOAD_ADDR, &game);
        install_im2_table(&mut m);
        m.cpu.pc = GAME_ENTRY;
        m.cpu.sp = 0x6000;
        m
    }

    /// The intro used to crash: a wrong `CP` corrupted the line-drawing clip
    /// logic, the drawer walked out of the screen buffer and self-modified
    /// code at 0xBAF7, after which the message printer recursed and leaked SP.
    #[test]
    fn intro_runs_without_corruption_or_leak() {
        let mut m = boot();
        assert_eq!(m.mem[0xBAF7], 0x38, "precondition: dispatcher intact");
        m.counts.push((0x71A2, 0)); // intro loop
        for _ in 0..60 {
            m.run_frame();
        }
        assert!(m.cpu.pc != 0, "CPU ran off the rails (pc=0)");
        assert_eq!(m.mem[0xBAF7], 0x38, "code at 0xBAF7 was self-modified");
        assert_eq!(m.mem[0xBAF5], 0xFE, "code at 0xBAF5 was self-modified");
        assert!(m.cpu.sp >= 0x5F00, "stack leaked: sp=0x{:04X}", m.cpu.sp);
        assert!(
            m.counts[0].1 > 0,
            "intro loop at 0x71A2 was never reached"
        );
    }
}
