//! Нативные (Rust) реализации подпрограмм игры.
//!
//! Каждая функция здесь повторяет поведение конкретной рутины из игры
//! (адрес указан в комментарии). Цель — постепенно заменить выполнение этих
//! рутин на Z80, вплоть до полного отказа от эмуляции.
//!
//! Инвариант: поведение обязано совпадать с исходной рутиной. Для каждой
//! функции есть тест эквивалентности (см. `mod tests`), который прогоняет
//! настоящий код из образа `zx_bin/kladov.bin` на эмуляторе и сверяет
//! результат с нативной реализацией.

use crate::machine::{Hook, Spectrum};
use crate::z80::{Z80, ZF};

/// Установить все нативные замены (хуки) на машину.
pub fn install(m: &mut Spectrum) {
    m.hooks.insert(0x5DFB, font_base_hook);
    m.hooks.insert(0xEB3E, eb3e_hook);
    m.hooks.insert(0xD3CE, d3ce_hook);
    m.hooks.insert(0xAB0B, mult_a_hib_c_hook);
}

/// Сколько байт клавиш сбрасывает родная `ResetPressedKeys`.
const PRESSED_KEY_BYTES: usize = 4;

/// Раскладка сейва командира, зависящая от версии игры.
///
/// Адреса рутин и переменных у Кладова и в оригинале разные (original:
/// `MenuSaveLoad` = `0xD2E4`, `ControlKey1` = `0xFC0E`, `PilotName` = `0xA816`;
/// Кладова: `0xD04E`, `0xFC08`, `0xD232`), поэтому выбираем раскладку по
/// запущенной версии (`--elite`).
#[derive(Clone, Copy)]
pub struct SaveLayout {
    /// Точка входа родной рутины сохранения/загрузки (`MenuSaveLoad`).
    ///
    /// Вызывается из главного цикла только при нажатии игровой клавиши
    /// сохранения (у Кладова `0x749F: call 0xD04E`), поэтому перехват не
    /// срабатывает каждый кадр. Родная рутина в версии Кладова уходит в TR-DOS
    /// (`jp 0x3D2F`) с пейджингом 128K — в 48K-ядре это не работает.
    pub menu_save_load: u16,
    /// Первая из четырёх «залипающих» ячеек клавиш (`ControlKey1`).
    ///
    /// Родная `ResetPressedKeys` обнуляет четыре байта начиная отсюда:
    /// `ControlKey1`, `ControlKeyMove`, `ControlKey2`, `ControlKey3`.
    pub control_key1: u16,
    /// Адрес блока данных командира (`PilotName`).
    pub pilot_name: u16,
    /// Длина блока командира. Кладова копирует 102 байта (`ld bc,66h` в
    /// `DemoModeOn`), оригинал сохраняет 104 (`ld ix,PilotName; ld de,68h` в
    /// `sub_D41B`) — на два байта больше: `PilotMission1_1` и
    /// `Pilot_CrdXTarget`.
    pub commander_len: usize,
    /// Нативная реализация `MenuSaveLoad` для этой версии.
    hook: Hook,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD04E,
    control_key1: 0xFC08,
    pilot_name: 0xD232,
    commander_len: 0x66,
    hook: menu_save_load_kladov_hook,
};

/// Раскладка оригинальной версии (`zx_bin/elite.bin`).
pub const ELITE_SAVE: SaveLayout = SaveLayout {
    menu_save_load: 0xD2E4,
    control_key1: 0xFC0E,
    pilot_name: 0xA816,
    commander_len: 0x68,
    hook: menu_save_load_elite_hook,
};

/// Блок сохранения командира в памяти.
pub fn commander_block(m: &Spectrum, layout: SaveLayout) -> &[u8] {
    let a = layout.pilot_name as usize;
    &m.mem[a..a + layout.commander_len]
}

/// Записать блок командира в файл `path`.
pub fn commander_save(m: &Spectrum, path: &str, layout: SaveLayout) -> std::io::Result<()> {
    std::fs::write(path, commander_block(m, layout))
}

/// Прочитать блок командира из файла `path` и поместить в память.
pub fn commander_load(
    m: &mut Spectrum,
    path: &str,
    layout: SaveLayout,
) -> std::io::Result<()> {
    let data = std::fs::read(path)?;
    let n = data.len().min(layout.commander_len);
    let a = layout.pilot_name as usize;
    m.mem[a..a + n].copy_from_slice(&data[..n]);
    Ok(())
}

/// Включить перехват игрового сохранения: при вызове `MenuSaveLoad` игра
/// выставляет флаг `machine.save_requested`, а главный цикл показывает диалог
/// выбора файла и записывает блок командира (родная рутина требует
/// 128K-пейджинга и в 48K-ядре не работает).
pub fn install_save(m: &mut Spectrum, layout: SaveLayout) {
    m.hooks.insert(layout.menu_save_load, layout.hook);
}

/// Общий код нативной замены `MenuSaveLoad`: сбросить «залипшие» клавиши и
/// попросить главный цикл сохранить блок командира.
///
/// Родная рутина заканчивается `jp ResetPressedKeys`, который обнуляет
/// `ControlKey1`..`ControlKey3`. Это обязательная часть: главный цикл проверяет
/// бит 5 `ControlKeyMove` (клавиша сохранения), и без сброса он остаётся
/// установленным — сохранение повторяется на каждой итерации цикла (диалог
/// открывается бесконечно). Хук подменяет рутину целиком, поэтому сброс
/// делаем сами.
///
/// Возвращает «не загружено» (A=0, Z=1), как и родная рутина при обычном
/// возврате, чтобы главный цикл не уходил на ветку переинициализации вида
/// (`jp nz, 0x7394` у Кладова, `jp nz, loc_71BE` в оригинале).
fn menu_save_load(cpu: &mut Z80, m: &mut Spectrum, layout: SaveLayout) -> u64 {
    let a = layout.control_key1 as usize;
    m.mem[a..a + PRESSED_KEY_BYTES].fill(0);
    m.save_requested = true;
    cpu.a = 0;
    cpu.f |= ZF;
    0
}

/// Нативная замена `MenuSaveLoad` в версии Кладова (`0xD04E`).
fn menu_save_load_kladov_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, KLADOV_SAVE)
}

/// Нативная замена `MenuSaveLoad` в оригинальной версии (`0xD2E4`).
fn menu_save_load_elite_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    menu_save_load(cpu, m, ELITE_SAVE)
}

/// Точное число тактов Z80, которое рутина `sub_5DFB` занимает для кода `a`
/// (включая завершающий `RET`).
fn font_base_cycles(a: u8) -> u64 {
    match a {
        0x20 => 28,           // пробел: cp 20h -> ret z (taken)
        _ if a < 0x30 => 61,  // < '0'
        _ if a < 0x3B => 54,  // '0'..='9'
        _ if a < 0x41 => 87,  // ':'..'@'
        _ if a < 0x5B => 80,  // 'A'..'Z'
        _ if a < 0x61 => 113, // '['..'`'
        _ if a < 0x7B => 106, // a..z
        _ => 120,             // >= '{'
    }
}

/// Нативная замена `sub_5DFB` для механизма хуков.
fn font_base_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    #[cfg(test)]
    FONT_BASE_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let hl = font_base(cpu.a);
    cpu.h = (hl >> 8) as u8;
    cpu.l = (hl & 0xFF) as u8;
    font_base_cycles(cpu.a)
}

/// Счётчик обращений к хуку (только для тестов).
#[cfg(test)]
static FONT_BASE_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `sub_EB3E`: `ld d,0; ld e,a; add hl,de; ret` — прибавляет к `HL`
/// беззнаковый `A` (через `DE`), выставляя флаги сложения. Флаги `S/Z/PV`
/// 16-битного `ADD HL,rr` не трогает.
fn eb3e_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    cpu.d = 0;
    cpu.e = cpu.a;
    let r = cpu.add16(cpu.hl(), cpu.de());
    cpu.h = (r >> 8) as u8;
    cpu.l = r as u8;
    32 // ld d,0(7) + ld e,a(4) + add hl,de(11) + ret(10)
}

/// `sub_D3CE`: `push hl; sbc hl,de; pop hl; ret` — выставляет флаги по
/// `HL - DE - C` (16-битное вычитание с заёмом), но `HL` оставляет прежним.
fn d3ce_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = cpu.hl();
    let de = cpu.de();
    cpu.sbc16(hl, de);
    46 // push hl(11) + sbc hl,de(15) + pop hl(10) + ret(10)
}

/// Таблица умножения (`0xBF00`), к которой обращается `Mult_A_hiB_C`.
const MULT_TABLE: usize = 0xBF00;

/// Нативная замена `Mult_A_hiB_C` (`0xAB0B`).
///
/// Табличное умножение через таблицу квадратов по адресу `0xBF00`.
///
/// ```text
/// ld a,b ; sub c ; jr nc,keep ; neg ; keep:
/// srl a ; ld h,0BFh ; ld l,a ; ld d,(hl)
/// ld a,b ; add a,c ; rra ; ld l,a ; ld a,(hl) ; sub d ; ret
/// ```
///
/// Вход: `B`, `C`. Выход: `A = table[(B+C)/2] - table[|B-C|/2]`, `H=0xBF`,
/// `L=(B+C)/2`, `D=table[|B-C|/2]`. `B`, `C` не изменяются.
fn mult_a_hib_c_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let (b, c) = (cpu.b, cpu.c);
    let no_borrow = b >= c; // результат `sub c`: CF=0
    let diff = if no_borrow { b - c } else { c - b };
    let d = m.mem[MULT_TABLE + (diff >> 1) as usize];
    let idx = ((b as u16 + c as u16) >> 1) as u8; // `add a,c` + `rra`
    let a2 = m.mem[MULT_TABLE + idx as usize];
    cpu.a = cpu.sub8(a2, d, 0);
    cpu.h = 0xBF;
    cpu.l = idx;
    cpu.d = d;
    // `jr nc` — taken при отсутствии заёма (b >= c), тогда `neg` не выполняется.
    if no_borrow { 83 } else { 86 }
}

/// `sub_5DFB`: по коду символа в `A` выбирает базовый адрес шрифта.
///
/// Возвращает значение `HL`: `0x3D00` для «печатных» символов (пробел, цифры
/// `0`–`9`, латиница `A`–`Z`/`a`–`z`) и `0x5D70` для остальных. Регистр `A` не
/// изменяется.
pub fn font_base(a: u8) -> u16 {
    let printable = a == 0x20
        || (0x30..=0x3A).contains(&a)
        || (0x41..=0x5A).contains(&a)
        || (0x61..=0x7A).contains(&a);
    if printable { 0x3D00 } else { 0x5D70 }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Загружает полный образ Кладова (для прогона настоящих рутин на Z80).
    fn kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let data = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(None);
        m.load(0x4000, &data);
        m
    }

    /// Вызывает рутину по адресу `addr` как подпрограмму (`CALL`/`RET`) и
    /// выполняет её на Z80 до возврата в адрес-маркер.
    fn run_routine(m: &mut Spectrum, addr: u16) {
        const RET: u16 = 0x1234;
        m.cpu.sp = 0xF000;
        m.cpu.sp = m.cpu.sp.wrapping_sub(2);
        let [lo, hi] = RET.to_le_bytes();
        m.mem[m.cpu.sp as usize] = lo;
        m.mem[m.cpu.sp as usize + 1] = hi;
        m.cpu.pc = addr;
        let mut cpu = std::mem::take(&mut m.cpu);
        let mut guard = 0;
        while cpu.pc != RET && guard < 10_000 {
            cpu.step(m);
            guard += 1;
        }
        assert_eq!(cpu.pc, RET, "рутина 0x{addr:04X} не вернулась (guard={guard})");
        m.cpu = cpu;
    }

    /// Готовит машину к запуску версии Кладова так же, как `main.rs`.
    fn boot_kladov() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/kladov.bin"))
            .expect("zx_bin/kladov.bin (собрать: python3 tools/make_kladov_image.py)");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::KLADOV_LOAD_ADDR, &game);
        let c = &mut m.cpu;
        c.a = crate::KLADOV_STATE_A;
        c.f = crate::KLADOV_STATE_F;
        c.b = (crate::KLADOV_STATE_BC >> 8) as u8;
        c.c = (crate::KLADOV_STATE_BC & 0xFF) as u8;
        c.d = (crate::KLADOV_STATE_DE >> 8) as u8;
        c.e = (crate::KLADOV_STATE_DE & 0xFF) as u8;
        c.h = (crate::KLADOV_STATE_HL >> 8) as u8;
        c.l = (crate::KLADOV_STATE_HL & 0xFF) as u8;
        c.ix = crate::KLADOV_STATE_IX;
        c.iy = crate::KLADOV_STATE_IY;
        c.sp = crate::KLADOV_STATE_SP;
        c.pc = crate::KLADOV_STATE_PC;
        c.i = crate::KLADOV_I;
        c.r = crate::KLADOV_STATE_R;
        c.im = 2;
        c.iff1 = true;
        c.iff2 = true;
        m
    }

    /// Готовит машину к запуску оригинальной версии так же, как `main.rs`:
    /// образ по адресу `0x6048`, таблица IM2 на обработчик `0xFF25` и подмена
    /// опроса Kempston (`in a,(c)` -> `ld a,2` в `BeginGame`), без которой игра
    /// останавливается на выборе устройства управления.
    fn boot_elite() -> Spectrum {
        let base = env!("CARGO_MANIFEST_DIR");
        let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
        let game = std::fs::read(format!("{base}/zx_bin/elite.bin"))
            .expect("zx_bin/elite.bin (собрать: ./build.sh)");
        let mut m = Spectrum::new(Some(&rom));
        m.load(crate::GAME_LOAD_ADDR, &game);
        let [lo, hi] = crate::INTERRUPT_HANDLER.to_le_bytes();
        for i in 0..128u16 {
            let a = crate::IM2_TABLE + i * 2;
            m.mem[a as usize] = lo;
            m.mem[a as usize + 1] = hi;
        }
        m.mem[0x71F8] = 0x3E;
        m.mem[0x71F9] = 0x02;
        m.cpu.pc = crate::GAME_ENTRY;
        m.cpu.sp = crate::GAME_SP;
        m
    }

    /// Полный прогон с хуками обязан побайтово совпасть с обычным прогоном:
    /// такты, регистры и содержимое экранной области.
    #[test]
    fn hooks_match_plain_run() {
        FONT_BASE_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);
        let mut plain = boot_kladov();
        let mut hooked = boot_kladov();
        install(&mut hooked);
        for _ in 0..120 {
            plain.run_frame();
            hooked.run_frame();
        }
        assert!(
            FONT_BASE_CALLS.load(std::sync::atomic::Ordering::Relaxed) > 0,
            "хук font_base ни разу не вызван за 120 кадров"
        );
        assert_eq!(plain.cpu.cycles, hooked.cpu.cycles, "такт разошёлся");
        assert_eq!(plain.cpu.pc, hooked.cpu.pc, "PC разошёлся");
        assert_eq!(plain.cpu.sp, hooked.cpu.sp, "SP разошёлся");
        assert_eq!(plain.cpu.hl(), hooked.cpu.hl(), "HL разошёлся");
        assert_eq!(plain.cpu.bc(), hooked.cpu.bc(), "BC разошёлся");
        assert_eq!(plain.cpu.de(), hooked.cpu.de(), "DE разошёлся");
        assert_eq!(
            &plain.mem[0x4000..0x5B00],
            &hooked.mem[0x4000..0x5B00],
            "экран разошёлся"
        );
    }

    /// Снимок регистров и тактов для строгого сравнения.
    #[derive(Debug, PartialEq)]
    struct Snap {
        a: u8,
        f: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        h: u8,
        l: u8,
        cycles: u64,
    }

    fn snap(c: &Z80) -> Snap {
        Snap {
            a: c.a,
            f: c.f,
            b: c.b,
            c: c.c,
            d: c.d,
            e: c.e,
            h: c.h,
            l: c.l,
            cycles: c.cycles,
        }
    }

    fn set(c: &mut Z80, a: u8, hl: u16, de: u16, f: u8) {
        c.a = a;
        c.f = f;
        c.h = (hl >> 8) as u8;
        c.l = hl as u8;
        c.d = (de >> 8) as u8;
        c.e = de as u8;
    }

    /// Прогон рутины на Z80 со счётчиком тактов от нуля.
    fn z80_sub(m: &mut Spectrum, addr: u16) -> Snap {
        m.cpu.cycles = 0;
        run_routine(m, addr);
        snap(&m.cpu)
    }

    /// Прогон нативной рутины через таблицу хуков.
    fn hook_sub(m: &mut Spectrum, addr: u16) -> Snap {
        let mut cpu = std::mem::take(&mut m.cpu);
        cpu.cycles = 0;
        let cyc = (m.hooks[&addr])(&mut cpu, m);
        cpu.cycles = cyc;
        m.cpu = cpu;
        snap(&m.cpu)
    }

    /// Регистровые рутины обязаны совпасть с Z80 по ВСЕМ регистрам, флагам
    /// и числу тактов.
    #[test]
    fn register_helpers_match_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n);
        let cases: &[(u8, u16, u16, u8)] = &[
            (0x00, 0x0000, 0x0000, 0x00),
            (0x01, 0x0001, 0x0000, 0x00),
            (0xFF, 0x00FF, 0x0001, 0x01),
            (0x80, 0xFF00, 0x00FF, 0x00),
            (0x7F, 0x0FFF, 0x0001, 0x00),
            (0x55, 0x8000, 0x8000, 0x01),
            (0xAA, 0x1234, 0x5678, 0x00),
            (0x00, 0xFFFF, 0x0001, 0x01),
            (0x03, 0x00FF, 0xFF00, 0x00),
        ];
        for &(a, hl, de, f) in cases {
            for &addr in &[0xEB3Eu16, 0xD3CEu16] {
                set(&mut z.cpu, a, hl, de, f);
                set(&mut n.cpu, a, hl, de, f);
                let zs = z80_sub(&mut z, addr);
                let ns = hook_sub(&mut n, addr);
                assert_eq!(
                    zs, ns,
                    "рутина 0x{addr:04X}: вход a={a:02X} hl={hl:04X} de={de:04X} f={f:02X}"
                );
            }
        }
    }

    /// `Mult_A_hiB_C` обязан совпасть с Z80 на всех 256×256 парах `B`,`C`
    /// по всем регистрам, флагам и числу тактов.
    #[test]
    fn mult_a_hib_c_matches_z80() {
        let mut z = kladov();
        let mut n = kladov();
        install(&mut n);
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                z.cpu.b = b;
                z.cpu.c = c;
                z.cpu.f = b ^ c;
                n.cpu.b = b;
                n.cpu.c = c;
                n.cpu.f = b ^ c;
                let zs = z80_sub(&mut z, 0xAB0B);
                let ns = hook_sub(&mut n, 0xAB0B);
                assert_eq!(zs, ns, "Mult_A_hiB_C: b={b:02X} c={c:02X}");
            }
        }
    }

    #[test]
    fn font_base_matches_z80() {
        let mut m = kladov();
        for a in 0u8..=255 {
            m.cpu.a = a;
            run_routine(&mut m, 0x5DFB);
            let z80 = m.cpu.hl();
            let native = font_base(a);
            assert_eq!(
                native, z80,
                "font_base(0x{a:02X}): native=0x{native:04X} z80=0x{z80:04X}"
            );
        }
    }

    /// Сохранение и загрузка блока командира в файл — round-trip (обе версии).
    #[test]
    fn commander_roundtrip() {
        let base = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{base}/tmp");
        std::fs::create_dir_all(&dir).unwrap();

        for layout in [KLADOV_SAVE, ELITE_SAVE] {
            let path = format!("{dir}/commander_roundtrip_{:04X}.zxelite", layout.pilot_name);
            let mut m = kladov();
            let a = layout.pilot_name as usize;
            let n = layout.commander_len;
            m.mem[a] = b'T';
            m.mem[a + 1] = b'E';
            m.mem[a + 2] = b'S';
            m.mem[a + 3] = b'T';
            m.mem[a + n - 1] = 0x5A;
            let expected: Vec<u8> = commander_block(&m, layout).to_vec();
            assert_eq!(expected.len(), n);

            commander_save(&m, &path, layout).unwrap();
            for b in &mut m.mem[a..a + n] {
                *b = 0xAA;
            }
            commander_load(&mut m, &path, layout).unwrap();
            assert_eq!(
                commander_block(&m, layout),
                &expected[..],
                "блок командира (0x{:04X}) не совпал",
                layout.pilot_name
            );

            let _ = std::fs::remove_file(&path);
        }
    }

    /// Перехват `MenuSaveLoad`: хук должен выставить запрос на сохранение и
    /// сбросить «залипшие» клавиши (`ControlKey1`..`ControlKey3`), как это
    /// делает родная рутина через `ResetPressedKeys`. Проверяем обе версии.
    #[test]
    fn menu_save_load_hook_sets_request() {
        for layout in [KLADOV_SAVE, ELITE_SAVE] {
            let mut m = kladov();
            install_save(&mut m, layout);
            assert!(
                m.hooks.contains_key(&layout.menu_save_load),
                "хук MenuSaveLoad 0x{:04X} не установлен",
                layout.menu_save_load
            );
            assert!(!m.save_requested);
            // Имитируем нажатие клавиши сохранения (Symbol Shift -> бит 5
            // `ControlKeyMove`) и других залипших клавиш.
            let a = layout.control_key1 as usize;
            m.mem[a] = 0x50;
            m.mem[a + 1] = 0x20;
            m.mem[a + 2] = 0x80;
            m.mem[a + 3] = 0x01;
            let hook = m.hooks.get(&layout.menu_save_load).copied().unwrap();
            let mut cpu = std::mem::take(&mut m.cpu);
            let cycles = hook(&mut cpu, &mut m);
            m.cpu = cpu;
            assert_eq!(cycles, 0);
            assert!(m.save_requested, "запрос на сохранение не выставлен");
            assert_eq!(
                &m.mem[a..a + PRESSED_KEY_BYTES],
                &[0u8; PRESSED_KEY_BYTES][..],
                "залипшие клавиши не сброшены (0x{:04X})",
                layout.control_key1
            );
        }
    }

    /// Прогон сценария «нажали клавишу сохранения» и подсчёт запросов.
    fn count_save_requests(m: &mut Spectrum) -> usize {
        let mut requests = 0;
        for f in 0..260 {
            m.release_all_keys();
            if (30..80).contains(&f) {
                m.set_key(7, 0, true); // SPACE — старт полёта с титула
            }
            if f == 150 {
                m.set_key(7, 1, true); // Symbol Shift — клавиша сохранения (1 кадр)
            }
            m.run_frame();
            if m.save_requested {
                m.save_requested = false;
                requests += 1;
            }
        }
        requests
    }

    /// Регрессия на бесконечный цикл сохранения: одно нажатие клавиши
    /// сохранения (Symbol Shift) должно дать ровно один запрос, а не
    /// повторяться на каждой итерации главного цикла. Проверяем обе версии.
    #[test]
    fn save_request_fires_once_per_press() {
        let mut kladov = boot_kladov();
        install_save(&mut kladov, KLADOV_SAVE);
        let requests = count_save_requests(&mut kladov);
        assert_eq!(
            requests, 1,
            "Кладова: сохранение сработало {requests} раз вместо одного"
        );

        let mut elite = boot_elite();
        install_save(&mut elite, ELITE_SAVE);
        let requests = count_save_requests(&mut elite);
        assert_eq!(
            requests, 1,
            "оригинал: сохранение сработало {requests} раз вместо одного"
        );
    }
}
