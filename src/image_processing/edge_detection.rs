use super::bmp_io::BmpImage;

/// A 2-D convolution mask (kernel) with signed coefficients.
#[derive(Debug, Clone)]
pub struct ConvMask {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i8>,
}

impl ConvMask {
    /// Build a mask from a slice of `i8` values in row-major order.
    #[must_use]
    pub fn new(rows: usize, cols: usize, data: &[i8]) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "mask data length must equal rows * cols"
        );
        Self {
            rows,
            cols,
            data: data.to_vec(),
        }
    }
}

// ---------------------------------------------------------------------------
// Generic 2-D discrete convolution (translated from the C `Convolve` function)
// ---------------------------------------------------------------------------

/// Apply a 2-D convolution to a grayscale image using the given mask.
///
/// The output image has the same dimensions as the input.  Border pixels that
/// fall outside the mask are clamped to zero.
#[must_use]
pub fn convolve2d(input: &BmpImage, mask: &ConvMask) -> BmpImage {
    let rows = input.height as usize;
    let cols = input.width as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for i in 0..rows {
        for j in 0..cols {
            let mut val: i32 = 0;
            for m in 0..mask.rows {
                for n in 0..mask.cols {
                    let ms = i32::from(mask.data[m * mask.cols + n]);
                    let idx = i as i64 - m as i64;
                    let jdx = j as i64 - n as i64;
                    if idx >= 0 && jdx >= 0 && (idx as usize) < rows && (jdx as usize) < cols {
                        let im = i32::from(input.data[idx as usize * cols + jdx as usize]);
                        val += ms * im;
                    }
                }
            }
            val = val.clamp(0, 255);
            output_data[i * cols + j] = val as u8;
        }
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Sobel edge detection
// ---------------------------------------------------------------------------

/// Sobel vertical edge detector (3×3).
#[must_use]
pub fn sobel_vertical(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, 0, 1, -2, 0, 2, -1, 0, 1]);
    convolve2d(input, &mask)
}

/// Sobel horizontal edge detector (3×3).
#[must_use]
pub fn sobel_horizontal(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, -2, -1, 0, 0, 0, 1, 2, 1]);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Prewitt edge detection
// ---------------------------------------------------------------------------

/// Prewitt vertical edge detector (3×3).
#[must_use]
pub fn prewitt_vertical(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, 0, 1, -1, 0, 1, -1, 0, 1]);
    convolve2d(input, &mask)
}

/// Prewitt horizontal edge detector (3×3).
#[must_use]
pub fn prewitt_horizontal(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, -1, -1, 0, 0, 0, 1, 1, 1]);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Roberts Cross edge detection
// ---------------------------------------------------------------------------

/// Roberts Cross vertical (Gx) operator (2×2).
#[must_use]
pub fn roberts_vertical(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(2, 2, &[1, 0, 0, -1]);
    convolve2d(input, &mask)
}

/// Roberts Cross horizontal (Gy) operator (2×2).
#[must_use]
pub fn roberts_horizontal(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(2, 2, &[0, 1, -1, 0]);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Kirsch compass edge detection
// ---------------------------------------------------------------------------

/// Kirsch north edge detector (3×3).
#[must_use]
pub fn kirsch_north(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[5, 5, 5, -3, 0, -3, -3, -3, -3]);
    convolve2d(input, &mask)
}

/// Kirsch south edge detector (3×3).
#[must_use]
pub fn kirsch_south(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-3, -3, -3, -3, 0, -3, 5, 5, 5]);
    convolve2d(input, &mask)
}

/// Kirsch east edge detector (3×3).
#[must_use]
pub fn kirsch_east(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-3, -3, 5, -3, 0, 5, -3, -3, 5]);
    convolve2d(input, &mask)
}

/// Kirsch west edge detector (3×3).
#[must_use]
pub fn kirsch_west(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[5, -3, -3, 5, 0, -3, 5, -3, -3]);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Robinson compass edge detection
// ---------------------------------------------------------------------------

/// Robinson north edge detector (3×3).
#[must_use]
pub fn robinson_north(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[1, 1, 1, 1, -2, 1, -1, -1, -1]);
    convolve2d(input, &mask)
}

/// Robinson south edge detector (3×3).
#[must_use]
pub fn robinson_south(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[1, 0, -1, 2, 0, -2, 1, 0, -1]);
    convolve2d(input, &mask)
}

/// Robinson east edge detector (3×3).
#[must_use]
pub fn robinson_east(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[-1, -2, -1, 0, 0, 0, 1, 2, 1]);
    convolve2d(input, &mask)
}

/// Robinson west edge detector (3×3).
#[must_use]
pub fn robinson_west(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[1, 2, 1, 0, 0, 0, -1, -2, -1]);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Laplacian edge detection
// ---------------------------------------------------------------------------

/// Laplacian edge detector – positive centre (3×3).
#[must_use]
pub fn laplacian_positive(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[0, 1, 0, 1, -4, 1, 0, 1, 0]);
    convolve2d(input, &mask)
}

/// Laplacian edge detector – negative centre (3×3).
#[must_use]
pub fn laplacian_negative(input: &BmpImage) -> BmpImage {
    let mask = ConvMask::new(3, 3, &[0, -1, 0, -1, 4, -1, 0, -1, 0]);
    convolve2d(input, &mask)
}

/// 5×5 Laplacian (high-pass) filter from the `DiscreteConvolution` example.
///
/// All weights are -1 except the centre which is 24.
#[must_use]
pub fn laplacian_5x5(input: &BmpImage) -> BmpImage {
    let mut data = [-1i8; 25];
    data[12] = 24; // centre element (row 2, col 2 in 0-indexed 5×5)
    let mask = ConvMask::new(5, 5, &data);
    convolve2d(input, &mask)
}

// ---------------------------------------------------------------------------
// Line detection
// ---------------------------------------------------------------------------

/// Generic 3×3 line detector (translated from the C `line_detector` function).
///
/// The mask is supplied as a 3×3 array in row-major order.
#[must_use]
pub fn line_detect(input: &BmpImage, mask_3x3: &[i32; 9]) -> BmpImage {
    let cols = input.width as usize;
    let rows = input.height as usize;
    let mut output_data = vec![0u8; input.data.len()];

    for y in 1..rows.saturating_sub(1) {
        for x in 1..cols.saturating_sub(1) {
            let mut sum: i32 = 0;
            for i in -1i32..=1 {
                for j in -1i32..=1 {
                    let px = input.data[((x as i32 + i) as usize)
                        + ((y as i32 + j) as usize) * cols] as i32;
                    sum += px * mask_3x3[((i + 1) * 3 + (j + 1)) as usize];
                }
            }
            sum = sum.clamp(0, 255);
            output_data[x + y * cols] = sum as u8;
        }
    }

    input.with_data(output_data)
}

/// Horizontal line detector (3×3).
#[must_use]
pub fn line_horizontal(input: &BmpImage) -> BmpImage {
    let mask = [-1, -1, -1, 2, 2, 2, -1, -1, -1];
    line_detect(input, &mask)
}

/// Vertical line detector (3×3).
#[must_use]
pub fn line_vertical(input: &BmpImage) -> BmpImage {
    let mask = [-1, 2, -1, -1, 2, -1, -1, 2, -1];
    line_detect(input, &mask)
}

/// Left-diagonal line detector (3×3).
#[must_use]
pub fn line_left_diagonal(input: &BmpImage) -> BmpImage {
    let mask = [2, -1, -1, -1, 2, -1, -1, -1, 2];
    line_detect(input, &mask)
}

/// Right-diagonal line detector (3×3).
#[must_use]
pub fn line_right_diagonal(input: &BmpImage) -> BmpImage {
    let mask = [-1, -1, 2, -1, 2, -1, 2, -1, -1];
    line_detect(input, &mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_mask_creation() {
        let mask = ConvMask::new(3, 3, &[-1, 0, 1, -2, 0, 2, -1, 0, 1]);
        assert_eq!(mask.rows, 3);
        assert_eq!(mask.cols, 3);
        assert_eq!(mask.data.len(), 9);
    }

    #[test]
    fn convolve2d_uniform_image() {
        let mut img = BmpImage::new();
        img.width = 4;
        img.height = 4;
        img.data = vec![128u8; 16];
        let mask = ConvMask::new(1, 1, &[1]);
        let out = convolve2d(&img, &mask);
        assert_eq!(out.data, vec![128u8; 16]);
    }
}
