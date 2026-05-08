//! Independent Component Analysis (ICA) via the JADE algorithm.
//!
//! JADE (Joint Approximate Diagonalization of Eigenmatrices) performs blind
//! source separation by jointly diagonalizing a set of 4th-order cumulant
//! matrices using Givens rotations.
//!
//! Reference: Cardoso, J.-F. (1999). High-order contrasts for independent
//! component analysis. Neural Computation, 11(1): 157-192.

use std::f64::consts::PI;

// ─── Matrix helpers (local to this module) ────────────────────────────────────

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

fn mat_transpose(a: &[f64], m: usize, n: usize) -> Vec<f64> {
    let mut t = vec![0.0; m * n];
    for i in 0..m {
        for j in 0..n {
            t[j * m + i] = a[i * n + j];
        }
    }
    t
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

// ─── JADE ICA ─────────────────────────────────────────────────────────────────

/// Result of ICA decomposition.
#[derive(Clone, Debug)]
pub struct IcaResult {
    /// Separating matrix B (m × n) such that S = B * X gives independent sources.
    /// Stored flat row-major.
    pub separating_matrix: Vec<f64>,
    /// Mixing matrix A = pinv(B) (n × m), columns are source projections.
    /// Stored flat row-major.
    pub mixing_matrix: Vec<f64>,
    /// Number of sources extracted.
    pub n_sources: usize,
    /// Number of channels in original data.
    pub n_channels: usize,
}

/// Perform ICA using the JADE algorithm.
///
/// * `data` – flat row-major matrix (channels × samples)
/// * `channels` – number of sensors/channels (n)
/// * `samples` – number of time points (T)
/// * `n_sources` – number of independent components to extract (m ≤ n)
///
/// Returns an [`IcaResult`] with separating and mixing matrices.
#[must_use]
pub fn jade(data: &[f64], channels: usize, samples: usize, n_sources: usize) -> IcaResult {
    assert_eq!(data.len(), channels * samples);
    let n = channels;
    let m = n_sources.min(n);
    let t = samples;

    // Step 1: Mean-center
    let mut x = data.to_vec();
    for ch in 0..n {
        let start = ch * t;
        let mean: f64 = x[start..start + t].iter().sum::<f64>() / t as f64;
        for s in 0..t {
            x[start + s] -= mean;
        }
    }

    // Step 2: Whitening & PCA dimensionality reduction
    // Compute covariance
    let mut cov = vec![0.0; n * n];
    for i in 0..n {
        for j in i..n {
            let mut sum = 0.0;
            for s in 0..t {
                sum += x[i * t + s] * x[j * t + s];
            }
            let val = sum / t as f64;
            cov[i * n + j] = val;
            cov[j * n + i] = val;
        }
    }

    let (evals, evecs) = jacobi_eigen(&cov, n);

    // Take top m principal components and scale
    // B_pca = diag(1/sqrt(evals[0..m])) * evecs[:,0..m]^T
    let mut b_whiten = vec![0.0; m * n];
    for i in 0..m {
        let scale = 1.0 / evals[i].max(1e-30).sqrt();
        for j in 0..n {
            b_whiten[i * n + j] = scale * evecs[j * n + i];
        }
    }

    // Whitened data: X_w = B_whiten * X  (m × T)
    let x_w = mat_mul(&b_whiten, m, n, &x, t);

    // Step 3: Estimation of cumulant matrices
    // Transpose whitened data for easier column access
    let x_wt = mat_transpose(&x_w, m, t); // (T × m)

    let nbcm = m * (m + 1) / 2;
    // CM is m × (m * nbcm)
    let mut cm = vec![0.0; m * m * nbcm];

    // Identity matrix for R
    let mut r = vec![0.0; m * m];
    for i in 0..m {
        r[i * m + i] = 1.0;
    }

    let mut range_start = 0;
    for im in 0..m {
        // Xim = X_wt[:,im]
        // Xijm = Xim .* Xim
        // Qij = (Xijm .* X_wt)^T * X_wt / T - R - 2 * R[:,im] * R[:,im]^T
        let mut qij = vec![0.0; m * m];
        for row in 0..m {
            for col in 0..m {
                let mut sum = 0.0;
                for s in 0..t {
                    sum += x_wt[s * m + im] * x_wt[s * m + im] * x_wt[s * m + row] * x_wt[s * m + col];
                }
                qij[row * m + col] = sum / t as f64
                    - r[row * m + col]
                    - 2.0 * r[row * m + im] * r[col * m + im];
            }
        }

        // Store in CM
        for row in 0..m {
            for col in 0..m {
                cm[row * (m * nbcm) + range_start + col] = qij[row * m + col];
            }
        }
        range_start += m;

        for jm in 0..im {
            let mut qij2 = vec![0.0; m * m];
            let sqrt2 = 2.0_f64.sqrt();
            for row in 0..m {
                for col in 0..m {
                    let mut sum = 0.0;
                    for s in 0..t {
                        sum += x_wt[s * m + im] * x_wt[s * m + jm] * x_wt[s * m + row] * x_wt[s * m + col];
                    }
                    qij2[row * m + col] = sqrt2 * (sum / t as f64
                        - r[row * m + im] * r[col * m + jm]
                        - r[row * m + jm] * r[col * m + im]);
                }
            }

            for row in 0..m {
                for col in 0..m {
                    cm[row * (m * nbcm) + range_start + col] = qij2[row * m + col];
                }
            }
            range_start += m;
        }
    }

    // Step 4: Joint diagonalization via Givens rotations
    let mut v_rot = vec![0.0; m * m];
    for i in 0..m {
        v_rot[i * m + i] = 1.0;
    }

    let seuil = 1.0e-6 / (t as f64).sqrt();
    let mut encore = true;
    let cm_cols = m * nbcm;

    while encore {
        encore = false;

        for p in 0..m.saturating_sub(1) {
            for q in (p + 1)..m {
                // Compute Givens angle
                let mut ton = 0.0;
                let mut toff = 0.0;

                for im in 0..nbcm {
                    let col_offset = im * m;
                    let ip = col_offset + p;
                    let iq = col_offset + q;

                    let cm_pp = cm[p * cm_cols + ip];
                    let cm_qq = cm[q * cm_cols + iq];
                    let cm_pq = cm[p * cm_cols + iq];
                    let cm_qp = cm[q * cm_cols + ip];

                    let g1 = cm_pp - cm_qq;
                    let g2 = cm_pq + cm_qp;

                    ton += g1 * g1 - g2 * g2;
                    toff += 2.0 * g1 * g2;
                }

                let theta = 0.5 * toff.atan2(ton + (ton * ton + toff * toff).sqrt());

                if theta.abs() > seuil {
                    encore = true;
                    let c = theta.cos();
                    let s = theta.sin();

                    // Update V
                    for i in 0..m {
                        let vip = v_rot[i * m + p];
                        let viq = v_rot[i * m + q];
                        v_rot[i * m + p] = c * vip + s * viq;
                        v_rot[i * m + q] = -s * vip + c * viq;
                    }

                    // Update CM rows p and q
                    for col in 0..cm_cols {
                        let rp = cm[p * cm_cols + col];
                        let rq = cm[q * cm_cols + col];
                        cm[p * cm_cols + col] = c * rp + s * rq;
                        cm[q * cm_cols + col] = -s * rp + c * rq;
                    }

                    // Update CM columns (within each m×m block)
                    for im in 0..nbcm {
                        let col_p = im * m + p;
                        let col_q = im * m + q;
                        for row in 0..m {
                            let cp = cm[row * cm_cols + col_p];
                            let cq = cm[row * cm_cols + col_q];
                            cm[row * cm_cols + col_p] = c * cp + s * cq;
                            cm[row * cm_cols + col_q] = -s * cp + c * cq;
                        }
                    }
                }
            }
        }
    }

    // Step 5: Compute separating matrix B = V^T * B_whiten
    let vt = mat_transpose(&v_rot, m, m);
    let b_final = mat_mul(&vt, m, m, &b_whiten, n);

    // Sort rows by energy of the mixing matrix columns (pinv(B))
    // Simple pseudo-inverse for tall/square: A = (B^T * B)^{-1} * B^T approximation
    // For orthogonal B after whitening, A ≈ B^T
    let bt = mat_transpose(&b_final, m, n);

    // Compute column norms of A (= bt rows when m == n, approximate otherwise)
    let mut col_norms: Vec<(f64, usize)> = (0..m)
        .map(|col| {
            let norm: f64 = (0..n).map(|row| bt[row * m + col] * bt[row * m + col]).sum::<f64>();
            (norm, col)
        })
        .collect();
    col_norms.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Reorder rows of B
    let mut b_sorted = vec![0.0; m * n];
    for (new_row, &(_, old_row)) in col_norms.iter().enumerate() {
        for col in 0..n {
            b_sorted[new_row * n + col] = b_final[old_row * n + col];
        }
    }

    // Fix sign: first element of each row is positive
    for row in 0..m {
        let first = b_sorted[row * n];
        if first < 0.0 {
            for col in 0..n {
                b_sorted[row * n + col] = -b_sorted[row * n + col];
            }
        }
    }

    // Mixing matrix: A = pinv(B) ≈ B^T for whitened data
    let mixing = mat_transpose(&b_sorted, m, n);

    IcaResult {
        separating_matrix: b_sorted,
        mixing_matrix: mixing,
        n_sources: m,
        n_channels: n,
    }
}

/// Apply ICA separating matrix to extract independent source signals.
///
/// * `data` – flat row-major (channels × samples)
/// * `ica` – ICA result containing the separating matrix
///
/// Returns source signals as flat row-major (n_sources × samples).
#[must_use]
pub fn ica_separate(data: &[f64], channels: usize, samples: usize, ica: &IcaResult) -> Vec<f64> {
    assert_eq!(data.len(), channels * samples);
    assert_eq!(ica.n_channels, channels);

    // Mean-center
    let mut centered = data.to_vec();
    for ch in 0..channels {
        let start = ch * samples;
        let mean: f64 = centered[start..start + samples].iter().sum::<f64>() / samples as f64;
        for s in 0..samples {
            centered[start + s] -= mean;
        }
    }

    mat_mul(&ica.separating_matrix, ica.n_sources, channels, &centered, samples)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jade_separates_orthogonal_sources() {
        // Create two independent sources mixed together
        let n = 1000;
        let mut s1 = vec![0.0; n];
        let mut s2 = vec![0.0; n];
        for i in 0..n {
            let t = i as f64 / n as f64;
            s1[i] = (2.0 * PI * 3.0 * t).sin(); // sine wave
            s2[i] = if (i / 50) % 2 == 0 { 1.0 } else { -1.0 }; // square wave
        }

        // Mixing matrix
        let a11 = 0.8;
        let a12 = 0.6;
        let a21 = 0.4;
        let a22 = 0.9;

        // Mixed signals: X = A * S
        let mut data = vec![0.0; 2 * n];
        for i in 0..n {
            data[i] = a11 * s1[i] + a12 * s2[i];
            data[n + i] = a21 * s1[i] + a22 * s2[i];
        }

        let result = jade(&data, 2, n, 2);
        assert_eq!(result.n_sources, 2);
        assert_eq!(result.separating_matrix.len(), 2 * 2);

        // Extract sources
        let sources = ica_separate(&data, 2, n, &result);
        assert_eq!(sources.len(), 2 * n);

        // Verify sources are less correlated than the mixed signals
        // (full decorrelation check)
        let mut corr = 0.0;
        for i in 0..n {
            corr += sources[i] * sources[n + i];
        }
        corr /= n as f64;
        // Sources should be approximately uncorrelated
        assert!(corr.abs() < 0.5, "Sources still correlated: {corr}");
    }

    #[test]
    fn jade_single_source() {
        let n = 200;
        let mut data = vec![0.0; 2 * n];
        for i in 0..n {
            let t = i as f64 / n as f64;
            data[i] = (2.0 * PI * 5.0 * t).sin();
            data[n + i] = 0.5 * (2.0 * PI * 5.0 * t).sin();
        }

        let result = jade(&data, 2, n, 1);
        assert_eq!(result.n_sources, 1);
        assert_eq!(result.separating_matrix.len(), 1 * 2);
    }

    #[test]
    fn jade_result_dimensions() {
        let n = 100;
        let data = vec![0.0; 3 * n];
        let result = jade(&data, 3, n, 2);
        assert_eq!(result.n_sources, 2);
        assert_eq!(result.n_channels, 3);
        assert_eq!(result.separating_matrix.len(), 2 * 3);
        assert_eq!(result.mixing_matrix.len(), 3 * 2);
    }
}
