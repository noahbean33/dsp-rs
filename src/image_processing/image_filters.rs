use super::bmp_io::BmpImage;
use super::edge_detection::{convolve2d, ConvMask};

// ---------------------------------------------------------------------------
// Median filter
// ---------------------------------------------------------------------------

/// Apply a median filter with a square window of size `n×n` (must be odd).
///
/// Translated from the C `median_filter` function which uses insertion sort
/// on the neighbourhood and picks the middle value.
#[must_use]
pub fn median_filter(input: &BmpImage, window_size: usize) -> BmpImage {
    let n = if window_size % 2 == 0 {
        window_size + 1
    } else {
        window_size
    };
    let half = n / 2;
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in half..rows.saturating_sub(half) {
        for x in half..cols.saturating_sub(half) {
            let mut neighbourhood: Vec<u8> = Vec::with_capacity(n * n);
            for j in 0..n {
                for i in 0..n {
                    let iy = (y + j).wrapping_sub(half);
                    let ix = (x + i).wrapping_sub(half);
                    if iy < rows && ix < cols {
                        neighbourhood.push(input.data[iy * cols + ix]);
                    }
                }
            }
            neighbourhood.sort_unstable();
            let median = neighbourhood[neighbourhood.len() / 2];
            output_data[y * cols + x] = median;
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Maximum filter (morphological dilation)
// ---------------------------------------------------------------------------

/// Apply a maximum filter with a square window of size `n×n`.
///
/// Translated from the C `max_filter` function.
#[must_use]
pub fn maximum_filter(input: &BmpImage, window_size: usize) -> BmpImage {
    let n = window_size;
    let half = n / 2;
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in half..rows.saturating_sub(half) {
        for x in half..cols.saturating_sub(half) {
            let mut smax: u8 = 0;
            for j in 0..n {
                for i in 0..n {
                    let iy = (y + j).wrapping_sub(half);
                    let ix = (x + i).wrapping_sub(half);
                    if iy < rows && ix < cols {
                        let val = input.data[iy * cols + ix];
                        if val > smax {
                            smax = val;
                        }
                    }
                }
            }
            output_data[y * cols + x] = smax;
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Minimum filter (morphological erosion)
// ---------------------------------------------------------------------------

/// Apply a minimum filter with a square window of size `n×n`.
///
/// Translated from the C `min_filter` function.
#[must_use]
pub fn minimum_filter(input: &BmpImage, window_size: usize) -> BmpImage {
    let n = window_size;
    let half = n / 2;
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in half..rows.saturating_sub(half) {
        for x in half..cols.saturating_sub(half) {
            let mut smin: u8 = 255;
            for j in 0..n {
                for i in 0..n {
                    let iy = (y + j).wrapping_sub(half);
                    let ix = (x + i).wrapping_sub(half);
                    if iy < rows && ix < cols {
                        let val = input.data[iy * cols + ix];
                        if val < smin {
                            smin = val;
                        }
                    }
                }
            }
            output_data[y * cols + x] = smin;
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Box blur (simple averaging 3×3)
// ---------------------------------------------------------------------------

/// Apply a 3×3 box blur to a **24-bit RGB** image.
///
/// Translated from the C `ImageBlur` program which operates on 3-channel
/// pixel data stored as interleaved BGR triplets.
#[must_use]
pub fn box_blur_rgb(input: &BmpImage) -> BmpImage {
    let cols = input.width as usize;
    let rows = input.height as usize;
    let pixel_count = rows * cols;

    if input.data.len() < pixel_count * 3 {
        return input.clone();
    }

    let mut output_data = vec![0u8; input.data.len()];
    let kernel: f32 = 1.0 / 9.0;

    for row in 1..rows.saturating_sub(1) {
        for col in 1..cols.saturating_sub(1) {
            let mut sum = [0.0f32; 3];
            for i in -1i32..=1 {
                for j in -1i32..=1 {
                    let idx = ((row as i32 + i) as usize) * cols + (col as i32 + j) as usize;
                    for ch in 0..3 {
                        sum[ch] += kernel * input.data[idx * 3 + ch] as f32;
                    }
                }
            }
            let out_idx = (row * cols + col) * 3;
            for ch in 0..3 {
                output_data[out_idx + ch] = sum[ch].clamp(0.0, 255.0) as u8;
            }
        }
    }

    input.with_data(output_data)
}

/// Apply a box blur (averaging filter) with a square window of size `n×n`
/// to a **grayscale** image.
///
/// Each pixel is replaced by the average of its neighbours in the window.
///
/// Translated from the C `box_blur` function.
#[must_use]
pub fn box_blur(input: &BmpImage, window_size: usize) -> BmpImage {
    let n = window_size;
    let half = n / 2;
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = input.data.clone();

    for y in half..rows.saturating_sub(half) {
        for x in half..cols.saturating_sub(half) {
            let mut sum: u32 = 0;
            let mut count: u32 = 0;
            for j in 0..n {
                for i in 0..n {
                    let iy = (y + j).wrapping_sub(half);
                    let ix = (x + i).wrapping_sub(half);
                    if iy < rows && ix < cols {
                        sum += u32::from(input.data[iy * cols + ix]);
                        count += 1;
                    }
                }
            }
            if count > 0 {
                output_data[y * cols + x] = (sum / count) as u8;
            }
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Gaussian blur (approximation via convolution)
// ---------------------------------------------------------------------------

/// Apply a 3×3 Gaussian-like blur to a grayscale image.
///
/// Uses the standard 3×3 approximation kernel [1 2 1; 2 4 2; 1 2 1] / 16.
#[must_use]
pub fn gaussian_blur(input: &BmpImage) -> BmpImage {
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = vec![0u8; input.data.len()];

    let kernel: [[f32; 3]; 3] = [
        [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
        [2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0],
        [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
    ];

    for y in 1..rows.saturating_sub(1) {
        for x in 1..cols.saturating_sub(1) {
            let mut sum: f32 = 0.0;
            for i in -1i32..=1 {
                for j in -1i32..=1 {
                    let idx =
                        ((y as i32 + i) as usize) * cols + (x as i32 + j) as usize;
                    sum += kernel[(i + 1) as usize][(j + 1) as usize]
                        * input.data[idx] as f32;
                }
            }
            output_data[y * cols + x] = sum.clamp(0.0, 255.0) as u8;
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// High-pass filter
// ---------------------------------------------------------------------------

/// Apply a 3×3 high-pass spatial filter.
///
/// Kernel: [-1 -1 -1; -1 9 -1; -1 -1 -1]
///
/// Translated from the C `HighPassSpatialFilter` program.
#[must_use]
pub fn highpass_filter(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, -1, -1, -1, 9, -1, -1, -1, -1]);
    convolve2d(input, &mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_of_uniform_image_is_unchanged() {
        let mut img = BmpImage::new();
        img.width = 8;
        img.height = 8;
        img.data = vec![100u8; 64];
        let out = median_filter(&img, 3);
        // interior pixels should remain 100
        assert_eq!(out.data[1 * 8 + 1], 100);
    }

    #[test]
    fn max_filter_picks_maximum() {
        let mut img = BmpImage::new();
        img.width = 5;
        img.height = 5;
        img.data = vec![50u8; 25];
        img.data[2 * 5 + 2] = 200; // centre pixel is bright
        let out = maximum_filter(&img, 3);
        // neighbours of centre should see 200
        assert_eq!(out.data[2 * 5 + 2], 200);
    }
}
