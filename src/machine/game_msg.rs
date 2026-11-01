//! Перехват игровой печати (`sub_D499`) для нативного вида полёта.

/// Одна строка, которую игра напечатала на ZX-экране.
#[derive(Clone, Debug, Default)]
pub struct GameMsgLine {
    /// Текст строки (только печатные символы).
    pub text: String,
    /// Номер текстовой строки ZX (0..=23).
    pub row: u8,
    /// Колонка (0..=31) первого символа строки.
    pub col: u8,
    /// Возраст строки в кадрах: 0 — символы печатались в текущем кадре.
    pub frames: u32,
}

/// Перехват игровой печати (`sub_D499`, 0xD499).
///
/// Игра печатает сообщения посимвольно на ZX-экран: «Front View» и прочие
/// подписи, строки гиперпрыжка, числа отсчёта. Пока [`GameMsgTap::enabled`]
/// взведён, хук печати копит текст строки (вместе с текстовой строкой ZX,
/// в которую он печатается) — нативный вид полёта показывает его на своём
/// HD-слое своим шрифтом. Сам эмулятор не меняется: хук только читает
/// `A` и `CrdXPrnSymbol`.
///
/// Кроме последней напечатанной строки (`text`/`row`/...), хук хранит
/// последнюю строку каждой ZX-строки в [`GameMsgTap::lines`]. Игра может
/// после сообщения гиперпрыжка напечатать подпись вида или индикатор, и
/// общая «текущая» строка уже не будет сообщением; история по строкам
/// сохраняет его.
#[derive(Clone, Debug, Default)]
pub struct GameMsgTap {
    /// Копить ли текст (нативный вид полёта открыт). Переход `false → true`
    /// сбрасывает строку, чтобы на новом открытии вида не висела старая.
    pub enabled: bool,
    /// Последняя напечатанная строка (только печатные символы). Меняется,
    /// когда игра печатает следующий символ на ту же строку; смена позиции
    /// печати начинает новую строку.
    pub text: String,
    /// Номер текстовой строки ZX (0..=23), в которую печатается строка.
    pub row: u8,
    /// Колонка (0..=31) первого символа строки.
    pub col: u8,
    /// Возраст последней строки в кадрах: 0 — символы печатались в текущем
    /// кадре.
    pub frames: u32,
    /// Последняя строка для каждой ZX-строки, в которую печатала игра.
    pub lines: Vec<GameMsgLine>,
}

impl GameMsgTap {
    /// Сбросить строки (при включении перехвата).
    pub fn clear(&mut self) {
        self.text.clear();
        self.row = 0;
        self.col = 0;
        self.frames = 0;
        self.lines.clear();
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

        // Отдельно обновляем историю по номеру ZX-строки: следующая строка
        // игры не должна затирать уже напечатанное сообщение гиперпрыжка.
        let idx = self.lines.iter().position(|l| l.row == row);
        let line = match idx {
            Some(idx) => &mut self.lines[idx],
            None => {
                self.lines.push(GameMsgLine {
                    text: String::new(),
                    row,
                    col,
                    frames: 0,
                });
                self.lines.last_mut().expect("строка только что добавлена")
            }
        };
        let continuing = line.col.wrapping_add(line.text.len() as u8) == col;
        if !continuing {
            line.text.clear();
            line.col = col;
        }
        line.row = row;
        line.frames = 0;
        line.text.push(c as char);
    }

    /// Старение строк: вызывается раз в кадр после прогона эмулятора (при
    /// выключенном перехвате возраст не растёт).
    pub fn next_frame(&mut self) {
        if self.enabled {
            self.frames = self.frames.saturating_add(1);
            for line in &mut self.lines {
                line.frames = line.frames.saturating_add(1);
            }
        }
    }
}
