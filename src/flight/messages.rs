//! Нижние сообщения игры: перехваченный текст строки 13.
//!
//! Игра печатает сообщения через `sub_D499` и перепечатывает их каждый такт,
//! пока надпись должна быть на экране. Перехватчик отдаёт готовые строки;
//! здесь хранится только нижнее сообщение, а рендер рисует его по центру.
//! Позиция из ZX-строки (`row`/`col`) не используется.

use crate::machine::{GameMsgTap, BOTTOM_MESSAGE_ROW};

use super::view::ViewDir;

/// Сколько кадров (50 Гц) держать сообщение после последнего повтора.
///
/// Игра перепечатывает нижнюю строку примерно раз в 8 кадров, поэтому
/// 16 кадров хватает, чтобы надпись не мигала между тактами, и при этом она
/// исчезает вскоре после того, как игра перестала её печатать.
pub(super) const BOTTOM_MSG_TTL: u32 = 16;

/// Нижнее сообщение, перехваченное из строки 13.
#[derive(Clone, Debug, Default)]
pub(super) struct BottomMsg {
    /// Текст ровно как напечатала игра.
    pub(super) text: String,
    /// Кадров с последнего повтора.
    pub(super) age: u32,
}

/// Перехваченные сообщения игры, которые показывает нативный вид.
#[derive(Default)]
pub(super) struct Messages {
    /// Текущее нижнее сообщение.
    pub(super) bottom: Option<BottomMsg>,
}

impl Messages {
    pub(super) fn new() -> Self {
        Messages::default()
    }

    pub(super) fn reset(&mut self) {
        self.bottom = None;
    }

    /// Забрать новые строки, напечатанные игрой. Для показа берётся только
    /// нижняя строка (`row == 0x0D`); повтор того же текста продлевает показ.
    pub(super) fn sync(&mut self, tap: &mut GameMsgTap) {
        for event in tap.take_text_events() {
            if event.row != BOTTOM_MESSAGE_ROW {
                continue;
            }
            let text = event.text.trim();
            if text.is_empty() {
                continue;
            }
            self.bottom = Some(BottomMsg {
                text: text.to_string(),
                age: 0,
            });
        }
    }

    /// Постареть сообщение на `frames` кадров и убрать истёкшее.
    pub(super) fn advance(&mut self, frames: u32) {
        if let Some(msg) = &mut self.bottom {
            msg.age = msg.age.saturating_add(frames);
        }
        if self.bottom.as_ref().is_some_and(|m| m.age > BOTTOM_MSG_TTL) {
            self.bottom = None;
        }
    }

    /// Текст нижнего сообщения для показа по центру вида.
    pub(super) fn bottom_label(&self) -> Option<&str> {
        self.bottom.as_ref().map(|m| m.text.as_str())
    }
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
