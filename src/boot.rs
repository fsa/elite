//! Загрузка образа игры и начальное состояние эмулятора.
//!
//! Версия Кладова грузится как 128K-образ: восемь банков ОЗУ по 16K из
//! `zx_src/kladov128.snapshot`. Образ и ROM 48K встроены в исполняемый файл
//! (`src/assets.rs`), поэтому запуск не зависит от внешних файлов. Образ
//! получен `tools/make_kladov_image.py` и хранится как `zx_bin/kladov128.bin`.
//! По умолчанию запускается GS-версия (`zx_bin/kladov128gs.bin`), собранная
//! `tools/make_kladov_gs_image.py`: это тот же образ состояния игры с патчами
//! General Sound. Точка старта и регистры остаются прежними.
//! Файл `ELITE3  C` на дискете — слепок уже работающей игры, без согласованной
//! точки холодного старта, поэтому мы не проходим загрузчик, а возобновляем
//! исполнение ровно там, где был прерван обработчик прерывания в снапшоте:
//! IM2, `I=0xFD`, обработчик 0xFEFE (таблица векторов уже в образе).

use std::env;

use crate::assets;
use crate::cli::Options;
use crate::machine::Spectrum;
use crate::native;
use crate::world;

pub const KLADOV_I: u8 = 0xFD;
/// Регистры из заголовка SNA-снапшота `kladov128.snapshot`.
pub const KLADOV_STATE_PC: u16 = 0xFEFE;
pub const KLADOV_STATE_SP: u16 = 0x4112;
pub const KLADOV_STATE_AF: u16 = 0x1101;
pub const KLADOV_STATE_BC: u16 = 0x0000;
pub const KLADOV_STATE_DE: u16 = 0x3D08;
pub const KLADOV_STATE_HL: u16 = 0xD045;
pub const KLADOV_STATE_IX: u16 = 0x6159;
pub const KLADOV_STATE_IY: u16 = 0x62B2;
pub const KLADOV_STATE_R: u8 = 0x7D;
pub const KLADOV_STATE_A: u8 = (KLADOV_STATE_AF >> 8) as u8;
pub const KLADOV_STATE_F: u8 = (KLADOV_STATE_AF & 0xFF) as u8;
/// Альтернативный набор регистров (после `EXX`/`EX AF,AF'`).
pub const KLADOV_STATE_AF_ALT: u16 = 0xC99A;
pub const KLADOV_STATE_BC_ALT: u16 = 0x0221;
pub const KLADOV_STATE_DE_ALT: u16 = 0x0060;
pub const KLADOV_STATE_HL_ALT: u16 = 0x0240;

/// Загрузить 128K-образ Кладова в уже созданную машину.
#[cfg(test)]
pub fn load_kladov_image(machine: &mut Spectrum) {
    machine.load_game_image(assets::KLADOV_IMAGE, assets::KLADOV_PAGE_REG);
}

/// Загрузить GS-версию Кладова (используется при обычном запуске игры).
pub fn load_kladov_gs_image(machine: &mut Spectrum) {
    machine.load_game_image(assets::KLADOV_GS_IMAGE, assets::KLADOV_PAGE_REG);
}

/// Выставить регистры из заголовка снапшота, включая альтернативный набор.
pub fn set_kladov_cpu(machine: &mut Spectrum) {
    let c = &mut machine.cpu;
    c.a = KLADOV_STATE_A;
    c.f = KLADOV_STATE_F;
    c.b = (KLADOV_STATE_BC >> 8) as u8;
    c.c = (KLADOV_STATE_BC & 0xFF) as u8;
    c.d = (KLADOV_STATE_DE >> 8) as u8;
    c.e = (KLADOV_STATE_DE & 0xFF) as u8;
    c.h = (KLADOV_STATE_HL >> 8) as u8;
    c.l = (KLADOV_STATE_HL & 0xFF) as u8;
    c.a_ = (KLADOV_STATE_AF_ALT >> 8) as u8;
    c.f_ = (KLADOV_STATE_AF_ALT & 0xFF) as u8;
    c.b_ = (KLADOV_STATE_BC_ALT >> 8) as u8;
    c.c_ = (KLADOV_STATE_BC_ALT & 0xFF) as u8;
    c.d_ = (KLADOV_STATE_DE_ALT >> 8) as u8;
    c.e_ = (KLADOV_STATE_DE_ALT & 0xFF) as u8;
    c.h_ = (KLADOV_STATE_HL_ALT >> 8) as u8;
    c.l_ = (KLADOV_STATE_HL_ALT & 0xFF) as u8;
    c.ix = KLADOV_STATE_IX;
    c.iy = KLADOV_STATE_IY;
    c.sp = KLADOV_STATE_SP;
    c.pc = KLADOV_STATE_PC;
    c.i = KLADOV_I;
    c.r = KLADOV_STATE_R;
    c.im = 2;
    // Как и в прежней точке старта: прерывания включены, чтобы обработчик
    // 0xFEFE продолжал работать после RETN.
    c.iff1 = true;
    c.iff2 = true;
}

/// Настроить машину с загруженной игрой под версию Кладова.
pub fn machine(opts: &Options) -> Spectrum {
    let mut machine = Spectrum::new(Some(assets::ROM_48));
    load_kladov_gs_image(&mut machine);
    set_kladov_cpu(&mut machine);

    if opts.native || env::var("ELITE_NATIVE").is_ok() {
        native::install(&mut machine);
    }

    // Отслеживание заставки: на ней нативные экраны не открываются.
    let (intro, game_start) = world::scene_markers();
    machine.intro_range = intro;
    machine.game_marker = game_start;
    machine.snapshot_marker = Some(world::snapshot_marker());
    machine.transition_snapshot_marker = Some(world::transition_snapshot_marker());
    // Образ снапшота уже содержит `IndDanger = 0`, то есть формально корабль
    // «пристыкован», хотя эмулятор ещё не исполнил ни одного кадра интро.
    // Без явного признака заставки приложение захватило бы управление станцией
    // до первого кадра и заморозило Z80 на заставке. Объявляем её сразу;
    // флаг снимается только входом в `GameMain` по `Space`.
    machine.intro_seen = true;

    // Нативный сейв/лоад командира: конфигурация сохранения должна быть
    // установлена до старта игры, поскольку сразу после входа на станцию
    // игрок может сохраниться.
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
