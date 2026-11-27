//! Sinclair ZX Spectrum machine: 64K address space with 128K RAM paging, ULA
//! (border, beeper, keyboard) and a 256x192 bitmap framebuffer.
//!
//! `mem` — это текущее окно адресного пространства: `0x4000..0x8000` всегда
//! банк 5, `0x8000..0xC000` — банк 2, а `0xC000..0x10000` — банк из младших
//! битов порта `0x7FFD`. При переключении банка текущая страница сохраняется
//! в `ram_banks`, поэтому прямой доступ к `mem` (хуки, рендер, тесты) видит
//! именно ту память, которая сейчас отображена CPU.

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

pub use game_msg::{BOTTOM_MESSAGE_ROW, GameMsgTap, MessageTag};
#[allow(unused_imports)]
pub use game_msg::TextEvent;
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
    /// Число записей в порт пейджинга 0x7FFD.
    pub paging_writes: u32,
    pub paging_last: u8,
    /// Восемь физических банков ОЗУ по 16K. Пусто, пока не загружен 128K-образ.
    ram_banks: Vec<Vec<u8>>,
    /// Включён ли пейджинг (после загрузки 128K-образа).
    paging_enabled: bool,
    /// Текущее значение порта `0x7FFD`.
    pub paging_reg: u8,
    /// Порт `0x7FFD` заблокирован битом 5 последней записи.
    paging_locked: bool,
    /// Банк, который сейчас отображён по `0xC000`.
    pub paged_bank: usize,
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
    /// Адрес точки в конце прохода игры (`PrintCompass`), где всё уже
    /// обновлено и нарисовано. Снимок берётся здесь, а не на входе в
    /// `GameMain`: иначе нативный слой отставал бы от смены объектов на целый
    /// проход — корабль у станции сменялся бы планетой с задержкой.
    pub snapshot_marker: Option<u16>,
    /// Адрес начала ролика перехода (`sub_9296`), где игра уже создала новую
    /// систему, но до конца ролика больше не возвращается в `PrintCompass`.
    /// Без этого снимка нативный тоннель показывал бы прежнюю систему.
    pub transition_snapshot_marker: Option<u16>,
    /// Снимок памяти на [`Spectrum::snapshot_marker`]. Граница кадра эмулятора
    /// может пройти в середине прохода игры: тогда часть объектов уже
    /// обновлена, а часть ещё нет. Нативный слой читает этот снимок, чтобы
    /// видеть всегда законченный проход, а не смесь двух проходов.
    pub game_snapshot: Vec<u8>,
    /// Был ли уже снимок в одной из точек [`Spectrum::snapshot_marker`] или
    /// [`Spectrum::transition_snapshot_marker`].
    pub game_snapshot_valid: bool,
    /// Игра сейчас на заставке (интро). Пока игра на заставке, нативные
    /// экраны (карта/закупка) не открываются — как и у оригинальных команд.
    pub intro_seen: bool,
    /// Частота дискретизации синтезируемого звука (Гц).
    pub audio_rate: u32,
    /// General Sound: значение защёлки порта `0xB3` (выбранная страница/регистр).
    pub gs_latch: u8,
    /// General Sound: последняя команда, записанная в порт `0xBB`.
    pub gs_command: u8,
    /// General Sound: бит 0 статуса порта `0xBB` (1 — команда ещё обрабатывается).
    pub gs_busy: bool,
    /// Команды воспроизведения GS (`0x39` + значение защёлки `0xB3`), которые
    /// игра выдала за кадр. Эмулятор не синтезирует GS-сэмплы: очередь отдаётся
    /// нативному звуковому модулю через [`crate::sfx`].
    pub gs_sound_requests: Vec<u8>,
    /// Переходы бипера (порт 0xFE, бит 4) в текущем кадре: (такт CPU, уровень).
    beeper_edges: Vec<(u64, bool)>,
    /// Уровень бипера на начало текущего кадра.
    beeper_frame_start: bool,
    /// Сглаженный уровень бипера между вызовами [`Spectrum::render_audio`].
    /// Резкий переход 1/0 даёт щелчок в динамике, поэтому Front всё же
    /// доходит до нужного уровня, но за несколько сэмплов.
    beeper_filter: f64,
    /// Такт CPU на начало текущего кадра (или серии кадров при ускорении).
    audio_cycle_base: u64,
    /// Длительность серии кадров, для которой собран звук: `CYCLES_PER_FRAME`
    /// для обычного кадра и кратно больше при ускорении (F12).
    audio_span_cycles: u64,
    /// Такт CPU на начало текущей инструкции (обновляется в `run_frame`).
    cycle_clock: u64,
    /// Нативные замены подпрограмм: `PC` → реализация на Rust.
    pub hooks: std::collections::BTreeMap<u16, Hook>,
    /// Для адресов из `0xC000..0x10000` — в каком банке допустим хук. Без этого
    /// один и тот же `PC` в другом банке вызвал бы чужую нативную рутину.
    hook_banks: std::collections::BTreeMap<u16, u8>,
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
    /// Приложение попросило загрузку ([`native::request_load`]). Хук
    /// [`native::request_load`] ждёт ближайшего `GameMain`/цикла заставки и
    /// уводит игру на станцию с загруженным командиром.
    pub load_requested: bool,
    /// Хук загрузки уже сработал, но original ещё не дошёл до `GameMain`.
    /// Нативный слой держит заглушку `LOADING` до этого момента: при загрузке
    /// из полёта подготовка станции (`ClsAndPrintRamk`) продолжается несколько
    /// кадров, и ранняя заморозка Z80 ломает переход на страницу капитана.
    pub load_wait_game: bool,
    /// Символ, который нативный диалог миссии отдаёт original вместо чтения
    /// `PressKeyPrnSymbol` (`0xFF` — ответа ещё нет).
    pub mission_answer: u8,
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
            ram_banks: Vec::new(),
            paging_enabled: false,
            paging_reg: 0x10,
            paging_locked: false,
            paged_bank: 0,
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
            snapshot_marker: None,
            transition_snapshot_marker: None,
            game_snapshot: vec![0; MEM_SIZE],
            game_snapshot_valid: false,
            intro_seen: false,
            audio_rate: crate::audio::SAMPLE_RATE,
            gs_latch: 0,
            gs_command: 0,
            gs_busy: false,
            gs_sound_requests: Vec::new(),
            beeper_edges: Vec::new(),
            beeper_frame_start: false,
            beeper_filter: 0.0,
            audio_cycle_base: 0,
            audio_span_cycles: CYCLES_PER_FRAME,
            cycle_clock: 0,
            hooks: std::collections::BTreeMap::new(),
            hook_banks: std::collections::BTreeMap::new(),
            hook_max: std::collections::BTreeMap::new(),
            hook_yield: false,
            frame_target: u64::MAX,
            save_requested: false,
            load_requested: false,
            load_wait_game: false,
            mission_answer: 0xFF,
            hd: display::Frame::new(display::ZX_W, display::ZX_H),
            game_msg: GameMsgTap::default(),
        }
    }

    /// Load a raw block of bytes into memory at `addr`.
    #[cfg(test)]
    pub fn load(&mut self, addr: u16, data: &[u8]) {
        let mut a = addr as usize;
        for &b in data {
            if a >= MEM_SIZE {
                break;
            }
            self.mem[a] = b;
            if self.paging_enabled && a >= 0xC000 {
                let off = a - 0xC000;
                self.ram_banks[self.paged_bank][off] = b;
            }
            a += 1;
        }
    }

    /// Загрузить 128K-образ: восемь банков по 16K и начальное значение `0x7FFD`.
    ///
    /// Банки 5 и 2 сразу отображаются по `0x4000`/`0x8000`, остальные хранятся
    /// в `ram_banks`; выбранный портом банк копируется в `mem[0xC000..]`.
    pub fn load_game_image(&mut self, image: &[u8], page_reg: u8) {
        assert_eq!(image.len(), 8 * 0x4000, "нужен 128K-образ из 8 банков");
        let banks: Vec<Vec<u8>> = (0..8)
            .map(|i| image[i * 0x4000..(i + 1) * 0x4000].to_vec())
            .collect();
        self.ram_banks = banks;
        self.paging_enabled = true;
        self.paging_reg = page_reg;
        self.paging_locked = false;
        self.paging_writes = 0;
        self.paging_last = page_reg;
        self.paged_bank = (page_reg & 7) as usize;

        self.mem[0x4000..0x8000].copy_from_slice(&self.ram_banks[5]);
        self.mem[0x8000..0xC000].copy_from_slice(&self.ram_banks[2]);
        self.mem[0xC000..0x10000].copy_from_slice(&self.ram_banks[self.paged_bank]);
        self.game_snapshot_valid = false;
    }

    /// Установить хук, запомнив банк для адресов из `0xC000..0x10000`.
    pub fn install_hook(&mut self, addr: u16, hook: Hook, max_cycles: u64) {
        self.hooks.insert(addr, hook);
        self.hook_max.insert(addr, max_cycles);
        if addr >= 0xC000 {
            self.hook_banks.insert(addr, self.paged_bank as u8);
        }
    }

    /// Снять хук вместе с привязкой к банку.
    pub fn remove_hook(&mut self, addr: u16) {
        self.hooks.remove(&addr);
        self.hook_max.remove(&addr);
        self.hook_banks.remove(&addr);
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
        if self.snapshot_marker == Some(pc) || self.transition_snapshot_marker == Some(pc) {
            // `PrintCompass` вызывается в конце прохода `GameMain`: координаты,
            // созданные/удалённые объекты и рисунок уже полностью обновлены.
            // Снимок в этой точке не отстаёт от игры на целый проход.
            //
            // Вторая точка — начало ролика `sub_9296`. Гиперпрыжок создаёт
            // новую систему прямо перед этим вызовом, а сам ролик не
            // возвращается в `PrintCompass` до своего конца. Без снимка здесь
            // нативный тоннель показывал бы последний кадр перед прыжком.
            self.game_snapshot.copy_from_slice(&self.mem);
            self.game_snapshot_valid = true;
        }
        if self.game_marker == Some(pc) {
            self.intro_seen = false;
            self.load_wait_game = false;
        } else if let Some((lo, hi)) = self.intro_range {
            if pc >= lo && pc <= hi {
                self.intro_seen = true;
                self.game_snapshot_valid = false;
            }
        }
    }

    /// Сбросить снимок прохода и вернуть чтение живой памяти.
    ///
    /// Пока игра стоит на странице капитана, снимок, снятый до посадки,
    /// оставался бы актуальным и после вылета: рендер видел бы старую сцену
    /// вместо той, которую игра готовит к полёту. Снимок снимается заново в
    /// первом же проходе `GameMain` после вылета.
    pub fn invalidate_game_snapshot(&mut self) {
        self.game_snapshot_valid = false;
    }

    /// Состояние игры в последней точке снимка, откуда его безопасно читать
    /// нативному слою. До первого прохода — обычная память.
    pub fn game_state(&self) -> &[u8] {
        if self.game_snapshot_valid {
            &self.game_snapshot
        } else {
            &self.mem
        }
    }

    /// Run one PAL frame's worth of instructions, raising a maskable
    /// interrupt at the start of the frame (у Кладова IM2 -> 0xFEFE).
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
                // Хук для адреса из 0xC000.. привязан к банку, в котором он
                // установлен. В другом банке по тому же адресу лежит иной код.
                let bank_ok = self
                    .hook_banks
                    .get(&pc)
                    .map_or(true, |&bank| bank as usize == self.paged_bank);
                // Запускаем хук только если его верхняя оценка тактов
                // укладывается в остаток кадра. Иначе исполняем рутину на Z80:
                // так кадровая граница попадает туда же, где без хука.
                let fits = bank_ok
                    && self
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
        self.game_msg.end_text_frame();
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
