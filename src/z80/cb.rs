//! Префикс CB: сдвиги, битовые операции и RES/SET.

use super::*;

impl Z80 {
    pub(super) fn cb(&mut self, bus: &mut dyn Bus) {
        let op = self.fetch(bus);
        let i = op & 7;
        let group = op >> 6;
        let bitn = (op >> 3) & 7;
        if i == 6 {
            let a = self.hl();
            let v = bus.read(a);
            let new = match group {
                0 => self.shift(op >> 3, v),
                1 => {
                    self.bit(op >> 3, v);
                    self.cycles += 12;
                    return;
                }
                2 => v & !(1 << bitn),
                _ => v | (1 << bitn),
            };
            bus.write(a, new);
            self.cycles += 15;
        } else {
            let v = self.get_r(i);
            let new = match group {
                0 => self.shift(op >> 3, v),
                1 => {
                    self.bit(op >> 3, v);
                    self.cycles += 8;
                    return;
                }
                2 => v & !(1 << bitn),
                _ => v | (1 << bitn),
            };
            self.set_r(i, new);
            self.cycles += 8;
        }
    }
    pub(super) fn shift(&mut self, kind: u8, v: u8) -> u8 {
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
}
