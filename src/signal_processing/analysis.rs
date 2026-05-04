/// Moving average filter with a sliding window.
///
/// Output length = `input.len() - window_size + 1`.
#[must_use]
pub fn moving_average(input: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || input.len() < window_size {
        return Vec::new();
    }
    let mut output = Vec::with_capacity(input.len() - window_size + 1);
    let mut sum: f64 = input[..window_size].iter().sum();
    output.push(sum / window_size as f64);
    for i in window_size..input.len() {
        sum += input[i] - input[i - window_size];
        output.push(sum / window_size as f64);
    }
    output
}

/// Centered (symmetric) moving average filter.
///
/// For each sample, averages `window_size` points centered on that sample.
/// `window_size` must be odd. Samples near edges use clamped (replicated) boundary.
#[must_use]
pub fn centered_moving_average(signal: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || window_size % 2 == 0 || signal.is_empty() {
        return Vec::new();
    }
    let half = window_size / 2;
    let n = signal.len();
    let mut output = Vec::with_capacity(n);
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..window_size {
            let idx_signed = i as isize + j as isize - half as isize;
            let idx = idx_signed.max(0).min(n as isize - 1) as usize;
            sum += signal[idx];
        }
        output.push(sum / window_size as f64);
    }
    output
}

/// Finite difference (first derivative approximation).
///
/// `interval` is the sampling interval Δt.
#[must_use]
pub fn finite_difference(signal: &[f64], interval: f64) -> Vec<f64> {
    if signal.len() < 2 || interval.abs() < 1e-30 {
        return Vec::new();
    }
    signal
        .windows(2)
        .map(|w| (w[1] - w[0]) / interval)
        .collect()
}

/// Count zero crossings in a signal.
#[must_use]
pub fn count_zero_crossings(signal: &[f64]) -> usize {
    let mut count = 0;
    for i in 1..signal.len() {
        if (signal[i - 1] < 0.0 && signal[i] >= 0.0)
            || (signal[i - 1] > 0.0 && signal[i] <= 0.0)
        {
            count += 1;
        }
    }
    count
}

/// Adaptive peak detection: finds local maxima exceeding an adaptive
/// threshold of `mean + threshold_multiplier * stddev`.
#[must_use]
pub fn detect_adaptive_peaks(signal: &[f64], threshold_multiplier: f64) -> Vec<usize> {
    if signal.len() < 3 {
        return Vec::new();
    }
    let n = signal.len() as f64;
    let mean: f64 = signal.iter().sum::<f64>() / n;
    let variance: f64 = signal.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let stddev = variance.sqrt();
    let adaptive_threshold = mean + threshold_multiplier * stddev;

    let mut peaks = Vec::new();
    for i in 1..signal.len() - 1 {
        if signal[i] > signal[i - 1]
            && signal[i] > signal[i + 1]
            && signal[i] > adaptive_threshold
        {
            peaks.push(i);
        }
    }
    peaks
}

/// Segment events based on an amplitude threshold.
///
/// Returns a list of `(start, end)` index pairs.
#[must_use]
pub fn segment_events(signal: &[f64], amplitude_threshold: f64) -> Vec<(usize, usize)> {
    let mut segments = Vec::new();
    let mut active = false;
    let mut start = 0;

    for (i, &v) in signal.iter().enumerate() {
        if !active && v.abs() > amplitude_threshold {
            active = true;
            start = i;
        } else if active && v.abs() <= amplitude_threshold {
            segments.push((start, i));
            active = false;
        }
    }
    if active {
        segments.push((start, signal.len() - 1));
    }
    segments
}

/// Sliding-window median filter for impulse noise suppression.
///
/// `window_size` must be odd.
#[must_use]
pub fn median_filter(signal: &[f64], window_size: usize) -> Vec<f64> {
    if window_size % 2 == 0 || window_size == 0 || signal.len() < window_size {
        return Vec::new();
    }
    let half = window_size / 2;
    let mut filtered = Vec::with_capacity(signal.len());
    for i in 0..signal.len() {
        let mut window = Vec::with_capacity(window_size);
        for j in 0..window_size {
            let idx = (i as isize + j as isize - half as isize)
                .max(0)
                .min(signal.len() as isize - 1) as usize;
            window.push(signal[idx]);
        }
        window.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        filtered.push(window[half]);
    }
    filtered
}

/// Linear interpolation at a fractional index.
#[must_use]
pub fn linear_interpolation(signal: &[f64], fractional_index: f64) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    let index = fractional_index as isize;
    if index < 0 {
        return signal[0];
    }
    if index + 1 >= signal.len() as isize {
        return *signal.last().unwrap_or(&0.0);
    }
    let alpha = fractional_index - index as f64;
    let i = index as usize;
    (1.0 - alpha) * signal[i] + alpha * signal[i + 1]
}

/// Estimate the synchronisation offset (delay) between two signals using
/// cross-correlation.
#[must_use]
pub fn synchronization_offset(stream1: &[f64], stream2: &[f64], max_delay: usize) -> isize {
    if stream1.is_empty() || stream2.is_empty() {
        return 0;
    }
    let length = stream1.len();
    let mut best_offset: isize = 0;
    let mut max_correlation = f64::NEG_INFINITY;

    let max_delay = max_delay as isize;
    for delay in -max_delay..=max_delay {
        let mut correlation = 0.0;
        for i in 0..length {
            let j = i as isize + delay;
            if j >= 0 && (j as usize) < length {
                correlation += stream1[i] * stream2[j as usize];
            }
        }
        if correlation > max_correlation {
            max_correlation = correlation;
            best_offset = delay;
        }
    }
    best_offset
}

/// Compute the signal energy: Σ x[n]².
#[must_use]
pub fn signal_energy(signal: &[f64]) -> f64 {
    signal.iter().map(|&x| x * x).sum()
}

/// Compute the RMS (root mean square) of a signal.
#[must_use]
pub fn rms(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    (signal_energy(signal) / signal.len() as f64).sqrt()
}

// ─── DC Removal Filter ───────────────────────────────────────────────────────

/// Remove DC offset from a signal using a single-pole high-pass IIR filter.
///
/// Implements: `y[n] = x[n] - x[n-1] + alpha * y[n-1]`
/// where `alpha = (1 - sin(2π * cutoff_hz / fs)) / cos(2π * cutoff_hz / fs)`.
///
/// `cutoff_hz` – cutoff frequency in Hz (typically very low, e.g. 5–20 Hz).
/// `sample_rate` – sampling frequency in Hz.
#[must_use]
pub fn dc_remove(signal: &[f64], cutoff_hz: f64, sample_rate: f64) -> Vec<f64> {
    if signal.is_empty() || sample_rate <= 0.0 || cutoff_hz <= 0.0 {
        return signal.to_vec();
    }
    let omega = 2.0 * std::f64::consts::PI * cutoff_hz / sample_rate;
    let alpha = (1.0 - omega.sin()) / omega.cos();

    let mut output = vec![0.0; signal.len()];
    output[0] = signal[0];
    for n in 1..signal.len() {
        output[n] = signal[n] - signal[n - 1] + alpha * output[n - 1];
    }
    output
}

// ─── Envelope Detection ───────────────────────────────────────────────────────

/// Envelope follower with configurable attack and decay coefficients.
///
/// Tracks the amplitude envelope of a signal using a one-pole filter:
/// - When `|x[n]| > env[n-1]`: `env[n] = attack * |x[n]| + (1 - attack) * env[n-1]`
/// - Otherwise:                 `env[n] = decay  * |x[n]| + (1 - decay)  * env[n-1]`
///
/// `attack` and `decay` are in the range `(0, 1]`. Larger values track faster.
#[must_use]
pub fn envelope(signal: &[f64], attack: f64, decay: f64) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }
    let mut env = Vec::with_capacity(signal.len());
    let mut state = 0.0_f64;
    for &x in signal {
        let abs_x = x.abs();
        if abs_x > state {
            state = attack * abs_x + (1.0 - attack) * state;
        } else {
            state = decay * abs_x + (1.0 - decay) * state;
        }
        env.push(state);
    }
    env
}

/// RMS envelope follower.
///
/// Computes a smoothed RMS envelope using a one-pole filter on `x²`:
/// `rms²[n] = α * x[n]² + (1 - α) * rms²[n-1]`,  output = √(rms²[n]).
///
/// `alpha` controls the smoothing (higher = faster tracking).
#[must_use]
pub fn envelope_rms(signal: &[f64], alpha: f64) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }
    let mut env = Vec::with_capacity(signal.len());
    let mut state = 0.0_f64;
    for &x in signal {
        state = alpha * x * x + (1.0 - alpha) * state;
        env.push(state.sqrt());
    }
    env
}

/// Hilbert envelope (analytic signal magnitude).
///
/// Computes the instantaneous amplitude envelope by:
/// 1. Taking the FFT of the signal.
/// 2. Zeroing the negative-frequency components (creating the analytic signal).
/// 3. Taking the inverse FFT.
/// 4. Returning the magnitude of the complex analytic signal.
///
/// The signal is zero-padded to the next power of 2.
#[must_use]
pub fn envelope_hilbert(signal: &[f64]) -> Vec<f64> {
    use super::fft::{Complex, fft, ifft, real_to_complex, zero_pad_to_power_of_2};

    if signal.is_empty() {
        return Vec::new();
    }
    let orig_len = signal.len();
    let mut data = zero_pad_to_power_of_2(&real_to_complex(signal));
    let n = data.len();
    fft(&mut data);

    // Build analytic signal: keep DC and Nyquist, double positive freqs, zero negative
    // Bin 0 (DC) stays, bins 1..N/2-1 doubled, bin N/2 stays, bins N/2+1..N-1 zeroed
    if n > 1 {
        for i in 1..n / 2 {
            data[i] = data[i] * 2.0;
        }
        for i in (n / 2 + 1)..n {
            data[i] = Complex::zero();
        }
    }

    ifft(&mut data);

    data[..orig_len].iter().map(|c| c.norm()).collect()
}

// ─── Signal Integration ───────────────────────────────────────────────────────

/// Numerical integration (cumulative trapezoidal rule).
///
/// `interval` is the sampling interval Δt.
/// `y[n] = y[n-1] + Δt * (x[n] + x[n-1]) / 2`
#[must_use]
pub fn integrate(signal: &[f64], interval: f64) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }
    let mut output = Vec::with_capacity(signal.len());
    output.push(0.0);
    for i in 1..signal.len() {
        let prev = output[i - 1];
        output.push(prev + interval * (signal[i] + signal[i - 1]) / 2.0);
    }
    output
}

// ─── Threshold / Clamp / Limit ────────────────────────────────────────────────

/// Threshold mode for signal processing utilities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThresholdMode {
    /// Zero values below the threshold.
    AboveZero,
    /// Zero values above the threshold.
    BelowZero,
    /// Replace values below the threshold with the threshold.
    AboveClamp,
    /// Replace values above the threshold with the threshold.
    BelowClamp,
}

/// Apply a threshold to a signal.
///
/// Depending on the mode, values that do not meet the threshold condition
/// are either zeroed out or clamped.
#[must_use]
pub fn threshold(signal: &[f64], thresh: f64, mode: ThresholdMode) -> Vec<f64> {
    signal
        .iter()
        .map(|&x| match mode {
            ThresholdMode::AboveZero => {
                if x >= thresh { x } else { 0.0 }
            }
            ThresholdMode::BelowZero => {
                if x <= thresh { x } else { 0.0 }
            }
            ThresholdMode::AboveClamp => {
                if x >= thresh { x } else { thresh }
            }
            ThresholdMode::BelowClamp => {
                if x <= thresh { x } else { thresh }
            }
        })
        .collect()
}

/// Soft threshold: shrink values towards zero.
///
/// `y = sign(x) * max(|x| - thresh, 0)`
#[must_use]
pub fn soft_threshold(signal: &[f64], thresh: f64) -> Vec<f64> {
    signal
        .iter()
        .map(|&x| {
            let abs = x.abs();
            if abs > thresh {
                x.signum() * (abs - thresh)
            } else {
                0.0
            }
        })
        .collect()
}

/// Clamp (limit) signal values to `[min_val, max_val]`.
#[must_use]
pub fn clamp(signal: &[f64], min_val: f64, max_val: f64) -> Vec<f64> {
    signal
        .iter()
        .map(|&x| x.max(min_val).min(max_val))
        .collect()
}

/// Count how many values exceed the threshold.
#[must_use]
pub fn count_over_threshold(signal: &[f64], thresh: f64) -> usize {
    signal.iter().filter(|&&x| x > thresh).count()
}

/// Count how many absolute values exceed the threshold.
#[must_use]
pub fn count_abs_over_threshold(signal: &[f64], thresh: f64) -> usize {
    signal.iter().filter(|&&x| x.abs() > thresh).count()
}

/// Element-wise maximum of two signals.
#[must_use]
pub fn select_max(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.max(y)).collect()
}

/// Element-wise minimum of two signals.
#[must_use]
pub fn select_min(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.min(y)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_average_constant_signal() {
        let signal = vec![5.0; 10];
        let avg = moving_average(&signal, 3);
        assert!(avg.iter().all(|&v| (v - 5.0).abs() < 1e-10));
    }

    #[test]
    fn zero_crossings_sine() {
        let signal: Vec<f64> = (0..100)
            .map(|i| (2.0 * std::f64::consts::PI * 5.0 * i as f64 / 100.0).sin())
            .collect();
        let crossings = count_zero_crossings(&signal);
        assert!(crossings >= 8 && crossings <= 12);
    }

    #[test]
    fn median_filter_removes_spike() {
        let mut signal = vec![1.0; 11];
        signal[5] = 100.0;
        let filtered = median_filter(&signal, 3);
        assert!((filtered[5] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn linear_interp_midpoint() {
        let signal = vec![0.0, 10.0];
        assert!((linear_interpolation(&signal, 0.5) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn sync_offset_identity() {
        let signal = vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0];
        let offset = synchronization_offset(&signal, &signal, 3);
        assert_eq!(offset, 0);
    }

    #[test]
    fn centered_moving_average_constant() {
        let signal = vec![5.0; 10];
        let avg = centered_moving_average(&signal, 3);
        assert_eq!(avg.len(), 10);
        assert!(avg.iter().all(|&v| (v - 5.0).abs() < 1e-10));
    }

    #[test]
    fn centered_moving_average_smooths_spike() {
        let mut signal = vec![0.0; 7];
        signal[3] = 7.0;
        let avg = centered_moving_average(&signal, 3);
        // Centre point: (0 + 7 + 0) / 3
        assert!((avg[3] - 7.0 / 3.0).abs() < 1e-10);
        // Neighbours: (0 + 0 + 7) / 3  and  (7 + 0 + 0) / 3
        assert!((avg[2] - 7.0 / 3.0).abs() < 1e-10);
        assert!((avg[4] - 7.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn centered_moving_average_even_window_returns_empty() {
        let signal = vec![1.0, 2.0, 3.0];
        assert!(centered_moving_average(&signal, 2).is_empty());
    }

    #[test]
    fn dc_remove_removes_offset() {
        // Signal with DC offset of 5.0
        let signal: Vec<f64> = (0..1000)
            .map(|i| 5.0 + (2.0 * std::f64::consts::PI * 50.0 * i as f64 / 1000.0).sin())
            .collect();
        let filtered = dc_remove(&signal, 10.0, 1000.0);
        // After settling, mean should be near zero
        let tail_mean: f64 = filtered[500..].iter().sum::<f64>() / 500.0;
        assert!(tail_mean.abs() < 0.5);
    }

    #[test]
    fn dc_remove_preserves_ac() {
        let signal: Vec<f64> = (0..1000)
            .map(|i| (2.0 * std::f64::consts::PI * 100.0 * i as f64 / 1000.0).sin())
            .collect();
        let filtered = dc_remove(&signal, 5.0, 1000.0);
        // AC energy should be mostly preserved
        let in_energy: f64 = signal[200..].iter().map(|x| x * x).sum();
        let out_energy: f64 = filtered[200..].iter().map(|x| x * x).sum();
        assert!((out_energy / in_energy - 1.0).abs() < 0.1);
    }

    #[test]
    fn dc_remove_empty_passthrough() {
        let empty: Vec<f64> = Vec::new();
        assert!(dc_remove(&empty, 10.0, 1000.0).is_empty());
    }

    #[test]
    fn envelope_tracks_amplitude() {
        let pi = std::f64::consts::PI;
        // AM signal: carrier modulated by a slow envelope
        let signal: Vec<f64> = (0..1000)
            .map(|i| {
                let t = i as f64 / 1000.0;
                let carrier = (2.0 * pi * 100.0 * t).sin();
                let mod_env = 0.5 + 0.5 * (2.0 * pi * 5.0 * t).sin();
                carrier * mod_env
            })
            .collect();
        let env = envelope(&signal, 0.1, 0.01);
        assert_eq!(env.len(), signal.len());
        // Envelope should be non-negative
        assert!(env.iter().all(|&v| v >= -1e-10));
    }

    #[test]
    fn envelope_rms_positive() {
        let pi = std::f64::consts::PI;
        let signal: Vec<f64> = (0..200)
            .map(|i| (2.0 * pi * 50.0 * i as f64 / 1000.0).sin())
            .collect();
        let env = envelope_rms(&signal, 0.05);
        assert_eq!(env.len(), signal.len());
        assert!(env.iter().all(|&v| v >= 0.0));
        // RMS should settle near 1/√2 ≈ 0.707 for a unit sine
        let tail_mean: f64 = env[100..].iter().sum::<f64>() / env[100..].len() as f64;
        assert!((tail_mean - 0.707).abs() < 0.2);
    }

    #[test]
    fn envelope_hilbert_of_sine() {
        let pi = std::f64::consts::PI;
        let n = 256;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * pi * 10.0 * i as f64 / n as f64).sin())
            .collect();
        let env = envelope_hilbert(&signal);
        assert_eq!(env.len(), n);
        // Hilbert envelope of a pure sine should be ~constant (~1.0)
        // Check the middle portion (edges have transient effects)
        let mid = &env[n / 4..3 * n / 4];
        let mean: f64 = mid.iter().sum::<f64>() / mid.len() as f64;
        assert!((mean - 1.0).abs() < 0.15);
    }

    #[test]
    fn integrate_constant() {
        // Integral of constant 2.0 with dt=0.1 over 10 samples
        let signal = vec![2.0; 10];
        let result = integrate(&signal, 0.1);
        assert_eq!(result.len(), 10);
        assert!((result[0] - 0.0).abs() < 1e-10);
        // After 9 intervals: 9 * 0.1 * 2.0 = 1.8
        assert!((result[9] - 1.8).abs() < 1e-10);
    }

    #[test]
    fn integrate_empty() {
        assert!(integrate(&[], 1.0).is_empty());
    }

    #[test]
    fn threshold_above_zero() {
        let signal = vec![-2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
        let out = threshold(&signal, 1.0, ThresholdMode::AboveZero);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn threshold_below_clamp() {
        let signal = vec![-2.0, 0.0, 5.0, 10.0];
        let out = threshold(&signal, 5.0, ThresholdMode::BelowClamp);
        assert_eq!(out, vec![-2.0, 0.0, 5.0, 5.0]);
    }

    #[test]
    fn soft_threshold_shrinks() {
        let signal = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
        let out = soft_threshold(&signal, 2.0);
        assert!((out[0] - (-1.0)).abs() < 1e-10);
        assert!((out[1] - 0.0).abs() < 1e-10);
        assert!((out[2] - 0.0).abs() < 1e-10);
        assert!((out[3] - 0.0).abs() < 1e-10);
        assert!((out[4] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn clamp_limits_values() {
        let signal = vec![-5.0, -1.0, 0.0, 1.0, 5.0];
        let out = clamp(&signal, -2.0, 2.0);
        assert_eq!(out, vec![-2.0, -1.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn count_over_threshold_works() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(count_over_threshold(&signal, 3.0), 2);
        assert_eq!(count_abs_over_threshold(&signal, 3.0), 2);
    }

    #[test]
    fn select_max_min() {
        let a = vec![1.0, 5.0, 3.0];
        let b = vec![4.0, 2.0, 6.0];
        assert_eq!(select_max(&a, &b), vec![4.0, 5.0, 6.0]);
        assert_eq!(select_min(&a, &b), vec![1.0, 2.0, 3.0]);
    }
}
