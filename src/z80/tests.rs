use super::*;

struct TestBus {
    mem: Vec<u8>,
    io: Vec<u8>,
}
impl TestBus {
    fn new() -> Self {
        TestBus {
            mem: vec![0; 0x10000],
            io: vec![0; 0x10000],
        }
    }
}
impl Bus for TestBus {
    fn read(&mut self, a: u16) -> u8 {
        self.mem[a as usize]
    }
    fn write(&mut self, a: u16, v: u8) {
        self.mem[a as usize] = v;
    }
    fn input(&mut self, p: u16) -> u8 {
        self.io[p as usize]
    }
    fn output(&mut self, p: u16, v: u8) {
        self.io[p as usize] = v;
    }
}

/// The self-modifying scanner-copy routine from Elite, in "save" mode.
/// It must keep the stack balanced and terminate.
#[test]
fn scanner_copy_balanced() {
    let mut bus = TestBus::new();
    let code: [u8; 0x39] = [
        0x11, 0x49, 0x50, // LD DE,5049
        0x21, 0x00, 0x5B, // LD HL,5B00
        0xEB, // EX DE,HL   (save-mode patch)
        0x3E, 0x06, // LD A,6
        0xE5, // PUSH HL    (save-mode patch)
        0x06, 0x08, // LD B,8
        0x0E, 0xFF, // LD C,0FF
        0xE5, // PUSH HL    (save-mode patch)
        0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED,
        0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, // 13x LDI
        0xE1, // POP HL     (save-mode patch)
        0x24, // INC H      (save-mode patch)
        0x10, 0xE1, // DJNZ 0x8E6A
        0xEB, // EX DE,HL   (save-mode patch)
        0xD1, // POP DE
        0xEB, // EX DE,HL
        0x01, 0x20, 0x00, // LD BC,20
        0x09, // ADD HL,BC
        0x00, // NOP        (save-mode patch)
        0x3D, // DEC A
        0x20, 0xD1, // JR NZ,0x8E65
        0xC9, // RET
    ];
    let base = 0x8E5Cusize;
    bus.mem[base..base + code.len()].copy_from_slice(&code);

    let mut cpu = Z80::new();
    cpu.pc = 0x8E5C;
    cpu.sp = 0x8000;
    // push return sentinel 0x1234
    bus.mem[(cpu.sp - 1) as usize] = 0x12;
    bus.mem[(cpu.sp - 2) as usize] = 0x34;
    cpu.sp -= 2;

    let mut steps = 0;
    while cpu.pc != 0x1234 && steps < 2_000_000 {
        cpu.step(&mut bus);
        steps += 1;
        assert!(
            cpu.sp <= 0x8000,
            "stack overflow at step {} pc=0x{:04X}",
            steps,
            cpu.pc
        );
    }
    assert_eq!(
        cpu.pc, 0x1234,
        "routine did not return (steps={}, sp=0x{:04X})",
        steps, cpu.sp
    );
    assert_eq!(cpu.sp, 0x8000, "stack not balanced: sp=0x{:04X}", cpu.sp);
}

/// DD prefix must target IX and FD must target IY (they were once swapped).
#[test]
fn ix_iy_prefixes_not_swapped() {
    let mut bus = TestBus::new();
    let code: [u8; 8] = [
        0xDD, 0x21, 0x59, 0x61, // LD IX,6159
        0xFD, 0x21, 0x00, 0x02, // LD IY,0200
    ];
    bus.mem[0x1000..0x1008].copy_from_slice(&code);
    let mut cpu = Z80::new();
    cpu.pc = 0x1000;
    let iy0 = cpu.iy;
    cpu.step(&mut bus);
    assert_eq!(cpu.ix, 0x6159, "DD 21 must load IX");
    assert_eq!(cpu.iy, iy0, "DD 21 must not touch IY");
    cpu.step(&mut bus);
    assert_eq!(cpu.iy, 0x0200, "FD 21 must load IY");
    assert_eq!(cpu.ix, 0x6159);
}

/// FD E3 = EX (SP),IY must swap IY (not HL) with the word at SP.
#[test]
fn ex_sp_iy_swaps_iy_not_hl() {
    let mut bus = TestBus::new();
    bus.mem[0x1000] = 0xFD;
    bus.mem[0x1001] = 0xE3;
    let mut cpu = Z80::new();
    cpu.pc = 0x1000;
    cpu.sp = 0x8000;
    cpu.iy = 0x6490;
    bus.mem[0x8000] = 0x59;
    bus.mem[0x8001] = 0x61;
    cpu.step(&mut bus);
    assert_eq!(cpu.iy, 0x6159, "EX (SP),IY must swap IY with (SP)");
    let w = bus.mem[0x8000] as u16 | ((bus.mem[0x8001] as u16) << 8);
    assert_eq!(w, 0x6490, "(SP) must receive old IY");
}

/// Run one instruction (possibly prefixed) from 0x1000 and return the CPU.
fn run1(bytes: &[u8]) -> (Z80, TestBus) {
    let mut bus = TestBus::new();
    bus.mem[0x1000..0x1000 + bytes.len()].copy_from_slice(bytes);
    let mut cpu = Z80::new();
    cpu.pc = 0x1000;
    cpu.step(&mut bus);
    (cpu, bus)
}

#[test]
fn rotates_and_shifts() {
    // RRA: a=(carry<<7)|(a>>1), carry=old a&1
    let (mut cpu, _) = run1(&[0x1F]); // RRA
    cpu.a = 0x20;
    cpu.f = CF;
    let (cpu2, _) = {
        let mut b = TestBus::new();
        b.mem[0x1000] = 0x1F;
        let mut c = cpu;
        c.pc = 0x1000;
        c.step(&mut b);
        (c, b)
    };
    assert_eq!(cpu2.a, 0x90, "RRA 0x20 with C=1 -> 0x90");
    assert_eq!(cpu2.f & CF, 0, "RRA carry = old bit0 (0)");

    // RLA: a=(a<<1)|carry, carry=old a&0x80
    let mut b = TestBus::new();
    b.mem[0x1000] = 0x17;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x20;
    c.f = CF;
    c.step(&mut b);
    assert_eq!(c.a, 0x41, "RLA 0x20 with C=1 -> 0x41");

    // RRCA / RLCA values
    let mut b = TestBus::new();
    b.mem[0x1000] = 0x0F;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x81;
    c.step(&mut b);
    assert_eq!(c.a, 0xC0, "RRCA 0x81 -> 0xC0");
    assert_eq!(c.f & CF, CF, "RRCA carry = old bit0");

    // SRA (CB 2F): 0x82 -> 0xC1
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0x2F;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x82;
    c.step(&mut b);
    assert_eq!(c.a, 0xC1, "SRA 0x82 -> 0xC1");
    // SRL (CB 3F): 0x81 -> 0x40, C=1
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0x3F;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x81;
    c.step(&mut b);
    assert_eq!(c.a, 0x40, "SRL 0x81 -> 0x40");
    assert_eq!(c.f & CF, CF, "SRL carry = old bit0");
}

#[test]
fn bit_set_res_ops() {
    // BIT 3,B (CB 58): b=0x08 -> Z=0 ; b=0 -> Z=1
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0x58;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.b = 0x08;
    c.step(&mut b);
    assert_eq!(c.f & ZF, 0, "BIT 3,B with bit set -> Z clear");
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0x58;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.b = 0x00;
    c.step(&mut b);
    assert_eq!(c.f & ZF, ZF, "BIT 3,B with bit clear -> Z set");
    // SET 0,B (CB C0): b=0 -> 1
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0xC0;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.b = 0x00;
    c.step(&mut b);
    assert_eq!(c.b, 0x01, "SET 0,B");
    // RES 5,B (CB A8): b=0x20 -> 0x00
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xCB;
    b.mem[0x1001] = 0xA8;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.b = 0x20;
    c.step(&mut b);
    assert_eq!(c.b, 0x00, "RES 5,B");
}

#[test]
fn sub_and_cp_flags() {
    // CP 0x5B with a=0x20: carry set (borrow), a unchanged
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xFE;
    b.mem[0x1001] = 0x5B;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x20;
    c.step(&mut b);
    assert_eq!(c.a, 0x20, "CP must not change A");
    assert_eq!(c.f & CF, CF, "CP 0x20-0x5B must set carry");
    // CP 0x80 with a=0xFF: carry clear
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xFE;
    b.mem[0x1001] = 0x80;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0xFF;
    c.step(&mut b);
    assert_eq!(c.f & CF, 0, "CP 0xFF-0x80 must clear carry");

    // CP must NOT modify A (register operand: B8 = CP B)
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xB8;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x20;
    c.b = 0x5B;
    c.step(&mut b);
    assert_eq!(c.a, 0x20, "CP B must keep A=0x20");
    assert_eq!(c.f & CF, CF, "CP B 0x20 vs 0x5B -> C set");

    // CP (IX+d) (DD BE) must NOT modify A
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0xBE;
    b.mem[0x1002] = 0x00;
    b.mem[0x6490] = 0x5B;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.a = 0x20;
    c.ix = 0x6490;
    c.step(&mut b);
    assert_eq!(c.a, 0x20, "CP (IX+d) must keep A=0x20");
    assert_eq!(c.f & CF, CF, "CP (IX+d) 0x20 vs 0x5B -> C set");
}

#[test]
fn exx_swaps_all() {
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xD9; // EXX
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.b = 0x11;
    c.c = 0x22;
    c.d = 0x33;
    c.e = 0x44;
    c.h = 0x55;
    c.l = 0x66;
    c.b_ = 0xAA;
    c.c_ = 0xBB;
    c.d_ = 0xCC;
    c.e_ = 0xDD;
    c.h_ = 0xEE;
    c.l_ = 0xFF;
    c.step(&mut b);
    assert_eq!(c.bc(), 0xAABB, "EXX swaps BC");
    assert_eq!(c.de(), 0xCCDD, "EXX swaps DE");
    assert_eq!(c.hl(), 0xEEFF, "EXX swaps HL");
}

#[test]
fn ixh_ixl_ld_variants() {
    // DD 68 = LD IXL,B
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x68;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x1122;
    c.b = 0x55;
    c.step(&mut b);
    assert_eq!(c.ix, 0x1155, "DD 68 = LD IXL,B");
    // DD 69 = LD IXL,C
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x69;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x1122;
    c.c = 0x77;
    c.step(&mut b);
    assert_eq!(c.ix, 0x1177, "DD 69 = LD IXL,C");
    // DD 44 = LD B,IXH
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x44;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x1122;
    c.step(&mut b);
    assert_eq!(c.b, 0x11, "DD 44 = LD B,IXH");
    // DD 7D = LD A,IXL
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x7D;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x1122;
    c.step(&mut b);
    assert_eq!(c.a, 0x22, "DD 7D = LD A,IXL");
    // FD 6C = LD IYL,IYH
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xFD;
    b.mem[0x1001] = 0x6C;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.iy = 0xAABB;
    c.step(&mut b);
    assert_eq!(c.iy, 0xAAAA, "FD 6C = LD IYL,IYH");
}

#[test]
fn alu_a_ixh_ixl() {
    // DD 84 = ADD A,IXH
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x84;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x1100;
    c.a = 0x05;
    c.step(&mut b);
    assert_eq!(c.a, 0x16, "DD 84 = ADD A,IXH");
    // DD 85 = ADD A,IXL
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0x85;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x0022;
    c.a = 0x05;
    c.step(&mut b);
    assert_eq!(c.a, 0x27, "DD 85 = ADD A,IXL");
    // DD BC = CP IXH, must keep A
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xDD;
    b.mem[0x1001] = 0xBC;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.ix = 0x5B00;
    c.a = 0x20;
    c.step(&mut b);
    assert_eq!(c.a, 0x20, "DD BC = CP IXH keeps A");
    assert_eq!(c.f & CF, CF, "DD BC 0x20 vs 0x5B -> C set");
    // FD AD = XOR IYL
    let mut b = TestBus::new();
    b.mem[0x1000] = 0xFD;
    b.mem[0x1001] = 0xAD;
    let mut c = Z80::new();
    c.pc = 0x1000;
    c.iy = 0x00F0;
    c.a = 0xFF;
    c.step(&mut b);
    assert_eq!(c.a, 0x0F, "FD AD = XOR IYL");
}

/// Full instruction-level audit against ZEXDOC (CP/M test binary).
/// Run with: cargo test --release zexdoc -- --ignored --nocapture
#[test]
#[ignore]
fn zexdoc() {
    use std::io::Write;
    let path = format!("{}/tmp/zexdoc.com", env!("CARGO_MANIFEST_DIR"));
    let prog = std::fs::read(&path).unwrap_or_else(|_| panic!("{path} not found (see PLAN.md)"));
    let mut bus = TestBus::new();
    bus.mem[0x100..0x100 + prog.len()].copy_from_slice(&prog);
    let mut cpu = Z80::new();
    cpu.pc = 0x0100;
    cpu.sp = 0xFF00;
    let mut out = String::new();
    let mut steps: u64 = 0;
    loop {
        if cpu.pc == 0x0005 {
            match cpu.c {
                2 => {
                    out.push(cpu.e as char);
                    print!("{}", cpu.e as char);
                    std::io::stdout().flush().ok();
                }
                9 => {
                    let mut addr = cpu.de();
                    loop {
                        let ch = bus.mem[addr as usize];
                        if ch == b'$' {
                            break;
                        }
                        out.push(ch as char);
                        print!("{}", ch as char);
                        addr = addr.wrapping_add(1);
                    }
                    std::io::stdout().flush().ok();
                }
                0 => {
                    println!("\n[exit]");
                    break;
                }
                _ => {
                    println!("\n[unknown BDOS c={}]", cpu.c);
                    break;
                }
            }
            cpu.pc = bus.mem[cpu.sp as usize] as u16 | ((bus.mem[cpu.sp as usize + 1] as u16) << 8);
            cpu.sp = cpu.sp.wrapping_add(2);
            continue;
        }
        if cpu.pc == 0x0000 {
            println!("\n[warm boot]");
            break;
        }
        cpu.step(&mut bus);
        steps += 1;
        if steps > 120_000_000_000 {
            println!("\n[step cap]");
            break;
        }
    }
    println!("\nsteps={}", steps);
    assert!(!out.contains("ERROR"), "ZEXDOC reported instruction errors");
}
