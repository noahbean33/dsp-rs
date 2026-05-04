/// Manchester encode a single byte.
///
/// Each input bit is encoded as two output bits:
/// - `0` → `01`
/// - `1` → `10`
///
/// Returns a 16-bit value containing the encoded dibits.
#[must_use]
pub fn manchester_encode_byte(byte: u8) -> u16 {
    let mut result: u16 = 0;
    for i in 0..8 {
        let bit = (byte >> (7 - i)) & 1;
        let dibit: u16 = if bit == 1 { 0b10 } else { 0b01 };
        result |= dibit << (14 - 2 * i);
    }
    result
}

/// Manchester decode a 16-bit encoded value back to a byte.
///
/// Each pair of bits is decoded:
/// - `10` → `1`
/// - `01` → `0`
/// - Other patterns indicate an error (mapped to 0).
#[must_use]
pub fn manchester_decode_byte(encoded: u16) -> u8 {
    let mut result: u8 = 0;
    for i in 0..8 {
        let dibit = (encoded >> (14 - 2 * i)) & 0b11;
        let bit: u8 = match dibit {
            0b10 => 1,
            0b01 => 0,
            _ => 0, // error case
        };
        result |= bit << (7 - i);
    }
    result
}

/// Manchester encode a single bit.
///
/// Returns 2-bit result: `1` → `0b10`, `0` → `0b01`.
#[must_use]
pub fn manchester_encode_bit(bit: bool) -> u8 {
    if bit { 0b10 } else { 0b01 }
}

/// Manchester decode a dibit.
///
/// `0b10` → `Some(true)`, `0b01` → `Some(false)`, else → `None` (error).
#[must_use]
pub fn manchester_decode_bit(dibit: u8) -> Option<bool> {
    match dibit & 0b11 {
        0b10 => Some(true),
        0b01 => Some(false),
        _ => None,
    }
}

/// Interleave samples from a signal with a given stride.
///
/// Rearranges elements: `[a0, a1, a2, ..., b0, b1, b2, ...]` →
/// `[a0, b0, a1, b1, a2, b2, ...]` for stride=2.
///
/// Generic block interleaver: reorders so that element `i` moves to
/// position `(i % stride) * (len / stride) + (i / stride)`.
#[must_use]
pub fn interleave(data: &[f64], stride: usize) -> Vec<f64> {
    if data.is_empty() || stride == 0 || stride > data.len() {
        return data.to_vec();
    }
    let n = data.len();
    let rows = n / stride;
    let mut result = vec![0.0; n];
    for i in 0..n {
        let row = i / stride;
        let col = i % stride;
        let new_idx = col * rows + row;
        if new_idx < n {
            result[new_idx] = data[i];
        }
    }
    result
}

/// Deinterleave (inverse of `interleave`).
#[must_use]
pub fn deinterleave(data: &[f64], stride: usize) -> Vec<f64> {
    if data.is_empty() || stride == 0 || stride > data.len() {
        return data.to_vec();
    }
    let n = data.len();
    let rows = n / stride;
    let mut result = vec![0.0; n];
    for i in 0..n {
        let col = i / rows;
        let row = i % rows;
        let new_idx = row * stride + col;
        if new_idx < n {
            result[new_idx] = data[i];
        }
    }
    result
}

/// Compute the bit error rate between two byte slices.
///
/// Returns `(bit_errors, total_bits, ber)`.
#[must_use]
pub fn bit_error_rate(a: &[u8], b: &[u8]) -> (usize, usize, f64) {
    let len = a.len().min(b.len());
    let total_bits = len * 8;
    if total_bits == 0 {
        return (0, 0, 0.0);
    }
    let mut errors = 0usize;
    for i in 0..len {
        errors += (a[i] ^ b[i]).count_ones() as usize;
    }
    (errors, total_bits, errors as f64 / total_bits as f64)
}

/// Compute the Error Vector Magnitude (EVM) between ideal and received
/// complex constellation points.
///
/// EVM = sqrt(mean(|error|²)) / sqrt(mean(|ideal|²)) × 100%
///
/// Returns EVM as a percentage.
#[must_use]
pub fn error_vector_magnitude(
    ideal_re: &[f64],
    ideal_im: &[f64],
    received_re: &[f64],
    received_im: &[f64],
) -> f64 {
    let n = ideal_re
        .len()
        .min(ideal_im.len())
        .min(received_re.len())
        .min(received_im.len());
    if n == 0 {
        return 0.0;
    }

    let mut error_power = 0.0;
    let mut ideal_power = 0.0;
    for i in 0..n {
        let er = received_re[i] - ideal_re[i];
        let ei = received_im[i] - ideal_im[i];
        error_power += er * er + ei * ei;
        ideal_power += ideal_re[i] * ideal_re[i] + ideal_im[i] * ideal_im[i];
    }

    if ideal_power < 1e-30 {
        return 0.0;
    }

    (error_power / ideal_power).sqrt() * 100.0
}

/// Compute EVM in decibels.
#[must_use]
pub fn error_vector_magnitude_db(
    ideal_re: &[f64],
    ideal_im: &[f64],
    received_re: &[f64],
    received_im: &[f64],
) -> f64 {
    let evm_pct = error_vector_magnitude(ideal_re, ideal_im, received_re, received_im);
    if evm_pct < 1e-30 {
        return f64::NEG_INFINITY;
    }
    20.0 * (evm_pct / 100.0).log10()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manchester_encode_decode_roundtrip() {
        for byte in 0..=255u8 {
            let encoded = manchester_encode_byte(byte);
            let decoded = manchester_decode_byte(encoded);
            assert_eq!(decoded, byte, "Failed for byte {byte}");
        }
    }

    #[test]
    fn manchester_bit_roundtrip() {
        assert_eq!(manchester_decode_bit(manchester_encode_bit(true)), Some(true));
        assert_eq!(manchester_decode_bit(manchester_encode_bit(false)), Some(false));
    }

    #[test]
    fn manchester_invalid_dibit() {
        assert_eq!(manchester_decode_bit(0b00), None);
        assert_eq!(manchester_decode_bit(0b11), None);
    }

    #[test]
    fn interleave_deinterleave_roundtrip() {
        let data: Vec<f64> = (0..12).map(|i| i as f64).collect();
        let interleaved = interleave(&data, 3);
        let recovered = deinterleave(&interleaved, 3);
        for (a, b) in data.iter().zip(recovered.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn interleave_stride_2() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let interleaved = interleave(&data, 2);
        // [1,2,3,4,5,6] with stride 2 → [1,3,5,2,4,6]
        assert_eq!(interleaved, vec![1.0, 3.0, 5.0, 2.0, 4.0, 6.0]);
    }

    #[test]
    fn bit_error_rate_identical() {
        let a = vec![0xAA, 0x55, 0xFF];
        let (errors, bits, ber) = bit_error_rate(&a, &a);
        assert_eq!(errors, 0);
        assert_eq!(bits, 24);
        assert!((ber - 0.0).abs() < 1e-10);
    }

    #[test]
    fn bit_error_rate_all_different() {
        let a = vec![0x00];
        let b = vec![0xFF];
        let (errors, bits, ber) = bit_error_rate(&a, &b);
        assert_eq!(errors, 8);
        assert_eq!(bits, 8);
        assert!((ber - 1.0).abs() < 1e-10);
    }

    #[test]
    fn evm_zero_for_perfect_signal() {
        let ideal_re = vec![1.0, -1.0, 1.0, -1.0];
        let ideal_im = vec![0.0, 0.0, 0.0, 0.0];
        let evm = error_vector_magnitude(&ideal_re, &ideal_im, &ideal_re, &ideal_im);
        assert!(evm.abs() < 1e-10);
    }

    #[test]
    fn evm_nonzero_for_noisy_signal() {
        let ideal_re = vec![1.0, -1.0, 1.0, -1.0];
        let ideal_im = vec![0.0, 0.0, 0.0, 0.0];
        let recv_re = vec![1.1, -0.9, 0.95, -1.05];
        let recv_im = vec![0.05, -0.05, 0.1, -0.1];
        let evm = error_vector_magnitude(&ideal_re, &ideal_im, &recv_re, &recv_im);
        assert!(evm > 0.0);
        assert!(evm < 50.0); // Should be a small percentage
    }

    #[test]
    fn evm_db_finite() {
        let ideal_re = vec![1.0, -1.0];
        let ideal_im = vec![0.0, 0.0];
        let recv_re = vec![1.1, -0.9];
        let recv_im = vec![0.05, -0.05];
        let evm_db = error_vector_magnitude_db(&ideal_re, &ideal_im, &recv_re, &recv_im);
        assert!(evm_db.is_finite());
        assert!(evm_db < 0.0); // Should be negative dB for small EVM
    }
}
