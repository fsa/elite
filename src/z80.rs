//! A Zilog Z80 CPU emulator core.
//!
//! Implements the documented instruction set plus the widely used
//! undocumented instructions (SLL, DDCB/FDCB variants, IXH/IXL/IYH/IYL).

pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn input(&mut self, port: u16) -> u8;
    fn output(&mut self, port: u16, val: u8);
}

const CF: u8 = 0x01;
const NF: u8 = 0x02;
const PF: u8 = 0x04;
const XF: u8 = 0x08;
const HF: u8 = 0x10;
const YF: u8 = 0x20;
const ZF: u8 = 0x40;
const SF: u8 = 0x80;

#[derive(Clone)]
pub struct Z80 {    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a_: u8,
    pub f_: u8,
    pub b_: u8,
    pub c_: u8,
    pub d_: u8,
    pub e_: u8,
    pub h_: u8,
    pub l_: u8,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub i: u8,
    pub r: u8,
    pub iff1: bool,
    pub iff2: bool,
    pub im: u8,
    pub halted: bool,
    pub ei_pending: bool,
    pub index_iy: bool,
    pub cycles: u64,
}

fn parity(v: u8) -> bool {
    v.count_ones() % 2 == 0
}

impl Z80 {
    pub fn new() -> Self {
        Z80 {
            a: 0xFF, f: 0xFF, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            a_: 0, f_: 0, b_: 0, c_: 0, d_: 0, e_: 0, h_: 0, l_: 0,
            ix: 0xFFFF, iy: 0xFFFF, sp: 0xFFFF, pc: 0,
            i: 0, r: 0, iff1: false, iff2: false, im: 0, halted: false,
            ei_pending: false, index_iy: false, cycles: 0,
        }
    }

    #[inline]
    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) | self.c as u16 }
    #[inline]
    pub fn de(&self) -> u16 { ((self.d as u16) << 8) | self.e as u16 }    #[inline]
    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }
    #[inline]
    fn af(&self) -> u16 { ((self.a as u16) << 8) | self.f as u16 }
    #[inline]
    fn set_bc(&mut self, v: u16) { self.b = (v >> 8) as u8; self.c = v as u8; }
    #[inline]
    fn set_de(&mut self, v: u16) { self.d = (v >> 8) as u8; self.e = v as u8; }
    #[inline]
    fn set_hl(&mut self, v: u16) { self.h = (v >> 8) as u8; self.l = v as u8; }
    #[inline]
    fn set_af(&mut self, v: u16) { self.a = (v >> 8) as u8; self.f = v as u8; }

    #[inline]
    fn zf(&self) -> bool { self.f & ZF != 0 }
    #[inline]
    fn cf(&self) -> bool { self.f & CF != 0 }

    // flags: S, Z, X/Y from result; reset H,N,C
    #[inline]
    fn sz53(&mut self, v: u8) {
        self.f = (self.f & (CF)) | if v == 0 { ZF } else { 0 } | (v & (SF | XF | YF));
        self.f &= !(HF | NF);
    }
    #[inline]
    fn sz53p(&mut self, v: u8) {
        self.f = (self.f & CF)
            | if v == 0 { ZF } else { 0 }
            | (v & (SF | XF | YF))
            | if parity(v) { PF } else { 0 };
        self.f &= !(HF | NF);
    }

    #[inline]
    fn inc8(&mut self, v: u8) -> u8 {
        let r = v.wrapping_add(1);
        let c = self.f & CF;
        self.f = c
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (v & 0x0F) == 0x0F { HF } else { 0 }
            | if v == 0x7F { PF } else { 0 };
        r
    }
    #[inline]
    fn dec8(&mut self, v: u8) -> u8 {
        let r = v.wrapping_sub(1);
        let c = self.f & CF;
        self.f = c
            | NF
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (v & 0x0F) == 0x00 { HF } else { 0 }
            | if v == 0x80 { PF } else { 0 };
        r
    }
    #[inline]
    fn add8(&mut self, a: u8, v: u8, carry: u8) -> u8 {
        let r16 = a as u16 + v as u16 + carry as u16;
        let r = r16 as u8;
        self.f = if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (a & 0x0F) + (v & 0x0F) + carry > 0x0F { HF } else { 0 }
            | if r16 > 0xFF { CF } else { 0 }
            | if !((a ^ v) & 0x80 != 0) && ((a ^ r) & 0x80 != 0) { PF } else { 0 };
        r
    }
    #[inline]
    fn sub8(&mut self, a: u8, v: u8, carry: u8) -> u8 {
        let r = a.wrapping_sub(v).wrapping_sub(carry);
        self.f = NF
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (a & 0x0F) < (v & 0x0F) + carry { HF } else { 0 }
            | if (a as u16) < (v as u16) + carry as u16 { CF } else { 0 }
            | if ((a ^ v) & 0x80 != 0) && ((a ^ r) & 0x80 != 0) { PF } else { 0 };
        r
    }
    #[inline]
    fn and8(&mut self, a: u8, v: u8) -> u8 {
        let r = a & v;
        self.f = HF | (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn or8(&mut self, a: u8, v: u8) -> u8 {
        let r = a | v;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn xor8(&mut self, a: u8, v: u8) -> u8 {
        let r = a ^ v;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn add16(&mut self, a: u16, b: u16) -> u16 {
        let r32 = a as u32 + b as u32;
        let r = r32 as u16;
        self.f = (self.f & (SF | ZF | PF))
            | ((r >> 8) as u8 & (XF | YF))
            | if ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF { HF } else { 0 }
            | if r32 > 0xFFFF { CF } else { 0 };
        r
    }
    #[inline]
    fn adc16(&mut self, a: u16, b: u16) -> u16 {
        let carry = if self.f & CF != 0 { 1u32 } else { 0 };
        let r32 = a as u32 + b as u32 + carry;
        let r = r32 as u16;
        self.f = if r == 0 { ZF } else { 0 }
            | (r32 as u8 & (XF | YF))
            | ((r >> 8) as u8 & SF)
            | if ((a & 0x0FFF) + (b & 0x0FFF) + carry as u16) > 0x0FFF { HF } else { 0 }
            | if r32 > 0xFFFF { CF } else { 0 }
            | if !((a ^ b) & 0x8000 != 0) && ((a ^ r) & 0x8000 != 0) { PF } else { 0 };
        r
    }
    #[inline]
    fn sbc16(&mut self, a: u16, b: u16) -> u16 {
        let carry = if self.f & CF != 0 { 1u32 } else { 0 };
        let r32 = (a as u32).wrapping_sub(b as u32).wrapping_sub(carry);
        let r = r32 as u16;
        self.f = NF
            | if r == 0 { ZF } else { 0 }
            | (r32 as u8 & (XF | YF))
            | ((r >> 8) as u8 & SF)
            | if (a & 0x0FFF) < (b & 0x0FFF) + carry as u16 { HF } else { 0 }
            | if (a as u32) < (b as u32) + carry { CF } else { 0 }
            | if ((a ^ b) & 0x8000 != 0) && ((a ^ r) & 0x8000 != 0) { PF } else { 0 };
        r
    }
    #[inline]
    fn daa(&mut self) -> u8 {
        let mut correction = 0u8;
        let mut carry = self.f & CF != 0;
        if self.f & HF != 0 || (self.a & 0x0F) > 9 {
            correction |= 0x06;
        }
        if carry || self.a > 0x99 {
            correction |= 0x60;
            carry = true;
        }
        let old_a = self.a;
        let a = if self.f & NF != 0 {
            old_a.wrapping_sub(correction)
        } else {
            old_a.wrapping_add(correction)
        };
        self.a = a;
        let h = if self.f & NF != 0 {
            (old_a & 0x0F) < (correction & 0x0F)
        } else {
            ((old_a & 0x0F) + (correction & 0x0F)) > 0x0F
        };
        self.f = (self.f & NF)
            | if a == 0 { ZF } else { 0 }
            | (a & (SF | XF | YF))
            | if h { HF } else { 0 }
            | if carry { CF } else { 0 }
            | if parity(a) { PF } else { 0 };
        a
    }
    #[inline]
    fn rlc(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | c;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn rrc(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | (c << 7);
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn rl(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | if self.f & CF != 0 { 1 } else { 0 };
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn rr(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | if self.f & CF != 0 { 0x80 } else { 0 };
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn sla(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = v << 1;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn sra(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | (v & 0x80);
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn sll(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | 1;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn srl(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = v >> 1;
        self.f = (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if c != 0 { CF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    fn bit(&mut self, n: u8, v: u8) {
        let bit = v & (1 << (n & 7));
        self.f = (self.f & CF) | HF
            | if bit == 0 { ZF | PF } else { 0 }
            | (bit & SF)
            | (v & (XF | YF));
    }

    fn push(&mut self, bus: &mut dyn Bus, v: u16) {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, (v >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, v as u8);
    }
    fn pop(&mut self, bus: &mut dyn Bus) -> u16 {
        let lo = bus.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi = bus.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (hi << 8) | lo
    }

    #[inline]
    fn fetch(&mut self, bus: &mut dyn Bus) -> u8 {
        let v = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }
    #[inline]
    fn fetch16(&mut self, bus: &mut dyn Bus) -> u16 {
        let lo = self.fetch(bus) as u16;
        let hi = self.fetch(bus) as u16;
        (hi << 8) | lo
    }
    #[inline]
    fn read16(&mut self, bus: &mut dyn Bus, a: u16) -> u16 {
        let lo = bus.read(a) as u16;
        let hi = bus.read(a.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    #[inline]
    fn write16(&mut self, bus: &mut dyn Bus, a: u16, v: u16) {
        bus.write(a, v as u8);
        bus.write(a.wrapping_add(1), (v >> 8) as u8);
    }

    #[inline]
    fn cond(&self, c: u8) -> bool {
        match c {
            0 => self.f & ZF == 0,
            1 => self.f & ZF != 0,
            2 => self.f & CF == 0,
            3 => self.f & CF != 0,
            4 => self.f & PF == 0,
            5 => self.f & PF != 0,
            6 => self.f & SF == 0,
            7 => self.f & SF != 0,
            _ => false,
        }
    }

    fn get_r(&self, i: u8) -> u8 {
        match i {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => 0,
            _ => self.a,
        }
    }
    fn set_r(&mut self, i: u8, v: u8) {
        match i {
            0 => self.b = v,
            1 => self.c = v,
            2 => self.d = v,
            3 => self.e = v,
            4 => self.h = v,
            5 => self.l = v,
            6 => {}
            _ => self.a = v,
        }
    }

    fn rp(&self, p: u8) -> u16 {
        match p {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            _ => self.sp,
        }
    }
    fn set_rp(&mut self, p: u8, v: u16) {
        match p {
            0 => self.set_bc(v),
            1 => self.set_de(v),
            2 => self.set_hl(v),
            _ => self.sp = v,
        }
    }
    fn rp2(&self, p: u8) -> u16 {
        match p {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            _ => self.af(),
        }
    }
    fn set_rp2(&mut self, p: u8, v: u16) {
        match p {
            0 => self.set_bc(v),
            1 => self.set_de(v),
            2 => self.set_hl(v),
            _ => self.set_af(v),
        }
    }

    /// Execute one instruction; returns elapsed T-states.
    pub fn step(&mut self, bus: &mut dyn Bus) -> u32 {
        if self.halted {
            // HALT repeats until interrupt; charges 4 cycles and does nothing.
            self.cycles += 4;
            return 4;
        }
        let start = self.cycles;
        self.r = (self.r & 0x80) | (self.r.wrapping_add(1) & 0x7F);
        let op = self.fetch(bus);
        self.execute(bus, op);
        let elapsed = (self.cycles - start) as u32;
        elapsed
    }

    fn execute(&mut self, bus: &mut dyn Bus, op: u8) {
        match op {
            0x00 => self.cycles += 4, // NOP
            0x08 => { std::mem::swap(&mut self.a, &mut self.a_); std::mem::swap(&mut self.f, &mut self.f_); self.cycles += 4; }
            0x10 => { // DJNZ d
                let d = self.fetch(bus) as i8;
                self.b = self.b.wrapping_sub(1);
                if self.b != 0 { self.pc = self.pc.wrapping_add(d as u16); self.cycles += 13; } else { self.cycles += 8; }
            }
            0x18 => { let d = self.fetch(bus) as i8; self.pc = self.pc.wrapping_add(d as u16); self.cycles += 12; }
            0x20..=0x38 if op & 0x07 == 0 => { // JR cc
                let cc = (op >> 3) & 3;
                let d = self.fetch(bus) as i8;
                if self.cond(cc) { self.pc = self.pc.wrapping_add(d as u16); self.cycles += 12; } else { self.cycles += 7; }
            }
            0x01 | 0x11 | 0x21 | 0x31 => { // LD rr,nn
                let v = self.fetch16(bus);
                self.set_rp((op >> 4) & 3, v);
                self.cycles += 10;
            }
            0x09 | 0x19 | 0x29 | 0x39 => { // ADD HL,rr
                let r = self.add16(self.hl(), self.rp((op >> 4) & 3));
                self.set_hl(r);
                self.cycles += 11;
            }
            0x02 => { bus.write(self.bc(), self.a); self.cycles += 7; }
            0x12 => { bus.write(self.de(), self.a); self.cycles += 7; }
            0x22 => { let a = self.fetch16(bus); self.write16(bus, a, self.hl()); self.cycles += 16; }
            0x32 => { let a = self.fetch16(bus); bus.write(a, self.a); self.cycles += 13; }
            0x0A => { self.a = bus.read(self.bc()); self.cycles += 7; }
            0x1A => { self.a = bus.read(self.de()); self.cycles += 7; }
            0x2A => { let a = self.fetch16(bus); let v = self.read16(bus, a); self.set_hl(v); self.cycles += 16; }
            0x3A => { let a = self.fetch16(bus); self.a = bus.read(a); self.cycles += 13; }
            0x03 | 0x13 | 0x23 | 0x33 => { let p = (op >> 4) & 3; let v = self.rp(p).wrapping_add(1); self.set_rp(p, v); self.cycles += 6; }
            0x0B | 0x1B | 0x2B | 0x3B => { let p = (op >> 4) & 3; let v = self.rp(p).wrapping_sub(1); self.set_rp(p, v); self.cycles += 6; }
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => { // INC r
                let i = (op >> 3) & 7;
                if i == 6 {
                    let a = self.hl();
                    let v = bus.read(a);
                    let r = self.inc8(v);
                    bus.write(a, r);
                    self.cycles += 11;
                } else {
                    let v = self.get_r(i);
                    let r = self.inc8(v);
                    self.set_r(i, r);
                    self.cycles += 4;
                }
            }
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => { // DEC r
                let i = (op >> 3) & 7;
                if i == 6 {
                    let a = self.hl();
                    let v = bus.read(a);
                    let r = self.dec8(v);
                    bus.write(a, r);
                    self.cycles += 11;
                } else {
                    let v = self.get_r(i);
                    let r = self.dec8(v);
                    self.set_r(i, r);
                    self.cycles += 4;
                }
            }
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => { // LD r,n
                let i = (op >> 3) & 7;
                let n = self.fetch(bus);
                if i == 6 { let a = self.hl(); bus.write(a, n); self.cycles += 10; }
                else { self.set_r(i, n); self.cycles += 7; }
            }
            0x07 => { let c = self.a >> 7; self.a = (self.a << 1) | c; self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 }; self.cycles += 4; }
            0x0F => { let c = self.a & 1; self.a = (self.a >> 1) | (c << 7); self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 }; self.cycles += 4; }
            0x17 => { let c = self.a >> 7; self.a = (self.a << 1) | if self.f & CF != 0 { 1 } else { 0 }; self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 }; self.cycles += 4; }
            0x1F => { let c = self.a & 1; self.a = (self.a >> 1) | if self.f & CF != 0 { 0x80 } else { 0 }; self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 }; self.cycles += 4; }
            0x27 => { self.daa(); self.cycles += 4; }
            0x2F => { self.a = !self.a; self.f = (self.f & (SF | ZF | PF | CF)) | HF | NF | (self.a & (XF | YF)); self.cycles += 4; }
            0x37 => { self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | CF; self.cycles += 4; }
            0x3F => { let c = self.f & CF; self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { HF } else { 0 } | if c != 0 { 0 } else { CF }; self.cycles += 4; }
            0x76 => { self.halted = true; self.cycles += 4; }
            0x40..=0x7F if op != 0x76 => { // LD r,r'  (HALT handled)
                let dst = (op >> 3) & 7;
                let src = op & 7;
                if dst == 6 && src == 6 { self.halted = true; self.cycles += 4; }
                else if src == 6 { let a = self.hl(); let v = bus.read(a); self.set_r(dst, v); self.cycles += 7; }
                else if dst == 6 { let a = self.hl(); bus.write(a, self.get_r(src)); self.cycles += 7; }
                else { let v = self.get_r(src); self.set_r(dst, v); self.cycles += 4; }
            }
            0x80..=0xBF => { // ALU A,r
                let src = op & 7;
                let v = if src == 6 { bus.read(self.hl()) } else { self.get_r(src) };
                let group = (op >> 3) & 7;
                let a = self.a;
                self.a = match group {
                    0 => self.add8(a, v, 0),
                    1 => self.add8(a, v, if self.f & CF != 0 { 1 } else { 0 }),
                    2 => self.sub8(a, v, 0),
                    3 => self.sub8(a, v, if self.f & CF != 0 { 1 } else { 0 }),
                    4 => self.and8(a, v),
                    5 => self.xor8(a, v),
                    6 => self.or8(a, v),
                    _ => { self.sub8(a, v, 0); a } // CP: flags only, keep A
                };
                self.cycles += if src == 6 { 7 } else { 4 };
            }
            0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => { // RET cc
                if self.cond((op >> 3) & 7) { self.pc = self.pop(bus); self.cycles += 11; } else { self.cycles += 5; }
            }
            0xC1 | 0xD1 | 0xE1 | 0xF1 => { let v = self.pop(bus); self.set_rp2((op >> 4) & 3, v); self.cycles += 10; }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => { self.push(bus, self.rp2((op >> 4) & 3)); self.cycles += 11; }
            0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => { // JP cc,nn
                let a = self.fetch16(bus);
                if self.cond((op >> 3) & 7) { self.pc = a; }
                self.cycles += 10;
            }
            0xC3 => { self.pc = self.fetch16(bus); self.cycles += 10; }
            0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => { // CALL cc,nn
                let a = self.fetch16(bus);
                if self.cond((op >> 3) & 7) { self.push(bus, self.pc); self.pc = a; self.cycles += 17; } else { self.cycles += 10; }
            }
            0xCD => { let a = self.fetch16(bus); self.push(bus, self.pc); self.pc = a; self.cycles += 17; }
            0xC6 => { let v = self.fetch(bus); let a = self.a; self.a = self.add8(a, v, 0); self.cycles += 7; }
            0xCE => { let v = self.fetch(bus); let a = self.a; let c = if self.f & CF != 0 { 1 } else { 0 }; self.a = self.add8(a, v, c); self.cycles += 7; }
            0xD6 => { let v = self.fetch(bus); let a = self.a; self.a = self.sub8(a, v, 0); self.cycles += 7; }
            0xDE => { let v = self.fetch(bus); let a = self.a; let c = if self.f & CF != 0 { 1 } else { 0 }; self.a = self.sub8(a, v, c); self.cycles += 7; }
            0xE6 => { let v = self.fetch(bus); let a = self.a; self.a = self.and8(a, v); self.cycles += 7; }
            0xEE => { let v = self.fetch(bus); let a = self.a; self.a = self.xor8(a, v); self.cycles += 7; }
            0xF6 => { let v = self.fetch(bus); let a = self.a; self.a = self.or8(a, v); self.cycles += 7; }
            0xFE => { let v = self.fetch(bus); let a = self.a; self.sub8(a, v, 0); self.cycles += 7; }
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => { self.push(bus, self.pc); self.pc = (op & 0x38) as u16; self.cycles += 11; }
            0xC9 => { self.pc = self.pop(bus); self.cycles += 10; }
            0xD9 => { // EXX
                std::mem::swap(&mut self.b, &mut self.b_); std::mem::swap(&mut self.c, &mut self.c_);
                std::mem::swap(&mut self.d, &mut self.d_); std::mem::swap(&mut self.e, &mut self.e_);
                std::mem::swap(&mut self.h, &mut self.h_); std::mem::swap(&mut self.l, &mut self.l_);
                self.cycles += 4;
            }
            0xE3 => { let t = self.read16(bus, self.sp); self.write16(bus, self.sp, self.hl()); self.set_hl(t); self.cycles += 19; }
            0xE9 => { self.pc = self.hl(); self.cycles += 4; }
            0xEB => { let t = self.de(); self.set_de(self.hl()); self.set_hl(t); self.cycles += 4; }
            0xF3 => { self.iff1 = false; self.iff2 = false; self.ei_pending = false; self.cycles += 4; }
            0xFB => { self.iff1 = true; self.iff2 = true; self.ei_pending = true; self.cycles += 4; }
            0xF9 => { self.sp = self.hl(); self.cycles += 6; }
            _ => self.extended(bus, op),
        }
    }

    fn extended(&mut self, bus: &mut dyn Bus, op: u8) {
        match op {
            0xCB => self.cb(bus),
            0xED => self.ed(bus),
            0xDD => self.index_prefix(bus, false),
            0xFD => self.index_prefix(bus, true),
            0xD3 => { let p = self.fetch(bus) as u16 | ((self.a as u16) << 8); bus.output(p, self.a); self.cycles += 11; }
            0xDB => { let p = self.fetch(bus) as u16 | ((self.a as u16) << 8); self.a = bus.input(p); self.cycles += 11; }
            _ => { self.cycles += 4; } // undocumented NOPs
        }
    }

    fn cb(&mut self, bus: &mut dyn Bus) {
        let op = self.fetch(bus);
        let i = op & 7;
        let group = op >> 6;
        let bitn = (op >> 3) & 7;
        if i == 6 {
            let a = self.hl();
            let v = bus.read(a);
            let new = match group {
                0 => self.shift(op >> 3, v),
                1 => { self.bit(op >> 3, v); self.cycles += 12; return; }
                2 => v & !(1 << bitn),
                _ => v | (1 << bitn),
            };
            bus.write(a, new);
            self.cycles += 15;
        } else {
            let v = self.get_r(i);
            let new = match group {
                0 => self.shift(op >> 3, v),
                1 => { self.bit(op >> 3, v); self.cycles += 8; return; }
                2 => v & !(1 << bitn),
                _ => v | (1 << bitn),
            };
            self.set_r(i, new);
            self.cycles += 8;
        }
    }
    fn shift(&mut self, kind: u8, v: u8) -> u8 {
        match kind & 7 {
            0 => self.rlc(v),
            1 => self.rrc(v),
            2 => self.rl(v),
            3 => self.rr(v),
            4 => self.sla(v),
            5 => self.sra(v),
            6 => self.sll(v),
            _ => self.srl(v),
        }
    }

    fn ed(&mut self, bus: &mut dyn Bus) {
        let op = self.fetch(bus);
        match op {
            0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => { // IN r,(C)
                let p = self.bc();
                let v = bus.input(p);
                self.f = (self.f & CF) | (v & (SF | XF | YF)) | if v == 0 { ZF } else { 0 } | if parity(v) { PF } else { 0 };
                let i = (op >> 3) & 7;
                if i != 6 { self.set_r(i, v); }
                self.cycles += 12;
            }
            0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x71 | 0x79 => { // OUT (C),r
                let p = self.bc();
                let i = (op >> 3) & 7;
                let v = if i == 6 { 0 } else { self.get_r(i) };
                bus.output(p, v);
                self.cycles += 12;
            }
            0x42 | 0x52 | 0x62 | 0x72 => { let v = self.sbc16(self.hl(), self.rp((op >> 4) & 3)); self.set_hl(v); self.cycles += 15; }
            0x4A | 0x5A | 0x6A | 0x7A => { let v = self.adc16(self.hl(), self.rp((op >> 4) & 3)); self.set_hl(v); self.cycles += 15; }
            0x43 | 0x53 | 0x63 | 0x73 => { let a = self.fetch16(bus); self.write16(bus, a, self.rp((op >> 4) & 3)); self.cycles += 20; }
            0x4B | 0x5B | 0x6B | 0x7B => { let a = self.fetch16(bus); let v = self.read16(bus, a); self.set_rp((op >> 4) & 3, v); self.cycles += 20; }
            0x44 | 0x4C | 0x54 | 0x5C | 0x64 | 0x6C | 0x74 | 0x7C => { // NEG
                let a = self.a; self.a = self.sub8(0, a, 0); self.cycles += 8;
            }
            0x45 | 0x4D | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D => { self.pc = self.pop(bus); self.iff1 = self.iff2; self.cycles += 14; } // RETN
            0x46 | 0x4E | 0x66 | 0x6E => { self.im = 0; self.cycles += 8; }
            0x56 | 0x76 => { self.im = 1; self.cycles += 8; }
            0x5E | 0x7E => { self.im = 2; self.cycles += 8; }
            0x47 => { self.i = self.a; self.cycles += 9; }
            0x4F => { self.r = self.a; self.cycles += 9; }
            0x57 => { self.a = self.i; self.f = (self.f & CF) | (self.a & (SF | XF | YF)) | if self.a == 0 { ZF } else { 0 } | if self.iff2 { PF } else { 0 }; self.cycles += 9; }
            0x5F => { self.a = self.r; self.f = (self.f & CF) | (self.a & (SF | XF | YF)) | if self.a == 0 { ZF } else { 0 } | if self.iff2 { PF } else { 0 }; self.cycles += 9; }
            0x67 => { // RRD
                let a = self.hl();
                let v = bus.read(a);
                let new = (v >> 4) | (self.a << 4);
                bus.write(a, new);
                self.a = (self.a & 0xF0) | (v & 0x0F);
                self.f = (self.f & CF) | (self.a & (SF | XF | YF)) | if self.a == 0 { ZF } else { 0 } | if parity(self.a) { PF } else { 0 };
                self.cycles += 18;
            }
            0x6F => { // RLD
                let a = self.hl();
                let v = bus.read(a);
                let new = (v << 4) | (self.a & 0x0F);
                bus.write(a, new);
                self.a = (self.a & 0xF0) | (v >> 4);
                self.f = (self.f & CF) | (self.a & (SF | XF | YF)) | if self.a == 0 { ZF } else { 0 } | if parity(self.a) { PF } else { 0 };
                self.cycles += 18;
            }
            0xA0 => self.block_ldi(bus),
            0xA1 => self.block_cpi(bus),
            0xA2 => self.block_ini(bus),
            0xA3 => self.block_outi(bus),
            0xA8 => self.block_ldd(bus),
            0xA9 => self.block_cpd(bus),
            0xAA => self.block_ind(bus),
            0xAB => self.block_outd(bus),
            0xB0 => { self.block_ldi(bus); if self.f & PF != 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xB1 => { self.block_cpi(bus); if self.f & PF != 0 && self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xB2 => { self.block_ini(bus); if self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xB3 => { self.block_outi(bus); if self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xB8 => { self.block_ldd(bus); if self.f & PF != 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xB9 => { self.block_cpd(bus); if self.f & PF != 0 && self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xBA => { self.block_ind(bus); if self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            0xBB => { self.block_outd(bus); if self.f & ZF == 0 { self.pc = self.pc.wrapping_sub(2); self.cycles += 21; } else { self.cycles += 16; } }
            _ => { self.cycles += 8; }
        }
    }

    fn block_ldi(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        bus.write(self.de(), v);
        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));
        let n = self.a.wrapping_add(v);
        self.f = (self.f & (SF | ZF | CF))
            | (n & XF)
            | if n & 0x02 != 0 { YF } else { 0 }
            | if self.bc() != 0 { PF } else { 0 };
        self.cycles += 16;
    }
    fn block_ldd(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        bus.write(self.de(), v);
        self.set_hl(self.hl().wrapping_sub(1));
        self.set_de(self.de().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));
        let n = self.a.wrapping_add(v);
        self.f = (self.f & (SF | ZF | CF))
            | (n & XF)
            | if n & 0x02 != 0 { YF } else { 0 }
            | if self.bc() != 0 { PF } else { 0 };
        self.cycles += 16;
    }
    fn block_cpi(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        let r = self.a.wrapping_sub(v);
        let hf = (self.a & 0x0F) < (v & 0x0F);
        self.set_hl(self.hl().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));
        let n = r.wrapping_sub(if hf { 1 } else { 0 });
        self.f = (self.f & CF) | NF | (r & SF)
            | if r == 0 { ZF } else { 0 }
            | if hf { HF } else { 0 }
            | (n & XF)
            | if n & 0x02 != 0 { YF } else { 0 }
            | if self.bc() != 0 { PF } else { 0 };
        self.cycles += 16;
    }
    fn block_cpd(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        let r = self.a.wrapping_sub(v);
        let hf = (self.a & 0x0F) < (v & 0x0F);
        self.set_hl(self.hl().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));
        let n = r.wrapping_sub(if hf { 1 } else { 0 });
        self.f = (self.f & CF) | NF | (r & SF)
            | if r == 0 { ZF } else { 0 }
            | if hf { HF } else { 0 }
            | (n & XF)
            | if n & 0x02 != 0 { YF } else { 0 }
            | if self.bc() != 0 { PF } else { 0 };
        self.cycles += 16;
    }
    fn block_ini(&mut self, bus: &mut dyn Bus) {
        let v = bus.input(self.bc());
        bus.write(self.hl(), v);
        self.set_hl(self.hl().wrapping_add(1));
        let b = self.b.wrapping_sub(1);
        self.b = b;
        let k = (v as u16).wrapping_add(b.wrapping_add(1) as u16);
        self.f = NF | if b == 0 { ZF } else { 0 } | ((k as u8) & (SF | XF | YF))
            | if parity((k as u8 & 7) ^ b) { PF } else { 0 }
            | if k > 0xFF { HF | CF } else { 0 };
        self.cycles += 16;
    }
    fn block_ind(&mut self, bus: &mut dyn Bus) {
        let v = bus.input(self.bc());
        bus.write(self.hl(), v);
        self.set_hl(self.hl().wrapping_sub(1));
        let b = self.b.wrapping_sub(1);
        self.b = b;
        let k = (v as u16).wrapping_add(b.wrapping_add(1) as u16);
        self.f = NF | if b == 0 { ZF } else { 0 } | ((k as u8) & (SF | XF | YF))
            | if parity((k as u8 & 7) ^ b) { PF } else { 0 }
            | if k > 0xFF { HF | CF } else { 0 };
        self.cycles += 16;
    }
    fn block_outi(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        let b = self.b.wrapping_sub(1);
        self.b = b;
        bus.output(self.bc(), v);
        self.set_hl(self.hl().wrapping_add(1));
        let k = (v as u16).wrapping_add(b.wrapping_add(1) as u16);
        self.f = NF | if b == 0 { ZF } else { 0 } | ((k as u8) & (SF | XF | YF))
            | if parity((k as u8 & 7) ^ b) { PF } else { 0 }
            | if k > 0xFF { HF | CF } else { 0 };
        self.cycles += 16;
    }
    fn block_outd(&mut self, bus: &mut dyn Bus) {
        let v = bus.read(self.hl());
        let b = self.b.wrapping_sub(1);
        self.b = b;
        bus.output(self.bc(), v);
        self.set_hl(self.hl().wrapping_sub(1));
        let k = (v as u16).wrapping_add(b.wrapping_add(1) as u16);
        self.f = NF | if b == 0 { ZF } else { 0 } | ((k as u8) & (SF | XF | YF))
            | if parity((k as u8 & 7) ^ b) { PF } else { 0 }
            | if k > 0xFF { HF | CF } else { 0 };
        self.cycles += 16;
    }

    fn index_offset(&mut self, bus: &mut dyn Bus) -> u16 {
        let d = self.fetch(bus) as i8;
        let base = if self.index_iy { self.iy } else { self.ix };
        base.wrapping_add(d as u16)
    }

    fn index_prefix(&mut self, bus: &mut dyn Bus, iy: bool) {
        self.index_iy = iy;
        self.cycles += 4;
        let op = self.fetch(bus);
        let idx = if iy { self.iy } else { self.ix };
        match op {
            0x09 => { let r = self.add16(idx, self.bc()); if iy { self.iy = r } else { self.ix = r }; self.cycles += 15; }
            0x19 => { let r = self.add16(idx, self.de()); if iy { self.iy = r } else { self.ix = r }; self.cycles += 15; }
            0x29 => { let r = self.add16(idx, idx); if iy { self.iy = r } else { self.ix = r }; self.cycles += 15; }
            0x39 => { let r = self.add16(idx, self.sp); if iy { self.iy = r } else { self.ix = r }; self.cycles += 15; }
            0x21 => { let v = self.fetch16(bus); if iy { self.iy = v } else { self.ix = v }; self.cycles += 14; }
            0x22 => { let a = self.fetch16(bus); self.write16(bus, a, idx); self.cycles += 20; }
            0x2A => { let a = self.fetch16(bus); let v = self.read16(bus, a); if iy { self.iy = v } else { self.ix = v }; self.cycles += 20; }
            0x23 => { if iy { self.iy = self.iy.wrapping_add(1) } else { self.ix = self.ix.wrapping_add(1) }; self.cycles += 10; }
            0x2B => { if iy { self.iy = self.iy.wrapping_sub(1) } else { self.ix = self.ix.wrapping_sub(1) }; self.cycles += 10; }
            0x34 => { let a = self.index_offset(bus); let v = bus.read(a); let r = self.inc8(v); bus.write(a, r); self.cycles += 23; }
            0x35 => { let a = self.index_offset(bus); let v = bus.read(a); let r = self.dec8(v); bus.write(a, r); self.cycles += 23; }
            0x36 => { let a = self.index_offset(bus); let n = self.fetch(bus); bus.write(a, n); self.cycles += 19; }
            0xE1 => { let v = self.pop(bus); if iy { self.iy = v } else { self.ix = v }; self.cycles += 14; }
            0xE3 => { let t = self.read16(bus, self.sp); self.write16(bus, self.sp, idx); if iy { self.iy = t } else { self.ix = t }; self.cycles += 23; }
            0xE5 => { self.push(bus, idx); self.cycles += 15; }
            0xE9 => { self.pc = idx; self.cycles += 8; }
            0xF9 => { self.sp = idx; self.cycles += 10; }
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => { let a = self.index_offset(bus); let v = bus.read(a); self.set_r((op >> 3) & 7, v); self.cycles += 19; }
            0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => { let a = self.index_offset(bus); bus.write(a, self.get_r(op & 7)); self.cycles += 19; }
            0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => {
                let a = self.index_offset(bus);
                let v = bus.read(a);
                let av = self.a;
                let group = (op >> 3) & 7;
                self.a = match group {
                    0 => self.add8(av, v, 0),
                    1 => self.add8(av, v, if self.f & CF != 0 { 1 } else { 0 }),
                    2 => self.sub8(av, v, 0),
                    3 => self.sub8(av, v, if self.f & CF != 0 { 1 } else { 0 }),
                    4 => self.and8(av, v),
                    5 => self.xor8(av, v),
                    6 => self.or8(av, v),
                    _ => { self.sub8(av, v, 0); av } // CP: flags only, keep A
                };
                self.cycles += 19;
            }
            // ALU A,IXH/IXL/IYH/IYL (undocumented, register operand)
            0x84 | 0x85 | 0x8C | 0x8D | 0x94 | 0x95 | 0x9C | 0x9D
            | 0xA4 | 0xA5 | 0xAC | 0xAD | 0xB4 | 0xB5 | 0xBC | 0xBD => {
                let v = if op & 7 == 4 { (idx >> 8) as u8 } else { idx as u8 };
                let av = self.a;
                let group = (op >> 3) & 7;
                self.a = match group {
                    0 => self.add8(av, v, 0),
                    1 => self.add8(av, v, if self.f & CF != 0 { 1 } else { 0 }),
                    2 => self.sub8(av, v, 0),
                    3 => self.sub8(av, v, if self.f & CF != 0 { 1 } else { 0 }),
                    4 => self.and8(av, v),
                    5 => self.xor8(av, v),
                    6 => self.or8(av, v),
                    _ => { self.sub8(av, v, 0); av } // CP: flags only, keep A
                };
                self.cycles += 8;
            }
            // IXH/IXL/IYH/IYL for LD r,r' group
            0x24 | 0x25 | 0x26 | 0x2C | 0x2D | 0x2E | 0x44..=0x45 | 0x4C..=0x4D | 0x54..=0x55 | 0x5C..=0x5D
            | 0x60..=0x65 | 0x67 | 0x68..=0x6D | 0x6F | 0x7C..=0x7D => {
                self.index_undoc(bus, idx, op);
            }
            0xCB => { self.index_cb(bus, iy); self.cycles += 4; }
            _ => { // prefix ignored, execute as normal opcode (mostly)
                self.execute(bus, op);
            }
        }
        self.index_iy = false;
    }

    fn index_undoc(&mut self, bus: &mut dyn Bus, idx: u16, op: u8) {
        let hi = (idx >> 8) as u8;
        let lo = idx as u8;
        match op {
            0x24 => { let v = self.inc8(hi); self.set_ixh(idx, v); }
            0x25 => { let v = self.dec8(hi); self.set_ixh(idx, v); }
            0x26 => { let n = self.fetch(bus); self.set_ixh(idx, n); }
            0x2C => { let v = self.inc8(lo); self.set_ixl(idx, v); }
            0x2D => { let v = self.dec8(lo); self.set_ixl(idx, v); }
            0x2E => { let n = self.fetch(bus); self.set_ixl(idx, n); }
            _ => {
                let dst = (op >> 3) & 7;
                let src = op & 7;
                let v = match src {
                    4 => hi,
                    5 => lo,
                    0 => self.b,
                    1 => self.c,
                    2 => self.d,
                    3 => self.e,
                    7 => self.a,
                    _ => 0,
                };
                match dst {
                    4 => self.set_ixh(idx, v),
                    5 => self.set_ixl(idx, v),
                    0 => self.b = v,
                    1 => self.c = v,
                    2 => self.d = v,
                    3 => self.e = v,
                    7 => self.a = v,
                    _ => {}
                }
            }
        }
        self.cycles += 8;
    }
    fn set_ixh(&mut self, idx: u16, v: u8) {
        let n = (v as u16) << 8 | (idx & 0x00FF);
        if self.index_iy { self.iy = n } else { self.ix = n }
    }
    fn set_ixl(&mut self, idx: u16, v: u8) {
        let n = (idx & 0xFF00) | v as u16;
        if self.index_iy { self.iy = n } else { self.ix = n }
    }

    fn index_cb(&mut self, bus: &mut dyn Bus, iy: bool) {
        let d = self.fetch(bus) as i8;
        let op = self.fetch(bus);
        let base = if iy { self.iy } else { self.ix };
        let a = base.wrapping_add(d as u16);
        let v = bus.read(a);
        let i = op & 7;
        let group = op >> 6;
        let bitn = (op >> 3) & 7;
        let new = match group {
            0 => self.shift(op >> 3, v),
            1 => { self.bit(op >> 3, v); if i != 6 { self.set_r(i, bus.read(a)); } self.cycles += 20; return; }
            2 => v & !(1 << bitn),
            _ => v | (1 << bitn),
        };
        bus.write(a, new);
        if i != 6 { self.set_r(i, new); }
        self.cycles += 23;
    }
}

impl Default for Z80 {
    fn default() -> Self {
        Z80::new()
    }
}

// Add a couple of fields the core needs but which are not part of the register file.
impl Z80 {
    pub fn init_extra(&mut self) {}
    /// Signal a maskable interrupt. Returns true if it was accepted.
    pub fn interrupt(&mut self, bus: &mut dyn Bus, data: u8) -> bool {
        if !self.iff1 {
            return false;
        }
        self.halted = false;
        self.iff1 = false;
        self.iff2 = false;
        self.r = (self.r & 0x80) | (self.r.wrapping_add(1) & 0x7F);
        match self.im {
            0 | 1 => {
                self.push(bus, self.pc);
                self.pc = 0x0038;
            }
            2 => {
                self.push(bus, self.pc);
                let addr = ((self.i as u16) << 8) | (data as u16 & 0xFE);
                self.pc = self.read16(bus, addr);
            }
            _ => {}
        }
        self.cycles += 13;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBus {
        mem: Vec<u8>,
        io: Vec<u8>,
    }
    impl TestBus {
        fn new() -> Self {
            TestBus { mem: vec![0; 0x10000], io: vec![0; 0x10000] }
        }
    }
    impl Bus for TestBus {
        fn read(&mut self, a: u16) -> u8 { self.mem[a as usize] }
        fn write(&mut self, a: u16, v: u8) { self.mem[a as usize] = v; }
        fn input(&mut self, p: u16) -> u8 { self.io[p as usize] }
        fn output(&mut self, p: u16, v: u8) { self.io[p as usize] = v; }
    }

    /// The self-modifying scanner-copy routine from Elite, in "save" mode.
    /// It must keep the stack balanced and terminate.
    #[test]
    fn scanner_copy_balanced() {
        let mut bus = TestBus::new();
        let code: [u8; 0x39] = [
            0x11, 0x49, 0x50, // LD DE,5049
            0x21, 0x00, 0x5B, // LD HL,5B00
            0xEB,             // EX DE,HL   (save-mode patch)
            0x3E, 0x06,       // LD A,6
            0xE5,             // PUSH HL    (save-mode patch)
            0x06, 0x08,       // LD B,8
            0x0E, 0xFF,       // LD C,0FF
            0xE5,             // PUSH HL    (save-mode patch)
            0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0,
            0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0,
            0xED, 0xA0, 0xED, 0xA0, 0xED, 0xA0, // 13x LDI
            0xE1,             // POP HL     (save-mode patch)
            0x24,             // INC H      (save-mode patch)
            0x10, 0xE1,       // DJNZ 0x8E6A
            0xEB,             // EX DE,HL   (save-mode patch)
            0xD1,             // POP DE
            0xEB,             // EX DE,HL
            0x01, 0x20, 0x00, // LD BC,20
            0x09,             // ADD HL,BC
            0x00,             // NOP        (save-mode patch)
            0x3D,             // DEC A
            0x20, 0xD1,       // JR NZ,0x8E65
            0xC9,             // RET
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
            assert!(cpu.sp <= 0x8000, "stack overflow at step {} pc=0x{:04X}", steps, cpu.pc);
        }
        assert_eq!(cpu.pc, 0x1234, "routine did not return (steps={}, sp=0x{:04X})", steps, cpu.sp);
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
        cpu.a = 0x20; cpu.f = CF;
        let (cpu2, _) = { let mut b = TestBus::new(); b.mem[0x1000] = 0x1F; let mut c = cpu; c.pc = 0x1000; c.step(&mut b); (c, b) };
        assert_eq!(cpu2.a, 0x90, "RRA 0x20 with C=1 -> 0x90");
        assert_eq!(cpu2.f & CF, 0, "RRA carry = old bit0 (0)");

        // RLA: a=(a<<1)|carry, carry=old a&0x80
        let mut b = TestBus::new(); b.mem[0x1000] = 0x17; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x20; c.f = CF; c.step(&mut b);
        assert_eq!(c.a, 0x41, "RLA 0x20 with C=1 -> 0x41");

        // RRCA / RLCA values
        let mut b = TestBus::new(); b.mem[0x1000] = 0x0F; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x81; c.step(&mut b);
        assert_eq!(c.a, 0xC0, "RRCA 0x81 -> 0xC0");
        assert_eq!(c.f & CF, CF, "RRCA carry = old bit0");

        // SRA (CB 2F): 0x82 -> 0xC1
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0x2F; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x82; c.step(&mut b);
        assert_eq!(c.a, 0xC1, "SRA 0x82 -> 0xC1");
        // SRL (CB 3F): 0x81 -> 0x40, C=1
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0x3F; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x81; c.step(&mut b);
        assert_eq!(c.a, 0x40, "SRL 0x81 -> 0x40");
        assert_eq!(c.f & CF, CF, "SRL carry = old bit0");
    }

    #[test]
    fn bit_set_res_ops() {
        // BIT 3,B (CB 58): b=0x08 -> Z=0 ; b=0 -> Z=1
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0x58; let mut c = Z80::new(); c.pc = 0x1000; c.b = 0x08; c.step(&mut b);
        assert_eq!(c.f & ZF, 0, "BIT 3,B with bit set -> Z clear");
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0x58; let mut c = Z80::new(); c.pc = 0x1000; c.b = 0x00; c.step(&mut b);
        assert_eq!(c.f & ZF, ZF, "BIT 3,B with bit clear -> Z set");
        // SET 0,B (CB C0): b=0 -> 1
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0xC0; let mut c = Z80::new(); c.pc = 0x1000; c.b = 0x00; c.step(&mut b);
        assert_eq!(c.b, 0x01, "SET 0,B");
        // RES 5,B (CB A8): b=0x20 -> 0x00
        let mut b = TestBus::new(); b.mem[0x1000] = 0xCB; b.mem[0x1001] = 0xA8; let mut c = Z80::new(); c.pc = 0x1000; c.b = 0x20; c.step(&mut b);
        assert_eq!(c.b, 0x00, "RES 5,B");
    }

    #[test]
    fn sub_and_cp_flags() {
        // CP 0x5B with a=0x20: carry set (borrow), a unchanged
        let mut b = TestBus::new(); b.mem[0x1000] = 0xFE; b.mem[0x1001] = 0x5B; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x20; c.step(&mut b);
        assert_eq!(c.a, 0x20, "CP must not change A");
        assert_eq!(c.f & CF, CF, "CP 0x20-0x5B must set carry");
        // CP 0x80 with a=0xFF: carry clear
        let mut b = TestBus::new(); b.mem[0x1000] = 0xFE; b.mem[0x1001] = 0x80; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0xFF; c.step(&mut b);
        assert_eq!(c.f & CF, 0, "CP 0xFF-0x80 must clear carry");

        // CP must NOT modify A (register operand: B8 = CP B)
        let mut b = TestBus::new(); b.mem[0x1000] = 0xB8; let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x20; c.b = 0x5B; c.step(&mut b);
        assert_eq!(c.a, 0x20, "CP B must keep A=0x20");
        assert_eq!(c.f & CF, CF, "CP B 0x20 vs 0x5B -> C set");

        // CP (IX+d) (DD BE) must NOT modify A
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0xBE; b.mem[0x1002] = 0x00; b.mem[0x6490] = 0x5B;
        let mut c = Z80::new(); c.pc = 0x1000; c.a = 0x20; c.ix = 0x6490; c.step(&mut b);
        assert_eq!(c.a, 0x20, "CP (IX+d) must keep A=0x20");
        assert_eq!(c.f & CF, CF, "CP (IX+d) 0x20 vs 0x5B -> C set");
    }

    #[test]
    fn exx_swaps_all() {
        let mut b = TestBus::new();
        b.mem[0x1000] = 0xD9; // EXX
        let mut c = Z80::new();
        c.pc = 0x1000;
        c.b = 0x11; c.c = 0x22; c.d = 0x33; c.e = 0x44; c.h = 0x55; c.l = 0x66;
        c.b_ = 0xAA; c.c_ = 0xBB; c.d_ = 0xCC; c.e_ = 0xDD; c.h_ = 0xEE; c.l_ = 0xFF;
        c.step(&mut b);
        assert_eq!(c.bc(), 0xAABB, "EXX swaps BC");
        assert_eq!(c.de(), 0xCCDD, "EXX swaps DE");
        assert_eq!(c.hl(), 0xEEFF, "EXX swaps HL");
    }

    #[test]
    fn ixh_ixl_ld_variants() {
        // DD 68 = LD IXL,B
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x68;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x1122; c.b = 0x55; c.step(&mut b);
        assert_eq!(c.ix, 0x1155, "DD 68 = LD IXL,B");
        // DD 69 = LD IXL,C
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x69;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x1122; c.c = 0x77; c.step(&mut b);
        assert_eq!(c.ix, 0x1177, "DD 69 = LD IXL,C");
        // DD 44 = LD B,IXH
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x44;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x1122; c.step(&mut b);
        assert_eq!(c.b, 0x11, "DD 44 = LD B,IXH");
        // DD 7D = LD A,IXL
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x7D;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x1122; c.step(&mut b);
        assert_eq!(c.a, 0x22, "DD 7D = LD A,IXL");
        // FD 6C = LD IYL,IYH
        let mut b = TestBus::new(); b.mem[0x1000] = 0xFD; b.mem[0x1001] = 0x6C;
        let mut c = Z80::new(); c.pc = 0x1000; c.iy = 0xAABB; c.step(&mut b);
        assert_eq!(c.iy, 0xAAAA, "FD 6C = LD IYL,IYH");
    }

    #[test]
    fn alu_a_ixh_ixl() {
        // DD 84 = ADD A,IXH
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x84;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x1100; c.a = 0x05; c.step(&mut b);
        assert_eq!(c.a, 0x16, "DD 84 = ADD A,IXH");
        // DD 85 = ADD A,IXL
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0x85;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x0022; c.a = 0x05; c.step(&mut b);
        assert_eq!(c.a, 0x27, "DD 85 = ADD A,IXL");
        // DD BC = CP IXH, must keep A
        let mut b = TestBus::new(); b.mem[0x1000] = 0xDD; b.mem[0x1001] = 0xBC;
        let mut c = Z80::new(); c.pc = 0x1000; c.ix = 0x5B00; c.a = 0x20; c.step(&mut b);
        assert_eq!(c.a, 0x20, "DD BC = CP IXH keeps A");
        assert_eq!(c.f & CF, CF, "DD BC 0x20 vs 0x5B -> C set");
        // FD AD = XOR IYL
        let mut b = TestBus::new(); b.mem[0x1000] = 0xFD; b.mem[0x1001] = 0xAD;
        let mut c = Z80::new(); c.pc = 0x1000; c.iy = 0x00F0; c.a = 0xFF; c.step(&mut b);
        assert_eq!(c.a, 0x0F, "FD AD = XOR IYL");
    }

    /// Full instruction-level audit against ZEXDOC (CP/M test binary).
    /// Run with: cargo test --release zexdoc -- --ignored --nocapture
    #[test]
    #[ignore]
    fn zexdoc() {
        use std::io::Write;
        let prog = std::fs::read("/tmp/opencode/zexdoc.com").expect("zexdoc.com not found");
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
                            if ch == b'$' { break; }
                            out.push(ch as char);
                            print!("{}", ch as char);
                            addr = addr.wrapping_add(1);
                        }
                        std::io::stdout().flush().ok();
                    }
                    0 => { println!("\n[exit]"); break; }
                    _ => { println!("\n[unknown BDOS c={}]", cpu.c); break; }
                }
                cpu.pc = bus.mem[cpu.sp as usize] as u16
                    | ((bus.mem[cpu.sp as usize + 1] as u16) << 8);
                cpu.sp = cpu.sp.wrapping_add(2);
                continue;
            }
            if cpu.pc == 0x0000 { println!("\n[warm boot]"); break; }
            cpu.step(&mut bus);
            steps += 1;
            if steps > 120_000_000_000 { println!("\n[step cap]"); break; }
        }
        println!("\nsteps={}", steps);
        assert!(!out.contains("ERROR"), "ZEXDOC reported instruction errors");
    }
}
