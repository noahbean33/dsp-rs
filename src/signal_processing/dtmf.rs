use std::f64::consts::PI;
use super::fft::goertzel_power;

/// DTMF frequency pairs for the standard 4×4 keypad.
///
/// Rows (low group):  697, 770, 852, 941 Hz
/// Columns (high group): 1209, 1336, 1477, 1633 Hz
const LOW_FREQS: [f64; 4] = [697.0, 770.0, 852.0, 941.0];
const HIGH_FREQS: [f64; 4] = [1209.0, 1336.0, 1477.0, 1633.0];

/// DTMF key layout matching (row, col) indices.
const KEYS: [[char; 4]; 4] = [
    ['1', '2', '3', 'A'],
    ['4', '5', '6', 'B'],
    ['7', '8', '9', 'C'],
    ['*', '0', '#', 'D'],
];

/// Generate a DTMF tone for the given key.
///
/// * `key` – one of `0-9`, `A-D`, `*`, `#`
/// * `sample_rate` – sampling frequency in Hz
/// * `duration_samples` – number of samples to generate
/// * `amplitude` – peak amplitude of each sinusoidal component
///
/// Returns the sum of the two component sinusoids, or an empty vector
/// if the key is invalid.
#[must_use]
pub fn dtmf_generate(key: char, sample_rate: f64, duration_samples: usize, amplitude: f64) -> Vec<f64> {
    let (low, high) = match key_to_freqs(key) {
        Some(pair) => pair,
        None => return Vec::new(),
    };
    (0..duration_samples)
        .map(|i| {
            let t = i as f64 / sample_rate;
            amplitude * (2.0 * PI * low * t).sin() + amplitude * (2.0 * PI * high * t).sin()
        })
        .collect()
}

/// Detect which DTMF key is present in a signal frame.
///
/// Uses the Goertzel algorithm to measure energy at each of the 8 DTMF
/// frequencies. The strongest low-group and high-group frequencies are
/// identified and mapped to a key character.
///
/// * `signal` – input samples (typically 10–50 ms of audio)
/// * `sample_rate` – sampling frequency in Hz
/// * `threshold` – minimum power level to consider a tone present
///
/// Returns `Some(key)` if a valid DTMF pair is detected, `None` otherwise.
#[must_use]
pub fn dtmf_detect(signal: &[f64], sample_rate: f64, threshold: f64) -> Option<char> {
    if signal.is_empty() || sample_rate <= 0.0 {
        return None;
    }
    let n = signal.len();

    // Compute Goertzel power at each DTMF frequency
    let low_powers: Vec<f64> = LOW_FREQS
        .iter()
        .map(|&f| {
            let k = (f * n as f64 / sample_rate).round() as usize;
            goertzel_power(signal, k)
        })
        .collect();

    let high_powers: Vec<f64> = HIGH_FREQS
        .iter()
        .map(|&f| {
            let k = (f * n as f64 / sample_rate).round() as usize;
            goertzel_power(signal, k)
        })
        .collect();

    // Find the strongest in each group
    let (low_idx, &low_max) = low_powers
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    let (high_idx, &high_max) = high_powers
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    // Both must exceed threshold
    if low_max < threshold || high_max < threshold {
        return None;
    }

    Some(KEYS[low_idx][high_idx])
}

/// Map a DTMF key character to its (low, high) frequency pair.
fn key_to_freqs(key: char) -> Option<(f64, f64)> {
    for (r, row) in KEYS.iter().enumerate() {
        for (c, &k) in row.iter().enumerate() {
            if k == key || (key.is_ascii_lowercase() && key.to_ascii_uppercase() == k) {
                return Some((LOW_FREQS[r], HIGH_FREQS[c]));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_correct_length() {
        let tone = dtmf_generate('5', 8000.0, 400, 1.0);
        assert_eq!(tone.len(), 400);
    }

    #[test]
    fn generate_invalid_key_empty() {
        let tone = dtmf_generate('X', 8000.0, 400, 1.0);
        assert!(tone.is_empty());
    }

    #[test]
    fn detect_digit_5() {
        let fs = 8000.0;
        let n = 400; // 50 ms
        let tone = dtmf_generate('5', fs, n, 1.0);
        let detected = dtmf_detect(&tone, fs, 1.0);
        assert_eq!(detected, Some('5'));
    }

    #[test]
    fn detect_digit_0() {
        let fs = 8000.0;
        let n = 400;
        let tone = dtmf_generate('0', fs, n, 1.0);
        let detected = dtmf_detect(&tone, fs, 1.0);
        assert_eq!(detected, Some('0'));
    }

    #[test]
    fn detect_star() {
        let fs = 8000.0;
        let n = 400;
        let tone = dtmf_generate('*', fs, n, 1.0);
        let detected = dtmf_detect(&tone, fs, 1.0);
        assert_eq!(detected, Some('*'));
    }

    #[test]
    fn detect_letter_a() {
        let fs = 8000.0;
        let n = 400;
        let tone = dtmf_generate('A', fs, n, 1.0);
        let detected = dtmf_detect(&tone, fs, 1.0);
        assert_eq!(detected, Some('A'));
    }

    #[test]
    fn detect_silence_returns_none() {
        let silence = vec![0.0; 400];
        let detected = dtmf_detect(&silence, 8000.0, 1.0);
        assert_eq!(detected, None);
    }

    #[test]
    fn roundtrip_all_keys() {
        let fs = 8000.0;
        let n = 400;
        let all_keys = "123A456B789C*0#D";
        for key in all_keys.chars() {
            let tone = dtmf_generate(key, fs, n, 1.0);
            let detected = dtmf_detect(&tone, fs, 1.0);
            assert_eq!(detected, Some(key), "Failed to detect key '{key}'");
        }
    }
}
