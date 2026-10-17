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
    use crate::machine::Spectrum;

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
