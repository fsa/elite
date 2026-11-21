//! Синтез звука бипера из переходов порта 0xFE.

use super::*;

impl Spectrum {
    /// Synthesize one display frame of 1-bit beeper audio (mono, [`AUDIO_RATE`])
    /// into `out`. Call after [`Spectrum::run_frame`] or
    /// [`Spectrum::run_frames`]; a multi-frame span is compressed into one
    /// display frame so fast-forward keeps audio in sync without buffering.
    pub fn render_audio(&mut self, out: &mut Vec<i16>) {
        const AMP: i16 = 4000;
        let frames = (self.audio_span_cycles / super::CYCLES_PER_FRAME).max(1);
        let samples = (self.audio_rate as usize / 50) * frames as usize;
        out.clear();
        out.reserve(samples);
        let base = self.audio_cycle_base;
        let mut idx = 0usize;
        let mut level = self.beeper_frame_start;
        let span = self.audio_span_cycles.max(1);
        for i in 0..samples {
            // Весь span (1+ кадров Z80) отображаем в соответствующее число
            // 20 мс буферов. Так сохраняется тактовая форма фронтов.
            let cycle = base + (i as u64 * span) / samples as u64;
            while idx < self.beeper_edges.len() && self.beeper_edges[idx].0 <= cycle {
                level = self.beeper_edges[idx].1;
                idx += 1;
            }
            out.push(if level { AMP } else { -AMP });
        }
        self.beeper_frame_start = level;
        self.beeper_edges.clear();
        self.beeper_filter = if level { AMP as f64 } else { -(AMP as f64) };
    }

    /// Сгенерировать звук удержания текущего уровня бипера на один 50 Гц кадр.
    /// Используется, когда Z80 не выполнялся в этом кадре рендера.
    pub fn render_audio_hold(&mut self, out: &mut Vec<i16>) {
        const AMP: i16 = 4000;
        let samples = (self.audio_rate as usize) / 50;
        out.clear();
        out.reserve(samples);
        let level = self.beeper;
        let s = if level { AMP } else { -AMP };
        out.resize(samples, s);
        self.beeper_frame_start = level;
        self.beeper_filter = if level { AMP as f64 } else { -(AMP as f64) };
    }
}
