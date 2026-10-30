//! Пересчёт угловых «скоростей» для индикаторов (`CalcAngSpeed4Prn`).

use super::*;

/// `CalcAngSpeed4Prn` (`0x713C`): пересчёт угловых «скоростей» для индикатора.
///
/// Читает старшие байты трёх 16-битных величин RAM (`0xFF09`, `0xFF0B`,
/// `0xFF0D` — старшие байты `AngleLeftRight`, `byte_FF0A`, `DeltaSpeed`) и
/// кладёт их в `HighAngleLR`/`HighAngleDU`/`HighDeltaSpeed`
/// (`0x70E4`..`0x70E6`), а рядом (`0x70FE`, `0x7100`, `0x70F4`) — производные
/// значения. Для первых двух байтов: если взведён бит 7 (величина
/// «отрицательная»), результат — `15h - (v & 7Fh)`, иначе `v + 15h`; в
/// отрицательной ветке промежуточное `v & 7Fh` остаётся в `C`. Третье значение
/// — `5 * v + 5` по модулю 256 (с сохранением `v` в `B`). Итоговые флаги — от
/// последнего `add a,5`; `HL` на выходе всегда `0xFF0D`.
///
/// ```text
/// 713C  ld hl,0FF09h        ; 10
/// 713F  ld a,(hl)           ;  7   7140  ld (70E4h),a  ; 13
/// 7143  bit 7,a             ;  8
/// 7145  jr nz,714Bh         ; 12/7
/// 7147  add a,15h           ;  7   7149  jr 7151h      ; 12   (бит 7 сброшен)
/// 714B  and 7Fh            ;  7   714D  ld c,a        ;  4
/// 714E  ld a,15h           ;  7   7150  sub c         ;  4
/// 7151  ld (70FEh),a        ; 13   ... то же для (0FF0Bh) -> 7100h
/// 716D  ld a,(hl)           ;  7   716E  ld (70E6h),a ; 13
/// 7171  ld b,a / add a,a ; add a,a ; add a,b ; add a,5
/// 7177  ld (70F4h),a        ; 13  717A  ret               ; 10
/// ```
pub(crate) fn calc_ang_speed_4_prn_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    /// Один «проход» рутины: чтение байта по `addr`, запись его как есть по
    /// `dump` и производной величины по `out`. Возвращает признак
    /// отрицательной ветки (бит 7 был взведён).
    fn pass(
        cpu: &mut Z80,
        m: &mut Spectrum,
        addr: usize,
        dump: usize,
        out: usize,
    ) -> bool {
        let v = m.mem[addr];
        m.mem[dump] = v;
        if v & 0x80 != 0 {
            let c = v & 0x7F;
            cpu.c = c; // `ld c,a` в отрицательной ветке
            m.mem[out] = 0x15u8.wrapping_sub(c);
            true
        } else {
            m.mem[out] = v.wrapping_add(0x15);
            false
        }
    }

    let neg1 = pass(cpu, m, 0xFF09, 0x70E4, 0x70FE);
    let neg2 = pass(cpu, m, 0xFF0B, 0x70E5, 0x7100);

    let v3 = m.mem[0xFF0D];
    m.mem[0x70E6] = v3;
    cpu.b = v3;
    cpu.h = 0xFF;
    cpu.l = 0x0D;

    // `add a,a ; add a,a ; add a,b ; add a,5` начиная с `a = v3`.
    let t = cpu.add8(v3, v3, 0);
    let t = cpu.add8(t, t, 0);
    let t = cpu.add8(t, v3, 0);
    let out3 = cpu.add8(t, 5, 0);
    cpu.a = out3;
    m.mem[0x70F4] = out3;

    // Инструкции: голова (5) + ветка (4/2) + середина (7) + ветка (4/2) +
    // хвост с `ret` (12).
    bump_r(cpu, 28 + if neg1 { 2 } else { 0 } + if neg2 { 2 } else { 0 });
    // Такты: 174 (голова+середина+хвост) + по 34 (ветка взята) или 26 (нет).
    174 + if neg1 { 34 } else { 26 } + if neg2 { 34 } else { 26 }
}
