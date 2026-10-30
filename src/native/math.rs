//! Арифметические вспомогательные рутины: умножения через таблицы,
//! сдвиг на переменное число бит, 16-битное сложение/вычитание.

use super::*;

/// `sub_EB3E`: `ld d,0; ld e,a; add hl,de; ret` — прибавляет к `HL`
/// беззнаковый `A` (через `DE`), выставляя флаги сложения. Флаги `S/Z/PV`
/// 16-битного `ADD HL,rr` не трогает.
pub(crate) fn eb3e_hook(cpu: &mut Z80, _m: &mut Spectrum) -> u64 {
    cpu.d = 0;
    cpu.e = cpu.a;
    let r = cpu.add16(cpu.hl(), cpu.de());
    cpu.h = (r >> 8) as u8;
    cpu.l = r as u8;
    bump_r(cpu, 4); // ld d,0; ld e,a; add hl,de; ret
    32 // ld d,0(7) + ld e,a(4) + add hl,de(11) + ret(10)
}

/// `sub_D3CE`: `push hl; sbc hl,de; pop hl; ret` — выставляет флаги по
/// `HL - DE - C` (16-битное вычитание с заёмом), но `HL` оставляет прежним.
pub(crate) fn d3ce_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let hl = cpu.hl();
    let de = cpu.de();
    cpu.sbc16(hl, de);
    // `push hl` кладёт HL под стек, `pop hl` читает его назад (SP не меняется),
    // поэтому в памяти остаётся след из двух байт HL. Стек игры живёт в
    // экранной памяти, так что след виден и должен совпадать с Z80.
    let sp = cpu.sp;
    m.mem[sp.wrapping_sub(1) as usize] = (hl >> 8) as u8;
    m.mem[sp.wrapping_sub(2) as usize] = hl as u8;
    bump_r(cpu, 4); // push hl; sbc hl,de; pop hl; ret
    46 // push hl(11) + sbc hl,de(15) + pop hl(10) + ret(10)
}

/// Таблица умножения (`0xBF00`), к которой обращается `Mult_A_hiB_C`.
pub(crate) const MULT_TABLE: usize = 0xBF00;

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
pub(crate) fn mult_a_hib_c_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
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
    bump_r(cpu, if no_borrow { 14 } else { 15 });
    if no_borrow { 83 } else { 86 }
}

/// Младшие байты 16-битной таблицы квадратов (`Mult_HL_B_C`, `0xBE00`).
///
/// Таблица лежит «двумя плоскостями»: младшие байты слов по `0xBE00+индекс`,
/// старшие — там же по `0xBF00+индекс` (это общая плоскость с однобайтовой
/// таблицей `Mult_A_hiB_C`). Слово с индексом `i` = `(mem[0xBF00+i] << 8) |
/// mem[0xBE00+i]`.
pub(crate) const MULT_LO_TABLE: usize = 0xBE00;

/// Нативная замена `Mult_HL_B_C` (`0xAB34`) — «приближенное табличное
/// умножение HL = B*C» (ядро 3D-математики, 11 точек вызова: `Sqr_HL_wordAdrIX`,
/// поворот/масштабирование кораблей).
///
/// ```text
/// ld a,b ; sub c ; jr nc,keep ; neg ; keep: srl a
/// ld h,0BEh ; ld l,a ; ld e,(hl) ; inc h ; ld d,(hl) ; dec h
/// ld a,b ; add a,c ; rra ; ld l,a ; ld a,(hl) ; inc h ; ld h,(hl) ; ld l,a
/// and a ; sbc hl,de ; ret
/// ```
///
/// Вход: `B`, `C`. Выход: `HL = word[(B+C)/2] - word[|B-C|/2]` (16-битное
/// приближение произведения), `DE = word[|B-C|/2]`, `A` = младший байт
/// слова-суммы (не меняется при `sbc`). Итоговые флаги задаёт `sbc hl,de`
/// с заёмом 0 (его очищает `and a`). `B`, `C` не изменяются.
pub(crate) fn mult_hl_b_c_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let (b, c) = (cpu.b, cpu.c);
    let no_borrow = b >= c; // результат `sub c`: CF=0
    let diff = if no_borrow { b - c } else { c - b };
    let di = (diff >> 1) as usize;
    let de = ((m.mem[MULT_TABLE + di] as u16) << 8) | m.mem[MULT_LO_TABLE + di] as u16;
    let idx = ((b as u16 + c as u16) >> 1) as u8; // `add a,c` + `rra`
    let lo = m.mem[MULT_LO_TABLE + idx as usize];
    let hi = m.mem[MULT_TABLE + idx as usize];
    cpu.d = (de >> 8) as u8;
    cpu.e = de as u8;
    cpu.a = lo; // `ld a,(hl)` — младший байт слова-суммы
    cpu.a = cpu.and8(cpu.a, cpu.a); // `and a` — очищает C перед sbc
    let r = cpu.sbc16(((hi as u16) << 8) | lo as u16, de);
    cpu.h = (r >> 8) as u8;
    cpu.l = r as u8;
    bump_r(cpu, if no_borrow { 21 } else { 22 });
    if no_borrow { 128 } else { 131 }
}

/// Адрес `VertexIncFactor` в версии Кладова (данные, `db 1`).
pub(crate) const VERTEX_INC_FACTOR: usize = 0xA9C7;

/// Нативная замена `sub_AB1E` — хелпер `RotateCrdN_Vertex`: сдвигает `B`
/// влево на `VertexIncFactor` бит и домножает на байт `(iy+1)` (без знака)
/// через ту же таблицу квадратов, что `Mult_A_hiB_C`.
///
/// ```text
/// ld a,(VertexIncFactor)
/// loop: sla b ; dec a ; jr nz,loop
/// ld c,(iy+1) ; res 7,c ; push de
/// call Mult_A_hiB_C ; ld l,a ; ld h,0 ; pop de ; ret
/// ```
///
/// Вход: `B`, `IY`. Выход: `B = B << VertexIncFactor` (за 8 бит — 0),
/// `C = (IY+1) & 0x7F`, `HL = A` (результат `Mult_A_hiB_C`, нулевой
/// расширенный), флаги — от `sub d` внутреннего умножения. `D`, `E`
/// сохраняются (`push`/`pop`). `VertexIncFactor = 0` даёт 256 итераций
/// сдвига (как `djnz` с нулём), результат — 0.
pub(crate) fn sub_ab1e_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let nv = m.mem[VERTEX_INC_FACTOR];
    let n = if nv == 0 { 256 } else { nv as u64 }; // итераций; 0 -> 256 (как djnz)
    let b = if n >= 8 { 0 } else { cpu.b << n as u32 };
    let c = m.mem[(cpu.iy as usize).wrapping_add(1) & 0xFFFF] & 0x7F;
    cpu.b = b;
    cpu.c = c;
    // Внутренний Mult_A_hiB_C (0xAB0B): таблица 0xBF00, вычет |b-c|/2 из (b+c)/2.
    let no_borrow = b >= c;
    let diff = if no_borrow { b - c } else { c - b };
    let d = m.mem[MULT_TABLE + (diff >> 1) as usize];
    let idx = ((b as u16 + c as u16) >> 1) as u8;
    let a2 = m.mem[MULT_TABLE + idx as usize];
    cpu.a = cpu.sub8(a2, d, 0); // флаги остаются от него (ld/pop/ret не трогают)
    cpu.h = 0;
    cpu.l = cpu.a;
    bump_r(cpu, 3 * n as u32 + 9 + if no_borrow { 14 } else { 15 });
    24 * n + 177 + if no_borrow { 0 } else { 3 }
}
