//! Перехват игровой печати (`sub_5DFB`) для нативного вида полёта.

/// Текстовая строка ZX, в которой игра печатает нижние сообщения.
/// Остальные надписи (вид, отсчёт, индикаторы) нативный рендер показывает сам.
pub const BOTTOM_MESSAGE_ROW: u8 = 0x0D;

/// Длина текстовой строки ZX в колонках. Буфер нижнего сообщения хранит
/// ровно эту строку: символ, напечатанный в колонке `col`, ложится в ячейку
/// `col` и остаётся там до конца жизни надписи.
const BOTTOM_MESSAGE_LEN: usize = 32;

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

/// Перехват игровой печати (`sub_5DFB`).
///
/// Игра печатает сообщения на ZX-экран посимвольно: «Front View» и прочие
/// подписи, строки гиперпрыжка, числа отсчёта. Пока перехват взведён, хук
/// печати копит готовый текст нижней строки — нативный вид полёта показывает
/// его на своём HD-слое своим шрифтом. Сам эмулятор не меняется: хук только
/// читает `A` и `CrdXPrnSymbol`.
///
/// Сюда попадают уже окончательные коды символов (после регистра и раскрытия
/// токенов), поэтому события содержат то же, что видно на ZX-экране.
#[derive(Clone, Debug, Default)]
pub struct GameMsgTap {
    /// Буфер строки и очередь готовых событий для рендера.
    text_log: TextCapture,
}

impl GameMsgTap {
    /// Включить или выключить перехват. При смене состояния буфер и очередь
    /// очищаются, чтобы на новом экране не висели строки с прошлого.
    ///
    /// Приложение включает перехват только пока открыт нативный вид полёта
    /// (режимы View); на заставке, карте, станции и в меню он выключен.
    pub fn set_capture(&mut self, enabled: bool) {
        if self.text_log.enabled != enabled {
            self.clear();
        }
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
        self.text_log.reset_lines();
    }
}
