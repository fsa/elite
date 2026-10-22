mod audio;
mod cockpit;
mod display;
mod equip;
mod galaxy;
mod hdmap;
mod machine;
mod native;
#[cfg(test)]
mod probe;
mod shipdata;
mod splash;
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
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

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
    /// Запустить игру сразу в полноэкранном режиме (borderless).
    /// Переключение во время игры: F11 или Alt+Enter.
    fullscreen: bool,
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
        fullscreen: false,
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
            "--fullscreen" => opts.fullscreen = true,
            "-h" | "--help" => {
                println!(
                    "usage: elite [game.bin] [--elite] [--rom rom.bin] [--headless] [--frames N] [--dump out.ppm] [--mute] [--audio] [--wav out.wav] [--native] [--save FILE] [--fullscreen]"
                );
                println!("  по умолчанию запускается версия Кладова (zx_bin/kladov.bin)");
                println!("  --mute     отключить вывод звука через PipeWire");
                println!("  --audio    выводить звук и в headless-режиме (реальное время)");
                println!("  --wav FILE сохранить звук в WAV-файл");
                println!("  --native   включить нативные (Rust) замены подпрограмм");
                println!("  --save FILE фиксированный файл командира вместо диалога");
                println!("  --fullscreen запуск в полноэкранном режиме (borderless)");
                println!("В игре: F10 — карта галактики (нативная), F4 — закупка");
                println!("        оборудования (нативная), Esc — меню паузы");
                println!("        (продолжить/сохранить/загрузить/выход),");
                println!("        F11 — полный экран.");
                println!();
                println!("Заставка нативная: [SPACE] — начать игру, [Esc] — меню");
                println!("  паузы (на заставке только загрузка, сохранение без игры");
                println!("  отключено).");
                println!();
                println!("Экран: ZX-кадр 256x192 показывается целым множителем (чёткие");
                println!("квадратные пиксели) по центру; нативные (Rust) реализации");
                println!("рисуют в HD-слой поверх него в полном разрешении окна.");
                println!();
                println!("Управление:");
                println!("  Esc        меню паузы (продолжить/сохранить/загрузить/выход)");
                println!("  F4         закупка оборудования (нативный экран)");
                println!("  F11, Alt+Enter полный экран");
                println!("  Ctrl+Q     выход из игры");
                println!();
                println!("Заставка: SPACE запускает игру, остальные клавиши не срабатывают.");
                println!();
                println!("Карта галактики (F10): стрелки/NM/XS — курсор, Enter —");
                println!("  ближайшая планета, +/- или колесо — масштаб, F1 — чит:");
                println!("  телепорт на станцию выбранной планеты, Esc — закрыть.");
                println!();
                println!("Закупка оборудования (F4): клик/Enter — купить,");
                println!("  1-9 (+вторая цифра) — номер товара, стрелки — выбор,");
                println!("  0-3 — крепление лазера, Esc — закрыть.");
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

    // Отслеживание заставки: на ней нативные экраны не открываются.
    let (intro, game_start) = scene_markers(opts.kladov);
    machine.intro_range = intro;
    machine.game_marker = game_start;

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

/// Маркеры «заставки» и старта игры для выбранной версии. Пока игра
/// проигрывает заставку, нативные экраны (карта F10, закупка F4) открывать
/// нельзя — как и у оригинальных команд, которых на заставке нет.
///
/// Интро определяется «залипающе»: адрес из первого диапазона взводит флаг
/// [`machine::Spectrum::intro_seen`], а вход в `GameMain` (второе значение) его
/// снимает. Просто держать флаг по кадру нельзя: цикл заставки вызывает
/// длинные рутины (очистка/копирование экрана) вне своего диапазона, и флаг
/// мерцал бы.
///
/// * Кладова — аттракт-цикл `sub_728D` (`loc_739E`..`loc_73FD`), старт игры
///   `GameMain` = `0x7471`.
/// * Оригинал — `BeginGame` (`loc_71A2`..`loc_721F`), старт игры `GameMain`
///   = `0x7287`.
fn scene_markers(kladov: bool) -> (Option<(u16, u16)>, Option<u16>) {
    if kladov {
        (Some((0x739E, 0x73FD)), Some(0x7471))
    } else {
        (Some((0x71A2, 0x721F)), Some(0x7287))
    }
}

/// Адреса переменных панели кокпита для выбранной версии игры.
fn cockpit_layout(opts: &Options) -> cockpit::CockpitLayout {
    if opts.kladov {
        cockpit::KLADOV_COCKPIT
    } else {
        cockpit::ELITE_COCKPIT
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

/// Размер кадра игры (ZX Spectrum). HD-слой и масштабирование — см. `display`.
const SRC_W: usize = display::ZX_W;
const SRC_H: usize = display::ZX_H;

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

/// Оконное приложение на winit: окно, кадр и состояние клавиатуры.
///
/// В игре показывается полный ZX-кадр 256x192 (вид из кабины и родная
/// растровая панель оригинала) целым множителем по центру окна (см.
/// [`display::layout`]), поверх — HD-слой [`Spectrum::hd`](machine::Spectrum).
/// Нативный кокпит (`src/cockpit.rs`) — черновик, он рисуется только на
/// заставке. Отдельные нативные экраны занимают окно целиком.
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
    /// Отдельная нативная карта галактики (F10) — заменяет экран игры.
    hdmap: hdmap::Map,
    /// Нативный экран закупки оборудования (F4) — заменяет экран игры.
    shop: equip::Shop,
    /// Нативная заставка (титульный экран): пока игра на интро, эмуляция не
    /// идёт, а окно занимает нативный рендер (`splash`).
    splash: splash::Splash,
    /// Нативный кокпит (приборная панель) — рисуется в нижней части окна
    /// поверх ZX-кадра, растянут на всю ширину (16:10).
    cockpit: cockpit::Cockpit,
    /// Выбранный пункт меню паузы (0 — продолжить, 1 — сохранить, 2 — загрузить,
    /// 3 — выйти). Стрелки двигают выбор, Enter подтверждает.
    menu_sel: usize,
    /// Признак выхода из игры.
    quit: bool,
    buf: Vec<u32>,
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    /// Состояние ZX-матрицы (полуряд, бит) — обновляется событиями клавиш.
    matrix: [[bool; 5]; 8],
    /// Нажатие игровой клавиши, которым был закрыт нативный экран: один кадр
    /// принудительно подаётся в эмулятор, даже если клавишу уже отпустили.
    pending_key: Option<(usize, u8)>,
    /// Состояние джойстика Kempston (5 бит).
    kempston: [bool; 5],
    ctrl: bool,
    /// Зажата ли Alt (для переключения полного экрана по Alt+Enter).
    alt: bool,
    /// Позиция мыши в координатах экрана игры (0..255, 0..191)
    mouse_x: i32,
    mouse_y: i32,
    mouse_left: bool,
    mouse_right: bool,
    mouse_wheel_up: bool,
    mouse_wheel_down: bool,
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
            hdmap: hdmap::Map::new(),
            shop: equip::Shop::new(),
            splash: splash::Splash::new(),
            cockpit: cockpit::Cockpit::new(),
            menu_sel: 0,
            quit: false,
            buf: vec![0u32; SRC_W * SRC_H],
            window: None,
            context: None,
            surface: None,
            matrix: [[false; 5]; 8],
            pending_key: None,
            kempston: [false; 5],
            ctrl: false,
            alt: false,
            mouse_x: -1,
            mouse_y: -1,
            mouse_left: false,
            mouse_right: false,
            mouse_wheel_up: false,
            mouse_wheel_down: false,
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
            .with_title("Elite — Esc: меню паузы, F10: карта, F4: закупка, F11: полный экран, Ctrl+Q: выход")
            .with_inner_size(LogicalSize::new(4.0 * SRC_W as f64, 4.0 * SRC_H as f64));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Rc::new(w),
            Err(e) => {
                eprintln!("cannot open window: {e}");
                exit(1);
            }
        };
        if self.opts.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
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
        if !pressed {
            return;
        }
        // F10 — отдельная нативная карта галактики. Пока это самостоятельный
        // просмотрщик: он не связан с игровыми картами (клавиши I/O) и не
        // трогает состояние командира.
        if code == KeyCode::F10 && !repeat {
            if self.hdmap.active {
                self.hdmap.close();
            } else if !self.machine.intro_seen {
                let layout = if self.opts.kladov {
                    galaxy::KLADOV_GALAXY
                } else {
                    galaxy::ELITE_GALAXY
                };
                self.hdmap.open(&self.machine.mem, layout);
                // Размер окна — чтобы зум/наведение работали до первого кадра.
                if let Some(w) = &self.window {
                    let size = w.inner_size();
                    self.hdmap.resize(size.width, size.height);
                }
                self.shop.close();
                self.menu = false;
                self.pending_key = None;
            }
            return;
        }
        // F4 — нативный экран закупки оборудования (EQUIP SHIP). Отдельный
        // полноэкранный экран: игра на паузе, данные читаются и пишутся прямо
        // в живой блок командира (покупки настоящие).
        if code == KeyCode::F4 && !repeat {
            if self.shop.active {
                self.shop.close();
            } else if !self.machine.intro_seen {
                let layout = if self.opts.kladov {
                    equip::KLADOV_SHOP
                } else {
                    equip::ELITE_SHOP
                };
                self.shop.open(&self.machine.mem, layout);
                // Размер окна — чтобы попадание мышью работало до первого кадра.
                if let Some(w) = &self.window {
                    let size = w.inner_size();
                    self.shop.resize(size.width, size.height);
                }
                self.hdmap.close();
                self.menu = false;
                self.pending_key = None;
            }
            return;
        }
        // При открытой карте всё управление принадлежит ей (игра на паузе).
        if self.hdmap.active {
            match code {
                KeyCode::ArrowLeft | KeyCode::KeyN => self.hdmap.move_by(-4, 0),
                KeyCode::ArrowRight | KeyCode::KeyM => self.hdmap.move_by(4, 0),
                KeyCode::ArrowUp | KeyCode::KeyS => self.hdmap.move_by(0, -4),
                KeyCode::ArrowDown | KeyCode::KeyX => self.hdmap.move_by(0, 4),
                KeyCode::Enter | KeyCode::Space => {
                    // На автоповторе не переходим в игру (Space — огонь).
                    if !repeat {
                        self.hdmap.snap_to_nearest();
                    }
                }
                // F1 — временный чит: телепорт «на станцию» выбранной планеты.
                // Пишем координаты системы и уходим в родную ветку
                // переинициализации (см. `native::request_teleport`), затем
                // закрываем карту — игра перерисует экран станции.
                KeyCode::F1 if !repeat => {
                    if let Some((x, y)) = self.hdmap.selected_coords() {
                        let layout = save_layout(&self.opts);
                        native::request_teleport(&mut self.machine, layout, x, y);
                        self.hdmap.close();
                    }
                }
                KeyCode::Equal | KeyCode::NumpadAdd => self.hdmap.zoom_by(1),
                KeyCode::Minus | KeyCode::NumpadSubtract => self.hdmap.zoom_by(-1),
                KeyCode::Escape if !repeat => self.hdmap.close(),
                // Игровая клавиша: закрываем карту и отдаём нажатие игре,
                // чтобы из карты сразу перейти к экранам игры (командир,
                // станция и т.п.), не закрывая её вручную.
                _ => {
                    if is_game_key(code) {
                        self.hdmap.close();
                        self.pending_key = matrix_pos(code);
                    }
                }
            }
            return;
        }
        // При открытом экране закупки всё управление принадлежит ему.
        if self.shop.active {
            match code {
                KeyCode::Escape if !repeat => {
                    if self.shop.mount_open() {
                        self.shop.cancel_mount();
                    } else {
                        self.shop.close();
                    }
                }
                KeyCode::ArrowUp => self.shop.move_selection(-1),
                KeyCode::ArrowDown => self.shop.move_selection(1),
                KeyCode::Enter | KeyCode::Space => {
                    // На автоповторе не переходим в игру (Space — огонь).
                    if !repeat {
                        self.shop.confirm(&mut self.machine.mem);
                    }
                }
                _ => {
                    if let Some(d) = digit_of(code) {
                        self.shop.input_digit(&mut self.machine.mem, d);
                    } else if is_game_key(code) {
                        // Игровая клавиша: закрываем закупку и отдаём нажатие игре.
                        self.shop.close();
                        self.pending_key = matrix_pos(code);
                    }
                }
            }
            return;
        }
        if repeat {
            return;
        }
        // Ctrl+Q — выход из игры.
        if code == KeyCode::KeyQ && self.ctrl {
            self.quit = true;
            return;
        }
        // F11 / Alt+Enter — переключение полноэкранного режима.
        if code == KeyCode::F11 || (code == KeyCode::Enter && self.alt) {
            self.toggle_fullscreen();
            return;
        }
        // Меню паузы: своё управление, в эмулятор клавиши не уходят.
        if self.menu {
            self.menu_key(code);
            return;
        }
        // На заставке своя нативная логика: запускает игру только SPACE,
        // стрелки влево/вправо переключают корабль (как в оригинальном
        // интро), остальные клавиши (кроме меню) игнорируются.
        if self.machine.intro_seen {
            if code == KeyCode::Space {
                let layout = save_layout(self.opts);
                native::start_game(&mut self.machine, layout);
            } else if code == KeyCode::ArrowLeft {
                self.splash.cycle_ship(-1);
            } else if code == KeyCode::ArrowRight {
                self.splash.cycle_ship(1);
            } else if code == KeyCode::Escape {
                self.menu = true;
                self.menu_sel = 0;
            }
            return;
        }
        match code {
            KeyCode::Escape => {
                self.menu = true;
                self.menu_sel = 0;
            }
            _ => {}
        }
    }

    /// Обработка клавиш в меню паузы (Esc). Пункты: 0 — продолжить,
    /// 1 — сохранить, 2 — загрузить, 3 — выйти. На заставке сохранение
    /// отключено (игрового процесса ещё нет), загрузка разрешена: данные
    /// ложатся в блок командира, и запуск по `SPACE` начнёт игру с ними.
    fn menu_key(&mut self, code: KeyCode) {
        let intro = self.machine.intro_seen;
        match code {
            KeyCode::Escape => self.menu = false,
            KeyCode::ArrowUp => self.menu_sel = self.menu_sel.saturating_sub(1),
            KeyCode::ArrowDown => self.menu_sel = (self.menu_sel + 1).min(3),
            KeyCode::Digit1 => self.menu_sel = 0,
            KeyCode::Digit2 => self.menu_sel = 1,
            KeyCode::Digit3 => self.menu_sel = 2,
            KeyCode::Digit4 => self.menu_sel = 3,
            KeyCode::Enter | KeyCode::Space => match self.menu_sel {
                0 => self.menu = false,
                1 => {
                    if !intro {
                        do_save(&mut self.machine, self.opts, &mut self.last_path);
                        self.menu = false;
                    }
                }
                2 => {
                    do_load(&mut self.machine, self.opts, &mut self.last_path);
                    self.menu = false;
                }
                _ => self.quit = true,
            },
            _ => {}
        }
    }

    /// Переключить полноэкранный (borderless) режим.
    fn toggle_fullscreen(&mut self) {
        let Some(window) = &self.window else {
            return;
        };
        if window.fullscreen().is_some() {
            window.set_fullscreen(None);
        } else {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
    }

    /// Перенести состояние клавиатуры и джойстика в эмулятор.
    fn apply_input(&mut self) {
        self.machine.release_all_keys();
        self.machine.release_joystick();
        if self.menu || self.hdmap.active || self.shop.active {
            return;
        }
        for (row, bits) in self.matrix.iter().enumerate() {
            for (bit, &down) in bits.iter().enumerate() {
                if down {
                    self.machine.set_key(row, bit as u8, true);
                }
            }
        }
        // Нажатие, которым закрыли нативный экран, подаём один кадр даже если
        // клавишу уже отпустили (иначе быстрый тап игра бы не заметила).
        if let Some((row, bit)) = self.pending_key.take() {
            self.machine.set_key(row, bit, true);
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
        let Some(window) = self.window.clone() else {
            return;
        };
        // Область показа и HD-слой нативного рендера — в разрешении окна.
        // Слой очищается каждый кадр: нативные хуки рисуют в него заново
        // во время эмуляции, показ накладывает слой поверх ZX-кадра.
        let size = window.inner_size();
        let layout = display::layout(size.width, size.height);
        if self.hdmap.active || self.shop.active || self.machine.intro_seen {
            // Отдельный нативный экран (карта/закупка/заставка): HD-слой во
            // всё окно, игра на паузе. Слой непрозрачный и покрывает окно
            // целиком, поэтому композитится в левый верхний угол (0, 0).
            self.machine.hd.resize(size.width as usize, size.height as usize);
            self.machine.hd.clear(display::TRANSPARENT);
            if self.machine.intro_seen {
                // Нативная заставка: вращающаяся тестовая фигура, ZX-кадр не
                // показывается. Эмуляция продолжается, чтобы Z80 шёл по своему
                // циклу (запуск игры делает `native::start_game`).
                self.splash.tick();
                self.machine.release_all_keys();
                self.machine.release_joystick();
                if !self.menu {
                    self.machine.run_frame();
                }
            } else {
                self.machine.release_all_keys();
                self.machine.release_joystick();
            }
        } else {
            // Игровой режим: HD-слой в разрешении области показа; кокпит
            // пока рисуется только на заставке (см. ветку интро выше).
            self.machine
                .hd
                .resize(layout.w as usize, layout.h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
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
        }
        // Сбрасываем однократные события мыши
        self.mouse_wheel_up = false;
        self.mouse_wheel_down = false;
        self.machine.render_rgba(&mut self.buf);
        if self.hdmap.active {
            self.hdmap.draw(&mut self.machine.hd, &self.font);
        } else if self.shop.active {
            self.shop
                .draw(&mut self.machine.hd, &self.font, &self.machine.mem);
        } else if self.machine.intro_seen {
            // Нативная заставка; внизу — демонстрация нативной приборной
            // панели (кокпит). Заголовок с фигурой занимают верхнюю область,
            // поэтому не заходят под панель.
            let g = cockpit::panel_geometry(size.width, size.height);
            self.splash
                .draw_in(&mut self.machine.hd, &self.font, g.panel_y.max(1));
            if self.menu {
                draw_pause_menu(&mut self.machine.hd, &self.font, self.menu_sel, true);
            } else {
                // Показания берутся из памяти Z80 (на заставке они нулевые).
                let cl = cockpit_layout(self.opts);
                self.cockpit
                    .draw(&mut self.machine.hd, &self.machine.mem, &self.font, cl);
            }
        } else if self.menu {
            // Меню паузы — в HD-слой, чтобы текст был чётким в любом окне.
            draw_pause_menu(&mut self.machine.hd, &self.font, self.menu_sel, false);
        }
        self.present(&window, &layout);
    }

    /// Показать кадр: ZX-кадр 256x192 целым множителем `l.scale` по центру
    /// окна (letterbox), поверх — непрозрачные пиксели HD-слоя.
    fn present(&mut self, window: &Rc<Window>, l: &display::Layout) {
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
        let (ow, oh) = (w as usize, h as usize);
        // Фон окна — чёрные поля вокруг области 4:3.
        out.fill(0);
        // Отдельный нативный экран (карта/закупка/заставка) показывается
        // вместо ZX-кадра: слой непрозрачный и покрывает всё окно.
        if self.hdmap.active || self.shop.active || self.machine.intro_seen {
            self.machine.hd.composite_onto(&mut out, ow, oh, 0, 0);
            window.pre_present_notify();
            let _ = out.present();
            return;
        }
        // Целочисленное масштабирование: каждый пиксель ZX — квадрат
        // scale x scale, картинка строго без смаза.
        blit_zx_frame(src, &mut out, ow, oh, l);
        // HD-слой нативного рендера поверх масштабированного ZX-кадра.
        self.machine
            .hd
            .composite_onto(&mut out, ow, oh, l.x as usize, l.y as usize);
        window.pre_present_notify();
        let _ = out.present();
    }
}

/// Скопировать ZX-кадр (`src`, 256x192) в буфер окна целым множителем
/// `l.scale` по центру области показа (letterbox).
///
/// Показывается кадр целиком: и вид из кабины (верхние 128 строк), и родная
/// растровая панель оригинала (нижние 64 строки) — нативная панель из
/// `src/cockpit.rs` в игре не рисуется.
fn blit_zx_frame(src: &[u32], out: &mut [u32], out_w: usize, out_h: usize, l: &display::Layout) {
    let (dw, dh) = ((l.w as usize).min(out_w), (l.h as usize).min(out_h));
    for row in 0..dh {
        let sy = row / l.scale;
        let src_row = &src[sy * SRC_W..sy * SRC_W + SRC_W];
        let dst_lo = (l.y as usize + row) * out_w + l.x as usize;
        let dst_row = &mut out[dst_lo..dst_lo + dw];
        for (col, d) in dst_row.iter_mut().enumerate() {
            *d = src_row[col / l.scale];
        }
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
                self.alt = false;
            }
            WindowEvent::ModifiersChanged(m) => {
                self.ctrl = m.state().contains(ModifiersState::CONTROL);
                self.alt = m.state().contains(ModifiersState::ALT);
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
            WindowEvent::CursorMoved { position, .. } => {
                // При открытой карте мышь наводит её курсор.
                if self.hdmap.active {
                    self.hdmap.point_at(position.x, position.y);
                    return;
                }
                // При открытом экране закупки мышь наводит строку списка.
                if self.shop.active {
                    self.shop.point_at(position.x, position.y);
                    return;
                }
                // Координаты окна -> координаты экрана игры (целый масштаб,
                // та же математика, что в `present`).
                if let Some(w) = &self.window {
                    let size = w.inner_size();
                    let l = display::layout(size.width, size.height);
                    let px = position.x - l.x as f64;
                    let py = position.y - l.y as f64;
                    if px >= 0.0 && px < l.w as f64 && py >= 0.0 && py < l.h as f64 {
                        self.mouse_x = (px / l.scale as f64) as i32;
                        self.mouse_y = (py / l.scale as f64) as i32;
                    } else {
                        self.mouse_x = -1;
                        self.mouse_y = -1;
                    }
                } else {
                    self.mouse_x = position.x as i32;
                    self.mouse_y = position.y as i32;
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                // При открытой карте ЛКМ ставит курсор на ближайшую планету.
                if self.hdmap.active {
                    if button == MouseButton::Left && pressed {
                        self.hdmap.snap_to_nearest();
                    }
                    return;
                }
                // При открытом экране закупки ЛКМ покупает строку/вариант.
                if self.shop.active {
                    if button == MouseButton::Left && pressed {
                        self.shop.click(&mut self.machine.mem);
                    }
                    return;
                }
                match button {
                    MouseButton::Left => self.mouse_left = pressed,
                    MouseButton::Right => self.mouse_right = pressed,
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                if dy == 0.0 {
                    return;
                }
                // При открытой карте колесо меняет масштаб (зум к текущей
                // системе — карта всегда центрирована на ней).
                if self.hdmap.active {
                    self.hdmap.zoom_by(if dy > 0.0 { 1 } else { -1 });
                    return;
                }
                if dy > 0.0 {
                    self.mouse_wheel_up = true;
                } else {
                    self.mouse_wheel_down = true;
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

/// Адрес знакогенератора ZX Spectrum в ПЗУ (коды символов 0x20..0x7F).
const PAUSE_FONT_ADDR: usize = 0x3D00;
/// Длина знакогенератора: 96 символов по 8 байт.
const PAUSE_FONT_LEN: usize = 0x300;

/// Наложение меню паузы в HD-слой: рамка и пункты «1 продолжить /
/// 2 сохранить / 3 загрузить / 4 выйти». Панель задана в пикселях ZX
/// (256x192) и увеличивается до целого масштаба слоя — текст чёткий в любом
/// окне. `sel` — выбранный пункт (0..3) для подсветки, `intro` — заставка
/// (сохранение отключено, поэтому пункт «сохранить» затемнён).
fn draw_pause_menu(frame: &mut display::Frame, font: &[u8], sel: usize, intro: bool) {
    const FG: u32 = 0xFFD7_D7D7; // белый (непрозрачный)
    const BG: u32 = 0xFF00_0000; // чёрный (непрозрачный)
    const DIM: u32 = 0x0080_8080; // затемнённый текст (прозрачный)
    const MARK: u32 = 0xFFFF_FF00; // жёлтый маркер выбора
    let scale = (frame.width / SRC_W).max(1);
    let s = scale as i32;
    let (x0, y0) = (40 * s, 52 * s);
    let (w, h) = (177 * scale, 89 * scale);
    frame.fill_rect(x0, y0, w, h, BG);
    // Рамка: внешняя и внутренняя линии.
    frame.hline(x0, y0, w, FG);
    frame.hline(x0, y0 + h as i32 - s, w, FG);
    frame.vline(x0, y0, h, FG);
    frame.vline(x0 + w as i32 - s, y0, h, FG);
    let lines = [
        (52 + 8, "ELITE - PAUSED"),
        (52 + 24, "1  RESUME"),
        (52 + 36, "2  SAVE"),
        (52 + 48, "3  LOAD"),
        (52 + 60, "4  QUIT GAME"),
    ];
    for (i, (y, text)) in lines.iter().enumerate() {
        // Пункты 1..4 идут после заголовка (i == 0).
        let item = i as isize - 1;
        let dim = intro && item == 1;
        let color = if dim { DIM } else { FG };
        let tx = x0 + 12 * s;
        let ty = *y as i32 * s;
        frame.draw_text(font, tx, ty, text, color, scale);
        if item >= 0 && item as usize == sel {
            frame.fill_rect(x0 + 4 * s, ty, (6 * scale) as usize, 8 * scale, MARK);
        }
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

/// Цифра физической клавиши (верхний ряд или цифровой блок), 0..9.
fn digit_of(code: KeyCode) -> Option<u8> {
    Some(match code {
        KeyCode::Digit0 | KeyCode::Numpad0 => 0,
        KeyCode::Digit1 | KeyCode::Numpad1 => 1,
        KeyCode::Digit2 | KeyCode::Numpad2 => 2,
        KeyCode::Digit3 | KeyCode::Numpad3 => 3,
        KeyCode::Digit4 | KeyCode::Numpad4 => 4,
        KeyCode::Digit5 | KeyCode::Numpad5 => 5,
        KeyCode::Digit6 | KeyCode::Numpad6 => 6,
        KeyCode::Digit7 | KeyCode::Numpad7 => 7,
        KeyCode::Digit8 | KeyCode::Numpad8 => 8,
        KeyCode::Digit9 | KeyCode::Numpad9 => 9,
        _ => return None,
    })
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

/// Нажатие, которое игра использует как управление (матрица ZX или джойстик
/// Kempston). По такой клавише нативный экран закрывается и нажатие уходит
/// в игру — чтобы из карты/закупки можно было сразу перейти к экранам игры.
fn is_game_key(code: KeyCode) -> bool {
    matrix_pos(code).is_some() || kempston_bit(code).is_some()
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
        // В `main` перехват `MenuSaveLoad` ставится всегда — повторяем, иначе
        // проверки сохранения/загрузки из меню паузы в тестах не сработают.
        native::install_save(&mut m, native::ELITE_SAVE);
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

    /// Меню паузы (Esc) рисуется в HD-слой поверх кадра: рамка и текст
    /// шрифтом ПЗУ в полном разрешении (масштаб x5 для слоя 1280x960).
    /// `ELITE_DUMP_MENU=<file>` сохраняет слой в PPM для визуальной проверки.
    #[test]
    fn pause_menu_overlay_renders() {
        let m = boot();
        let mut hd = display::Frame::new(256 * 5, 192 * 5);
        let font = &m.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN];
        draw_pause_menu(&mut hd, font, 0, false);

        const WHITE: u32 = 0xFFD7_D7D7;
        // Угол рамки панели (ZX 40,52 -> HD 200,260) — белый.
        assert_eq!(hd.pixels()[260 * 1280 + 200], WHITE, "panel border not drawn");
        // Внутри панели нарисован текст: заметное число белых пикселей.
        let white = hd.pixels().iter().filter(|&&p| p == WHITE).count();
        assert!(white > 5000, "menu text too sparse: white={white}");
        // Панель непрозрачная (старший байт != 0), фон слоя — прозрачный.
        assert_eq!(hd.pixels()[261 * 1280 + 201], 0xFF00_0000, "panel bg not opaque");
        assert_eq!(hd.pixels()[0], display::TRANSPARENT, "outside panel must stay transparent");

        // Композитинг поверх масштабированного ZX-кадра действительно меняет картинку.
        let mut zx = vec![0u32; 256 * 192];
        m.render_rgba(&mut zx);
        let mut out = vec![0u32; 1280 * 960];
        for y in 0..960 {
            for x in 0..1280 {
                out[y * 1280 + x] = zx[(y / 5) * 256 + x / 5];
            }
        }
        let before = out.clone();
        hd.composite_onto(&mut out, 1280, 960, 0, 0);
        assert!(out != before, "overlay did not change the frame");
        assert_eq!(out[260 * 1280 + 200], WHITE, "overlay not composited");

        if let Ok(path) = env::var("ELITE_DUMP_MENU") {
            write_ppm(&path, &out, 1280, 960);
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
        // F5/F9 removed from UI
        assert!(matrix_pos(KeyCode::F5).is_none());
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

    fn test_options() -> Options {
        Options {
            game: String::new(),
            rom: None,
            headless: true,
            frames: 0,
            dump: None,
            mute: true,
            audio: false,
            wav: None,
            kladov: false,
            native: false,
            save: None,
            fullscreen: false,
        }
    }

    /// В игре показывается полный ZX-кадр: нижние 64 строки (родная растровая
    /// панель оригинала) не обрезаются — нативная панель `src/cockpit.rs` в
    /// игре не рисуется, она осталась только на заставке.
    #[test]
    fn game_screen_shows_full_zx_frame() {
        let (w, h) = (1024usize, 768usize);
        let l = display::layout(w as u32, h as u32);
        assert_eq!(l.scale, 4);
        // Кадр: номер строки ZX записан в младший байт цвета.
        let mut src = vec![0u32; SRC_W * SRC_H];
        for y in 0..SRC_H {
            for x in 0..SRC_W {
                src[y * SRC_W + x] = 0xFF00_0000 | y as u32;
            }
        }
        let mut out = vec![0u32; w * h];
        blit_zx_frame(&src, &mut out, w, h, &l);
        // Верхняя строка кадра — на месте.
        assert_eq!(out[l.y as usize * w + l.x as usize] & 0xFF, 0);
        // Нижняя строка ZX-кадра (191) тоже попала в окно: родная панель
        // оригинала на месте, кадр не обрезан.
        let last = l.y as usize + (SRC_H - 1) * l.scale;
        assert!(last + l.scale <= h, "нижние строки кадра обрезаны");
        assert_eq!(out[last * w + l.x as usize] & 0xFF, (SRC_H - 1) as u32);
    }

    /// F10 открывает отдельную нативную карту галактики: игра на паузе, ввод
    /// уходит карте, повторный F10/Esc возвращает экран игры.
    #[test]
    fn f10_opens_and_closes_galaxy_map() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.hdmap.active);
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active, "F10 не открыл карту");
        assert_eq!(app.hdmap.planet_count(), galaxy::COUNT);
        // Курсор карты стартует на текущей системе (из памяти игры).
        let sys_x = app.machine.mem[galaxy::ELITE_GALAXY.sys_x] as i32;
        let sys_y = app.machine.mem[galaxy::ELITE_GALAXY.sys_y] as i32;
        assert_eq!(app.hdmap.cursor(), (sys_x, sys_y));
        // Стрелка двигает курсор карты, а не корабль.
        app.key_event(KeyCode::ArrowRight, true, false);
        assert_eq!(app.hdmap.cursor().0, (sys_x + 4).min(255));
        // Пока карта открыта, ввод не попадает в эмулятор.
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "ввод не должен уходить в игру");
        // Повторное F10 закрывает карту.
        app.key_event(KeyCode::F10, true, false);
        assert!(!app.hdmap.active, "F10 не закрыл карту");
        // Открыть заново и закрыть по Esc.
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.hdmap.active, "Esc не закрыл карту");
    }

    /// F4 открывает нативный экран закупки: игра на паузе, ввод уходит
    /// экрану (не эмулятору), Esc/F4 возвращают экран игры.
    #[test]
    fn f4_opens_and_closes_equip_shop() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        assert!(!app.shop.active);
        app.key_event(KeyCode::F4, true, false);
        assert!(app.shop.active, "F4 не открыл экран закупки");
        assert!(!app.hdmap.active, "карта галактики не должна открыться");
        // Экран снят данные живого командира (раскладка Elite: `--elite`).
        let p = equip::ELITE_SHOP.pilot as usize;
        let cash = u16::from_le_bytes([app.machine.mem[p + 18], app.machine.mem[p + 19]]) as u32 * 10000
            + u16::from_le_bytes([app.machine.mem[p + 20], app.machine.mem[p + 21]]) as u32;
        assert_eq!(app.shop.cash_tenths(), cash, "деньги не из памяти игры");
        // Пока экран открыт, ввод не попадает в эмулятор.
        app.apply_input();
        assert_eq!(app.machine.kempston, 0, "ввод не должен уходить в игру");
        // Стрелка двигает выбор списка, а не корабль.
        app.key_event(KeyCode::ArrowDown, true, false);
        // Esc закрывает экран (окна крепления не было).
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.shop.active, "Esc не закрыл экран закупки");
        // Повторное открытие и закрытие по F4.
        app.key_event(KeyCode::F4, true, false);
        assert!(app.shop.active);
        app.key_event(KeyCode::F4, true, false);
        assert!(!app.shop.active, "F4 не закрыл экран закупки");
    }

    /// Загрузка версии Кладова: снапшот сразу после запуска — на заставке.
    fn boot_kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
        let mut m = Spectrum::new(Some(&rom));
        m.load(KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = KLADOV_STATE_A;
        c.f = KLADOV_STATE_F;
        c.b = (KLADOV_STATE_BC >> 8) as u8;
        c.c = (KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (KLADOV_STATE_DE >> 8) as u8;
        c.e = (KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (KLADOV_STATE_HL >> 8) as u8;
        c.l = (KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = KLADOV_STATE_IX;
        c.iy = KLADOV_STATE_IY;
        c.sp = KLADOV_STATE_SP;
        c.pc = KLADOV_STATE_PC;
        c.i = KLADOV_I;
        c.r = KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        native::install_save(&mut m, native::KLADOV_SAVE);
        m
    }

    fn kladov_options() -> Options {
        Options {
            kladov: true,
            ..test_options()
        }
    }

    /// На заставке нативные экраны не открываются: F10/F4 игнорируются, пока
    /// игра проигрывает интро (обе версии).
    #[test]
    fn native_screens_blocked_on_intro() {
        // Оригинал: интро-цикл `BeginGame` 0x71A2..0x721F, старт `GameMain`.
        let opts = test_options();
        let mut m = boot();
        let (intro, game) = scene_markers(false);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..600 {
            m.run_frame();
        }
        assert!(m.intro_seen, "интро оригинала не распознано");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(!app.hdmap.active, "F10 не должен открываться на заставке");
        app.key_event(KeyCode::F4, true, false);
        assert!(!app.shop.active, "F4 не должен открываться на заставке");

        // Кладова: аттракт-цикл `sub_728D` 0x739E..0x73FD, старт `GameMain`.
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(
            !app.hdmap.active,
            "F10 не должен открываться на заставке Кладова"
        );
        app.key_event(KeyCode::F4, true, false);
        assert!(
            !app.shop.active,
            "F4 не должен открываться на заставке Кладова"
        );
    }

    /// Сброс флага заставки по входу в `GameMain`: как только игра вошла в
    /// игровой цикл, нативные экраны снова доступны.
    #[test]
    fn native_screens_available_after_game_main() {
        // Кладова: интро взводит флаг, вход в `GameMain` его снимает.
        let opts = kladov_options();
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "сначала должны быть на заставке");
        m.note_pc(game.unwrap());
        assert!(!m.intro_seen, "вход в GameMain не снял флаг заставки");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(
            app.hdmap.active,
            "после старта игры F10 должен открывать карту"
        );
    }

    /// Проверка на живом образе Кладова: пока не нажат SPACE, флаг заставки
    /// держится; после старта игры (вход в `GameMain`) снимается.
    #[test]
    fn kladov_intro_clears_after_space() {
        let mut m = boot_kladov();
        let (intro, game) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "должны быть на заставке");
        m.set_key(7, 0, true); // SPACE
        let mut cleared_at = None;
        for f in 0..3000 {
            m.run_frame();
            if !m.intro_seen {
                cleared_at = Some(f);
                break;
            }
        }
        assert!(cleared_at.is_some(), "SPACE не запустил игру (флаг не снят)");
    }

    /// Точная логика «залипающего» флага заставки.
    #[test]
    fn intro_flag_is_sticky() {
        let mut m = Spectrum::new(None);
        m.intro_range = Some((0x1000, 0x1010));
        m.game_marker = Some(0x2000);
        m.note_pc(0x0800);
        assert!(!m.intro_seen, "флаг не должен взводиться вне диапазонов");
        m.note_pc(0x1005);
        assert!(m.intro_seen, "адрес интро должен взводить флаг");
        m.note_pc(0x0800);
        assert!(m.intro_seen, "флаг заставки должен «залипать»");
        m.note_pc(0x2000);
        assert!(!m.intro_seen, "вход в GameMain должен снимать флаг");
        m.note_pc(0x1010);
        assert!(m.intro_seen, "верхняя граница диапазона включается");
        m.note_pc(0x1011);
        assert!(m.intro_seen, "за верхней границей флаг держится");
    }

    /// На заставке работает только SPACE: остальные клавиши (в т.ч. F10/F4)
    /// игнорируются и в эмулятор не уходят. SPACE запускает игру родным путём.
    #[test]
    fn intro_accepts_only_space() {
        let opts = test_options();
        let mut m = boot();
        let (intro, game) = scene_markers(false);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..30 {
            m.run_frame();
        }
        assert!(m.intro_seen, "интро не распознано");
        let mut app = App::new(m, &opts);
        // Лишние клавиши на заставке ничего не делают.
        app.key_event(KeyCode::F5, true, false);
        app.key_event(KeyCode::F9, true, false);
        app.key_event(KeyCode::F10, true, false);
        app.key_event(KeyCode::F4, true, false);
        app.key_event(KeyCode::Digit1, true, false);
        app.key_event(KeyCode::KeyO, true, false);
        assert!(!app.menu, "лишние клавиши не должны открывать меню");
        assert!(!app.hdmap.active && !app.shop.active);
        assert!(app.machine.intro_seen, "игра должна остаться на заставке");
        // Esc открывает меню паузы (в нём сохранить недоступно).
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu, "Esc на заставке должен открывать меню");
        app.key_event(KeyCode::Escape, true, false);
        assert!(!app.menu, "Esc должен закрывать меню");
        // SPACE запускает игру: хук переводит Z80 на ветку переинициализации,
        // она зовёт `InitGame`, печатает станцию и уходит в `GameMain`.
        native::start_game(&mut app.machine, save_layout(&opts));
        let mut cleared = false;
        for _ in 0..5000 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                cleared = true;
                break;
            }
        }
        assert!(cleared, "SPACE не запустил игру");
    }

    /// Нативная заставка рисуется в HD-слой во всё окно: непрозрачный фон,
    /// заголовок и подсказки, вращающаяся фигура. `ELITE_DUMP_SPLASH=<file>`
    /// сохраняет слой в PPM для визуальной проверки.
    #[test]
    fn splash_overlay_renders() {
        let m = boot();
        let mut hd = display::Frame::new(256 * 5, 192 * 5);
        let font = &m.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN];
        let mut st = splash::Splash::new();
        st.draw(&mut hd, font);
        // Фон полностью непрозрачный.
        assert_eq!(hd.pixels()[0], 0xFF00_0000, "фон заставки не непрозрачный");
        // Заголовок «ELITE» (белый) и подсказка меню (серый) нарисованы.
        let white = hd.pixels().iter().filter(|&&p| p == 0xFFD7_D7D7).count();
        let grey = hd.pixels().iter().filter(|&&p| p == 0xFF90_9090).count();
        assert!(white > 1000, "заголовок заставки не нарисован: {white}");
        assert!(grey > 1000, "подсказка меню не нарисована: {grey}");
        if let Ok(path) = env::var("ELITE_DUMP_SPLASH") {
            write_ppm(&path, hd.pixels(), 256 * 5, 192 * 5);
        }
    }

    /// Нативный кокпит: панель рисуется под заставкой (пока игра не
    /// запущена). `ELITE_DUMP_COCKPIT=<file>` сохраняет заставку вместе с
    /// панелью в PPM для визуальной проверки.
    #[test]
    fn cockpit_overlay_renders() {
        let base = env!("CARGO_MANIFEST_DIR");
        let m = boot();
        let font = &m.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN];
        let (w, h) = (256 * 5, 192 * 5);
        let mut hd = display::Frame::new(w, h);
        let g = cockpit::panel_geometry(w as u32, h as u32);
        // Заставка в верхней области, панель — внизу (как в игровом цикле).
        splash::Splash::new().draw_in(&mut hd, font, g.panel_y.max(1));
        cockpit::Cockpit::new().draw(&mut hd, &m.mem, font, cockpit::ELITE_COCKPIT);
        // Панель непрозрачна.
        for y in g.panel_y..h as i32 {
            for x in 0..w {
                assert_ne!(
                    hd.pixels()[y as usize * w + x],
                    0,
                    "прозрачный пиксель панели ({x},{y})"
                );
            }
        }
        if let Ok(path) = env::var("ELITE_DUMP_COCKPIT") {
            std::fs::create_dir_all(format!("{base}/tmp")).unwrap();
            write_ppm(&path, hd.pixels(), w, h);
        }
    }

    /// На заставке в меню паузы доступна только загрузка: сохранение отключено.
    #[test]
    fn intro_menu_allows_load_but_not_save() {
        let base = env!("CARGO_MANIFEST_DIR");
        let mut opts = test_options();
        opts.save = Some(format!("{base}/tmp/test_intro_load.zxelite"));
        // Создаём минимальный файл для загрузки.
        std::fs::create_dir_all(format!("{base}/tmp")).unwrap();
        std::fs::write(&opts.save.as_ref().unwrap(), vec![0u8; 0x68]).unwrap();

        let mut m = boot();
        let (intro, game) = scene_markers(false);
        m.intro_range = intro;
        m.game_marker = game;
        for _ in 0..600 {
            m.run_frame();
        }
        let mut app = App::new(m, &opts);
        app.machine.intro_seen = true;
        app.machine.save_requested = false;
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu);
        // Пункт «сохранить» (1) на заставке не срабатывает.
        app.menu_sel = 1;
        app.key_event(KeyCode::Enter, true, false);
        assert!(app.menu, "на заставке сохранение не должно закрывать меню");
        assert!(!app.machine.save_requested, "на заставке сохранять нельзя");
        // Пункт «загрузить» (2) работает: просит загрузку у игры.
        app.menu_sel = 2;
        app.key_event(KeyCode::Enter, true, false);
        assert!(!app.menu, "загрузка должна закрыть меню");
        assert!(app.machine.load_requested, "загрузка не запрошена");
    }

    /// Меню паузы в игре: стрелки двигают выбор, Enter по «сохранить»
    /// закрывает меню и просит игру сохраниться.
    #[test]
    fn pause_menu_selection_and_actions() {
        let base = env!("CARGO_MANIFEST_DIR");
        let mut opts = test_options();
        opts.save = Some(format!("{base}/tmp/test_menu_save.zxelite"));
        let mut app = App::new(boot(), &opts);
        app.machine.intro_seen = false;
        app.machine.save_requested = false;
        app.key_event(KeyCode::Escape, true, false);
        assert!(app.menu);
        assert_eq!(app.menu_sel, 0);
        // Стрелка вниз двигает выбор, вверх — назад (не ниже нуля).
        app.key_event(KeyCode::ArrowDown, true, false);
        assert_eq!(app.menu_sel, 1);
        app.key_event(KeyCode::ArrowUp, true, false);
        app.key_event(KeyCode::ArrowUp, true, false);
        assert_eq!(app.menu_sel, 0, "выбор не должен уходить ниже нуля");
        // Цифра задаёт пункт напрямую, Enter по «сохранить» закрывает меню и
        // сохраняет блок командира.
        app.key_event(KeyCode::Digit2, true, false);
        assert_eq!(app.menu_sel, 1);
        app.key_event(KeyCode::Enter, true, false);
        assert!(!app.menu, "сохранение должно закрыть меню");
        assert!(
            std::path::Path::new(&opts.save.as_ref().unwrap()).exists(),
            "файл сохранения не создан"
        );
    }

    /// На заставке Кладова `SPACE` запускает игру: хук на точках цикла
    /// заставки уводит Z80 на ветку переинициализации и снимает `intro_seen`.
    #[test]
    fn kladov_intro_accepts_space() {
        let mut opts = test_options();
        opts.kladov = true;
        let mut m = boot_kladov();
        let (intro, gs) = scene_markers(true);
        m.intro_range = intro;
        m.game_marker = gs;
        for _ in 0..120 {
            m.run_frame();
        }
        assert!(m.intro_seen, "заставка Кладова не распознана");
        let mut app = App::new(m, &opts);
        app.key_event(KeyCode::Space, true, false);
        let mut started = false;
        for _ in 0..300 {
            app.machine.run_frame();
            if !app.machine.intro_seen {
                started = true;
                break;
            }
        }
        assert!(started, "SPACE не запустил игру Кладова");
    }

    /// Меню паузы рисует затемнённый пункт SAVE на заставке.
    #[test]
    fn intro_pause_menu_dims_save() {
        let m = boot();
        let font = &m.mem[PAUSE_FONT_ADDR..PAUSE_FONT_ADDR + PAUSE_FONT_LEN];
        // Цвета пунктов SAVE: затемнённый на заставке и обычный в игре.
        let counts = |intro: bool| {
            let mut hd = display::Frame::new(256 * 5, 192 * 5);
            draw_pause_menu(&mut hd, font, 0, intro);
            let dim = hd.pixels().iter().filter(|&&p| p == 0x0080_8080).count();
            let white = hd.pixels().iter().filter(|&&p| p == 0xFFD7_D7D7).count();
            (dim, white)
        };
        let (dim_on_intro, white_on_intro) = counts(true);
        let (dim_in_game, white_in_game) = counts(false);
        assert!(dim_on_intro > 0, "на заставке пункт SAVE должен быть затемнён");
        assert_eq!(dim_in_game, 0, "в игре SAVE не затемняется");
        assert!(
            white_on_intro < white_in_game,
            "затемнение должно уменьшать белый текст"
        );
    }

    /// Игровая клавиша при открытом нативном экране закрывает его и уходит в
    /// игру — из карты можно сразу перейти к экранам игры, не закрывая карту.
    #[test]
    fn game_key_closes_native_screen_and_forwards() {
        let opts = test_options();
        let mut app = App::new(boot(), &opts);
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        // `L` (матрица 6,1) — игровая клавиша: карта закрывается.
        app.key_event(KeyCode::KeyL, true, false);
        assert!(!app.hdmap.active, "игровая клавиша не закрыла карту");
        app.apply_input();
        assert_eq!(
            app.machine.keyboard[6] & (1 << 1),
            0,
            "нажатие L не передано игре"
        );
        // Управляющая клавиша карты (стрелка) карту не закрывает.
        app.key_event(KeyCode::F10, true, false);
        assert!(app.hdmap.active);
        app.key_event(KeyCode::ArrowRight, true, false);
        assert!(app.hdmap.active, "стрелка не должна закрывать карту");
        // Автоповтор управляющей клавиши тоже не закрывает (иначе Space
        // «залипал» бы в игру как огонь).
        app.key_event(KeyCode::Space, true, false);
        assert!(app.hdmap.active, "Space должен листать карту, а не закрывать");
        app.key_event(KeyCode::Space, true, true);
        assert!(app.hdmap.active, "автоповтор Space не должен закрывать карту");

        // То же для экрана закупки.
        app.key_event(KeyCode::F4, true, false);
        assert!(app.shop.active);
        app.key_event(KeyCode::KeyO, true, false);
        assert!(!app.shop.active, "игровая клавиша не закрыла закупку");
        app.apply_input();
        let (row, bit) = matrix_pos(KeyCode::KeyO).expect("O в матрице");
        assert_eq!(
            app.machine.keyboard[row] & (1 << bit),
            0,
            "нажатие O не передано игре"
        );
    }
}
