/// Fixed-length delay line backed by a circular buffer.
///
/// Delays an input signal by exactly `delay` samples. Useful for comb filters,
/// echo effects, and feed-forward / feed-back delay networks.
pub struct FixedDelay {
    buffer: Vec<f64>,
    write_pos: usize,
    delay: usize,
}

impl FixedDelay {
    /// Create a new fixed delay line.
    ///
    /// `delay` – the number of samples to delay the signal.
    #[must_use]
    pub fn new(delay: usize) -> Self {
        Self {
            buffer: vec![0.0; delay.max(1)],
            write_pos: 0,
            delay: delay.max(1),
        }
    }

    /// Process a single sample through the delay line.
    ///
    /// Returns the sample that was written `delay` samples ago.
    pub fn process_sample(&mut self, input: f64) -> f64 {
        let output = self.buffer[self.write_pos];
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.delay;
        output
    }

    /// Process an entire signal through the delay line.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    /// Reset the delay line to all zeros.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

/// Variable-length delay line with linear interpolation.
///
/// Supports fractional and time-varying delays. Useful for chorus, flanger,
/// vibrato effects, and Doppler simulation.
pub struct VariableDelay {
    buffer: Vec<f64>,
    write_pos: usize,
    max_delay: usize,
}

impl VariableDelay {
    /// Create a new variable delay line.
    ///
    /// `max_delay` – the maximum delay in samples the line can support.
    #[must_use]
    pub fn new(max_delay: usize) -> Self {
        let size = max_delay.max(2);
        Self {
            buffer: vec![0.0; size],
            write_pos: 0,
            max_delay: size,
        }
    }

    /// Process one sample with the given fractional delay.
    ///
    /// `delay` – current delay in samples (can be fractional). Clamped to
    /// `[1, max_delay - 1]`.
    ///
    /// Uses linear interpolation between adjacent buffer positions.
    pub fn process_sample(&mut self, input: f64, delay: f64) -> f64 {
        // Write current sample
        self.buffer[self.write_pos] = input;

        // Clamp delay
        let d = delay.max(1.0).min((self.max_delay - 1) as f64);
        let d_int = d.floor() as usize;
        let frac = d - d_int as f64;

        // Read with linear interpolation
        let idx0 = (self.write_pos + self.max_delay - d_int) % self.max_delay;
        let idx1 = (self.write_pos + self.max_delay - d_int - 1) % self.max_delay;

        let output = (1.0 - frac) * self.buffer[idx0] + frac * self.buffer[idx1];

        self.write_pos = (self.write_pos + 1) % self.max_delay;
        output
    }

    /// Process an entire signal with a constant fractional delay.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64], delay: f64) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x, delay)).collect()
    }

    /// Reset the delay line to all zeros.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

/// FIFO (first-in, first-out) delay buffer.
///
/// Buffers blocks of data and outputs them after a configurable number of
/// block-periods have passed. Useful for block-level pipeline delays.
pub struct FifoDelay {
    buffer: Vec<Vec<f64>>,
    write_pos: usize,
    num_blocks: usize,
}

impl FifoDelay {
    /// Create a new FIFO delay that holds `num_blocks` blocks.
    #[must_use]
    pub fn new(num_blocks: usize) -> Self {
        let n = num_blocks.max(1);
        Self {
            buffer: vec![Vec::new(); n],
            write_pos: 0,
            num_blocks: n,
        }
    }

    /// Push a new block and return the oldest buffered block.
    pub fn push_block(&mut self, block: Vec<f64>) -> Vec<f64> {
        let read_pos = self.write_pos;
        let output = std::mem::replace(&mut self.buffer[read_pos], block);
        self.write_pos = (self.write_pos + 1) % self.num_blocks;
        output
    }

    /// Reset all buffered blocks to empty.
    pub fn reset(&mut self) {
        for b in &mut self.buffer {
            b.clear();
        }
        self.write_pos = 0;
    }
}

/// Apply a fixed delay to a signal (convenience function, non-streaming).
///
/// Returns a new signal of the same length, delayed by `delay` samples.
/// The first `delay` samples are zero.
#[must_use]
pub fn apply_fixed_delay(signal: &[f64], delay: usize) -> Vec<f64> {
    let mut dl = FixedDelay::new(delay);
    dl.apply(signal)
}

// ─── Tapped Delay Line ────────────────────────────────────────────────────────

/// Tapped delay line: a delay line with multiple taps at configurable
/// positions, each with its own gain.
///
/// Output is the weighted sum of the tapped samples.
pub struct TappedDelayLine {
    buffer: Vec<f64>,
    write_pos: usize,
    taps: Vec<usize>,
    gains: Vec<f64>,
}

impl TappedDelayLine {
    /// Create a new tapped delay line.
    ///
    /// * `max_delay` – size of the internal buffer (must be ≥ max tap position)
    /// * `taps` – delay positions in samples for each tap
    /// * `gains` – gain applied to each tap
    #[must_use]
    pub fn new(max_delay: usize, taps: &[usize], gains: &[f64]) -> Self {
        let len = max_delay.max(1);
        Self {
            buffer: vec![0.0; len],
            write_pos: 0,
            taps: taps.to_vec(),
            gains: gains.to_vec(),
        }
    }

    /// Process one sample; returns the weighted sum of tapped outputs.
    pub fn process_sample(&mut self, input: f64) -> f64 {
        self.buffer[self.write_pos] = input;
        let buf_len = self.buffer.len();
        let mut output = 0.0;
        for (i, &tap) in self.taps.iter().enumerate() {
            let idx = (self.write_pos + buf_len - tap) % buf_len;
            let g = if i < self.gains.len() { self.gains[i] } else { 1.0 };
            output += self.buffer[idx] * g;
        }
        self.write_pos = (self.write_pos + 1) % buf_len;
        output
    }

    /// Process an entire signal.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    /// Reset the delay line state.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

// ─── Echo Generator ───────────────────────────────────────────────────────────

/// Echo type for the echo generator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EchoType {
    /// Single echo (feedforward only).
    Single,
    /// Repeating echo (feedback / reverb-like).
    Feedback,
}

/// Simple echo generator using a circular delay buffer.
pub struct EchoGenerator {
    buffer: Vec<f64>,
    write_pos: usize,
    delay: usize,
    decay: f64,
    echo_type: EchoType,
}

impl EchoGenerator {
    /// Create a new echo generator.
    ///
    /// * `delay` – echo delay in samples
    /// * `decay` – echo decay coefficient (0.0 … 1.0)
    /// * `echo_type` – `Single` for one echo, `Feedback` for repeating
    #[must_use]
    pub fn new(delay: usize, decay: f64, echo_type: EchoType) -> Self {
        let d = delay.max(1);
        Self {
            buffer: vec![0.0; d],
            write_pos: 0,
            delay: d,
            decay,
            echo_type,
        }
    }

    /// Process one sample.
    pub fn process_sample(&mut self, input: f64) -> f64 {
        let delayed = self.buffer[self.write_pos];
        let output = input + self.decay * delayed;
        self.buffer[self.write_pos] = match self.echo_type {
            EchoType::Single => input,
            EchoType::Feedback => output,
        };
        self.write_pos = (self.write_pos + 1) % self.delay;
        output
    }

    /// Process an entire signal.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    /// Reset internal state.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_delay_shifts_signal() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mut dl = FixedDelay::new(2);
        let output = dl.apply(&signal);
        // First 2 samples should be zero (initial buffer)
        assert!((output[0] - 0.0).abs() < 1e-10);
        assert!((output[1] - 0.0).abs() < 1e-10);
        assert!((output[2] - 1.0).abs() < 1e-10);
        assert!((output[3] - 2.0).abs() < 1e-10);
        assert!((output[4] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn fixed_delay_reset() {
        let mut dl = FixedDelay::new(3);
        dl.process_sample(1.0);
        dl.process_sample(2.0);
        dl.reset();
        // After reset, should output zeros again
        let out = dl.process_sample(10.0);
        assert!((out - 0.0).abs() < 1e-10);
    }

    #[test]
    fn variable_delay_integer() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut vd = VariableDelay::new(10);
        let output = vd.apply(&signal, 3.0);
        // First 3 samples should be ~0
        for &v in &output[..3] {
            assert!(v.abs() < 1e-10);
        }
        assert!((output[3] - 1.0).abs() < 1e-10);
        assert!((output[4] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn variable_delay_fractional_interpolates() {
        let mut vd = VariableDelay::new(10);
        // Push known values
        vd.process_sample(0.0, 2.0);
        vd.process_sample(10.0, 2.0);
        vd.process_sample(20.0, 2.0);
        // Now with delay=1.5, should interpolate between sample at pos-1 and pos-2
        let out = vd.process_sample(30.0, 1.5);
        // Delay 1.5 from current: between 20.0 (delay 1) and 10.0 (delay 2) = 15.0
        assert!((out - 15.0).abs() < 1e-10);
    }

    #[test]
    fn fifo_delay_returns_old_blocks() {
        let mut fifo = FifoDelay::new(2);
        let out1 = fifo.push_block(vec![1.0, 2.0]);
        assert!(out1.is_empty()); // First block: buffer was empty
        let out2 = fifo.push_block(vec![3.0, 4.0]);
        assert!(out2.is_empty()); // Second block: still filling
        let out3 = fifo.push_block(vec![5.0, 6.0]);
        assert_eq!(out3, vec![1.0, 2.0]); // Now we get the first block back
    }

    #[test]
    fn apply_fixed_delay_convenience() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let delayed = apply_fixed_delay(&signal, 1);
        assert!((delayed[0] - 0.0).abs() < 1e-10);
        assert!((delayed[1] - 1.0).abs() < 1e-10);
        assert!((delayed[2] - 2.0).abs() < 1e-10);
        assert!((delayed[3] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn tapped_delay_line_single_tap() {
        // Tap at position 0 with gain 1.0 should pass through directly
        let mut tdl = TappedDelayLine::new(4, &[0], &[1.0]);
        let out = tdl.apply(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(out, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn tapped_delay_line_two_taps() {
        // Tap at 0 (gain 1.0) + tap at 2 (gain 0.5)
        let mut tdl = TappedDelayLine::new(4, &[0, 2], &[1.0, 0.5]);
        let out = tdl.apply(&[1.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10); // tap0: 1.0, tap2: 0
        assert!((out[1] - 0.0).abs() < 1e-10);
        assert!((out[2] - 0.5).abs() < 1e-10); // tap0: 0, tap2: 1.0*0.5
    }

    #[test]
    fn echo_single_impulse() {
        let mut echo = EchoGenerator::new(3, 0.5, EchoType::Single);
        let out = echo.apply(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[3] - 0.5).abs() < 1e-10); // single echo
        assert!((out[4] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn echo_feedback_repeats() {
        let mut echo = EchoGenerator::new(2, 0.5, EchoType::Feedback);
        let out = echo.apply(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[2] - 0.5).abs() < 1e-10);  // first echo
        assert!((out[4] - 0.25).abs() < 1e-10); // second echo (feedback)
    }

    #[test]
    fn echo_reset() {
        let mut echo = EchoGenerator::new(2, 0.5, EchoType::Feedback);
        echo.apply(&[1.0, 0.0]);
        echo.reset();
        let out = echo.process_sample(0.0);
        assert!((out - 0.0).abs() < 1e-10);
    }
}
