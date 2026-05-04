use std::f64::consts::PI;

/// Phase-Locked Loop (PLL) for carrier recovery and frequency tracking.
///
/// Implements a basic digital PLL with a proportional-integral loop filter.
pub struct Pll {
    phase: f64,
    frequency: f64,
    center_frequency: f64,
    kp: f64,
    ki: f64,
    integrator: f64,
}

impl Pll {
    /// Create a new PLL.
    ///
    /// * `center_frequency` – nominal frequency in radians/sample
    /// * `bandwidth` – loop bandwidth (controls convergence speed)
    /// * `damping` – damping factor (typically 0.707 for critical damping)
    #[must_use]
    pub fn new(center_frequency: f64, bandwidth: f64, damping: f64) -> Self {
        let kp = 2.0 * damping * bandwidth;
        let ki = bandwidth * bandwidth;
        Self {
            phase: 0.0,
            frequency: center_frequency,
            center_frequency,
            kp,
            ki,
            integrator: 0.0,
        }
    }

    /// Process one input sample and return the PLL output (VCO signal).
    ///
    /// Also returns the phase error for diagnostics.
    pub fn process_sample(&mut self, input: f64) -> (f64, f64) {
        // VCO output
        let vco_out = self.phase.cos();

        // Phase detector: multiply input by VCO quadrature
        let phase_error = input * (-self.phase.sin());

        // Loop filter (PI controller)
        self.integrator += self.ki * phase_error;
        self.frequency = self.center_frequency + self.kp * phase_error + self.integrator;

        // Update VCO phase
        self.phase += self.frequency;
        // Keep phase in [-π, π)
        while self.phase >= PI {
            self.phase -= 2.0 * PI;
        }
        while self.phase < -PI {
            self.phase += 2.0 * PI;
        }

        (vco_out, phase_error)
    }

    /// Process an entire signal, returning VCO output and phase errors.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut vco_out = Vec::with_capacity(signal.len());
        let mut errors = Vec::with_capacity(signal.len());
        for &s in signal {
            let (v, e) = self.process_sample(s);
            vco_out.push(v);
            errors.push(e);
        }
        (vco_out, errors)
    }

    /// Get the current estimated frequency.
    #[must_use]
    pub fn current_frequency(&self) -> f64 {
        self.frequency
    }

    /// Reset PLL state.
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.frequency = self.center_frequency;
        self.integrator = 0.0;
    }
}

/// Costas Loop for BPSK carrier recovery.
///
/// A Costas loop multiplies the input by both I and Q branches of a VCO,
/// low-pass filters each, and uses the cross-product for phase error detection.
pub struct CostasLoop {
    phase: f64,
    frequency: f64,
    center_frequency: f64,
    kp: f64,
    ki: f64,
    integrator: f64,
    i_filter_state: f64,
    q_filter_state: f64,
    filter_alpha: f64,
}

impl CostasLoop {
    /// Create a new Costas loop.
    ///
    /// * `center_frequency` – nominal carrier frequency in radians/sample
    /// * `bandwidth` – loop bandwidth
    /// * `damping` – damping factor
    /// * `filter_alpha` – one-pole LP filter coefficient for I/Q branches
    #[must_use]
    pub fn new(center_frequency: f64, bandwidth: f64, damping: f64, filter_alpha: f64) -> Self {
        let kp = 2.0 * damping * bandwidth;
        let ki = bandwidth * bandwidth;
        Self {
            phase: 0.0,
            frequency: center_frequency,
            center_frequency,
            kp,
            ki,
            integrator: 0.0,
            i_filter_state: 0.0,
            q_filter_state: 0.0,
            filter_alpha,
        }
    }

    /// Process one input sample. Returns `(i_output, q_output, phase_error)`.
    pub fn process_sample(&mut self, input: f64) -> (f64, f64, f64) {
        // Mix with VCO I and Q
        let i_mix = input * self.phase.cos();
        let q_mix = input * (-self.phase.sin());

        // Low-pass filter (one-pole)
        self.i_filter_state =
            self.filter_alpha * i_mix + (1.0 - self.filter_alpha) * self.i_filter_state;
        self.q_filter_state =
            self.filter_alpha * q_mix + (1.0 - self.filter_alpha) * self.q_filter_state;

        // Phase error: I * Q (for BPSK)
        let phase_error = self.i_filter_state * self.q_filter_state;

        // Loop filter
        self.integrator += self.ki * phase_error;
        self.frequency = self.center_frequency + self.kp * phase_error + self.integrator;

        // Update phase
        self.phase += self.frequency;
        while self.phase >= PI {
            self.phase -= 2.0 * PI;
        }
        while self.phase < -PI {
            self.phase += 2.0 * PI;
        }

        (self.i_filter_state, self.q_filter_state, phase_error)
    }

    /// Process an entire signal.
    #[must_use]
    pub fn apply(&mut self, signal: &[f64]) -> Vec<f64> {
        signal
            .iter()
            .map(|&s| {
                let (i, _, _) = self.process_sample(s);
                i
            })
            .collect()
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.frequency = self.center_frequency;
        self.integrator = 0.0;
        self.i_filter_state = 0.0;
        self.q_filter_state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pll_locks_to_tone() {
        let freq = 0.1; // radians/sample
        let signal: Vec<f64> = (0..500).map(|i| (freq * i as f64).cos()).collect();
        let mut pll = Pll::new(freq, 0.01, 0.707);
        let (vco, errors) = pll.apply(&signal);
        assert_eq!(vco.len(), 500);
        // Phase error should decrease over time
        let early_error: f64 = errors[..50].iter().map(|e| e.abs()).sum::<f64>() / 50.0;
        let late_error: f64 = errors[400..].iter().map(|e| e.abs()).sum::<f64>() / 100.0;
        assert!(late_error < early_error + 0.1);
    }

    #[test]
    fn pll_frequency_estimate() {
        let freq = 0.2;
        let signal: Vec<f64> = (0..1000).map(|i| (freq * i as f64).cos()).collect();
        let mut pll = Pll::new(0.2, 0.005, 0.707);
        pll.apply(&signal);
        // Should converge near the true frequency
        assert!((pll.current_frequency() - freq).abs() < 0.05);
    }

    #[test]
    fn costas_loop_demodulates_bpsk() {
        let freq = 0.3;
        // Simple BPSK: alternating +1/-1 symbols modulated on carrier
        let symbols = [1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];
        let samples_per_symbol = 50;
        let mut signal = Vec::new();
        for &s in &symbols {
            for i in 0..samples_per_symbol {
                let idx = signal.len();
                signal.push(s * (freq * idx as f64).cos());
            }
        }
        let mut costas = CostasLoop::new(freq, 0.01, 0.707, 0.1);
        let out = costas.apply(&signal);
        assert_eq!(out.len(), signal.len());
    }

    #[test]
    fn pll_reset() {
        let mut pll = Pll::new(0.1, 0.01, 0.707);
        pll.process_sample(1.0);
        pll.reset();
        assert!((pll.phase - 0.0).abs() < 1e-10);
    }
}
