//! Sinclair ZX Spectrum 48K machine: 64K address space, ULA (border, beeper,
//! keyboard) and a 256x192 bitmap framebuffer.

use crate::display;
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

mod audio;
mod bus;
mod game_msg;
mod rom;
mod video;

pub use game_msg::{GameMsgLine, GameMsgTap};
pub use rom::install_stub_rom;

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
    /// Гистограмма исполненных PC (`ELITE_PROFILE`): пуста — выключена.
    pub profile: Vec<u32>,
    /// Сколько верхних строк гистограммы печатать.
    pub profile_top: usize,
    pub pc_for_write: u16,
    pub wwatch: Option<(u16, u16)>,
    pub wwatch_hist: std::collections::BTreeMap<u16, u64>,
    pub wtrace: Option<(u16, u16)>,
    pub sp_for_write: u16,
    pub spat: std::collections::BTreeMap<u16, u64>,
    pub dump_at: Vec<u16>,
    /// Диапазон адресов «заставки» (интро) для выбранной версии игры; `None`
    /// — отслеживание выключено. Любой исполненный адрес из диапазона
    /// взводит [`Spectrum::intro_seen`].
    pub intro_range: Option<(u16, u16)>,
    /// Адрес входа в игровой цикл (`GameMain`): его исполнение снимает
    /// [`Spectrum::intro_seen`]. Так флаг «залипает» на заставке, хотя её
    /// цикл вызывает длинные рутины вне своего диапазона.
    pub game_marker: Option<u16>,
    /// Игра сейчас на заставке (интро). Пока игра на заставке, нативные
    /// экраны (карта/закупка) не открываются — как и у оригинальных команд.
    pub intro_seen: bool,
    /// Частота дискретизации синтезируемого звука (Гц).
    pub audio_rate: u32,
    /// Переходы бипера (порт 0xFE, бит 4) в текущем кадре: (такт CPU, уровень).
    beeper_edges: Vec<(u64, bool)>,
    /// Уровень бипера на начало текущего кадра.
    beeper_frame_start: bool,
    /// Такт CPU на начало текущего кадра (или серии кадров при ускорении).
    audio_cycle_base: u64,
    /// Длительность серии кадров, для которой собран звук: `CYCLES_PER_FRAME`
    /// для обычного кадра и кратно больше при ускорении (F12).
    audio_span_cycles: u64,
    /// Такт CPU на начало текущей инструкции (обновляется в `run_frame`).
    cycle_clock: u64,
    /// Нативные замены подпрограмм: `PC` → реализация на Rust.
    pub hooks: std::collections::BTreeMap<u16, Hook>,
    /// Верхняя оценка тактов, которые хук добавит до того, как вернёт
    /// управление (или отдаст его через [`Spectrum::hook_yield`]). Хук **не
    /// запускается**, если эта оценка не укладывается в остаток кадра: иначе
    /// кадровая граница попадёт внутрь рутины и её состояние разойдётся с
    /// прогоном без хука (прерывание сработает в другом месте). В этом случае
    /// рутина исполняется обычным Z80. Нет записи — ограничения нет.
    pub hook_max: std::collections::BTreeMap<u16, u64>,
    /// Хук выставил флаг, отдав управление на середине рутины (см.
    /// [`Hook`]): сам `RET` машина тогда не выполняет, а `PC`/`SP` уже
    /// подготовлены хуком — так длинная рутина, не влезающая в кадр,
    /// продолжается обычным Z80 и прерывается ровно там же, где без хука.
    pub hook_yield: bool,
    /// Такт, до которого идёт текущий кадр (`frame_start + CYCLES_PER_FRAME`).
    /// `u64::MAX` означает «без ограничения» (используется в тестах, где хук
    /// должен отработать рутину целиком, не отдавая управление).
    pub frame_target: u64,
    /// Игра запросила сохранение (перехвачен `MenuSaveLoad`). Обрабатывает
    /// главный цикл: показывает диалог выбора файла и записывает блок командира.
    pub save_requested: bool,
    /// Приложение попросило загрузку ([`native::request_load`]). Перехват
    /// `MenuSaveLoad` отвечает «загружено» (NZ), и родной главный цикл сам
    /// уходит на перезапуск (`jp nz, 0x7394` у Кладова).
    pub load_requested: bool,
    /// HD-слой нативного рендера (см. [`display::Frame`]): поверхность в
    /// разрешении области показа окна. ZX-кадр (256x192) показывается
    /// целым множителем, а нативные хуки рисуют в этот слой поверх него
    /// в полном разрешении. Приложение очищает слой перед каждым кадром и
    /// меняет размер под окно; непрозрачные пиксели слоя (старший байт
    /// != 0) накладываются на масштабированный ZX-кадр при показе.
    pub hd: display::Frame,
    /// Перехват игровых сообщений (заполняет хук печати, читает нативный
    /// вид полёта). См. [`GameMsgTap`].
    pub game_msg: GameMsgTap,
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
            profile: Vec::new(),
            profile_top: 40,
            pc_for_write: 0,
            wwatch: None,
            wwatch_hist: std::collections::BTreeMap::new(),
            wtrace: None,
            sp_for_write: 0,
            spat: std::collections::BTreeMap::new(),
            dump_at: Vec::new(),
            intro_range: None,
            game_marker: None,
            intro_seen: false,
            audio_rate: crate::audio::SAMPLE_RATE,
            beeper_edges: Vec::new(),
            beeper_frame_start: false,
            audio_cycle_base: 0,
            audio_span_cycles: CYCLES_PER_FRAME,
            cycle_clock: 0,
            hooks: std::collections::BTreeMap::new(),
            hook_max: std::collections::BTreeMap::new(),
            hook_yield: false,
            frame_target: u64::MAX,
            save_requested: false,
            load_requested: false,
            hd: display::Frame::new(display::ZX_W, display::ZX_H),
            game_msg: GameMsgTap::default(),
        }
    }

    /// Load a raw block of bytes into memory at `addr`.
    pub fn load(&mut self, addr: u16, data: &[u8]) {
        let mut a = addr as usize;
        for &b in data {
            if a >= MEM_SIZE {
                break;
            }
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

    /// Обновляет флаг заставки по адресу исполняемой инструкции.
    ///
    /// «Залипающе»: вход в игровой цикл (`game_marker`) снимает флаг, адрес из
    /// интро-диапазона — взводит. Так флаг не мерцает между итерациями цикла
    /// заставки, которые вызывают длинные рутины вне её диапазона.
    #[inline]
    pub fn note_pc(&mut self, pc: u16) {
        if self.game_marker == Some(pc) {
            self.intro_seen = false;
        } else if let Some((lo, hi)) = self.intro_range {
            if pc >= lo && pc <= hi {
                self.intro_seen = true;
            }
        }
    }

    /// Run one PAL frame's worth of instructions, raising a maskable
    /// interrupt at the start of the frame (IM1 -> 0x0038).
    pub fn run_frame(&mut self) {
        self.run_frames(1);
    }

    /// Run `count` PAL frames in one go.
    ///
    /// Переходы бипера копятся за все кадры серии, чтобы
    /// [`Spectrum::render_audio`] мог синтезировать весь отрезок в объём
    /// одного кадра показа. Так ускорение эмуляции (F12) не теряет звук и
    /// не копит задержку: звук просто проигрывается вдвое быстрее.
    pub fn run_frames(&mut self, count: u64) {
        if count == 0 {
            return;
        }
        self.beeper_edges.clear();
        self.beeper_frame_start = self.beeper;
        self.audio_cycle_base = self.frame_start;
        self.audio_span_cycles = count * CYCLES_PER_FRAME;
        for _ in 0..count {
            self.run_frame_once();
        }
    }

    /// Execute exactly one PAL frame (without resetting the audio span).
    fn run_frame_once(&mut self) {
        let target = self.frame_start + CYCLES_PER_FRAME;
        let mut cpu = std::mem::take(&mut self.cpu);
        self.frame_target = target;
        self.cycle_clock = cpu.cycles;
        cpu.interrupt(self, 0xFF);
        while cpu.cycles < target {
            let pc = cpu.pc;
            self.note_pc(pc);
            self.pc_for_write = pc;
            if self.wtrace.is_some() {
                self.sp_for_write = cpu.sp;
            }
            if let Some(hook) = self.hooks.get(&pc).copied() {
                // Запускаем хук только если его верхняя оценка тактов
                // укладывается в остаток кадра. Иначе исполняем рутину на Z80:
                // так кадровая граница попадает туда же, где без хука.
                let fits = self
                    .hook_max
                    .get(&pc)
                    .copied()
                    .map_or(true, |max| cpu.cycles.saturating_add(max) <= target);
                if fits {
                    self.hook_yield = false;
                    let cycles = hook(&mut cpu, self);
                    cpu.cycles += cycles;
                    self.cycle_clock = cpu.cycles;
                    if self.hook_yield {
                        // Хук вернул управление на середине рутины: `RET` не
                        // делаем, `PC`/`SP` уже выставлены хуком.
                        continue;
                    }
                    let lo = self.mem[cpu.sp as usize] as u16;
                    let hi = self.mem[cpu.sp.wrapping_add(1) as usize] as u16;
                    cpu.sp = cpu.sp.wrapping_add(2);
                    cpu.pc = lo | (hi << 8);
                    continue;
                }
            }
            if !self.dump_at.is_empty() && self.dump_at.contains(&pc) {
                println!(
                    "AT pc=0x{:04X} hl=0x{:04X} de=0x{:04X} bc=0x{:04X} a=0x{:02X} f=0x{:02X} sp=0x{:04X} ix=0x{:04X} iy=0x{:04X} mem[0xAA0F]=0x{:02X}",
                    pc,
                    cpu.hl(),
                    cpu.de(),
                    cpu.bc(),
                    cpu.a,
                    cpu.f,
                    cpu.sp,
                    cpu.ix,
                    cpu.iy,
                    self.mem[0xAA0F]
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
            if !self.profile.is_empty() {
                self.profile[pc as usize] += 1;
            }
            if let Some((lo, hi)) = self.watch {
                if pc >= lo && pc <= hi {
                    println!(
                        "  cpu pc=0x{:04X} a=0x{:02X} f=0x{:02X} hl=0x{:04X} bc=0x{:04X}",
                        pc,
                        cpu.a,
                        cpu.f,
                        cpu.hl(),
                        cpu.bc()
                    );
                }
            }
            if let Some(n) = self.spat.get_mut(&pc) {
                if *n < 400 {
                    *n += 1;
                    println!(
                        "  spat pc=0x{:04X} sp=0x{:04X} a=0x{:02X} f=0x{:02X} hl=0x{:04X} de=0x{:04X} bc=0x{:04X} ix=0x{:04X} iy=0x{:04X} frame={}",
                        pc,
                        cpu.sp,
                        cpu.a,
                        cpu.f,
                        cpu.hl(),
                        cpu.de(),
                        cpu.bc(),
                        cpu.ix,
                        cpu.iy,
                        self.frames
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

    /// Такт начала инструкции для хуков, эмулирующих запись в порт (`out`).
    /// `run_frame` выставляет `cycle_clock` перед каждым `step`; хук обязан
    /// сделать то же самое, чтобы момент переключения бипера (порт `0xFE`,
    /// бит 4) совпал с прогоном без хука.
    pub fn set_cycle_clock(&mut self, cycle: u64) {
        self.cycle_clock = cycle;
    }

    /// Переходы бипера текущего кадра (`такт CPU`, уровень) — для проверок
    /// эквивалентности нативных хуков.
    #[cfg(test)]
    pub fn beeper_edges(&self) -> &[(u64, bool)] {
        &self.beeper_edges
    }
}

#[cfg(test)]
mod tests;
