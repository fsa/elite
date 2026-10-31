//! Детерминированный генератор случайных чисел для звёзд и выстрелов.
//!
//! Один и тот же xorshift32 используется и звёздным полем, и противниками,
//! поэтому кадр воспроизводим при одинаковой последовательности тактов.

/// Шаг xorshift32.
pub(super) struct Rng(u32);

impl Rng {
    pub(super) fn new(seed: u32) -> Self {
        Rng(seed.max(1))
    }

    /// Следующее 32-битное значение.
    pub(super) fn next_u32(&mut self) -> u32 {
        let mut x = self.0.max(1);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// Следующее значение в диапазоне `0.0..1.0`.
    pub(super) fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
}
