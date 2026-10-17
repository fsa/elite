//! Sinclair ZX Spectrum 48K machine: 64K address space, ULA (border, beeper,
//! keyboard) and a 256x192 bitmap framebuffer.

use crate::z80::{Bus, Z80};

pub const ROM_SIZE: usize = 0x4000;
pub const MEM_SIZE: usize = 0x10000;
pub const CYCLES_PER_FRAME: u64 = 69888;

/// Нативная замена подпрограммы игры. При попадании `PC` на адрес-ключ
/// вызывается эта функция (вместо выполнения кода на Z80). Она должна
/// воспроизвести эффект рутины (регистры/память) и вернуть число тактов,
/// которое эта рутина заняла бы на Z80, включая завершающий `RET`. Сам `RET`
/// (возврат по адресу из стека) машина выполняет сама.
pub type Hook = fn(&mut Z80, &mut Spectrum) -> u64;


pub struct Spectrum {
    pub mem: Vec<u8>,
    pub cpu: Z80,
    pub keyboard: [u8; 8],
    /// Kempston joystick state, active high: bit0=right, bit1=left, bit2=down,
    /// bit3=up, bit4=fire. Read at port `0x1F` (A5=0).
    pub kempston: u8,
    pub border: u8,
    pub beeper: bool,
    /// Число записей в порт пейджинга 0x7FFD (для проверки использования 128K).
    pub paging_writes: u32,
    pub paging_last: u8,
    pub frame_start: u64,
    pub frames: u64,
    pub watch: Option<(u16, u16)>,
    pub counts: Vec<(u16, u64)>,
    pub pc_for_write: u16,
    pub wwatch: Option<(u16, u16)>,
    pub wwatch_hist: std::collections::BTreeMap<u16, u64>,
    pub spat: std::collections::BTreeMap<u16, u64>,
    pub dump_at: Vec<u16>,
    /// Частота дискретизации синтезируемого звука (Гц).
    pub audio_rate: u32,
    /// Переходы бипера (порт 0xFE, бит 4) в текущем кадре: (такт CPU, уровень).
    beeper_edges: Vec<(u64, bool)>,
    /// Уровень бипера на начало текущего кадра.
    beeper_frame_start: bool,
    /// Такт CPU на начало текущего кадра.
    audio_cycle_base: u64,
    /// Такт CPU на начало текущей инструкции (обновляется в `run_frame`).
    cycle_clock: u64,
    /// Нативные замены подпрограмм: `PC` → реализация на Rust.
    pub hooks: std::collections::BTreeMap<u16, Hook>,
}

impl Spectrum {
    pub fn new(rom: Option<&[u8]>) -> Self {
        let mut mem = vec![0u8; MEM_SIZE];
        if let Some(rom) = rom {
            let n = rom.len().min(ROM_SIZE);
            mem[..n].copy_from_slice(&rom[..n]);
        } else {
            install_stub_rom(&mut mem[..ROM_SIZE]);
        }
        Spectrum {
            mem,
            cpu: Z80::new(),
            keyboard: [0xFF; 8],
            kempston: 0,
            border: 0,
            beeper: false,
            paging_writes: 0,
            paging_last: 0,
            frame_start: 0,
            frames: 0,
            watch: None,
            counts: Vec::new(),
            pc_for_write: 0,
            wwatch: None,
            wwatch_hist: std::collections::BTreeMap::new(),
            spat: std::collections::BTreeMap::new(),
            dump_at: Vec::new(),
            audio_rate: crate::audio::SAMPLE_RATE,
            beeper_edges: Vec::new(),
            beeper_frame_start: false,
            audio_cycle_base: 0,
            cycle_clock: 0,
            hooks: std::collections::BTreeMap::new(),
        }
    }

    /// Load a raw block of bytes into memory at `addr`.
    pub fn load(&mut self, addr: u16, data: &[u8]) {
        let mut a = addr as usize;
        for &b in data {
            if a >= MEM_SIZE { break; }
            self.mem[a] = b;
            a += 1;
        }
    }

    pub fn set_key(&mut self, row: usize, bit: u8, pressed: bool) {
        if pressed {
            self.keyboard[row] &= !(1 << bit);
        } else {
            self.keyboard[row] |= 1 << bit;
        }
    }

    pub fn release_all_keys(&mut self) {
        self.keyboard = [0xFF; 8];
    }

    /// Set one Kempston bit (0=right, 1=left, 2=down, 3=up, 4=fire).
    pub fn set_kempston(&mut self, bit: u8, pressed: bool) {
        if pressed {
            self.kempston |= 1 << bit;
        } else {
            self.kempston &= !(1 << bit);
        }
    }

    pub fn release_joystick(&mut self) {
        self.kempston = 0;
    }

    /// Run one PAL frame's worth of instructions, raising a maskable
    /// interrupt at the start of the frame (IM1 -> 0x0038).
    pub fn run_frame(&mut self) {
        let target = self.frame_start + CYCLES_PER_FRAME;
        let mut cpu = std::mem::take(&mut self.cpu);
        self.beeper_edges.clear();
        self.beeper_frame_start = self.beeper;
        self.audio_cycle_base = self.frame_start;
        self.cycle_clock = cpu.cycles;
        cpu.interrupt(self, 0xFF);
        while cpu.cycles < target {
            let pc = cpu.pc;
            self.pc_for_write = pc;
            if let Some(hook) = self.hooks.get(&pc).copied() {
                let cycles = hook(&mut cpu, self);
                let lo = self.mem[cpu.sp as usize] as u16;
                let hi = self.mem[cpu.sp.wrapping_add(1) as usize] as u16;
                cpu.sp = cpu.sp.wrapping_add(2);
                cpu.pc = lo | (hi << 8);
                cpu.cycles += cycles;
                self.cycle_clock = cpu.cycles;
                continue;
            }
            if !self.dump_at.is_empty() && self.dump_at.contains(&pc) {
                println!(
                    "AT pc=0x{:04X} hl=0x{:04X} de=0x{:04X} bc=0x{:04X} a=0x{:02X} f=0x{:02X} sp=0x{:04X} ix=0x{:04X} iy=0x{:04X}",
                    pc, cpu.hl(), cpu.de(), cpu.bc(), cpu.a, cpu.f, cpu.sp, cpu.ix, cpu.iy
                );
                let _ = std::fs::write(
                    format!("/tmp/opencode/at_{:04X}.bin", pc),
                    &self.mem[0x4000..0x5B00],
                );
                if pc == 0xE008 {
                    println!(
                        "  WhatOnScreen=0x{:02X} dist=0x{:04X}",
                        self.mem[0x7041],
                        (self.mem[0xE2E1] as u16) << 8 | self.mem[0xE2E0] as u16
                    );
                }
                let _ = std::fs::write(
                    format!("/tmp/opencode/atbuf_{:04X}.bin", pc),
                    &self.mem[0xC000..0xD000],
                );
                let _ = std::fs::write(
                    format!("/tmp/opencode/atfull_{:04X}.bin", pc),
                    &self.mem[0..0x10000],
                );
            }
            if !self.counts.is_empty() {
                for c in self.counts.iter_mut() {
                    if c.0 == pc {
                        c.1 += 1;
                    }
                }
            }
            if let Some((lo, hi)) = self.watch {
                if pc >= lo && pc <= hi {
                    println!(
                        "  cpu pc=0x{:04X} a=0x{:02X} f=0x{:02X} hl=0x{:04X} bc=0x{:04X}",
                        pc, cpu.a, cpu.f, cpu.hl(), cpu.bc()
                    );
                }
            }
            if let Some(n) = self.spat.get_mut(&pc) {
                if *n < 400 {
                    *n += 1;
                    println!(
                        "  spat pc=0x{:04X} sp=0x{:04X} a=0x{:02X} f=0x{:02X} hl=0x{:04X} de=0x{:04X} bc=0x{:04X} ix=0x{:04X} iy=0x{:04X} frame={}",
                        pc, cpu.sp, cpu.a, cpu.f, cpu.hl(), cpu.de(), cpu.bc(), cpu.ix, cpu.iy, self.frames
                    );
                }
            }
            self.cycle_clock = cpu.cycles;
            cpu.step(self);
        }
        self.frame_start = target;
        self.frames += 1;
        self.cpu = cpu;
    }

    /// Decode the bitmap and attributes into a 256x192 RGBA pixel buffer.
    pub fn render_rgba(&self, out: &mut [u32]) {
        for y in 0..192u16 {
            for x in 0..256u16 {
                let (addr, bit) = pixel_addr(x, y);
                let on = self.mem[addr as usize] & bit != 0;
                let attr = self.mem[0x5800 + (y as usize / 8) * 32 + x as usize / 8];
                let ink = attr & 7;
                let paper = (attr >> 3) & 7;
                let bright = if attr & 0x40 != 0 { 8 } else { 0 };
                let palette = if on { ink } else { paper };
                out[y as usize * 256 + x as usize] = ZX_PALETTE[(palette | bright) as usize];
            }
        }
    }

    /// Synthesize one frame of 1-bit beeper audio (mono, [`AUDIO_RATE`]) into
    /// `out`. Call after [`Spectrum::run_frame`].
    pub fn render_audio(&mut self, out: &mut Vec<i16>) {
        const AMP: i16 = 4000;
        let samples = (self.audio_rate as usize) / 50;
        out.clear();
        out.reserve(samples);
        let base = self.audio_cycle_base;
        let mut idx = 0usize;
        let mut level = self.beeper_frame_start;
        for i in 0..samples {
            let cycle = base + (i as u64 * CYCLES_PER_FRAME) / samples as u64;
            while idx < self.beeper_edges.len() && self.beeper_edges[idx].0 <= cycle {
                level = self.beeper_edges[idx].1;
                idx += 1;
            }
            out.push(if level { AMP } else { -AMP });
        }
    }
}

/// Compute the bitmap byte address and bit mask for a screen pixel.
/// (0,0) is the top-left corner.
#[inline]
pub fn pixel_addr(x: u16, y: u16) -> (u16, u8) {
    let addr = 0x4000
        + ((y & 0xC0) << 5)
        + ((y & 0x07) << 8)
        + ((y & 0x38) << 2)
        + (x >> 3);
    (addr, 0x80 >> (x & 7))
}

pub const ZX_PALETTE: [u32; 16] = [
    0x0000_0000, // black
    0x0000_00D7, // blue
    0x00D7_0000, // red
    0x00D7_00D7, // magenta
    0x0000_D700, // green
    0x0000_D7D7, // cyan
    0x00D7_D700, // yellow
    0x00D7_D7D7, // white
    0x0000_0000, // bright black
    0x0000_00FF, // bright blue
    0x00FF_0000, // bright red
    0x00FF_00FF, // bright magenta
    0x0000_FF00, // bright green
    0x0000_FFFF, // bright cyan
    0x00FF_FF00, // bright yellow
    0x00FF_FFFF, // bright white
];

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

/// Minimal substitute ROM implementing only the entry points Elite needs.
/// Used when a real ROM image is not supplied. Addresses match the standard
/// ZX Spectrum 48K ROM layout.
pub fn install_stub_rom(rom: &mut [u8]) {
    // Reset vector: DI / JR to itself is not desired; start of ROM just spins.
    rom[0x0000] = 0xF3; // DI
    rom[0x0001] = 0x76; // HALT
    rom[0x0002] = 0x18; // JR
    rom[0x0003] = 0xFD; // -3

    // Interrupt handler at 0x0038: EI / RET
    rom[0x0038] = 0xFB; // EI
    rom[0x0039] = 0xC9; // RET

    // BEEPER at 0x03B5: RET (silent)
    rom[0x03B5] = 0xC9;

    // PIXEL_ADD exact bytes at 0x22AA (contract: B=y, C=x -> HL=addr, A=x&7).
    const PIXEL_ADD: [u8; 33] = [
        0x3E, 0xAF, 0x90, 0xDA, 0xF9, 0x24, 0x47, 0xA7, 0x1F, 0x37, 0x1F, 0xA7, 0x1F, 0xA8, 0xE6,
        0xF8, 0xA8, 0x67, 0x79, 0x07, 0x07, 0x07, 0xA8, 0xE6, 0xC7, 0xA8, 0x07, 0x07, 0x6F, 0x79,
        0xE6, 0x07, 0xC9,
    ];
    rom[0x22AA..0x22AA + PIXEL_ADD.len()].copy_from_slice(&PIXEL_ADD);

    // Target of the "out of range" branch: just RET.
    rom[0x24F9] = 0xC9;

    // Tape load/save stubs (LD-BYTES 0x0562, SA-BYTES 0x04C6): RET.
    rom[0x0562] = 0xC9;
    rom[0x04C6] = 0xC9;
}

#[cfg(test)]
mod audio_tests {
    use super::*;

    #[test]
    fn beeper_edges_become_audio() {
        let mut m = Spectrum::new(None);
        // DI ; loop: ld a,0x10 ; out (0xfe),a ; ld a,0 ; out (0xfe),a ; jr loop
        let prog = [
            0xF3, 0x3E, 0x10, 0xD3, 0xFE, 0x3E, 0x00, 0xD3, 0xFE, 0x18, 0xF6,
        ];
        m.load(0x8000, &prog);
        m.cpu.pc = 0x8000;
        m.run_frame();
        let mut out = Vec::new();
        m.render_audio(&mut out);
        assert_eq!(out.len(), m.audio_rate as usize / 50);
        assert!(out.iter().any(|&s| s > 0), "no beeper-high samples");
        assert!(out.iter().any(|&s| s < 0), "no beeper-low samples");
    }
}
