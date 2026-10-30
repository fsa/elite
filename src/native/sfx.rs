//! Звуковой автомат (`sfx_update`, 0x9579): воспроизведение звуковых
//! эффектов AY по битовой маске `nBeepFX`.

use super::*;

/// `byte_9522` — состояние звукового автомата: 1/2/3 выбирают обработчик
/// активного эффекта, 0 — «свободен» (запуск нового по `nBeepFX`).
pub(crate) const SFX_STATE: u16 = 0x9522;
/// `word_9523` — счётчик `DE` активного эффекта (шаг/остаток).
pub(crate) const SFX_DE: u16 = 0x9523;
/// `word_9525` — указатель `HL` на данные активного эффекта.
pub(crate) const SFX_HL: u16 = 0x9525;
/// `byte_96BF` — номер канала, декодированный из `nBeepFX`.
pub(crate) const SFX_CHAN: u16 = 0x96BF;
/// `nBeepFX` — битовая маска запрошенных звуковых эффектов.
pub(crate) const SFX_REQUEST: u16 = 0x7128;
/// `FlagsSetUp` — бит 7 разрешает звук.
pub(crate) const SFX_FLAGS: u16 = 0xFC0E;
/// Адрес инструкции, которую игра пишет себе сама (`ld (95F2h),hl`):
/// `3FCBh` восстанавливает `srl a` (`CB 3F`), `10C6h` даёт `add a,10h`.
pub(crate) const SFX_PATCH: u16 = 0x95F2;

/// Верхняя оценка тактов, которые хук тратит до следующей проверки остатка
/// кадра: самая дорогая инструкция рутины — `ld (nn),de` (20 тактов).
pub(crate) const SFX_MAX_STEP: u64 = 20;

/// Читает 16-битное слово (little-endian) так же, как это делает Z80.
pub(crate) fn mem_read16(m: &Spectrum, addr: u16) -> u16 {
    m.mem[addr as usize] as u16 | ((m.mem[addr.wrapping_add(1) as usize] as u16) << 8)
}

/// Пишет 16-битное слово (little-endian) через шину (`mem_write`).
pub(crate) fn mem_write16(m: &mut Spectrum, addr: u16, val: u16) {
    mem_write(m, addr, val as u8);
    mem_write(m, addr.wrapping_add(1), (val >> 8) as u8);
}

/// `sub_9579` — покадровый автомат звуковых эффектов. Вызывается раз в кадр из
/// обработчика прерывания (`sub_7125` — это `jp 9579h`).
///
/// На входе состояние в `byte_9522`:
/// - `0` — эффекта нет: если `nBeepFX == 0`, выход; иначе по младшему
///   установленному биту `nBeepFX` выбирается обработчик из таблицы `jp`-ов
///   `0x9561` и запускает соответствующий эффект;
/// - `1` — эффект `0x9624` (пакет импульсов);
/// - `2` — эффект `0x95E7` (шум по данным `HL`);
/// - `3` — эффект `0x95A6` (тон с огибающей), продолжение.
///
/// Один вызов продвигает эффект на один «чанк» и возвращается по `ret nz`
/// (`0x95C7`, `0x960F`) либо `ret` (`0x95E6`, `0x9623`), когда `DE`/состояние
/// исчерпаны, поэтому вызов не обязан укладываться в кадр.
///
/// Рутина самомодифицирующаяся: она пишет себе операнды `ld c,n` (`0x95AE`),
/// `add a,n` (`0x95BD`, `0x962B`) и целую инструкцию по `0x95F2`, поэтому все
/// операнды хук читает из памяти. Обработчики таблицы (`0x9641`–`0x9699`) —
/// общий код: им пользуются и `sub_9579`, и стартовые рутины эффектов
/// (`sub_969B`/`sub_96A9`/`sub_96B7`), которые пока остаются на Z80; поэтому
/// хук обязан записывать операнды ровно так, как это делал бы Z80.
///
/// Кадровая граница: перед каждой инструкцией хук проверяет, влезает ли она в
/// остаток кадра, и при нехватке отдаёт управление Z80 ровно на этой
/// инструкции (`hook_yield`), оставив регистры и память как на Z80. Так
/// длинный чанк (до сотен тысяч тактов, больше кадра) переживает
/// vblank-прерывание ровно там же, где прогон без хука. Запись в порт `0xFE`
/// идёт через шину с тактом начала инструкции (`cycle_clock`), поэтому бипер
/// звучит идентично.
pub(crate) fn sfx_update_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    // Остаток кадра считаем от такта на входе; `frame_target` = u64::MAX вне
    // кадрового цикла (тесты) — тогда хук отрабатывает рутину целиком.
    let budget = m.frame_target.saturating_sub(cpu.cycles);
    let mut c = 0u64;
    // Выполнить одну инструкцию заменяемого кода. Не влезает в остаток
    // кадра — отдать её Z80, не меняя состояния.
    macro_rules! z80 {
        ($addr:expr, $cost:expr, $body:block) => {{
            if c + $cost > budget {
                cpu.pc = $addr;
                m.hook_yield = true;
                return c;
            }
            bump_r(cpu, 1);
            $body
            c += $cost;
        }};
    }
    // `out (0FEh),a`: бипер/бордюр; такт — начало инструкции, как это видит
    // `Spectrum::run_frame` (`cycle_clock = cpu.cycles` перед `step`).
    macro_rules! out_fe {
        () => {{
            m.set_cycle_clock(cpu.cycles + c);
            Bus::output(m, ((cpu.a as u16) << 8) | 0xFE, cpu.a);
        }};
    }
    macro_rules! djnz {
        ($addr:expr, $target:expr, $next:expr, $pc:ident) => {{
            let b = cpu.b.wrapping_sub(1);
            let taken = b != 0;
            z80!($addr, if taken { 13 } else { 8 }, { cpu.b = b; });
            $pc = if taken { $target } else { $next };
        }};
    }
    let mut pc = 0x9579u16;
    loop {
        match pc {
            // `ld a,(FlagsSetUp) ; bit 7,a ; ret z`
            0x9579 => {
                z80!(0x9579, 13, { cpu.a = m.mem[SFX_FLAGS as usize]; });
                pc = 0x957C;
            }
            0x957C => {
                z80!(0x957C, 8, { cpu.bit(7, cpu.a); });
                pc = 0x957E;
            }
            0x957E => {
                let taken = cpu.f & ZF != 0;
                z80!(0x957E, if taken { 11 } else { 5 }, {});
                if taken {
                    return c; // RET выполнит диспетчер хуков
                }
                pc = 0x957F;
            }
            // `ld a,(byte_9522) ; and a ; jr nz,959Fh`
            0x957F => {
                z80!(0x957F, 13, { cpu.a = m.mem[SFX_STATE as usize]; });
                pc = 0x9582;
            }
            0x9582 => {
                z80!(0x9582, 4, { cpu.a = cpu.and8(cpu.a, cpu.a); });
                pc = 0x9583;
            }
            0x9583 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x9583, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x959F } else { 0x9585 };
            }
            // Запуск нового эффекта: `ld a,(nBeepFX) ; and a ; ret z`; далее
            // B = номер младшего установленного бита, переход по таблице.
            0x9585 => {
                z80!(0x9585, 13, { cpu.a = m.mem[SFX_REQUEST as usize]; });
                pc = 0x9588;
            }
            0x9588 => {
                z80!(0x9588, 4, { cpu.a = cpu.and8(cpu.a, cpu.a); });
                pc = 0x9589;
            }
            0x9589 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x9589, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x958A;
            }
            0x958A => {
                z80!(0x958A, 7, { cpu.b = 0xFF; });
                pc = 0x958C;
            }
            0x958C => {
                z80!(0x958C, 4, { cpu.b = cpu.inc8(cpu.b); });
                pc = 0x958D;
            }
            0x958D => {
                z80!(0x958D, 4, { cpu.rra(); });
                pc = 0x958E;
            }
            0x958E => {
                let taken = cpu.f & CF == 0;
                z80!(0x958E, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x958C } else { 0x9590 };
            }
            0x9590 => {
                z80!(0x9590, 4, { cpu.a = cpu.b; });
                pc = 0x9591;
            }
            0x9591 => {
                z80!(0x9591, 13, { mem_write(m, SFX_CHAN, cpu.a); });
                pc = 0x9594;
            }
            0x9594 => {
                z80!(0x9594, 4, { cpu.a = cpu.add8(cpu.a, cpu.a, 0); });
                pc = 0x9595;
            }
            0x9595 => {
                z80!(0x9595, 4, { cpu.a = cpu.add8(cpu.a, cpu.b, 0); });
                pc = 0x9596;
            }
            0x9596 => {
                z80!(0x9596, 10, { cpu.h = 0x95; cpu.l = 0x61; });
                pc = 0x9599;
            }
            0x9599 => {
                z80!(0x9599, 4, { cpu.a = cpu.add8(cpu.a, cpu.l, 0); });
                pc = 0x959A;
            }
            0x959A => {
                z80!(0x959A, 4, { cpu.l = cpu.a; });
                pc = 0x959B;
            }
            0x959B => {
                let taken = cpu.f & CF == 0;
                z80!(0x959B, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x959E } else { 0x959D };
            }
            0x959D => {
                z80!(0x959D, 4, { cpu.h = cpu.inc8(cpu.h); });
                pc = 0x959E;
            }
            0x959E => {
                z80!(0x959E, 4, {});
                pc = cpu.hl(); // jp (hl)
            }
            // Таблица `jp nn` (0x9561..0x9577): цель читаем из образа.
            _ if (0x9561..=0x9577).contains(&pc) => {
                let target = mem_read16(m, pc + 1);
                z80!(pc, 10, {});
                pc = target;
            }
            // `dec a ; jp z,9624h ; dec a ; jr z,95E7h` — выбор по byte_9522.
            0x959F => {
                z80!(0x959F, 4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0x95A0;
            }
            0x95A0 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95A0, 10, {});
                pc = if taken { 0x9624 } else { 0x95A3 };
            }
            0x95A3 => {
                z80!(0x95A3, 4, { cpu.a = cpu.dec8(cpu.a); });
                pc = 0x95A4;
            }
            0x95A4 => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95A4, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95E7 } else { 0x95A6 };
            }
            // Хвост эффекта 3 (тон с огибающей), 0x95A6..0x95C7.
            0x95A6 => {
                z80!(0x95A6, 16, {
                    let v = mem_read16(m, SFX_HL);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95AA;
            }
            0x95AA => {
                z80!(0x95AA, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x95AD;
            }
            0x95AD => {
                z80!(0x95AD, 7, { cpu.c = m.mem[0x95AE]; });
                pc = 0x95AF;
            }
            0x95AF => {
                z80!(0x95AF, 7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0x95B0;
            }
            0x95B0 => {
                z80!(0x95B0, 7, { cpu.a = cpu.and8(cpu.a, 0x18); });
                pc = 0x95B2;
            }
            0x95B2 => {
                z80!(0x95B2, 11, { out_fe!(); });
                pc = 0x95B4;
            }
            0x95B4 => {
                z80!(0x95B4, 4, { cpu.b = cpu.d; });
                pc = 0x95B5;
            }
            0x95B5 => djnz!(0x95B5, 0x95B5, 0x95B7, pc),
            0x95B7 => {
                z80!(0x95B7, 6, {
                    let v = cpu.hl().wrapping_add(1);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95B8;
            }
            0x95B8 => {
                z80!(0x95B8, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x95B9;
            }
            0x95B9 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x95B9, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95AF } else { 0x95BB };
            }
            0x95BB => {
                z80!(0x95BB, 4, { cpu.a = cpu.d; });
                pc = 0x95BC;
            }
            0x95BC => {
                let imm = m.mem[0x95BD];
                z80!(0x95BC, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                pc = 0x95BE;
            }
            0x95BE => {
                z80!(0x95BE, 4, { cpu.d = cpu.a; });
                pc = 0x95BF;
            }
            0x95BF => {
                z80!(0x95BF, 4, { cpu.e = cpu.dec8(cpu.e); });
                pc = 0x95C0;
            }
            0x95C0 => {
                z80!(0x95C0, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x95C4;
            }
            0x95C4 => {
                z80!(0x95C4, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x95C7;
            }
            0x95C7 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x95C7, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x95C8;
            }
            0x95C8 => {
                z80!(0x95C8, 13, { cpu.a = m.mem[SFX_STATE as usize]; });
                pc = 0x95CB;
            }
            0x95CB => {
                z80!(0x95CB, 8, { cpu.bit(7, cpu.a); });
                pc = 0x95CD;
            }
            0x95CD => {
                let taken = cpu.f & ZF != 0;
                z80!(0x95CD, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x9610 } else { 0x95CF };
            }
            // Перезапуск эффекта 3 (byte_9522 = 3, C = 0A0h).
            0x95CF => {
                z80!(0x95CF, 7, { cpu.a = 3; });
                pc = 0x95D1;
            }
            0x95D1 => {
                z80!(0x95D1, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x95D4;
            }
            0x95D4 => {
                z80!(0x95D4, 7, { cpu.a = 0xA0; });
                pc = 0x95D6;
            }
            0x95D6 => {
                z80!(0x95D6, 13, { mem_write(m, 0x95AE, cpu.a); });
                pc = 0x95D9;
            }
            0x95D9 => {
                z80!(0x95D9, 10, { cpu.h = 0x01; cpu.l = 0x27; });
                pc = 0x95DC;
            }
            0x95DC => {
                z80!(0x95DC, 10, { cpu.d = 0x40; cpu.e = 0x02; });
                pc = 0x95DF;
            }
            0x95DF => {
                z80!(0x95DF, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x95E3;
            }
            0x95E3 => {
                z80!(0x95E3, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x95E6;
            }
            0x95E6 => {
                z80!(0x95E6, 10, {});
                return c;
            }
            // Эффект 2 (шум по данным HL), 0x95E7..0x9607.
            0x95E7 => {
                z80!(0x95E7, 16, {
                    let v = mem_read16(m, SFX_HL);
                    cpu.h = (v >> 8) as u8;
                    cpu.l = v as u8;
                });
                pc = 0x95EA;
            }
            0x95EA => {
                z80!(0x95EA, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x95EE;
            }
            0x95EE => {
                z80!(0x95EE, 7, { cpu.c = 0x64; });
                pc = 0x95F0;
            }
            0x95F0 => {
                z80!(0x95F0, 7, { cpu.a = m.mem[cpu.hl() as usize]; });
                pc = 0x95F1;
            }
            0x95F1 => {
                z80!(0x95F1, 4, { cpu.l = cpu.inc8(cpu.l); });
                pc = 0x95F2;
            }
            0x95F2 => {
                // Инструкцию по этому адресу игра пишет себе сама
                // (`ld (95F2h),hl`): 3FCBh -> `srl a`, 10C6h -> `add a,10h`.
                match (m.mem[SFX_PATCH as usize], m.mem[SFX_PATCH as usize + 1]) {
                    (0xCB, 0x3F) => {
                        z80!(0x95F2, 8, { cpu.a = cpu.srl(cpu.a); });
                        pc = 0x95F4;
                    }
                    (0xC6, imm) => {
                        z80!(0x95F2, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                        pc = 0x95F4;
                    }
                    // В игре такого не бывает; отдаём инструкцию Z80, чтобы
                    // поведение осталось верным при любом содержимом.
                    _ => {
                        cpu.pc = 0x95F2;
                        m.hook_yield = true;
                        return c;
                    }
                }
            }
            0x95F4 => {
                z80!(0x95F4, 8, { cpu.a = cpu.srl(cpu.a); });
                pc = 0x95F6;
            }
            0x95F6 => {
                z80!(0x95F6, 4, { cpu.e = cpu.a; });
                pc = 0x95F7;
            }
            0x95F7 => {
                z80!(0x95F7, 4, { cpu.b = cpu.a; });
                pc = 0x95F8;
            }
            0x95F8 => {
                z80!(0x95F8, 7, { cpu.a = 0x30; });
                pc = 0x95FA;
            }
            0x95FA => {
                z80!(0x95FA, 11, { out_fe!(); });
                pc = 0x95FC;
            }
            0x95FC => djnz!(0x95FC, 0x95FC, 0x95FE, pc),
            0x95FE => {
                z80!(0x95FE, 4, { cpu.b = cpu.e; });
                pc = 0x95FF;
            }
            0x95FF => {
                z80!(0x95FF, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9600;
            }
            0x9600 => {
                z80!(0x9600, 11, { out_fe!(); });
                pc = 0x9602;
            }
            0x9602 => djnz!(0x9602, 0x9602, 0x9604, pc),
            0x9604 => {
                z80!(0x9604, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x9605;
            }
            0x9605 => {
                let taken = cpu.f & ZF == 0;
                z80!(0x9605, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x95F0 } else { 0x9607 };
            }
            // Эффект 1 (пакет импульсов), 0x9624..0x963F.
            0x9624 => {
                z80!(0x9624, 20, {
                    let v = mem_read16(m, SFX_DE);
                    cpu.d = (v >> 8) as u8;
                    cpu.e = v as u8;
                });
                pc = 0x9628;
            }
            0x9628 => {
                z80!(0x9628, 4, { cpu.a = cpu.e; });
                pc = 0x9629;
            }
            0x9629 => {
                z80!(0x9629, 4, { cpu.a = cpu.add8(cpu.a, cpu.a, 0); });
                pc = 0x962A;
            }
            0x962A => {
                let imm = m.mem[0x962B];
                z80!(0x962A, 7, { cpu.a = cpu.add8(cpu.a, imm, 0); });
                pc = 0x962C;
            }
            0x962C => {
                z80!(0x962C, 4, { cpu.e = cpu.a; });
                pc = 0x962D;
            }
            0x962D => {
                z80!(0x962D, 7, { cpu.c = 0x0F; });
                pc = 0x962F;
            }
            0x962F => {
                z80!(0x962F, 7, { cpu.a = 0x30; });
                pc = 0x9631;
            }
            0x9631 => {
                z80!(0x9631, 4, { cpu.b = cpu.e; });
                pc = 0x9632;
            }
            0x9632 => {
                z80!(0x9632, 11, { out_fe!(); });
                pc = 0x9634;
            }
            0x9634 => djnz!(0x9634, 0x9634, 0x9636, pc),
            0x9636 => {
                z80!(0x9636, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9637;
            }
            0x9637 => {
                z80!(0x9637, 4, { cpu.b = cpu.e; });
                pc = 0x9638;
            }
            0x9638 => {
                z80!(0x9638, 11, { out_fe!(); });
                pc = 0x963A;
            }
            0x963A => djnz!(0x963A, 0x963A, 0x963C, pc),
            0x963C => {
                z80!(0x963C, 4, { cpu.c = cpu.dec8(cpu.c); });
                pc = 0x963D;
            }
            0x963D => {
                let taken = cpu.f & ZF == 0;
                z80!(0x963D, if taken { 12 } else { 7 }, {});
                pc = if taken { 0x962F } else { 0x963F };
            }
            0x963F => {
                z80!(0x963F, 12, {});
                pc = 0x9607;
            }
            // Общий эпилог: `dec d ; ... ; ret nz` и сброс состояния.
            0x9607 => {
                z80!(0x9607, 4, { cpu.d = cpu.dec8(cpu.d); });
                pc = 0x9608;
            }
            0x9608 => {
                z80!(0x9608, 20, {
                    let de = cpu.de();
                    mem_write16(m, SFX_DE, de);
                });
                pc = 0x960C;
            }
            0x960C => {
                z80!(0x960C, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_HL, hl);
                });
                pc = 0x960F;
            }
            0x960F => {
                let taken = cpu.f & ZF == 0;
                z80!(0x960F, if taken { 11 } else { 5 }, {});
                if taken {
                    return c;
                }
                pc = 0x9610;
            }
            0x9610 => {
                z80!(0x9610, 4, { cpu.a = cpu.xor8(cpu.a, cpu.a); });
                pc = 0x9611;
            }
            0x9611 => {
                z80!(0x9611, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9614;
            }
            0x9614 => {
                z80!(0x9614, 13, { cpu.a = m.mem[SFX_CHAN as usize]; });
                pc = 0x9617;
            }
            0x9617 => {
                z80!(0x9617, 4, { cpu.a = cpu.inc8(cpu.a); });
                pc = 0x9618;
            }
            0x9618 => {
                z80!(0x9618, 4, { cpu.b = cpu.a; });
                pc = 0x9619;
            }
            0x9619 => {
                z80!(0x9619, 7, { cpu.a = 0x80; });
                pc = 0x961B;
            }
            0x961B => {
                z80!(0x961B, 4, { cpu.rlca(); });
                pc = 0x961C;
            }
            0x961C => djnz!(0x961C, 0x961B, 0x961E, pc),
            0x961E => {
                z80!(0x961E, 10, { cpu.h = 0x71; cpu.l = 0x28; });
                pc = 0x9621;
            }
            0x9621 => {
                z80!(0x9621, 7, { cpu.a = cpu.xor8(cpu.a, m.mem[cpu.hl() as usize]); });
                pc = 0x9622;
            }
            0x9622 => {
                let addr = cpu.hl();
                z80!(0x9622, 7, { mem_write(m, addr, cpu.a); });
                pc = 0x9623;
            }
            0x9623 => {
                z80!(0x9623, 10, {});
                return c;
            }
            // Обработчики таблицы запуска эффектов.
            0x9678 => {
                z80!(0x9678, 7, { cpu.c = 0xFE; });
                pc = 0x967A;
            }
            0x967A => {
                z80!(0x967A, 7, { cpu.a = 3; });
                pc = 0x967C;
            }
            0x967C => {
                z80!(0x967C, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x967F;
            }
            0x967F => {
                z80!(0x967F, 7, { cpu.b = 0x40; });
                pc = 0x9681;
            }
            0x9681 => {
                z80!(0x9681, 10, { cpu.d = 0x40; cpu.e = 0x08; });
                pc = 0x9684;
            }
            0x9684 => {
                z80!(0x9684, 10, { cpu.h = 0x01; cpu.l = 0x27; });
                pc = 0x9687;
            }
            0x9687 => {
                z80!(0x9687, 4, { cpu.a = cpu.c; });
                pc = 0x9688;
            }
            0x9688 => {
                z80!(0x9688, 13, { mem_write(m, 0x95BD, cpu.a); });
                pc = 0x968B;
            }
            0x968B => {
                z80!(0x968B, 4, { cpu.a = cpu.b; });
                pc = 0x968C;
            }
            0x968C => {
                z80!(0x968C, 13, { mem_write(m, 0x95AE, cpu.a); });
                pc = 0x968F;
            }
            0x968F => {
                z80!(0x968F, 10, {});
                pc = 0x95AD;
            }
            0x9674 => {
                z80!(0x9674, 7, { cpu.c = 0xE4; });
                pc = 0x9676;
            }
            0x9676 => {
                z80!(0x9676, 12, {});
                pc = 0x967A;
            }
            0x9692 => {
                z80!(0x9692, 7, { cpu.a = 0x83; });
                pc = 0x9694;
            }
            0x9694 => {
                z80!(0x9694, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9697;
            }
            0x9697 => {
                z80!(0x9697, 7, { cpu.c = 2; });
                pc = 0x9699;
            }
            0x9699 => {
                z80!(0x9699, 12, {});
                pc = 0x967F;
            }
            0x965E => {
                z80!(0x965E, 7, { cpu.a = 3; });
                pc = 0x9660;
            }
            0x9660 => {
                z80!(0x9660, 10, { cpu.h = 0x3F; cpu.l = 0xCB; });
                pc = 0x9663;
            }
            0x9657 => {
                z80!(0x9657, 7, { cpu.a = 0x1A; });
                pc = 0x9659;
            }
            0x9659 => {
                z80!(0x9659, 10, { cpu.h = 0x10; cpu.l = 0xC6; });
                pc = 0x965C;
            }
            0x965C => {
                z80!(0x965C, 12, {});
                pc = 0x9663;
            }
            0x9663 => {
                z80!(0x9663, 10, { cpu.d = 0x01; cpu.e = 0x00; });
                pc = 0x9666;
            }
            0x9666 => {
                z80!(0x9666, 16, {
                    let hl = cpu.hl();
                    mem_write16(m, SFX_PATCH, hl);
                });
                pc = 0x9669;
            }
            0x9669 => {
                z80!(0x9669, 4, { cpu.h = cpu.a; });
                pc = 0x966A;
            }
            0x966A => {
                z80!(0x966A, 7, { cpu.l = 0x1A; });
                pc = 0x966C;
            }
            0x966C => {
                z80!(0x966C, 7, { cpu.a = 2; });
                pc = 0x966E;
            }
            0x966E => {
                z80!(0x966E, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9671;
            }
            0x9671 => {
                z80!(0x9671, 10, {});
                pc = 0x95EE;
            }
            0x9641 => {
                z80!(0x9641, 7, { cpu.a = 0xFD; });
                pc = 0x9643;
            }
            0x9643 => {
                z80!(0x9643, 10, { cpu.d = 0x02; cpu.e = 0x00; });
                pc = 0x9646;
            }
            0x9646 => {
                z80!(0x9646, 12, {});
                pc = 0x964D;
            }
            0x9648 => {
                z80!(0x9648, 7, { cpu.a = 0xD3; });
                pc = 0x964A;
            }
            0x964A => {
                z80!(0x964A, 10, { cpu.d = 0x08; cpu.e = 0x00; });
                pc = 0x964D;
            }
            0x964D => {
                z80!(0x964D, 13, { mem_write(m, 0x962B, cpu.a); });
                pc = 0x9650;
            }
            0x9650 => {
                z80!(0x9650, 7, { cpu.a = 1; });
                pc = 0x9652;
            }
            0x9652 => {
                z80!(0x9652, 13, { mem_write(m, SFX_STATE, cpu.a); });
                pc = 0x9655;
            }
            0x9655 => {
                z80!(0x9655, 12, {});
                pc = 0x9628;
            }
            _ => unreachable!("sfx_update: неизвестный адрес 0x{pc:04X}"),
        }
    }
}
