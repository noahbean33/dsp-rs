use super::bmp_io::BmpImage;

// ---------------------------------------------------------------------------
// Gaussian noise
// ---------------------------------------------------------------------------

/// Add Gaussian noise to a grayscale image.
///
/// Uses the Box-Muller / Marsaglia polar method to generate normally-
/// distributed noise which is **added** to each existing pixel value.
/// `mean` is the noise centre (typically 0.0) and `stddev` is the standard
/// deviation.
///
/// A simple built-in PRNG is used so no external crate is needed.  Supply a
/// `seed` for reproducibility.
///
/// Translated from the C library `add_gaussian_noise` function.
#[must_use]
pub fn add_gaussian_noise(input: &BmpImage, mean: f64, stddev: f64, seed: u64) -> BmpImage {
    let mut output_data = input.data.clone();
    let mut rng = SimpleRng::new(seed);
    let mut has_spare = false;
    let mut spare: f64 = 0.0;

    for px in &mut output_data {
        let noise = if has_spare {
            has_spare = false;
            mean + stddev * spare
        } else {
            let (u, v, s) = loop {
                let u = rng.next_f64() * 2.0 - 1.0;
                let v = rng.next_f64() * 2.0 - 1.0;
                let s = u * u + v * v;
                if s > 0.0 && s < 1.0 {
                    break (u, v, s);
                }
            };
            let factor = (-2.0 * s.ln() / s).sqrt();
            spare = v * factor;
            has_spare = true;
            mean + stddev * u * factor
        };

        let val = (*px as f64 + noise).clamp(0.0, 255.0);
        *px = val as u8;
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Salt-and-pepper noise
// ---------------------------------------------------------------------------

/// Add salt-and-pepper (impulse) noise to a grayscale image.
///
/// `probability` controls the total density of noise (0.0 = none, 1.0 = all).
/// Half the noisy pixels become 0 (pepper), the other half become 255 (salt).
///
/// Translated from the C library `add_salt_pepper_noise` function.
#[must_use]
pub fn add_salt_pepper_noise(input: &BmpImage, probability: f64, seed: u64) -> BmpImage {
    let mut output_data = input.data.clone();
    let mut rng = SimpleRng::new(seed);

    for px in &mut output_data {
        let r = rng.next_f64();
        if r < probability / 2.0 {
            *px = 0; // pepper (black)
        } else if r < probability {
            *px = 255; // salt (white)
        }
        // otherwise keep original value
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Uniform noise
// ---------------------------------------------------------------------------

/// Add uniform noise to a grayscale image.
///
/// Each pixel is offset by a random value in the range `[-amplitude, +amplitude]`.
#[must_use]
pub fn add_uniform_noise(input: &BmpImage, amplitude: f32, seed: u64) -> BmpImage {
    let mut output_data = input.data.clone();
    let mut rng = SimpleRng::new(seed);

    for px in &mut output_data {
        let noise = rng.next_f32() * 2.0 * amplitude - amplitude;
        let val = (*px as f32 + noise).clamp(0.0, 255.0);
        *px = val as u8;
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Minimal PRNG (no external dependency required)
// ---------------------------------------------------------------------------

/// A simple xorshift64-based PRNG so the noise functions do not need an
/// external random number crate.
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 16) as u32
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() & 0x00FF_FFFF) as f32 / 16_777_216.0
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_noise_changes_image() {
        let mut img = BmpImage::new();
        img.width = 8;
        img.height = 8;
        img.data = vec![128u8; 64];
        let noisy = add_gaussian_noise(&img, 0.0, 20.0, 42);
        assert_ne!(noisy.data, img.data);
    }

    #[test]
    fn salt_pepper_zero_prob_is_identity() {
        let mut img = BmpImage::new();
        img.width = 8;
        img.height = 8;
        img.data = vec![128u8; 64];
        let noisy = add_salt_pepper_noise(&img, 0.0_f64, 42);
        assert_eq!(noisy.data, img.data);
    }

    #[test]
    fn uniform_noise_zero_amplitude_is_identity() {
        let mut img = BmpImage::new();
        img.width = 4;
        img.height = 4;
        img.data = vec![100u8; 16];
        let noisy = add_uniform_noise(&img, 0.0, 42);
        assert_eq!(noisy.data, img.data);
    }
}
