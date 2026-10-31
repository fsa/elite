//! Основная таблица опкодов Z80 и префиксная диспетчеризация.

use super::*;

impl Z80 {
    pub(super) fn execute(&mut self, bus: &mut dyn Bus, op: u8) {
        match op {
            0x00 => self.cycles += 4, // NOP
            0x08 => {
                std::mem::swap(&mut self.a, &mut self.a_);
                std::mem::swap(&mut self.f, &mut self.f_);
                self.cycles += 4;
            }
            0x10 => {
                // DJNZ d
                let d = self.fetch(bus) as i8;
                self.b = self.b.wrapping_sub(1);
                if self.b != 0 {
                    self.pc = self.pc.wrapping_add(d as u16);
                    self.cycles += 13;
                } else {
                    self.cycles += 8;
                }
            }
            0x18 => {
                let d = self.fetch(bus) as i8;
                self.pc = self.pc.wrapping_add(d as u16);
                self.cycles += 12;
            }
            0x20..=0x38 if op & 0x07 == 0 => {
                // JR cc
                let cc = (op >> 3) & 3;
                let d = self.fetch(bus) as i8;
                if self.cond(cc) {
                    self.pc = self.pc.wrapping_add(d as u16);
                    self.cycles += 12;
                } else {
                    self.cycles += 7;
                }
            }
            0x01 | 0x11 | 0x21 | 0x31 => {
                // LD rr,nn
                let v = self.fetch16(bus);
                self.set_rp((op >> 4) & 3, v);
                self.cycles += 10;
            }
            0x09 | 0x19 | 0x29 | 0x39 => {
                // ADD HL,rr
                let r = self.add16(self.hl(), self.rp((op >> 4) & 3));
                self.set_hl(r);
                self.cycles += 11;
            }
            0x02 => {
                bus.write(self.bc(), self.a);
                self.cycles += 7;
            }
            0x12 => {
                bus.write(self.de(), self.a);
                self.cycles += 7;
            }
            0x22 => {
                let a = self.fetch16(bus);
                self.write16(bus, a, self.hl());
                self.cycles += 16;
            }
            0x32 => {
                let a = self.fetch16(bus);
                bus.write(a, self.a);
                self.cycles += 13;
            }
            0x0A => {
                self.a = bus.read(self.bc());
                self.cycles += 7;
            }
            0x1A => {
                self.a = bus.read(self.de());
                self.cycles += 7;
            }
            0x2A => {
                let a = self.fetch16(bus);
                let v = self.read16(bus, a);
                self.set_hl(v);
                self.cycles += 16;
            }
            0x3A => {
                let a = self.fetch16(bus);
                self.a = bus.read(a);
                self.cycles += 13;
            }
            0x03 | 0x13 | 0x23 | 0x33 => {
                let p = (op >> 4) & 3;
                let v = self.rp(p).wrapping_add(1);
                self.set_rp(p, v);
                self.cycles += 6;
            }
            0x0B | 0x1B | 0x2B | 0x3B => {
                let p = (op >> 4) & 3;
                let v = self.rp(p).wrapping_sub(1);
                self.set_rp(p, v);
                self.cycles += 6;
            }
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                // INC r
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
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                // DEC r
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
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                // LD r,n
                let i = (op >> 3) & 7;
                let n = self.fetch(bus);
                if i == 6 {
                    let a = self.hl();
                    bus.write(a, n);
                    self.cycles += 10;
                } else {
                    self.set_r(i, n);
                    self.cycles += 7;
                }
            }
            0x07 => {
                self.rlca();
                self.cycles += 4;
            }
            0x0F => {
                let c = self.a & 1;
                self.a = (self.a >> 1) | (c << 7);
                self.f =
                    (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 };
                self.cycles += 4;
            }
            0x17 => {
                let c = self.a >> 7;
                self.a = (self.a << 1) | if self.f & CF != 0 { 1 } else { 0 };
                self.f =
                    (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 };
                self.cycles += 4;
            }
            0x1F => {
                self.rra();
                self.cycles += 4;
            }
            0x27 => {
                self.daa();
                self.cycles += 4;
            }
            0x2F => {
                self.a = !self.a;
                self.f = (self.f & (SF | ZF | PF | CF)) | HF | NF | (self.a & (XF | YF));
                self.cycles += 4;
            }
            0x37 => {
                self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | CF;
                self.cycles += 4;
            }
            0x3F => {
                let c = self.f & CF;
                self.f = (self.f & (SF | ZF | PF))
                    | (self.a & (XF | YF))
                    | if c != 0 { HF } else { 0 }
                    | if c != 0 { 0 } else { CF };
                self.cycles += 4;
            }
            0x76 => {
                self.halted = true;
                self.cycles += 4;
            }
            0x40..=0x7F if op != 0x76 => {
                // LD r,r'  (HALT handled)
                let dst = (op >> 3) & 7;
                let src = op & 7;
                if dst == 6 && src == 6 {
                    self.halted = true;
                    self.cycles += 4;
                } else if src == 6 {
                    let a = self.hl();
                    let v = bus.read(a);
                    self.set_r(dst, v);
                    self.cycles += 7;
                } else if dst == 6 {
                    let a = self.hl();
                    bus.write(a, self.get_r(src));
                    self.cycles += 7;
                } else {
                    let v = self.get_r(src);
                    self.set_r(dst, v);
                    self.cycles += 4;
                }
            }
            0x80..=0xBF => {
                // ALU A,r
                let src = op & 7;
                let v = if src == 6 {
                    bus.read(self.hl())
                } else {
                    self.get_r(src)
                };
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
                    _ => {
                        self.sub8(a, v, 0);
                        a
                    } // CP: flags only, keep A
                };
                self.cycles += if src == 6 { 7 } else { 4 };
            }
            0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => {
                // RET cc
                if self.cond((op >> 3) & 7) {
                    self.pc = self.pop(bus);
                    self.cycles += 11;
                } else {
                    self.cycles += 5;
                }
            }
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let v = self.pop(bus);
                self.set_rp2((op >> 4) & 3, v);
                self.cycles += 10;
            }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                self.push(bus, self.rp2((op >> 4) & 3));
                self.cycles += 11;
            }
            0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => {
                // JP cc,nn
                let a = self.fetch16(bus);
                if self.cond((op >> 3) & 7) {
                    self.pc = a;
                }
                self.cycles += 10;
            }
            0xC3 => {
                self.pc = self.fetch16(bus);
                self.cycles += 10;
            }
            0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
                // CALL cc,nn
                let a = self.fetch16(bus);
                if self.cond((op >> 3) & 7) {
                    self.push(bus, self.pc);
                    self.pc = a;
                    self.cycles += 17;
                } else {
                    self.cycles += 10;
                }
            }
            0xCD => {
                let a = self.fetch16(bus);
                self.push(bus, self.pc);
                self.pc = a;
                self.cycles += 17;
            }
            0xC6 => {
                let v = self.fetch(bus);
                let a = self.a;
                self.a = self.add8(a, v, 0);
                self.cycles += 7;
            }
            0xCE => {
                let v = self.fetch(bus);
                let a = self.a;
                let c = if self.f & CF != 0 { 1 } else { 0 };
                self.a = self.add8(a, v, c);
                self.cycles += 7;
            }
            0xD6 => {
                let v = self.fetch(bus);
                let a = self.a;
                self.a = self.sub8(a, v, 0);
                self.cycles += 7;
            }
            0xDE => {
                let v = self.fetch(bus);
                let a = self.a;
                let c = if self.f & CF != 0 { 1 } else { 0 };
                self.a = self.sub8(a, v, c);
                self.cycles += 7;
            }
            0xE6 => {
                let v = self.fetch(bus);
                let a = self.a;
                self.a = self.and8(a, v);
                self.cycles += 7;
            }
            0xEE => {
                let v = self.fetch(bus);
                let a = self.a;
                self.a = self.xor8(a, v);
                self.cycles += 7;
            }
            0xF6 => {
                let v = self.fetch(bus);
                let a = self.a;
                self.a = self.or8(a, v);
                self.cycles += 7;
            }
            0xFE => {
                let v = self.fetch(bus);
                let a = self.a;
                self.sub8(a, v, 0);
                self.cycles += 7;
            }
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                self.push(bus, self.pc);
                self.pc = (op & 0x38) as u16;
                self.cycles += 11;
            }
            0xC9 => {
                self.pc = self.pop(bus);
                self.cycles += 10;
            }
            0xD9 => {
                // EXX
                std::mem::swap(&mut self.b, &mut self.b_);
                std::mem::swap(&mut self.c, &mut self.c_);
                std::mem::swap(&mut self.d, &mut self.d_);
                std::mem::swap(&mut self.e, &mut self.e_);
                std::mem::swap(&mut self.h, &mut self.h_);
                std::mem::swap(&mut self.l, &mut self.l_);
                self.cycles += 4;
            }
            0xE3 => {
                let t = self.read16(bus, self.sp);
                self.write16(bus, self.sp, self.hl());
                self.set_hl(t);
                self.cycles += 19;
            }
            0xE9 => {
                self.pc = self.hl();
                self.cycles += 4;
            }
            0xEB => {
                let t = self.de();
                self.set_de(self.hl());
                self.set_hl(t);
                self.cycles += 4;
            }
            0xF3 => {
                self.iff1 = false;
                self.iff2 = false;
                self.ei_pending = false;
                self.cycles += 4;
            }
            0xFB => {
                self.iff1 = true;
                self.iff2 = true;
                self.ei_pending = true;
                self.cycles += 4;
            }
            0xF9 => {
                self.sp = self.hl();
                self.cycles += 6;
            }
            _ => self.extended(bus, op),
        }
    }

    fn extended(&mut self, bus: &mut dyn Bus, op: u8) {
        match op {
            0xCB => self.cb(bus),
            0xED => self.ed(bus),
            0xDD => self.index_prefix(bus, false),
            0xFD => self.index_prefix(bus, true),
            0xD3 => {
                let p = self.fetch(bus) as u16 | ((self.a as u16) << 8);
                bus.output(p, self.a);
                self.cycles += 11;
            }
            0xDB => {
                let p = self.fetch(bus) as u16 | ((self.a as u16) << 8);
                self.a = bus.input(p);
                self.cycles += 11;
            }
            _ => {
                self.cycles += 4;
            } // undocumented NOPs
        }
    }
}
