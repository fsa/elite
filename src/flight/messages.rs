//! Надписи вида: перехваченные сообщения игры, подписи направления и отсчёт.
//!
//! Текст сообщений печатает сама игра через `sub_D499`; нативный вид только
//! сохраняет напечатанные строки и показывает их тем же шрифтом, что заголовок
//! заставки. Когда свежего сообщения нет, на его месте держится подпись
//! направления вида.

use crate::machine::GameMsgTap;

use super::view::ViewDir;

/// Сколько кадров (50 Гц) держать перехваченное сообщение игры после
/// последнего напечатанного символа: старые строки не должны висеть вечно.
pub(super) const MSG_MAX_AGE: u32 = 75;
/// Текстовые строки ZX `0..=2` — верхняя зона вида, куда игра печатает
/// подписи; здесь нативный экран показывает верхнюю надпись.
pub(super) const MSG_TOP_ROWS: u8 = 2;

/// Снимок перехваченного игрового сообщения для показа на виде.
#[derive(Clone, Debug, Default)]
pub(super) struct CapturedMsg {
    /// Текст строки ровно как напечатал Z80.
    pub(super) text: String,
    /// Текстовая строка ZX (0..=23), куда она печаталась.
    pub(super) row: u8,
    /// Возраст в кадрах после последнего символа.
    pub(super) frames: u32,
}

/// Перехваченные сообщения игры.
#[derive(Default)]
pub(super) struct Messages {
    /// Последняя напечатанная строка (для верхней надписи вида).
    pub(super) tap: Option<CapturedMsg>,
    /// Строка гиперпрыжка, выбранная из истории печати по строкам ZX.
    pub(super) jump: Option<CapturedMsg>,
}

impl Messages {
    pub(super) fn new() -> Self {
        Messages::default()
    }

    pub(super) fn reset(&mut self) {
        self.tap = None;
        self.jump = None;
    }

    /// Забрать перехваченное игровое сообщение (заполняет хук печати
    /// `sub_D499`; см. [`crate::machine::GameMsgTap`]).
    ///
    /// Игра печатает строку гиперпрыжка в отдельную ZX-строку (13), а следом
    /// за ней — индикаторы и подписи в другие строки. Поэтому общая «последняя
    /// строка» `tap.text` уже не содержит `HYPERSPACE`; саму строку прыжка
    /// берём из истории по ZX-строкам.
    pub(super) fn sync(&mut self, tap: &GameMsgTap) {
        if !tap.enabled {
            return;
        }
        if !tap.text.trim().is_empty() {
            self.tap = Some(CapturedMsg {
                text: tap.text.clone(),
                row: tap.row,
                frames: tap.frames,
            });
        }
        self.jump = tap
            .lines
            .iter()
            .filter(|l| l.row > MSG_TOP_ROWS && l.text.contains("HYPERSPACE"))
            .min_by_key(|l| l.frames)
            .map(|l| CapturedMsg {
                text: l.text.clone(),
                row: l.row,
                frames: l.frames,
            });
    }

    /// Текст верхней надписи: свежее сообщение игры из верхней зоны вида,
    /// причёсанное с большой буквы; иначе — подпись направления вида.
    pub(super) fn top_label(&self, view_dir: ViewDir) -> String {
        self.tap
            .as_ref()
            .and_then(|m| {
                (m.frames <= MSG_MAX_AGE && m.row <= MSG_TOP_ROWS && !m.text.trim().is_empty())
                    .then(|| pretty_title(m.text.trim()))
            })
            .unwrap_or_else(|| view_label(view_dir).to_string())
    }

    /// Свежая строка гиперпрыжка, напечатанная игрой.
    ///
    /// Во время отсчёта гиперпрыжка оригинал печатает `HYPERSPACE -- <планета>`
    /// в отдельную ZX-строку (токен `msgNumber` из `GoJump`). Нативный вид
    /// показывает именно перехваченный текст, а не собирает строку сам.
    pub(super) fn jump_label(&self) -> Option<&str> {
        let m = self.jump.as_ref()?;
        if m.frames > MSG_MAX_AGE || m.row <= MSG_TOP_ROWS || !m.text.contains("HYPERSPACE") {
            return None;
        }
        Some(m.text.trim())
    }
}

/// «Причесать» сообщение игры: слова с большой буквы, остальные символы
/// строчные — как заголовок «Elite» на заставке. «FRONT VIEW» → «Front View»,
/// «DOCKING COMPUTERS  ON» → «Docking Computers On».
pub(super) fn pretty_title(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Подпись направления вида: те же слова, что в оригинале (`FRONT`, `REAR`,
/// ...), но с большой буквы, как заголовок «Elite» на заставке.
pub(super) fn view_label(dir: ViewDir) -> &'static str {
    match dir {
        ViewDir::Forward => "Front View",
        ViewDir::Back => "Rear View",
        ViewDir::Left => "Left View",
        ViewDir::Right => "Right View",
    }
}

/// Число на экране во время отсчёта гиперпрыжка.
///
/// Z80 показывает `JumpCountdown >> 1` после уменьшения, поэтому сырым
/// значениям `0x15..1` соответствуют `10..1`. Здесь то же самое, но без
/// «прыжка» на первом кадре: `(raw + 1) / 2` даёт 10 уже при `0x15`.
pub(super) fn countdown_number(raw: u8) -> u32 {
    if raw == 0 {
        0
    } else {
        ((raw as u32 + 1) / 2).min(10)
    }
}
