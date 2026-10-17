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
use crate::z80::{Z80, ZF};

/// Установить все нативные замены (хуки) на машину.
pub fn install(m: &mut Spectrum) {
    m.hooks.insert(0x5DFB, font_base_hook);
    m.hooks.insert(0xEB3E, eb3e_hook);
    m.hooks.insert(0xD3CE, d3ce_hook);
    m.hooks.insert(0xAB0B, mult_a_hib_c_hook);
}

/// Адрес блока данных командира (`PilotName`) в образе Кладова.
pub const PILOT_NAME: u16 = 0xD232;
/// Длина сохраняемого блока командира: 102 байта (`ld bc,102`).
pub const COMMANDER_LEN: usize = 0x66;
/// Точка входа родной рутины сохранения/загрузки игры (`MenuSaveLoad`).
///
/// Вызывается из главного цикла только при нажатии игровой клавиши
/// сохранения (`0x749F: call 0xD04E`), поэтому перехват не срабатывает
/// каждый кадр. Далее рутина прыгает на `0xD907` и уходит в TR-DOS
/// (`jp 0x3D2F`) с пейджингом 128K — в 48K-ядре это не работает.
const MENU_SAVE_LOAD: u16 = 0xD04E;

/// Блок сохранения командира в памяти.
pub fn commander_block(m: &Spectrum) -> &[u8] {
    let a = PILOT_NAME as usize;
    &m.mem[a..a + COMMANDER_LEN]
}

/// Записать блок командира в файл `path`.
pub fn commander_save(m: &Spectrum, path: &str) -> std::io::Result<()> {
    std::fs::write(path, commander_block(m))
}

/// Прочитать блок командира из файла `path` и поместить в память.
pub fn commander_load(m: &mut Spectrum, path: &str) -> std::io::Result<()> {
    let data = std::fs::read(path)?;
    let n = data.len().min(COMMANDER_LEN);
    let a = PILOT_NAME as usize;
    m.mem[a..a + n].copy_from_slice(&data[..n]);
    Ok(())
}

/// Включить перехват игрового сохранения: при вызове `MenuSaveLoad` игра
/// выставляет флаг `machine.save_requested`, а главный цикл показывает диалог
/// выбора файла и записывает блок командира (родная рутина требует
/// 128K-пейджинга и в 48K-ядре не работает).
pub fn install_save(m: &mut Spectrum) {
    m.hooks.insert(MENU_SAVE_LOAD, menu_save_load_hook);
}

/// Нативная замена `MenuSaveLoad` (`0xD04E`): просит главный цикл сохранить
/// блок командира (диалог выбора файла). Возвращает «не загружено» (A=0, Z=1),
/// как и родная рутина при обычном возврате, чтобы главный цикл не уходил на
/// ветку переинициализации вида (`jp nz, 0x7394`).
fn menu_save_load_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    m.save_requested = true;
    cpu.a = 0;
    cpu.f |= ZF;
    0
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

/// `sub_EB3E`: `ld d,0; ld e,a; add hl,de; ret` — прибавляет к `HL`
/// беззнаковый `A` (через `DE`), выставляя флаги сложения. Флаги `S/Z/PV`
/// 16-битного `ADD HL,rr` не трогает.
fn eb3e_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    cpu.d = 0;
    cpu.e = cpu.a;
    let r = cpu.add16(cpu.hl(), cpu.de());
    cpu.h = (r >> 8) as u8;
    cpu.l = r as u8;
    32 // ld d,0(7) + ld e,a(4) + add hl,de(11) + ret(10)
}

/// `sub_D3CE`: `push hl; sbc hl,de; pop hl; ret` — выставляет флаги по
/// `HL - DE - C` (16-битное вычитание с заёмом), но `HL` оставляет прежним.
fn d3ce_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = cpu.hl();
    let de = cpu.de();
    cpu.sbc16(hl, de);
    46 // push hl(11) + sbc hl,de(15) + pop hl(10) + ret(10)
}

/// Таблица умножения (`0xBF00`), к которой обращается `Mult_A_hiB_C`.
const MULT_TABLE: usize = 0xBF00;

/// Нативная замена `Mult_A_hiB_C` (`0xAB0B`).
///
/// Табличное умножение через таблицу квадратов по адресу `0xBF00`.
///
/// ```text
/// ld a,b ; sub c ; jr nc,keep ; neg ; keep:
/// srl a ; ld h,0BFh ; ld l,a ; ld d,(hl)
/// ld a,b ; add a,c ; rra ; ld l,a ; ld a,(hl) ; sub d ; ret
/// ```
///
/// Вход: `B`, `C`. Выход: `A = table[(B+C)/2] - table[|B-C|/2]`, `H=0xBF`,
/// `L=(B+C)/2`, `D=table[|B-C|/2]`. `B`, `C` не изменяются.
fn mult_a_hib_c_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let (b, c) = (cpu.b, cpu.c);
    let no_borrow = b >= c; // результат `sub c`: CF=0
    let diff = if no_borrow { b - c } else { c - b };
    let d = m.mem[MULT_TABLE + (diff >> 1) as usize];
    let idx = ((b as u16 + c as u16) >> 1) as u8; // `add a,c` + `rra`
    let a2 = m.mem[MULT_TABLE + idx as usize];
    cpu.a = cpu.sub8(a2, d, 0);
    cpu.h = 0xBF;
    cpu.l = idx;
    cpu.d = d;
    // `jr nc` — taken при отсутствии заёма (b >= c), тогда `neg` не выполняется.
    if no_borrow { 83 } else { 86 }
}

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

    /// Снимок регистров и тактов для строгого сравнения.
    #[derive(Debug, PartialEq)]
    struct Snap {
        a: u8,
        f: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        h: u8,
        l: u8,
        cycles: u64,
    }

    fn snap(c: &Z80) -> Snap {
        Snap {
            a: c.a,
            f: c.f,
            b: c.b,
            c: c.c,
            d: c.d,
            e: c.e,
            h: c.h,
            l: c.l,
            cycles: c.cycles,
        }
    }

    fn set(c: &mut Z80, a: u8, hl: u16, de: u16, f: u8) {
        c.a = a;
        c.f = f;
        c.h = (hl >> 8) as u8;
        c.l = hl as u8;
        c.d = (de >> 8) as u8;
        c.e = de as u8;
    }

    /// Прогон рутины на Z80 со счётчиком тактов от нуля.
    fn z80_sub(m: &mut Spectrum, addr: u16) -> Snap {
        m.cpu.cycles = 0;
        run_routine(m, addr);
        snap(&m.cpu)
    }

    /// Прогон нативной рутины через таблицу хуков.
    fn hook_sub(m: &mut Spectrum, addr: u16) -> Snap {
        let mut cpu = std::mem::take(&mut m.cpu);
        cpu.cycles = 0;
        let cyc = (m.hooks[&addr])(&mut cpu, m);
        cpu.cycles = cyc;
        m.cpu = cpu;
        snap(&m.cpu)
    }

    /// Регистровые рутины обязаны совпасть с Z80 по ВСЕМ регистрам, флагам
    /// и числу тактов.
    #[test]
    fn register_helpers_match_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n);
        let cases: &[(u8, u16, u16, u8)] = &[
            (0x00, 0x0000, 0x0000, 0x00),
            (0x01, 0x0001, 0x0000, 0x00),
            (0xFF, 0x00FF, 0x0001, 0x01),
            (0x80, 0xFF00, 0x00FF, 0x00),
            (0x7F, 0x0FFF, 0x0001, 0x00),
            (0x55, 0x8000, 0x8000, 0x01),
            (0xAA, 0x1234, 0x5678, 0x00),
            (0x00, 0xFFFF, 0x0001, 0x01),
            (0x03, 0x00FF, 0xFF00, 0x00),
        ];
        for &(a, hl, de, f) in cases {
            for &addr in &[0xEB3Eu16, 0xD3CEu16] {
                set(&mut z.cpu, a, hl, de, f);
                set(&mut n.cpu, a, hl, de, f);
                let zs = z80_sub(&mut z, addr);
                let ns = hook_sub(&mut n, addr);
                assert_eq!(
                    zs, ns,
                    "рутина 0x{addr:04X}: вход a={a:02X} hl={hl:04X} de={de:04X} f={f:02X}"
                );
            }
        }
    }

    /// `Mult_A_hiB_C` обязан совпасть с Z80 на всех 256×256 парах `B`,`C`
    /// по всем регистрам, флагам и числу тактов.
    #[test]
    fn mult_a_hib_c_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n);
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                z.cpu.b = b;
                z.cpu.c = c;
                z.cpu.f = b ^ c;
                n.cpu.b = b;
                n.cpu.c = c;
                n.cpu.f = b ^ c;
                let zs = z80_sub(&mut z, 0xAB0B);
                let ns = hook_sub(&mut n, 0xAB0B);
                assert_eq!(zs, ns, "Mult_A_hiB_C: b={b:02X} c={c:02X}");
            }
        }
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

    /// Сохранение и загрузка блока командира в файл — round-trip.
    #[test]
    fn commander_roundtrip() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        std::fs::create_dir_all(&dir).unwrap();
        let path = format!("{dir}/commander_roundtrip.zxelite");

        let mut m = kladov();
        let a = PILOT_NAME as usize;
        m.mem[a] = b'T';
        m.mem[a + 1] = b'E';
        m.mem[a + 2] = b'S';
        m.mem[a + 3] = b'T';
        m.mem[a + COMMANDER_LEN - 1] = 0x5A;
        let expected: Vec<u8> = commander_block(&m).to_vec();

        commander_save(&m, &path).unwrap();
        for b in &mut m.mem[a..a + COMMANDER_LEN] {
            *b = 0xAA;
        }
        commander_load(&mut m, &path).unwrap();
        assert_eq!(commander_block(&m), &expected[..], "блок командира не совпал");

        let _ = std::fs::remove_file(&path);
    }

    /// Перехват `MenuSaveLoad`: хук должен выставить запрос на сохранение.
    #[test]
    fn menu_save_load_hook_sets_request() {
        let mut m = kladov();
        install_save(&mut m);
        assert!(
            m.hooks.contains_key(&MENU_SAVE_LOAD),
            "хук MenuSaveLoad не установлен"
        );
        assert!(!m.save_requested);
        let hook = m.hooks.get(&MENU_SAVE_LOAD).copied().unwrap();
        let mut cpu = std::mem::take(&mut m.cpu);
        let cycles = hook(&mut cpu, &mut m);
        m.cpu = cpu;
        assert_eq!(cycles, 0);
        assert!(m.save_requested, "запрос на сохранение не выставлен");
    }
}
