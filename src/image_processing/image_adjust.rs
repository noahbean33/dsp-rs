use super::bmp_io::BmpImage;

// ---------------------------------------------------------------------------
// Brightness adjustment
// ---------------------------------------------------------------------------

/// Adjust brightness by adding `factor` to every pixel.
///
/// Positive values brighten, negative values darken.  Results are clamped to
/// [0, 255].
///
/// Translated from the C `ImageBrightness`, `ImageDarkness`, and
/// `ImageBrightnessCorrection` programs.
#[must_use]
pub fn adjust_brightness(input: &BmpImage, factor: i32) -> BmpImage {
    let output_data: Vec<u8> = input
        .data
        .iter()
        .map(|&px| (i32::from(px) + factor).clamp(0, 255) as u8)
        .collect();
    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Histogram computation
// ---------------------------------------------------------------------------

/// Compute the normalised intensity histogram of a grayscale image.
///
/// Returns an array of 256 `f32` values where `hist[i]` is the fraction of
/// pixels with intensity `i`.
///
/// Translated from the C `ImgHistogram` function.
#[must_use]
pub fn compute_histogram(input: &BmpImage) -> [f32; 256] {
    let mut counts = [0u64; 256];
    let mut total: u64 = 0;

    for &px in &input.data {
        counts[px as usize] += 1;
        total += 1;
    }

    let mut hist = [0.0f32; 256];
    if total > 0 {
        for i in 0..256 {
            hist[i] = counts[i] as f32 / total as f32;
        }
    }
    hist
}

// ---------------------------------------------------------------------------
// Histogram equalization
// ---------------------------------------------------------------------------

/// Apply histogram equalization to a grayscale image for contrast enhancement.
///
/// Computes the CDF from the histogram and remaps pixel intensities.
///
/// Translated from the C `ImgHistogramEqualization` function.
#[must_use]
pub fn histogram_equalization(input: &BmpImage) -> BmpImage {
    let hist = compute_histogram(input);

    let mut histeq = [0u8; 256];
    for i in 0..256 {
        let mut cdf: f32 = 0.0;
        for j in 0..=i {
            cdf += hist[j];
        }
        histeq[i] = (255.0 * cdf + 0.5).clamp(0.0, 255.0) as u8;
    }

    let output_data: Vec<u8> = input
        .data
        .iter()
        .map(|&px| histeq[px as usize])
        .collect();
    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Image rotation
// ---------------------------------------------------------------------------

/// Rotate a grayscale image 90° clockwise.
///
/// Translated from the C library `rotate_90_cw` function:
/// `output[(cols-1-x)*rows + y] = input[y*cols + x]`
#[must_use]
pub fn rotate_90_cw(input: &BmpImage) -> BmpImage {
    let rows = input.height as usize;
    let cols = input.width as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in 0..rows {
        for x in 0..cols {
            let src = y * cols + x;
            let dst = (cols - 1 - x) * rows + y;
            if src < input.data.len() && dst < output_data.len() {
                output_data[dst] = input.data[src];
            }
        }
    }

    input.with_data(output_data)
}

/// Rotate a grayscale image 90° counter-clockwise.
///
/// Translated from the C library `rotate_90_ccw` function:
/// `output[x*rows + (rows-1-y)] = input[y*cols + x]`
#[must_use]
pub fn rotate_90_ccw(input: &BmpImage) -> BmpImage {
    let rows = input.height as usize;
    let cols = input.width as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in 0..rows {
        for x in 0..cols {
            let src = y * cols + x;
            let dst = x * rows + (rows - 1 - y);
            if src < input.data.len() && dst < output_data.len() {
                output_data[dst] = input.data[src];
            }
        }
    }

    input.with_data(output_data)
}

/// Rotate a grayscale image 180°.
///
/// Translated from the C library `rotate_180` function which reverses
/// the entire pixel data array.
#[must_use]
pub fn rotate_180(input: &BmpImage) -> BmpImage {
    let len = input.data.len();
    let mut output_data = vec![0u8; len];

    for i in 0..len {
        output_data[i] = input.data[len - 1 - i];
    }

    input.with_data(output_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_increase_clamps_at_255() {
        let mut img = BmpImage::new();
        img.width = 2;
        img.height = 2;
        img.data = vec![200, 200, 200, 200];
        let out = adjust_brightness(&img, 100);
        assert!(out.data.iter().all(|&px| px == 255));
    }

    #[test]
    fn brightness_decrease_clamps_at_0() {
        let mut img = BmpImage::new();
        img.width = 2;
        img.height = 2;
        img.data = vec![30, 30, 30, 30];
        let out = adjust_brightness(&img, -50);
        assert!(out.data.iter().all(|&px| px == 0));
    }

    #[test]
    fn histogram_sums_to_one() {
        let mut img = BmpImage::new();
        img.width = 4;
        img.height = 4;
        img.data = vec![0, 64, 128, 192, 0, 64, 128, 192, 0, 64, 128, 192, 0, 64, 128, 192];
        let hist = compute_histogram(&img);
        let sum: f32 = hist.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn rotate_180_is_involution() {
        let mut img = BmpImage::new();
        img.width = 3;
        img.height = 3;
        img.data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let rotated = rotate_180(&rotate_180(&img));
        assert_eq!(rotated.data, img.data);
    }
}
