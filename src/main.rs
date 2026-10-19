mod audio;
mod machine;
mod native;
mod z80;

use audio::Audio;
use machine::Spectrum;
use softbuffer::{Context, Surface};
use std::env;
use std::fs;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Window, WindowId};

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
                println!();
                println!("Управление:");
                println!("  Esc        меню: сохранить / загрузить / выйти");
                println!("  F5         сохранить командира");
                println!("  F9         загрузить командира");
                println!("  Ctrl+Q     выход из игры");
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
        native::install(&mut machine, opts.kladov);
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

    if let Ok(w) = env::var("ELITE_WTRACE") {
        if let Some((a, b)) = w.split_once('-') {
            let lo = u16::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0);
            let hi = u16::from_str_radix(b.trim_start_matches("0x"), 16).unwrap_or(0xFFFF);
            machine.wtrace = Some((lo, hi));
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

    // ELITE_PROFILE[=N] — гистограмма всех исполненных PC, печатает топ-N
    // (по умолчанию 40) после headless-прогона. Ищем горячие рутины для M5.
    if let Ok(v) = env::var("ELITE_PROFILE") {
        machine.profile = vec![0u32; 0x1_0000];
        machine.profile_top = v.parse().unwrap_or(40).clamp(1, 512);
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
            match native::commander_save(&mut *machine, &path, save_layout(opts)) {
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
        let dir = if dir.is_empty() { "tmp".to_string() } else { dir };
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(format!("{dir}/screen.bin"), &machine.mem[0x4000..0x5B00]);
        let _ = fs::write(format!("{dir}/buf.bin"), &machine.mem[0xC000..0xD000]);
        let _ = fs::write(format!("{dir}/buf800.bin"), &machine.mem[0xC800..0xD800]);
        let _ = fs::write(format!("{dir}/font.bin"), &machine.mem[0x5D70..0x5D70 + 0x2D8]);
        let _ = fs::write(format!("{dir}/mem.bin"), &machine.mem);
        println!("wrote {dir}/screen.bin buf.bin buf800.bin font.bin mem.bin");
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
fn do_save(machine: &mut Spectrum, opts: &Options, last: &mut String) {
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

/// Диалог открытия файла командира. `last` — прошлый путь (задаёт каталог).
fn load_dialog(last: &str) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new().add_filter("Elite (ZX)", &["zxelite"]);
    if let Some(dir) = Path::new(last).parent().filter(|p| !p.as_os_str().is_empty()) {
        d = d.set_directory(dir);
    }
    d.pick_file()
}

/// Путь загрузки: фиксированный `--save` или интерактивный диалог открытия.
fn choose_load_path(opts: &Options, last: &str) -> Option<PathBuf> {
    match &opts.save {
        Some(p) => Some(PathBuf::from(p)),
        None => load_dialog(last),
    }
}

/// Загрузить блок командира из файла; при успехе запомнить путь в `last`.
fn do_load(machine: &mut Spectrum, opts: &Options, last: &mut String) {
    let Some(path) = choose_load_path(opts, last) else {
        return;
    };
    match native::commander_load(machine, &path.to_string_lossy(), save_layout(opts)) {
        Ok(()) => {
            println!("Загружено из {}", path.display());
            *last = path.to_string_lossy().into_owned();
            // Прервать текущий процесс и запустить игру с новыми данными:
            // после стартовой последовательности корабль пристыкован на
            // станции системы из сейва. Перезапуск делает сама игра (родной
            // путь через `MenuSaveLoad`), см. `native::request_load`.
            native::request_load(machine, save_layout(opts));
        }
        Err(e) => eprintln!("Ошибка загрузки: {e}"),
    }
}

/// Размер кадра игры (ZX Spectrum).
const SRC_W: usize = 256;
const SRC_H: usize = 192;

/// Частота кадров ZX Spectrum: 50 кадров в секунду (кадр каждые 20 мс).
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 50);

fn run_window(machine: Spectrum, opts: &Options) {
    let event_loop = match EventLoop::new() {
        Ok(el) => el,
        Err(e) => {
            eprintln!("cannot create event loop: {e}");
            exit(1);
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(machine, opts);
    let result = event_loop.run_app(&mut app);
    app.finish();
    if let Err(e) = result {
        eprintln!("event loop error: {e}");
        exit(1);
    }
}

/// Оконное приложение на winit: окно, кадр 256x192 и состояние клавиатуры.
///
/// Клавиши читаются как `physical_key` (позиция клавиши), поэтому активная
/// раскладка (в частности русская) на управление не влияет.
struct App<'a> {
    machine: Spectrum,
    opts: &'a Options,
    font: Vec<u8>,
    audio: Option<Audio>,
    audio_buf: Vec<i16>,
    wav_samples: Option<Vec<i16>>,
    last_path: String,
    /// Открыто ли меню паузы (Esc) — игра не выполняется.
    menu: bool,
    quit: bool,
    buf: Vec<u32>,
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    /// Состояние ZX-матрицы (полуряд, бит) — обновляется событиями клавиш.
    matrix: [[bool; 5]; 8],
    /// Состояние джойстика Kempston (5 бит).
    kempston: [bool; 5],
    ctrl: bool,
    next_frame: Instant,
}

impl<'a> App<'a> {
    fn new(machine: Spectrum, opts: &'a Options) -> Self {
        // Шрифт знакогенератора из ПЗУ (0x3D00, коды 0x20..0x7F) — копируем один
        // раз, чтобы наложение меню не зависело от содержимого памяти в рантайме.
        let font = machine.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN].to_vec();
        Self {
            machine,
            opts,
            font,
            audio: start_audio(opts),
            audio_buf: Vec::new(),
            wav_samples: opts.wav.as_ref().map(|_| Vec::new()),
            last_path: String::new(),
            menu: false,
            quit: false,
            buf: vec![0u32; SRC_W * SRC_H],
            window: None,
            context: None,
            surface: None,
            matrix: [[false; 5]; 8],
            kempston: [false; 5],
            ctrl: false,
            next_frame: Instant::now(),
        }
    }

    /// Записать накопленный звук в WAV (если запрошено `--wav`).
    fn finish(&mut self) {
        if let (Some(path), Some(samples)) = (&self.opts.wav, &self.wav_samples) {
            write_wav(path, samples, audio::SAMPLE_RATE);
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Elite — Esc: меню, F5: сохранить, F9: загрузить, Ctrl+Q: выход")
            .with_inner_size(LogicalSize::new(2.0 * SRC_W as f64, 2.0 * SRC_H as f64));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Rc::new(w),
            Err(e) => {
                eprintln!("cannot open window: {e}");
                exit(1);
            }
        };
        let context = match Context::new(window.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("cannot init softbuffer: {e}");
                exit(1);
            }
        };
        let surface = match Surface::new(&context, window.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cannot create surface: {e}");
                exit(1);
            }
        };
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.next_frame = Instant::now();
    }

    /// Обработать событие клавиши (по физической позиции, раскладка не важна).
    fn key_event(&mut self, code: KeyCode, pressed: bool, repeat: bool) {
        if let Some((row, bit)) = matrix_pos(code) {
            self.matrix[row][bit as usize] = pressed;
        }
        if let Some(bit) = kempston_bit(code) {
            self.kempston[bit as usize] = pressed;
        }
        if !pressed || repeat {
            return;
        }
        // Ctrl+Q — выход из игры.
        if code == KeyCode::KeyQ && self.ctrl {
            self.quit = true;
            return;
        }
        match code {
            KeyCode::Escape => self.menu = !self.menu,
            KeyCode::F5 => do_save(&mut self.machine, self.opts, &mut self.last_path),
            KeyCode::F9 => do_load(&mut self.machine, self.opts, &mut self.last_path),
            KeyCode::Digit1 if self.menu => {
                do_save(&mut self.machine, self.opts, &mut self.last_path);
                self.menu = false;
            }
            KeyCode::Digit2 if self.menu => {
                do_load(&mut self.machine, self.opts, &mut self.last_path);
                self.menu = false;
            }
            KeyCode::Digit3 if self.menu => self.quit = true,
            _ => {}
        }
    }

    /// Перенести состояние клавиатуры и джойстика в эмулятор.
    fn apply_input(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
        if self.menu {
            return;
        }
        for (row, bits) in self.matrix.iter().enumerate() {
            for (bit, &down) in bits.iter().enumerate() {
                if down {
                    self.machine.set_key(row, bit as u8, true);
                }
            }
        }
        for (bit, &down) in self.kempston.iter().enumerate() {
            self.machine.set_kempston(bit as u8, down);
        }
        // В версии Кладова опрос порта Kempston в игре убран: направления она
        // читает только с клавиатуры (N/M/X/S), а огонь — по клавише A.
        // Дублируем джойстик на те же клавиши, чтобы стрелки и RightCtrl работали.
        if self.opts.kladov {
            if self.kempston[0] {
                self.machine.set_key(7, 2, true); // вправо
            }
            if self.kempston[1] {
                self.machine.set_key(7, 3, true); // влево
            }
            if self.kempston[2] {
                self.machine.set_key(0, 2, true); // вниз
            }
            if self.kempston[3] {
                self.machine.set_key(1, 1, true); // вверх
            }
            if self.kempston[4] {
                self.machine.set_key(1, 0, true); // огонь
            }
        }
    }

    /// Один кадр: ввод, эмуляция, звук, отрисовка и показ.
    fn frame(&mut self) {
        if self.menu {
            // В меню игра не выполняется, клавиши не попадают в эмулятор.
            self.machine.release_all_keys();
            self.machine.release_joystick();
        } else {
            self.apply_input();
            self.machine.run_frame();
            if self.machine.save_requested {
                self.machine.save_requested = false;
                do_save(&mut self.machine, self.opts, &mut self.last_path);
            }
            self.machine.render_audio(&mut self.audio_buf);
            if let Some(audio) = &self.audio {
                audio.push(&self.audio_buf);
            }
            if let Some(wav) = &mut self.wav_samples {
                wav.extend_from_slice(&self.audio_buf);
            }
        }
        self.machine.render_rgba(&mut self.buf);
        if self.menu {
            draw_pause_menu(&mut self.buf, &self.font);
        }
        self.present();
    }

    /// Показать кадр 256x192 в окне, сохраняя пропорции 4:3 (по центру).
    fn present(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let size = window.inner_size();
        let (w, h) = (size.width, size.height);
        if w == 0 || h == 0 {
            return;
        }
        let Some(surface) = self.surface.as_mut() else {
            return;
        };
        if surface
            .resize(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap())
            .is_err()
        {
            return;
        }
        let src = &self.buf;
        let Ok(mut out) = surface.buffer_mut() else {
            return;
        };
        // Масштаб с сохранением пропорций, выравнивание по центру (letterbox).
        let scale = (w as f64 / SRC_W as f64).min(h as f64 / SRC_H as f64);
        let dw = ((SRC_W as f64 * scale).round() as u32).clamp(1, w);
        let dh = ((SRC_H as f64 * scale).round() as u32).clamp(1, h);
        let ox = (w - dw) / 2;
        let oy = (h - dh) / 2;
        let mut i = 0usize;
        for y in 0..h {
            let in_y = y >= oy && y < oy + dh;
            let sy = if in_y { ((y - oy) as u64 * SRC_H as u64 / dh as u64) as usize } else { 0 };
            for x in 0..w {
                let inside = in_y && x >= ox && x < ox + dw;
                out[i] = if inside {
                    let sx = ((x - ox) as u64 * SRC_W as u64 / dw as u64) as usize;
                    src[sy * SRC_W + sx]
                } else {
                    0
                };
                i += 1;
            }
        }
        window.pre_present_notify();
        let _ = out.present();
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(false) => {
                // При потере фокуса не «залипаем» на нажатых клавишах.
                self.matrix = [[false; 5]; 8];
                self.kempston = [false; 5];
                self.ctrl = false;
            }
            WindowEvent::ModifiersChanged(m) => {
                self.ctrl = m.state().contains(ModifiersState::CONTROL);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    self.key_event(code, pressed, event.repeat);
                }
                if self.quit {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.quit {
            event_loop.exit();
            return;
        }
        let now = Instant::now();
        if now >= self.next_frame {
            self.frame();
            self.next_frame += FRAME_TIME;
            // Если не успеваем за реальным временем — не копим долг.
            if self.next_frame <= now {
                self.next_frame = now + FRAME_TIME;
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

/// Нарисовать закрашенный прямоугольник поверх кадра.
fn fill_rect(buf: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            if x < 256 && y < 192 {
                buf[y * 256 + x] = color;
            }
        }
    }
}

/// Адрес знакогенератора ZX Spectrum в ПЗУ (коды символов 0x20..0x7F).
const PAUSE_FONT_ADDR: usize = 0x3D00;
/// Длина знакогенератора: 96 символов по 8 байт.
const PAUSE_FONT_LEN: usize = 0x300;

/// Нарисовать строку шрифтом ПЗУ (8x8, знакогенератор ZX Spectrum) поверх
/// кадра. Рисуются только установленные биты (фон не закрашивается).
fn draw_text(buf: &mut [u32], font: &[u8], x: usize, y: usize, s: &str, color: u32) {
    let mut cx = x;
    for b in s.bytes() {
        let ch = b.clamp(0x20, 0x7F) as usize;
        let off = (ch - 0x20) * 8;
        if off + 8 > font.len() {
            break;
        }
        for (row, bits) in font[off..off + 8].iter().enumerate() {
            for col in 0..8 {
                if bits & (0x80 >> col) != 0 {
                    let (px, py) = (cx + col, y + row);
                    if px < 256 && py < 192 {
                        buf[py * 256 + px] = color;
                    }
                }
            }
        }
        cx += 8;
    }
}

/// Наложение меню паузы: рамка и пункты «1 сохранить / 2 загрузить / 3 выйти».
fn draw_pause_menu(buf: &mut [u32], font: &[u8]) {
    const FG: u32 = 0x00D7_D7D7; // белый
    const BG: u32 = 0x0000_0000; // чёрный
    let (x0, y0, x1, y1) = (40usize, 52usize, 216usize, 140usize);
    fill_rect(buf, x0, y0, x1, y1, BG);
    // Рамка: внешняя и внутренняя линии.
    for x in x0..=x1 {
        buf[y0 * 256 + x] = FG;
        buf[y1 * 256 + x] = FG;
    }
    for y in y0..=y1 {
        buf[y * 256 + x0] = FG;
        buf[y * 256 + x1] = FG;
    }
    let lines = [
        (y0 + 8, "ELITE - PAUSED"),
        (y0 + 24, "1  SAVE GAME"),
        (y0 + 36, "2  LOAD GAME"),
        (y0 + 48, "3  QUIT GAME"),
        (y0 + 68, "ESC RESUME"),
    ];
    for (y, s) in lines {
        draw_text(buf, font, x0 + 12, y, s, FG);
    }
}

/// Позиция клавиши в матрице ZX Spectrum: (полуряд 0..8, бит 0..5).
///
/// Клавиши берутся по физической позиции (`physical_key`), поэтому активная
/// раскладка (например русская) на управление не влияет.
const MATRIX_MAP: &[(KeyCode, usize, u8)] = &[
    (KeyCode::ShiftLeft, 0, 0), (KeyCode::KeyZ, 0, 1), (KeyCode::KeyX, 0, 2), (KeyCode::KeyC, 0, 3), (KeyCode::KeyV, 0, 4),
    (KeyCode::KeyA, 1, 0), (KeyCode::KeyS, 1, 1), (KeyCode::KeyD, 1, 2), (KeyCode::KeyF, 1, 3), (KeyCode::KeyG, 1, 4),
    (KeyCode::KeyQ, 2, 0), (KeyCode::KeyW, 2, 1), (KeyCode::KeyE, 2, 2), (KeyCode::KeyR, 2, 3), (KeyCode::KeyT, 2, 4),
    (KeyCode::Digit1, 3, 0), (KeyCode::Digit2, 3, 1), (KeyCode::Digit3, 3, 2), (KeyCode::Digit4, 3, 3), (KeyCode::Digit5, 3, 4),
    (KeyCode::Digit0, 4, 0), (KeyCode::Digit9, 4, 1), (KeyCode::Digit8, 4, 2), (KeyCode::Digit7, 4, 3), (KeyCode::Digit6, 4, 4),
    (KeyCode::KeyP, 5, 0), (KeyCode::KeyO, 5, 1), (KeyCode::KeyI, 5, 2), (KeyCode::KeyU, 5, 3), (KeyCode::KeyY, 5, 4),
    (KeyCode::Enter, 6, 0), (KeyCode::KeyL, 6, 1), (KeyCode::KeyK, 6, 2), (KeyCode::KeyJ, 6, 3), (KeyCode::KeyH, 6, 4),
    (KeyCode::Space, 7, 0), (KeyCode::ShiftRight, 7, 1), (KeyCode::KeyM, 7, 2), (KeyCode::KeyN, 7, 3), (KeyCode::KeyB, 7, 4),
];

/// Позиция физической клавиши в матрице ZX Spectrum (полуряд, бит).
fn matrix_pos(code: KeyCode) -> Option<(usize, u8)> {
    MATRIX_MAP
        .iter()
        .find(|&&(c, _, _)| c == code)
        .map(|&(_, row, bit)| (row, bit))
}

/// Бит джойстика Kempston: курсорные клавиши — направления, правая Ctrl — огонь.
fn kempston_bit(code: KeyCode) -> Option<u8> {
    Some(match code {
        KeyCode::ArrowRight => 0,
        KeyCode::ArrowLeft => 1,
        KeyCode::ArrowDown => 2,
        KeyCode::ArrowUp => 3,
        KeyCode::ControlRight => 4,
        _ => return None,
    })
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

    /// Меню паузы (Esc) рисуется поверх кадра: рамка и текст шрифтом ПЗУ.
    /// `ELITE_DUMP_MENU=<file>` сохраняет кадр в PPM для визуальной проверки.
    #[test]
    fn pause_menu_overlay_renders() {
        let m = boot();
        let mut buf = vec![0u32; 256 * 192];
        m.render_rgba(&mut buf);
        let before = buf.clone();
        let font = &m.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN];
        draw_pause_menu(&mut buf, font);

        const WHITE: u32 = 0x00D7_D7D7;
        // Угол рамки панели — белый (панель x0=40,y0=52).
        assert_eq!(buf[52 * 256 + 40], WHITE, "panel border not drawn");
        // Внутри панели нарисован текст: заметное число белых пикселей.
        let white = buf.iter().filter(|&&p| p == WHITE).count();
        assert!(white > 200, "menu text too sparse: white={white}");
        // Меню действительно поверх прежнего кадра.
        assert!(buf != before, "overlay did not change the frame");

        if let Ok(path) = env::var("ELITE_DUMP_MENU") {
            write_ppm(&path, &buf, 256, 192);
        }
    }

    /// Физические клавиши раскладываются по матрице ZX Spectrum без зависимости
    /// от активной раскладки: (полуряд, бит) совпадают с оригинальной схемой.
    #[test]
    fn matrix_map_covers_keyboard() {
        assert_eq!(matrix_pos(KeyCode::ShiftLeft), Some((0, 0)));
        assert_eq!(matrix_pos(KeyCode::KeyQ), Some((2, 0)));
        assert_eq!(matrix_pos(KeyCode::Digit1), Some((3, 0)));
        assert_eq!(matrix_pos(KeyCode::Enter), Some((6, 0)));
        assert_eq!(matrix_pos(KeyCode::Space), Some((7, 0)));
        assert_eq!(matrix_pos(KeyCode::KeyB), Some((7, 4)));
        // Вне матрицы (управление игрой) — ничего.
        assert_eq!(matrix_pos(KeyCode::ArrowUp), None);
        assert_eq!(matrix_pos(KeyCode::F5), None);
        // Все 40 клавиш матрицы заданы и уникальны.
        assert_eq!(MATRIX_MAP.len(), 40);
        let mut seen = std::collections::HashSet::new();
        for &(code, row, bit) in MATRIX_MAP {
            assert!(row < 8 && bit < 5, "позиция вне матрицы: {code:?} ({row},{bit})");
            assert!(seen.insert((row, bit)), "дубликат позиции {row},{bit}");
        }
        // Kempston: стрелки и правая Ctrl.
        assert_eq!(kempston_bit(KeyCode::ArrowRight), Some(0));
        assert_eq!(kempston_bit(KeyCode::ControlRight), Some(4));
        assert_eq!(kempston_bit(KeyCode::Space), None);
    }
}
