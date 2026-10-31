//! Префикс ED: расширенные инструкции и блочные операции.

use super::*;

impl Z80 {
    pub(super) fn ed(&mut self, bus: &mut dyn Bus) {
        let op = self.fetch(bus);
        match op {
            0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => {
                // IN r,(C)
                let p = self.bc();
                let v = bus.input(p);
                self.f = (self.f & CF)
                    | (v & (SF | XF | YF))
                    | if v == 0 { ZF } else { 0 }
                    | if parity(v) { PF } else { 0 };
                let i = (op >> 3) & 7;
                if i != 6 {
                    self.set_r(i, v);
                }
                self.cycles += 12;
            }
            0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x71 | 0x79 => {
                // OUT (C),r
                let p = self.bc();
                let i = (op >> 3) & 7;
                let v = if i == 6 { 0 } else { self.get_r(i) };
                bus.output(p, v);
                self.cycles += 12;
            }
            0x42 | 0x52 | 0x62 | 0x72 => {
                let v = self.sbc16(self.hl(), self.rp((op >> 4) & 3));
                self.set_hl(v);
                self.cycles += 15;
            }
            0x4A | 0x5A | 0x6A | 0x7A => {
                let v = self.adc16(self.hl(), self.rp((op >> 4) & 3));
                self.set_hl(v);
                self.cycles += 15;
            }
            0x43 | 0x53 | 0x63 | 0x73 => {
                let a = self.fetch16(bus);
                self.write16(bus, a, self.rp((op >> 4) & 3));
                self.cycles += 20;
            }
            0x4B | 0x5B | 0x6B | 0x7B => {
                let a = self.fetch16(bus);
                let v = self.read16(bus, a);
                self.set_rp((op >> 4) & 3, v);
                self.cycles += 20;
            }
            0x44 | 0x4C | 0x54 | 0x5C | 0x64 | 0x6C | 0x74 | 0x7C => {
                // NEG
                let a = self.a;
                self.a = self.sub8(0, a, 0);
                self.cycles += 8;
            }
            0x45 | 0x4D | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D => {
                self.pc = self.pop(bus);
                self.iff1 = self.iff2;
                self.cycles += 14;
            } // RETN
            0x46 | 0x4E | 0x66 | 0x6E => {
                self.im = 0;
                self.cycles += 8;
            }
            0x56 | 0x76 => {
                self.im = 1;
                self.cycles += 8;
            }
            0x5E | 0x7E => {
                self.im = 2;
                self.cycles += 8;
            }
            0x47 => {
                self.i = self.a;
                self.cycles += 9;
            }
            0x4F => {
                self.r = self.a;
                self.cycles += 9;
            }
            0x57 => {
                self.a = self.i;
                self.f = (self.f & CF)
                    | (self.a & (SF | XF | YF))
                    | if self.a == 0 { ZF } else { 0 }
                    | if self.iff2 { PF } else { 0 };
                self.cycles += 9;
            }
            0x5F => {
                self.a = self.r;
                self.f = (self.f & CF)
                    | (self.a & (SF | XF | YF))
                    | if self.a == 0 { ZF } else { 0 }
                    | if self.iff2 { PF } else { 0 };
                self.cycles += 9;
            }
            0x67 => {
                // RRD
                let a = self.hl();
                let v = bus.read(a);
                let new = (v >> 4) | (self.a << 4);
                bus.write(a, new);
                self.a = (self.a & 0xF0) | (v & 0x0F);
                self.f = (self.f & CF)
                    | (self.a & (SF | XF | YF))
                    | if self.a == 0 { ZF } else { 0 }
                    | if parity(self.a) { PF } else { 0 };
                self.cycles += 18;
            }
            0x6F => {
                // RLD
                let a = self.hl();
                let v = bus.read(a);
                let new = (v << 4) | (self.a & 0x0F);
                bus.write(a, new);
                self.a = (self.a & 0xF0) | (v >> 4);
                self.f = (self.f & CF)
                    | (self.a & (SF | XF | YF))
                    | if self.a == 0 { ZF } else { 0 }
                    | if parity(self.a) { PF } else { 0 };
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
            0xB0 => {
                self.block_ldi(bus);
                if self.f & PF != 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xB1 => {
                self.block_cpi(bus);
                if self.f & PF != 0 && self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xB2 => {
                self.block_ini(bus);
                if self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xB3 => {
                self.block_outi(bus);
                if self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xB8 => {
                self.block_ldd(bus);
                if self.f & PF != 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xB9 => {
                self.block_cpd(bus);
                if self.f & PF != 0 && self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xBA => {
                self.block_ind(bus);
                if self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            0xBB => {
                self.block_outd(bus);
                if self.f & ZF == 0 {
                    self.pc = self.pc.wrapping_sub(2);
                    self.cycles += 21;
                } else {
                    self.cycles += 16;
                }
            }
            _ => {
                self.cycles += 8;
            }
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
        self.f = (self.f & CF)
            | NF
            | (r & SF)
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
        self.f = (self.f & CF)
            | NF
            | (r & SF)
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
        self.f = NF
            | if b == 0 { ZF } else { 0 }
            | ((k as u8) & (SF | XF | YF))
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
        self.f = NF
            | if b == 0 { ZF } else { 0 }
            | ((k as u8) & (SF | XF | YF))
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
        self.f = NF
            | if b == 0 { ZF } else { 0 }
            | ((k as u8) & (SF | XF | YF))
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
        self.f = NF
            | if b == 0 { ZF } else { 0 }
            | ((k as u8) & (SF | XF | YF))
            | if parity((k as u8 & 7) ^ b) { PF } else { 0 }
            | if k > 0xFF { HF | CF } else { 0 };
        self.cycles += 16;
    }
}
