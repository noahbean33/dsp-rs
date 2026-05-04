use std::f64::consts::PI;

/// A complex number with double-precision real and imaginary parts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    #[must_use]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[must_use]
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    #[must_use]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    #[must_use]
    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    #[must_use]
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[must_use]
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    #[must_use]
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    #[must_use]
    pub fn exp(self) -> Self {
        let e = self.re.exp();
        Self {
            re: e * self.im.cos(),
            im: e * self.im.sin(),
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.norm_sqr();
        if denom < 1e-30 {
            return Self::zero();
        }
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl std::ops::MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

/// Compute the twiddle factor W_N^k = exp(-2πik/N).
#[must_use]
pub fn twiddle_factor(k: usize, n: usize) -> Complex {
    if n == 0 {
        return Complex::new(1.0, 0.0);
    }
    let angle = -2.0 * PI * k as f64 / n as f64;
    Complex::new(angle.cos(), angle.sin())
}

/// Perform bit-reversal permutation on `data` in-place.
pub fn bit_reversal_permutation(data: &mut [Complex]) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    let mut n_bits = 0u32;
    while (1usize << n_bits) < n {
        n_bits += 1;
    }
    for i in 0..n {
        let mut j = 0usize;
        for k in 0..n_bits {
            if i & (1 << k) != 0 {
                j |= 1 << (n_bits - 1 - k);
            }
        }
        if j > i {
            data.swap(i, j);
        }
    }
}

/// In-place iterative Cooley-Tukey radix-2 FFT.
///
/// `data` length **must** be a power of 2.
pub fn fft(data: &mut [Complex]) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    bit_reversal_permutation(data);

    let mut len = 2;
    while len <= n {
        let angle = -2.0 * PI / len as f64;
        let wlen = Complex::new(angle.cos(), angle.sin());
        let mut i = 0;
        while i < n {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..len / 2 {
                let u = data[i + j];
                let v = data[i + j + len / 2] * w;
                data[i + j] = u + v;
                data[i + j + len / 2] = u - v;
                w *= wlen;
            }
            i += len;
        }
        len <<= 1;
    }
}

/// In-place inverse FFT. Divides by N after the transform.
pub fn ifft(data: &mut [Complex]) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    // Conjugate input
    for c in data.iter_mut() {
        c.im = -c.im;
    }
    fft(data);
    // Conjugate and scale
    let scale = 1.0 / n as f64;
    for c in data.iter_mut() {
        c.re *= scale;
        c.im = -c.im * scale;
    }
}

/// Convert a real-valued signal to a complex vector (imaginary parts zero).
#[must_use]
pub fn real_to_complex(signal: &[f64]) -> Vec<Complex> {
    signal.iter().map(|&v| Complex::new(v, 0.0)).collect()
}

/// Zero-pad a complex vector to the next power of 2.
#[must_use]
pub fn zero_pad_to_power_of_2(data: &[Complex]) -> Vec<Complex> {
    let n = data.len();
    let mut size = 1;
    while size < n {
        size <<= 1;
    }
    let mut padded = Vec::with_capacity(size);
    padded.extend_from_slice(data);
    padded.resize(size, Complex::zero());
    padded
}

/// Compute the magnitude spectrum from a complex FFT result.
#[must_use]
pub fn magnitude_spectrum(fft_result: &[Complex]) -> Vec<f64> {
    fft_result.iter().map(Complex::norm).collect()
}

/// Compute the power spectrum (|X[k]|² / N).
#[must_use]
pub fn power_spectrum(fft_result: &[Complex]) -> Vec<f64> {
    let n = fft_result.len() as f64;
    fft_result.iter().map(|c| c.norm_sqr() / n).collect()
}

/// Compute the phase spectrum in radians.
#[must_use]
pub fn phase_spectrum(fft_result: &[Complex]) -> Vec<f64> {
    fft_result.iter().map(Complex::arg).collect()
}

// ─── Direct O(N²) DFT/IDFT for arbitrary-length signals ─────────────────────

/// Direct DFT of a real-valued signal (O(N²), works on any length).
///
/// Returns N/2 complex frequency bins (DC to Nyquist).
#[must_use]
pub fn dft_real(signal: &[f64]) -> Vec<Complex> {
    let n = signal.len();
    if n == 0 {
        return Vec::new();
    }
    let n_out = n / 2;
    let mut output = vec![Complex::zero(); n_out];
    for k in 0..n_out {
        for (i, &x) in signal.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
            output[k].re += x * angle.cos();
            output[k].im -= x * angle.sin();
        }
    }
    output
}

/// Direct DFT of a complex-valued signal (O(N²), works on any length).
#[must_use]
pub fn dft_complex(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n == 0 {
        return Vec::new();
    }
    let mut output = vec![Complex::zero(); n];
    for k in 0..n {
        for (i, &x) in input.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
            let twiddle = Complex::new(angle.cos(), -angle.sin());
            output[k] += x * twiddle;
        }
    }
    output
}

/// Direct inverse DFT producing a real-valued output (O(N²)).
///
/// `input` contains N/2 frequency bins; output length = 2 × input length.
#[must_use]
pub fn idft_real(input: &[Complex]) -> Vec<f64> {
    let n_in = input.len();
    if n_in == 0 {
        return Vec::new();
    }
    let n = n_in * 2;
    let mut output = vec![0.0; n];
    for i in 0..n {
        for (k, &bin) in input.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
            output[i] += bin.re * angle.cos() - bin.im * angle.sin();
        }
        output[i] /= n as f64;
    }
    output
}

/// Direct inverse DFT producing a complex-valued output (O(N²)).
#[must_use]
pub fn idft_complex(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n == 0 {
        return Vec::new();
    }
    let mut output = vec![Complex::zero(); n];
    for i in 0..n {
        for (k, &bin) in input.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
            let twiddle = Complex::new(angle.cos(), angle.sin());
            output[i] += bin * twiddle;
        }
        output[i].re /= n as f64;
        output[i].im /= n as f64;
    }
    output
}

// ─── Goertzel Algorithm ──────────────────────────────────────────────────────

/// Compute a single DFT bin using the Goertzel algorithm.
///
/// Efficiently evaluates X[k] for one frequency bin `k` in O(N) time without
/// computing the full FFT. Useful for DTMF detection, single-tone measurement,
/// or any scenario where only a few frequency bins are needed.
///
/// `signal` – input samples.
/// `k` – the DFT bin index (0 ≤ k < N).
///
/// Returns the complex DFT value X[k].
#[must_use]
pub fn goertzel(signal: &[f64], k: usize) -> Complex {
    let n = signal.len();
    if n == 0 {
        return Complex::zero();
    }
    let omega = 2.0 * PI * k as f64 / n as f64;
    let coeff = 2.0 * omega.cos();
    let mut q1 = 0.0;
    let mut q2 = 0.0;
    for &x in signal {
        let q0 = coeff * q1 - q2 + x;
        q2 = q1;
        q1 = q0;
    }
    Complex::new(q1 * omega.cos() - q2, q1 * omega.sin())
}

/// Compute the magnitude of a single DFT bin using the Goertzel algorithm.
///
/// Returns |X[k]| without forming the full complex result.
#[must_use]
pub fn goertzel_magnitude(signal: &[f64], k: usize) -> f64 {
    goertzel(signal, k).norm()
}

/// Compute the power of a single DFT bin using the Goertzel algorithm.
///
/// Returns |X[k]|² without forming the full complex result.
#[must_use]
pub fn goertzel_power(signal: &[f64], k: usize) -> f64 {
    goertzel(signal, k).norm_sqr()
}

// ─── Phase Unwrapping ─────────────────────────────────────────────────────────

/// Unwrap a phase sequence by adding/subtracting 2π to remove discontinuities.
///
/// Adjacent phase differences larger than `π` are corrected so the result is
/// a continuous (monotonic where appropriate) phase trajectory.
#[must_use]
pub fn phase_unwrap(phases: &[f64]) -> Vec<f64> {
    if phases.is_empty() {
        return Vec::new();
    }
    let mut unwrapped = Vec::with_capacity(phases.len());
    unwrapped.push(phases[0]);
    for i in 1..phases.len() {
        let mut diff = phases[i] - phases[i - 1];
        while diff > PI {
            diff -= 2.0 * PI;
        }
        while diff < -PI {
            diff += 2.0 * PI;
        }
        unwrapped.push(unwrapped[i - 1] + diff);
    }
    unwrapped
}

/// Wrap a phase sequence into the range `[-π, π)`.
#[must_use]
pub fn phase_wrap(phases: &[f64]) -> Vec<f64> {
    phases
        .iter()
        .map(|&p| {
            let mut w = p % (2.0 * PI);
            if w >= PI {
                w -= 2.0 * PI;
            }
            if w < -PI {
                w += 2.0 * PI;
            }
            w
        })
        .collect()
}

// ─── Cepstrum Analysis ────────────────────────────────────────────────────────

/// Compute the real cepstrum of a signal.
///
/// The real cepstrum is defined as the inverse DFT of the log magnitude
/// spectrum: `c[n] = IFFT(log(|FFT(x)|))`.
///
/// The input is zero-padded to the next power of 2.
/// Returns the real-valued cepstrum coefficients.
#[must_use]
pub fn real_cepstrum(signal: &[f64]) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }
    let mut data = zero_pad_to_power_of_2(&real_to_complex(signal));
    fft(&mut data);

    // log magnitude spectrum → treat as real signal for IFFT
    let mut log_mag: Vec<Complex> = data
        .iter()
        .map(|c| {
            let mag = c.norm().max(1e-30);
            Complex::new(mag.ln(), 0.0)
        })
        .collect();

    ifft(&mut log_mag);
    log_mag.iter().map(|c| c.re).collect()
}

/// Compute the complex cepstrum of a signal.
///
/// The complex cepstrum is `IFFT(log(FFT(x)))` where `log` is the complex
/// logarithm using the unwrapped phase.
///
/// Returns `(cepstrum_real, cepstrum_imag)` as two separate vectors.
#[must_use]
pub fn complex_cepstrum(signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
    if signal.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut data = zero_pad_to_power_of_2(&real_to_complex(signal));
    fft(&mut data);

    // Compute unwrapped phase
    let phases: Vec<f64> = data.iter().map(|c| c.arg()).collect();
    let unwrapped = phase_unwrap(&phases);

    // Complex log: ln|X[k]| + j * unwrapped_phase[k]
    let mut log_spectrum: Vec<Complex> = data
        .iter()
        .zip(unwrapped.iter())
        .map(|(c, &ph)| {
            let mag = c.norm().max(1e-30);
            Complex::new(mag.ln(), ph)
        })
        .collect();

    ifft(&mut log_spectrum);
    let re = log_spectrum.iter().map(|c| c.re).collect();
    let im = log_spectrum.iter().map(|c| c.im).collect();
    (re, im)
}

// ─── Chirp Z-Transform ───────────────────────────────────────────────────────

/// Compute the Chirp Z-Transform (CZT) of a signal.
///
/// Evaluates the Z-transform at `m` points along a spiral contour in the
/// z-plane starting at `a` and spiralling with ratio `w`:
///   `X[k] = Σ_{n=0}^{N-1} x[n] · A^{-n} · W^{nk}`,  k = 0, …, M-1
///
/// This generalises the DFT: with `a = 1` and `w = exp(-j2π/M)` it computes
/// M equally-spaced frequency bins around the unit circle.
///
/// # Arguments
/// * `signal` – input samples
/// * `m` – number of output points
/// * `w` – spiral ratio per output point (complex)
/// * `a` – starting point on the contour (complex)
///
/// Uses Bluestein's algorithm for O(N log N) computation via FFT convolution.
#[must_use]
pub fn czt(signal: &[f64], m: usize, w: Complex, a: Complex) -> Vec<Complex> {
    let n = signal.len();
    if n == 0 || m == 0 {
        return Vec::new();
    }

    // Total convolution length, rounded up to next power of 2
    let conv_len = n + m - 1;
    let mut fft_len = 1;
    while fft_len < conv_len {
        fft_len <<= 1;
    }

    // Pre-compute W^(k²/2) chirp coefficients
    let chirp = |k: i64| -> Complex {
        // W^(k²/2) = exp(j * arg(W) * k² / 2)  — note W is the ratio
        let angle = w.arg() * (k * k) as f64 / 2.0;
        let r = w.norm().powf((k * k) as f64 / 2.0);
        Complex::new(r * angle.cos(), r * angle.sin())
    };

    // Build y[n] = x[n] · A^{-n} · W^{n²/2}
    let mut yn = vec![Complex::zero(); fft_len];
    for i in 0..n {
        // A^{-n}: raise a to power -n
        let a_neg_n = {
            let angle = -a.arg() * i as f64;
            let r = a.norm().powf(-(i as f64));
            Complex::new(r * angle.cos(), r * angle.sin())
        };
        yn[i] = Complex::new(signal[i], 0.0) * a_neg_n * chirp(i as i64);
    }

    // Build h[n] = W^{-n²/2} (the chirp convolution kernel)
    let mut hn = vec![Complex::zero(); fft_len];
    for k in 0..m {
        let c = chirp(k as i64);
        // Inverse: 1/c
        let denom = c.norm_sqr();
        if denom > 1e-30 {
            hn[k] = Complex::new(c.re / denom, -c.im / denom);
        }
    }
    // Negative-index portion wrapped around
    for k in 1..n {
        let c = chirp(k as i64);
        let denom = c.norm_sqr();
        if denom > 1e-30 {
            hn[fft_len - k] = Complex::new(c.re / denom, -c.im / denom);
        }
    }

    // Convolve via FFT
    fft(&mut yn);
    fft(&mut hn);
    for i in 0..fft_len {
        yn[i] = yn[i] * hn[i];
    }
    ifft(&mut yn);

    // Multiply output by W^{k²/2}
    let mut result = Vec::with_capacity(m);
    for k in 0..m {
        result.push(yn[k] * chirp(k as i64));
    }
    result
}

/// Zoom FFT: compute M frequency bins in the range `[f1, f2]` Hz.
///
/// Uses the CZT to evaluate only the bins of interest at arbitrary resolution.
///
/// * `signal` – input samples
/// * `sample_rate` – sampling frequency in Hz
/// * `f1` – start frequency in Hz
/// * `f2` – end frequency in Hz
/// * `m` – number of output frequency bins
#[must_use]
pub fn zoom_fft(signal: &[f64], sample_rate: f64, f1: f64, f2: f64, m: usize) -> Vec<Complex> {
    if signal.is_empty() || m == 0 || sample_rate <= 0.0 {
        return Vec::new();
    }
    let n = signal.len();
    let theta1 = 2.0 * PI * f1 / sample_rate;
    let theta2 = 2.0 * PI * f2 / sample_rate;
    let a = Complex::from_polar(1.0, theta1);
    let step = if m > 1 {
        (theta2 - theta1) / (m - 1) as f64
    } else {
        0.0
    };
    let w = Complex::from_polar(1.0, -step);
    czt(signal, m, w, a)
}

// ─── DCT / IDCT ──────────────────────────────────────────────────────────────

/// Compute the Type-II Discrete Cosine Transform.
///
/// `X[k] = Σ_{n=0}^{N-1} x[n] * cos(π/N * (n + 0.5) * k)`, k = 0, …, N-1
#[must_use]
pub fn dct_ii(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n == 0 {
        return Vec::new();
    }
    let mut result = vec![0.0; n];
    for k in 0..n {
        let mut sum = 0.0;
        for i in 0..n {
            sum += signal[i] * (PI / n as f64 * (i as f64 + 0.5) * k as f64).cos();
        }
        result[k] = sum;
    }
    result
}

/// Compute the inverse Type-II DCT (Type-III DCT).
///
/// `x[n] = (1/N) * X[0] + (2/N) * Σ_{k=1}^{N-1} X[k] * cos(π/N * k * (n + 0.5))`
#[must_use]
pub fn idct_ii(spectrum: &[f64]) -> Vec<f64> {
    let n = spectrum.len();
    if n == 0 {
        return Vec::new();
    }
    let mut result = vec![0.0; n];
    for i in 0..n {
        let mut sum = spectrum[0] / 2.0;
        for k in 1..n {
            sum += spectrum[k] * (PI / n as f64 * k as f64 * (i as f64 + 0.5)).cos();
        }
        result[i] = sum * 2.0 / n as f64;
    }
    result
}

/// Orthonormal DCT-II.
///
/// Same as `dct_ii` but with orthonormal scaling so that the transform is
/// its own inverse (up to the `idct_ii_ortho` call).
#[must_use]
pub fn dct_ii_ortho(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n == 0 {
        return Vec::new();
    }
    let mut result = dct_ii(signal);
    let scale_0 = (1.0 / n as f64).sqrt();
    let scale_k = (2.0 / n as f64).sqrt();
    result[0] *= scale_0;
    for k in 1..n {
        result[k] *= scale_k;
    }
    result
}

/// Inverse orthonormal DCT-II.
#[must_use]
pub fn idct_ii_ortho(spectrum: &[f64]) -> Vec<f64> {
    let n = spectrum.len();
    if n == 0 {
        return Vec::new();
    }
    let scale_0 = (1.0 / n as f64).sqrt();
    let scale_k = (2.0 / n as f64).sqrt();
    let mut scaled = spectrum.to_vec();
    scaled[0] /= scale_0;
    for k in 1..n {
        scaled[k] /= scale_k;
    }
    idct_ii(&scaled)
}

// ─── FFT Shift ────────────────────────────────────────────────────────────────

/// Shift zero-frequency component to the centre of the spectrum.
///
/// For a length-N array, this swaps the two halves so that bin N/2 moves to
/// index 0, matching the convention used for visualising spectra.
#[must_use]
pub fn fft_shift(data: &[Complex]) -> Vec<Complex> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let mid = n / 2;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&data[mid..]);
    out.extend_from_slice(&data[..mid]);
    out
}

/// Inverse FFT shift — undoes `fft_shift`.
#[must_use]
pub fn ifft_shift(data: &[Complex]) -> Vec<Complex> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let mid = (n + 1) / 2;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&data[mid..]);
    out.extend_from_slice(&data[..mid]);
    out
}

/// Shift zero-frequency component to centre for real-valued spectra.
#[must_use]
pub fn fft_shift_real(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let mid = n / 2;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&data[mid..]);
    out.extend_from_slice(&data[..mid]);
    out
}

/// Inverse FFT shift for real-valued spectra.
#[must_use]
pub fn ifft_shift_real(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let mid = (n + 1) / 2;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&data[mid..]);
    out.extend_from_slice(&data[..mid]);
    out
}

// ─── Instantaneous Frequency ──────────────────────────────────────────────────

/// Estimate instantaneous frequency from two successive phase arrays.
///
/// `inst_freq[n] = (leading_phase[n] - lagging_phase[n]) / (2π)`
/// Result is in normalised frequency (cycles per sample).
#[must_use]
pub fn instantaneous_frequency(leading_phase: &[f64], lagging_phase: &[f64]) -> Vec<f64> {
    let n = leading_phase.len().min(lagging_phase.len());
    (0..n)
        .map(|i| {
            let mut diff = leading_phase[i] - lagging_phase[i];
            while diff > PI {
                diff -= 2.0 * PI;
            }
            while diff < -PI {
                diff += 2.0 * PI;
            }
            diff / (2.0 * PI)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_of_dc_signal() {
        let mut data = vec![Complex::new(1.0, 0.0); 4];
        fft(&mut data);
        assert!((data[0].re - 4.0).abs() < 1e-10);
        for c in &data[1..] {
            assert!(c.norm() < 1e-10);
        }
    }

    #[test]
    fn fft_ifft_roundtrip() {
        let original = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        let mut data = original.clone();
        fft(&mut data);
        ifft(&mut data);
        for (a, b) in original.iter().zip(data.iter()) {
            assert!((a.re - b.re).abs() < 1e-10);
            assert!((a.im - b.im).abs() < 1e-10);
        }
    }

    #[test]
    fn bit_reversal_identity_for_len_1() {
        let mut data = vec![Complex::new(42.0, 0.0)];
        bit_reversal_permutation(&mut data);
        assert!((data[0].re - 42.0).abs() < 1e-10);
    }

    #[test]
    fn twiddle_w0_is_one() {
        let w = twiddle_factor(0, 8);
        assert!((w.re - 1.0).abs() < 1e-10);
        assert!(w.im.abs() < 1e-10);
    }

    #[test]
    fn dft_real_dc_signal() {
        let signal = vec![1.0; 10];
        let result = dft_real(&signal);
        assert_eq!(result.len(), 5);
        assert!((result[0].re - 10.0).abs() < 1e-6);
        for c in &result[1..] {
            assert!(c.norm() < 1e-6);
        }
    }

    #[test]
    fn dft_complex_roundtrip() {
        let input: Vec<Complex> = (0..7)
            .map(|i| Complex::new(i as f64, 0.0))
            .collect();
        let freq = dft_complex(&input);
        let recovered = idft_complex(&freq);
        for (a, b) in input.iter().zip(recovered.iter()) {
            assert!((a.re - b.re).abs() < 1e-6);
            assert!((a.im - b.im).abs() < 1e-6);
        }
    }

    #[test]
    fn dft_real_arbitrary_length() {
        // Verify DFT works on non-power-of-2 length
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let result = dft_real(&signal);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn goertzel_dc_bin() {
        let signal = vec![1.0; 8];
        let result = goertzel(&signal, 0);
        assert!((result.re - 8.0).abs() < 1e-10);
        assert!(result.im.abs() < 1e-10);
    }

    #[test]
    fn goertzel_matches_dft() {
        let signal: Vec<f64> = (0..16)
            .map(|i| (2.0 * PI * 3.0 * i as f64 / 16.0).sin())
            .collect();
        let mut complex_signal = real_to_complex(&signal);
        let mut fft_result = complex_signal.clone();
        fft(&mut fft_result);
        for k in 0..16 {
            let g = goertzel(&signal, k);
            assert!((g.re - fft_result[k].re).abs() < 1e-6);
            assert!((g.im - fft_result[k].im).abs() < 1e-6);
        }
    }

    #[test]
    fn goertzel_magnitude_single_tone() {
        let n = 64;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 5.0 * i as f64 / n as f64).sin())
            .collect();
        let mag_at_5 = goertzel_magnitude(&signal, 5);
        let mag_at_10 = goertzel_magnitude(&signal, 10);
        assert!(mag_at_5 > 20.0);
        assert!(mag_at_10 < 1e-6);
    }

    #[test]
    fn phase_unwrap_removes_discontinuity() {
        // Linear ramp that wraps at π
        let phases: Vec<f64> = (0..10)
            .map(|i| {
                let p = 0.4 * PI * i as f64;
                // Wrap into [-π, π)
                let mut w = p % (2.0 * PI);
                if w >= PI { w -= 2.0 * PI; }
                w
            })
            .collect();
        let unwrapped = phase_unwrap(&phases);
        // Should be monotonically increasing
        for i in 1..unwrapped.len() {
            assert!(unwrapped[i] >= unwrapped[i - 1] - 1e-10);
        }
    }

    #[test]
    fn phase_wrap_into_range() {
        let phases = vec![0.0, PI + 0.1, -PI - 0.1, 4.0 * PI];
        let wrapped = phase_wrap(&phases);
        for &w in &wrapped {
            assert!(w >= -PI && w < PI + 1e-10);
        }
    }

    #[test]
    fn real_cepstrum_symmetric_for_symmetric_signal() {
        // A symmetric signal should produce a symmetric cepstrum
        let signal = vec![1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 0.5, 0.5];
        let ceps = real_cepstrum(&signal);
        assert!(!ceps.is_empty());
        // First coefficient (quefrency 0) should be nonzero
        assert!(ceps[0].abs() > 1e-10);
    }

    #[test]
    fn complex_cepstrum_returns_correct_length() {
        let signal = vec![1.0, 0.5, 0.25, 0.125];
        let (re, im) = complex_cepstrum(&signal);
        assert_eq!(re.len(), im.len());
        // Padded to power of 2 (already 4)
        assert_eq!(re.len(), 4);
    }

    #[test]
    fn czt_matches_fft_on_unit_circle() {
        // CZT with a=1, w=exp(-j2π/N) should match the standard DFT
        let signal: Vec<f64> = (0..8)
            .map(|i| (2.0 * PI * 2.0 * i as f64 / 8.0).sin())
            .collect();
        let n = signal.len();
        let a = Complex::new(1.0, 0.0);
        let w = Complex::from_polar(1.0, -2.0 * PI / n as f64);
        let czt_result = czt(&signal, n, w, a);

        let mut fft_data = real_to_complex(&signal);
        fft(&mut fft_data);

        for k in 0..n {
            assert!(
                (czt_result[k].re - fft_data[k].re).abs() < 1e-4,
                "Mismatch at bin {k} re: czt={} fft={}",
                czt_result[k].re, fft_data[k].re
            );
            assert!(
                (czt_result[k].im - fft_data[k].im).abs() < 1e-4,
                "Mismatch at bin {k} im: czt={} fft={}",
                czt_result[k].im, fft_data[k].im
            );
        }
    }

    #[test]
    fn zoom_fft_detects_tone() {
        let fs = 1000.0;
        let n = 256;
        let freq = 100.0;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * freq * i as f64 / fs).sin())
            .collect();
        // Zoom into 80–120 Hz with 64 bins
        let result = zoom_fft(&signal, fs, 80.0, 120.0, 64);
        assert_eq!(result.len(), 64);
        // Find peak bin
        let mags: Vec<f64> = result.iter().map(|c| c.norm()).collect();
        let peak_idx = mags
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        // Peak should be near the center (bin 32 corresponds to ~100 Hz)
        let peak_freq = 80.0 + (120.0 - 80.0) * peak_idx as f64 / 63.0;
        assert!((peak_freq - 100.0).abs() < 5.0);
    }

    #[test]
    fn dct_ii_idct_ii_roundtrip() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let dct = dct_ii(&signal);
        let recovered = idct_ii(&dct);
        for (a, b) in signal.iter().zip(recovered.iter()) {
            assert!((a - b).abs() < 1e-10, "DCT roundtrip failed: {a} vs {b}");
        }
    }

    #[test]
    fn dct_ii_ortho_roundtrip() {
        let signal = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
        let dct = dct_ii_ortho(&signal);
        let recovered = idct_ii_ortho(&dct);
        for (a, b) in signal.iter().zip(recovered.iter()) {
            assert!((a - b).abs() < 1e-8, "Ortho DCT roundtrip: {a} vs {b}");
        }
    }

    #[test]
    fn dct_ii_dc_component() {
        let signal = vec![2.0; 8];
        let dct = dct_ii(&signal);
        // DC component should be sum of all values
        assert!((dct[0] - 16.0).abs() < 1e-10);
        // All other coefficients should be ~0
        for &v in &dct[1..] {
            assert!(v.abs() < 1e-10);
        }
    }

    #[test]
    fn fft_shift_roundtrip() {
        let data: Vec<Complex> = (0..8)
            .map(|i| Complex::new(i as f64, 0.0))
            .collect();
        let shifted = fft_shift(&data);
        let unshifted = ifft_shift(&shifted);
        for (a, b) in data.iter().zip(unshifted.iter()) {
            assert!((a.re - b.re).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_shift_real_swaps_halves() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let shifted = fft_shift_real(&data);
        assert_eq!(shifted, vec![3.0, 4.0, 5.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn instantaneous_frequency_constant_phase_diff() {
        let n = 16;
        let freq = 0.1; // normalised
        let leading: Vec<f64> = (0..n).map(|i| 2.0 * PI * freq * (i + 1) as f64).collect();
        let lagging: Vec<f64> = (0..n).map(|i| 2.0 * PI * freq * i as f64).collect();
        let inst = instantaneous_frequency(&leading, &lagging);
        for &f in &inst {
            assert!((f - freq).abs() < 1e-10);
        }
    }
}
