//! Раскладка физических клавиш: матрица ZX Spectrum, джойстик Kempston и
//! функциональные клавиши.

use winit::keyboard::KeyCode;

use crate::flight;

/// Позиция клавиши в матрице ZX Spectrum: (полуряд 0..8, бит 0..5).
///
/// Клавиши берутся по физической позиции (`physical_key`), поэтому активная
/// раскладка (например русская) на управление не влияет.
pub(crate) const MATRIX_MAP: &[(KeyCode, usize, u8)] = &[
    (KeyCode::ShiftLeft, 0, 0),
    (KeyCode::KeyZ, 0, 1),
    (KeyCode::KeyX, 0, 2),
    (KeyCode::KeyC, 0, 3),
    (KeyCode::KeyV, 0, 4),
    (KeyCode::KeyA, 1, 0),
    (KeyCode::KeyS, 1, 1),
    (KeyCode::KeyD, 1, 2),
    (KeyCode::KeyF, 1, 3),
    (KeyCode::KeyG, 1, 4),
    (KeyCode::KeyQ, 2, 0),
    (KeyCode::KeyW, 2, 1),
    (KeyCode::KeyE, 2, 2),
    (KeyCode::KeyR, 2, 3),
    (KeyCode::KeyT, 2, 4),
    (KeyCode::Digit1, 3, 0),
    (KeyCode::Digit2, 3, 1),
    (KeyCode::Digit3, 3, 2),
    (KeyCode::Digit4, 3, 3),
    (KeyCode::Digit5, 3, 4),
    (KeyCode::Digit0, 4, 0),
    (KeyCode::Digit9, 4, 1),
    (KeyCode::Digit8, 4, 2),
    (KeyCode::Digit7, 4, 3),
    (KeyCode::Digit6, 4, 4),
    (KeyCode::KeyP, 5, 0),
    (KeyCode::KeyO, 5, 1),
    (KeyCode::KeyI, 5, 2),
    (KeyCode::KeyU, 5, 3),
    (KeyCode::KeyY, 5, 4),
    (KeyCode::Enter, 6, 0),
    (KeyCode::KeyL, 6, 1),
    (KeyCode::KeyK, 6, 2),
    (KeyCode::KeyJ, 6, 3),
    (KeyCode::KeyH, 6, 4),
    (KeyCode::Space, 7, 0),
    (KeyCode::ShiftRight, 7, 1),
    (KeyCode::KeyM, 7, 2),
    (KeyCode::KeyN, 7, 3),
    (KeyCode::KeyB, 7, 4),
];

/// Позиция физической клавиши в матрице ZX Spectrum (полуряд, бит).
pub(crate) fn matrix_pos(code: KeyCode) -> Option<(usize, u8)> {
    MATRIX_MAP
        .iter()
        .find(|&&(c, _, _)| c == code)
        .map(|&(_, row, bit)| (row, bit))
}

/// Оригинальная клавиша, которой соответствует функциональная: `F1` → `1`,
/// `F2` → `2`, `F3` → `3`, `F4` → `4`.
pub(crate) fn function_game_key(code: KeyCode) -> Option<KeyCode> {
    Some(match code {
        KeyCode::F1 => KeyCode::Digit1,
        KeyCode::F2 => KeyCode::Digit2,
        KeyCode::F3 => KeyCode::Digit3,
        KeyCode::F4 => KeyCode::Digit4,
        _ => return None,
    })
}

/// Вид из кабины, который включает функциональная клавиша `F1`-`F4`.
pub(crate) fn function_view(code: KeyCode) -> Option<flight::ViewDir> {
    Some(match code {
        KeyCode::F1 => flight::ViewDir::Forward,
        KeyCode::F2 => flight::ViewDir::Back,
        KeyCode::F3 => flight::ViewDir::Left,
        KeyCode::F4 => flight::ViewDir::Right,
        _ => return None,
    })
}

/// Цифра физической клавиши (верхний ряд или цифровой блок), 0..9.
pub(crate) fn digit_of(code: KeyCode) -> Option<u8> {
    Some(match code {
        KeyCode::Digit0 | KeyCode::Numpad0 => 0,
        KeyCode::Digit1 | KeyCode::Numpad1 => 1,
        KeyCode::Digit2 | KeyCode::Numpad2 => 2,
        KeyCode::Digit3 | KeyCode::Numpad3 => 3,
        KeyCode::Digit4 | KeyCode::Numpad4 => 4,
        KeyCode::Digit5 | KeyCode::Numpad5 => 5,
        KeyCode::Digit6 | KeyCode::Numpad6 => 6,
        KeyCode::Digit7 | KeyCode::Numpad7 => 7,
        KeyCode::Digit8 | KeyCode::Numpad8 => 8,
        KeyCode::Digit9 | KeyCode::Numpad9 => 9,
        _ => return None,
    })
}

/// Бит джойстика Kempston: курсорные клавиши — направления, правая Ctrl — огонь.
pub(crate) fn kempston_bit(code: KeyCode) -> Option<u8> {
    Some(match code {
        KeyCode::ArrowRight => 0,
        KeyCode::ArrowLeft => 1,
        KeyCode::ArrowDown => 2,
        KeyCode::ArrowUp => 3,
        KeyCode::ControlRight => 4,
        _ => return None,
    })
}

/// Нажатие, которое игра использует как управление (матрица ZX или джойстик
/// Kempston). По такой клавише нативный экран закрывается и нажатие уходит
/// в игру — чтобы из карты/закупки можно было сразу перейти к экранам игры.
pub(crate) fn is_game_key(code: KeyCode) -> bool {
    matrix_pos(code).is_some() || kempston_bit(code).is_some()
}
