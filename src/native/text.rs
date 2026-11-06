//! Текстовый вывод: выбор шрифта, печать символов и токенов, игровые
//! сообщения (`game_msg`).

use super::*;
use crate::machine::MessageTag;

/// Точное число тактов Z80, которое рутина `sub_5DFB` занимает для кода `a`
/// (включая завершающий `RET`).
pub(crate) fn font_base_cycles(a: u8) -> u64 {
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
///
/// Заодно перехватывает готовый текст нижнего сообщения: `A` здесь —
/// окончательный код символа (после регистра и раскрытия токенов), а
/// `CrdXPrnSymbol` — точная ячейка глифа (курсор уже сдвинут). Символ
/// складывается в 32-колоночный буфер строки; `msgNumber`/`msgTimeLife`
/// подсказывают перехвату, когда началась новая надпись. Готовые строки
/// уходят в `debug.log` и в очередь нативного вида полёта.
pub(crate) fn font_base_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    #[cfg(test)]
    FONT_BASE_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let a = cpu.a;
    let col = m
        .mem
        .get(KLADOV_CRD_X_PRN_SYMBOL)
        .copied()
        .unwrap_or(0);
    let row = m
        .mem
        .get(KLADOV_CRD_X_PRN_SYMBOL + 1)
        .copied()
        .unwrap_or(0);
    let tag = MessageTag {
        number: m.mem.get(KLADOV_MSG_NUMBER).copied().unwrap_or(0),
        life: m.mem.get(KLADOV_MSG_TIME_LIFE).copied().unwrap_or(0),
    };
    m.game_msg.capture_text_char_tagged(a, row, col, tag);
    let hl = font_base(a);
    cpu.h = (hl >> 8) as u8;
    cpu.l = (hl & 0xFF) as u8;
    bump_r(cpu, font_base_steps(a));
    font_base_cycles(a)
}

/// Число инструкций `sub_5DFB` по пути, выбираемому символом `A` (для
/// эмуляции `R`; пути и границы совпадают с `font_base_cycles`).
pub(crate) fn font_base_steps(a: u8) -> u32 {
    match a {
        0x20 => 3,
        _ if a < 0x30 => 7,
        _ if a < 0x3B => 7,
        _ if a < 0x41 => 11,
        _ if a < 0x5B => 11,
        _ if a < 0x61 => 15,
        _ if a < 0x7B => 15,
        _ => 17,
    }
}

/// Верхняя оценка тактов `print_message1_a_hook`: самый долгий путь — поиск
/// токена с индексом `0x91` (таблица `(word_F11A)` = `0xF11E`: 939 байт,
/// 145 проходов `cpir`) = 39 897 тактов. Оценка проверена перебором всех 256
/// значений `A` по фактической таблице образа; запас — на случай правок.
pub(crate) const PRINT_MSG1_MAX: u64 = 41_000;

/// Нативная замена `PrintMessage1_A` (`0xD425`) — диспетчер печати символа и
/// поиск строки токена в таблице `(word_F11A)`.
///
/// Рутина рекурсивна: `A` — код символа/токена. Коды `< 0x0E` — управляющие
/// (обработчик в `0xD7E2`), `0x0E..0x1F` и `>= 0xA0` — индексы токенов,
/// `0x20..0x5F` — обычные знаки (рисует `sub_D499`), `0x60..0x7F` — тоже
/// токены, `0x80..0x9F` — двухбайтовые диграфы (`0xD59C`). Строка токена
/// печатается посимвольно: `0xD454` читает байт, и пока он не ноль, рекурсивно
/// вызывает эту же рутину.
///
/// Здесь переносится только «горячая» часть — просмотр таблицы токенов
/// (`0xD443..0xD453`, `cpir` по строкам с нулевым терм-байтом), доводящая `HL`
/// до начала нужной строки. Все прочие ветви отдаются Z80 через `hook_yield`:
/// посимвольный вывод не дублируется (иначе управляющие коды внутри строк
/// потребовали бы восстанавливать стек рекурсии), а `sub_D499`/диграфы/
/// обработчики управляющих кодов оставлены на Z80 — их стековые «следы»
/// (стек игры лежит в экранной памяти, `SP=0x4116`) и зависимости слишком
/// ветвисты. Такты и `R` для пропущенных инструкций заряжаются вручную.
pub(crate) fn print_message1_a_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    let a = cpu.a;
    // Диспетчер `0xD425..0xD442`. Для терминальных ветвей заряжаем такты
    // инструкций до цели и отдаём управление; для токенов идём дальше.
    if a < 0x0E {
        // `cp 0Eh` (7) + взятый `jp c,0xD7E2` (10).
        bump_r(cpu, 2);
        cpu.pc = 0xD7E2;
        m.hook_yield = true;
        return 17;
    }
    if a < 0x20 {
        // `cp 0Eh` (7) + невзятый `jp c` (10) + `cp 20h` (7) + взятый
        // `jp c,0xD441` (10) + `add a,72h` (7).
        cpu.a = cpu.add8(a, 0x72, 0);
        token_scan(cpu, m, 41, 5)
    } else if a < 0x60 {
        // ... + `cp 60h` (7) + взятый `jp c,0xD499` (10).
        bump_r(cpu, 6);
        cpu.pc = 0xD499;
        m.hook_yield = true;
        51
    } else if a < 0x80 {
        // ... + `cp 80h` (7) + взятый `jr c,0xD443` (12).
        token_scan(cpu, m, 70, 8)
    } else if a < 0xA0 {
        // ... + невзятый `jr c` (7) + `cp 0A0h` (7) + взятый `jp c,0xD59C` (10).
        bump_r(cpu, 10);
        cpu.pc = 0xD59C;
        m.hook_yield = true;
        82
    } else {
        // ... + невзятый `jp c` (10) + `sub 0A0h` (7) + `jr 0xD443` (12).
        cpu.a = cpu.sub8(a, 0xA0, 0);
        token_scan(cpu, m, 101, 12)
    }
}

/// Хвост `PrintMessage1_A` с `0xD443`: `ld hl,(word_F11A)` (16) + `ld d,a` (4)
/// + `inc d` (4), затем цикл `dec d / jp z,0xD454 / ld bc,0FFFFh / xor a /
/// cpir / jr 0xD448`, отыскивающий `A`-ю по счёту строку токена. `pre_cycles`/
/// `pre_steps` — такты и число инструкций диспетчера до этой точки.
///
/// Такты идут по правилам этого эмулятора: каждая итерация `cpir` — отдельный
/// `step` (её `PC=0xD450` повторяется), повтор стоит `16+21`, завершающая —
/// `16+16`. Хук доводит `HL` до строки и отдаёт управление Z80 на `0xD454`.
pub(crate) fn token_scan(cpu: &mut Z80, m: &mut Spectrum, pre_cycles: u64, pre_steps: u32) -> u64 {
    let mut cycles = pre_cycles + 16 + 4 + 4;
    let mut steps = pre_steps + 3;
    let mut hl = m.mem[0xF11A] as u16 | ((m.mem[0xF11B] as u16) << 8);
    let mut bc_final = cpu.bc(); // при нулевом индексе `BC` не трогается
    let mut d = cpu.a.wrapping_add(1);
    loop {
        d = d.wrapping_sub(1); // `dec d`
        cycles += 4;
        steps += 1;
        cycles += 10; // `jp z,0xD454` (10 тактов независимо от исхода)
        steps += 1;
        if d == 0 {
            break;
        }
        cycles += 10 + 4; // `ld bc,0FFFFh` + `xor a`
        steps += 2;
        // `cpir`: ищем байт `0` от `HL`, `BC` — счётчик.
        let mut bc = 0xFFFFu32;
        loop {
            let byte = m.mem[hl as usize];
            hl = hl.wrapping_add(1);
            bc -= 1;
            steps += 1;
            if byte == 0 || bc == 0 {
                cycles += 16 + 16; // завершающая итерация
                break;
            }
            cycles += 16 + 21; // повтор
        }
        bc_final = bc as u16;
        cycles += 12; // `jr 0xD448`
        steps += 1;
    }
    bump_r(cpu, steps);
    cpu.b = (bc_final >> 8) as u8;
    cpu.c = bc_final as u8;
    cpu.d = 0;
    cpu.h = (hl >> 8) as u8;
    cpu.l = hl as u8;
    cpu.pc = 0xD454;
    m.hook_yield = true;
    cycles
}

/// `CrdXPrnSymbol` в версии Кладова (`0xD045`) — печатный курсор игры:
/// колонка (0..31), строка хода. У оригинала адрес другой (`0xA80D`).
pub(crate) const KLADOV_CRD_X_PRN_SYMBOL: usize = 0xD045;

/// `msgTimeLife` (`0xDA73`) — сколько тактов ещё печатать текущее сообщение.
/// `sub_DAB4` уменьшает счётчик перед печатью; когда приходит новое
/// сообщение, он снова взводится вверх. По этому признаку перехват отличает
/// новую надпись от повторной печати старой.
pub(crate) const KLADOV_MSG_TIME_LIFE: usize = 0xDA73;

/// `msgNumber` (`0xDA76`) — номер/токен текущего нижнего сообщения.
pub(crate) const KLADOV_MSG_NUMBER: usize = 0xDA76;

/// Перехват `sub_D499` (0xD499) — печать одного символа на экран ZX.
///
/// Через `sub_D499` проходят **все** печатаемые символы всех сообщений
/// игры (диспетчер `PrintMessage1_A` → литералы здесь, диграфы тоже
/// через неё, `PrintMessage2` → `sub_D470` → сюда). Хук только подглядывает:
/// пока [`Spectrum::game_msg`] включён, он записывает символ и позицию
/// печати (`CrdXPrnSymbol` до продвижения курсора), после чего аккуратно
/// отдаёт управление Z80 на второй инструкции рутины. Первая инструкция
/// `cp 0x41h` заряжается флагами и тактами вручную, поэтому состояние
/// машины (регистры, память, `R`, число тактов) не отличается от прогона
/// без хука — `hooks_match_plain_run` это проверяет.
pub(crate) fn print_char_tap_hook(cpu: &mut Z80, m: &mut Spectrum) -> u64 {
    if m.game_msg.enabled {
        let col = m.mem.get(KLADOV_CRD_X_PRN_SYMBOL).copied().unwrap_or(0);
        let row = m.mem.get(KLADOV_CRD_X_PRN_SYMBOL + 1).copied().unwrap_or(0);
        m.game_msg.push_char(cpu.a, row, col);
    }
    // `cp 0x41h`: только флаги, `A` не меняется (как `sub8(a, v, 0)` без
    // сохранения результата).
    let _ = cpu.sub8(cpu.a, 0x41, 0);
    bump_r(cpu, 1);
    cpu.pc = 0xD49B;
    m.hook_yield = true;
    7 // `cp n` в этой машине — 7 тактов
}

/// Счётчик обращений к хуку (только для тестов).
#[cfg(test)]
pub(crate) static FONT_BASE_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

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
