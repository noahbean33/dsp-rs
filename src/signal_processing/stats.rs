/// Compute the arithmetic mean of a signal.
#[must_use]
pub fn mean(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    signal.iter().sum::<f64>() / signal.len() as f64
}

/// Compute the variance of a signal.
///
/// Uses population variance (divide by N).
#[must_use]
pub fn variance(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    let m = mean(signal);
    signal.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / signal.len() as f64
}

/// Compute the sample variance of a signal (Bessel-corrected, divide by N-1).
#[must_use]
pub fn sample_variance(signal: &[f64]) -> f64 {
    if signal.len() < 2 {
        return 0.0;
    }
    let m = mean(signal);
    signal.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / (signal.len() - 1) as f64
}

/// Compute the standard deviation of a signal (population).
#[must_use]
pub fn std_dev(signal: &[f64]) -> f64 {
    variance(signal).sqrt()
}

/// Compute the sample standard deviation (Bessel-corrected).
#[must_use]
pub fn sample_std_dev(signal: &[f64]) -> f64 {
    sample_variance(signal).sqrt()
}

/// Find the minimum value in a signal.
#[must_use]
pub fn min(signal: &[f64]) -> f64 {
    signal
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}

/// Find the maximum value in a signal.
#[must_use]
pub fn max(signal: &[f64]) -> f64 {
    signal
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Compute the total energy of a signal: Σ x[n]².
#[must_use]
pub fn energy(signal: &[f64]) -> f64 {
    signal.iter().map(|&x| x * x).sum()
}

/// Compute the average power of a signal: (1/N) Σ x[n]².
#[must_use]
pub fn power(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    energy(signal) / signal.len() as f64
}

/// Compute the root mean square (RMS) of a signal.
#[must_use]
pub fn rms(signal: &[f64]) -> f64 {
    power(signal).sqrt()
}

/// Compute the peak-to-peak amplitude of a signal.
#[must_use]
pub fn peak_to_peak(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    max(signal) - min(signal)
}

/// Compute the crest factor (peak / RMS).
#[must_use]
pub fn crest_factor(signal: &[f64]) -> f64 {
    let r = rms(signal);
    if r < 1e-30 {
        return 0.0;
    }
    let peak = signal.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
    peak / r
}

// ─── dB Conversion Utilities ──────────────────────────────────────────────────

/// Convert an amplitude (voltage) value to decibels: 20 * log10(|amp|).
///
/// Returns `f64::NEG_INFINITY` for zero amplitude.
#[must_use]
pub fn amp_to_db(amp: f64) -> f64 {
    let abs_amp = amp.abs();
    if abs_amp < 1e-30 {
        return f64::NEG_INFINITY;
    }
    20.0 * abs_amp.log10()
}

/// Convert a decibel value to amplitude (voltage): 10^(dB / 20).
#[must_use]
pub fn db_to_amp(db: f64) -> f64 {
    10.0_f64.powf(db / 20.0)
}

/// Convert a power value to decibels: 10 * log10(|power|).
///
/// Returns `f64::NEG_INFINITY` for zero power.
#[must_use]
pub fn power_to_db(power: f64) -> f64 {
    let abs_power = power.abs();
    if abs_power < 1e-30 {
        return f64::NEG_INFINITY;
    }
    10.0 * abs_power.log10()
}

/// Convert a decibel value to power: 10^(dB / 10).
#[must_use]
pub fn db_to_power(db: f64) -> f64 {
    10.0_f64.powf(db / 10.0)
}

// ─── Histogram Functions ──────────────────────────────────────────────────────

/// Compute a histogram of `signal` values into `num_bins` equally-spaced bins.
///
/// Bins span `[min_val, max_val]`. Values outside the range are clamped to the
/// nearest bin. Returns a vector of counts (as `f64` for easy normalisation).
#[must_use]
pub fn histogram(signal: &[f64], min_val: f64, max_val: f64, num_bins: usize) -> Vec<f64> {
    if signal.is_empty() || num_bins == 0 || max_val <= min_val {
        return Vec::new();
    }
    let mut bins = vec![0.0; num_bins];
    let range = max_val - min_val;
    for &x in signal {
        let frac = (x - min_val) / range;
        let idx = (frac * num_bins as f64).floor() as isize;
        let idx = idx.max(0).min(num_bins as isize - 1) as usize;
        bins[idx] += 1.0;
    }
    bins
}

/// Compute a cumulative histogram of `signal` values.
///
/// Each bin `i` contains the count of samples in bins `0..=i`.
#[must_use]
pub fn histogram_cumulative(signal: &[f64], min_val: f64, max_val: f64, num_bins: usize) -> Vec<f64> {
    let hist = histogram(signal, min_val, max_val, num_bins);
    if hist.is_empty() {
        return Vec::new();
    }
    let mut cum = Vec::with_capacity(hist.len());
    let mut sum = 0.0;
    for &h in &hist {
        sum += h;
        cum.push(sum);
    }
    cum
}

/// Histogram equalisation: remap signal values so the histogram is approximately
/// uniform over `[0, new_peak]`.
#[must_use]
pub fn histogram_equalize(signal: &[f64], new_peak: f64) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }
    let n = signal.len() as f64;
    let min_v = min(signal);
    let max_v = max(signal);
    if (max_v - min_v).abs() < 1e-30 {
        return vec![new_peak / 2.0; signal.len()];
    }
    let num_bins = 256;
    let cum = histogram_cumulative(signal, min_v, max_v, num_bins);
    let range = max_v - min_v;

    signal
        .iter()
        .map(|&x| {
            let frac = (x - min_v) / range;
            let idx = (frac * num_bins as f64).floor() as isize;
            let idx = idx.max(0).min(num_bins as isize - 1) as usize;
            cum[idx] / n * new_peak
        })
        .collect()
}

// ─── Peak-to-Average Ratio ────────────────────────────────────────────────────

/// Peak-to-average ratio (PAR).
///
/// `PAR = max(|x|) / mean(|x|)`
#[must_use]
pub fn peak_to_average_ratio(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    let peak = signal.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
    let avg = signal.iter().map(|x| x.abs()).sum::<f64>() / signal.len() as f64;
    if avg < 1e-30 {
        return 0.0;
    }
    peak / avg
}

/// Peak-to-average power ratio (PAPR).
///
/// `PAPR = max(|x|²) / mean(|x|²)`
#[must_use]
pub fn peak_to_average_power_ratio(signal: &[f64]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    let peak_sq = signal.iter().map(|x| x * x).fold(0.0_f64, f64::max);
    let avg_sq = signal.iter().map(|x| x * x).sum::<f64>() / signal.len() as f64;
    if avg_sq < 1e-30 {
        return 0.0;
    }
    peak_sq / avg_sq
}

/// Peak-to-average power ratio in decibels.
///
/// `PAPR_dB = 10 * log10(PAPR)`
#[must_use]
pub fn peak_to_average_power_ratio_db(signal: &[f64]) -> f64 {
    let papr = peak_to_average_power_ratio(signal);
    if papr < 1e-30 {
        return f64::NEG_INFINITY;
    }
    10.0 * papr.log10()
}

/// dBm to voltage conversion.
///
/// `V = sqrt(Z * 10^((dBm - 30) / 10))`
///
/// * `zero_dbm_level` – impedance in ohms (typically 50 Ω)
#[must_use]
pub fn dbm_to_voltage(dbm: f64, impedance: f64) -> f64 {
    (impedance * 10.0_f64.powf((dbm - 30.0) / 10.0)).sqrt()
}

/// Voltage to dBm conversion.
///
/// `dBm = 10 * log10(V² / Z) + 30`
#[must_use]
pub fn voltage_to_dbm(voltage: f64, impedance: f64) -> f64 {
    if impedance < 1e-30 {
        return f64::NEG_INFINITY;
    }
    10.0 * (voltage * voltage / impedance).log10() + 30.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_constant() {
        assert!((mean(&[5.0, 5.0, 5.0]) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn mean_of_empty() {
        assert!((mean(&[]) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn variance_of_constant_is_zero() {
        assert!(variance(&[3.0, 3.0, 3.0]).abs() < 1e-10);
    }

    #[test]
    fn std_dev_known_value() {
        let sig = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let sd = std_dev(&sig);
        assert!((sd - 2.0).abs() < 0.1);
    }

    #[test]
    fn sample_variance_bessel() {
        let sig = vec![2.0, 4.0, 6.0];
        let sv = sample_variance(&sig);
        // mean=4, deviations: 4,0,4 → sum=8, /(3-1)=4
        assert!((sv - 4.0).abs() < 1e-10);
    }

    #[test]
    fn min_max_correct() {
        let sig = vec![3.0, 1.0, 4.0, 1.0, 5.0];
        assert!((min(&sig) - 1.0).abs() < 1e-10);
        assert!((max(&sig) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn energy_correct() {
        let sig = vec![1.0, 2.0, 3.0];
        assert!((energy(&sig) - 14.0).abs() < 1e-10);
    }

    #[test]
    fn power_correct() {
        let sig = vec![1.0, 2.0, 3.0];
        assert!((power(&sig) - 14.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn rms_dc_signal() {
        assert!((rms(&[3.0, 3.0, 3.0]) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn peak_to_peak_range() {
        let sig = vec![-2.0, 0.0, 3.0];
        assert!((peak_to_peak(&sig) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn amp_db_roundtrip() {
        let amp = 2.5;
        let db = amp_to_db(amp);
        let recovered = db_to_amp(db);
        assert!((recovered - amp).abs() < 1e-10);
    }

    #[test]
    fn power_db_roundtrip() {
        let pwr = 100.0;
        let db = power_to_db(pwr);
        assert!((db - 20.0).abs() < 1e-10); // 10*log10(100) = 20
        let recovered = db_to_power(db);
        assert!((recovered - pwr).abs() < 1e-6);
    }

    #[test]
    fn amp_to_db_unity_is_zero() {
        assert!(amp_to_db(1.0).abs() < 1e-10);
    }

    #[test]
    fn amp_to_db_zero_is_neg_inf() {
        assert!(amp_to_db(0.0).is_infinite());
        assert!(amp_to_db(0.0) < 0.0);
    }

    #[test]
    fn db_to_amp_zero_is_one() {
        assert!((db_to_amp(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn histogram_uniform_distribution() {
        // 100 values evenly spread across [0, 10)
        let signal: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let hist = histogram(&signal, 0.0, 10.0, 10);
        assert_eq!(hist.len(), 10);
        // Each bin should have 10 samples
        for &h in &hist {
            assert!((h - 10.0).abs() < 1e-10);
        }
    }

    #[test]
    fn histogram_cumulative_last_equals_total() {
        let signal: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let cum = histogram_cumulative(&signal, 0.0, 50.0, 5);
        assert_eq!(cum.len(), 5);
        assert!((*cum.last().unwrap() - 50.0).abs() < 1e-10);
        // Should be non-decreasing
        for i in 1..cum.len() {
            assert!(cum[i] >= cum[i - 1]);
        }
    }

    #[test]
    fn histogram_equalize_range() {
        let signal: Vec<f64> = (0..100).map(|i| (i as f64).powi(2)).collect();
        let eq = histogram_equalize(&signal, 1.0);
        assert_eq!(eq.len(), 100);
        // All values should be in [0, 1]
        for &v in &eq {
            assert!(v >= -1e-10 && v <= 1.0 + 1e-10);
        }
    }

    #[test]
    fn histogram_empty_inputs() {
        assert!(histogram(&[], 0.0, 1.0, 10).is_empty());
        assert!(histogram_cumulative(&[], 0.0, 1.0, 10).is_empty());
        assert!(histogram_equalize(&[], 1.0).is_empty());
    }

    #[test]
    fn par_of_dc_is_one() {
        let signal = vec![3.0; 100];
        assert!((peak_to_average_ratio(&signal) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn papr_of_dc_is_one() {
        let signal = vec![2.0; 100];
        assert!((peak_to_average_power_ratio(&signal) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn papr_db_of_dc_is_zero() {
        let signal = vec![1.0; 100];
        assert!((peak_to_average_power_ratio_db(&signal) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn papr_of_sine_about_3db() {
        use std::f64::consts::PI;
        let signal: Vec<f64> = (0..1000)
            .map(|i| (2.0 * PI * i as f64 / 1000.0).sin())
            .collect();
        let papr_db = peak_to_average_power_ratio_db(&signal);
        // PAPR of a sine wave is 10*log10(2) ≈ 3.01 dB
        assert!((papr_db - 3.01).abs() < 0.1);
    }

    #[test]
    fn dbm_voltage_roundtrip() {
        let v = 1.0; // 1 volt
        let z = 50.0;
        let dbm = voltage_to_dbm(v, z);
        let v_back = dbm_to_voltage(dbm, z);
        assert!((v_back - v).abs() < 1e-6);
    }
}
