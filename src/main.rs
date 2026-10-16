mod machine;
mod z80;

use machine::Spectrum;
use minifb::{Key, Scale, Window, WindowOptions};
use std::env;
use std::fs;
use std::process::exit;

/// Load address of the assembled game image.
const GAME_LOAD_ADDR: u16 = 0x6048;
/// Entry point (label `Start`).
const GAME_ENTRY: u16 = 0x7000;

struct Options {
    game: String,
    rom: Option<String>,
    headless: bool,
    frames: u64,
    dump: Option<String>,
}

fn parse_args() -> Options {
    let mut opts = Options {
        game: "elite.bin".to_string(),
        rom: None,
        headless: false,
        frames: 600,
        dump: None,
    };
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--rom" => opts.rom = args.next(),
            "--frames" => opts.frames = args.next().and_then(|s| s.parse().ok()).unwrap_or(600),
            "--dump" => {
                opts.dump = args.next();
                opts.headless = true;
            }
            "--headless" => opts.headless = true,
            "-h" | "--help" => {
                println!(
                    "usage: elite [game.bin] [--rom rom.bin] [--headless] [--frames N] [--dump out.ppm]"
                );
                exit(0);
            }
            other => opts.game = other.to_string(),
        }
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
            if opts.game == "elite.bin" {
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
    machine.load(GAME_LOAD_ADDR, &game);
    install_im2_table(&mut machine);
    machine.cpu.pc = GAME_ENTRY;
    machine.cpu.sp = 0x6000;

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

fn run_headless(machine: &mut Spectrum, opts: &Options) {
    let trace = env::var("ELITE_TRACE").is_ok();
    let sp_from: Option<u64> = env::var("ELITE_SP").ok().and_then(|s| s.parse().ok());
    for f in 0..opts.frames {
        machine.run_frame();
        if trace && f < 120 {
            println!(
                "frame {:3} pc=0x{:04X} sp=0x{:04X} presskey=0x{:02X}",
                f, machine.cpu.pc, machine.cpu.sp, machine.mem[0xFC13]
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
    }
    println!(
        "frames={} pc=0x{:04X} cycles={} sp=0x{:04X}",
        machine.frames, machine.cpu.pc, machine.cpu.cycles, machine.cpu.sp
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
    if env::var("ELITE_MEM").is_ok() {
        let _ = fs::write("/tmp/opencode/screen.bin", &machine.mem[0x4000..0x5B00]);
        let _ = fs::write("/tmp/opencode/buf.bin", &machine.mem[0xC000..0xD000]);
        let _ = fs::write("/tmp/opencode/buf800.bin", &machine.mem[0xC800..0xD800]);
        let _ = fs::write("/tmp/opencode/font.bin", &machine.mem[0x5D70..0x5D70 + 0x2D8]);
        println!("wrote screen.bin buf.bin buf800.bin font.bin");
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

fn run_window(mut machine: Spectrum, _opts: &Options) {
    let scale = Scale::X2;
    let mut window = Window::new(
        "Elite (ZX Spectrum)",
        256,
        192,
        WindowOptions {
            resize: true,
            scale,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("cannot open window: {}", e);
        exit(1);
    });
    window.set_target_fps(50);

    let mut buf = vec![0u32; 256 * 192];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        machine.release_all_keys();
        apply_keys(&mut machine, &window);
        machine.run_frame();
        machine.render_rgba(&mut buf);
        let _ = window.update_with_buffer(&buf, 256, 192);
    }
}

fn apply_keys(machine: &mut Spectrum, window: &Window) {
    // (minifb key, half-row index, bit)
    const MAP: &[(Key, usize, u8)] = &[
        (Key::Key1, 3, 4), (Key::Key2, 3, 3), (Key::Key3, 3, 2), (Key::Key4, 3, 1), (Key::Key5, 3, 0),
        (Key::Key6, 4, 0), (Key::Key7, 4, 1), (Key::Key8, 4, 2), (Key::Key9, 4, 3), (Key::Key0, 4, 4),
        (Key::Q, 2, 4), (Key::W, 2, 3), (Key::E, 2, 2), (Key::R, 2, 1), (Key::T, 2, 0),
        (Key::A, 1, 4), (Key::S, 1, 3), (Key::D, 1, 2), (Key::F, 1, 1), (Key::G, 1, 0),
        (Key::P, 5, 4), (Key::O, 5, 3), (Key::I, 5, 2), (Key::U, 5, 1), (Key::Y, 5, 0),
        (Key::Enter, 6, 4), (Key::L, 6, 3), (Key::K, 6, 2), (Key::J, 6, 1), (Key::H, 6, 0),
        (Key::Space, 7, 4), (Key::RightShift, 7, 3), (Key::M, 7, 2), (Key::N, 7, 1), (Key::B, 7, 0),
        (Key::Z, 0, 3), (Key::X, 0, 2), (Key::C, 0, 1), (Key::V, 0, 0),
        (Key::LeftShift, 0, 4),
    ];
    for &(key, row, bit) in MAP {
        machine.set_key(row, bit, window.is_key_down(key));
    }
}
