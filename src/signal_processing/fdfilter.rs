use super::fft::{Complex, fft, ifft};

/// FFT-based overlap-add convolution for efficient filtering of long signals.
///
/// Splits the input `signal` into blocks of size `block_size`, performs circular
/// convolution with `kernel` in the frequency domain via FFT, and combines
/// the results using the overlap-add method.
///
/// Complexity: O(N log N) where N is the FFT size, versus O(N·M) for direct
/// convolution. Significantly faster when `kernel` is long.
///
/// Output length = `signal.len() + kernel.len() - 1`.
#[must_use]
pub fn fft_overlap_add(signal: &[f64], kernel: &[f64], block_size: usize) -> Vec<f64> {
    if signal.is_empty() || kernel.is_empty() || block_size == 0 {
        return Vec::new();
    }
    let sig_len = signal.len();
    let ker_len = kernel.len();
    let out_len = sig_len + ker_len - 1;

    // FFT size: next power of 2 >= block_size + kernel_len - 1
    let seg_len = block_size + ker_len - 1;
    let mut fft_size = 1;
    while fft_size < seg_len {
        fft_size <<= 1;
    }

    // Pre-compute kernel FFT (zero-padded)
    let mut h_fft: Vec<Complex> = kernel
        .iter()
        .map(|&v| Complex::new(v, 0.0))
        .chain(std::iter::repeat(Complex::zero()).take(fft_size - ker_len))
        .collect();
    fft(&mut h_fft);

    let mut output = vec![0.0; out_len];
    let mut pos = 0;

    while pos < sig_len {
        let end = (pos + block_size).min(sig_len);
        let current_block_len = end - pos;

        // Zero-padded block
        let mut block: Vec<Complex> = signal[pos..end]
            .iter()
            .map(|&v| Complex::new(v, 0.0))
            .chain(std::iter::repeat(Complex::zero()).take(fft_size - current_block_len))
            .collect();

        // Circular convolution via FFT
        fft(&mut block);
        for i in 0..fft_size {
            block[i] = block[i] * h_fft[i];
        }
        ifft(&mut block);

        // Overlap-add into output
        for i in 0..fft_size {
            let out_idx = pos + i;
            if out_idx < out_len {
                output[out_idx] += block[i].re;
            }
        }

        pos += block_size;
    }
    output
}

/// FFT-based overlap-save (overlap-scrap) convolution.
///
/// An alternative to overlap-add that avoids the addition step by discarding
/// the circular convolution artifacts and keeping only valid samples.
///
/// Output length = `signal.len() + kernel.len() - 1`.
#[must_use]
pub fn fft_overlap_save(signal: &[f64], kernel: &[f64], block_size: usize) -> Vec<f64> {
    if signal.is_empty() || kernel.is_empty() || block_size == 0 {
        return Vec::new();
    }
    let sig_len = signal.len();
    let ker_len = kernel.len();
    let out_len = sig_len + ker_len - 1;

    // FFT size: next power of 2 >= block_size + kernel_len - 1
    let mut fft_size = 1;
    let needed = block_size + ker_len - 1;
    while fft_size < needed {
        fft_size <<= 1;
    }
    let overlap = ker_len - 1;
    let valid_per_block = fft_size - overlap;

    // Pre-compute kernel FFT
    let mut h_fft: Vec<Complex> = kernel
        .iter()
        .map(|&v| Complex::new(v, 0.0))
        .chain(std::iter::repeat(Complex::zero()).take(fft_size - ker_len))
        .collect();
    fft(&mut h_fft);

    // Pad the signal with overlap zeros at front and enough at the back
    let padded_len = sig_len + 2 * overlap;
    let mut padded = vec![0.0; padded_len];
    for i in 0..sig_len {
        padded[overlap + i] = signal[i];
    }

    let mut output = vec![0.0; out_len];
    let mut in_pos = 0;
    let mut out_pos = 0;

    while out_pos < out_len {
        let mut block: Vec<Complex> = (in_pos..in_pos + fft_size)
            .map(|i| {
                if i < padded.len() {
                    Complex::new(padded[i], 0.0)
                } else {
                    Complex::zero()
                }
            })
            .collect();

        fft(&mut block);
        for i in 0..fft_size {
            block[i] = block[i] * h_fft[i];
        }
        ifft(&mut block);

        // Keep only valid portion (discard first `overlap` samples)
        for i in 0..valid_per_block {
            let idx = out_pos + i;
            if idx < out_len {
                output[idx] = block[overlap + i].re;
            }
        }

        in_pos += valid_per_block;
        out_pos += valid_per_block;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn direct_convolve(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
        if signal.is_empty() || kernel.is_empty() {
            return Vec::new();
        }
        let n = signal.len();
        let k = kernel.len();
        let mut result = vec![0.0; n + k - 1];
        for i in 0..n {
            for j in 0..k {
                result[i + j] += signal[i] * kernel[j];
            }
        }
        result
    }

    #[test]
    fn fft_overlap_add_matches_direct() {
        let signal: Vec<f64> = (0..100).map(|i| (i as f64 * 0.1).sin()).collect();
        let kernel = vec![0.2, 0.3, 0.5, 0.3, 0.2];
        let direct = direct_convolve(&signal, &kernel);
        let ola = fft_overlap_add(&signal, &kernel, 32);
        assert_eq!(direct.len(), ola.len());
        for (i, (a, b)) in direct.iter().zip(ola.iter()).enumerate() {
            assert!(
                (a - b).abs() < 1e-8,
                "Mismatch at index {i}: direct={a}, ola={b}"
            );
        }
    }

    #[test]
    fn fft_overlap_add_short_signal() {
        let signal = vec![1.0, 2.0, 3.0];
        let kernel = vec![1.0, 1.0];
        let result = fft_overlap_add(&signal, &kernel, 2);
        let expected = direct_convolve(&signal, &kernel);
        assert_eq!(result.len(), expected.len());
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-8);
        }
    }

    #[test]
    fn fft_overlap_save_matches_direct() {
        let signal: Vec<f64> = (0..100).map(|i| (i as f64 * 0.1).sin()).collect();
        let kernel = vec![0.2, 0.3, 0.5, 0.3, 0.2];
        let direct = direct_convolve(&signal, &kernel);
        let ols = fft_overlap_save(&signal, &kernel, 32);
        assert_eq!(direct.len(), ols.len());
        for (i, (a, b)) in direct.iter().zip(ols.iter()).enumerate() {
            assert!(
                (a - b).abs() < 1e-6,
                "Mismatch at index {i}: direct={a}, ols={b}"
            );
        }
    }

    #[test]
    fn fft_overlap_add_empty_inputs() {
        assert!(fft_overlap_add(&[], &[1.0], 4).is_empty());
        assert!(fft_overlap_add(&[1.0], &[], 4).is_empty());
    }
}
