use dashu::float::FBig;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct HighPrecisionComplex {
    pub re: FBig,
    pub im: FBig,
}

impl Serialize for HighPrecisionComplex {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("HighPrecisionComplex", 2)?;
        s.serialize_field("re", &self.re.to_string())?;
        s.serialize_field("im", &self.im.to_string())?;
        s.end()
    }
}

impl HighPrecisionComplex {
    pub fn new(re: FBig, im: FBig) -> Self {
        Self { re, im }
    }

    pub fn norm_sqr(&self) -> FBig {
        &self.re * &self.re + &self.im * &self.im
    }

    pub fn square_add(&self, c: &HighPrecisionComplex) -> Self {
        let re = &self.re * &self.re - &self.im * &self.im + &c.re;
        let im = FBig::from(2) * &self.re * &self.im + &c.im;
        Self { re, im }
    }
}

#[derive(Clone, Debug)]
pub struct HighPrecisionPosition {
    pub x: FBig,
    pub y: FBig,
}

impl Serialize for HighPrecisionPosition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("HighPrecisionPosition", 3)?;
        s.serialize_field("x", &self.x.to_string())?;
        s.serialize_field("y", &self.y.to_string())?;
        s.serialize_field("precision_bits", &self.x.precision())?;
        s.end()
    }
}

impl HighPrecisionPosition {
    pub fn from_f32(x: f32, y: f32, precision: usize) -> Self {
        Self {
            x: FBig::try_from(x as f64).unwrap().with_precision(precision).value(),
            y: FBig::try_from(y as f64).unwrap().with_precision(precision).value(),
        }
    }

    pub fn to_f32(&self) -> (f32, f32) {
        (self.x.to_f64().value() as f32, self.y.to_f64().value() as f32)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ReferenceOrbit {
    /// Orbit values stored in f64 to avoid fixed-point traps.
    /// Near periodic attractors, consecutive Z values can differ by less than
    /// f32 ULP, causing the f32 orbit to get "stuck" at a constant value.
    pub values: Vec<[f64; 2]>,
    pub center: [f64; 2],
    pub escape_iteration: usize,
    /// The max_iterations used when computing this orbit.
    /// Prevents unnecessary recomputation when orbit escapes early.
    pub computed_max_iterations: usize,
    pub sa_coefficients: Option<SACoefficients>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SACoefficients {
    /// Pre-scaled coefficients in f64. N terms where coefficients[k] = C_k * scale^(k+1).
    /// CPU evaluates in f64 directly. GPU casts to f32 at upload time.
    pub coefficients: Vec<[f64; 2]>,
    /// Scale stored as f64 — at ultra-deep zoom, scale (= max_delta ≈ zoom)
    /// can be below f32 min subnormal (~1.4e-45) and would underflow to 0.
    pub scale: f64,
    pub skip_iterations: usize,
}

/// Quick f64 orbit test — returns the iteration at which escape occurs (or max_iterations if none).
pub fn test_orbit_f64(c_re: f64, c_im: f64, max_iterations: usize, bailout: f64) -> usize {
    let mut z_re = 0.0;
    let mut z_im = 0.0;
    for i in 0..max_iterations {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        if re2 + im2 > bailout {
            return i;
        }
        z_im = 2.0 * z_re * z_im + c_im;
        z_re = re2 - im2 + c_re;
    }
    max_iterations
}

pub fn needs_perturbation(zoom_x: f64, width: usize) -> bool {
    let pixel_spacing = (2.0 * zoom_x) / width as f64;
    pixel_spacing < 1e-5
}

pub fn required_precision_bits(zoom_x: f64) -> usize {
    let log2_zoom = -(zoom_x.ln() / std::f64::consts::LN_2);
    let bits = (log2_zoom as usize).max(0) + 64;
    bits.max(64)
}

/// Compute SA coefficients from orbit values using the scaled recurrence.
///
/// Tracks `num_terms` terms plus one guard term for tolerance checking.
/// The recurrence for coefficient k (0-indexed) is:
/// - k=0: c[0]_new = 2·Z·c[0] + δ
/// - k≥1: c[k]_new = 2·Z·c[k] + Σ_{j=0}^{k-1} c[j]·c[k-1-j]  (Cauchy product)
fn compute_sa_from_orbit(
    orbit_values: &[[f64; 2]],
    escape_iteration: usize,
    delta: f64,
    num_terms: usize,
) -> Option<SACoefficients> {
    let total = num_terms + 1; // +1 guard term for tolerance
    let mut coeffs: Vec<[f64; 2]> = vec![[0.0; 2]; total];
    let mut new_coeffs: Vec<[f64; 2]> = vec![[0.0; 2]; total];

    let mut skip_iterations = 0;
    let mut final_coeffs: Vec<[f64; 2]> = vec![[0.0; 2]; num_terms];

    let limit = escape_iteration.min(orbit_values.len());

    for i in 0..limit {
        let z_re = orbit_values[i][0];
        let z_im = orbit_values[i][1];

        if i > 0 {
            // Check finiteness of all coefficients
            if coeffs.iter().any(|c| !c[0].is_finite() || !c[1].is_finite()) {
                break;
            }

            // Tolerance: |guard| / |sum of terms| < 1e-6
            let guard_mag = coeffs[num_terms][0].hypot(coeffs[num_terms][1]);
            let sum_re: f64 = coeffs[..num_terms].iter().map(|c| c[0]).sum();
            let sum_im: f64 = coeffs[..num_terms].iter().map(|c| c[1]).sum();
            let denominator = sum_re.hypot(sum_im);

            if denominator > 0.0 && guard_mag / denominator < 1e-6 {
                skip_iterations = i;
                final_coeffs.copy_from_slice(&coeffs[..num_terms]);
            } else {
                break;
            }
        }

        new_coeffs.fill([0.0, 0.0]);

        for k in 0..total {
            // 2·Z·c[k]
            let two_z_re = 2.0 * (z_re * coeffs[k][0] - z_im * coeffs[k][1]);
            let two_z_im = 2.0 * (z_re * coeffs[k][1] + z_im * coeffs[k][0]);

            if k == 0 {
                new_coeffs[0] = [two_z_re + delta, two_z_im];
            } else {
                // Cauchy product: Σ_{j=0}^{k-1} c[j]·c[k-1-j]
                let mut cauchy_re = 0.0;
                let mut cauchy_im = 0.0;
                for j in 0..k {
                    let a = coeffs[j];
                    let b = coeffs[k - 1 - j];
                    cauchy_re += a[0] * b[0] - a[1] * b[1];
                    cauchy_im += a[0] * b[1] + a[1] * b[0];
                }
                new_coeffs[k] = [two_z_re + cauchy_re, two_z_im + cauchy_im];
            }
        }

        std::mem::swap(&mut coeffs, &mut new_coeffs);
    }

    if skip_iterations > 0
        && final_coeffs.iter().all(|c| c[0].is_finite() && c[1].is_finite())
        && delta.is_finite()
        && delta > 0.0
    {
        Some(SACoefficients {
            coefficients: final_coeffs,
            scale: delta,
            skip_iterations,
        })
    } else {
        None
    }
}

/// Evaluate SA polynomial using Horner's method in f64.
/// Returns (eps_re, eps_im) = u * (c[0] + u * (c[1] + ... + u * c[N-1]))
pub fn evaluate_sa_horner(coefficients: &[[f64; 2]], u_re: f64, u_im: f64) -> (f64, f64) {
    let n = coefficients.len();
    let mut t_re = coefficients[n - 1][0];
    let mut t_im = coefficients[n - 1][1];
    for k in (0..n - 1).rev() {
        // t = c[k] + u * t
        let ut_re = u_re * t_re - u_im * t_im;
        let ut_im = u_re * t_im + u_im * t_re;
        t_re = coefficients[k][0] + ut_re;
        t_im = coefficients[k][1] + ut_im;
    }
    // eps = u * t
    (u_re * t_re - u_im * t_im, u_re * t_im + u_im * t_re)
}

pub fn compute_reference_orbit_with_sa(
    center: &HighPrecisionComplex,
    max_iterations: usize,
    bailout: f32,
    max_delta: f64,
    sa_order: usize,
) -> ReferenceOrbit {
    let precision = center.re.precision();
    let bailout_big: FBig = FBig::try_from(bailout as f64)
        .unwrap()
        .with_precision(precision)
        .value();
    let mut z = HighPrecisionComplex::new(
        FBig::ZERO.with_precision(precision).value(),
        FBig::ZERO.with_precision(precision).value(),
    );

    let mut values = Vec::with_capacity(max_iterations);
    let mut escape_iteration = max_iterations;

    for i in 0..max_iterations {
        values.push([z.re.to_f64().value(), z.im.to_f64().value()]);

        z = z.square_add(center);

        if z.norm_sqr() > bailout_big {
            values.push([z.re.to_f64().value(), z.im.to_f64().value()]);
            escape_iteration = i + 1;
            break;
        }
    }

    let sa_coefficients = compute_sa_from_orbit(&values, escape_iteration, max_delta, sa_order);

    ReferenceOrbit {
        values,
        center: [center.re.to_f64().value(), center.im.to_f64().value()],
        escape_iteration,
        computed_max_iterations: max_iterations,
        sa_coefficients,
    }
}

/// Recompute SA coefficients from stored orbit values with a new max_delta.
/// Called when the orbit is reused at a different zoom level:
/// - Zoom-in: smaller delta → SA converges better → more skip iterations.
/// - Zoom-out: larger delta → SA tolerance fails earlier → fewer skip iterations.
pub fn recompute_sa(orbit: &ReferenceOrbit, max_delta: f64, sa_order: usize) -> Option<SACoefficients> {
    compute_sa_from_orbit(&orbit.values, orbit.escape_iteration, max_delta, sa_order)
}

pub fn iterate_perturbation(
    orbit: &ReferenceOrbit,
    dc_re: f64,
    dc_im: f64,
    max_iterations: usize,
    bailout: f64,
) -> (f64, usize) {
    let limit = orbit.escape_iteration.min(max_iterations);

    // SA polynomial skip
    let (mut d_re, mut d_im, start_n) = if let Some(sa) = &orbit.sa_coefficients {
        if sa.skip_iterations > 0 && sa.skip_iterations < limit && sa.scale > 0.0 {
            let u_re = dc_re / sa.scale;
            let u_im = dc_im / sa.scale;
            let (eps_re, eps_im) = evaluate_sa_horner(&sa.coefficients, u_re, u_im);
            (eps_re, eps_im, sa.skip_iterations)
        } else {
            (0.0, 0.0, 0)
        }
    } else {
        (0.0, 0.0, 0)
    };

    let mut iterations = start_n;

    // Pre-compute c in f64 for rebasing fallback and orbit-escape fallback
    let c_re = orbit.center[0] + dc_re;
    let c_im = orbit.center[1] + dc_im;

    for n in start_n..limit {
        // Use f64 orbit values directly
        let z_re = orbit.values[n][0];
        let z_im = orbit.values[n][1];

        // Rebasing: when |δ|² > |Z|², perturbation has lost precision.
        let d_norm_sq = d_re * d_re + d_im * d_im;
        let z_norm_sq = z_re * z_re + z_im * z_im;
        if d_norm_sq > z_norm_sq {
            let mut fz_re = z_re + d_re;
            let mut fz_im = z_im + d_im;
            for i in n..max_iterations {
                let re2 = fz_re * fz_re;
                let im2 = fz_im * fz_im;
                let norm_sq = re2 + im2;
                if norm_sq > bailout {
                    return (norm_sq, i);
                }
                let new_re = re2 - im2 + c_re;
                fz_im = 2.0 * fz_re * fz_im + c_im;
                fz_re = new_re;
            }
            return (0.0, max_iterations);
        }

        // δ(n+1) = 2·Z(n)·δ(n) + δ(n)² + δc
        let new_d_re = 2.0 * (z_re * d_re - z_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
        let new_d_im = 2.0 * (z_re * d_im + z_im * d_re) + 2.0 * d_re * d_im + dc_im;
        d_re = new_d_re;
        d_im = new_d_im;

        // Check escape: |Z(n+1) + δ(n+1)|²
        if n + 1 < orbit.values.len() {
            let full_re = orbit.values[n + 1][0] + d_re;
            let full_im = orbit.values[n + 1][1] + d_im;
            let norm_sq = full_re * full_re + full_im * full_im;
            if norm_sq > bailout {
                return (norm_sq, n + 1);
            }
        }

        iterations = n + 1;
    }

    // Fallback: reference orbit escaped before max_iterations — direct f64 iteration
    if iterations < max_iterations {
        let (mut z_re, mut z_im) = if limit < orbit.values.len() {
            (orbit.values[limit][0] + d_re, orbit.values[limit][1] + d_im)
        } else {
            (d_re, d_im)
        };

        for i in iterations..max_iterations {
            let re2 = z_re * z_re;
            let im2 = z_im * z_im;
            let norm_sq = re2 + im2;
            if norm_sq > bailout {
                return (norm_sq, i);
            }
            let new_re = re2 - im2 + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_re;
        }
    }

    (0.0, max_iterations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sa_coefficients_match_brute_force() {
        // Use c = -0.75 + 0.1i (near the main cardioid boundary, non-escaping)
        let precision = 128;
        let center = HighPrecisionComplex::new(
            FBig::try_from(-0.75).unwrap().with_precision(precision).value(),
            FBig::try_from(0.1).unwrap().with_precision(precision).value(),
        );

        let max_delta = 1e-6;
        let orbit = compute_reference_orbit_with_sa(&center, 1000, 65536.0, max_delta, 6);

        // Verify SA was computed
        let sa = orbit.sa_coefficients.as_ref().expect("SA should be computed");
        assert!(sa.skip_iterations > 0, "Should skip at least some iterations");

        // Pick a test delta smaller than max_delta
        let dc_re = 3e-7;
        let dc_im = 5e-7;

        // Evaluate using Horner's method (matching CPU iterate_perturbation_inner)
        let u_re = dc_re / sa.scale;
        let u_im = dc_im / sa.scale;
        let (eps_re, eps_im) = evaluate_sa_horner(&sa.coefficients, u_re, u_im);

        // Brute-force: iterate perturbation from n=0 to skip_iterations
        let mut d_re = 0.0;
        let mut d_im = 0.0;
        for n in 0..sa.skip_iterations {
            let z_re = orbit.values[n][0];
            let z_im = orbit.values[n][1];
            let new_d_re = 2.0 * (z_re * d_re - z_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
            let new_d_im = 2.0 * (z_re * d_im + z_im * d_re) + 2.0 * d_re * d_im + dc_im;
            d_re = new_d_re;
            d_im = new_d_im;
        }

        // Compare — SA polynomial vs brute force (both f64)
        let err_re = (eps_re - d_re).abs();
        let err_im = (eps_im - d_im).abs();
        let brute_mag = (d_re * d_re + d_im * d_im).sqrt();
        let rel_err = ((err_re * err_re + err_im * err_im).sqrt()) / brute_mag.max(1e-30);
        assert!(
            rel_err < 0.05,
            "SA f64 should approximate brute force within 5%, got relative error {rel_err}"
        );
    }

    fn make_orbit(center_re: f64, center_im: f64, max_iterations: usize, max_delta: f64) -> ReferenceOrbit {
        make_orbit_with_precision(center_re, center_im, max_iterations, max_delta, 128)
    }

    fn make_orbit_with_precision(center_re: f64, center_im: f64, max_iterations: usize, max_delta: f64, precision: usize) -> ReferenceOrbit {
        let center = HighPrecisionComplex::new(
            FBig::try_from(center_re).unwrap().with_precision(precision).value(),
            FBig::try_from(center_im).unwrap().with_precision(precision).value(),
        );
        compute_reference_orbit_with_sa(&center, max_iterations, 4.0, max_delta, 6)
    }

    /// Direct f64 Mandelbrot iteration: z = z² + c, starting from z = 0.
    fn direct_iterate(c_re: f64, c_im: f64, max_iterations: usize, bailout: f64) -> (f64, usize) {
        let mut z_re = 0.0;
        let mut z_im = 0.0;
        for i in 0..max_iterations {
            let re2 = z_re * z_re;
            let im2 = z_im * z_im;
            let norm = re2 + im2;
            if norm > bailout {
                return (norm, i);
            }
            let new_re = re2 - im2 + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_re;
        }
        (0.0, max_iterations)
    }

    #[test]
    fn dc_zero_matches_reference_non_escaping() {
        let orbit = make_orbit(-0.5, 0.0, 100, 0.01);
        assert_eq!(orbit.escape_iteration, 100, "Reference should not escape");

        let (norm_sq, iterations) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(iterations, 100, "dc=0 with non-escaping reference should give max_iterations");
        assert_eq!(norm_sq, 0.0, "dc=0 should have zero norm_sq");
    }

    #[test]
    fn dc_zero_matches_reference_escaping() {
        let orbit = make_orbit(0.3, 0.0, 100, 0.01);
        let ref_escape = orbit.escape_iteration;
        assert!(ref_escape < 100, "Reference should escape, got {ref_escape}");

        let (_, direct_iter) = direct_iterate(0.3, 0.0, 100, 4.0);
        assert_eq!(ref_escape, direct_iter, "Reference orbit escape should match direct iteration");

        let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(
            pert_iter, direct_iter,
            "Perturbation dc=0 should match direct: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn perturbation_matches_direct_small_dc() {
        let orbit = make_orbit(-0.5, 0.0, 200, 0.01);
        assert_eq!(orbit.escape_iteration, 200, "Reference should not escape");

        let test_cases: &[(f64, f64)] = &[
            (0.001, 0.001),
            (-0.005, 0.003),
            (0.0, 0.008),
            (0.009, 0.0),
        ];

        for &(dc_re, dc_im) in test_cases {
            let (pert_norm, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
            let (direct_norm, direct_iter) = direct_iterate(-0.5 + dc_re, dc_im, 200, 4.0);

            assert_eq!(
                pert_iter, direct_iter,
                "Iteration mismatch for dc=({dc_re}, {dc_im}): pert={pert_iter}, direct={direct_iter}"
            );
            if direct_iter < 200 {
                let rel_diff = (pert_norm - direct_norm).abs() / direct_norm.max(1e-30);
                assert!(
                    rel_diff < 1e-6,
                    "Norm mismatch for dc=({dc_re}, {dc_im}): pert={pert_norm}, direct={direct_norm}"
                );
            }
        }
    }

    #[test]
    fn perturbation_step_by_step_matches_direct() {
        let center_re = -0.5;
        let center_im = 0.0;
        let orbit = make_orbit(center_re, center_im, 50, 0.01);

        let dc_re = 0.002;
        let dc_im = 0.003;
        let c_re = center_re + dc_re;
        let c_im = center_im + dc_im;

        let mut z_re = 0.0;
        let mut z_im = 0.0;
        let mut d_re = 0.0;
        let mut d_im = 0.0;

        for n in 0..orbit.values.len().min(50) {
            let ref_re = orbit.values[n][0];
            let ref_im = orbit.values[n][1];

            let full_re = ref_re + d_re;
            let full_im = ref_im + d_im;
            let err = ((full_re - z_re).powi(2) + (full_im - z_im).powi(2)).sqrt();
            let z_mag = (z_re * z_re + z_im * z_im).sqrt().max(1e-10);
            assert!(
                err / z_mag < 1e-6 || err < 1e-12,
                "Step {n}: Z+δ=({full_re},{full_im}) vs z=({z_re},{z_im}), err={err}"
            );

            let new_d_re = 2.0 * (ref_re * d_re - ref_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
            let new_d_im = 2.0 * (ref_re * d_im + ref_im * d_re) + 2.0 * d_re * d_im + dc_im;
            d_re = new_d_re;
            d_im = new_d_im;

            let new_z_re = z_re * z_re - z_im * z_im + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_z_re;
        }
    }

    #[test]
    fn escaping_orbit_stores_escaped_value() {
        let orbit = make_orbit(0.3, 0.0, 100, 0.01);
        assert!(orbit.escape_iteration < 100, "Reference should escape");
        assert_eq!(
            orbit.values.len(),
            orbit.escape_iteration + 1,
            "Orbit should store Z_0 through Z_escape"
        );
        let last = orbit.values[orbit.escape_iteration];
        let norm_sq = last[0] * last[0] + last[1] * last[1];
        assert!(norm_sq > 4.0, "Last orbit value should be escaped");
    }

    #[test]
    fn escaping_reference_non_escaping_pixel() {
        let orbit = make_orbit(0.3, 0.0, 200, 0.5);
        assert!(orbit.escape_iteration < 200, "Reference should escape");

        let dc_re = -0.8;
        let dc_im = 0.0;

        let (pert_norm, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
        let (direct_norm, direct_iter) = direct_iterate(0.3 + dc_re, dc_im, 200, 4.0);

        assert_eq!(
            pert_iter, direct_iter,
            "Interior pixel: pert={pert_iter}, direct={direct_iter}"
        );
        assert_eq!(pert_norm, direct_norm, "Both should be 0.0 for interior");
    }

    #[test]
    fn grid_perturbation_vs_direct() {
        let center_re = -0.75;
        let center_im = 0.1;
        let orbit = make_orbit(center_re, center_im, 300, 0.1);
        let max_iter = 300;

        let mut mismatches = 0;
        let total = 11 * 11;
        for i in 0..11 {
            for j in 0..11 {
                let dc_re = (i as f64 - 5.0) * 0.005;
                let dc_im = (j as f64 - 5.0) * 0.005;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, max_iter, 4.0);
                let (_, direct_iter) = direct_iterate(center_re + dc_re, center_im + dc_im, max_iter, 4.0);

                if pert_iter != direct_iter {
                    mismatches += 1;
                }
            }
        }
        assert!(
            mismatches == 0,
            "Mismatches: {mismatches}/{total}"
        );
    }

    #[test]
    fn sa_skip_matches_no_skip() {
        let center_re = -0.75;
        let center_im = 0.1;

        let orbit_sa = make_orbit(center_re, center_im, 500, 1e-4);
        let has_sa = orbit_sa.sa_coefficients.is_some();

        let mut orbit_no_sa = make_orbit(center_re, center_im, 500, 1e-4);
        orbit_no_sa.sa_coefficients = None;

        if !has_sa {
            return;
        }

        let dc_re = 5e-5;
        let dc_im = 3e-5;

        let (norm_sa, iter_sa) = iterate_perturbation(&orbit_sa, dc_re, dc_im, 500, 4.0);
        let (norm_no_sa, iter_no_sa) = iterate_perturbation(&orbit_no_sa, dc_re, dc_im, 500, 4.0);

        assert!(
            (iter_sa as i64 - iter_no_sa as i64).unsigned_abs() <= 1,
            "SA skip should match no-skip: sa={iter_sa}, no_sa={iter_no_sa}"
        );
        if iter_sa < 500 && iter_no_sa < 500 {
            let rel_diff = (norm_sa - norm_no_sa).abs() / norm_no_sa.max(1e-30);
            assert!(
                rel_diff < 0.01,
                "Norm with SA should match no-SA: sa={norm_sa}, no_sa={norm_no_sa}"
            );
        }
    }

    #[test]
    fn max_iterations_one() {
        let orbit = make_orbit(-0.5, 0.0, 1, 0.01);
        let (norm, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 1, 4.0);
        assert_eq!(iter, 1);
        assert_eq!(norm, 0.0);
    }

    #[test]
    fn max_iterations_constrains_result() {
        let orbit = make_orbit(0.3, 0.0, 20, 0.01);
        let ref_escape = orbit.escape_iteration;
        assert!(ref_escape < 20);

        let (_, direct_5) = direct_iterate(0.3, 0.0, 5, 4.0);
        let (_, pert_5) = iterate_perturbation(&orbit, 0.0, 0.0, 5, 4.0);

        if ref_escape > 5 {
            assert_eq!(pert_5, 5, "Should be capped at max_iterations=5");
        } else {
            assert_eq!(pert_5, direct_5, "Should match direct at max_iterations=5");
        }
    }

    #[test]
    fn orbit_values_match_direct_f64() {
        let orbit = make_orbit(-0.5, 0.0, 50, 0.01);

        let mut z_re = 0.0;
        let mut z_im = 0.0;
        for i in 0..50.min(orbit.values.len()) {
            let ref_re = orbit.values[i][0];
            let ref_im = orbit.values[i][1];

            let err = ((ref_re - z_re).powi(2) + (ref_im - z_im).powi(2)).sqrt();
            assert!(
                err < 1e-10,
                "Orbit value mismatch at step {i}: orbit=({ref_re},{ref_im}) vs direct=({z_re},{z_im})"
            );

            let new_re = z_re * z_re - z_im * z_im - 0.5;
            z_im = 2.0 * z_re * z_im;
            z_re = new_re;
        }
    }

    #[test]
    fn needs_perturbation_threshold() {
        assert!(!needs_perturbation(1.0, 1920), "Default zoom should not need perturbation");
        assert!(!needs_perturbation(0.01, 1920), "Moderate zoom should not need perturbation");
        assert!(needs_perturbation(1e-7, 1920), "Deep zoom should need perturbation");
        assert!(needs_perturbation(1e-10, 1920), "Very deep zoom should need perturbation");
    }

    #[test]
    fn required_precision_bits_increases_with_zoom() {
        let bits_shallow = required_precision_bits(1.0);
        let bits_deep = required_precision_bits(1e-10);
        let bits_very_deep = required_precision_bits(1e-30);

        assert!(bits_shallow >= 64, "Should always be at least 64 bits");
        assert!(bits_deep > bits_shallow, "Deeper zoom needs more precision");
        assert!(bits_very_deep > bits_deep, "Even deeper zoom needs even more");
    }

    #[test]
    fn sa_not_computed_for_zero_delta() {
        let orbit = make_orbit(-0.75, 0.1, 100, 1e-20);
        let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        let (_, direct_iter) = direct_iterate(-0.75, 0.1, 100, 4.0);
        assert_eq!(pert_iter, direct_iter);
    }

    #[test]
    fn test_orbit_f64_escaping() {
        assert_eq!(test_orbit_f64(1.0, 0.0, 100, 4.0), 3);
        assert_eq!(test_orbit_f64(0.0, 0.0, 100, 4.0), 100);
        assert_eq!(test_orbit_f64(2.0, 0.0, 100, 4.0), 2);
        assert_eq!(test_orbit_f64(-2.0, 0.0, 100, 4.0), 100);
        let esc = test_orbit_f64(0.3, 0.6, 1000, 4.0);
        assert!(esc < 1000, "Should escape");
    }

    #[test]
    fn high_precision_complex_square_add() {
        let precision = 128;
        let z = HighPrecisionComplex::new(
            FBig::try_from(3.0).unwrap().with_precision(precision).value(),
            FBig::try_from(4.0).unwrap().with_precision(precision).value(),
        );
        let c = HighPrecisionComplex::new(
            FBig::try_from(1.0).unwrap().with_precision(precision).value(),
            FBig::try_from(2.0).unwrap().with_precision(precision).value(),
        );
        let result = z.square_add(&c);
        let re = result.re.to_f64().value();
        let im = result.im.to_f64().value();
        assert!((re - (-6.0)).abs() < 1e-10, "re={re}");
        assert!((im - 26.0).abs() < 1e-10, "im={im}");
    }

    #[test]
    fn high_precision_complex_norm_sqr() {
        let precision = 128;
        let z = HighPrecisionComplex::new(
            FBig::try_from(3.0).unwrap().with_precision(precision).value(),
            FBig::try_from(4.0).unwrap().with_precision(precision).value(),
        );
        let norm = z.norm_sqr().to_f64().value();
        assert!((norm - 25.0).abs() < 1e-10, "norm_sqr={norm}");
    }

    #[test]
    fn escape_on_last_perturbation_iteration() {
        let orbit = make_orbit(0.5, 0.0, 100, 0.01);
        let limit = orbit.escape_iteration;
        assert!(limit < 100);

        let (norm, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(iter, limit, "dc=0 should escape at reference escape iteration");
        assert!(norm > 4.0, "Should have escaped norm");
    }

    #[test]
    fn non_escaping_orbit_last_iteration_pixel_enters_fallback() {
        let orbit = make_orbit(-0.5, 0.0, 50, 0.1);
        assert_eq!(orbit.escape_iteration, 50, "Reference should not escape");
        assert_eq!(orbit.values.len(), 50, "Non-escaping orbit has max_iterations values");

        let dc_re = 0.06;
        let dc_im = 0.06;

        let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 50, 4.0);
        let (_, direct_iter) = direct_iterate(-0.5 + dc_re, dc_im, 50, 4.0);

        assert_eq!(
            pert_iter, direct_iter,
            "Last-iter edge: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn max_iterations_larger_than_orbit() {
        let orbit = make_orbit(-0.5, 0.0, 50, 0.01);
        assert_eq!(orbit.escape_iteration, 50);

        let dc_re = 0.001;
        let dc_im = 0.001;

        let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 100, 4.0);
        let (_, direct_iter) = direct_iterate(-0.5 + dc_re, dc_im, 100, 4.0);

        assert_eq!(
            pert_iter, direct_iter,
            "Extended max_iter: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn sa_disabled_when_skip_equals_limit() {
        let mut orbit = make_orbit(-0.75, 0.1, 100, 1e-6);
        if let Some(ref mut sa) = orbit.sa_coefficients {
            sa.skip_iterations = 100;
        }

        let (_, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        let (_, direct) = direct_iterate(-0.75, 0.1, 100, 4.0);
        assert_eq!(iter, direct);
    }

    #[test]
    fn different_bailout_values() {
        let center = HighPrecisionComplex::new(
            FBig::try_from(0.3).unwrap().with_precision(128).value(),
            FBig::try_from(0.5).unwrap().with_precision(128).value(),
        );

        for bailout in [4.0, 16.0, 256.0, 65536.0] {
            let orbit = compute_reference_orbit_with_sa(&center, 200, bailout as f32, 0.01, 6);

            let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 200, bailout);
            let (_, direct_iter) = direct_iterate(0.3, 0.5, 200, bailout);

            assert_eq!(
                pert_iter, direct_iter,
                "Bailout {bailout}: pert={pert_iter}, direct={direct_iter}"
            );
        }
    }

    #[test]
    fn high_precision_position_roundtrip() {
        let pos = HighPrecisionPosition::from_f32(1.5, -2.5, 128);
        let (x, y) = pos.to_f32();
        assert!((x - 1.5).abs() < 1e-6, "x roundtrip: {x}");
        assert!((y - (-2.5)).abs() < 1e-6, "y roundtrip: {y}");
    }

    #[test]
    fn perturbation_correct_when_d_dominates_z() {
        let mut orbit = make_orbit(-0.5, 0.0, 200, 1.0);
        orbit.sa_coefficients = None;
        assert_eq!(orbit.escape_iteration, 200);

        let grid_size = 16;
        let mut mismatches = 0;
        let total = grid_size * grid_size;

        for py in 0..grid_size {
            for px in 0..grid_size {
                let dc_re = px as f64 / (grid_size - 1) as f64 * 2.0 - 1.0;
                let dc_im = py as f64 / (grid_size - 1) as f64 * 2.0 - 1.0;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
                let (_, direct_iter) = direct_iterate(-0.5 + dc_re, dc_im, 200, 4.0);

                if pert_iter != direct_iter {
                    mismatches += 1;
                }
            }
        }

        assert!(
            mismatches == 0,
            "Large dc (d >> Z): {mismatches}/{total} mismatches"
        );
    }

    #[test]
    fn escaping_reference_grid_perturbation_vs_direct() {
        let center_re = -0.75;
        let center_im = 0.15;
        let max_iter = 200;

        let orbit = make_orbit(center_re, center_im, max_iter, 0.1);
        assert!(
            orbit.escape_iteration < max_iter,
            "Reference should escape (at {})",
            orbit.escape_iteration
        );

        let mut mismatches = 0;
        let total = 11 * 11;
        for i in 0..11 {
            for j in 0..11 {
                let dc_re = (i as f64 - 5.0) * 0.005;
                let dc_im = (j as f64 - 5.0) * 0.005;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, max_iter, 4.0);
                let (_, direct_iter) = direct_iterate(center_re + dc_re, center_im + dc_im, max_iter, 4.0);

                if pert_iter != direct_iter {
                    mismatches += 1;
                }
            }
        }
        assert!(
            mismatches == 0,
            "Escaping reference grid: {mismatches}/{total} mismatches"
        );
    }

    #[test]
    fn recompute_sa_increases_skip_at_deeper_zoom() {
        let z_re = -0.46113;
        let z_im = -0.33428;
        let orbit_len = 1500;
        let mut values = Vec::with_capacity(orbit_len);
        values.push([0.0, 0.0]);
        for _ in 1..orbit_len {
            values.push([z_re, z_im]);
        }

        let wide_delta = 1e-10;
        let deep_delta = 1e-16;

        let mut orbit = ReferenceOrbit {
            values,
            center: [z_re, z_im],
            escape_iteration: orbit_len,
            computed_max_iterations: orbit_len,
            sa_coefficients: None,
        };

        let original_sa = recompute_sa(&orbit, wide_delta, 6);
        let original_skip = original_sa.as_ref().map_or(0, |sa| sa.skip_iterations);
        orbit.sa_coefficients = original_sa;

        let new_sa = recompute_sa(&orbit, deep_delta, 6);
        let new_skip = new_sa.as_ref().map_or(0, |sa| sa.skip_iterations);

        assert!(
            original_skip > 100,
            "Original SA should skip a meaningful number of iterations: {original_skip}"
        );
        assert!(
            new_skip > original_skip,
            "Recomputed SA at deeper zoom should skip more: \
             original={original_skip}, recomputed={new_skip}"
        );
        assert!(
            new_skip > original_skip + 50,
            "Recomputed SA should skip meaningfully more: \
             original={original_skip}, recomputed={new_skip}"
        );
    }

    #[test]
    fn sa_accuracy_at_high_skip_count() {
        let center_re = -0.6748093390464783;
        let center_im = -0.4372471585273743;

        let max_iter = 2000;
        let max_delta = 3e-20;

        let orbit_sa = make_orbit(center_re, center_im, max_iter, max_delta);
        let mut orbit_no_sa = make_orbit(center_re, center_im, max_iter, max_delta);
        orbit_no_sa.sa_coefficients = None;

        let skip = orbit_sa.sa_coefficients.as_ref().map_or(0, |sa| sa.skip_iterations);

        let grid_size = 16;
        let dc_scale = max_delta;
        let mut mismatches = 0;
        let total = grid_size * grid_size;

        for py in 0..grid_size {
            for px in 0..grid_size {
                let dc_re = (px as f64 / (grid_size - 1) as f64 * 2.0 - 1.0) * dc_scale;
                let dc_im = (py as f64 / (grid_size - 1) as f64 * 2.0 - 1.0) * dc_scale;

                let (_, iter_sa) = iterate_perturbation(&orbit_sa, dc_re, dc_im, max_iter, 65536.0);
                let (_, iter_no_sa) = iterate_perturbation(&orbit_no_sa, dc_re, dc_im, max_iter, 65536.0);

                if (iter_sa as i64 - iter_no_sa as i64).unsigned_abs() > 1 {
                    mismatches += 1;
                }
            }
        }

        assert!(
            mismatches * 100 / total < 5,
            "SA skip {skip}: {mismatches}/{total} mismatches vs brute-force perturbation"
        );
    }
}
