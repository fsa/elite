//! Перехват игровой печати (`sub_D499`, `sub_5DFB`) для нативного вида полёта.

/// Текстовая строка ZX, в которой игра печатает нижние сообщения.
/// Остальные надписи (вид, отсчёт, индикаторы) нативный рендер показывает сам.
pub const BOTTOM_MESSAGE_ROW: u8 = 0x0D;

/// Длина текстовой строки ZX в колонках. Буфер нижнего сообщения хранит
/// ровно эту строку: символ, напечатанный в колонке `col`, ложится в ячейку
/// `col` и остаётся там до конца жизни надписи.
pub const BOTTOM_MESSAGE_LEN: usize = 32;

/// Сколько готовых строк держать для рендера, если он их не забирает
/// (например, в headless-прогоне). Старые строки отбрасываются.
const MAX_PENDING_EVENTS: usize = 64;

/// Состояние сообщения в памяти игры, по которому перехват отличает новую
/// надпись от повторной печати той же самой.
///
/// `msgNumber` (`0xDA76`) — номер (токен) сообщения, `msgTimeLife` (`0xDA73`)
/// — остаток тактов его показа. Игра перепечатывает строку каждые несколько
/// кадров, уменьшая `msgTimeLife`; когда приходит новое сообщение, счётчик
/// снова взводится вверх. Это и служит признаком новой надписи, даже если
/// номер сообщения совпал.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageTag {
    /// `msgNumber`: номер/токен сообщения.
    pub number: u8,
    /// `msgTimeLife`: остаток тактов показа.
    pub life: u8,
}

/// Одна готовая строка, которую игра вывела на экран.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEvent {
    /// Текст строки (только печатные символы).
    pub text: String,
    /// Номер текстовой строки ZX (0..=23).
    pub row: u8,
    /// Колонка (0..=31) первого символа строки.
    pub col: u8,
}

/// Сборка строки из символов, которые игра напечатала через `sub_5DFB`.
///
/// Буфер — 32 колонки нижней строки ZX. Игра может печатать одну надпись
/// несколькими кусками (в разных тактах или с пропуском колонок), поэтому
/// символы складываются в буфер по колонкам, а не в растущую строку. Буфер
/// живёт всё время показа надписи; готовое событие всегда содержит всю
/// строку с обрезанными пустыми краями.
#[derive(Clone, Debug)]
struct TextCapture {
    /// Включён ли перехват. Приложение включает его только на нативном виде
    /// полёта (см. [`GameMsgTap::set_capture`]).
    enabled: bool,
    /// Готовые строки, ещё не забранные рендером.
    render_events: Vec<TextEvent>,
    /// 32 колонки нижней строки; пробел — пустая ячейка.
    buffer: [u8; BOTTOM_MESSAGE_LEN],
    /// Сообщение, которому принадлежит буфер.
    message: Option<MessageTag>,
    /// Был ли напечатан хотя бы один символ нижней строки в этом кадре.
    touched: bool,
}

impl Default for TextCapture {
    fn default() -> Self {
        TextCapture {
            enabled: false,
            render_events: Vec::new(),
            buffer: [b' '; BOTTOM_MESSAGE_LEN],
            message: None,
            touched: false,
        }
    }
}

impl TextCapture {
    /// Сбросить накопленные строки и буфер, не трогая флаг перехвата.
    fn reset_lines(&mut self) {
        self.render_events.clear();
        self.buffer.fill(b' ');
        self.message = None;
        self.touched = false;
    }

    /// Добавить символ, напечатанный на ZX-экране. `row`/`col` — точная
    /// ячейка глифа (`CrdXPrnSymbol` после продвижения курсора). Символы вне
    /// нижней строки сообщений игнорируются.
    ///
    /// `tag` — состояние `msgNumber`/`msgTimeLife` в момент печати. Если
    /// сообщение сменилось, буфер очищается: в нём не остаются хвосты
    /// предыдущей надписи.
    fn push(&mut self, c: u8, row: u8, col: u8, tag: MessageTag) {
        if !self.enabled
            || row != BOTTOM_MESSAGE_ROW
            || !(0x20..=0x7E).contains(&c)
        {
            return;
        }
        let new_message = self
            .message
            .is_none_or(|prev| prev.number != tag.number || tag.life > prev.life);
        if new_message {
            self.buffer.fill(b' ');
            self.message = Some(tag);
        }

        // `0x21..=0x26` — служебные значки, а не текст. Они занимают ячейку,
        // поэтому в буфере остаётся пробел: слова не слипаются.
        let cell = match c {
            0x21..=0x26 => b' ',
            0x2A => b'!',
            0x40 => b'_',
            _ => c,
        };
        self.buffer[usize::from(col) % BOTTOM_MESSAGE_LEN] = cell;
        self.touched = true;
    }

    /// Конец кадра: если в этом кадре что-то печаталось в нижнюю строку,
    /// отдать всю накопленную строку рендеру. Повторная печать каждый такт
    /// даёт повторные события — по ним видно, что надпись ещё на экране.
    fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.finish_line();
    }

    /// Собрать строку из буфера и положить её в очередь рендера.
    fn finish_line(&mut self) {
        if !self.touched {
            return;
        }
        self.touched = false;
        let Some((col, text)) = self.buffer_text() else {
            return;
        };
        let event = TextEvent {
            text,
            row: BOTTOM_MESSAGE_ROW,
            col,
        };
        self.render_events.push(event);
        if self.render_events.len() > MAX_PENDING_EVENTS {
            let excess = self.render_events.len() - MAX_PENDING_EVENTS;
            self.render_events.drain(..excess);
        }
    }

    /// Вся непустая часть буфера с обрезанными пробелами по краям.
    /// Возвращает колонку первого символа и готовый текст.
    fn buffer_text(&self) -> Option<(u8, String)> {
        let first = self.buffer.iter().position(|&b| b != b' ')?;
        let last = self.buffer.iter().rposition(|&b| b != b' ')?;
        let text = self.buffer[first..=last]
            .iter()
            .map(|&b| b as char)
            .collect();
        Some((first as u8, text))
    }

    /// Забрать готовые строки для рендера.
    fn take_events(&mut self) -> Vec<TextEvent> {
        std::mem::take(&mut self.render_events)
    }
}

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
    /// Копить ли старые поля `text`/`lines` (прежний перехват). Очередь для
    /// рендера (`text_log`) работает независимо.
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
    /// Перехват готового текста нижней строки: буфер строки и очередь для
    /// рендера.
    text_log: TextCapture,
}

impl GameMsgTap {
    /// Включить или выключить перехват. При смене состояния буфер и очереди
    /// очищаются, чтобы на новом экране не висели строки с прошлого.
    ///
    /// Приложение включает перехват только пока открыт нативный вид полёта
    /// (режимы View); на заставке, карте, станции и в меню он выключен.
    pub fn set_capture(&mut self, enabled: bool) {
        if self.enabled != enabled {
            self.clear();
        }
        self.enabled = enabled;
        self.text_log.enabled = enabled;
    }

    /// Добавить готовый символ, который игра вывела на экран (`sub_5DFB`).
    /// `row`/`col` — точная ячейка глифа после сдвига курсора. Для рендера
    /// берутся только символы нижней строки сообщений (`BOTTOM_MESSAGE_ROW`).
    #[allow(dead_code)] // используется тестами
    pub fn capture_text_char(&mut self, c: u8, row: u8, col: u8) {
        self.text_log.push(c, row, col, MessageTag::default());
    }

    /// То же, что [`Self::capture_text_char`], но с состоянием сообщения из
    /// памяти игры (`msgNumber`/`msgTimeLife`). По нему перехват понимает,
    /// что началась новая надпись, и очищает буфер.
    pub fn capture_text_char_tagged(
        &mut self,
        c: u8,
        row: u8,
        col: u8,
        tag: MessageTag,
    ) {
        self.text_log.push(c, row, col, tag);
    }

    /// Завершить кадр перехвата. Вызывается после прогона эмулятора: всё,
    /// что игра напечатала за кадр, уходит в очередь рендера.
    pub fn end_text_frame(&mut self) {
        self.text_log.end_frame();
    }

    /// Включить перехват для тестов сборки строк.
    #[cfg(test)]
    pub fn enable_text_capture_for_test(&mut self) {
        self.text_log.enabled = true;
    }

    /// Забрать готовые строки нижней строки для рендера. Очередь при этом
    /// очищается; строки других строк сюда не попадают.
    pub fn take_text_events(&mut self) -> Vec<TextEvent> {
        self.text_log.take_events()
    }

    /// Готовые строки, ещё не забранные рендером (для тестов).
    #[cfg(test)]
    pub fn text_events(&self) -> &[TextEvent] {
        &self.text_log.render_events
    }

    /// Сбросить строки и буфер (при включении перехвата).
    pub fn clear(&mut self) {
        self.text.clear();
        self.row = 0;
        self.col = 0;
        self.frames = 0;
        self.lines.clear();
        self.text_log.reset_lines();
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
