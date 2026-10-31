//! Реализация шины Z80 для Spectrum: память, порты, клавиатура, бипер.

use super::*;

impl Bus for Spectrum {
    fn read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        // 48K has no memory paging: RAM everywhere, ROM is read-only-ish
        // (the original ignores writes to ROM).
        if let Some((lo, hi)) = self.wwatch {
            if addr >= lo && addr <= hi {
                *self.wwatch_hist.entry(self.pc_for_write).or_insert(0) += 1;
            }
        }
        if let Some((lo, hi)) = self.wtrace {
            if addr >= lo && addr <= hi {
                eprintln!(
                    "W 0x{addr:04X} <- 0x{val:02X} pc=0x{:04X} sp=0x{:04X}",
                    self.pc_for_write, self.sp_for_write
                );
            }
        }
        if addr >= ROM_SIZE as u16 {
            self.mem[addr as usize] = val;
        }
    }

    fn input(&mut self, port: u16) -> u8 {
        let high = (port >> 8) as u8;
        // Kempston joystick: selected when A5=0 (e.g. port 0x1F / 0xFF1F).
        if port & 0x20 == 0 {
            // Игра читает Kempston как active-high: бит4=огонь, биты0-3 = направления.
            return self.kempston & 0x1F;
        }
        if port & 0x01 == 0 {
            let mut res = 0xFFu8;
            for (r, row) in self.keyboard.iter().enumerate() {
                if high & (1 << r) == 0 {
                    res &= *row;
                }
            }
            res | 0xE0 // bits 5-7 read as 1 (no tape input)
        } else {
            0xFF
        }
    }

    fn output(&mut self, port: u16, val: u8) {
        if port & 0xC000 == 0x4000 && port & 0x0002 == 0 {
            // Порт пейджинга памяти 0x7FFD (и родственный 0x1FFD).
            self.paging_writes = self.paging_writes.wrapping_add(1);
            self.paging_last = val;
        }
        if port & 0x01 == 0 {
            self.border = val & 0x07;
            let beep = val & 0x10 != 0;
            if beep != self.beeper {
                self.beeper = beep;
                self.beeper_edges.push((self.cycle_clock, beep));
            }
        }
    }
}
