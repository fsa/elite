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
//!
//! Полное разрешение: хук может рисовать в HD-слой `Spectrum::hd` (см.
//! [`display::Frame`](crate::display::Frame)) — поверхность в разрешении
//! области показа окна. Слой очищается перед каждым кадром, непрозрачные
//! пиксели (старший байт != 0) накладываются поверх масштабированного
//! ZX-кадра 256x192. Так нативные реализации рисования могут переходить
//! на полное разрешение постепенно, не дожидаясь замены всего Z80-кода.

pub(crate) use crate::machine::{Hook, Spectrum, ROM_SIZE};
pub(crate) use crate::z80::{Bus, Z80, ZF, CF, HF, PF, SF, XF, YF};

/// Установить все нативные замены (хуки) на машину.
pub fn install(m: &mut Spectrum) {
    install_hook(m, 0x5DFB, font_base_hook, 120);
    install_hook(m, 0xEB3E, eb3e_hook, 32);
    install_hook(m, 0xD3CE, d3ce_hook, 46);
    install_hook(m, 0xAB0B, mult_a_hib_c_hook, 86);
    install_hook(m, 0xAB34, mult_hl_b_c_hook, 131);
    install_hook(m, 0xAB1E, sub_ab1e_hook, 24 * 256 + 180);
    install_hook(m, 0xE621, pix_crd_2_scr_adr_hook, 120);
    install_hook(m, 0x7A59, smag_add_mem_hook, 148);
    install_hook(m, 0x7FE7, scr_adr_next_line_hook, 87);
    install_hook(m, 0x8445, scr_adr_next_char_line_hook, 45);
    install_hook(m, 0xD546, scr_xor_col_down_hook, 48 * 256 + 5);
    // Пересчёт угловых «скоростей» для индикатора — вызывается из главного
    // цикла игры (`GameMain`) по нескольку раз за кадр.
    install_hook(m, 0x713C, calc_ang_speed_4_prn_hook, 242);
    // Копия длиннее кадра, поэтому хук внутри себя отдаёт управление Z80
    // (Yield) на границе кадра; верхняя оценка — одна итерация цикла.
    install_hook(m, 0x7FCD, scr_copy_alt_lines_hook, COPY_ALT_MAX_ITER);
    // Звуковой автомат: чанк эффекта тоже длиннее кадра, хук отдаёт Z80
    // каждую инструкцию, что не влезает в остаток кадра (оценка — одна
    // инструкция рутины).
    install_hook(m, 0x9579, sfx_update_hook, SFX_MAX_STEP);
    // Опрос клавиатуры из обработчика прерывания — раз в кадр, всё влезает.
    install_hook(m, 0xFD69, scan_keys_hook, SCAN_KEYS_MAX);
    // Печать символа/токена: нативно — просмотр таблицы токенов, остальное
    // отдаётся Z80 (Yield). Верхняя оценка — самый длинный поиск токена.
    install_hook(m, 0xD425, print_message1_a_hook, PRINT_MSG1_MAX);
    // Отдельная точка перехвата игровых сообщений: `sub_D499` печатает один
    // символ на ZX-экран, через неё проходят все сообщения игры (подписи
    // вида, гиперпрыжок, отсчёт). Хук только копит текст, состояние машины
    // не меняет (см. `print_char_tap_hook`).
    install_hook(m, 0xD499, print_char_tap_hook, 8);
}

/// Включить перехват готового текста нижней строки: события получает нативный
/// вид полёта, копия пишется в `debug.log` рядом с исполняемым файлом. Нужен
/// только `font_base_hook` (`sub_5DFB`), поэтому работает и без остальных
/// нативных замен.
pub fn install_text_capture(m: &mut Spectrum) {
    if !m.hooks.contains_key(&0x5DFB) {
        install_hook(m, 0x5DFB, font_base_hook, 120);
    }
    m.game_msg.start_text_log();
}

/// Регистрирует хук вместе с верхней оценкой его тактов (см. `hook_max`).
fn install_hook(m: &mut Spectrum, addr: u16, run: Hook, max_cycles: u64) {
    m.hooks.insert(addr, run);
    m.hook_max.insert(addr, max_cycles);
}

/// Эмуляция обновления регистра `R` (Memory Refresh) за заменённые инструкции.
///
/// На Z80 `R` инкрементируется на единицу за каждую выполняемую инструкцию
/// (бит 7 сохраняется). Хук подменяет рутину целиком и выполняет ноль
/// инструкций, поэтому `R` нужно двигать самим — иначе расходится поток
/// случайных чисел: игра читает `R` через `ld a, r` в `PrintStars` (дрожание
/// звёзд), и любой пропущенный сдвиг в итоге меняет экран.
///
/// `steps` — число инструкций (не такт заменённого кода), `ED`/`CB`-префиксы
/// считаются за одну инструкцию, как в `Z80::step`.
#[inline]
pub(crate) fn bump_r(cpu: &mut Z80, steps: u32) {
    cpu.r = (cpu.r & 0x80) | (cpu.r.wrapping_add(steps as u8) & 0x7F);
}

#[cfg(test)]
mod tests;

mod indicators;
mod keyboard;
mod math;
mod save;
mod screen;
mod sfx;
mod text;

// Каждая подсистема экспортирует все свои элементы в модуль `native`,
// чтобы `install`, соседние подсистемы и тесты видели их без путей.
pub(crate) use indicators::*;
pub(crate) use keyboard::*;
pub(crate) use math::*;
pub(crate) use save::*;
pub(crate) use screen::*;
pub(crate) use sfx::*;
pub(crate) use text::*;

/// Пишет байт так же, как шина `Spectrum::write`: ниже `ROM_SIZE` запись
/// отбрасывается. Хуки обязаны пользоваться этой функцией (а не `m.mem`),
/// чтобы ПЗУ оставалось неизменным, как на настоящей шине.
pub(crate) fn mem_write(m: &mut Spectrum, addr: u16, val: u8) {
    if addr as usize >= ROM_SIZE {
        m.mem[addr as usize] = val;
        if let Some((lo, hi)) = m.wtrace {
            if addr >= lo && addr <= hi {
                eprintln!("H 0x{addr:04X} <- 0x{val:02X}");
            }
        }
    }
}
