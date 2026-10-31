//! Перехват игровой печати (`sub_D499`) для нативного вида полёта.

use super::*;

/// Перехват игровой печати (`sub_D499`, 0xD499).
///
/// Игра печатает сообщения посимвольно на ZX-экран: «Front View» и прочие
/// подписи, строки гиперпрыжка, числа отсчёта. Пока [`GameMsgTap::enabled`]
/// взведён, хук печати копит текст строки (вместе с текстовой строкой ZX,
/// в которую он печатается) — нативный вид полёта показывает его на своём
/// HD-слое своим шрифтом. Сам эмулятор не меняется: хук только читает
/// `A` и `CrdXPrnSymbol`.
#[derive(Clone, Debug, Default)]
pub struct GameMsgTap {
    /// Копить ли текст (нативный вид полёта открыт). Переход `false → true`
    /// сбрасывает строку, чтобы на новом открытии вида не висела старая.
    pub enabled: bool,
    /// Текущая строка сообщения (только печатные символы). Меняется, когда
    /// игра печатает следующий символ на ту же строку; смена позиции печати
    /// начинает новую строку.
    pub text: String,
    /// Номер текстовой строки ZX (0..=23), в которую печатается строка.
    pub row: u8,
    /// Колонка (0..=31) первого символа строки.
    pub col: u8,
    /// Возраст строки в кадрах: 0 — символы печатались в текущем кадре.
    pub frames: u32,
}

impl GameMsgTap {
    /// Сбросить строку (при включении перехвата).
    pub fn clear(&mut self) {
        self.text.clear();
        self.row = 0;
        self.col = 0;
        self.frames = 0;
    }

    /// Добавить напечатанный символ. `row`/`col` — позиция следующего
    /// символа на экране ZX (`CrdXPrnSymbol` до его продвижения).
    pub fn push_char(&mut self, c: u8, row: u8, col: u8) {
        if !self.enabled || !(0x20..=0x7E).contains(&c) {
            return;
        }
        self.frames = 0;
        // Продолжение той же строки: та же строка, колонка следует за хвостом.
        let continuing = self.row == row
            && self.text.len() <= usize::from(u8::MAX)
            && self.col.wrapping_add(self.text.len() as u8) == col;
        if !continuing {
            self.text.clear();
            self.row = row;
            self.col = col;
        }
        self.text.push(c as char);
    }

    /// Старение строки: вызывается раз в кадр после прогона эмулятора (при
    /// выключенном перехвате возраст не растёт).
    pub fn next_frame(&mut self) {
        if self.enabled {
            self.frames = self.frames.saturating_add(1);
        }
    }
}
