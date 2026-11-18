//! Загрузка образа игры и начальное состояние эмулятора.
//!
//! Версия Кладова (с дискеты `ELITE3HL.TRD`) грузится как полный 48K-образ
//! памяти (0x4000-0xFFFF). Образ и ROM 48K встроены в исполняемый файл
//! (`src/assets.rs`), поэтому запуск не зависит от внешних файлов. Образ
//! получен `tools/make_kladov_image.py` из снапшота `zx_src/kladov.snapshot` и
//! хранится в репозитории как `zx_bin/kladov.bin`. Файл `ELITE3  C` на дискете
//! — это слепок уже работающей игры, без согласованной точки холодного старта,
//! поэтому мы не проходим загрузчик, а возобновляем исполнение ровно там, где
//! был прерван обработчик прерывания в снапшоте:
//! IM2, `I=0xFD`, обработчик 0xFEFE (таблица векторов уже в образе).

use std::env;

use crate::assets;
use crate::cli::Options;
use crate::machine::Spectrum;
use crate::native;
use crate::world;

/// Адрес загрузки полного образа версии Кладова.
pub const KLADOV_LOAD_ADDR: u16 = 0x4000;
pub const KLADOV_I: u8 = 0xFD;
pub const KLADOV_STATE_PC: u16 = 0xFEFE;
pub const KLADOV_STATE_SP: u16 = 0x4114;
pub const KLADOV_STATE_AF: u16 = 0x0045;
pub const KLADOV_STATE_BC: u16 = 0x0012;
pub const KLADOV_STATE_DE: u16 = 0x482E;
pub const KLADOV_STATE_HL: u16 = 0xC80E;
pub const KLADOV_STATE_IX: u16 = 0x6159;
pub const KLADOV_STATE_IY: u16 = 0x629B;
pub const KLADOV_STATE_R: u8 = 0x3E;
pub const KLADOV_STATE_A: u8 = (KLADOV_STATE_AF >> 8) as u8;
pub const KLADOV_STATE_F: u8 = (KLADOV_STATE_AF & 0xFF) as u8;

/// Настроить машину с загруженной игрой под версию Кладова.
pub fn machine(opts: &Options) -> Spectrum {
    let mut machine = Spectrum::new(Some(assets::ROM_48));
    // Версия Кладова: полный 48K-образ (0x4000..0xFFFF) загружается как есть.
    machine.load(KLADOV_LOAD_ADDR, assets::KLADOV_IMAGE);
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

    if opts.native || env::var("ELITE_NATIVE").is_ok() {
        native::install(&mut machine);
    }

    // Отслеживание заставки: на ней нативные экраны не открываются.
    let (intro, game_start) = world::scene_markers();
    machine.intro_range = intro;
    machine.game_marker = game_start;

    // Нативный сейв/лоад командира
    native::install_save(&mut machine, world::save_layout_kladov());

    // Перехват готового текста нижней строки: события уходят в нативный вид
    // полёта.
    native::install_text_capture(&mut machine);

    install_debug_env(&mut machine);
    machine
}

/// Настройка отладочных перехватов из переменных окружения.
fn install_debug_env(machine: &mut Spectrum) {
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
}
