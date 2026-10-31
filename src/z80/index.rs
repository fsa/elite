//! Префиксы DD/FD: регистры IX/IY и недокументированные варианты.

use super::*;

impl Z80 {
    fn index_offset(&mut self, bus: &mut dyn Bus) -> u16 {
        let d = self.fetch(bus) as i8;
        let base = if self.index_iy { self.iy } else { self.ix };
        base.wrapping_add(d as u16)
    }

    pub(super) fn index_prefix(&mut self, bus: &mut dyn Bus, iy: bool) {
        self.index_iy = iy;
        // Здесь не добавляем 4 T за чтение префикса: документированное время
        // каждой инструкции с DD/FD (19 для `ld r,(iy+d)`, 15 для `add ix,rr`
        // и т.д.) уже включает префикс. Раньше префикс начислялся дважды и
        // все инструкции с индексом шли на 4 T дольше настоящего Z80.
        let op = self.fetch(bus);
        let idx = if iy { self.iy } else { self.ix };
        match op {
            0x09 => {
                let r = self.add16(idx, self.bc());
                if iy {
                    self.iy = r
                } else {
                    self.ix = r
                };
                self.cycles += 15;
            }
            0x19 => {
                let r = self.add16(idx, self.de());
                if iy {
                    self.iy = r
                } else {
                    self.ix = r
                };
                self.cycles += 15;
            }
            0x29 => {
                let r = self.add16(idx, idx);
                if iy {
                    self.iy = r
                } else {
                    self.ix = r
                };
                self.cycles += 15;
            }
            0x39 => {
                let r = self.add16(idx, self.sp);
                if iy {
                    self.iy = r
                } else {
                    self.ix = r
                };
                self.cycles += 15;
            }
            0x21 => {
                let v = self.fetch16(bus);
                if iy {
                    self.iy = v
                } else {
                    self.ix = v
                };
                self.cycles += 14;
            }
            0x22 => {
                let a = self.fetch16(bus);
                self.write16(bus, a, idx);
                self.cycles += 20;
            }
            0x2A => {
                let a = self.fetch16(bus);
                let v = self.read16(bus, a);
                if iy {
                    self.iy = v
                } else {
                    self.ix = v
                };
                self.cycles += 20;
            }
            0x23 => {
                if iy {
                    self.iy = self.iy.wrapping_add(1)
                } else {
                    self.ix = self.ix.wrapping_add(1)
                };
                self.cycles += 10;
            }
            0x2B => {
                if iy {
                    self.iy = self.iy.wrapping_sub(1)
                } else {
                    self.ix = self.ix.wrapping_sub(1)
                };
                self.cycles += 10;
            }
            0x34 => {
                let a = self.index_offset(bus);
                let v = bus.read(a);
                let r = self.inc8(v);
                bus.write(a, r);
                self.cycles += 23;
            }
            0x35 => {
                let a = self.index_offset(bus);
                let v = bus.read(a);
                let r = self.dec8(v);
                bus.write(a, r);
                self.cycles += 23;
            }
            0x36 => {
                let a = self.index_offset(bus);
                let n = self.fetch(bus);
                bus.write(a, n);
                self.cycles += 19;
            }
            0xE1 => {
                let v = self.pop(bus);
                if iy {
                    self.iy = v
                } else {
                    self.ix = v
                };
                self.cycles += 14;
            }
            0xE3 => {
                let t = self.read16(bus, self.sp);
                self.write16(bus, self.sp, idx);
                if iy {
                    self.iy = t
                } else {
                    self.ix = t
                };
                self.cycles += 23;
            }
            0xE5 => {
                self.push(bus, idx);
                self.cycles += 15;
            }
            0xE9 => {
                self.pc = idx;
                self.cycles += 8;
            }
            0xF9 => {
                self.sp = idx;
                self.cycles += 10;
            }
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                let a = self.index_offset(bus);
                let v = bus.read(a);
                self.set_r((op >> 3) & 7, v);
                self.cycles += 19;
            }
            0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => {
                let a = self.index_offset(bus);
                bus.write(a, self.get_r(op & 7));
                self.cycles += 19;
            }
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
                    _ => {
                        self.sub8(av, v, 0);
                        av
                    } // CP: flags only, keep A
                };
                self.cycles += 19;
            }
            // ALU A,IXH/IXL/IYH/IYL (undocumented, register operand)
            0x84 | 0x85 | 0x8C | 0x8D | 0x94 | 0x95 | 0x9C | 0x9D | 0xA4 | 0xA5 | 0xAC | 0xAD
            | 0xB4 | 0xB5 | 0xBC | 0xBD => {
                let v = if op & 7 == 4 {
                    (idx >> 8) as u8
                } else {
                    idx as u8
                };
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
                    _ => {
                        self.sub8(av, v, 0);
                        av
                    } // CP: flags only, keep A
                };
                self.cycles += 8;
            }
            // IXH/IXL/IYH/IYL for LD r,r' group
            0x24
            | 0x25
            | 0x26
            | 0x2C
            | 0x2D
            | 0x2E
            | 0x44..=0x45
            | 0x4C..=0x4D
            | 0x54..=0x55
            | 0x5C..=0x5D
            | 0x60..=0x65
            | 0x67
            | 0x68..=0x6D
            | 0x6F
            | 0x7C..=0x7D => {
                self.index_undoc(bus, idx, op);
            }
            // index_cb начисляет документированное время DD/FD CB-операций
            // (20/23 T) уже вместе с обоими префиксами.
            0xCB => self.index_cb(bus, iy),
            _ => {
                // prefix ignored, execute as normal opcode (mostly)
                self.execute(bus, op);
                self.cycles += 4; // прочитанный, но не применённый префикс
            }
        }
        self.index_iy = false;
    }

    fn index_undoc(&mut self, bus: &mut dyn Bus, idx: u16, op: u8) {
        let hi = (idx >> 8) as u8;
        let lo = idx as u8;
        match op {
            0x24 => {
                let v = self.inc8(hi);
                self.set_ixh(idx, v);
            }
            0x25 => {
                let v = self.dec8(hi);
                self.set_ixh(idx, v);
            }
            0x26 => {
                let n = self.fetch(bus);
                self.set_ixh(idx, n);
                self.cycles += 3;
            } // LD IXH,n = 11 T (8 + imm)
            0x2C => {
                let v = self.inc8(lo);
                self.set_ixl(idx, v);
            }
            0x2D => {
                let v = self.dec8(lo);
                self.set_ixl(idx, v);
            }
            0x2E => {
                let n = self.fetch(bus);
                self.set_ixl(idx, n);
                self.cycles += 3;
            } // LD IXL,n = 11 T
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
        if self.index_iy {
            self.iy = n
        } else {
            self.ix = n
        }
    }
    fn set_ixl(&mut self, idx: u16, v: u8) {
        let n = (idx & 0xFF00) | v as u16;
        if self.index_iy {
            self.iy = n
        } else {
            self.ix = n
        }
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
            1 => {
                self.bit(op >> 3, v);
                if i != 6 {
                    self.set_r(i, bus.read(a));
                }
                self.cycles += 20;
                return;
            }
            2 => v & !(1 << bitn),
            _ => v | (1 << bitn),
        };
        bus.write(a, new);
        if i != 6 {
            self.set_r(i, new);
        }
        self.cycles += 23;
    }
}
