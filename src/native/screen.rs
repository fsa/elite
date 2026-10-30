//! Экранные примитивы: перевод пиксельных координат в адрес
//! (`PixCrd2ScrAdr`), насыщающее сложение (`SmagAddMem`), шаг к следующей
//! строке экрана, XOR-столбцы и копирование альтернативных линий экрана.

use super::*;

/// `sub_E621` (`PixCrd2ScrAdr` в оригинале): пиксельные координаты -> адрес
/// экранной области ZX Spectrum.
///
/// Вход: `X = A` (пиксельная колонка 0..255), `Y = C` (пиксельная строка
/// 0..191). Выход: `HL` — адрес байта битмапа, содержащего пиксель.
pub fn pix_crd_2_scr_adr(x: u8, y: u8) -> u16 {
    let l = ((y & 0x38) << 2) | ((x >> 3) & 0x1F);
    let h = 0x40 | (y & 7) | ((y & 0xC0) >> 3);
    ((h as u16) << 8) | l as u16
}

/// Нативная замена `sub_E621`. Рутина полностью линейная (ветвлений нет),
/// поэтому такты фиксированы (120). Единственная операция, задающая итоговые
/// флаги, — финальное `or h`; флаги `rra`/`and`/`add` до неё перезаписываются.
pub(crate) fn pix_crd_2_scr_adr_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let hl = pix_crd_2_scr_adr(cpu.a, cpu.c);
    let h = (hl >> 8) as u8;
    let a_before = ((cpu.c & 0xC0) >> 3) & 0x1F; // результат `and 1Fh`
    cpu.a = cpu.or8(a_before, h); // финальное `or h` (выставляет флаги)
    cpu.h = h;
    cpu.l = hl as u8;
    bump_r(cpu, 24); // рутина линейная: 24 инструкции
    120
}

/// `smag_add_mem` (`sub_7A59`): прибавить к байту по `HL` значение `A` в
/// формате «знак + 7-битная величина» (знак — бит 7, величина — биты 0..6).
/// Рутина горячая: вызывается на каждое обновление координаты звезды в
/// `PrintStars` (звёздные координаты хранятся именно в этом формате).
///
/// Правила (по эталонному коду):
/// - одинаковые знаки: величины складываются; если сумма ≥ 80h, величина
///   обнуляется, знак сохраняется;
/// - разные знаки: величины вычитаются, знак результата — от большей величины
///   (при равенстве — от прежнего значения по адресу).
///
/// Результат записывается в `(HL)`, `BC` сохраняются (рутина делает
/// `push bc`/`pop bc`).
pub fn smag_add_mem(old: u8, a: u8) -> u8 {
    if (a ^ old) & 0x80 == 0 {
        let sum = (a & 0x7F) + (old & 0x7F);
        if sum & 0x80 != 0 { a & 0x80 } else { (a & 0x80) | sum }
    } else if old & 0x7F >= a & 0x7F {
        (old & 0x80) | (old & 0x7F) - (a & 0x7F)
    } else {
        (a & 0x80) | (a & 0x7F) - (old & 0x7F)
    }
}

/// Нативная замена `sub_7A59`. Итоговые флаги задаёт финальное `or c`
/// (операции `xor`/`and`/`add`/`sub`/`bit`/`neg` перезаписываются).
pub(crate) fn smag_add_mem_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let hl = cpu.hl() as usize;
    let old = m.mem[hl];
    let a = cpu.a;
    let res = smag_add_mem(old, a);
    m.mem[hl] = res;
    cpu.a = cpu.or8(res, 0); // финальные флаги — от `or c`
    let same_sign = (a ^ old) & 0x80 == 0;
    if same_sign {
        // без переполнения: 20 инструкций (132 такта); с переполнением
        // (`sbc a,a`): 21 инструкция (131 такт).
        if ((a & 0x7F) + (old & 0x7F)) & 0x80 == 0 {
            bump_r(cpu, 20);
            132
        } else {
            bump_r(cpu, 21);
            131
        }
    } else if old & 0x7F >= a & 0x7F {
        bump_r(cpu, 21); // `jr nc` берётся, `neg` не выполняется
        148
    } else {
        bump_r(cpu, 21);
        144
    }
}

/// `scr_adr_next_char_line` (`compDownScrAdr_HL`, `0x8445`): продолжение
/// расчёта экранного адреса при переходе на строку чартов ниже.
///
/// ```text
/// ld a,l ; add a,20h ; ld l,a ; ret c ; ld a,h ; sub 8 ; ld h,a ; ret
/// ```
pub(crate) fn scr_adr_next_char_line_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    let sum = cpu.add8(cpu.l, 0x20, 0);
    cpu.a = sum;
    cpu.l = sum;
    if cpu.f & CF != 0 {
        bump_r(cpu, 4); // ld a,l; add a,20h; ld l,a; ret c
        return 26; // ld a,l(4)+add a,20h(7)+ld l,a(4)+ret c(11)
    }
    let d = cpu.sub8(cpu.h, 8, 0);
    cpu.a = d;
    cpu.h = d;
    bump_r(cpu, 8); // + ld a,h; sub 8; ld h,a; ret
    45 // 4+7+4+5 + 4+7+4+10
}

/// `scr_adr_next_line` (`sub_7FE7`, `0x7FE7`): переход экранного адреса на
/// одну пиксельную строку вниз.
///
/// ```text
/// inc h ; ld a,h ; and 7 ; call z, compDownScrAdr_HL ; ret
/// ```
///
/// Это регистровый примитив: им пользуется и хук адреса `0x7FE7`, и хук
/// `0x7FCD`, который сам ведёт резиду стека в нужные слоты.
pub(crate) fn scr_adr_next_line_advance(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let h = cpu.h.wrapping_add(1);
    cpu.h = h;
    let a = cpu.and8(h, 7);
    cpu.a = a;
    // Свои инструкции: inc h, ld a,h, and 7, call z (взята или нет) и
    // финальный `ret` — всегда 5; внутренний хук добавляет свои 4/8.
    bump_r(cpu, 5);
    if a != 0 {
        return 35; // inc h(4)+ld a,h(4)+and 7(7)+call z not taken(10)+ret(10)
    }
    let inner = scr_adr_next_char_line_hook(cpu, m);
    // 4+4+7+17 (вход в ветку) + 10 (финальный ret)
    42 + inner
}

/// Хук адреса `0x7FE7`: примитив `scr_adr_next_line_advance` плюс резида
/// стека от `call z, compDownScrAdr_HL` — по [SP-2] ложится адрес возврата
/// 7FEEh, как у настоящего `call`. В игре стек живёт в экранной памяти
/// (SP=4116h..411Eh), и этот байт виден на экране, поэтому его нужно
/// воспроизводить. Хук работает и для обычного `call 0x7FE7h`, и для
/// jp-режима, который игра включает самомодификацией (см. `0xDA2F`).
pub(crate) fn scr_adr_next_line_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let taken = cpu.h.wrapping_add(1) & 7 == 0;
    let cycles = scr_adr_next_line_advance(cpu, m);
    if taken {
        let sp = cpu.sp;
        mem_write(m, sp.wrapping_sub(2), 0xEE);
        mem_write(m, sp.wrapping_sub(1), 0x7F);
    }
    cycles
}

/// Адрес самомодифицирующегося опкода `xor c` внутри `sub_D546`.
pub(crate) const XOR_COL_OP: usize = 0xD548;
/// Значение опкода `xor c` (режим XOR); игра пишет на это место только `00h`
/// (`nop` — режим копирования) и `A9h` (`xor c` — режим XOR).
pub(crate) const XOR_COL_XOR_OP: u8 = 0xA9;

/// `scr_xor_col_down` (`sub_D546`): отрисовка вертикальной колонки из `B`
/// байтов: `(HL+i*100h) ^= (DE+i)` (режим XOR) либо `(HL+i*100h) = (DE+i)`
/// (режим копирования).
///
/// ```text
/// loop: ld a,(de) ; ld c,(hl) ; xor c ; ld (hl),a ; inc h ; inc de
///       djnz loop
/// ret
/// ```
///
/// Рутина **самоизменяется**: по адресу `0xD548` лежит опкод `xor c`,
/// который игра переключает между `A9h` (XOR — двухпроходное рисование
/// спрайтов, вторая проходка стирает первый) и `00h` (`nop` — обычное
/// копирование). Значение читается из памяти в момент вызова. Других значений
/// игра на это место не пишет.
///
/// `B = 0` означает 256 проходов (как у `djnz`). `inc h` двигает адрес вниз
/// по колонке (только старший байт, `L` не меняется). Записи в адреса ниже
/// `writable_from` (область ПЗУ) отбрасываются — так же поступает шина
/// настоящего Spectrum. Возвращает итоговые `(A, C, HL, DE)`: `A` — последний
/// вычисленный байт (пишется он или нет), `C` — содержимое экрана до записи
/// в последнем проходе, `B = 0`.
pub fn scr_xor_col_down(
    mem: &mut [u8],
    hl: u16,
    de: u16,
    b: u8,
    xor_mode: bool,
    writable_from: usize,
) -> (u8, u8, u16, u16) {
    let mut hl = hl;
    let mut de = de;
    let mut a = 0u8;
    let mut c = 0u8;
    let mut n = if b == 0 { 256u32 } else { b as u32 };
    while n != 0 {
        a = mem[de as usize];
        c = mem[hl as usize];
        if xor_mode {
            a ^= c;
        }
        if hl as usize >= writable_from {
            mem[hl as usize] = a;
        }
        hl = hl.wrapping_add(0x0100); // `inc h`: только старший байт, L не меняется
        de = de.wrapping_add(1);
        n -= 1;
    }
    (a, c, hl, de)
}

/// Нативная замена `sub_D546`. Итоговые флаги задаёт последний `inc h`
/// (`SZ5.3` и `PV` переполнения от итогового `H`); `xor c` всегда обнуляет
/// `C`, а в режиме копирования ни одна инструкция цикла флаги не трогает —
/// `C` остаётся от входа в рутину.
pub(crate) fn scr_xor_col_down_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let n = if cpu.b == 0 { 256u64 } else { cpu.b as u64 };
    let xor_mode = m.mem[XOR_COL_OP] == XOR_COL_XOR_OP;
    let cf_in = cpu.f & CF;
    let (a, c, hl, de) =
        scr_xor_col_down(&mut m.mem, cpu.hl(), cpu.de(), cpu.b, xor_mode, ROM_SIZE);
    cpu.a = a;
    cpu.c = c;
    cpu.b = 0;
    cpu.h = (hl >> 8) as u8;
    cpu.d = (de >> 8) as u8;
    cpu.e = de as u8;
    let h = (hl >> 8) as u8;
    cpu.f = (h & (SF | XF | YF))
        | if h == 0 { ZF } else { 0 }
        | if h == 0x80 { PF } else { 0 }
        | if h & 0x0F == 0 { HF } else { 0 } // полуперенос последнего `inc h`
        | if xor_mode { 0 } else { cf_in };
    bump_r(cpu, (7 * n + 1) as u32); // 7 инструкций на проход + `ret`
    48 * n + 5 // 35 на проход + 13 (`djnz` взят) на все, кроме последнего, + 8 + 10
}

/// Число строк копирования в `scr_copy_alt_lines` (`ld b,3Ch`).
pub(crate) const COPY_ALT_LINES: u8 = 0x3C;
/// Длина строки в байтах (`ld bc,0020h` перед `ldir`).
pub(crate) const COPY_ALT_LINE: u16 = 0x20;
/// Верхняя оценка тактов одной итерации цикла `scr_copy_alt_lines`. Складывается
/// из push bc/hl/de (33), ld bc (10), ldir (37·31+32 = 1179), pop de/hl/bc (30),
/// двух юнитов call 7FE3/ex de,hl/call 7FE7 (2·38 = 76), четырёх сдвигов адреса
/// (4·87 = 348, худший случай) и djnz (13) — итого 1689. Хук гоняет итерацию
/// быстро, только если она целиком укладывается в остаток кадра; иначе отдаёт
/// управление Z80, чтобы прерывание сработало ровно там же, где без хука.
pub(crate) const COPY_ALT_MAX_ITER: u64 = 1689;

/// `scr_copy_alt_lines` (`0x7FCD`, внутренний цикл `CopyBuf2Screen`): копирует
/// `3Ch` строк по `20h` байтов из `HL` в `DE`, после каждой строки сдвигая оба
/// указателя на **две** пиксельные строки — то есть заполняет строки «через
/// одну». Записи в адреса ниже `ROM_SIZE` отбрасываются — так же поступает
/// шина настоящего Spectrum.
///
/// ```text
///         ld b,3Ch
/// loop:   push bc ; push hl ; push de
///         ld bc,20h
///         ldir
///         pop de ; pop hl ; pop bc
///         call unit ; call unit
///         djnz loop
///         ret
/// unit:   ex de,hl ; call 7FE7h
/// 7FE7h:  inc h ; ld a,h ; and 7 ; call z, compDownScrAdr_HL ; ret
/// ```
///
/// `call 7FE7h` кладёт в стек адрес, указывающий на собственное тело: после
/// первого `ret` тело выполняется повторно, поэтому каждый `unit` двигает
/// указатель на две строки, а не на одну. `CopyBuf2Screen` (`0x7FBE`) вызывает
/// рутину дважды подряд (0xC000→0x4020 и 0xC100→0x4120): проходы дополняют
/// друг друга — первый пишет строки 8, 10, …, второй 9, 11, … и вместе
/// покрывают экран начиная со строки 8.
///
/// `push`/`pop`/`ld bc` на регистрах в сумме следа не оставляют (кроме `B`,
/// который `djnz` обнуляет, и `F` от последнего `ldir`, перекрываемого телом
/// `scr_adr_next_line`), поэтому хук копирует прямо по текущим `HL`/`DE`, а
/// сдвиг указателей выполняют те же точные примитивы, что и настоящий код.
///
/// Резиду в СТЕКЕ, напротив, обязателен: в игре стек живёт прямо в экранной
/// памяти (SP=4116h..411Eh — строки 1–7, которые копирование не трогает),
/// и шесть байт под SP с регистрами и адресами возврата видны на экране.
/// Слоты: `push bc/hl/de` пишут в S-2/S-4/S-6; `call 7FE3h` перезаписывает
/// S-2 адресом возврата (7FDDh/7FE0h), `call 7FE7h` — S-4 (7FE7h). Тело
/// под `call 7FE7h` работает с SP=S-4 и пишет резиду `call z` (7FEEh) в
/// слот S-6, а его повторный прогон после `ret` идёт с SP=S-2 и
/// перезаписывает слот S-4. Хук пишет резиду в том же хронологическом
/// порядке, что и настоящие `push`/`call`, включая перезапись ldir-копией.
///
/// Прямые заходы в середину рутины (профиль показал единичные попадания на
/// `0x7FCF`/`0x7FD5` минуя вход) хук не перехватывает — код рутины в памяти
/// сохранён и выполняется как есть.
pub(crate) fn scr_copy_alt_lines_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let mut cycles = 7u64; // ld b,3Ch (в этом эмуляторе ld r,n = 7)
    bump_r(cpu, 1);
    cpu.b = COPY_ALT_LINES;
    let sp = cpu.sp;
    // Остаток кадра считаем от такта на входе; `frame_target` = u64::MAX вне
    // кадрового цикла (тесты) — тогда хук отрабатывает рутину целиком.
    let budget_end = m.frame_target.saturating_sub(cpu.cycles);
    loop {
        // Итерация длиннее остатка кадра — отдаём управление в точку цикла
        // 0x7FCF с состоянием на начало итерации. Дальше Z80 доиграет рутину
        // как обычно, и vblank-прерывание попадёт ровно в тот же такт, что и
        // без хука (иначе хук «проскочил» бы прерывание и разошёлся с plain).
        if cycles + COPY_ALT_MAX_ITER > budget_end {
            cpu.pc = 0x7FCF;
            m.hook_yield = true;
            return cycles;
        }
        // push bc ; push hl ; push de
        bump_r(cpu, 3);
        cycles += 33;
        let hl = cpu.hl();
        let de = cpu.de();
        mem_write(m, sp.wrapping_sub(2), cpu.c);
        mem_write(m, sp.wrapping_sub(1), cpu.b);
        mem_write(m, sp.wrapping_sub(4), cpu.l);
        mem_write(m, sp.wrapping_sub(3), cpu.h);
        mem_write(m, sp.wrapping_sub(6), cpu.e);
        mem_write(m, sp.wrapping_sub(5), cpu.d);
        // ld bc,20h + ldir. У эмулятора каждый байт ldir — отдельный step:
        // повторяющиеся байты стоят block_ldi(16)+21, последний 16+16.
        let mut last = 0u8;
        for i in 0..COPY_ALT_LINE {
            let s = hl.wrapping_add(i) as usize;
            let d = de.wrapping_add(i) as usize;
            last = m.mem[s];
            if d >= ROM_SIZE {
                m.mem[d] = last;
            }
        }
        // Флаги block_ldi: S,Z,C сохраняются от входа, X/Y от A+последнего
        // перенесённого байта, PV=0 (BC кончился), N=H=0.
        let n = cpu.a.wrapping_add(last);
        cpu.f = (cpu.f & (SF | ZF | CF)) | (n & XF) | if n & 0x02 != 0 { YF } else { 0 };
        bump_r(cpu, 1 + COPY_ALT_LINE as u32);
        cycles += 10 + 37 * (COPY_ALT_LINE as u64 - 1) + 32;
        // pop de ; pop hl ; pop bc. Регистры восстанавливаются ИЗ ПАМЯТИ:
        // когда копия перекрывает кадр стека (в игре SP=411Eh, и строка 1
        // экрана лежит в 4100h..411Fh), `pop` возвращает уже испорченные
        // `ldir` значения — ровно так, как это делает настоящий Z80.
        bump_r(cpu, 3);
        cycles += 30;
        cpu.e = m.mem[sp.wrapping_sub(6) as usize];
        cpu.d = m.mem[sp.wrapping_sub(5) as usize];
        cpu.l = m.mem[sp.wrapping_sub(4) as usize];
        cpu.h = m.mem[sp.wrapping_sub(3) as usize];
        cpu.c = m.mem[sp.wrapping_sub(2) as usize];
        cpu.b = m.mem[sp.wrapping_sub(1) as usize];
        for unit in 0..2u16 {
            std::mem::swap(&mut cpu.h, &mut cpu.d);
            std::mem::swap(&mut cpu.l, &mut cpu.e);
            bump_r(cpu, 3); // call unit + ex de,hl + call 7FE7h
            cycles += 17 + 4 + 17;
            // call 7FE3h кладёт в стек адрес возврата (7FDDh или 7FE0h),
            // затем call 7FE7h кладёт 7FE7h — по два байта на слот.
            let ret = if unit == 0 { 0x7FDDu16 } else { 0x7FE0 };
            mem_write(m, sp.wrapping_sub(2), ret as u8);
            mem_write(m, sp.wrapping_sub(1), (ret >> 8) as u8);
            mem_write(m, sp.wrapping_sub(4), 0xE7);
            mem_write(m, sp.wrapping_sub(3), 0x7F);
            // Первое тело работает под `call 7FE7h` (SP=S-4) и пишет резиду
            // `call z` в слот S-6; повторный прогон после `ret` идёт с
            // SP=S-2 и перезаписывает слот S-4 — адрес возврата 7FE7h.
            for slot in [6u16, 4] {
                // call z,8445h берётся, когда (H+1) mod 8 == 0 — так же
                // решает внутренний хук scr_adr_next_line.
                if cpu.h.wrapping_add(1) & 7 == 0 {
                    mem_write(m, sp.wrapping_sub(slot), 0xEE);
                    mem_write(m, sp.wrapping_sub(slot - 1), 0x7F);
                }
                cycles += scr_adr_next_line_advance(cpu, m);
            }
        }
        // djnz loop
        bump_r(cpu, 1);
        cpu.b = cpu.b.wrapping_sub(1);
        if cpu.b == 0 {
            cycles += 8;
            break;
        }
        cycles += 13;
    }
    bump_r(cpu, 1); // ret
    cycles + 10
}
