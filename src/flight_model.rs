//! Модель полёта: нативная симуляция и адаптер чтения из Z80.
//! Рендер получает только готовые данные для отрисовки.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub pitch_up: bool,
    pub pitch_down: bool,
    pub roll_left: bool,
    pub roll_right: bool,
    pub throttle_up: bool,
    pub throttle_down: bool,
    pub fire: bool,
}

impl Controls {
    pub fn from_matrix(matrix: &[[bool; 5]; 8], kempston: &[bool; 5]) -> Self {
        let key = |row: usize, bit: usize| matrix[row][bit];
        Controls {
            pitch_up: key(1, 1) || kempston[3],
            pitch_down: key(0, 2) || kempston[2],
            roll_left: key(7, 3) || kempston[1],
            roll_right: key(7, 2) || kempston[0],
            // Speed up/down — Space и Symbol Shift оригинала. На PC Symbol
            // Shift — правый Shift, но принимаем и левый для удобства.
            throttle_up: key(7, 0),
            throttle_down: key(7, 1) || key(0, 0),
            fire: key(1, 0) || kempston[4],
        }
    }
}
