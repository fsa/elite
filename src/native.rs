//! Нативные (Rust) реализации подпрограмм игры.
//!
//! Каждая функция здесь повторяет поведение конкретной рутины из игры
//! (адрес указан в комментарии). Цель — постепенно заменить выполнение этих
//! рутин на Z80, вплоть до полного отказа от эмуляции.
//!
//! Инвариант: поведение обязано совпадать с исходной рутиной. Для каждой
//! функции есть тест эквивалентности (см. `mod tests`), который прогоняет
//! настоящий код из образа `zx_bin/kladov.bin` на эмуляторе и сверяет
//! результат с нативной реализацией.

use crate::machine::Spectrum;
use crate::z80::Z80;

/// Установить все нативные замены (хуки) на машину.
pub fn install(m: &mut Spectrum) {
    m.hooks.insert(0x5DFB, font_base_hook);
}

/// Точное число тактов Z80, которое рутина `sub_5DFB` занимает для кода `a`
/// (включая завершающий `RET`).
fn font_base_cycles(a: u8) -> u64 {
    match a {
        0x20 => 28,           // пробел: cp 20h -> ret z (taken)
        _ if a < 0x30 => 61,  // < '0'
        _ if a < 0x3B => 54,  // '0'..='9'
        _ if a < 0x41 => 87,  // ':'..'@'
        _ if a < 0x5B => 80,  // 'A'..'Z'
        _ if a < 0x61 => 113, // '['..'`'
        _ if a < 0x7B => 106, // a..z
        _ => 120,             // >= '{'
    }
}

/// Нативная замена `sub_5DFB` для механизма хуков.
fn font_base_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    #[cfg(test)]
    FONT_BASE_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let hl = font_base(cpu.a);
    cpu.h = (hl >> 8) as u8;
    cpu.l = (hl & 0xFF) as u8;
    font_base_cycles(cpu.a)
}

/// Счётчик обращений к хуку (только для тестов).
#[cfg(test)]
static FONT_BASE_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `sub_5DFB`: по коду символа в `A` выбирает базовый адрес шрифта.
///
/// Возвращает значение `HL`: `0x3D00` для «печатных» символов (пробел, цифры
/// `0`–`9`, латиница `A`–`Z`/`a`–`z`) и `0x5D70` для остальных. Регистр `A` не
/// изменяется.
pub fn font_base(a: u8) -> u16 {
    let printable = a == 0x20
        || (0x30..=0x3A).contains(&a)
        || (0x41..=0x5A).contains(&a)
        || (0x61..=0x7A).contains(&a);
    if printable { 0x3D00 } else { 0x5D70 }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Загружает полный образ Кладова (для прогона настоящих рутин на Z80).
    fn kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let data = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(None);
        m.load(0x4000, &data);
        m
    }

    /// Вызывает рутину по адресу `addr` как подпрограмму (`CALL`/`RET`) и
    /// выполняет её на Z80 до возврата в адрес-маркер.
    fn run_routine(m: &mut Spectrum, addr: u16) {
        const RET: u16 = 0x1234;
        m.cpu.sp = 0xF000;
        m.cpu.sp = m.cpu.sp.wrapping_sub(2);
        let [lo, hi] = RET.to_le_bytes();
        m.mem[m.cpu.sp as usize] = lo;
        m.mem[m.cpu.sp as usize + 1] = hi;
        m.cpu.pc = addr;
        let mut cpu = std::mem::take(&mut m.cpu);
        let mut guard = 0;
        while cpu.pc != RET && guard < 10_000 {
            cpu.step(m);
            guard += 1;
        }
        assert_eq!(cpu.pc, RET, "рутина 0x{addr:04X} не вернулась (guard={guard})");
        m.cpu = cpu;
    }

    /// Готовит машину к запуску версии Кладова так же, как `main.rs`.
    fn boot_kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = crate::KLADOV_STATE_A;
        c.f = crate::KLADOV_STATE_F;
        c.b = (crate::KLADOV_STATE_BC >> 8) as u8;
        c.c = (crate::KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (crate::KLADOV_STATE_DE >> 8) as u8;
        c.e = (crate::KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (crate::KLADOV_STATE_HL >> 8) as u8;
        c.l = (crate::KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = crate::KLADOV_STATE_IX;
        c.iy = crate::KLADOV_STATE_IY;
        c.sp = crate::KLADOV_STATE_SP;
        c.pc = crate::KLADOV_STATE_PC;
        c.i = crate::KLADOV_I;
        c.r = crate::KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        m
    }

    /// Полный прогон с хуками обязан побайтово совпасть с обычным прогоном:
    /// такты, регистры и содержимое экранной области.
    #[test]
    fn hooks_match_plain_run() {
        FONT_BASE_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);
        let mut plain = boot_kladov();
        let mut hooked = boot_kladov();
        install(&mut hooked);
        for _ in 0..120 {
            plain.run_frame();
            hooked.run_frame();
        }
        assert!(
            FONT_BASE_CALLS.load(std::sync::atomic::Ordering::Relaxed) > 0,
            "хук font_base ни разу не вызван за 120 кадров"
        );
        assert_eq!(plain.cpu.cycles, hooked.cpu.cycles, "такт разошёлся");
        assert_eq!(plain.cpu.pc, hooked.cpu.pc, "PC разошёлся");
        assert_eq!(plain.cpu.sp, hooked.cpu.sp, "SP разошёлся");
        assert_eq!(plain.cpu.hl(), hooked.cpu.hl(), "HL разошёлся");
        assert_eq!(plain.cpu.bc(), hooked.cpu.bc(), "BC разошёлся");
        assert_eq!(plain.cpu.de(), hooked.cpu.de(), "DE разошёлся");
        assert_eq!(
            &plain.mem[0x4000..0x5B00],
            &hooked.mem[0x4000..0x5B00],
            "экран разошёлся"
        );
    }

    #[test]
    fn font_base_matches_z80() {
        let mut m = kladov();
        for a in 0u8..=255 {
            m.cpu.a = a;
            run_routine(&mut m, 0x5DFB);
            let z80 = m.cpu.hl();
            let native = font_base(a);
            assert_eq!(
                native, z80,
                "font_base(0x{a:02X}): native=0x{native:04X} z80=0x{z80:04X}"
            );
        }
    }
}
