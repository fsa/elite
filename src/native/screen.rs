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

/// Число проходов внутреннего цикла `0xB2A0` (`ld a,4`) и внешнего цикла
/// `0x7FCD` (`ld a,0Fh`) в оптимизированной версии рутины.
const COPY_ALT_INNER: u8 = 4;
const COPY_ALT_OUTER: u8 = 15;
/// Длина одной строки в байтах (32 `ldi` за проход `0xB2A0`).
const COPY_ALT_LINE: u16 = 0x20;
/// Верхняя оценка тактов `scr_copy_alt_lines` (все 30 вызовов `0x8445` идут
/// по длинной ветке): 36_661 такт.
pub(crate) const COPY_ALT_MAX_ITER: u64 = 37_000;

/// `scr_copy_alt_lines` (`0x7FCD`) в 128K-образе Кладова. Оптимизированная
/// версия отличается от старой 48K: вместо `ld b,3Ch` + `ldir` здесь
/// `ld bc,01E0h`, внешний счётчик `A=0Fh` и 4 прохода по 32 `ldi` с
/// интересом адресов на `0x200` внутри `0xB2A0`. Итого 15×4×32 = 1920 байт —
/// те же 60 строк «через одну», что и раньше, но за ~36K тактов вместо ~99K
/// (укладывается в один кадр, поэтому `Yield` больше не нужен).
///
/// ```text
/// 7FCD: ld bc,01E0h ; ld a,0Fh ; ex af,af'
/// loop: ex af,af' ; call B2A0h ; ex de,hl ; call 8445h
///       ex de,hl ; call 8445h ; ex af,af' ; dec a ; jr nz,loop ; ret
/// B2A0: ld a,04h
/// inner: 32× ldi ; ld c,0E0h
///        ex de,hl ; add hl,bc ; ex de,hl ; add hl,bc
///        dec a ; jp nz,inner ; ret
/// ```
///
/// Сдвиг на `0x200` получается как 32 (`ldi`) + `0x1E0` (`add hl,bc`).
/// Резиду стека: три `call` пишут по `SP-2` адреса возврата `7FD6h`,
/// `7FDAh`, `7FDEh` (последний остаётся в памяти).
pub(crate) fn scr_copy_alt_lines_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let sp = cpu.sp;
    // ld bc,01E0h ; ld a,0Fh. Первый `ex af,af'` — это уже начало цикла
    // по адресу 7FD2h, отдельного обмена на входе в рутину нет.
    cpu.b = 0x01;
    cpu.c = 0xE0;
    cpu.a = 0x0F;
    bump_r(cpu, 2);
    let mut cycles = 17u64;

    let mut outer = COPY_ALT_OUTER;
    loop {
        // 7FD2h: ex af,af' — вернуть счётчик внешнего цикла в A.
        std::mem::swap(&mut cpu.a, &mut cpu.a_);
        std::mem::swap(&mut cpu.f, &mut cpu.f_);
        bump_r(cpu, 2); // ex af,af' + call B2A0h
        cycles += 4 + 17;
        mem_write(m, sp.wrapping_sub(2), 0xD6);
        mem_write(m, sp.wrapping_sub(1), 0x7F);

        // B2A0h: ld a,04h, затем 4 прохода по 32 `ldi`.
        cpu.a = COPY_ALT_INNER;
        bump_r(cpu, 1);
        cycles += 7;
        let mut inner = COPY_ALT_INNER;
        loop {
            for _ in 0..COPY_ALT_LINE {
                let src = cpu.hl() as usize;
                let dst = cpu.de() as usize;
                let v = m.mem[src];
                if dst >= ROM_SIZE {
                    m.mem[dst] = v;
                }
                let hl = cpu.hl().wrapping_add(1);
                cpu.h = (hl >> 8) as u8;
                cpu.l = hl as u8;
                let de = cpu.de().wrapping_add(1);
                cpu.d = (de >> 8) as u8;
                cpu.e = de as u8;
                cpu.c = cpu.c.wrapping_sub(1);
                if cpu.c == 0 {
                    cpu.b = cpu.b.wrapping_sub(1);
                }
                let n = cpu.a.wrapping_add(v);
                cpu.f = (cpu.f & (SF | ZF | CF))
                    | (n & XF)
                    | if n & 0x02 != 0 { YF } else { 0 }
                    | if cpu.bc() != 0 { PF } else { 0 };
                bump_r(cpu, 1);
                cycles += 16;
            }
            // ld c,0E0h ; ex de,hl ; add hl,bc ; ex de,hl ; add hl,bc
            cpu.c = 0xE0;
            bump_r(cpu, 1);
            cycles += 7;
            std::mem::swap(&mut cpu.h, &mut cpu.d);
            std::mem::swap(&mut cpu.l, &mut cpu.e);
            let r = cpu.add16(cpu.hl(), cpu.bc());
            cpu.h = (r >> 8) as u8;
            cpu.l = r as u8;
            bump_r(cpu, 2); // ex de,hl + add hl,bc
            cycles += 4 + 11;
            std::mem::swap(&mut cpu.h, &mut cpu.d);
            std::mem::swap(&mut cpu.l, &mut cpu.e);
            let r = cpu.add16(cpu.hl(), cpu.bc());
            cpu.h = (r >> 8) as u8;
            cpu.l = r as u8;
            bump_r(cpu, 2); // ex de,hl + add hl,bc
            cycles += 4 + 11;
            // dec a ; jp nz,inner
            cpu.a = cpu.dec8(cpu.a);
            bump_r(cpu, 2);
            cycles += 4 + 10;
            inner -= 1;
            if inner == 0 {
                break;
            }
        }
        // ret из B2A0h.
        bump_r(cpu, 1);
        cycles += 10;

        // ex de,hl ; call 8445h (сдвиг HL)
        std::mem::swap(&mut cpu.h, &mut cpu.d);
        std::mem::swap(&mut cpu.l, &mut cpu.e);
        bump_r(cpu, 2);
        cycles += 4 + 17;
        mem_write(m, sp.wrapping_sub(2), 0xDA);
        mem_write(m, sp.wrapping_sub(1), 0x7F);
        cycles += scr_adr_next_char_line_hook(cpu, m);

        // ex de,hl ; call 8445h (сдвиг DE)
        std::mem::swap(&mut cpu.h, &mut cpu.d);
        std::mem::swap(&mut cpu.l, &mut cpu.e);
        bump_r(cpu, 2);
        cycles += 4 + 17;
        mem_write(m, sp.wrapping_sub(2), 0xDE);
        mem_write(m, sp.wrapping_sub(1), 0x7F);
        cycles += scr_adr_next_char_line_hook(cpu, m);

        // ex af,af' ; dec a ; jr nz,loop
        std::mem::swap(&mut cpu.a, &mut cpu.a_);
        std::mem::swap(&mut cpu.f, &mut cpu.f_);
        cpu.a = cpu.dec8(cpu.a);
        bump_r(cpu, 3);
        cycles += 4 + 4;
        outer -= 1;
        if outer == 0 {
            cycles += 7; // jr nz не взят
            break;
        }
        cycles += 12; // jr nz взят
    }
    bump_r(cpu, 1); // ret
    cycles + 10
}
