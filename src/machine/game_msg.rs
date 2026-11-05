//! Перехват игровой печати (`sub_D499`, `sub_5DFB`) для нативного вида полёта
//! и отладочного журнала текста.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Текстовая строка ZX, в которой игра печатает нижние сообщения.
/// Остальные надписи (вид, отсчёт, индикаторы) нативный рендер показывает сам.
pub const BOTTOM_MESSAGE_ROW: u8 = 0x0D;

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

/// Сборка готовых строк из символов, которые игра напечатала через
/// `sub_5DFB`. Берёт только нижнее сообщение (`BOTTOM_MESSAGE_ROW`), пишет
/// строки в `debug.log` и ничего никому не передаёт.
#[derive(Clone, Debug, Default)]
struct TextCapture {
    /// Включён ли перехват. Включается один раз при старте приложения.
    enabled: bool,
    /// Куда писать строки; `None` — только накопление (тесты).
    log_path: Option<PathBuf>,
    /// Готовые строки, ещё не записанные в файл.
    events: Vec<TextEvent>,
    /// Строка и колонка начала текущей строки.
    row: u8,
    col: u8,
    /// Ячейка последнего напечатанного символа (включая значки).
    last_col: Option<u8>,
    /// Текст текущей строки.
    text: String,
}

impl TextCapture {
    /// Начать новый журнал рядом с исполняемым файлом.
    fn start(&mut self) {
        let path = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|dir| dir.join("debug.log")))
            .unwrap_or_else(|| PathBuf::from("debug.log"));
        if let Err(e) = File::create(&path) {
            eprintln!("cannot create {}: {e}", path.display());
        }
        self.enabled = true;
        self.log_path = Some(path);
        self.events.clear();
        self.text.clear();
        self.last_col = None;
    }

    /// Добавить символ, напечатанный на ZX-экране. `row`/`col` — точная
    /// ячейка глифа (`CrdXPrnSymbol` после продвижения курсора). Символы вне
    /// нижней строки сообщений игнорируются.
    fn push(&mut self, c: u8, row: u8, col: u8) {
        if !self.enabled
            || row != BOTTOM_MESSAGE_ROW
            || !(0x20..=0x7E).contains(&c)
        {
            return;
        }
        let continuing = self.row == row
            && self
                .last_col
                .map_or(false, |last| last.wrapping_add(1) == col);
        if !continuing {
            self.finish_line();
            self.row = row;
            self.col = col;
        }
        self.last_col = Some(col);

        // `0x21..=0x26` — служебные значки, а не текст. Они занимают ячейку,
        // поэтому `last_col` выше всё равно обновлён.
        if (0x21..=0x26).contains(&c) {
            return;
        }
        let ch = match c {
            0x2A => '!',
            0x40 => '_',
            _ => c as char,
        };
        self.text.push(ch);
    }

    /// Конец кадра: завершить накопленную строку и записать готовые строки.
    /// Если игра перепечатывает сообщение каждый кадр, каждая копия попадёт
    /// в журнал — по ним видно, что надпись ещё на экране.
    fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.finish_line();
        self.flush();
    }

    /// Завершить текущую строку и положить её в очередь на запись.
    fn finish_line(&mut self) {
        if !self.text.trim().is_empty() {
            self.events.push(TextEvent {
                text: std::mem::take(&mut self.text),
                row: self.row,
                col: self.col,
            });
        }
        self.text.clear();
        self.last_col = None;
    }

    /// Дописать готовые строки в журнал.
    fn flush(&mut self) {
        let Some(path) = &self.log_path else {
            return;
        };
        if self.events.is_empty() {
            return;
        }
        let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
            self.events.clear();
            return;
        };
        for event in self.events.drain(..) {
            let _ = writeln!(
                file,
                "[{:02}:{:02}] {}",
                event.row, event.col, event.text
            );
        }
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
    /// Перехват готового текста для `debug.log` рядом с исполняемым файлом.
    /// На нативный вид полёта не влияет.
    text_log: TextCapture,
}

impl GameMsgTap {
    /// Начать отладочный журнал текста. Создаёт `debug.log` рядом с
    /// исполняемым файлом (файл прошлого запуска перезаписывается).
    pub fn start_text_log(&mut self) {
        self.text_log.start();
    }

    /// Добавить готовый символ, который игра вывела на экран (`sub_5DFB`).
    /// `row`/`col` — точная ячейка глифа после сдвига курсора. Для журнала
    /// берутся только символы нижней строки сообщений (`BOTTOM_MESSAGE_ROW`).
    pub fn capture_text_char(&mut self, c: u8, row: u8, col: u8) {
        self.text_log.push(c, row, col);
    }

    /// Завершить кадр журнала. Вызывается после прогона эмулятора: всё, что
    /// игра напечатала за кадр, уходит в `debug.log`.
    pub fn end_text_frame(&mut self) {
        self.text_log.end_frame();
    }

    /// Включить перехват без файла (для тестов сборки строк).
    #[cfg(test)]
    pub fn enable_text_capture_for_test(&mut self) {
        self.text_log.enabled = true;
    }

    /// Готовые строки, ещё не записанные в файл (для тестов).
    #[cfg(test)]
    pub fn text_events(&self) -> &[TextEvent] {
        &self.text_log.events
    }

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
