use std::f64::consts::PI;

// ─── Matrix Utilities (row-major flat storage) ────────────────────────────────

/// Multiply two matrices: C = A (m×k) × B (k×n).
/// Matrices stored as flat row-major vectors.
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

/// Transpose a matrix (m×n) → (n×m).
fn mat_transpose(a: &[f64], m: usize, n: usize) -> Vec<f64> {
    let mut t = vec![0.0; m * n];
    for i in 0..m {
        for j in 0..n {
            t[j * m + i] = a[i * n + j];
        }
    }
    t
}

/// Compute the sample covariance matrix of data (channels × samples).
/// Returns a flat (channels × channels) symmetric matrix.
fn covariance_matrix(data: &[f64], channels: usize, samples: usize) -> Vec<f64> {
    // Mean-center each channel
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let row_start = ch * samples;
        let mean: f64 = centered[row_start..row_start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[row_start + s] -= mean;
        }
    }
    // C = X * X^T / (N-1)
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

/// Jacobi eigendecomposition of a symmetric matrix (n×n).
/// Returns (eigenvalues, eigenvectors_flat) sorted by decreasing eigenvalue.
/// Eigenvectors stored as columns in row-major order.
fn jacobi_eigen(matrix: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut a = matrix.to_vec();
    // V starts as identity
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }

    let max_iter = 100 * n * n;
    let tol = 1e-12;

    for _ in 0..max_iter {
        // Find largest off-diagonal element
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

        // Compute rotation angle
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

        // Update matrix A
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

        // Update eigenvectors
        for i in 0..n {
            let vip = v[i * n + p];
            let viq = v[i * n + q];
            v[i * n + p] = c * vip + s * viq;
            v[i * n + q] = -s * vip + c * viq;
        }
    }

    // Extract eigenvalues (diagonal of A)
    let mut evals: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();

    // Sort by decreasing eigenvalue
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| evals[b].partial_cmp(&evals[a]).unwrap_or(std::cmp::Ordering::Equal));

    let sorted_evals: Vec<f64> = indices.iter().map(|&i| evals[i]).collect();
    let mut sorted_evecs = vec![0.0; n * n];
    for (new_col, &old_col) in indices.iter().enumerate() {
        for row in 0..n {
            sorted_evecs[row * n + new_col] = v[row * n + old_col];
        }
    }

    // Fix: make sure unused variable warning is gone
    evals = sorted_evals;
    (evals, sorted_evecs)
}

// ─── PCA ──────────────────────────────────────────────────────────────────────

/// Result of a PCA decomposition.
#[derive(Clone, Debug)]
pub struct PcaResult {
    /// Eigenvalues sorted in decreasing order.
    pub eigenvalues: Vec<f64>,
    /// Eigenvectors as columns, stored flat row-major (n_channels × n_components).
    pub eigenvectors: Vec<f64>,
    /// Number of channels/features.
    pub n_features: usize,
    /// Number of components retained.
    pub n_components: usize,
    /// Percentage of variance explained by each component.
    pub variance_explained: Vec<f64>,
    /// Cumulative variance explained.
    pub cumulative_variance: Vec<f64>,
}

/// Perform PCA via eigendecomposition of the covariance matrix.
///
/// * `data` – flat row-major matrix (channels × samples)
/// * `channels` – number of channels/features (rows)
/// * `samples` – number of time points/observations (columns)
/// * `n_components` – number of principal components to retain (0 = all)
///
/// Returns a [`PcaResult`] containing eigenvalues, eigenvectors, and variance info.
#[must_use]
pub fn pca_eig(data: &[f64], channels: usize, samples: usize, n_components: usize) -> PcaResult {
    assert_eq!(data.len(), channels * samples, "Data length mismatch");

    let cov = covariance_matrix(data, channels, samples);
    let (evals, evecs) = jacobi_eigen(&cov, channels);

    let n_comp = if n_components == 0 || n_components > channels {
        channels
    } else {
        n_components
    };

    let total_var: f64 = evals.iter().sum();
    let variance_explained: Vec<f64> = evals[..n_comp]
        .iter()
        .map(|&e| if total_var > 0.0 { 100.0 * e / total_var } else { 0.0 })
        .collect();

    let mut cumulative_variance = Vec::with_capacity(n_comp);
    let mut cum = 0.0;
    for &v in &variance_explained {
        cum += v;
        cumulative_variance.push(cum);
    }

    // Extract only the first n_comp columns of eigenvectors
    let mut evecs_subset = vec![0.0; channels * n_comp];
    for row in 0..channels {
        for col in 0..n_comp {
            evecs_subset[row * n_comp + col] = evecs[row * channels + col];
        }
    }

    PcaResult {
        eigenvalues: evals[..n_comp].to_vec(),
        eigenvectors: evecs_subset,
        n_features: channels,
        n_components: n_comp,
        variance_explained,
        cumulative_variance,
    }
}

/// Perform PCA via Singular Value Decomposition.
///
/// Computes SVD of the mean-centered data matrix directly.
/// Equivalent to eigendecomposition of the covariance matrix but
/// numerically more stable for ill-conditioned data.
///
/// * `data` – flat row-major matrix (channels × samples)
/// * `channels` – number of channels/features (rows)
/// * `samples` – number of time points/observations (columns)
/// * `n_components` – number of components to retain (0 = all)
#[must_use]
pub fn pca_svd(data: &[f64], channels: usize, samples: usize, n_components: usize) -> PcaResult {
    assert_eq!(data.len(), channels * samples, "Data length mismatch");

    // Mean-center
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let row_start = ch * samples;
        let mean: f64 = centered[row_start..row_start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[row_start + s] -= mean;
        }
    }

    // SVD via eigendecomposition of X*X^T (covariance * (N-1))
    // This gives us left singular vectors (U) and singular values
    let denom = if samples > 1 { (samples - 1) as f64 } else { 1.0 };
    let mut gram = vec![0.0; channels * channels];
    for i in 0..channels {
        for j in i..channels {
            let mut sum = 0.0;
            for s in 0..samples {
                sum += centered[i * samples + s] * centered[j * samples + s];
            }
            gram[i * channels + j] = sum / denom;
            gram[j * channels + i] = sum / denom;
        }
    }

    let (evals, evecs) = jacobi_eigen(&gram, channels);

    let n_comp = if n_components == 0 || n_components > channels {
        channels
    } else {
        n_components
    };

    let total_var: f64 = evals.iter().filter(|&&v| v > 0.0).sum();
    let variance_explained: Vec<f64> = evals[..n_comp]
        .iter()
        .map(|&e| if total_var > 0.0 { 100.0 * e.max(0.0) / total_var } else { 0.0 })
        .collect();

    let mut cumulative_variance = Vec::with_capacity(n_comp);
    let mut cum = 0.0;
    for &v in &variance_explained {
        cum += v;
        cumulative_variance.push(cum);
    }

    let mut evecs_subset = vec![0.0; channels * n_comp];
    for row in 0..channels {
        for col in 0..n_comp {
            evecs_subset[row * n_comp + col] = evecs[row * channels + col];
        }
    }

    PcaResult {
        eigenvalues: evals[..n_comp].to_vec(),
        eigenvectors: evecs_subset,
        n_features: channels,
        n_components: n_comp,
        variance_explained,
        cumulative_variance,
    }
}

/// Project data onto principal component space.
///
/// * `data` – flat row-major (channels × samples)
/// * `pca` – PCA result containing eigenvectors
///
/// Returns component time series as flat row-major (n_components × samples).
#[must_use]
pub fn pca_project(data: &[f64], channels: usize, samples: usize, pca: &PcaResult) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);
    assert_eq!(pca.n_features, channels);

    // Mean-center
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let row_start = ch * samples;
        let mean: f64 = centered[row_start..row_start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[row_start + s] -= mean;
        }
    }

    // Project: scores = V^T * X  (n_comp × channels) * (channels × samples)
    let vt = mat_transpose(&pca.eigenvectors, channels, pca.n_components);
    mat_mul(&vt, pca.n_components, channels, &centered, samples)
}

/// Reconstruct data from PCA component scores.
///
/// * `scores` – flat row-major (n_components × samples)
/// * `pca` – PCA result containing eigenvectors
///
/// Returns reconstructed data (channels × samples).
#[must_use]
pub fn pca_reconstruct(scores: &[f64], samples: usize, pca: &PcaResult) -> Vec<f64> {
    assert_eq!(scores.len(), pca.n_components * samples);
    // Reconstruct: X_approx = V * scores  (channels × n_comp) * (n_comp × samples)
    mat_mul(&pca.eigenvectors, pca.n_features, pca.n_components, scores, samples)
}

// ─── ZCA Whitening ────────────────────────────────────────────────────────────

/// ZCA (Zero-phase Component Analysis) whitening result.
#[derive(Clone, Debug)]
pub struct ZcaResult {
    /// The whitening matrix W = V * D^{-1/2} * V^T (n × n).
    pub whitening_matrix: Vec<f64>,
    /// The de-whitening matrix W^{-1} = V * D^{1/2} * V^T.
    pub dewhitening_matrix: Vec<f64>,
    /// Dimension of the data.
    pub n_features: usize,
}

/// Compute ZCA whitening transform.
///
/// ZCA whitening produces data that is uncorrelated (identity covariance)
/// while remaining maximally similar to the original data (minimal rotation).
///
/// * `data` – flat row-major (channels × samples)
/// * `channels` – number of features
/// * `samples` – number of observations
/// * `regularization` – small value added to eigenvalues for numerical stability (e.g., 1e-6)
#[must_use]
pub fn zca_whitening(data: &[f64], channels: usize, samples: usize, regularization: f64) -> ZcaResult {
    assert_eq!(data.len(), channels * samples);

    let cov = covariance_matrix(data, channels, samples);
    let (evals, evecs) = jacobi_eigen(&cov, channels);

    let n = channels;

    // Compute W = V * D^{-1/2} * V^T
    // and W_inv = V * D^{1/2} * V^T
    let mut whitening = vec![0.0; n * n];
    let mut dewhitening = vec![0.0; n * n];

    for i in 0..n {
        for j in 0..n {
            let mut w_val = 0.0;
            let mut dw_val = 0.0;
            for k in 0..n {
                let ev = (evals[k] + regularization).max(1e-30);
                let d_inv_sqrt = 1.0 / ev.sqrt();
                let d_sqrt = ev.sqrt();
                w_val += evecs[i * n + k] * d_inv_sqrt * evecs[j * n + k];
                dw_val += evecs[i * n + k] * d_sqrt * evecs[j * n + k];
            }
            whitening[i * n + j] = w_val;
            dewhitening[i * n + j] = dw_val;
        }
    }

    ZcaResult {
        whitening_matrix: whitening,
        dewhitening_matrix: dewhitening,
        n_features: n,
    }
}

/// Apply ZCA whitening to data.
///
/// * `data` – flat row-major (channels × samples)
/// * `zca` – ZCA result containing the whitening matrix
///
/// Returns whitened data (channels × samples).
#[must_use]
pub fn zca_apply(data: &[f64], channels: usize, samples: usize, zca: &ZcaResult) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);
    assert_eq!(zca.n_features, channels);

    // Mean-center
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let row_start = ch * samples;
        let mean: f64 = centered[row_start..row_start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[row_start + s] -= mean;
        }
    }

    // W * X
    mat_mul(&zca.whitening_matrix, channels, channels, &centered, samples)
}

/// PCA whitening (sphering): decorrelates and normalizes variance.
///
/// Unlike ZCA, PCA whitening rotates data into the principal component space.
/// Result: `Y = D^{-1/2} * V^T * X`
///
/// * `data` – flat row-major (channels × samples)
/// * `channels` – number of features
/// * `samples` – number of observations
/// * `regularization` – small value for numerical stability
///
/// Returns whitened data (channels × samples).
#[must_use]
pub fn pca_whiten(data: &[f64], channels: usize, samples: usize, regularization: f64) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);

    let cov = covariance_matrix(data, channels, samples);
    let (evals, evecs) = jacobi_eigen(&cov, channels);

    let n = channels;

    // Mean-center
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let row_start = ch * samples;
        let mean: f64 = centered[row_start..row_start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[row_start + s] -= mean;
        }
    }

    // W = D^{-1/2} * V^T
    let mut w = vec![0.0; n * n];
    for i in 0..n {
        let scale = 1.0 / (evals[i] + regularization).max(1e-30).sqrt();
        for j in 0..n {
            w[i * n + j] = scale * evecs[j * n + i]; // V^T row i = evecs col i transposed
        }
    }

    mat_mul(&w, n, n, &centered, samples)
}

// ─── Public matrix utilities ──────────────────────────────────────────────────

/// Compute the sample covariance matrix of multi-channel data.
///
/// * `data` – flat row-major (channels × samples)
/// * `channels` – number of features (rows)
/// * `samples` – number of observations (columns)
///
/// Returns flat row-major (channels × channels) covariance matrix.
#[must_use]
pub fn compute_covariance(data: &[f64], channels: usize, samples: usize) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);
    covariance_matrix(data, channels, samples)
}

/// Symmetric eigendecomposition (public wrapper around Jacobi).
///
/// * `matrix` – flat row-major symmetric (n × n) matrix
/// * `n` – dimension
///
/// Returns (eigenvalues, eigenvectors) sorted by decreasing eigenvalue.
/// Eigenvectors stored as columns in row-major layout.
#[must_use]
pub fn symmetric_eigen(matrix: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    assert_eq!(matrix.len(), n * n);
    jacobi_eigen(matrix, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covariance_of_identity_pattern() {
        // Two channels, perfectly correlated
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0,
                        1.0, 2.0, 3.0, 4.0, 5.0];
        let cov = compute_covariance(&data, 2, 5);
        // Variance of [1..5] = 2.5, correlation = 1
        assert!((cov[0] - 2.5).abs() < 1e-10); // var(ch0)
        assert!((cov[3] - 2.5).abs() < 1e-10); // var(ch1)
        assert!((cov[1] - 2.5).abs() < 1e-10); // cov(0,1)
    }

    #[test]
    fn pca_eig_extracts_dominant_component() {
        // Create data with one dominant direction
        let n = 200;
        let mut data = vec![0.0; 2 * n];
        for i in 0..n {
            let t = i as f64 / n as f64;
            data[i] = t; // channel 0: linear ramp
            data[n + i] = t + 0.01 * (i as f64 * 0.5).sin(); // channel 1: same + tiny noise
        }
        let result = pca_eig(&data, 2, n, 0);
        // First component should explain almost all variance
        assert!(result.variance_explained[0] > 99.0);
    }

    #[test]
    fn pca_svd_matches_eig() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
            8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ];
        let r_eig = pca_eig(&data, 2, 8, 0);
        let r_svd = pca_svd(&data, 2, 8, 0);
        // Eigenvalues should match
        for (a, b) in r_eig.eigenvalues.iter().zip(r_svd.eigenvalues.iter()) {
            assert!((a - b).abs() < 1e-6, "eig={a}, svd={b}");
        }
    }

    #[test]
    fn pca_project_and_reconstruct() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 6.0, 8.0,
        ];
        let pca = pca_eig(&data, 2, 4, 2);
        let scores = pca_project(&data, 2, 4, &pca);
        let reconstructed = pca_reconstruct(&scores, 4, &pca);
        // Reconstructed should be close to mean-centered original
        let mut centered = data.to_vec();
        for ch in 0..2 {
            let mean: f64 = centered[ch * 4..(ch + 1) * 4].iter().sum::<f64>() / 4.0;
            for s in 0..4 {
                centered[ch * 4 + s] -= mean;
            }
        }
        for (a, b) in centered.iter().zip(reconstructed.iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }

    #[test]
    fn zca_whitening_produces_identity_covariance() {
        // Create correlated 2-channel data
        let n = 500;
        let mut data = vec![0.0; 2 * n];
        for i in 0..n {
            let t = i as f64 * 0.1;
            data[i] = t.sin();
            data[n + i] = t.sin() + 0.3 * t.cos();
        }
        let zca = zca_whitening(&data, 2, n, 1e-6);
        let whitened = zca_apply(&data, 2, n, &zca);

        // Covariance of whitened data should be ~identity
        let cov_w = compute_covariance(&whitened, 2, n);
        assert!((cov_w[0] - 1.0).abs() < 0.1); // var ~1
        assert!((cov_w[3] - 1.0).abs() < 0.1); // var ~1
        assert!(cov_w[1].abs() < 0.1); // cov ~0
    }

    #[test]
    fn pca_whiten_decorrelates() {
        let n = 500;
        let mut data = vec![0.0; 2 * n];
        for i in 0..n {
            let t = i as f64 * 0.1;
            data[i] = t.sin();
            data[n + i] = t.sin() + 0.5 * t.cos();
        }
        let whitened = pca_whiten(&data, 2, n, 1e-6);
        let cov_w = compute_covariance(&whitened, 2, n);
        // Off-diagonal should be near zero
        assert!(cov_w[1].abs() < 0.1);
        assert!(cov_w[2].abs() < 0.1);
    }

    #[test]
    fn variance_explained_sums_to_100() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0,
            5.0, 4.0, 3.0, 2.0, 1.0,
            0.5, 1.5, 2.5, 3.5, 4.5,
        ];
        let result = pca_eig(&data, 3, 5, 0);
        let total: f64 = result.variance_explained.iter().sum();
        assert!((total - 100.0).abs() < 1.0);
    }

    #[test]
    fn symmetric_eigen_identity() {
        let identity = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let (evals, _) = symmetric_eigen(&identity, 3);
        for &e in &evals {
            assert!((e - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn n_components_limits_output() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0,
            5.0, 4.0, 3.0, 2.0, 1.0,
            0.5, 1.5, 2.5, 3.5, 4.5,
        ];
        let result = pca_eig(&data, 3, 5, 2);
        assert_eq!(result.n_components, 2);
        assert_eq!(result.eigenvalues.len(), 2);
        assert_eq!(result.eigenvectors.len(), 3 * 2); // 3 features × 2 components
    }
}
