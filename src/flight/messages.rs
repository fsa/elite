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

/// Период мигания надписи стыковочного компьютера (кадры 50 Гц).
///
/// В оригинале `sub_DAB4` взводит `msgTimeLife = 0x0E` и печатает надпись, пока
/// счётчик не упадёт до 4; после этого идёт короткая пауза и счётчик взводится
/// снова. На эмуляторе один проход `GameMain` занимает 3–4 кадра, поэтому
/// между печатями внутри пачки тоже бывают пропуски по 2–5 кадров. Просто
/// сократить TTL до одного кадра — получить частое мигание внутри пачки.
/// Нативный вид поэтому держит текст постоянно в течение
/// [`DOCKING_BLINK_ON_FRAMES`], а гасит его на остаток периода.
pub(super) const DOCKING_BLINK_PERIOD: u32 = 35;
pub(super) const DOCKING_BLINK_ON_FRAMES: u32 = 28;

/// Сколько кадров держать надпись стыковочного компьютера без печати.
///
/// Дольше, чем самая длинная пауза между печатями в оригинале, но короче
/// полного периода мигания: если стыковка закончилась, надпись быстро уйдёт.
pub(super) const DOCKING_MSG_TTL: u32 = 12;

/// Признак надписи, которую оригинал показывает миганием.
fn is_blinking_message(text: &str) -> bool {
    text.to_ascii_uppercase().contains("DOCKING COMPUTER")
}

/// Нижнее сообщение, перехваченное из строки 13.
#[derive(Clone, Debug, Default)]
pub(super) struct BottomMsg {
    /// Текст ровно как напечатала игра.
    pub(super) text: String,
    /// Кадров с последнего повтора.
    pub(super) age: u32,
    /// Сколько кадров текст живёт без повторной печати.
    ttl: u32,
    /// Мигает ли надпись (стыковочный компьютер).
    blink: bool,
    /// Кадр внутри цикла мигания; не сбрасывается повторной печатью.
    phase: u32,
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
            let blink = is_blinking_message(text);
            let ttl = if blink { DOCKING_MSG_TTL } else { BOTTOM_MSG_TTL };
            // Повтор той же надписи продлевает показ, но не начинает мигание
            // заново: иначе печать каждые 3–4 кадра держала бы текст видимым.
            let phase = match self.bottom.as_ref() {
                Some(m) if blink && m.blink && m.text == text => m.phase,
                _ => 0,
            };
            self.bottom = Some(BottomMsg {
                text: text.to_string(),
                age: 0,
                ttl,
                blink,
                phase,
            });
        }
    }

    /// Постареть сообщение на `frames` кадров и убрать истёкшее.
    pub(super) fn advance(&mut self, frames: u32) {
        if let Some(msg) = &mut self.bottom {
            msg.age = msg.age.saturating_add(frames);
            if msg.blink {
                msg.phase = msg.phase.saturating_add(frames) % DOCKING_BLINK_PERIOD;
            }
        }
        if self.bottom.as_ref().is_some_and(|m| m.age > m.ttl) {
            self.bottom = None;
        }
    }

    /// Текст нижнего сообщения для показа по центру вида.
    ///
    /// Мигающая надпись скрывается во второй части периода, даже если игра
    /// продолжает её печатать: повтор сбрасывает только срок жизни, но не фазу.
    pub(super) fn bottom_label(&self) -> Option<&str> {
        self.bottom.as_ref().and_then(|m| {
            if !m.blink || m.phase < DOCKING_BLINK_ON_FRAMES {
                Some(m.text.as_str())
            } else {
                None
            }
        })
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
