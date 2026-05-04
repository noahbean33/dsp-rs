use std::f64::consts::PI;

use super::fft::Complex;

/// Calculate FIR low-pass filter coefficients using the windowed-sinc method
/// with a Hamming window.
///
/// `taps` – number of filter coefficients (filter order + 1).
/// `cutoff` – normalised cutoff frequency (0.0 … 0.5 of Nyquist).
#[must_use]
pub fn design_lowpass(taps: usize, cutoff: f64) -> Vec<f64> {
    if taps <= 1 {
        return Vec::new();
    }
    let m = taps - 1;
    let mut coeffs = Vec::with_capacity(taps);
    let mut sum = 0.0;
    for n in 0..taps {
        let pos = n as f64 - m as f64 / 2.0;
        let ideal = if pos.abs() < 1e-6 {
            1.0
        } else {
            (PI * cutoff * pos).sin() / (PI * cutoff * pos)
        };
        let window = 0.54 - 0.46 * (2.0 * PI * n as f64 / m as f64).cos();
        let c = ideal * window;
        coeffs.push(c);
        sum += c;
    }
    if sum.abs() > 1e-9 {
        for c in &mut coeffs {
            *c /= sum;
        }
    }
    coeffs
}

/// Calculate FIR high-pass filter coefficients via spectral inversion of a
/// windowed-sinc low-pass filter.
///
/// `taps` – number of filter coefficients (must be odd for a type-I high-pass).
/// `cutoff` – normalised cutoff frequency (0.0 … 0.5 of Nyquist).
#[must_use]
pub fn design_highpass(taps: usize, cutoff: f64) -> Vec<f64> {
    let mut lp = design_lowpass(taps, cutoff);
    if lp.is_empty() {
        return lp;
    }
    // Spectral inversion: negate all coefficients, add 1 to centre tap
    for c in lp.iter_mut() {
        *c = -*c;
    }
    let centre = (taps - 1) / 2;
    lp[centre] += 1.0;
    lp
}

/// Calculate FIR band-pass filter coefficients using the windowed-sinc method
/// with a Hamming window.
///
/// `taps` – number of filter coefficients (filter order + 1).
/// `low_cutoff` – normalised lower cutoff frequency (0.0 … 0.5).
/// `high_cutoff` – normalised upper cutoff frequency (low_cutoff … 0.5).
#[must_use]
pub fn design_bandpass(taps: usize, low_cutoff: f64, high_cutoff: f64) -> Vec<f64> {
    if taps <= 1 || low_cutoff >= high_cutoff {
        return Vec::new();
    }
    let m = taps - 1;
    let mut coeffs = Vec::with_capacity(taps);
    let mut sum = 0.0;
    for n in 0..taps {
        let pos = n as f64 - m as f64 / 2.0;
        let ideal = if pos.abs() < 1e-6 {
            2.0 * (high_cutoff - low_cutoff)
        } else {
            (PI * high_cutoff * pos).sin() / (PI * pos)
                - (PI * low_cutoff * pos).sin() / (PI * pos)
        };
        let window = 0.54 - 0.46 * (2.0 * PI * n as f64 / m as f64).cos();
        let c = ideal * window;
        coeffs.push(c);
        sum += c;
    }
    if sum.abs() > 1e-9 {
        for c in &mut coeffs {
            *c /= sum;
        }
    }
    coeffs
}

/// Calculate FIR band-stop (notch) filter coefficients using the windowed-sinc
/// method with a Hamming window.
///
/// `taps` – number of filter coefficients (filter order + 1).
/// `low_cutoff` – normalised lower cutoff frequency (0.0 … 0.5).
/// `high_cutoff` – normalised upper cutoff frequency (low_cutoff … 0.5).
#[must_use]
pub fn design_bandstop(taps: usize, low_cutoff: f64, high_cutoff: f64) -> Vec<f64> {
    if taps <= 1 || low_cutoff >= high_cutoff {
        return Vec::new();
    }
    let m = taps - 1;
    let mut coeffs = Vec::with_capacity(taps);
    let mut sum = 0.0;
    for n in 0..taps {
        let pos = n as f64 - m as f64 / 2.0;
        let ideal = if pos.abs() < 1e-6 {
            1.0 - 2.0 * (high_cutoff - low_cutoff)
        } else {
            (PI * low_cutoff * pos).sin() / (PI * pos)
                - (PI * high_cutoff * pos).sin() / (PI * pos)
        };
        let window = 0.54 - 0.46 * (2.0 * PI * n as f64 / m as f64).cos();
        let c = ideal * window;
        coeffs.push(c);
        sum += c;
    }
    if sum.abs() > 1e-9 {
        for c in &mut coeffs {
            *c /= sum;
        }
    }
    coeffs
}

/// Apply a FIR filter to an input signal via direct convolution (valid mode).
///
/// Output length = `signal.len() - coeffs.len() + 1`.
#[must_use]
pub fn apply(signal: &[f64], coeffs: &[f64]) -> Vec<f64> {
    if signal.len() < coeffs.len() || coeffs.is_empty() {
        return Vec::new();
    }
    let out_size = signal.len() - coeffs.len() + 1;
    let mut output = vec![0.0; out_size];
    for n in 0..out_size {
        let mut acc = 0.0;
        for k in 0..coeffs.len() {
            acc += signal[n + k] * coeffs[k];
        }
        output[n] = acc;
    }
    output
}

/// Apply a FIR filter in full-length mode (output length = signal length).
///
/// Past samples that fall before the signal start are treated as zero.
#[must_use]
pub fn apply_full(signal: &[f64], coeffs: &[f64]) -> Vec<f64> {
    if signal.is_empty() || coeffs.is_empty() {
        return Vec::new();
    }
    let mut output = vec![0.0; signal.len()];
    for i in 0..signal.len() {
        let mut acc = 0.0;
        for (j, &c) in coeffs.iter().enumerate() {
            if i >= j {
                acc += signal[i - j] * c;
            }
        }
        output[i] = acc;
    }
    output
}

/// Compute the complex frequency response H(e^{jω}) of a FIR filter at a
/// given frequency.
#[must_use]
pub fn frequency_response(coeffs: &[f64], frequency: f64, sampling_rate: f64) -> Complex {
    let mut response = Complex::zero();
    for (n, &c) in coeffs.iter().enumerate() {
        let angle = -2.0 * PI * frequency * n as f64 / sampling_rate;
        response += Complex::new(c, 0.0) * Complex::new(angle.cos(), angle.sin());
    }
    response
}

/// Quantise FIR coefficients to fixed-point representation.
#[must_use]
pub fn quantize_coefficients(coeffs: &[f64], q_factor: i32) -> Vec<i32> {
    coeffs
        .iter()
        .map(|&c| (c * f64::from(q_factor)).round() as i32)
        .collect()
}

// ─── Streaming / Stateful FIR Filter ─────────────────────────────────────────

/// A stateful FIR filter that processes one sample at a time using a circular
/// buffer, retaining state between calls.
pub struct StreamingFir {
    coefficients: Vec<f64>,
    buffer: Vec<f64>,
    index: usize,
}

impl StreamingFir {
    /// Create a new streaming FIR filter with the given coefficients.
    #[must_use]
    pub fn new(coefficients: &[f64]) -> Self {
        let len = coefficients.len();
        Self {
            coefficients: coefficients.to_vec(),
            buffer: vec![0.0; len],
            index: 0,
        }
    }

    /// Reset internal buffer to zero.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
    }

    /// Process a single input sample and return the filtered output.
    pub fn process_sample(&mut self, sample: f64) -> f64 {
        let len = self.coefficients.len();
        if len == 0 {
            return 0.0;
        }
        // Write sample into circular buffer
        self.buffer[self.index] = sample;

        // Convolve: walk backwards through the buffer using the circular index
        let mut acc = 0.0;
        let mut buf_idx = self.index;
        for k in 0..len {
            acc += self.coefficients[k] * self.buffer[buf_idx];
            // Wrap-around decrement
            if buf_idx == 0 {
                buf_idx = len - 1;
            } else {
                buf_idx -= 1;
            }
        }

        // Advance circular index
        self.index = (self.index + 1) % len;

        acc
    }

    /// Process an entire signal through the streaming filter.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&s| self.process_sample(s)).collect()
    }
}

// ─── Comb Filter ──────────────────────────────────────────────────────────────

/// Feed-forward comb filter.
///
/// `y[n] = x[n] + α * x[n - delay]`
///
/// Creates a series of notches in the frequency response spaced at
/// `Fs / delay` Hz.
pub struct CombFilter {
    buffer: Vec<f64>,
    write_pos: usize,
    delay: usize,
    alpha: f64,
}

impl CombFilter {
    /// Create a new feed-forward comb filter.
    ///
    /// * `delay` – delay in samples
    /// * `alpha` – feedforward gain coefficient
    #[must_use]
    pub fn new(delay: usize, alpha: f64) -> Self {
        let d = delay.max(1);
        Self {
            buffer: vec![0.0; d],
            write_pos: 0,
            delay: d,
            alpha,
        }
    }

    /// Process a single sample.
    pub fn process_sample(&mut self, input: f64) -> f64 {
        let delayed = self.buffer[self.write_pos];
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.delay;
        input + self.alpha * delayed
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

/// Feedback comb filter (IIR comb).
///
/// `y[n] = x[n] + α * y[n - delay]`
pub struct FeedbackCombFilter {
    buffer: Vec<f64>,
    write_pos: usize,
    delay: usize,
    alpha: f64,
}

impl FeedbackCombFilter {
    #[must_use]
    pub fn new(delay: usize, alpha: f64) -> Self {
        let d = delay.max(1);
        Self {
            buffer: vec![0.0; d],
            write_pos: 0,
            delay: d,
            alpha,
        }
    }

    pub fn process_sample(&mut self, input: f64) -> f64 {
        let delayed = self.buffer[self.write_pos];
        let output = input + self.alpha * delayed;
        self.buffer[self.write_pos] = output;
        self.write_pos = (self.write_pos + 1) % self.delay;
        output
    }

    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

// ─── Raised-Cosine / Root-Raised-Cosine Filters ──────────────────────────────

/// Design a raised-cosine FIR pulse-shaping filter.
///
/// * `taps` – number of filter coefficients (must be odd)
/// * `symbol_period` – symbol period in samples (Ts)
/// * `alpha` – roll-off factor (0 ≤ α ≤ 1)
#[must_use]
pub fn design_raised_cosine(taps: usize, symbol_period: f64, alpha: f64) -> Vec<f64> {
    if taps == 0 || symbol_period <= 0.0 {
        return Vec::new();
    }
    let m = (taps - 1) as f64 / 2.0;
    let mut coeffs = Vec::with_capacity(taps);
    let mut energy = 0.0;

    for n in 0..taps {
        let t = (n as f64 - m) / symbol_period;
        let h = if t.abs() < 1e-12 {
            1.0
        } else if alpha > 0.0 && (1.0 - (2.0 * alpha * t).powi(2)).abs() < 1e-12 {
            alpha / 2.0 * (PI / (2.0 * alpha)).sin()
        } else {
            let sinc = (PI * t).sin() / (PI * t);
            let cos_term = (PI * alpha * t).cos();
            let denom = 1.0 - (2.0 * alpha * t).powi(2);
            sinc * cos_term / denom
        };
        coeffs.push(h);
        energy += h * h;
    }

    // Normalise energy
    if energy > 1e-30 {
        let scale = 1.0 / energy.sqrt();
        for c in &mut coeffs {
            *c *= scale;
        }
    }
    coeffs
}

/// Design a root-raised-cosine FIR pulse-shaping filter.
///
/// * `taps` – number of filter coefficients (must be odd)
/// * `symbol_period` – symbol period in samples (Ts)
/// * `alpha` – roll-off factor (0 ≤ α ≤ 1)
#[must_use]
pub fn design_root_raised_cosine(taps: usize, symbol_period: f64, alpha: f64) -> Vec<f64> {
    if taps == 0 || symbol_period <= 0.0 {
        return Vec::new();
    }
    let m = (taps - 1) as f64 / 2.0;
    let mut coeffs = Vec::with_capacity(taps);
    let mut energy = 0.0;

    for n in 0..taps {
        let t = (n as f64 - m) / symbol_period;
        let h = if t.abs() < 1e-12 {
            1.0 - alpha + 4.0 * alpha / PI
        } else if alpha > 0.0 && (1.0 - (4.0 * alpha * t).powi(2)).abs() < 1e-12 {
            alpha / (2.0_f64.sqrt())
                * ((1.0 + 2.0 / PI) * (PI / (4.0 * alpha)).sin()
                    + (1.0 - 2.0 / PI) * (PI / (4.0 * alpha)).cos())
        } else {
            let num = (PI * t * (1.0 - alpha)).sin()
                + 4.0 * alpha * t * (PI * t * (1.0 + alpha)).cos();
            let denom = PI * t * (1.0 - (4.0 * alpha * t).powi(2));
            num / denom
        };
        coeffs.push(h);
        energy += h * h;
    }

    if energy > 1e-30 {
        let scale = 1.0 / energy.sqrt();
        for c in &mut coeffs {
            *c *= scale;
        }
    }
    coeffs
}

/// Design a Gaussian FIR filter.
///
/// * `taps` – number of filter coefficients
/// * `bt` – bandwidth-symbol time product (BT)
#[must_use]
pub fn design_gaussian(taps: usize, bt: f64) -> Vec<f64> {
    if taps == 0 || bt <= 0.0 {
        return Vec::new();
    }
    let m = (taps - 1) as f64 / 2.0;
    let sigma = (2.0 * PI * bt).recip() * (2.0_f64.ln()).sqrt();
    let mut coeffs = Vec::with_capacity(taps);
    let mut sum = 0.0;
    for n in 0..taps {
        let t = n as f64 - m;
        let h = (-0.5 * (t / sigma).powi(2)).exp();
        coeffs.push(h);
        sum += h;
    }
    if sum.abs() > 1e-30 {
        for c in &mut coeffs {
            *c /= sum;
        }
    }
    coeffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowpass_coefficients_sum_to_one() {
        let coeffs = design_lowpass(51, 0.25);
        let sum: f64 = coeffs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn apply_impulse_response() {
        let coeffs = vec![0.25, 0.5, 0.25];
        let signal = vec![0.0, 0.0, 1.0, 0.0, 0.0];
        let out = apply(&signal, &coeffs);
        assert_eq!(out.len(), 3);
        assert!((out[0] - 0.25).abs() < 1e-10);
        assert!((out[1] - 0.5).abs() < 1e-10);
        assert!((out[2] - 0.25).abs() < 1e-10);
    }

    #[test]
    fn quantize_roundtrip() {
        let coeffs = vec![0.25, 0.5, 0.25];
        let q = quantize_coefficients(&coeffs, 1000);
        assert_eq!(q, vec![250, 500, 250]);
    }

    #[test]
    fn streaming_fir_matches_batch() {
        let coeffs = vec![0.25, 0.5, 0.25];
        let signal = vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
        let batch = apply_full(&signal, &coeffs);
        let mut fir = StreamingFir::new(&coeffs);
        let streaming = fir.apply(&signal);
        assert_eq!(batch.len(), streaming.len());
        for (a, b) in batch.iter().zip(streaming.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn streaming_fir_sample_by_sample() {
        let coeffs = vec![1.0, 0.0, 0.0];
        let mut fir = StreamingFir::new(&coeffs);
        assert!((fir.process_sample(5.0) - 5.0).abs() < 1e-10);
        assert!((fir.process_sample(3.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn streaming_fir_reset() {
        let coeffs = vec![0.5, 0.5];
        let mut fir = StreamingFir::new(&coeffs);
        fir.process_sample(10.0);
        fir.reset();
        // After reset, buffer should be zeroed
        let out = fir.process_sample(2.0);
        assert!((out - 1.0).abs() < 1e-10); // 0.5*2 + 0.5*0
    }

    #[test]
    fn highpass_attenuates_dc() {
        let coeffs = design_highpass(51, 0.25);
        assert_eq!(coeffs.len(), 51);
        // Sum of coefficients should be near zero (rejects DC)
        let sum: f64 = coeffs.iter().sum();
        assert!(sum.abs() < 0.05);
    }

    #[test]
    fn bandpass_correct_length() {
        let coeffs = design_bandpass(51, 0.1, 0.3);
        assert_eq!(coeffs.len(), 51);
    }

    #[test]
    fn bandstop_correct_length() {
        let coeffs = design_bandstop(51, 0.1, 0.3);
        assert_eq!(coeffs.len(), 51);
    }

    #[test]
    fn bandpass_invalid_cutoffs_empty() {
        let coeffs = design_bandpass(51, 0.3, 0.1);
        assert!(coeffs.is_empty());
    }

    #[test]
    fn comb_filter_impulse_response() {
        let mut comb = CombFilter::new(3, 1.0);
        // Impulse: should appear at n=0 and echoed at n=3
        let out = comb.apply(&[1.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[1] - 0.0).abs() < 1e-10);
        assert!((out[2] - 0.0).abs() < 1e-10);
        assert!((out[3] - 1.0).abs() < 1e-10); // delayed copy
        assert!((out[4] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn feedback_comb_filter_grows() {
        let mut comb = FeedbackCombFilter::new(2, 0.5);
        let out = comb.apply(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[2] - 0.5).abs() < 1e-10);  // feedback of out[0]
        assert!((out[4] - 0.25).abs() < 1e-10); // feedback of out[2]
    }

    #[test]
    fn raised_cosine_correct_length() {
        let coeffs = design_raised_cosine(65, 8.0, 0.35);
        assert_eq!(coeffs.len(), 65);
        // Should have finite values
        assert!(coeffs.iter().all(|c| c.is_finite()));
    }

    #[test]
    fn root_raised_cosine_correct_length() {
        let coeffs = design_root_raised_cosine(65, 8.0, 0.35);
        assert_eq!(coeffs.len(), 65);
        assert!(coeffs.iter().all(|c| c.is_finite()));
    }

    #[test]
    fn gaussian_filter_sums_to_one() {
        let coeffs = design_gaussian(31, 0.3);
        assert_eq!(coeffs.len(), 31);
        let sum: f64 = coeffs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn gaussian_filter_symmetric() {
        let coeffs = design_gaussian(31, 0.3);
        for i in 0..coeffs.len() / 2 {
            assert!((coeffs[i] - coeffs[coeffs.len() - 1 - i]).abs() < 1e-10);
        }
    }
}
