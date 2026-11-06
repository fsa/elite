//! Тесты эквивалентности: нативные реализации против настоящих рутин
//! из образа `zx_bin/kladov.bin`, исполняемых на эмуляторе.

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
    run_routine_sp(m, addr, 0xF000u16.wrapping_sub(2));
}

/// То же, но со своим `SP`: стековый маркер ложится по нему.
fn run_routine_sp(m: &mut Spectrum, addr: u16, sp: u16) {
    const RET: u16 = 0x1234;
    m.cpu.sp = sp;
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
    m.load(crate::boot::KLADOV_LOAD_ADDR, &game);
    let c = &mut m.cpu;
    c.a = crate::boot::KLADOV_STATE_A;
    c.f = crate::boot::KLADOV_STATE_F;
    c.b = (crate::boot::KLADOV_STATE_BC >> 8) as u8;
    c.c = (crate::boot::KLADOV_STATE_BC & 0xFF) as u8;
    c.d = (crate::boot::KLADOV_STATE_DE >> 8) as u8;
    c.e = (crate::boot::KLADOV_STATE_DE & 0xFF) as u8;
    c.h = (crate::boot::KLADOV_STATE_HL >> 8) as u8;
    c.l = (crate::boot::KLADOV_STATE_HL & 0xFF) as u8;
    c.ix = crate::boot::KLADOV_STATE_IX;
    c.iy = crate::boot::KLADOV_STATE_IY;
    c.sp = crate::boot::KLADOV_STATE_SP;
    c.pc = crate::boot::KLADOV_STATE_PC;
    c.i = crate::boot::KLADOV_I;
    c.r = crate::boot::KLADOV_STATE_R;
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
    r: u8,
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
        r: c.r,
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

/// Прогон рутины на Z80 со своим `SP`.
fn z80_sub_sp(m: &mut Spectrum, addr: u16, sp: u16) -> Snap {
    m.cpu.cycles = 0;
    run_routine_sp(m, addr, sp);
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

/// `Mult_HL_B_C` (`0xAB34`) обязан совпасть с Z80 на всех 256×256 парах
/// `B`,`C` по всем регистрам, флагам и числу тактов (16-битный результат
/// табличного умножения через плоскости 0xBE00/0xBF00).
#[test]
fn mult_hl_b_c_matches_z80() {
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
            let zs = z80_sub(&mut z, 0xAB34);
            let ns = hook_sub(&mut n, 0xAB34);
            assert_eq!(zs, ns, "Mult_HL_B_C: b={b:02X} c={c:02X}");
        }
    }
}

/// `sub_AB1E` (хелпер `RotateCrdN_Vertex`) обязан совпасть с Z80 по всем
/// регистрам, флагам и тактам: сдвиг `B` на `VertexIncFactor` бит и
/// табличное умножение на `(iy+1) & 7Fh`.
#[test]
fn sub_ab1e_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    let inc_factors = [0u8, 1, 2, 3, 7, 8, 9, 255];
    let iy1s = [0x00u8, 0x3F, 0x40, 0x7F, 0x80, 0xFF];
    let run = |z: &mut Spectrum, n: &mut Spectrum, b: u8| {
        z.cpu.b = b;
        n.cpu.b = b;
        let zs = z80_sub(z, 0xAB1E);
        let ns = hook_sub(n, 0xAB1E);
        assert_eq!(
            zs, ns,
            "sub_AB1E: inc={} b={b:02X} iy+1={:02X}",
            z.mem[VERTEX_INC_FACTOR],
            (z.cpu.iy as u8).wrapping_add(1)
        );
    };
    for &nf in &inc_factors {
        z.mem[VERTEX_INC_FACTOR] = nf;
        n.mem[VERTEX_INC_FACTOR] = nf;
        for &iy1 in &iy1s {
            z.cpu.iy = 0xD2F0;
            n.cpu.iy = 0xD2F0;
            z.mem[0xD2F1] = iy1;
            n.mem[0xD2F1] = iy1;
            for b in 0u8..=255 {
                run(&mut z, &mut n, b);
            }
        }
    }
    // Полный обход (B, IY+1) при одном сдвиге.
    z.mem[VERTEX_INC_FACTOR] = 1;
    n.mem[VERTEX_INC_FACTOR] = 1;
    for iy1 in 0u8..=255 {
        z.cpu.iy = 0xD2F0;
        n.cpu.iy = 0xD2F0;
        z.mem[0xD2F1] = iy1;
        n.mem[0xD2F1] = iy1;
        for b in 0u8..=255 {
            run(&mut z, &mut n, b);
        }
    }
}

/// `sub_E621` (`PixCrd2ScrAdr`) обязан совпасть с Z80 по всем регистрам,
/// флагам и тактам для всех пар `X` (A) и `Y` (C).
#[test]
fn pix_crd_2_scr_adr_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    for x in 0u8..=255 {
        for y in 0u8..=255 {
            z.cpu.a = x;
            z.cpu.c = y;
            z.cpu.f = x ^ y; // произвольные входные флаги
            n.cpu.a = x;
            n.cpu.c = y;
            n.cpu.f = x ^ y;
            let zs = z80_sub(&mut z, 0xE621);
            let ns = hook_sub(&mut n, 0xE621);
            assert_eq!(zs, ns, "PixCrd2ScrAdr: x={x:02X} y={y:02X}");
        }
    }
}

#[test]
fn scr_adr_next_char_line_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    for hl in 0..=0xFFFFu16 {
        let h = (hl >> 8) as u8;
        let l = hl as u8;
        z.cpu.h = h;
        z.cpu.l = l;
        z.cpu.a = 0x55;
        z.cpu.b = 0xAA;
        z.cpu.c = 0x33;
        z.cpu.d = 0xCC;
        z.cpu.e = 0x77;
        z.cpu.f = hl as u8 ^ ((hl >> 8) as u8);
        n.cpu.h = h;
        n.cpu.l = l;
        n.cpu.a = 0x55;
        n.cpu.b = 0xAA;
        n.cpu.c = 0x33;
        n.cpu.d = 0xCC;
        n.cpu.e = 0x77;
        n.cpu.f = z.cpu.f;
        let zs = z80_sub(&mut z, 0x8445);
        let ns = hook_sub(&mut n, 0x8445);
        assert_eq!(zs, ns, "compDownScrAdr_HL hl=0x{hl:04X}");
    }
}

#[test]
fn scr_adr_next_line_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    for hl in 0..=0xFFFFu16 {
        let h = (hl >> 8) as u8;
        let l = hl as u8;
        z.cpu.h = h;
        z.cpu.l = l;
        z.cpu.a = 0x12;
        z.cpu.b = 0x34;
        z.cpu.c = 0x56;
        z.cpu.d = 0x78;
        z.cpu.e = 0x9A;
        z.cpu.f = hl as u8;
        n.cpu.h = h;
        n.cpu.l = l;
        n.cpu.a = 0x12;
        n.cpu.b = 0x34;
        n.cpu.c = 0x56;
        n.cpu.d = 0x78;
        n.cpu.e = 0x9A;
        n.cpu.f = z.cpu.f;
        let zs = z80_sub(&mut z, 0x7FE7);
        let ns = hook_sub(&mut n, 0x7FE7);
        assert_eq!(zs, ns, "sub_7FE7 hl=0x{hl:04X}");
    }
}

/// `sub_7A59` обязан совпасть с Z80 на всех 65536 парах (старое значение
/// байта, прибавляемое `A`): регистры, флаги, такты и записанный байт.
#[test]
fn smag_add_mem_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    let addr = 0x7100u16;
    for old in 0u8..=255 {
        for a in 0u8..=255 {
            z.mem[addr as usize] = old;
            n.mem[addr as usize] = old;
            set(&mut z.cpu, a, addr, 0x5678, a ^ old);
            set(&mut n.cpu, a, addr, 0x5678, a ^ old);
            z.cpu.b = 0x11;
            z.cpu.c = 0x22;
            n.cpu.b = 0x11;
            n.cpu.c = 0x22;
            let zs = z80_sub(&mut z, 0x7A59);
            let ns = hook_sub(&mut n, 0x7A59);
            assert_eq!(zs, ns, "sub_7A59 old=0x{old:02X} a=0x{a:02X}");
            assert_eq!(
                z.mem[addr as usize],
                n.mem[addr as usize],
                "sub_7A59: память по HL разошлась (old=0x{old:02X} a=0x{a:02X})"
            );
        }
    }
}

/// `CalcAngSpeed4Prn` (`0x713C`) обязан совпасть с Z80 на всех 65536
/// парах первых двух входных байтов (обе ветки по биту 7 — независимо) и
/// на всех 256 значениях третьего (для набора краевых первых двух).
/// Сверяются регистры, флаги, такты, `R` и все шесть выходных байтов.
#[test]
fn calc_ang_speed_4_prn_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);

    /// Один прогон: входные байты по `0xFF09`/`0xFF0B`/`0xFF0D`,
    /// произвольные регистры (`A`/`B`/`C`/`HL`/`F`/`R` — вход-мусор,
    /// `D`/`E` обязаны сохраниться), затем сверка с Z80.
    fn case(z: &mut Spectrum, n: &mut Spectrum, v1: u8, v2: u8, v3: u8) {
        for m in [&mut *z, &mut *n] {
            m.mem[0xFF09] = v1;
            m.mem[0xFF0B] = v2;
            m.mem[0xFF0D] = v3;
            set(&mut m.cpu, 0x55, 0x1234, 0x5678, 0xA5);
            m.cpu.b = 0x11;
            m.cpu.c = 0x22;
            m.cpu.r = 0x80;
        }
        let zs = z80_sub(z, 0x713C);
        let ns = hook_sub(n, 0x713C);
        assert_eq!(
            zs, ns,
            "CalcAngSpeed4Prn v1=0x{v1:02X} v2=0x{v2:02X} v3=0x{v3:02X}"
        );
        for (out, name) in [
            (0x70E4usize, "HighAngleLR"),
            (0x70FE, "byte_70FE"),
            (0x70E5, "HighAngleDU"),
            (0x7100, "byte_7100"),
            (0x70E6, "HighDeltaSpeed"),
            (0x70F4, "byte_70F4"),
        ] {
            assert_eq!(
                z.mem[out], n.mem[out],
                "CalcAngSpeed4Prn {name}: v1=0x{v1:02X} v2=0x{v2:02X} v3=0x{v3:02X}"
            );
        }
    }

    // Первые два байта — все пары, третий — краевые значения.
    const V3: &[u8] = &[0x00, 0x01, 0x05, 0x33, 0x51, 0x7F, 0x80, 0xFF];
    for v1 in 0u8..=255 {
        for v2 in 0u8..=255 {
            for &v3 in V3 {
                case(&mut z, &mut n, v1, v2, v3);
            }
        }
    }
    // Третий байт — все 256 значений (флаги финального `add a,5`).
    for &v1 in &[0x00u8, 0x7F, 0x80, 0xFF] {
        for &v2 in &[0x00u8, 0x7F, 0x80, 0xFF] {
            for v3 in 0u8..=255 {
                case(&mut z, &mut n, v1, v2, v3);
            }
        }
    }
}

/// `sub_D546` обязан совпасть с Z80 по всем проходам: все `B` (включая
/// 0 = 256 проходов, как у `djnz`), оба режима самоизменяющегося кода
/// (`0xD548`: `A9h` = XOR, `00h` = копирование), краевые адреса
/// (переносы `H` и `DE`), обе установки `C`-флага на входе. Сверяются
/// регистры, флаги, такты, `R` и вся память (кроме 2 байт стекового
/// маркера `run_routine` в 0xEFFE..0xEFFF, которых хук не пишет).
///
/// Значения `L` в парах не задевают маркер стека (L≠0FEh,0FFh) и код
/// рутины (0xD546..0xD54E, L≠46h..4Eh) — иначе настоящий Z80 перезаписал
/// бы собственный стек или опкоды посреди цикла.
#[test]
fn scr_xor_col_down_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    // (HL, DE): обычные + краевые.
    let pairs: &[(u16, u16)] = &[
        (0x7000, 0x8000),
        (0x707E, 0x8100),
        (0x7FF7, 0x8FFF), // `inc h` со стыка страниц
        (0x7123, 0x9ABC),
        (0x7800, 0xFFFF), // перенос `DE` в 0000h
        (0x77F0, 0x0000),
        (0x7F80, 0x8080),
        (0x7ABD, 0x8DEF),
        (0x7600, 0x86FF),
        (0x7456, 0xF0F0),
        (0x7E00, 0x7EFF), // источник пересекается с колонкой назначения
        (0x7900, 0x8901),
    ];
    for &xor_mode in [true, false].iter() {
        let mode_byte = if xor_mode { XOR_COL_XOR_OP } else { 0x00 };
        for &(hl, de) in pairs {
            for b in 0u8..=255 {
                let seed = (b as usize).wrapping_mul(0x9E37)
                    ^ (hl as usize)
                    ^ ((de as usize) << 1)
                    ^ ((mode_byte as usize) << 8);
                // Колонка назначения: все адреса с данным младшим байтом
                // (`inc h` меняет только старший), и 256 байт источника.
                let lo = (hl & 0xFF) as usize;
                for i in 0..256usize {
                    let v = (i.wrapping_mul(31).wrapping_add(seed) & 0xFF) as u8;
                    z.mem[(i << 8) | lo] = v;
                    n.mem[(i << 8) | lo] = v;
                    let s = (de as usize).wrapping_add(i) & 0xFFFF;
                    z.mem[s] = v ^ 0x5A;
                    n.mem[s] = v ^ 0x5A;
                }
                z.mem[XOR_COL_OP] = mode_byte;
                n.mem[XOR_COL_OP] = mode_byte;
                // 0xFF — C=1, 0xFE — C=0: в режиме копирования `C`-флаг
                // рутина не трогает и обязан сохранить входной.
                let f_in = if b & 1 == 0 { 0xFF } else { 0xFE };
                set(&mut z.cpu, 0x11, hl, de, f_in);
                set(&mut n.cpu, 0x11, hl, de, f_in);
                z.cpu.b = b;
                z.cpu.c = 0x22;
                n.cpu.b = b;
                n.cpu.c = 0x22;
                let zs = z80_sub(&mut z, 0xD546);
                let ns = hook_sub(&mut n, 0xD546);
                assert_eq!(
                    zs, ns,
                    "sub_D546 hl=0x{hl:04X} de=0x{de:04X} b=0x{b:02X} xor={xor_mode}"
                );
                // Вся память, кроме 2 байт маркера стека (0xEFFE..0xEFFF):
                // хук их не пишет, их пишет только run_routine у z.
                let diff = if z.mem[0..0xEFFE] != n.mem[0..0xEFFE] {
                    (0..0xEFFE).find(|&i| z.mem[i] != n.mem[i])
                } else if z.mem[0xF000..] != n.mem[0xF000..] {
                    (0xF000..0x1_0000).find(|&i| z.mem[i] != n.mem[i])
                } else {
                    None
                };
                if let Some(i) = diff {
                    panic!(
                        "sub_D546: память разошлась по адресу 0x{i:04X} \
                         (z=0x{:02X} n=0x{:02X}) hl=0x{hl:04X} de=0x{de:04X} \
                         b=0x{b:02X} xor={xor_mode}",
                        z.mem[i], n.mem[i]
                    );
                }
            }
        }
    }
}

/// `0x7FCD` (внутренний цикл `CopyBuf2Screen`) обязан совпасть с Z80:
/// копирование 60 строк по 32 байта через одну строку — регистры, флаги,
/// такты, `R` и вся память, включая резиду стека. Адреса назначения
/// подобраны так, чтобы копия не задела код рутины (0x7FCD..0x7FEE),
/// `compDownScrAdr_HL` (0x8445..0x8458) и страницу 0xEF00..0xF000 —
/// иначе расхождение было бы артефактом теста, а не хука.
///
/// SP взят игровыми значениями 4116h..411Eh: стек в игре живёт в
/// строках 1–7 экрана, и резиду push/call виден на экране. SP внутри
/// зоны записи копии тест не использует: там настоящий код сам себя
/// ломает — ldir перезаписывает вытолкнутые регистры перед pop, — так
/// что эквивалентность в таких условиях не определена.
#[test]
fn scr_copy_alt_lines_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    // (HL = источник, DE = назначение, входные A, F, C, SP).
    let cases: &[(u16, u16, u8, u8, u8, u16)] = &[
        (0xC000, 0x4020, 0x11, 0x00, 0x22, 0x411C), // реальная пара CopyBuf2Screen
        (0xC100, 0x4120, 0x23, 0xFF, 0x33, 0x411E), // реальная пара, второй проход
        (0xC000, 0x4020, 0x34, 0x01, 0x44, 0x4116), // игровой SP из полёта
        (0x0000, 0x0000, 0x67, 0x0F, 0x77, 0xEFFE), // записи в ПЗУ отбрасываются шиной
        (0x1234, 0x0800, 0x78, 0xF0, 0x88, 0xEFFE),
        (0x7E00, 0x08FF, 0x89, 0x3C, 0x99, 0xEFFE), // источник у кода рутины
        (0xFF00, 0x0C00, 0x9A, 0xC3, 0xAA, 0xEFFE), // H источника через FFh
        (0x8445, 0xC000, 0xAB, 0x00, 0xBB, 0xEFFE), // чтение со строки compDown
        (0xAABB, 0xFF00, 0xBC, 0x99, 0xCC, 0xEFFE),
        (0x4000, 0x9040, 0xCD, 0x69, 0xDD, 0xEFFE),
        (0x0100, 0x6000, 0xDE, 0x00, 0xEE, 0xEFFE),
        (0xE800, 0xB800, 0xEF, 0xA5, 0xFF, 0xEFFE),
        (0xFFFF, 0x0F00, 0xF0, 0x5A, 0x11, 0xEFFE), // чтения через 0000h
    ];
    for &(hl, de, a, f, c, sp) in cases {
        // Вся память — детерминированный паттерн, кроме кода рутины и
        // compDownScrAdr_HL (иначе Z80 перезаписал бы свои инструкции
        // посреди цикла).
        let seed = (hl as usize) ^ ((de as usize) << 1) ^ (a as usize);
        for i in 0..0x1_0000usize {
            if (0x7FCD..0x7FEF).contains(&i) || (0x8445..0x8459).contains(&i) {
                continue;
            }
            let v = (i.wrapping_mul(31).wrapping_add(seed) & 0xFF) as u8;
            z.mem[i] = v;
            n.mem[i] = v;
        }
        set(&mut z.cpu, a, hl, de, f);
        set(&mut n.cpu, a, hl, de, f);
        z.cpu.b = 0x99; // входной B перезаписывается `ld b,3Ch`
        n.cpu.b = 0x99;
        z.cpu.c = c;
        n.cpu.c = c;
        let zs = z80_sub_sp(&mut z, 0x7FCD, sp);
        n.cpu.sp = sp;
        let ns = hook_sub(&mut n, 0x7FCD);
        assert_eq!(
            zs, ns,
            "0x7FCD hl=0x{hl:04X} de=0x{de:04X} a={a:02X} f={f:02X} c={c:02X} sp=0x{sp:04X}"
        );
        // Вся память, кроме страницы 0xEF00..0xF000 (маркер и резиду
        // стека харнесса при SP=0xEFFE) и двух байт маркера по SP.
        let skip = |i: usize| {
            (0xEF00..0xF000).contains(&i) || i == sp as usize || i == sp as usize + 1
        };
        if let Some(i) = (0..0x1_0000usize).find(|&i| !skip(i) && z.mem[i] != n.mem[i]) {
            panic!(
                "0x7FCD: память разошлась по адресу 0x{i:04X} \
                 (z=0x{:02X} n=0x{:02X}) hl=0x{hl:04X} de=0x{de:04X} \
                 a={a:02X} f={f:02X} sp=0x{sp:04X}",
                z.mem[i], n.mem[i]
            );
        }
    }
}

/// Ветка Yield: когда целая итерация копии не влезает в остаток кадра,
/// хук обязан отдать управление в точку цикла `0x7FCF` с состоянием ровно
/// на начало итерации, а рутину дальше доигрывает обычный Z80. Так длинная
/// копия (≈99000 тактов, больше кадра) переживает vblank-прерывание в
/// середине, не расходясь с прогоном без хука.
///
/// Мини-диспетчер ниже повторяет `Spectrum::run_frame`: на хуке циклы
/// прибавляются, `RET` выполняется только если хук не отдал управление.
/// Состояние по достижении такта `budget` обязано совпасть с чистым
/// Z80-прогоном — по регистрам, флагам, `R`, SP, PC и всей памяти.
#[test]
fn scr_copy_alt_lines_yield_matches_z80() {
    // Плотная сетка бюджетов: кадровая граница попадает и в середину
    // копии, и внутрь коротких хуков (0x7FE7/0x8445), которые должны
    // уступать место Z80, чтобы не «проскочить» границу.
    let mut budgets: Vec<u64> = vec![500, 2000, 58664, 59954, 60000, 60050, 69000];
    budgets.extend((5000..61000).step_by(1009));
    for &budget in &budgets {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n);
        // Детерминированный паттерн, кроме кода рутины и compDownScrAdr_HL.
        for i in 0..0x1_0000usize {
            if (0x7FCD..0x7FEF).contains(&i) || (0x8445..0x8459).contains(&i) {
                continue;
            }
            let v = (i.wrapping_mul(31).wrapping_add(7) & 0xFF) as u8;
            z.mem[i] = v;
            n.mem[i] = v;
        }
        for m in [&mut z, &mut n] {
            set(&mut m.cpu, 0x11, 0xC000, 0x4020, 0x00);
            m.cpu.c = 0x22;
            m.cpu.sp = 0x411E; // игровой стек в экранной памяти
            m.mem[0x411E] = 0x34;
            m.mem[0x411F] = 0x12;
            m.cpu.cycles = 0;
            m.cpu.pc = 0x7FCD;
            m.frame_target = budget;
        }
        // Чистый Z80: шагаем, пока не выйдем за бюджет кадра.
        let mut cz = std::mem::take(&mut z.cpu);
        while cz.cycles < budget {
            cz.step(&mut z);
        }
        z.cpu = cz;
        // Хук + Z80: мини-диспетчер, как в `Spectrum::run_frame`.
        let mut cn = std::mem::take(&mut n.cpu);
        let mut yielded = false;
        while cn.cycles < budget {
            let pc = cn.pc;
            if let Some(hook) = n.hooks.get(&pc).copied() {
                let fits = n
                    .hook_max
                    .get(&pc)
                    .copied()
                    .map_or(true, |max| cn.cycles + max <= budget);
                if fits {
                    n.hook_yield = false;
                    let cyc = hook(&mut cn, &mut n);
                    cn.cycles += cyc;
                    if n.hook_yield {
                        yielded = true;
                        continue;
                    }
                    let lo = n.mem[cn.sp as usize] as u16;
                    let hi = n.mem[cn.sp.wrapping_add(1) as usize] as u16;
                    cn.sp = cn.sp.wrapping_add(2);
                    cn.pc = lo | (hi << 8);
                    continue;
                }
            }
            cn.step(&mut n);
        }
        n.cpu = cn;
        // При бюджете меньше одной итерации хук не запускается (охрана
        // `hook_max`) — рутину целиком играет Z80, отдачи управления нет.
        if budget >= COPY_ALT_MAX_ITER {
            assert!(yielded, "budget={budget}: хук ни разу не отдал управление");
        }
        assert_eq!(snap(&z.cpu), snap(&n.cpu), "регистры budget={budget}");
        assert_eq!(z.cpu.pc, n.cpu.pc, "PC budget={budget}");
        assert_eq!(z.cpu.sp, n.cpu.sp, "SP budget={budget}");
        if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
            panic!(
                "0x7FCD yield: память разошлась по адресу 0x{i:04X} \
                 (z=0x{:02X} n=0x{:02X}) budget={budget}",
                z.mem[i], n.mem[i]
            );
        }
    }
}

/// Прогон рутины по адресу-маркеру `0x1234`; при `dispatch` задействован
/// мини-диспетчер, повторяющий `Spectrum::run_frame` (такты хука
/// прибавляются, `RET` делается только если хук не отдал управление).
/// Нужен там, где нативный хук уходит в `Yield` и рутину доигрывает Z80.
fn run_until_ret(m: &mut Spectrum, addr: u16, sp: u16, dispatch: bool) {
    const RET: u16 = 0x1234;
    m.frame_target = u64::MAX;
    m.cpu.sp = sp;
    m.cpu.pc = addr;
    let [lo, hi] = RET.to_le_bytes();
    m.mem[sp as usize] = lo;
    m.mem[sp as usize + 1] = hi;
    let mut cpu = std::mem::take(&mut m.cpu);
    let mut guard = 0u32;
    while cpu.pc != RET {
        assert!(guard < 2_000_000, "рутина не вернулась (guard={guard})");
        guard += 1;
        if dispatch {
            let pc = cpu.pc;
            if let Some(hook) = m.hooks.get(&pc).copied() {
                let fits = m
                    .hook_max
                    .get(&pc)
                    .copied()
                    .map_or(true, |max| cpu.cycles.saturating_add(max) <= m.frame_target);
                if fits {
                    m.hook_yield = false;
                    let c = hook(&mut cpu, m);
                    cpu.cycles += c;
                    if m.hook_yield {
                        continue;
                    }
                    let lo = m.mem[cpu.sp as usize] as u16;
                    let hi = m.mem[cpu.sp.wrapping_add(1) as usize] as u16;
                    cpu.sp = cpu.sp.wrapping_add(2);
                    cpu.pc = lo | (hi << 8);
                    continue;
                }
            }
        }
        cpu.step(m);
    }
    m.cpu = cpu;
}

/// `PrintMessage1_A` (`0xD425`) обязан совпасть с Z80 на всех входах `A`:
/// диспетчер, поиск строки токена (`cpir` по таблице) и `Yield` на печать
/// (`0xD499`), диграфы (`0xD59C`) и управляющие коды (`0xD7E2`).
///
/// Таблица токенов подменена детерминированной в `0x9000` (201 строка
/// только из печатных символов), чтобы не тянуть зависимости реальных
/// строк; сама печать, диграфы и обработчики управляющих кодов остаются
/// на Z80 в обеих машинах и должны дать тот же результат. Сверяются все
/// регистры, флаги, `R`, такты, `PC`/`SP` и вся память.
#[test]
fn print_message1_matches_z80() {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    // Детерминированная таблица токенов: 201 строка печатных глифов.
    const TBL: u16 = 0x9000;
    let mut p = TBL as usize;
    for i in 0..201usize {
        let ch = b'A' + (i % 26) as u8;
        for _ in 0..(1 + i % 4) {
            z.mem[p] = ch;
            n.mem[p] = ch;
            p += 1;
        }
        z.mem[p] = 0;
        n.mem[p] = 0;
        p += 1;
    }
    for m in [&mut z, &mut n] {
        let [lo, hi] = TBL.to_le_bytes();
        m.mem[0xF11A] = lo;
        m.mem[0xF11B] = hi;
    }
    for &sp in &[0x4116u16, 0x411E, 0xEFFE] {
        for code in 0u16..=255 {
            let a = code as u8;
            for m in [&mut z, &mut n] {
                set(&mut m.cpu, a, 0x1234, 0x5678, 0x00);
                m.cpu.b = 0x11;
                m.cpu.c = 0x22;
                m.cpu.ix = 0x629B;
                m.cpu.iy = 0x6159;
                m.cpu.r = 0x3E;
                m.cpu.cycles = 0;
            }
            run_until_ret(&mut z, 0xD425, sp, false);
            run_until_ret(&mut n, 0xD425, sp, true);
            assert_eq!(
                snap(&z.cpu),
                snap(&n.cpu),
                "PrintMessage1_A a=0x{a:02X} sp=0x{sp:04X}: регистры"
            );
            assert_eq!(z.cpu.pc, n.cpu.pc, "a=0x{a:02X} sp=0x{sp:04X}: PC");
            assert_eq!(z.cpu.sp, n.cpu.sp, "a=0x{a:02X} sp=0x{sp:04X}: SP");
            if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
                let lo = i & !0x1F;
                let dump = |mm: &Spectrum| {
                    (lo..lo + 32)
                        .map(|k| format!("{:02X}", mm.mem[k]))
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                panic!(
                    "PrintMessage1_A a=0x{a:02X} sp=0x{sp:04X}: память разошлась \
                     по адресу 0x{i:04X}: z={} | n={}",
                    dump(&z),
                    dump(&n)
                );
            }
        }
    }
}

/// Сквозная печать сообщения через нативные хуки: диспетчер
/// `PrintMessage1_A` раскрывает токен, `sub_D499` печатает символы —
/// хук перехвата копит их в [`Spectrum::game_msg`].
#[test]
fn message_capture_collects_token_string() {
    let mut n = kladov();
    install(&mut n);
    n.game_msg.enabled = true;

    // Детерминированная таблица токенов, как в `print_message1_matches_z80`:
    // строка 0x87 (135) — четыре буквы 'F'.
    const TBL: u16 = 0x9000;
    let mut p = TBL as usize;
    for i in 0..201usize {
        let ch = b'A' + (i % 26) as u8;
        for _ in 0..(1 + i % 4) {
            n.mem[p] = ch;
            p += 1;
        }
        n.mem[p] = 0;
        p += 1;
    }
    n.mem[0xF11A] = TBL as u8;
    n.mem[0xF11B] = (TBL >> 8) as u8;
    // Печатный курсор (CrdXPrnSymbol версии Кладова): строка 2, колонка 0.
    n.mem[KLADOV_CRD_X_PRN_SYMBOL] = 0;
    n.mem[KLADOV_CRD_X_PRN_SYMBOL + 1] = 2;

    // Токен `0x15` → `0x15 + 0x72 = 0x87` → "FFFF".
    set(&mut n.cpu, 0x15, 0x1234, 0x5678, 0x00);
    n.cpu.b = 0x11;
    n.cpu.c = 0x22;
    n.cpu.r = 0x3E;
    run_until_ret(&mut n, 0xD425, 0x4116, true);

    assert_eq!(n.game_msg.text, "FFFF", "хук печати не собрал строку");
    assert_eq!(n.game_msg.row, 2);
    assert_eq!(n.game_msg.col, 0);
    assert_eq!(n.game_msg.frames, 0);
}

/// Строка гиперпрыжка из настоящей таблицы токенов: `msgNumber = 0xC0`
/// раскрывается в `HYPERSPACE -- <PlanetName>`. Печать идёт в ZX-строку 13,
/// как её ставит `sub_DAB4`, и остаётся в истории строк.
#[test]
fn message_capture_collects_hyperspace_line() {
    let mut n = kladov();
    install(&mut n);
    n.game_msg.enabled = true;
    n.game_msg.enable_text_capture_for_test();

    // Имя планеты (`PlanetName`), которое печатает управляющий код 3
    // внутри токена.
    n.mem[0xDEED..0xDEF6].copy_from_slice(b"LAVE\0\0\0\0\0");
    // Курсор `sub_DAB4`: строка 13, колонка 5.
    n.mem[KLADOV_CRD_X_PRN_SYMBOL] = 5;
    n.mem[KLADOV_CRD_X_PRN_SYMBOL + 1] = 13;

    set(&mut n.cpu, 0xC0, 0x1234, 0x5678, 0x00);
    n.cpu.b = 0x11;
    n.cpu.c = 0x22;
    n.cpu.r = 0x3E;
    run_until_ret(&mut n, 0xD425, 0x4116, true);

    assert_eq!(n.game_msg.text, "HYPERSPACE -- LAVE");
    let line = n
        .game_msg
        .lines
        .iter()
        .find(|l| l.row == 13)
        .expect("строка гиперпрыжка не попала в историю");
    assert_eq!(line.text, "HYPERSPACE -- LAVE");
    assert_eq!(line.col, 5);

    // Тот же текст приходит готовым событием для рендера: строка 13,
    // позиция из `CrdXPrnSymbol` после сдвига курсора.
    n.game_msg.end_text_frame();
    let events = n.game_msg.take_text_events();
    assert_eq!(events.len(), 1, "ожидалось одно событие строки 13");
    assert_eq!(events[0].text, "HYPERSPACE -- LAVE");
    assert_eq!(events[0].row, 13);
    // Хук `sub_5DFB` видит курсор уже сдвинутым: стартовая колонка 5
    // превращается в ячейку первого напечатанного глифа — 6.
    assert_eq!(events[0].col, 6);
}

/// Пара машин (чистый Z80 и с хуками) с одинаковым детерминированным
/// паттерном памяти везде, кроме кода и таблицы звуковой рутины
/// (`0x9560..0x96C0`): данные эффектов (`HL`), переменные автомата и
/// самомодифицируемые операнды лежат вне этого диапазона либо задаются
/// случаем явно.
fn sfx_pair() -> (Spectrum, Spectrum) {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    for i in 0..0x1_0000usize {
        if (0x9560..0x96C0).contains(&i) {
            continue;
        }
        let v = (i.wrapping_mul(37).wrapping_add(11) & 0xFF) as u8;
        z.mem[i] = v;
        n.mem[i] = v;
    }
    (z, n)
}

/// Готовит обе машины к запуску `sub_9579`: «мусорные» входные регистры
/// (рутина обязана перезаписать их до использования), нулевые такты и
/// стековый маркер возврата.
fn sfx_prepare(m: &mut Spectrum) {
    let c = &mut m.cpu;
    c.cycles = 0;
    c.pc = 0x9579;
    c.sp = 0x411E;
    c.a = 0xA5;
    c.f = 0x00;
    c.b = 0x77;
    c.c = 0x66;
    c.d = 0x55;
    c.e = 0x44;
    c.h = 0x33;
    c.l = 0x22;
    m.mem[0x411E] = 0x34; // маркер возврата 0x1234
    m.mem[0x411F] = 0x12;
}

/// Выполняет рутину на Z80 до маркера, обновляя такт начала инструкции
/// (как `Spectrum::run_frame`), чтобы переходы бипера встали на те же такты.
fn sfx_z80_run(m: &mut Spectrum) {
    const RET: u16 = 0x1234;
    let mut cpu = std::mem::take(&mut m.cpu);
    let mut guard = 0u32;
    while cpu.pc != RET {
        assert!(
            guard < 20_000_000,
            "sub_9579 не вернулась (pc=0x{:04X})",
            cpu.pc
        );
        m.set_cycle_clock(cpu.cycles);
        cpu.step(m);
        guard += 1;
    }
    m.cpu = cpu;
}

/// Выполняет хук `sub_9579` и завершает его `RET` так же, как это делает
/// диспетчер `Spectrum::run_frame` (хук сам `RET` не делает).
fn sfx_hook_run(m: &mut Spectrum) {
    let mut cpu = std::mem::take(&mut m.cpu);
    m.frame_target = u64::MAX;
    m.hook_yield = false;
    let cyc = (m.hooks[&0x9579])(&mut cpu, m);
    cpu.cycles += cyc;
    assert!(!m.hook_yield, "хук 0x9579 отдал управление вне кадра");
    let lo = m.mem[cpu.sp as usize] as u16;
    let hi = m.mem[cpu.sp.wrapping_add(1) as usize] as u16;
    cpu.sp = cpu.sp.wrapping_add(2);
    cpu.pc = lo | (hi << 8);
    m.cpu = cpu;
}

/// Строгое сравнение двух машин после прогона звуковой рутины.
fn sfx_assert(name: &str, z: &Spectrum, n: &Spectrum) {
    assert_eq!(snap(&z.cpu), snap(&n.cpu), "{name}: регистры/флаги/R/такты");
    assert_eq!(z.cpu.pc, n.cpu.pc, "{name}: PC");
    assert_eq!(z.cpu.sp, n.cpu.sp, "{name}: SP");
    assert_eq!(z.border, n.border, "{name}: бордюр");
    assert_eq!(z.beeper, n.beeper, "{name}: бипер");
    assert_eq!(z.beeper_edges(), n.beeper_edges(), "{name}: переходы бипера");
    assert_eq!(z.paging_writes, n.paging_writes, "{name}: обращения к 7FFD");
    assert_eq!(z.paging_last, n.paging_last, "{name}: значение 7FFD");
    if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
        panic!(
            "{name}: память разошлась по адресу 0x{i:04X} \
             (z=0x{:02X} n=0x{:02X})",
            z.mem[i], n.mem[i]
        );
    }
}

/// `sub_9579` (звуковой автомат) обязан совпасть с Z80 по всем регистрам,
/// флагам, `R`, тактам, `SP`, всей памяти, бордюру и биперу на всех ветвях:
/// ранние выходы, состояния `byte_9522` = 1/2/3 и `0x83`, все 8 битов
/// `nBeepFX` (таблица `jp`-ов `0x9561`), оба варианта самомодификации
/// `0x95F2` и оба исхода `ret nz`.
#[test]
fn sfx_update_matches_z80() {
    let cases: &[(&str, fn(&mut Spectrum))] = &[
        // FlagsSetUp: бит 7 снят -> немедленный выход.
        ("flags bit7=0", |m| m.mem[0xFC0E] = 0x00),
        // Эффекта нет и не запрошен -> выход.
        ("idle: no state, no request", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0;
        }),
        // byte_9522 = 1: пакет импульсов (0x9624) с разными операндами
        // самомодифицируемого `add a,n` (0x962B).
        ("state 1, op 05", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 1;
            m.mem[0x962B] = 0x05;
            m.mem[0x9523] = 0x08; // E
            m.mem[0x9524] = 0x02; // D -> ret nz
            m.mem[0x96BF] = 0x02;
        }),
        ("state 1, op FD", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 1;
            m.mem[0x962B] = 0xFD;
            m.mem[0x9523] = 0x0F;
            m.mem[0x9524] = 0x01; // D -> эпилог 0x9610
            m.mem[0x96BF] = 0x01;
        }),
        ("state 1, op D3", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 1;
            m.mem[0x962B] = 0xD3;
            m.mem[0x9523] = 0x00;
            m.mem[0x9524] = 0x03; // D -> ret nz
            m.mem[0x96BF] = 0x07;
        }),
        // byte_9522 = 2: шум (0x95E7) с обеими самомодификациями 0x95F2.
        ("state 2, srl patch, D->9610", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 2;
            m.mem[0x95F2] = 0xCB;
            m.mem[0x95F3] = 0x3F;
            m.mem[0x9523] = 0x00;
            m.mem[0x9524] = 0x01; // D = 1
            m.mem[0x9525] = 0x1A;
            m.mem[0x9526] = 0x03; // HL = 0x031A
            m.mem[0x96BF] = 0x03;
        }),
        ("state 2, add 10h patch, D->ret nz", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 2;
            m.mem[0x95F2] = 0xC6;
            m.mem[0x95F3] = 0x10;
            m.mem[0x9523] = 0x00;
            m.mem[0x9524] = 0x02; // D = 2 -> ret nz
            m.mem[0x9525] = 0x00;
            m.mem[0x9526] = 0x80; // HL = 0x8000
            m.mem[0x96BF] = 0x00;
        }),
        // byte_9522 = 3: тон с огибающей (0x95A6).
        ("state 3, ret nz", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 3;
            m.mem[0x95AE] = 0x40; // C
            m.mem[0x95BD] = 0x00; // add a,n
            m.mem[0x9523] = 0x02; // E = 2
            m.mem[0x9524] = 0x01; // D = 1
            m.mem[0x9525] = 0x00;
            m.mem[0x9526] = 0x80;
            m.mem[0x96BF] = 0x01;
        }),
        ("state 3, exhausted -> 9610", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 3;
            m.mem[0x95AE] = 0x40;
            m.mem[0x95BD] = 0x60;
            m.mem[0x9523] = 0x01; // E = 1 -> dec e = 0
            m.mem[0x9524] = 0x01;
            m.mem[0x9525] = 0x00;
            m.mem[0x9526] = 0x80;
            m.mem[0x96BF] = 0x04;
        }),
        // byte_9522 = 0x83: исчерпание уводит на перезапуск 0x95CF.
        ("state 3 (0x83), restart -> 95CF", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0x83;
            m.mem[0x95AE] = 0x40;
            m.mem[0x95BD] = 0xA0;
            m.mem[0x9523] = 0x01;
            m.mem[0x9524] = 0x01;
            m.mem[0x9525] = 0x00;
            m.mem[0x9526] = 0x80;
            m.mem[0x96BF] = 0x05;
        }),
        // Запуск эффекта по каждому биту nBeepFX (таблица 0x9561).
        ("req bit0 -> 9678 (C=FE)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x01;
            m.mem[0x96BF] = 0x00;
        }),
        ("req bit1 -> 9674 (C=E4)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x02;
            m.mem[0x96BF] = 0x01;
        }),
        ("req bit2 -> 9692 (C=02)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x04;
            m.mem[0x96BF] = 0x02;
        }),
        ("req bit3 -> 965E (state 2)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x08;
            m.mem[0x96BF] = 0x03;
        }),
        ("req bit4 -> 9641 (state 1, FD)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x10;
            m.mem[0x96BF] = 0x04;
        }),
        ("req bit5 -> 9648 (state 1, D3)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x20;
            m.mem[0x96BF] = 0x05;
        }),
        ("req bit6 -> 9657 (add 10h)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x40;
            m.mem[0x96BF] = 0x06;
        }),
        ("req bit7 -> 9657 (add 10h)", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x80;
            m.mem[0x96BF] = 0x07;
        }),
        // Несколько битов сразу: берётся младший установленный.
        ("req 0x2A -> bit1", |m| {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x2A;
            m.mem[0x96BF] = 0x01;
        }),
    ];
    for &(name, setup) in cases {
        let (mut z, mut n) = sfx_pair();
        setup(&mut z);
        setup(&mut n);
        sfx_prepare(&mut z);
        sfx_prepare(&mut n);
        sfx_z80_run(&mut z);
        sfx_hook_run(&mut n);
        sfx_assert(name, &z, &n);
    }
}

/// Ветка Yield звукового автомата: один чанк эффекта длиннее кадра, поэтому
/// на границе кадра хук обязан отдать управление Z80 ровно на той
/// инструкции, что не влезает, с состоянием на её начало. Мини-диспетчер
/// повторяет `Spectrum::run_frame`; состояние по достижении такта `budget`
/// обязано совпасть с чистым Z80-прогоном по регистрам, флагам, `R`, `SP`,
/// `PC`, всей памяти, бордюру и биперу.
#[test]
fn sfx_update_yield_matches_z80() {
    // Биты nBeepFX = 0x40 дают самый длинный чанк (100 итераций шумового
    // цикла), гарантированно длиннее кадра.
    let mut budgets: Vec<u64> = vec![10, 100, 500, 5000, 69000, 69888, 200_000, 400_000];
    budgets.extend((1000..300_000).step_by(7919));
    for &budget in &budgets {
        let (mut z, mut n) = sfx_pair();
        for m in [&mut z, &mut n] {
            m.mem[0xFC0E] = 0x80;
            m.mem[0x9522] = 0;
            m.mem[0x7128] = 0x40; // бит 6 -> 0x9657, длинный шум
            m.mem[0x96BF] = 0x06;
            sfx_prepare(m);
            m.frame_target = budget;
        }
        // Чистый Z80: шагаем, пока не выйдем за бюджет кадра или рутина
        // не вернётся по маркеру.
        {
            let mut cz = std::mem::take(&mut z.cpu);
            while cz.cycles < budget && cz.pc != 0x1234 {
                z.set_cycle_clock(cz.cycles);
                cz.step(&mut z);
            }
            z.cpu = cz;
        }
        // Хук + Z80: мини-диспетчер, как в `Spectrum::run_frame`.
        let mut cn = std::mem::take(&mut n.cpu);
        let mut yielded = false;
        while cn.cycles < budget && cn.pc != 0x1234 {
            let pc = cn.pc;
            if let Some(hook) = n.hooks.get(&pc).copied() {
                let fits = n
                    .hook_max
                    .get(&pc)
                    .copied()
                    .map_or(true, |max| cn.cycles + max <= budget);
                if fits {
                    n.hook_yield = false;
                    let cyc = hook(&mut cn, &mut n);
                    cn.cycles += cyc;
                    n.set_cycle_clock(cn.cycles);
                    if n.hook_yield {
                        yielded = true;
                        continue;
                    }
                    let lo = n.mem[cn.sp as usize] as u16;
                    let hi = n.mem[cn.sp.wrapping_add(1) as usize] as u16;
                    cn.sp = cn.sp.wrapping_add(2);
                    cn.pc = lo | (hi << 8);
                    continue;
                }
            }
            n.set_cycle_clock(cn.cycles);
            cn.step(&mut n);
        }
        n.cpu = cn;
        // Если кадровая граница пришлась на середину чанка, хук обязан был
        // отдать управление хотя бы раз (иначе он «проскочил» бы границу).
        // При бюджете меньше одного шага охрана `hook_max` не пускает хук —
        // рутину целиком доигрывает Z80.
        if budget >= SFX_MAX_STEP && z.cpu.pc != 0x1234 {
            assert!(yielded, "budget={budget}: хук ни разу не отдал управление");
        }
        sfx_assert(&format!("yield budget={budget}"), &z, &n);
    }
}

/// Пара машин для проверки `ScanKeys`: память забивается «мусором», кроме
/// диапазонов, которые рутина читает как код и таблицы — тела `ScanKeys`
/// и хвоста `0xFC10..0xFCEA`, таблицы ASCII `unk_FD41` и пар
/// `(смещение,маска)` `unk_FF58`.
fn scan_keys_pair() -> (Spectrum, Spectrum) {
    let mut z = kladov();
    let mut n = kladov();
    install(&mut n);
    for i in 0..0x1_0000usize {
        let keep = (0xFC10..0xFCEA).contains(&i)
            || (0xFD41..0xFD8E).contains(&i)
            || (0xFF58..0xFFA8).contains(&i);
        let v = if keep {
            z.mem[i]
        } else {
            (i.wrapping_mul(37).wrapping_add(11) & 0xFF) as u8
        };
        z.mem[i] = v;
        n.mem[i] = v;
    }
    (z, n)
}

/// Готовит обе машины к вызову `ScanKeys`: раскладка клавиш, переменные
/// `ControlKey1/2/Move` и `FlagsSetUp`, «мусорные» регистры, нулевые такты
/// и стековый маркер возврата (`SP` = 0xF000, маркер 0x1234).
///
/// Клавиша с индексом `idx` (0..39): бит `idx>>3` клавиатурной строки
/// `7 - (idx&7)` (строка сканирования `r` несёт клавиши с `idx&7 = 7-r`,
/// бит0 порта — первая проверяемая клавиша строки).
fn scan_keys_prepare(m: &mut Spectrum, keys: &[u8], vars: &[u8]) {
    m.release_all_keys();
    for &k in keys {
        m.set_key((7 - (k & 7)) as usize, k >> 3, true);
    }
    // vars: [ControlKey1, ControlKeyMove, ControlKey2, FlagsSetUp]
    m.mem[0xFC08] = vars[0];
    m.mem[0xFC09] = vars[1];
    m.mem[0xFC0A] = vars[2];
    m.mem[0xFC0E] = vars[3];
    let c = &mut m.cpu;
    c.cycles = 0;
    c.pc = 0xFD69;
    c.sp = 0xF000;
    c.r = 0x2B;
    c.a = 0xA5;
    c.f = 0x00;
    c.b = 0x77;
    c.c = 0x66;
    c.d = 0x55;
    c.e = 0x44;
    c.h = 0x33;
    c.l = 0x22;
    m.mem[0xF000] = 0x34; // маркер возврата 0x1234
    m.mem[0xF001] = 0x12;
}

/// Выполняет хук `ScanKeys` и завершает его `RET` так же, как диспетчер
/// `Spectrum::run_frame` (хук сам адрес из стека не снимает).
fn scan_keys_hook_run(m: &mut Spectrum) {
    let mut cpu = std::mem::take(&mut m.cpu);
    m.frame_target = u64::MAX;
    m.hook_yield = false;
    cpu.cycles = 0;
    let cyc = (m.hooks[&0xFD69])(&mut cpu, m);
    cpu.cycles += cyc;
    assert!(!m.hook_yield, "хук 0xFD69 отдал управление вне кадра");
    let lo = m.mem[cpu.sp as usize] as u16;
    let hi = m.mem[cpu.sp.wrapping_add(1) as usize] as u16;
    cpu.sp = cpu.sp.wrapping_add(2);
    cpu.pc = lo | (hi << 8);
    m.cpu = cpu;
}

/// `ScanKeys` обязана совпасть с Z80 по регистрам, флагам, `R`, тактам,
/// `SP`/`PC` и всей памяти 64K: пустой опрос, все 40 одиночных клавиш,
/// все клавиши сразу и псевдослучайные подмножества; переменные
/// `ControlKey1/2/Move` и режим `FlagsSetUp&3` перебираются по всем
/// сочетаниям бит6/бит7 и режимов 0..3.
#[test]
fn scan_keys_matches_z80() {
    let mut cases: Vec<(Vec<u8>, [u8; 4])> = Vec::new();
    // Пустая клавиатура × все режимы × бит6 ControlKey1/бит7 ControlKey2.
    for &mode in &[0x00u8, 0x01, 0x02, 0x03, 0x80, 0x83] {
        for &ck1 in &[0x00u8, 0x40] {
            for &ck2 in &[0x00u8, 0x80] {
                cases.push((
                    vec![],
                    [ck1, (mode << 4) | 0x05, ck2, mode],
                ));
            }
        }
    }
    // Каждая одиночная клавиша (режимы чередуются).
    for k in 0u8..40 {
        let mode = [0x00u8, 0x01, 0x02, 0x03][(k & 3) as usize];
        cases.push((vec![k], [0x00, 0x0F, 0x00, mode]));
    }
    // Все клавиши сразу.
    cases.push(((0u8..40).collect(), [0x40, 0x0A, 0x80, 0x03]));
    // Псевдослучайные подмножества (детерминированный LCG).
    let mut seed = 0x1234_5678u32;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    };
    for _ in 0..48 {
        let nkeys = (rnd() % 13) as usize;
        let keys: Vec<u8> = (0..nkeys).map(|_| (rnd() % 40) as u8).collect();
        let r = rnd();
        let vars = [
            (r & 0xC0) as u8,
            (r >> 8) as u8,
            (r >> 16) as u8,
            (r >> 24) as u8,
        ];
        cases.push((keys, vars));
    }
    for (keys, vars) in cases {
        let (mut z, mut n) = scan_keys_pair();
        scan_keys_prepare(&mut z, &keys, &vars);
        scan_keys_prepare(&mut n, &keys, &vars);
        // Z80: полный прогон до RET по маркеру.
        z.cpu.cycles = 0;
        run_routine_sp(&mut z, 0xFD69, 0xF000);
        // Нативно: хук + диспетчерный RET.
        scan_keys_hook_run(&mut n);
        let name = format!("keys={keys:?} vars={vars:02X?}");
        assert_eq!(snap(&z.cpu), snap(&n.cpu), "{name}: регистры/такты");
        assert_eq!(z.cpu.pc, n.cpu.pc, "{name}: PC");
        assert_eq!(z.cpu.sp, n.cpu.sp, "{name}: SP");
        assert!(
            n.cpu.cycles <= SCAN_KEYS_MAX,
            "{name}: такты {} > SCAN_KEYS_MAX",
            n.cpu.cycles
        );
        if let Some(i) = (0..0x1_0000usize).find(|&i| z.mem[i] != n.mem[i]) {
            panic!(
                "{name}: память разошлась по адресу 0x{i:04X} \
                 (z=0x{:02X} n=0x{:02X})",
                z.mem[i], n.mem[i]
            );
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

    for layout in [KLADOV_SAVE] {
        let path = format!("{dir}/commander_roundtrip_{:04X}.zxelite", layout.pilot_name);
        let mut m = kladov();
        let a = layout.pilot_name as usize;
        let n = layout.commander_len;
        m.mem[a] = b'T';
        m.mem[a + 1] = b'E';
        m.mem[a + 2] = b'S';
        m.mem[a + 3] = b'T';
        m.mem[a + n - 1] = 0x5A;
        let expected: Vec<u8> = {
            let mut ex = m.mem[a..a + n].to_vec();
            let sc = layout.sun_crd as usize;
            ex[SUN_FIELD_OFF..SUN_FIELD_OFF + 9].copy_from_slice(&m.mem[sc..sc + 9]);
            ex
        };
        assert_eq!(expected.len(), n);

        // Синхронизация тени при сохранении: `InitGame` восстанавливает
        // пилота именно из неё, поэтому после сохранения она должна
        // совпадать с живым блоком.
        let shadow_a = layout.pilot_shadow.map(|(addr, _)| addr as usize);
        commander_save(&mut m, &path, layout).unwrap();
        if let Some(sa) = shadow_a {
            let sn = layout.commander_len.min(layout.pilot_shadow.unwrap().1);
            // После сохранения теневой буфер должен полностью совпадать с
            // живым блоком (как `SaveGame`: `PilotName`→`PilotDataSaved`).
            let live_now = &m.mem[a..a + sn];
            assert_eq!(
                &m.mem[sa..sa + sn],
                live_now,
                "теневой буфер (0x{sa:04X}) не синхронизировался при сохранении"
            );
        }
        for b in &mut m.mem[a..a + n] {
            *b = 0xAA;
        }
        commander_load(&mut m, &path, layout).unwrap();
        assert_eq!(
            commander_block(&m, layout),
            &expected[..],
            "блок командира (0x{:04X}) не совпал",
            layout.pilot_name
        );

        let _ = std::fs::remove_file(&path);
    }
}

/// Перехват `MenuSaveLoad`: хук должен выставить запрос на сохранение и
/// сбросить «залипшие» клавиши (`ControlKey1`..`ControlKey3`), как это
/// делает родная рутина через `ResetPressedKeys`.
#[test]
fn menu_save_load_hook_sets_request() {
    for layout in [KLADOV_SAVE] {
        let mut m = kladov();
        install_save(&mut m, layout);
        assert!(
            m.hooks.contains_key(&layout.menu_save_load),
            "хук MenuSaveLoad 0x{:04X} не установлен",
            layout.menu_save_load
        );
        assert!(!m.save_requested);
        // Имитируем нажатие клавиши сохранения (Symbol Shift -> бит 5
        // `ControlKeyMove`) и других залипших клавиш.
        let a = layout.control_key1 as usize;
        m.mem[a] = 0x50;
        m.mem[a + 1] = 0x20;
        m.mem[a + 2] = 0x80;
        m.mem[a + 3] = 0x01;
        let hook = m.hooks.get(&layout.menu_save_load).copied().unwrap();
        let mut cpu = std::mem::take(&mut m.cpu);
        let cycles = hook(&mut cpu, &mut m);
        m.cpu = cpu;
        assert_eq!(cycles, 0);
        assert!(m.save_requested, "запрос на сохранение не выставлен");
        assert_eq!(
            &m.mem[a..a + PRESSED_KEY_BYTES],
            &[0u8; PRESSED_KEY_BYTES][..],
            "залипшие клавиши не сброшены (0x{:04X})",
            layout.control_key1
        );
    }
}

/// Прогон сценария «нажали клавишу сохранения» и подсчёт запросов.
fn count_save_requests(m: &mut Spectrum) -> usize {
    let mut requests = 0;
    for f in 0..260 {
        m.release_all_keys();
        if (30..80).contains(&f) {
            m.set_key(7, 0, true); // SPACE — старт полёта с титула
        }
        if f == 150 {
            m.set_key(7, 1, true); // Symbol Shift — клавиша сохранения (1 кадр)
        }
        m.run_frame();
        if m.save_requested {
            m.save_requested = false;
            requests += 1;
        }
    }
    requests
}

/// Регрессия на бесконечный цикл сохранения: одно нажатие клавиши
/// сохранения (Symbol Shift) должно дать ровно один запрос, а не
/// повторяться на каждой итерации главного цикла.
#[test]
fn save_request_fires_once_per_press() {
    let mut kladov = boot_kladov();
    install_save(&mut kladov, KLADOV_SAVE);
    let requests = count_save_requests(&mut kladov);
    assert_eq!(
        requests, 1,
        "Кладова: сохранение сработало {requests} раз вместо одного"
    );
}

/// Загрузка прерывает текущий процесс и перезапускает игру: пилот из файла
/// сохраняется, а после стартовой последовательности корабль пристыкован
/// (`IndDanger=0`) — то есть мы на станции. Загрузка выполняется «в полёте»
/// (родная игра разрешает её только на станции, поэтому [`request_load`]
/// обходит эту проверку).
#[test]
fn load_restarts_docked_with_saved_pilot() {
    let base = env!("CARGO_MANIFEST_DIR");
    let dir = format!("{base}/tmp");
    std::fs::create_dir_all(&dir).unwrap();

    // Стартовая последовательность сначала ждёт отпускания клавиши, а
    // затем её нажатия (выбор устройства управления). Подаём SPACE
    // импульсами, чтобы попасть в оба состояния.
    let impulse = |m: &mut Spectrum, f: u64| {
        m.release_all_keys();
        if f >= 30 && f % 16 < 6 {
            m.set_key(7, 0, true);
        }
    };

    for (layout, ind_danger, mut m) in [
        (KLADOV_SAVE, 0x7104u16, boot_kladov()),
    ] {
        install_save(&mut m, layout);

        // Приводим игру в рабочее состояние и имитируем полёт.
        for f in 0..150u64 {
            impulse(&mut m, f);
            m.run_frame();
        }
        m.mem[ind_danger as usize] = 4; // «в полёте» (вылет со станции)
        assert_ne!(
            m.mem[ind_danger as usize], 0,
            "подготовка: корабль должен быть в полёте"
        );

        // Готовим файл: валидный командир из образа с помеченным именем.
        let mut data = commander_block(&m, layout).to_vec();
        data[0..8].copy_from_slice(b"TESTPILO");
        let path = format!("{dir}/load_restart_{:04X}.zxelite", layout.pilot_name);
        std::fs::write(&path, &data).unwrap();

        commander_load(&mut m, &path, layout).unwrap();
        request_load(&mut m, layout);
        assert!(m.load_requested, "запрос загрузки не выставлен");

        // Игра сама доходит до вызова `MenuSaveLoad` и уходит на перезапуск;
        // стартовая последовательность требует нажатия SPACE.
        for f in 0..900u64 {
            impulse(&mut m, f);
            m.run_frame();
        }

        assert_eq!(
            &m.mem[layout.pilot_name as usize..][..8],
            b"TESTPILO",
            "загруженный пилот сброшен после перезапуска (0x{:04X})",
            layout.pilot_name
        );
        assert_eq!(
            m.mem[ind_danger as usize], 0,
            "после загрузки корабль не пристыкован (IndDanger 0x{:04X})",
            ind_danger
        );

        let _ = std::fs::remove_file(&path);
    }
}

/// Запуск игры с нативной заставки: `start_game` вешает одноразовые хуки
/// на точки цикла заставки, они уводят Z80 в ветку переинициализации, и
/// после `InitGame`/`GameMain` корабль оказывается на станции
/// (`IndDanger == 0`). Клавиши не подаём — запуск делает сам хук.
#[test]
fn start_game_from_intro_proceeds() {
    for (layout, ind_danger, mut m) in [
        (KLADOV_SAVE, 0x7104u16, boot_kladov()),
    ] {
        install_save(&mut m, layout);
        // Даём игре дойти до заставки.
        for _ in 0..60 {
            m.run_frame();
        }

        start_game(&mut m, layout);
        // Должен сработать хук и запустить игру.
        let mut started = false;
        for _ in 0..1200 {
            m.run_frame();
            if m.mem[ind_danger as usize] == 0 {
                started = true;
                break;
            }
        }
        assert!(started, "игра не стартовала по start_game (0x{:04X})", ind_danger);
    }
}

/// Регрессия: после загрузки сейва «в полёте» игра не разваливается.
///
/// Загрузка выполняется родным путём (см. [`request_load`]): игра сама
/// уходит на перезапуск со своего главного цикла. Если же войти в ветку
/// перезапуска вручную с кадровой границы (минуя главный цикл), стек
/// постепенно уезжает в ПЗУ и через ~1000 кадров
/// игра «рассыпается»: атрибуты заполняются мусором, а `SP` уходит ниже
/// `0x4000`. Проверяем, что ни того, ни другого нет.
#[test]
fn load_restart_keeps_screen_clean() {
    let base = env!("CARGO_MANIFEST_DIR");
    let dir = format!("{base}/tmp");
    std::fs::create_dir_all(&dir).unwrap();

    // В файле атрибутов (0x5800..0x5AFF) допустимы только 0x00/0x04/0x07
    // и 0x17 (bright + ink 7). Всё остальное — мусор.
    let is_garbage = |v: u8| v & 0xF8 != 0 && v != 0x10 && v != 0x17;

    // Титул -> старт (импульсы SPACE), затем '1' — вылет со станции.
    let frame_input = |m: &mut Spectrum, f: u64| {
        m.release_all_keys();
        if f >= 30 && f < 900 && f % 16 < 6 {
            m.set_key(7, 0, true); // SPACE
        }
        if (900..903).contains(&f) {
            m.set_key(3, 0, true); // '1' = LaunchDocking
        }
    };
    let run = |m: &mut Spectrum, n: u64| {
        for f in 0..n {
            frame_input(m, f);
            m.run_frame();
        }
    };

    for (layout, ind_danger, mut m) in [
        (KLADOV_SAVE, 0x7104u16, boot_kladov()),
    ] {
        install_save(&mut m, layout);
        // Полёт, в котором сохраняем сейв, затем загрузка и перезапуск.
        run(&mut m, 1508);
        let path = format!("{dir}/load_restart_screen_{:04X}.zxelite", layout.pilot_name);
        commander_save(&mut m, &path, layout).unwrap();
        commander_load(&mut m, &path, layout).unwrap();
        request_load(&mut m, layout);
        // Длинный прогон: прежний сломанный путь разваливался к ~1000 кадров.
        run(&mut m, 3000);
        let _ = std::fs::remove_file(&path);

        assert_ne!(
            m.mem[ind_danger as usize], 0,
            "0x{:04X}: после вылета корабль должен быть в полёте",
            layout.pilot_name
        );
        // Стек игры живёт в экранной памяти (0x40xx); уход ниже 0x4000 —
        // признак того, что игра ушла в ПЗУ и развалилась.
        assert!(
            m.cpu.sp >= 0x4000,
            "0x{:04X}: после загрузки стек ушёл в ПЗУ: SP=0x{:04X}, PC=0x{:04X}",
            layout.pilot_name,
            m.cpu.sp,
            m.cpu.pc
        );
        let garbage: Vec<(usize, u8)> = (0x5800..0x5B00)
            .filter(|&a| is_garbage(m.mem[a]))
            .map(|a| (a, m.mem[a]))
            .collect();
        assert!(
            garbage.is_empty(),
            "0x{:04X}: после загрузки в атрибутах остался мусор: {garbage:04X?}",
            layout.pilot_name
        );
    }
}

/// Регрессия на сохранение из «мёртвого» блока.
///
/// У Кладова блок командира живёт в `PilotName` (0xD054), а `aJameson_1`
/// (0xD232) — тень, из которой `InitGame` восстанавливает пилота при
/// старте. Если читать при сохранении тень, на диск уходит стартовое
/// состояние «JAMESON», прогресс теряется, а после загрузки игра
/// рисует солнце по устаревшим координатам из хвоста тени. Проверяем:
/// сохранение берёт данные из живого блока, а загрузка возвращает их
/// обратно (в т.ч. хвост 93..101, который `InitGame` не переносит).
#[test]
fn save_load_uses_live_commander_block() {
    let base = env!("CARGO_MANIFEST_DIR");
    let dir = format!("{base}/tmp");
    std::fs::create_dir_all(&dir).unwrap();

    // `live` — настоящий живой блок пилота по данным дизассемблирования,
    // независимый от того, что записано в раскладке: у Кладова это
    // назначение `ldir` из тени в `InitGame` (`ld de,0xD054`) и `ld de,
    // PilotName` в `DemoModeOn`.
    for (layout, live, ind_danger, mut m) in [
        (KLADOV_SAVE, 0xD054usize, 0x7104u16, boot_kladov()),
    ] {
        assert_eq!(
            layout.pilot_name as usize, live,
            "раскладка должна читать сохранение из живого блока"
        );
        install_save(&mut m, layout);
        // Имитируем полёт до пристыковки.
        for f in 0..150u64 {
            m.release_all_keys();
            if f >= 30 && f % 16 < 6 {
                m.set_key(7, 0, true);
            }
            m.run_frame();
        }
        let a = live;
        const FREE_CARGO: usize = 40; // PilotFreeCargo
        const SUN: std::ops::Range<usize> = 93..102; // PilotDataOfSun

        // «Игрок» изменил трюм в живом блоке, а солнце на небе сдвинулось —
        // игра хранит его координаты в `SunCrd` и кладёт в блок только при
        // сохранении.
        let sun = [0xF2u8, 0x88, 0x05, 0x27, 0x08, 0x01, 0x2A, 0x4D, 0x8F];
        m.mem[a + FREE_CARGO] = 0x20;
        let sc = layout.sun_crd as usize;
        m.mem[sc..sc + 9].copy_from_slice(&sun);

        let path = format!("{dir}/live_block_{:04X}.zxelite", layout.pilot_name);
        commander_save(&mut m, &path, layout).unwrap();
        let file = std::fs::read(&path).unwrap();
        assert_eq!(
            file[FREE_CARGO], 0x20,
            "0x{:04X}: сохранение не взяло трюм из живого блока (0x{:02X})",
            layout.pilot_name, file[FREE_CARGO]
        );
        assert_eq!(
            &file[SUN.start..SUN.end], &sun,
            "0x{:04X}: сохранение не перенесло SunCrd в PilotDataOfSun",
            layout.pilot_name
        );

        // Загрузка и перезапуск: игра проходит стартовую последовательность
        // и пристыковывается, вернув загруженные данные.
        commander_load(&mut m, &path, layout).unwrap();
        request_load(&mut m, layout);
        for f in 0..700u64 {
            m.release_all_keys();
            if f >= 30 && f % 16 < 6 {
                m.set_key(7, 0, true);
            }
            m.run_frame();
        }
        let ind_danger = ind_danger as usize;
        assert_eq!(
            m.mem[ind_danger], 0,
            "после загрузки корабль не пристыкован (IndDanger 0x{ind_danger:04X})"
        );
        assert_eq!(
            m.mem[a + FREE_CARGO], 0x20,
            "0x{:04X}: трюм не вернулся после загрузки (0x{:02X})",
            layout.pilot_name, m.mem[a + FREE_CARGO]
        );
        assert_eq!(
            &m.mem[a + SUN.start..a + SUN.end], &sun,
            "0x{:04X}: координаты солнца не вернулись после загрузки",
            layout.pilot_name
        );
        // Главное для отрисовки: `InitGame` восстановила `SunCrd` из тени,
        // иначе солнце рисуется по старым координатам.
        let sc = layout.sun_crd as usize;
        assert_eq!(
            &m.mem[sc..sc + 9], &sun,
            "0x{:04X}: SunCrd (0x{:04X}) не восстановилась после загрузки",
            layout.pilot_name, layout.sun_crd
        );

        let _ = std::fs::remove_file(&path);
    }
}

/// Чит `F1`: телепорт «на станцию» выбранной планеты.
///
/// После [`request_teleport`] игра должна оказаться пристыкованной
/// (`IndDanger == 0`) в системе с заданными координатами, а данные планеты
/// (имя) — пересобраны под неё.
#[test]
fn teleport_docks_at_selected_system() {
    // Буфер сгенерированного имени планеты (`TempName`): Кладова 0xDEDB
    for (layout, gal, temp_name, ind_danger, mut m) in [
        (
            KLADOV_SAVE,
            crate::galaxy::KLADOV_GALAXY,
            0xDEDBusize,
            0x7104u16,
            boot_kladov(),
        ),
    ] {
        // Стартовая последовательность: подаём SPACE импульсами, пока игра
        // не окажется на станции.
        for f in 0..900u64 {
            m.release_all_keys();
            if f >= 30 && f % 16 < 6 {
                m.set_key(7, 0, true); // SPACE
            }
            m.run_frame();
        }
        assert_eq!(
            m.mem[ind_danger as usize], 0,
            "подготовка: корабль должен быть на станции"
        );

        // Выбираем планету из модели галактики (не текущую систему).
        let mut seed = [0u8; 6];
        seed.copy_from_slice(&m.mem[gal.seed..gal.seed + 6]);
        let planets = crate::galaxy::planets(seed);
        let sys = (m.mem[gal.sys_x] as i32, m.mem[gal.sys_y] as i32);
        let target = planets
            .iter()
            .find(|p| (p.x, p.y_raw) != sys)
            .expect("в галактике нет планеты кроме текущей системы");
        let (x, y) = (target.x as u8, target.y_raw as u8);
        let want = crate::galaxy::name(target.state, &m.mem[gal.digraphs..gal.digraphs + 64]);

        request_teleport(&mut m, layout, x, y);
        for _ in 0..200u64 {
            m.release_all_keys();
            m.run_frame();
        }
        assert_eq!(
            m.mem[ind_danger as usize], 0,
            "0x{:04X}: после телепорта корабль не пристыкован",
            layout.pilot_name
        );
        let live = layout.pilot_name as usize;
        assert_eq!(
            m.mem[live + CRD_X_SYSTEM_OFF], x,
            "0x{:04X}: X системы не переписана",
            layout.pilot_name
        );
        assert_eq!(
            m.mem[live + CRD_Y_SYSTEM_OFF], y,
            "0x{:04X}: Y системы не переписана",
            layout.pilot_name
        );

        let got: Vec<u8> = m.mem[temp_name..temp_name + 8]
            .iter()
            .copied()
            .take_while(|&b| b != 0)
            .collect();
        assert_eq!(
            String::from_utf8_lossy(&got),
            want,
            "0x{:04X}: имя планеты после телепорта не совпало",
            layout.pilot_name
        );

        // Игра из этого состояния должна нормально вылетать со станции:
        // жмём '1' (LaunchDocking) и проверяем, что корабль в полёте, стек
        // не ушёл в ПЗУ и атрибуты экрана не забиты мусором.
        m.release_all_keys();
        m.set_key(3, 0, true); // '1'
        m.run_frame();
        m.release_all_keys();
        for _ in 0..1200u64 {
            m.run_frame();
        }
        assert_ne!(
            m.mem[ind_danger as usize], 0,
            "0x{:04X}: после вылета со станции корабль не в полёте",
            layout.pilot_name
        );
        assert!(
            m.cpu.sp >= 0x4000,
            "0x{:04X}: после вылета стек ушёл в ПЗУ: SP=0x{:04X}",
            layout.pilot_name,
            m.cpu.sp
        );
        let is_garbage =
            |v: u8| v & 0xF8 != 0 && v != 0x10 && v != 0x17;
        let garbage: Vec<(usize, u8)> = (0x5800..0x5B00)
            .filter(|&a| is_garbage(m.mem[a]))
            .map(|a| (a, m.mem[a]))
            .collect();
        assert!(
            garbage.is_empty(),
            "0x{:04X}: после вылета в атрибутах мусор: {garbage:04X?}",
            layout.pilot_name
        );
    }
}
