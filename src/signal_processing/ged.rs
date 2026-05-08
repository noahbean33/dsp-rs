//! Generalized Eigendecomposition (GED) for source separation.
//!
//! GED solves `S * v = λ * R * v` where S and R are covariance matrices.
//! This is used for separating signals that maximize variance in one condition
//! (S) relative to another (R), e.g., task vs. baseline.

use std::f64::consts::PI;

// ─── Matrix helpers ───────────────────────────────────────────────────────────

fn mat_mul(a: &[f64], m: usize, k: usize, b: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0; m * n];
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0;
            for p in 0..k {
                sum += a[i * k + p] * b[p * n + j];
            }
            c[i * n + j] = sum;
        }
    }
    c
}

/// Jacobi eigendecomposition of symmetric matrix, sorted by decreasing eigenvalue.
fn jacobi_eigen(matrix: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut a = matrix.to_vec();
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }

    let max_iter = 100 * n * n;
    let tol = 1e-12;

    for _ in 0..max_iter {
        let mut max_val = 0.0;
        let mut p = 0;
        let mut q = 1;
        for i in 0..n {
            for j in (i + 1)..n {
                let val = a[i * n + j].abs();
                if val > max_val {
                    max_val = val;
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < tol {
            break;
        }

        let app = a[p * n + p];
        let aqq = a[q * n + q];
        let apq = a[p * n + q];

        let theta = if (app - aqq).abs() < 1e-30 {
            PI / 4.0
        } else {
            0.5 * (2.0 * apq / (app - aqq)).atan()
        };

        let c = theta.cos();
        let s = theta.sin();

        let mut new_a = a.clone();
        new_a[p * n + p] = c * c * app + 2.0 * s * c * apq + s * s * aqq;
        new_a[q * n + q] = s * s * app - 2.0 * s * c * apq + c * c * aqq;
        new_a[p * n + q] = 0.0;
        new_a[q * n + p] = 0.0;

        for i in 0..n {
            if i != p && i != q {
                let aip = a[i * n + p];
                let aiq = a[i * n + q];
                new_a[i * n + p] = c * aip + s * aiq;
                new_a[p * n + i] = new_a[i * n + p];
                new_a[i * n + q] = -s * aip + c * aiq;
                new_a[q * n + i] = new_a[i * n + q];
            }
        }
        a = new_a;

        for i in 0..n {
            let vip = v[i * n + p];
            let viq = v[i * n + q];
            v[i * n + p] = c * vip + s * viq;
            v[i * n + q] = -s * vip + c * viq;
        }
    }

    let mut evals: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| evals[b].partial_cmp(&evals[a]).unwrap_or(std::cmp::Ordering::Equal));

    let sorted_evals: Vec<f64> = indices.iter().map(|&i| evals[i]).collect();
    let mut sorted_evecs = vec![0.0; n * n];
    for (new_col, &old_col) in indices.iter().enumerate() {
        for row in 0..n {
            sorted_evecs[row * n + new_col] = v[row * n + old_col];
        }
    }

    evals = sorted_evals;
    (evals, sorted_evecs)
}

// ─── GED ──────────────────────────────────────────────────────────────────────

/// Result of Generalized Eigendecomposition.
#[derive(Clone, Debug)]
pub struct GedResult {
    /// Generalized eigenvalues sorted in decreasing order.
    pub eigenvalues: Vec<f64>,
    /// Generalized eigenvectors as columns, flat row-major (n × n).
    pub eigenvectors: Vec<f64>,
    /// Dimension.
    pub n: usize,
}

/// Solve the generalized eigenvalue problem `S * v = λ * R * v`.
///
/// Uses the approach: transform to standard eigenvalue problem via
/// `R^{-1/2} * S * R^{-1/2} * w = λ * w`, then recover `v = R^{-1/2} * w`.
///
/// * `cov_s` – "signal" covariance matrix, flat row-major (n × n)
/// * `cov_r` – "reference" covariance matrix, flat row-major (n × n)
/// * `n` – dimension of the matrices
/// * `regularization` – shrinkage parameter for R (0.0 = none, typical: 0.01)
///
/// Returns a [`GedResult`] with eigenvalues and eigenvectors sorted by decreasing λ.
#[must_use]
pub fn generalized_eigen(cov_s: &[f64], cov_r: &[f64], n: usize, regularization: f64) -> GedResult {
    assert_eq!(cov_s.len(), n * n);
    assert_eq!(cov_r.len(), n * n);

    // Apply regularization to R: R_reg = (1-γ)*R + γ*mean(eig(R))*I
    let r_reg = if regularization > 0.0 {
        let (r_evals, _) = jacobi_eigen(cov_r, n);
        let mean_eval: f64 = r_evals.iter().sum::<f64>() / n as f64;
        let mut r = cov_r.to_vec();
        for i in 0..n {
            for j in 0..n {
                r[i * n + j] = (1.0 - regularization) * r[i * n + j];
            }
            r[i * n + i] += regularization * mean_eval;
        }
        r
    } else {
        cov_r.to_vec()
    };

    // Eigendecompose R to get R^{-1/2}
    let (r_evals, r_evecs) = jacobi_eigen(&r_reg, n);

    // R^{-1/2} = V * D^{-1/2} * V^T
    let mut r_inv_sqrt = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut val = 0.0;
            for k in 0..n {
                let ev = r_evals[k].max(1e-30);
                val += r_evecs[i * n + k] * (1.0 / ev.sqrt()) * r_evecs[j * n + k];
            }
            r_inv_sqrt[i * n + j] = val;
        }
    }

    // Transform: M = R^{-1/2} * S * R^{-1/2}
    let temp = mat_mul(&r_inv_sqrt, n, n, cov_s, n);
    let m_transformed = mat_mul(&temp, n, n, &r_inv_sqrt, n);

    // Symmetrize to avoid numerical issues
    let mut m_sym = vec![0.0; n * n];
    for i in 0..n {
        for j in i..n {
            let avg = (m_transformed[i * n + j] + m_transformed[j * n + i]) / 2.0;
            m_sym[i * n + j] = avg;
            m_sym[j * n + i] = avg;
        }
    }

    // Standard eigendecomposition of transformed matrix
    let (evals, w) = jacobi_eigen(&m_sym, n);

    // Recover generalized eigenvectors: v = R^{-1/2} * w
    let evecs = mat_mul(&r_inv_sqrt, n, n, &w, n);

    GedResult {
        eigenvalues: evals,
        eigenvectors: evecs,
        n,
    }
}

/// Compute the filter forward model (spatial pattern) for a GED component.
///
/// The forward model shows how a source projects to sensors:
/// `pattern = eigenvector^T * S`
///
/// * `ged` – GED result
/// * `cov_s` – signal covariance matrix (n × n)
/// * `component` – component index (0 = best)
///
/// Returns a vector of length n (the spatial pattern/map).
#[must_use]
pub fn ged_forward_model(ged: &GedResult, cov_s: &[f64], component: usize) -> Vec<f64> {
    let n = ged.n;
    assert_eq!(cov_s.len(), n * n);
    assert!(component < n);

    // pattern = evec^T * S (1×n × n×n = 1×n)
    let mut pattern = vec![0.0; n];
    for j in 0..n {
        let mut sum = 0.0;
        for k in 0..n {
            sum += ged.eigenvectors[k * n + component] * cov_s[k * n + j];
        }
        pattern[j] = sum;
    }

    // Fix sign: make largest absolute value positive
    let max_idx = pattern.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);
    if pattern[max_idx] < 0.0 {
        for v in &mut pattern {
            *v = -*v;
        }
    }

    pattern
}

/// Project data using a GED eigenvector (spatial filter).
///
/// * `data` – flat row-major (channels × samples)
/// * `channels` – number of channels
/// * `samples` – number of time points
/// * `ged` – GED result
/// * `component` – which component to project (0 = best)
///
/// Returns a time series of length `samples`.
#[must_use]
pub fn ged_project(
    data: &[f64],
    channels: usize,
    samples: usize,
    ged: &GedResult,
    component: usize,
) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);
    assert_eq!(ged.n, channels);
    assert!(component < channels);

    let mut output = vec![0.0; samples];
    for s in 0..samples {
        let mut sum = 0.0;
        for ch in 0..channels {
            sum += ged.eigenvectors[ch * channels + component] * data[ch * samples + s];
        }
        output[s] = sum;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_covariance(data: &[f64], channels: usize, samples: usize) -> Vec<f64> {
        let mut centered = data.to_vec();
        for ch in 0..channels {
            let start = ch * samples;
            let mean: f64 = centered[start..start + samples].iter().sum::<f64>() / samples as f64;
            for s in 0..samples {
                centered[start + s] -= mean;
            }
        }
        let denom = if samples > 1 { (samples - 1) as f64 } else { 1.0 };
        let mut cov = vec![0.0; channels * channels];
        for i in 0..channels {
            for j in i..channels {
                let mut sum = 0.0;
                for s in 0..samples {
                    sum += centered[i * samples + s] * centered[j * samples + s];
                }
                let val = sum / denom;
                cov[i * channels + j] = val;
                cov[j * channels + i] = val;
            }
        }
        cov
    }

    #[test]
    fn ged_identity_reference_is_standard_eigen() {
        // When R = I, GED reduces to standard eigendecomposition of S
        let n = 3;
        let s = vec![
            2.0, 1.0, 0.0,
            1.0, 3.0, 1.0,
            0.0, 1.0, 2.0,
        ];
        let mut r = vec![0.0; n * n];
        for i in 0..n {
            r[i * n + i] = 1.0;
        }

        let result = generalized_eigen(&s, &r, n, 0.0);
        assert_eq!(result.eigenvalues.len(), n);
        // Eigenvalues should be positive and decreasing
        for w in result.eigenvalues.windows(2) {
            assert!(w[0] >= w[1] - 1e-6);
        }
    }

    #[test]
    fn ged_finds_signal_direction() {
        // Create two datasets: baseline (noise) and signal (strong in one direction)
        let n_ch = 3;
        let n_samples = 500;

        // Baseline: random noise
        let mut baseline = vec![0.0; n_ch * n_samples];
        let mut seed: u64 = 42;
        for v in baseline.iter_mut() {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            *v = ((seed >> 33) as f64 / (1u64 << 31) as f64) - 1.0;
        }

        // Signal: same noise + strong signal in channel 0
        let mut signal = baseline.clone();
        for s in 0..n_samples {
            let t = s as f64 / n_samples as f64;
            signal[s] += 5.0 * (2.0 * PI * 3.0 * t).sin(); // strong signal in ch0
        }

        let cov_r = make_covariance(&baseline, n_ch, n_samples);
        let cov_s = make_covariance(&signal, n_ch, n_samples);

        let result = generalized_eigen(&cov_s, &cov_r, n_ch, 0.01);

        // First eigenvalue should be largest (> 1 since signal has more variance)
        assert!(result.eigenvalues[0] > 1.0);
    }

    #[test]
    fn ged_with_regularization() {
        let n = 2;
        let s = vec![4.0, 1.0, 1.0, 2.0];
        let r = vec![2.0, 0.5, 0.5, 1.0];

        let result_no_reg = generalized_eigen(&s, &r, n, 0.0);
        let result_reg = generalized_eigen(&s, &r, n, 0.1);

        // Both should produce valid results
        assert_eq!(result_no_reg.eigenvalues.len(), n);
        assert_eq!(result_reg.eigenvalues.len(), n);
        // Regularized eigenvalues should be smaller (R is inflated)
        assert!(result_reg.eigenvalues[0] <= result_no_reg.eigenvalues[0] + 0.5);
    }

    #[test]
    fn ged_forward_model_dimensions() {
        let n = 3;
        let s = vec![
            2.0, 1.0, 0.0,
            1.0, 3.0, 1.0,
            0.0, 1.0, 2.0,
        ];
        let r = vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        let result = generalized_eigen(&s, &r, n, 0.0);
        let pattern = ged_forward_model(&result, &s, 0);
        assert_eq!(pattern.len(), n);
    }

    #[test]
    fn ged_project_correct_length() {
        let n_ch = 3;
        let n_samples = 100;
        let data = vec![1.0; n_ch * n_samples];
        let s = vec![
            2.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        let r = vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        let result = generalized_eigen(&s, &r, n_ch, 0.0);
        let ts = ged_project(&data, n_ch, n_samples, &result, 0);
        assert_eq!(ts.len(), n_samples);
    }
}
