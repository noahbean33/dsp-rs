/// Peak-based Automatic Gain Control.
///
/// Adjusts signal amplitude so that the peak approaches `target_level`.
/// Uses separate attack and decay rates for smooth gain adjustment.
pub struct AgcPeak {
    gain: f64,
    prev_max: f64,
    target: f64,
    min_threshold: f64,
    attack: f64,
    decay: f64,
}

impl AgcPeak {
    /// Create a new peak-based AGC.
    ///
    /// * `target_level` – desired peak output level
    /// * `min_threshold` – minimum input level below which no gain control occurs
    /// * `attack` – attack sensitivity (0 < attack ≤ 1, higher = faster)
    /// * `decay` – decay sensitivity (0 < decay ≤ 1, higher = faster)
    #[must_use]
    pub fn new(target_level: f64, min_threshold: f64, attack: f64, decay: f64) -> Self {
        Self {
            gain: 1.0,
            prev_max: 0.0,
            target: target_level,
            min_threshold,
            attack,
            decay,
        }
    }

    /// Process a block of samples, applying gain control.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        if signal.is_empty() {
            return Vec::new();
        }
        let peak = signal.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);

        if peak > self.prev_max {
            self.prev_max = self.attack * peak + (1.0 - self.attack) * self.prev_max;
        } else {
            self.prev_max = self.decay * peak + (1.0 - self.decay) * self.prev_max;
        }

        if self.prev_max > self.min_threshold {
            let desired_gain = self.target / self.prev_max;
            self.gain = desired_gain;
        }

        signal.iter().map(|&x| x * self.gain).collect()
    }

    /// Reset the AGC state.
    pub fn reset(&mut self) {
        self.gain = 1.0;
        self.prev_max = 0.0;
    }
}

/// Mean-absolute-value based AGC.
///
/// Smoother than peak AGC, tracks the mean absolute value of the signal
/// using a moving average and adjusts gain accordingly.
pub struct AgcMeanAbs {
    gain: f64,
    mean_state: f64,
    target: f64,
    min_threshold: f64,
    attack: f64,
    decay: f64,
}

impl AgcMeanAbs {
    #[must_use]
    pub fn new(target_level: f64, min_threshold: f64, attack: f64, decay: f64) -> Self {
        Self {
            gain: 1.0,
            mean_state: 0.0,
            target: target_level,
            min_threshold,
            attack,
            decay,
        }
    }

    /// Process a single sample.
    pub fn process_sample(&mut self, input: f64) -> f64 {
        let abs_in = input.abs();
        if abs_in > self.mean_state {
            self.mean_state = self.attack * abs_in + (1.0 - self.attack) * self.mean_state;
        } else {
            self.mean_state = self.decay * abs_in + (1.0 - self.decay) * self.mean_state;
        }

        if self.mean_state > self.min_threshold {
            self.gain = self.target / self.mean_state;
        }

        input * self.gain
    }

    /// Process an entire signal.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    pub fn reset(&mut self) {
        self.gain = 1.0;
        self.mean_state = 0.0;
    }
}

/// Mean-squared AGC.
///
/// Tracks the mean squared value for RMS-like gain control.
pub struct AgcMeanSquared {
    gain: f64,
    ms_state: f64,
    target_ms: f64,
    min_threshold_ms: f64,
    alpha: f64,
}

impl AgcMeanSquared {
    /// * `target_rms` – desired RMS output level
    /// * `min_threshold` – minimum RMS below which no gain control occurs
    /// * `alpha` – smoothing coefficient (0 < α ≤ 1)
    #[must_use]
    pub fn new(target_rms: f64, min_threshold: f64, alpha: f64) -> Self {
        Self {
            gain: 1.0,
            ms_state: 0.0,
            target_ms: target_rms * target_rms,
            min_threshold_ms: min_threshold * min_threshold,
            alpha,
        }
    }

    pub fn process_sample(&mut self, input: f64) -> f64 {
        self.ms_state = self.alpha * input * input + (1.0 - self.alpha) * self.ms_state;

        if self.ms_state > self.min_threshold_ms {
            self.gain = (self.target_ms / self.ms_state).sqrt();
        }

        input * self.gain
    }

    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.process_sample(x)).collect()
    }

    pub fn reset(&mut self) {
        self.gain = 1.0;
        self.ms_state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agc_peak_boosts_quiet_signal() {
        let signal: Vec<f64> = (0..100).map(|i| 0.1 * (i as f64 * 0.1).sin()).collect();
        let mut agc = AgcPeak::new(1.0, 0.01, 0.5, 0.1);
        let out = agc.apply(&signal);
        let out_peak = out.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
        // Output should be amplified
        let in_peak = signal.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
        assert!(out_peak > in_peak);
    }

    #[test]
    fn agc_mean_abs_tracks() {
        let mut agc = AgcMeanAbs::new(0.5, 0.01, 0.1, 0.05);
        let signal: Vec<f64> = (0..200).map(|i| (i as f64 * 0.05).sin()).collect();
        let out = agc.apply(&signal);
        assert_eq!(out.len(), signal.len());
        // Gain should have adjusted — output mean abs should be closer to target
        let in_mean: f64 = signal.iter().map(|x| x.abs()).sum::<f64>() / signal.len() as f64;
        let out_mean: f64 = out.iter().map(|x| x.abs()).sum::<f64>() / out.len() as f64;
        assert!((out_mean - 0.5).abs() < (in_mean - 0.5).abs() + 0.3);
    }

    #[test]
    fn agc_mean_squared_positive_gain() {
        let mut agc = AgcMeanSquared::new(1.0, 0.01, 0.1);
        let signal: Vec<f64> = (0..100).map(|i| 0.1 * (i as f64 * 0.1).sin()).collect();
        let out = agc.apply(&signal);
        assert_eq!(out.len(), 100);
        // Gain should be > 1 since input is quiet
        assert!(agc.gain > 1.0);
    }

    #[test]
    fn agc_peak_reset() {
        let mut agc = AgcPeak::new(1.0, 0.01, 0.5, 0.1);
        agc.apply(&[10.0; 50]);
        agc.reset();
        assert!((agc.gain - 1.0).abs() < 1e-10);
    }
}
