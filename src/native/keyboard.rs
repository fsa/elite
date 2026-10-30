//! Опрос клавиатуры: `ScanKeys` (0xFD69) и `sub_FCBA`.

use super::*;

/// Верхняя оценка тактов `ScanKeys` со всеми клавишами нажатыми.
///
/// Считается как сумма худших стоимостей узлов: вход (51) + 8 строк × 250
/// (строка со всеми5 нажатыми клавишами: 41 на чтение порта + 5×37 на
/// `rra`/`call c`/`dec e`/`jr nz` + 10 на `jp FC10` + 14 на `dec d`/`jp nz`)
/// + 40 вызовов `sub_FCBA` по 235 (наихудший путь с `jr nc`, взятым за
/// перенос) + хвост `FC10` по 537 (наихудший режим `FlagsSetUp&3 = 1|3`;
/// режим 0 короче — пропускает `FC3D`/`FC57`). Итог 11988; берём 12062 —
/// запас от суммы независимых худших, в кадр (69888) всё равно влезает.
pub(crate) const SCAN_KEYS_MAX: u64 = 12062;

/// `ScanKeys` (`0xFD69`) — покадровый опрос клавиатуры, вызывается из
/// обработчика прерывания `sub_FF25`.
///
/// Разбор по образу `zx_bin/kladov.bin` (листинг `kladov.asm` в этом районе
/// смещён: `call ScanKeys` в `sub_FF25` указывает на `0xFD69`, а не `0xFD72`):
///
/// - зануляет `byte_FC0F` (`0xFC0F`) и ставит `PressKeyPrnSymbol` (`0xFC0D`)
///   в `FFh`;
/// - читает 8 полуклавиатур (`BC` = `7FFEh`, `rlc b` перебирает `FE,FD,FB,F7,
///   EF,DF,BF,7F`), из 5 бит каждой строки `rra` выдаёт по одной клавише;
/// - каждая нажатая клавиша обрабатывается `sub_FCBA` (`0xFCBA`): по таблице
///   `unk_FD41` (`0xFD41`) — ASCII-код клавиши в `PressKeyPrnSymbol`, по
///   таблице `unk_FF58` (`0xFF58`) — `(смещение, маска)` байта переменной
///   `ControlKey1`+смещение (`0xFC08..0xFC0F`), значение OR-ится в ячейку;
/// - хвост `loc_FC10` (`0xFC10`) после 8-й строки: режим `FlagsSetUp&3`
///   (0 — обычный полёт; 1/3 — блок `FC3D` перекодирует нажатые клавиши из
///   `byte_FC0F`; 2 — пропускает перекодировку), побитовые перестановки
///   `bit4/bit5` старшей тетрады `ControlKeyMove` (разворот осей), автосброс
///   паузы при движении (`FC97`), запись низшей тетрады в `ControlKeyMove`,
///   и выходы `ret z` по биту6 `ControlKey1` / биту7 `ControlKey2`.
///
/// Хук зеркалирует рутину инструкция в инструкцию: такты, флаги (в том числе
/// цепочка переносов `rra`), регистры `R` и стековые байты `push`/`call`
/// внутри `sub_FCBA` — сравниваются с Z80 побайтово по всей памяти.
/// Таблицы и код читаются/пишутся в `m.mem` как на Z80.
pub(crate) fn scan_keys_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let mut c = 0u64;
    macro_rules! ins {
        ($cost:expr, $body:block) => {{
            bump_r(cpu, 1);
            $body;
            c += $cost;
        }};
    }
    let mut pc = 0xFD69u16;
    loop {
        match pc {
            // xor a ; ld (byte_FC0F),a ; dec a ; ld (PressKeyPrnSymbol),a
            0xFD69 => {
                ins!(4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0xFD6A;
            }
            0xFD6A => {
                ins!(13, { m.mem[0xFC0F] = cpu.a; });
                pc = 0xFD6D;
            }
            0xFD6D => {
                ins!(4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0xFD6E;
            }
            0xFD6E => {
                ins!(13, { m.mem[0xFC0D] = cpu.a; });
                pc = 0xFD71;
            }
            // ld bc,7FFEh ; ld d,8
            0xFD71 => {
                ins!(10, {
                    cpu.b = 0x7F;
                    cpu.c = 0xFE;
                });
                pc = 0xFD74;
            }
            0xFD74 => {
                ins!(7, { cpu.d = 8; });
                pc = 0xFD76;
            }
            // Цикл строк: rlc b ; in a,(c) ; and 1Fh ; xor 1Fh ; ld e,5
            0xFD76 => {
                ins!(8, { cpu.b = cpu.rlc(cpu.b); });
                pc = 0xFD78;
            }
            0xFD78 => {
                ins!(12, {
                    // Флаги `in a,(c)` не читаются: следующий `and` их
                    // перезаписывает целиком (включая C от `rlc b`).
                    let port = ((cpu.b as u16) << 8) | cpu.c as u16;
                    cpu.a = Bus::input(m, port);
                });
                pc = 0xFD7A;
            }
            0xFD7A => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x1F); });
                pc = 0xFD7C;
            }
            0xFD7C => {
                ins!(7, { cpu.a = cpu.xor8(cpu.a, 0x1F); });
                pc = 0xFD7E;
            }
            0xFD7E => {
                ins!(7, { cpu.e = 5; });
                pc = 0xFD80;
            }
            // Цикл клавиш строки: rra ; call c,sub_FCBA ; dec e ; jr nz
            0xFD80 => {
                ins!(4, { cpu.rra(); });
                pc = 0xFD81;
            }
            0xFD81 => {
                let taken = cpu.f & CF != 0;
                ins!(if taken { 17 } else { 10 }, {});
                if taken {
                    // CALL 0xFC84: в стек возвращаемый адрес (как push PC).
                    let sp = cpu.sp as usize;
                    m.mem[sp.wrapping_sub(1)] = 0xFD;
                    m.mem[sp.wrapping_sub(2)] = 0x84;
                    cpu.sp = cpu.sp.wrapping_sub(2);
                    scan_keys_fcba(cpu, m, &mut c);
                    // RET из sub_FCBA снимает адрес возврата.
                    cpu.sp = cpu.sp.wrapping_add(2);
                }
                pc = 0xFD84;
            }
            0xFD84 => {
                ins!(4, { cpu.e = cpu.dec8(cpu.e); });
                pc = 0xFD85;
            }
            0xFD85 => {
                let taken = cpu.f & ZF == 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFD80 } else { 0xFD87 };
            }
            // jp loc_FC10
            0xFD87 => {
                ins!(10, {});
                pc = 0xFC10;
            }
            // dec d ; jp nz,loc_FD76
            0xFC10 => {
                ins!(4, { cpu.d = cpu.dec8(cpu.d); });
                pc = 0xFC11;
            }
            0xFC11 => {
                let taken = cpu.f & ZF == 0;
                ins!(10, {});
                pc = if taken { 0xFD76 } else { 0xFC14 };
            }
            // Хвост: D = ControlKeyMove&0F ; E = FlagsSetUp ; и 3
            0xFC14 => {
                ins!(13, { cpu.a = m.mem[0xFC09]; });
                pc = 0xFC17;
            }
            0xFC17 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x0F); });
                pc = 0xFC19;
            }
            0xFC19 => {
                ins!(4, { cpu.d = cpu.a; });
                pc = 0xFC1A;
            }
            0xFC1A => {
                ins!(13, { cpu.a = m.mem[0xFC0E]; });
                pc = 0xFC1D;
            }
            0xFC1D => {
                ins!(4, { cpu.e = cpu.a; });
                pc = 0xFC1E;
            }
            0xFC1E => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x03); });
                pc = 0xFC20;
            }
            0xFC20 => {
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFC63 } else { 0xFC22 };
            }
            // dec a ; jr loc_FC3D (режим 1/3)
            0xFC22 => {
                ins!(4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0xFC23;
            }
            0xFC23 => {
                ins!(12, {});
                pc = 0xFC3D;
            }
            // Перекодировка нажатых клавиш из byte_FC0F (режим 1/3).
            0xFC3D => {
                ins!(4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0xFC3E;
            }
            0xFC3E => {
                ins!(13, { cpu.a = m.mem[0xFC0F]; });
                pc = 0xFC41;
            }
            0xFC41 => {
                ins!(4, { cpu.rra(); });
                pc = 0xFC42;
            }
            0xFC42 => {
                ins!(4, { cpu.d = cpu.a; });
                pc = 0xFC43;
            }
            0xFC43 => {
                // Z — от `dec a` на 0xFC3D (режим 2), не от `rra`.
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFC57 } else { 0xFC45 };
            }
            0xFC45 => {
                ins!(7, { cpu.d = 0; });
                pc = 0xFC47;
            }
            0xFC47 => {
                ins!(8, { cpu.d = cpu.rl(cpu.d); });
                pc = 0xFC49;
            }
            0xFC49 => {
                ins!(4, { cpu.rra(); });
                pc = 0xFC4A;
            }
            0xFC4A => {
                ins!(4, { cpu.rra(); });
                pc = 0xFC4B;
            }
            0xFC4B => {
                ins!(8, { cpu.d = cpu.rl(cpu.d); });
                pc = 0xFC4D;
            }
            0xFC4D => {
                ins!(8, { cpu.d = cpu.sla(cpu.d); });
                pc = 0xFC4F;
            }
            0xFC4F => {
                ins!(4, { cpu.rra(); });
                pc = 0xFC50;
            }
            0xFC50 => {
                ins!(8, { cpu.d = cpu.rr(cpu.d); });
                pc = 0xFC52;
            }
            0xFC52 => {
                ins!(4, { cpu.rra(); });
                pc = 0xFC53;
            }
            0xFC53 => {
                ins!(8, { cpu.d = cpu.rl(cpu.d); });
                pc = 0xFC55;
            }
            0xFC55 => {
                ins!(8, { cpu.d = cpu.rl(cpu.d); });
                pc = 0xFC57;
            }
            // ld hl,ControlKey1 ; bit 6,(hl) ; jr z,loc_FC63
            0xFC57 => {
                ins!(10, {
                    cpu.h = 0xFC;
                    cpu.l = 0x08;
                });
                pc = 0xFC5A;
            }
            0xFC5A => {
                ins!(12, { cpu.bit(6, m.mem[cpu.hl() as usize]); });
                pc = 0xFC5C;
            }
            0xFC5C => {
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFC63 } else { 0xFC5E };
            }
            // ld hl,ControlKey3 ; set 0,(hl)
            0xFC5E => {
                ins!(10, {
                    cpu.h = 0xFC;
                    cpu.l = 0x0B;
                });
                pc = 0xFC61;
            }
            0xFC61 => {
                ins!(15, {
                    let i = cpu.hl() as usize;
                    m.mem[i] |= 1;
                });
                pc = 0xFC63;
            }
            // Разворот бит0/бит1 и бит2/бит3 направления по битам 4/5 E.
            0xFC63 => {
                ins!(8, { cpu.bit(4, cpu.e); });
                pc = 0xFC65;
            }
            0xFC65 => {
                // `jr z,loc_FC74`: бит4 E не задан -> прыжок.
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFC74 } else { 0xFC67 };
            }
            0xFC67 => {
                ins!(4, { cpu.a = cpu.d; });
                pc = 0xFC68;
            }
            0xFC68 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x05); });
                pc = 0xFC6A;
            }
            0xFC6A => {
                ins!(8, { cpu.a = cpu.sla(cpu.a); });
                pc = 0xFC6C;
            }
            0xFC6C => {
                ins!(4, { cpu.c = cpu.a; });
                pc = 0xFC6D;
            }
            0xFC6D => {
                ins!(4, { cpu.a = cpu.d; });
                pc = 0xFC6E;
            }
            0xFC6E => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x0A); });
                pc = 0xFC70;
            }
            0xFC70 => {
                ins!(8, { cpu.a = cpu.srl(cpu.a); });
                pc = 0xFC72;
            }
            0xFC72 => {
                ins!(4, { cpu.a = cpu.or8(cpu.a, cpu.c); });
                pc = 0xFC73;
            }
            0xFC73 => {
                ins!(4, { cpu.d = cpu.a; });
                pc = 0xFC74;
            }
            0xFC74 => {
                ins!(8, { cpu.bit(5, cpu.e); });
                pc = 0xFC76;
            }
            0xFC76 => {
                // `jr z,loc_FC89`: бит5 E не задан -> прыжок.
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFC89 } else { 0xFC78 };
            }
            0xFC78 => {
                ins!(4, { cpu.a = cpu.d; });
                pc = 0xFC79;
            }
            0xFC79 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x03); });
                pc = 0xFC7B;
            }
            0xFC7B => {
                ins!(4, { cpu.b = cpu.a; });
                pc = 0xFC7C;
            }
            0xFC7C => {
                ins!(4, { cpu.a = cpu.d; });
                pc = 0xFC7D;
            }
            0xFC7D => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x0C); });
                pc = 0xFC7F;
            }
            0xFC7F => {
                ins!(4, { cpu.c = cpu.a; });
                pc = 0xFC80;
            }
            0xFC80 => {
                ins!(8, { cpu.c = cpu.rr(cpu.c); });
                pc = 0xFC82;
            }
            0xFC82 => {
                ins!(8, { cpu.a = cpu.sla(cpu.a); });
                pc = 0xFC84;
            }
            0xFC84 => {
                ins!(4, { cpu.a = cpu.or8(cpu.a, cpu.c); });
                pc = 0xFC85;
            }
            0xFC85 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x0C); });
                pc = 0xFC87;
            }
            0xFC87 => {
                ins!(4, { cpu.a = cpu.or8(cpu.a, cpu.b); });
                pc = 0xFC88;
            }
            0xFC88 => {
                ins!(4, { cpu.d = cpu.a; });
                pc = 0xFC89;
            }
            // ld hl,ControlKeyMove ; и 3 ; режим 0 и есть движение —
            // автосброс бит паузы/бомбы.
            0xFC89 => {
                ins!(10, {
                    cpu.h = 0xFC;
                    cpu.l = 0x09;
                });
                pc = 0xFC8C;
            }
            0xFC8C => {
                ins!(13, { cpu.a = m.mem[0xFC0E]; });
                pc = 0xFC8F;
            }
            0xFC8F => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x03); });
                pc = 0xFC91;
            }
            0xFC91 => {
                // `jr nz,FCA1` (байты образа `20 0E`): режим != 0 — прыжок.
                let taken = cpu.f & ZF == 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFCA1 } else { 0xFC93 };
            }
            0xFC93 => {
                ins!(7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0xFC94;
            }
            0xFC94 => {
                ins!(4, { cpu.a = cpu.and8(cpu.a, cpu.a); });
                pc = 0xFC95;
            }
            0xFC95 => {
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 12 } else { 7 }, {});
                pc = if taken { 0xFCA1 } else { 0xFC97 };
            }
            0xFC97 => {
                ins!(4, { cpu.l = cpu.inc8(cpu.l); });
                pc = 0xFC98;
            }
            0xFC98 => {
                ins!(15, {
                    let i = cpu.hl() as usize;
                    m.mem[i] &= !(1 << 7);
                });
                pc = 0xFC9A;
            }
            0xFC9A => {
                ins!(4, { cpu.l = cpu.inc8(cpu.l); });
                pc = 0xFC9B;
            }
            0xFC9B => {
                ins!(15, {
                    let i = cpu.hl() as usize;
                    m.mem[i] &= !(1 << 2);
                });
                pc = 0xFC9D;
            }
            0xFC9D => {
                ins!(15, {
                    let i = cpu.hl() as usize;
                    m.mem[i] &= !(1 << 7);
                });
                pc = 0xFC9F;
            }
            0xFC9F => {
                ins!(4, { cpu.l = cpu.dec8(cpu.l); });
                pc = 0xFCA0;
            }
            0xFCA0 => {
                ins!(4, { cpu.l = cpu.dec8(cpu.l); });
                pc = 0xFCA1;
            }
            // ControlKeyMove = (старшая тетрада) | D ; далее выходы.
            0xFCA1 => {
                ins!(7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0xFCA2;
            }
            0xFCA2 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0xF0); });
                pc = 0xFCA4;
            }
            0xFCA4 => {
                ins!(4, { cpu.a = cpu.or8(cpu.a, cpu.d); });
                pc = 0xFCA5;
            }
            0xFCA5 => {
                ins!(7, {
                    let i = cpu.hl() as usize;
                    m.mem[i] = cpu.a;
                });
                pc = 0xFCA6;
            }
            // ld hl,ControlKey1 ; bit 6,(hl) ; ret z
            0xFCA6 => {
                ins!(10, {
                    cpu.h = 0xFC;
                    cpu.l = 0x08;
                });
                pc = 0xFCA9;
            }
            0xFCA9 => {
                ins!(12, { cpu.bit(6, m.mem[cpu.hl() as usize]); });
                pc = 0xFCAB;
            }
            0xFCAB => {
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 11 } else { 5 }, {});
                if taken {
                    return c; // RET выполнит диспетчер хуков
                }
                pc = 0xFCAC;
            }
            // set 6,(hl) ; ld a,(ControlKey2) ; and 80h ; ret z
            0xFCAC => {
                // CB B6 = RES 6,(HL)
                ins!(15, {
                    let i = cpu.hl() as usize;
                    m.mem[i] &= !(1 << 6);
                });
                pc = 0xFCAE;
            }
            0xFCAE => {
                ins!(13, { cpu.a = m.mem[0xFC0A]; });
                pc = 0xFCB1;
            }
            0xFCB1 => {
                ins!(7, { cpu.a = cpu.and8(cpu.a, 0x80); });
                pc = 0xFCB3;
            }
            0xFCB3 => {
                let taken = cpu.f & ZF != 0;
                ins!(if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0xFCB4;
            }
            // ld a,0FEh ; ld (PressKeyPrnSymbol),a ; ret
            0xFCB4 => {
                ins!(7, { cpu.a = 0xFE; });
                pc = 0xFCB6;
            }
            0xFCB6 => {
                ins!(13, { m.mem[0xFC0D] = cpu.a; });
                pc = 0xFCB9;
            }
            0xFCB9 => {
                ins!(10, {});
                return c;
            }
            _ => unreachable!("scan_keys_hook: неожиданный pc=0x{pc:04X}"),
        }
    }
}

/// Тело `sub_FCBA` (`0xFCBA`) — обработка одной нажатой клавиши (вызывается
/// из [`scan_keys_hook`] после `call c`; адрес возврата уже лежит в стеке).
///
/// Индекс клавиши `(5-E)*8 + D - 1` (0..39) — код в `unk_FD41` пишется в
/// `PressKeyPrnSymbol`, пара `(смещение, маска)` из `unk_FF58` OR-ится в
/// `ControlKey1 + смещение`. `push af`/`push bc` сохраняют входные `A/F` и
/// `B/C` — восстанавливаются на выходе, а их байты остаются в памяти стека
/// (сверяются тестом). Такты и `R` начисляются как на Z80; `c` пополняется
/// вызывающим хуком.
pub(crate) fn scan_keys_fcba(cpu: &mut Z80, m: &mut Spectrum, c: &mut u64) {
    macro_rules! ins {
        ($cost:expr, $body:block) => {{
            bump_r(cpu, 1);
            $body;
            *c += $cost;
        }};
    }
    // push bc / push af: байты по адресам SP-1..SP-4 (как у настоящего
    // CALL+push), SP не двигаем — снимет вызывающий хук после «ret».
    let sp = cpu.sp as usize;
    let (pa, pf, pb, pc0) = (cpu.a, cpu.f, cpu.b, cpu.c);
    ins!(11, {
        m.mem[sp.wrapping_sub(1)] = cpu.b;
        m.mem[sp.wrapping_sub(2)] = cpu.c;
    });
    ins!(11, {
        m.mem[sp.wrapping_sub(3)] = cpu.a;
        m.mem[sp.wrapping_sub(4)] = cpu.f;
    });
    // ld a,5 ; sub e ; sla a ×3 ; add a,d ; dec a — индекс клавиши 0..39
    ins!(7, { cpu.a = 5; });
    ins!(4, { cpu.a = cpu.sub8(cpu.a, cpu.e, 0); });
    ins!(8, { cpu.a = cpu.sla(cpu.a); });
    ins!(8, { cpu.a = cpu.sla(cpu.a); });
    ins!(8, { cpu.a = cpu.sla(cpu.a); });
    ins!(4, { cpu.a = cpu.add8(cpu.a, cpu.d, 0); });
    ins!(4, { cpu.a = cpu.dec8(cpu.a); });
    // ld b,a ; sla b — удвоенный индекс для таблицы unk_FF58
    ins!(4, { cpu.b = cpu.a; });
    ins!(8, { cpu.b = cpu.sla(cpu.b); });
    // ld hl,unk_FD41 ; add a,l ; jr nc / inc h ; ld l,a — код клавиши
    ins!(10, {
        cpu.h = 0xFD;
        cpu.l = 0x41;
    });
    ins!(4, { cpu.a = cpu.add8(cpu.a, cpu.l, 0); });
    let nc = cpu.f & CF == 0;
    ins!(if nc { 12 } else { 7 }, {});
    if !nc {
        ins!(4, { cpu.h = cpu.inc8(cpu.h); });
    }
    ins!(4, { cpu.l = cpu.a; });
    ins!(7, { cpu.a = m.mem[cpu.hl() as usize]; });
    ins!(13, { m.mem[0xFC0D] = cpu.a; }); // PressKeyPrnSymbol
    // ld hl,unk_FF58 ; ld a,l ; add a,b ; ld l,a — пара (смещение, маска)
    ins!(10, {
        cpu.h = 0xFF;
        cpu.l = 0x58;
    });
    ins!(4, { cpu.a = cpu.l; });
    ins!(4, { cpu.a = cpu.add8(cpu.a, cpu.b, 0); });
    ins!(4, { cpu.l = cpu.a; });
    ins!(7, { cpu.a = m.mem[cpu.hl() as usize]; }); // смещение
    ins!(6, { cpu.l = cpu.l.wrapping_add(1); }); // inc hl (без флагов)
    ins!(7, { cpu.b = m.mem[cpu.hl() as usize]; }); // маска
    // ld hl,ControlKey1 ; add a,l ; ld l,a ; or (hl) ; ld (hl),a
    ins!(10, {
        cpu.h = 0xFC;
        cpu.l = 0x08;
    });
    ins!(4, { cpu.a = cpu.add8(cpu.a, cpu.l, 0); });
    ins!(4, { cpu.l = cpu.a; });
    ins!(7, { cpu.a = m.mem[cpu.hl() as usize]; });
    ins!(4, { cpu.a = cpu.or8(cpu.a, cpu.b); });
    ins!(7, {
        let i = cpu.hl() as usize;
        m.mem[i] = cpu.a;
    });
    // pop af ; pop bc ; ret
    ins!(10, {
        cpu.a = pa;
        cpu.f = pf;
    });
    ins!(10, {
        cpu.b = pb;
        cpu.c = pc0;
    });
    ins!(10, {});
}
