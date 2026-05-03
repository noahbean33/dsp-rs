use super::bmp_io::BmpImage;

// ---------------------------------------------------------------------------
// RGB to Grayscale
// ---------------------------------------------------------------------------

/// Convert a 24-bit RGB BMP image to grayscale.
///
/// Uses the standard luminance coefficients: 0.30 R + 0.59 G + 0.11 B.
/// The output is still stored as 3-channel data (R=G=B=gray) so the BMP
/// header remains valid.
///
/// Translated from the C `RGB_To_Grayscale` program.
#[must_use]
pub fn rgb_to_grayscale(input: &BmpImage) -> BmpImage {
    let pixel_count = input.pixel_count();
    let mut output_data = Vec::with_capacity(pixel_count * 3);

    for i in 0..pixel_count {
        let base = i * 3;
        if base + 2 >= input.data.len() {
            break;
        }
        let r = input.data[base] as f32;
        let g = input.data[base + 1] as f32;
        let b = input.data[base + 2] as f32;

        let gray = (r * 0.30 + g * 0.59 + b * 0.11).clamp(0.0, 255.0) as u8;
        output_data.push(gray);
        output_data.push(gray);
        output_data.push(gray);
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// RGB to Sepia
// ---------------------------------------------------------------------------

/// Convert a 24-bit RGB BMP image to sepia tone.
///
/// Uses the standard sepia transformation matrix:
/// ```text
/// new_r = 0.393*R + 0.769*G + 0.189*B
/// new_g = 0.349*R + 0.686*G + 0.168*B
/// new_b = 0.272*R + 0.534*G + 0.131*B
/// ```
///
/// Translated from the C `RGB_to_Sepia` program.
#[must_use]
pub fn rgb_to_sepia(input: &BmpImage) -> BmpImage {
    let pixel_count = input.pixel_count();
    let mut output_data = Vec::with_capacity(pixel_count * 3);

    for i in 0..pixel_count {
        let base = i * 3;
        if base + 2 >= input.data.len() {
            break;
        }
        let r = input.data[base] as f32;
        let g = input.data[base + 1] as f32;
        let b = input.data[base + 2] as f32;

        let new_r = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0) as u8;
        let new_g = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0) as u8;
        let new_b = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0) as u8;

        output_data.push(new_r);
        output_data.push(new_g);
        output_data.push(new_b);
    }

    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Negative image
// ---------------------------------------------------------------------------

/// Invert every pixel value (255 − pixel) to produce a negative image.
///
/// Works for both 8-bit grayscale and 24-bit RGB data.
///
/// Translated from the C `NegativeImage` program.
#[must_use]
pub fn negative_image(input: &BmpImage) -> BmpImage {
    let output_data: Vec<u8> = input.data.iter().map(|&px| 255 - px).collect();
    input.with_data(output_data)
}

// ---------------------------------------------------------------------------
// Black and White (binary threshold)
// ---------------------------------------------------------------------------

/// Convert a grayscale image to binary black-and-white using the given
/// threshold.  Pixels at or above the threshold become 255 (white), others
/// become 0 (black).
///
/// Translated from the C `BlackNWhite` program.
#[must_use]
pub fn black_and_white(input: &BmpImage, threshold: u8) -> BmpImage {
    let output_data: Vec<u8> = input
        .data
        .iter()
        .map(|&px| if px >= threshold { 255 } else { 0 })
        .collect();
    input.with_data(output_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_of_zero_is_255() {
        let mut img = BmpImage::new();
        img.width = 2;
        img.height = 2;
        img.data = vec![0, 0, 0, 0];
        let out = negative_image(&img);
        assert!(out.data.iter().all(|&px| px == 255));
    }

    #[test]
    fn bw_threshold_splits_correctly() {
        let mut img = BmpImage::new();
        img.width = 4;
        img.height = 1;
        img.data = vec![10, 50, 100, 200];
        let out = black_and_white(&img, 50);
        assert_eq!(out.data, vec![0, 255, 255, 255]);
    }
}
