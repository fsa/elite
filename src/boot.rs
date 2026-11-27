//! Загрузка образа игры и начальное состояние эмулятора.
//!
//! Обычный запуск — холодный старт с готового 128K-образа. `tools/make_cold_image.py`
//! один раз применяет загрузчик с `Elite3GS.trd` офлайн и пересобирает
//! `zx_bin/kladov128gs.bin`. В рантайме образ просто кладётся в ОЗУ, выставляются
//! регистры точки входа `0x7394` и игра получает управление. TR-DOS-диски в
//! исполняемый файл не входят и во время работы не читаются; единственное
//! исключение — импорт 102 байт сохранения commander-а.
//!
//! Подробности — в `docs/architecture/game-integration.md`.

use std::env;

use crate::assets;
use crate::cli::Options;
use crate::machine::Spectrum;
use crate::native;
use crate::world;

/// Точка входа в игру, куда приходит готовый загрузчик после `OUT (C),A; RET`.
pub const GAME_ENTRY: u16 = 0x7394;
/// Порт пейджинга в момент старта: банк 0 по `0xC000`, экран из банка 5.
const COLD_PAGE_REG: u8 = 0x10;
/// Регистры на входе в `0x7394` после однократной работы загрузчика GS-диска.
/// Они сохраняются в образе холодного старта, чтобы обычный запуск не исполнял загрузчик.
const COLD_AF: u16 = 0x1041;
const COLD_BC: u16 = 0x7FFD;
const COLD_DE: u16 = 0x5B00;
const COLD_HL: u16 = 0xFC94;
const COLD_SP: u16 = 0x4120;
const COLD_IX: u16 = 0x6170;
const COLD_IY: u16 = 0xFFFF;
const COLD_I: u8 = 0xFD;
const COLD_R: u8 = 0x0A;

#[cfg(test)]
const KLADOV_I: u8 = 0xFD;
/// Регистры из заголовка SNA-снапшота `kladov128.snapshot`.
#[cfg(test)]
const KLADOV_STATE_PC: u16 = 0xFEFE;
#[cfg(test)]
const KLADOV_STATE_SP: u16 = 0x4112;
#[cfg(test)]
const KLADOV_STATE_AF: u16 = 0x1101;
#[cfg(test)]
const KLADOV_STATE_BC: u16 = 0x0000;
#[cfg(test)]
const KLADOV_STATE_DE: u16 = 0x3D08;
#[cfg(test)]
const KLADOV_STATE_HL: u16 = 0xD045;
#[cfg(test)]
const KLADOV_STATE_IX: u16 = 0x6159;
#[cfg(test)]
const KLADOV_STATE_IY: u16 = 0x62B2;
#[cfg(test)]
const KLADOV_STATE_R: u8 = 0x7D;
#[cfg(test)]
const KLADOV_STATE_A: u8 = (KLADOV_STATE_AF >> 8) as u8;
#[cfg(test)]
const KLADOV_STATE_F: u8 = (KLADOV_STATE_AF & 0xFF) as u8;
/// Альтернативный набор регистров (после `EXX`/`EX AF,AF'`).
#[cfg(test)]
const KLADOV_STATE_AF_ALT: u16 = 0xC99A;
#[cfg(test)]
const KLADOV_STATE_BC_ALT: u16 = 0x0221;
#[cfg(test)]
const KLADOV_STATE_DE_ALT: u16 = 0x0060;
#[cfg(test)]
const KLADOV_STATE_HL_ALT: u16 = 0x0240;

/// Загрузить 128K-образ Кладова в уже созданную машину.
#[cfg(test)]
pub fn load_kladov_image(machine: &mut Spectrum) {
    machine.load_game_image(assets::KLADOV_IMAGE, assets::KLADOV_PAGE_REG);
}

/// Выставить регистры из заголовка снапшота, включая альтернативный набор.
#[cfg(test)]
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

/// Выставить регистры готового холодного старта.
///
/// `ei_pending` остаётся взведённым: загрузчик исполнил `EI`, но ни одной
/// команды после него до точки входа ещё не было. Отладочные такты обнуляются,
/// чтобы первые кадры после старта не съедали время загрузчика.
fn set_cold_cpu(machine: &mut Spectrum) {
    let c = &mut machine.cpu;
    c.a = (COLD_AF >> 8) as u8;
    c.f = (COLD_AF & 0xFF) as u8;
    c.b = (COLD_BC >> 8) as u8;
    c.c = (COLD_BC & 0xFF) as u8;
    c.d = (COLD_DE >> 8) as u8;
    c.e = (COLD_DE & 0xFF) as u8;
    c.h = (COLD_HL >> 8) as u8;
    c.l = (COLD_HL & 0xFF) as u8;
    // Альтернативный набор после загрузчика остался нулевым.
    c.a_ = 0;
    c.f_ = 0;
    c.b_ = 0;
    c.c_ = 0;
    c.d_ = 0;
    c.e_ = 0;
    c.h_ = 0;
    c.l_ = 0;
    c.ix = COLD_IX;
    c.iy = COLD_IY;
    c.sp = COLD_SP;
    c.pc = GAME_ENTRY;
    c.i = COLD_I;
    c.r = COLD_R;
    c.im = 2;
    c.iff1 = true;
    c.iff2 = true;
    c.halted = false;
    c.ei_pending = true;
    c.index_iy = false;
    c.cycles = 0;
    machine.frame_start = 0;
}

/// Настроить машину с загруженной игрой под версию Кладова.
pub fn machine(opts: &Options) -> Spectrum {
    let mut machine = Spectrum::new(Some(assets::ROM_48));

    // Готовый образ уже содержит результат работы загрузчика. Нативные хуки и
    // маркеры сцены ставятся после загрузки ОЗУ: образ записал бы их поверх.
    machine.load_game_image(assets::COLD_IMAGE, COLD_PAGE_REG);
    set_cold_cpu(&mut machine);

    if opts.native || env::var("ELITE_NATIVE").is_ok() {
        native::install(&mut machine);
    }

    // Отслеживание заставки: на ней нативные экраны не открываются.
    let (intro, game_start) = world::scene_markers();
    machine.intro_range = intro;
    machine.game_marker = game_start;
    machine.snapshot_marker = Some(world::snapshot_marker());
    machine.transition_snapshot_marker = Some(world::transition_snapshot_marker());
    // Образ приведён к точке `0x7394`, но ни одного кадра интро ещё не
    // исполнено. Без явного признака заставки приложение захватило бы
    // управление станцией до первого кадра и заморозило Z80. Флаг снимается
    // только входом в `GameMain` по `Space`.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Готовый образ действительно содержит разложенный по банкам код игры,
    /// а не пустую RAM с точкой входа `0x7394` случайным образом.
    #[test]
    fn cold_image_places_game_code() {
        let mut machine = Spectrum::new(Some(assets::ROM_48));
        machine.load_game_image(assets::COLD_IMAGE, COLD_PAGE_REG);

        // Заглушка `OUT (C),A; RET` по `0x4000` возвращает в `0x7394`.
        assert_eq!(&machine.mem[0x4000..0x4003], &[0xED, 0x79, 0xC9]);
        assert_eq!(&machine.mem[0x411E..0x4120], &[0x94, 0x73]);
        // Начало интро и код в банках 2 и 0.
        assert_eq!(&machine.mem[0x7394..0x7398], &[0xAF, 0x32, 0xE3, 0x70]);
        assert_eq!(machine.mem[0x8000], 0x38);
        // Код банка 0, который загрузчик положил по `0xC000..0xFFFF`.
        assert_eq!(machine.mem[0xD100], 0xF1);
        // Хвостовой код загрузчика остался в банке 3.
        assert_eq!(assets::COLD_IMAGE[0xC000], 0xED);
        // Хвостовой блок загрузчика перенесён в `0x5B00`.
        assert_eq!(machine.mem[0x5B00], 0x07);
    }

    /// Старт всегда приходит в `0x7394` с зафиксированным состоянием CPU.
    #[test]
    fn cold_start_cpu_state() {
        let mut machine = Spectrum::new(Some(assets::ROM_48));
        set_cold_cpu(&mut machine);
        let c = &machine.cpu;

        assert_eq!(c.pc, GAME_ENTRY);
        assert_eq!(c.sp, 0x4120);
        assert_eq!((c.a as u16) << 8 | c.f as u16, 0x1041);
        assert_eq!(c.bc(), 0x7FFD);
        assert_eq!(c.de(), 0x5B00);
        assert_eq!(c.hl(), 0xFC94);
        assert_eq!(c.ix, 0x6170);
        assert_eq!(c.iy, 0xFFFF);
        assert_eq!(c.i, 0xFD);
        assert_eq!(c.r, 0x0A);
        assert_eq!(c.im, 2);
        assert!(c.iff1 && c.iff2);
        assert!(c.ei_pending);
        assert_eq!(c.cycles, 0);
    }
}
