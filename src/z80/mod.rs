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

pub(crate) const CF: u8 = 0x01;
pub(crate) const NF: u8 = 0x02;
pub(crate) const PF: u8 = 0x04;
pub(crate) const XF: u8 = 0x08;
pub(crate) const HF: u8 = 0x10;
pub(crate) const YF: u8 = 0x20;
pub(crate) const ZF: u8 = 0x40;
pub(crate) const SF: u8 = 0x80;
#[derive(Clone)]
pub struct Z80 {
    pub a: u8,
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
impl Z80 {
    pub fn new() -> Self {
        Z80 {
            a: 0xFF,
            f: 0xFF,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            a_: 0,
            f_: 0,
            b_: 0,
            c_: 0,
            d_: 0,
            e_: 0,
            h_: 0,
            l_: 0,
            ix: 0xFFFF,
            iy: 0xFFFF,
            sp: 0xFFFF,
            pc: 0,
            i: 0,
            r: 0,
            iff1: false,
            iff2: false,
            im: 0,
            halted: false,
            ei_pending: false,
            index_iy: false,
            cycles: 0,
        }
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }
    #[inline]
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }
    #[inline]
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }
    #[inline]
    fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }
    #[inline]
    fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = v as u8;
    }
    #[inline]
    fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as u8;
        self.e = v as u8;
    }
    #[inline]
    fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = v as u8;
    }
    #[inline]
    fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = v as u8;
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
}

impl Default for Z80 {
    fn default() -> Self {
        Z80::new()
    }
}

// Add a couple of fields the core needs but which are not part of the register file.
impl Z80 {
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

mod alu;
mod cb;
mod ed;
mod execute;
mod index;

use alu::parity;

#[cfg(test)]
mod tests;
