//! Синтез звука бипера из переходов порта 0xFE.

use super::*;

impl Spectrum {
    /// Synthesize one frame of 1-bit beeper audio (mono, [`AUDIO_RATE`]) into
    /// `out`. Call after [`Spectrum::run_frame`].
    pub fn render_audio(&mut self, out: &mut Vec<i16>) {
        const AMP: i16 = 4000;
        let samples = (self.audio_rate as usize) / 50;
        out.clear();
        out.reserve(samples);
        let base = self.audio_cycle_base;
        let mut idx = 0usize;
        let mut level = self.beeper_frame_start;
        for i in 0..samples {
            let cycle = base + (i as u64 * CYCLES_PER_FRAME) / samples as u64;
            while idx < self.beeper_edges.len() && self.beeper_edges[idx].0 <= cycle {
                level = self.beeper_edges[idx].1;
                idx += 1;
            }
            out.push(if level { AMP } else { -AMP });
        }
    }
}
