//! Синтез звука бипера из переходов порта 0xFE.

use super::*;

impl Spectrum {
    /// Synthesize one display frame of 1-bit beeper audio (mono, [`AUDIO_RATE`])
    /// into `out`. Call after [`Spectrum::run_frame`] or
    /// [`Spectrum::run_frames`]; a multi-frame span is compressed into one
    /// display frame so fast-forward keeps audio in sync without buffering.
    pub fn render_audio(&mut self, out: &mut Vec<i16>) {
        const AMP: i16 = 4000;
        // Постоянная времени ~2 сэмпла: фронт 1/0 всё ещё слышен как переход,
        // но сглаживается за несколько сэмплов и не даёт резкого щелчка.
        const ALPHA: f64 = 0.6;
        let samples = (self.audio_rate as usize) / 50;
        out.clear();
        out.reserve(samples);
        let base = self.audio_cycle_base;
        let mut idx = 0usize;
        let mut level = self.beeper_frame_start;
        for i in 0..samples {
            // При ускорении `audio_span_cycles` кратно больше кадра: та же
            // длительность звука на выходе, но события бипера проигрываются
            // быстрее — как и сама игра.
            let cycle = base + (i as u64 * self.audio_span_cycles) / samples as u64;
            while idx < self.beeper_edges.len() && self.beeper_edges[idx].0 <= cycle {
                level = self.beeper_edges[idx].1;
                idx += 1;
            }
            let target = if level { AMP as f64 } else { -(AMP as f64) };
            self.beeper_filter += (target - self.beeper_filter) * ALPHA;
            out.push(self.beeper_filter.round() as i16);
        }
    }
}
