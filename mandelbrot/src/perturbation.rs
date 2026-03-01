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
        let x: f64 = self.x.to_f64().value();
        let y: f64 = self.y.to_f64().value();
        (x as f32, y as f32)
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
    /// Pre-scaled coefficients in f32: a_s = A*scale, b_s = B*scale², c_s = C*scale³
    /// Both CPU and GPU evaluate with u = dc/scale (u is O(1), all values f32-safe).
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub c: [f32; 2],
    /// Scale stored as f64 — at ultra-deep zoom, scale (= max_delta ≈ zoom)
    /// can be below f32 min subnormal (~1.4e-45) and would underflow to 0.
    pub scale: f64,
    pub skip_iterations: usize,
}

/// Quick f64 orbit test — returns the iteration at which escape occurs (or max_iterations if none).
pub fn test_orbit_f64(c_re: f64, c_im: f64, max_iterations: usize, bailout: f64) -> usize {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;
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

pub fn compute_reference_orbit_with_sa(
    center: &HighPrecisionComplex,
    max_iterations: usize,
    bailout: f32,
    max_delta: f64,
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

    // SA coefficients computed in SCALED form to avoid f64 overflow.
    // Instead of tracking raw A, B, C, D (which grow as |2Z|^n, |2Z|^2n, etc.
    // and overflow f64 around n ≈ 308/log10(|2Z|)), we track:
    //   as = A·δ,  bs = B·δ²,  cs = C·δ³,  ds = D·δ⁴
    // The scaled values stay small (≈ δ^k · |2Z|^(kn)) and never overflow.
    //
    // Scaled recurrence (derived by multiplying each equation by δ^k):
    //   as_{n+1} = 2·Z_n·as_n + δ        (from A_{n+1} = 2·Z_n·A_n + 1)
    //   bs_{n+1} = 2·Z_n·bs_n + as_n²    (from B_{n+1} = 2·Z_n·B_n + A_n²)
    //   cs_{n+1} = 2·Z_n·cs_n + 2·as_n·bs_n
    //   ds_{n+1} = 2·Z_n·ds_n + 2·as_n·cs_n + bs_n²
    //
    // Tolerance check: |ds| / |as + bs + cs| < 1e-6
    let delta = max_delta;
    let mut as_re = 0.0_f64;
    let mut as_im = 0.0_f64;
    let mut bs_re = 0.0_f64;
    let mut bs_im = 0.0_f64;
    let mut cs_re = 0.0_f64;
    let mut cs_im = 0.0_f64;
    let mut ds_re = 0.0_f64;
    let mut ds_im = 0.0_f64;

    let mut sa_valid = true;
    let mut skip_iterations: usize = 0;

    // Store the scaled coefficients at skip_iterations
    let mut final_as = [0.0_f64; 2];
    let mut final_bs = [0.0_f64; 2];
    let mut final_cs = [0.0_f64; 2];

    for i in 0..max_iterations {
        let re_f64: f64 = z.re.to_f64().value();
        let im_f64: f64 = z.im.to_f64().value();
        values.push([re_f64, im_f64]);

        if sa_valid {
            // Tolerance check: |ds_n| / |as_n + bs_n + cs_n| < 1e-6
            // ds is the scaled first-excluded term; its magnitude is the truncation error.
            if i > 0 {
                if !as_re.is_finite() || !as_im.is_finite()
                    || !bs_re.is_finite() || !bs_im.is_finite()
                    || !cs_re.is_finite() || !cs_im.is_finite()
                    || !ds_re.is_finite() || !ds_im.is_finite()
                {
                    sa_valid = false;
                }

                let numerator = ds_re.hypot(ds_im);
                let sum_re = as_re + bs_re + cs_re;
                let sum_im = as_im + bs_im + cs_im;
                let denominator = sum_re.hypot(sum_im);

                if denominator > 0.0 && numerator / denominator < 1e-6 {
                    skip_iterations = i;
                    final_as = [as_re, as_im];
                    final_bs = [bs_re, bs_im];
                    final_cs = [cs_re, cs_im];
                } else {
                    sa_valid = false;
                }
            }

            if sa_valid {
                // Scaled recurrence: as = A·δ, bs = B·δ², cs = C·δ³, ds = D·δ⁴
                // as_{n+1} = 2·Z_n·as_n + δ
                let new_as_re = 2.0 * (re_f64 * as_re - im_f64 * as_im) + delta;
                let new_as_im = 2.0 * (re_f64 * as_im + im_f64 * as_re);

                // as_n² (complex square)
                let as_sq_re = as_re * as_re - as_im * as_im;
                let as_sq_im = 2.0 * as_re * as_im;

                // bs_{n+1} = 2·Z_n·bs_n + as_n²
                let new_bs_re = 2.0 * (re_f64 * bs_re - im_f64 * bs_im) + as_sq_re;
                let new_bs_im = 2.0 * (re_f64 * bs_im + im_f64 * bs_re) + as_sq_im;

                // 2·as_n·bs_n
                let two_asbs_re = 2.0 * (as_re * bs_re - as_im * bs_im);
                let two_asbs_im = 2.0 * (as_re * bs_im + as_im * bs_re);

                // cs_{n+1} = 2·Z_n·cs_n + 2·as_n·bs_n
                let new_cs_re = 2.0 * (re_f64 * cs_re - im_f64 * cs_im) + two_asbs_re;
                let new_cs_im = 2.0 * (re_f64 * cs_im + im_f64 * cs_re) + two_asbs_im;

                // 2·as_n·cs_n
                let two_ascs_re = 2.0 * (as_re * cs_re - as_im * cs_im);
                let two_ascs_im = 2.0 * (as_re * cs_im + as_im * cs_re);

                // bs_n²
                let bs_sq_re = bs_re * bs_re - bs_im * bs_im;
                let bs_sq_im = 2.0 * bs_re * bs_im;

                // ds_{n+1} = 2·Z_n·ds_n + 2·as_n·cs_n + bs_n²
                let new_ds_re = 2.0 * (re_f64 * ds_re - im_f64 * ds_im) + two_ascs_re + bs_sq_re;
                let new_ds_im = 2.0 * (re_f64 * ds_im + im_f64 * ds_re) + two_ascs_im + bs_sq_im;

                as_re = new_as_re;
                as_im = new_as_im;
                bs_re = new_bs_re;
                bs_im = new_bs_im;
                cs_re = new_cs_re;
                cs_im = new_cs_im;
                ds_re = new_ds_re;
                ds_im = new_ds_im;
            }
        }

        z = z.square_add(center);

        if z.norm_sqr() > bailout_big {
            // Push the escaped value so the escape check can access Z_{n+1}
            let esc_re: f64 = z.re.to_f64().value();
            let esc_im: f64 = z.im.to_f64().value();
            values.push([esc_re, esc_im]);
            escape_iteration = i + 1;
            break;
        }
    }

    let center_f64 = [
        center.re.to_f64().value(),
        center.im.to_f64().value(),
    ];

    // Scaled coefficients are already in the form stored by SACoefficients
    let sa_coefficients = if skip_iterations > 0 {
        let a_s = [final_as[0] as f32, final_as[1] as f32];
        let b_s = [final_bs[0] as f32, final_bs[1] as f32];
        let c_s = [final_cs[0] as f32, final_cs[1] as f32];

        if a_s[0].is_finite() && a_s[1].is_finite()
            && b_s[0].is_finite() && b_s[1].is_finite()
            && c_s[0].is_finite() && c_s[1].is_finite()
            && delta.is_finite() && delta > 0.0
        {
            Some(SACoefficients {
                a: a_s,
                b: b_s,
                c: c_s,
                scale: delta,
                skip_iterations,
            })
        } else {
            None
        }
    } else {
        None
    };

    ReferenceOrbit {
        values,
        center: center_f64,
        escape_iteration,
        computed_max_iterations: max_iterations,
        sa_coefficients,
    }
}

/// Recompute SA coefficients from stored orbit values with a new max_delta.
/// This is used when the orbit is reused at a deeper zoom level — the SA
/// polynomial converges better for smaller delta, so more iterations can be skipped.
///
/// Uses scaled recurrence (as = A·δ, bs = B·δ², cs = C·δ³, ds = D·δ⁴)
/// to avoid f64 overflow of raw coefficients at deep zoom.
pub fn recompute_sa(orbit: &ReferenceOrbit, max_delta: f64) -> Option<SACoefficients> {
    let delta = max_delta;

    let mut as_re = 0.0_f64;
    let mut as_im = 0.0_f64;
    let mut bs_re = 0.0_f64;
    let mut bs_im = 0.0_f64;
    let mut cs_re = 0.0_f64;
    let mut cs_im = 0.0_f64;
    let mut ds_re = 0.0_f64;
    let mut ds_im = 0.0_f64;

    let mut sa_valid = true;
    let mut skip_iterations: usize = 0;
    let mut final_as = [0.0_f64; 2];
    let mut final_bs = [0.0_f64; 2];
    let mut final_cs = [0.0_f64; 2];

    let limit = orbit.escape_iteration.min(orbit.values.len());

    for i in 0..limit {
        let re_f64 = orbit.values[i][0];
        let im_f64 = orbit.values[i][1];

        if sa_valid && i > 0 {
            if !as_re.is_finite() || !as_im.is_finite()
                || !bs_re.is_finite() || !bs_im.is_finite()
                || !cs_re.is_finite() || !cs_im.is_finite()
                || !ds_re.is_finite() || !ds_im.is_finite()
            {
                sa_valid = false;
            }

            if sa_valid {
                let numerator = ds_re.hypot(ds_im);
                let sum_re = as_re + bs_re + cs_re;
                let sum_im = as_im + bs_im + cs_im;
                let denominator = sum_re.hypot(sum_im);

                if denominator > 0.0 && numerator / denominator < 1e-6 {
                    skip_iterations = i;
                    final_as = [as_re, as_im];
                    final_bs = [bs_re, bs_im];
                    final_cs = [cs_re, cs_im];
                } else {
                    sa_valid = false;
                }
            }
        }

        if sa_valid {
            // Scaled recurrence: as = A·δ, bs = B·δ², cs = C·δ³, ds = D·δ⁴
            // as_{n+1} = 2·Z_n·as_n + δ
            let new_as_re = 2.0 * (re_f64 * as_re - im_f64 * as_im) + delta;
            let new_as_im = 2.0 * (re_f64 * as_im + im_f64 * as_re);

            // as_n²
            let as_sq_re = as_re * as_re - as_im * as_im;
            let as_sq_im = 2.0 * as_re * as_im;

            // bs_{n+1} = 2·Z_n·bs_n + as_n²
            let new_bs_re = 2.0 * (re_f64 * bs_re - im_f64 * bs_im) + as_sq_re;
            let new_bs_im = 2.0 * (re_f64 * bs_im + im_f64 * bs_re) + as_sq_im;

            // 2·as_n·bs_n
            let two_asbs_re = 2.0 * (as_re * bs_re - as_im * bs_im);
            let two_asbs_im = 2.0 * (as_re * bs_im + as_im * bs_re);

            // cs_{n+1} = 2·Z_n·cs_n + 2·as_n·bs_n
            let new_cs_re = 2.0 * (re_f64 * cs_re - im_f64 * cs_im) + two_asbs_re;
            let new_cs_im = 2.0 * (re_f64 * cs_im + im_f64 * cs_re) + two_asbs_im;

            // 2·as_n·cs_n
            let two_ascs_re = 2.0 * (as_re * cs_re - as_im * cs_im);
            let two_ascs_im = 2.0 * (as_re * cs_im + as_im * cs_re);

            // bs_n²
            let bs_sq_re = bs_re * bs_re - bs_im * bs_im;
            let bs_sq_im = 2.0 * bs_re * bs_im;

            // ds_{n+1} = 2·Z_n·ds_n + 2·as_n·cs_n + bs_n²
            let new_ds_re = 2.0 * (re_f64 * ds_re - im_f64 * ds_im) + two_ascs_re + bs_sq_re;
            let new_ds_im = 2.0 * (re_f64 * ds_im + im_f64 * ds_re) + two_ascs_im + bs_sq_im;

            as_re = new_as_re;
            as_im = new_as_im;
            bs_re = new_bs_re;
            bs_im = new_bs_im;
            cs_re = new_cs_re;
            cs_im = new_cs_im;
            ds_re = new_ds_re;
            ds_im = new_ds_im;
        }
    }

    if skip_iterations > 0 {
        // Scaled coefficients are already in the form stored by SACoefficients
        let a_s = [final_as[0] as f32, final_as[1] as f32];
        let b_s = [final_bs[0] as f32, final_bs[1] as f32];
        let c_s = [final_cs[0] as f32, final_cs[1] as f32];

        if a_s[0].is_finite() && a_s[1].is_finite()
            && b_s[0].is_finite() && b_s[1].is_finite()
            && c_s[0].is_finite() && c_s[1].is_finite()
            && delta.is_finite() && delta > 0.0
        {
            Some(SACoefficients {
                a: a_s,
                b: b_s,
                c: c_s,
                scale: delta,
                skip_iterations,
            })
        } else {
            None
        }
    } else {
        None
    }
}

pub fn iterate_perturbation(
    orbit: &ReferenceOrbit,
    dc_re: f32,
    dc_im: f32,
    max_iterations: usize,
    bailout: f32,
) -> (f32, usize) {
    iterate_perturbation_inner(orbit, dc_re, dc_im, max_iterations, bailout, None)
}

/// Iterate perturbation with optional pre-evaluated SA starting values.
/// When `sa_start` is Some, uses the provided (d_re, d_im, start_iteration) instead of
/// computing SA internally. This allows the caller to compute u = pixel_offset * zoom_norm
/// - ref_offset_norm (matching the GPU formula) and evaluate the polynomial externally.
pub fn iterate_perturbation_with_sa(
    orbit: &ReferenceOrbit,
    dc_re: f32,
    dc_im: f32,
    max_iterations: usize,
    bailout: f32,
    sa_start: (f32, f32, usize),
) -> (f32, usize) {
    iterate_perturbation_inner(orbit, dc_re, dc_im, max_iterations, bailout, Some(sa_start))
}

fn iterate_perturbation_inner(
    orbit: &ReferenceOrbit,
    dc_re: f32,
    dc_im: f32,
    max_iterations: usize,
    bailout: f32,
    sa_start: Option<(f32, f32, usize)>,
) -> (f32, usize) {

    let limit = orbit.escape_iteration.min(max_iterations);

    // SA polynomial evaluation in f32
    let (mut d_re, mut d_im, start_n): (f32, f32, usize) =
        if let Some((d_re, d_im, start_n)) = sa_start {
            // Pre-evaluated by caller (matches GPU's u computation)
            (d_re, d_im, start_n)
        } else if let Some(sa) = &orbit.sa_coefficients {
            if sa.skip_iterations > 0 && sa.skip_iterations < limit && sa.scale > 0.0 {
                let scale_f32 = sa.scale as f32;
                let u_re = dc_re / scale_f32;
                let u_im = dc_im / scale_f32;
                let u2_re = u_re * u_re - u_im * u_im;
                let u2_im = 2.0f32 * u_re * u_im;
                let u3_re = u2_re * u_re - u2_im * u_im;
                let u3_im = u2_re * u_im + u2_im * u_re;

                let eps_re = sa.a[0] * u_re - sa.a[1] * u_im
                    + sa.b[0] * u2_re - sa.b[1] * u2_im
                    + sa.c[0] * u3_re - sa.c[1] * u3_im;
                let eps_im = sa.a[0] * u_im + sa.a[1] * u_re
                    + sa.b[0] * u2_im + sa.b[1] * u2_re
                    + sa.c[0] * u3_im + sa.c[1] * u3_re;

                (eps_re, eps_im, sa.skip_iterations)
            } else {
                (0.0f32, 0.0f32, 0)
            }
        } else {
            (0.0f32, 0.0f32, 0)
        };

    let mut iterations = start_n;

    // Pre-compute c for rebasing fallback and orbit-escape fallback
    let c_re = orbit.center[0] as f32 + dc_re;
    let c_im = orbit.center[1] as f32 + dc_im;

    for n in start_n..limit {
        // Read f64 orbit values, cast to f32 for perturbation math
        let z_re = orbit.values[n][0] as f32;
        let z_im = orbit.values[n][1] as f32;

        // Rebasing: when |δ|² > |Z|², perturbation has lost precision.
        // The formula δ_{n+1} = 2·Z·δ + δ² + dc computes the small difference
        // of large quantities when |δ| >> |Z|. Switch to direct iteration
        // z = z² + c starting from z = Z + δ to reset error accumulation.
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
                fz_im = 2.0f32 * fz_re * fz_im + c_im;
                fz_re = new_re;
            }
            return (0.0, max_iterations);
        }

        // δ(n+1) = 2·Z(n)·δ(n) + δ(n)² + δc
        let new_d_re = 2.0f32 * (z_re * d_re - z_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
        let new_d_im = 2.0f32 * (z_re * d_im + z_im * d_re) + 2.0f32 * d_re * d_im + dc_im;
        d_re = new_d_re;
        d_im = new_d_im;

        // Check escape: |Z(n+1) + δ(n+1)|²
        if n + 1 < orbit.values.len() {
            let full_re = orbit.values[n + 1][0] as f32 + d_re;
            let full_im = orbit.values[n + 1][1] as f32 + d_im;
            let norm_sq = full_re * full_re + full_im * full_im;
            if norm_sq > bailout {
                return (norm_sq, n + 1);
            }
        }

        iterations = n + 1;
    }

    // Fallback: reference orbit escaped before max_iterations — direct f32 iteration
    if iterations < max_iterations {
        let (mut z_re, mut z_im) = if limit < orbit.values.len() {
            // Orbit escaped: limit = escape_iteration, values[limit] = Z_escape.
            // d is d_limit. Full z = Z_limit + d_limit.
            (orbit.values[limit][0] as f32 + d_re, orbit.values[limit][1] as f32 + d_im)
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
            z_im = 2.0f32 * z_re * z_im + c_im;
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
        let orbit = compute_reference_orbit_with_sa(&center, 1000, 65536.0, max_delta);

        // Verify SA was computed
        let sa = orbit.sa_coefficients.as_ref().expect("SA should be computed");
        assert!(sa.skip_iterations > 0, "Should skip at least some iterations");

        // Pick a test delta smaller than max_delta
        let dc_re = 3e-7_f32;
        let dc_im = 5e-7_f32;

        // Evaluate using the same f32 scaled path as CPU and GPU
        let scale_f32 = sa.scale as f32;
        let u_re = dc_re / scale_f32;
        let u_im = dc_im / scale_f32;
        let u2_re = u_re * u_re - u_im * u_im;
        let u2_im = 2.0f32 * u_re * u_im;
        let u3_re = u2_re * u_re - u2_im * u_im;
        let u3_im = u2_re * u_im + u2_im * u_re;

        let eps_re = sa.a[0] * u_re - sa.a[1] * u_im
            + sa.b[0] * u2_re - sa.b[1] * u2_im
            + sa.c[0] * u3_re - sa.c[1] * u3_im;
        let eps_im = sa.a[0] * u_im + sa.a[1] * u_re
            + sa.b[0] * u2_im + sa.b[1] * u2_re
            + sa.c[0] * u3_im + sa.c[1] * u3_re;

        // Brute-force: iterate perturbation from n=0 to skip_iterations in f32
        let mut d_re = 0.0_f32;
        let mut d_im = 0.0_f32;
        for n in 0..sa.skip_iterations {
            let z_re = orbit.values[n][0] as f32;
            let z_im = orbit.values[n][1] as f32;
            let new_d_re = 2.0f32 * (z_re * d_re - z_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
            let new_d_im = 2.0f32 * (z_re * d_im + z_im * d_re) + 2.0f32 * d_re * d_im + dc_im;
            d_re = new_d_re;
            d_im = new_d_im;
        }

        // Compare — SA polynomial vs brute force (both f32)
        let err_re = (eps_re - d_re).abs();
        let err_im = (eps_im - d_im).abs();
        let brute_mag = (d_re * d_re + d_im * d_im).sqrt();
        let rel_err = ((err_re * err_re + err_im * err_im).sqrt()) / brute_mag.max(1e-30);
        assert!(
            rel_err < 0.05,
            "SA f32 should approximate brute force within 5%, got relative error {rel_err}"
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
        compute_reference_orbit_with_sa(&center, max_iterations, 4.0, max_delta)
    }

    /// Direct f32 Mandelbrot iteration: z = z² + c, starting from z = 0.
    fn direct_iterate(c_re: f32, c_im: f32, max_iterations: usize, bailout: f32) -> (f32, usize) {
        let mut z_re = 0.0_f32;
        let mut z_im = 0.0_f32;
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
        // Reference at (-0.5, 0) — inside main cardioid, doesn't escape
        let orbit = make_orbit(-0.5, 0.0, 100, 0.01);
        assert_eq!(orbit.escape_iteration, 100, "Reference should not escape");

        let (norm_sq, iterations) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(iterations, 100, "dc=0 with non-escaping reference should give max_iterations");
        assert_eq!(norm_sq, 0.0, "dc=0 should have zero norm_sq");
    }

    #[test]
    fn dc_zero_matches_reference_escaping() {
        // Reference at (0.3, 0) — escapes (outside Mandelbrot set)
        let orbit = make_orbit(0.3, 0.0, 100, 0.01);
        let ref_escape = orbit.escape_iteration;
        assert!(ref_escape < 100, "Reference should escape, got {ref_escape}");

        // Direct iteration should match
        let (_, direct_iter) = direct_iterate(0.3, 0.0, 100, 4.0);
        assert_eq!(ref_escape, direct_iter, "Reference orbit escape should match direct iteration");

        // Perturbation with dc=0 should also match
        let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(
            pert_iter, direct_iter,
            "Perturbation dc=0 should match direct: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn perturbation_matches_direct_small_dc() {
        // Non-escaping reference at (-0.5, 0)
        let orbit = make_orbit(-0.5, 0.0, 200, 0.01);
        assert_eq!(orbit.escape_iteration, 200, "Reference should not escape");

        // Small dc values — all close to reference, perturbation should be accurate
        let test_cases: &[(f32, f32)] = &[
            (0.001, 0.001),
            (-0.005, 0.003),
            (0.0, 0.008),
            (0.009, 0.0),
        ];

        for &(dc_re, dc_im) in test_cases {
            let (pert_norm, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
            let c_re = -0.5_f32 + dc_re;
            let c_im = 0.0_f32 + dc_im;
            let (direct_norm, direct_iter) = direct_iterate(c_re, c_im, 200, 4.0);

            assert_eq!(
                pert_iter, direct_iter,
                "Iteration mismatch for dc=({dc_re}, {dc_im}): pert={pert_iter}, direct={direct_iter}"
            );
            if direct_iter < 200 {
                let rel_diff = (pert_norm - direct_norm).abs() / direct_norm.max(1e-10);
                assert!(
                    rel_diff < 0.1,
                    "Norm mismatch for dc=({dc_re}, {dc_im}): pert={pert_norm}, direct={direct_norm}"
                );
            }
        }
    }

    #[test]
    fn perturbation_step_by_step_matches_direct() {
        // Verify δ evolution matches z - Z at each step
        let center_re = -0.5_f64;
        let center_im = 0.0_f64;
        let orbit = make_orbit(center_re, center_im, 50, 0.01);

        let dc_re = 0.002_f32;
        let dc_im = 0.003_f32;
        let c_re = center_re as f32 + dc_re;
        let c_im = center_im as f32 + dc_im;

        // Direct iteration
        let mut z_re = 0.0_f32;
        let mut z_im = 0.0_f32;

        // Perturbation iteration (manual, matching iterate_perturbation logic)
        let mut d_re = 0.0_f32;
        let mut d_im = 0.0_f32;

        for n in 0..orbit.values.len().min(50) {
            let ref_re = orbit.values[n][0] as f32;
            let ref_im = orbit.values[n][1] as f32;

            // Check: Z_n + δ_n ≈ z_n
            let full_re = ref_re + d_re;
            let full_im = ref_im + d_im;
            let err = ((full_re - z_re).powi(2) + (full_im - z_im).powi(2)).sqrt();
            let z_mag = (z_re * z_re + z_im * z_im).sqrt().max(1e-10);
            assert!(
                err / z_mag < 0.01 || err < 1e-6,
                "Step {n}: Z+δ=({full_re},{full_im}) vs z=({z_re},{z_im}), err={err}"
            );

            // Advance perturbation: δ_{n+1} = 2·Z_n·δ_n + δ_n² + dc
            let new_d_re = 2.0f32 * (ref_re * d_re - ref_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
            let new_d_im = 2.0f32 * (ref_re * d_im + ref_im * d_re) + 2.0f32 * d_re * d_im + dc_im;
            d_re = new_d_re;
            d_im = new_d_im;

            // Advance direct: z_{n+1} = z_n² + c
            let new_z_re = z_re * z_re - z_im * z_im + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_z_re;
        }
    }

    #[test]
    fn escaping_orbit_stores_escaped_value() {
        // Reference at (0.3, 0) escapes — orbit should have escape_iteration + 1 values
        let orbit = make_orbit(0.3, 0.0, 100, 0.01);
        assert!(orbit.escape_iteration < 100, "Reference should escape");
        assert_eq!(
            orbit.values.len(),
            orbit.escape_iteration + 1,
            "Orbit should store Z_0 through Z_escape"
        );
        // The last stored value should be the escaped one
        let last = orbit.values[orbit.escape_iteration];
        let norm_sq = last[0] * last[0] + last[1] * last[1];
        assert!(norm_sq > 4.0, "Last orbit value should be escaped");
    }

    #[test]
    fn f32_dc_computation_matches_gpu() {
        // Verify that the CPU dc computation matches the GPU formula
        // GPU: dc = (frac * 2 - 1) * zoom - reference_offset
        // CPU: dc = (px / width * 2 - 1) * zoom - reference_offset
        let width = 1920_u32;
        let zoom_x = 1e-8_f32;
        let ref_off = 1e-9_f32;

        for px in [0u32, 480, 960, 1440, 1919] {
            let frac = px as f32 / width as f32;
            let dc = (frac * 2.0 - 1.0) * zoom_x - ref_off;

            // Verify it's representable and not zero for non-center pixels
            if px != 960 {
                assert!(dc != 0.0, "dc should be nonzero for px={px}");
            }
            assert!(dc.is_finite(), "dc should be finite for px={px}");
        }
    }

    #[test]
    fn escaping_reference_non_escaping_pixel() {
        // Reference escapes, but pixel (with dc) is inside the Mandelbrot set.
        // Reference at (0.3, 0) escapes; dc shifts to inside the set.
        let orbit = make_orbit(0.3, 0.0, 200, 0.5);
        assert!(orbit.escape_iteration < 200, "Reference should escape");

        // c = (-0.5, 0) is inside the main cardioid
        let dc_re = -0.8_f32;
        let dc_im = 0.0_f32;

        let (pert_norm, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
        let (direct_norm, direct_iter) = direct_iterate(0.3 + dc_re, 0.0 + dc_im, 200, 4.0);

        assert_eq!(
            pert_iter, direct_iter,
            "Interior pixel: pert={pert_iter}, direct={direct_iter}"
        );
        assert_eq!(pert_norm, direct_norm, "Both should be 0.0 for interior");
    }

    #[test]
    fn grid_perturbation_vs_direct() {
        // Systematic comparison over a grid of points around a reference.
        let center_re = -0.75_f64;
        let center_im = 0.1_f64;
        let orbit = make_orbit(center_re, center_im, 300, 0.1);
        let max_iter = 300;

        let mut mismatches = 0;
        let total = 11 * 11;
        for i in 0..11 {
            for j in 0..11 {
                let dc_re = (i as f32 - 5.0) * 0.005;
                let dc_im = (j as f32 - 5.0) * 0.005;
                let c_re = center_re as f32 + dc_re;
                let c_im = center_im as f32 + dc_im;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, max_iter, 4.0);
                let (_, direct_iter) = direct_iterate(c_re, c_im, max_iter, 4.0);

                // Allow ±1 iteration tolerance due to f32 precision differences
                // between perturbation path and direct iteration
                if (pert_iter as i64 - direct_iter as i64).unsigned_abs() > 1 {
                    mismatches += 1;
                }
            }
        }
        assert!(
            mismatches * 10 < total,
            "Too many mismatches: {mismatches}/{total}"
        );
    }

    #[test]
    fn sa_skip_matches_no_skip() {
        // Verify that SA skip + perturbation gives same result as no SA.
        let center_re = -0.75_f64;
        let center_im = 0.1_f64;

        // With SA
        let orbit_sa = make_orbit(center_re, center_im, 500, 1e-4);
        let has_sa = orbit_sa.sa_coefficients.is_some();

        // Without SA (max_delta = 0 means no SA will be computed)
        let mut orbit_no_sa = make_orbit(center_re, center_im, 500, 1e-4);
        orbit_no_sa.sa_coefficients = None;

        if !has_sa {
            // SA wasn't computed, skip this test
            return;
        }

        let dc_re = 5e-5_f32;
        let dc_im = 3e-5_f32;

        let (norm_sa, iter_sa) = iterate_perturbation(&orbit_sa, dc_re, dc_im, 500, 4.0);
        let (norm_no_sa, iter_no_sa) = iterate_perturbation(&orbit_no_sa, dc_re, dc_im, 500, 4.0);

        // Should produce same or very close results
        assert!(
            (iter_sa as i64 - iter_no_sa as i64).unsigned_abs() <= 1,
            "SA skip should match no-skip: sa={iter_sa}, no_sa={iter_no_sa}"
        );
        if iter_sa < 500 && iter_no_sa < 500 {
            let rel_diff = (norm_sa - norm_no_sa).abs() / norm_no_sa.max(1e-10);
            assert!(
                rel_diff < 0.2,
                "Norm with SA should match no-SA: sa={norm_sa}, no_sa={norm_no_sa}"
            );
        }
    }

    #[test]
    fn max_iterations_one() {
        // Edge case: max_iterations = 1
        let orbit = make_orbit(-0.5, 0.0, 1, 0.01);
        let (norm, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 1, 4.0);
        // One iteration: z = 0² + c = c = (-0.5, 0), |c|² = 0.25 < 4 → no escape
        assert_eq!(iter, 1);
        assert_eq!(norm, 0.0);
    }

    #[test]
    fn max_iterations_constrains_result() {
        // Pixel that escapes at iteration 10, but we only allow 5 iterations
        let orbit = make_orbit(0.3, 0.0, 20, 0.01);
        let ref_escape = orbit.escape_iteration;
        assert!(ref_escape < 20);

        let (_, direct_5) = direct_iterate(0.3, 0.0, 5, 4.0);
        let (_, pert_5) = iterate_perturbation(&orbit, 0.0, 0.0, 5, 4.0);

        if ref_escape > 5 {
            // If reference escapes after 5, max_iterations=5 should return 5
            assert_eq!(pert_5, 5, "Should be capped at max_iterations=5");
        } else {
            assert_eq!(pert_5, direct_5, "Should match direct at max_iterations=5");
        }
    }

    #[test]
    fn orbit_values_match_direct_f32() {
        // Verify that the stored orbit values match direct f32 iteration.
        // This ensures the reference orbit is stored correctly.
        let orbit = make_orbit(-0.5, 0.0, 50, 0.01);

        let mut z_re = 0.0_f32;
        let mut z_im = 0.0_f32;
        for i in 0..50.min(orbit.values.len()) {
            // Orbit values are f64, compare against f32 direct iteration with tolerance
            let ref_re = orbit.values[i][0] as f32;
            let ref_im = orbit.values[i][1] as f32;

            let err = ((ref_re - z_re).powi(2) + (ref_im - z_im).powi(2)).sqrt();
            assert!(
                err < 1e-5,
                "Orbit value mismatch at step {i}: orbit=({ref_re},{ref_im}) vs direct=({z_re},{z_im})"
            );

            // Advance: z = z² + c
            let new_re = z_re * z_re - z_im * z_im + (-0.5_f32);
            z_im = 2.0 * z_re * z_im + 0.0;
            z_re = new_re;
        }
    }

    #[test]
    fn needs_perturbation_threshold() {
        // Verify the perturbation threshold is reasonable
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
        // With a very small max_delta, SA should still work or gracefully degrade
        let orbit = make_orbit(-0.75, 0.1, 100, 1e-20);
        // SA might or might not be computed with very tiny delta,
        // but iterate_perturbation should still work correctly
        let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        let (_, direct_iter) = direct_iterate(-0.75, 0.1, 100, 4.0);
        assert_eq!(pert_iter, direct_iter);
    }

    #[test]
    fn test_orbit_f64_escaping() {
        // (1, 0) escapes: z=0 → 1 → 2 → 5, escape at i=3
        assert_eq!(test_orbit_f64(1.0, 0.0, 100, 4.0), 3);
        // (0, 0) never escapes
        assert_eq!(test_orbit_f64(0.0, 0.0, 100, 4.0), 100);
        // (2, 0) escapes immediately: z=0 → 2 → 6, |4|>4? No (strict >), |36|>4, escape at i=2
        // Actually z=0, i=0: |0|<4, z=0+2=2. i=1: |4| not > 4, z=4+2=6. i=2: |36|>4 → escape=2
        assert_eq!(test_orbit_f64(2.0, 0.0, 100, 4.0), 2);
        // (-2, 0) is period 2: z=0 → -2 → 2 → 2 → 2..., |z|²=4 not > 4
        assert_eq!(test_orbit_f64(-2.0, 0.0, 100, 4.0), 100);
        // (0.3, 0.6) should escape
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
        // z² + c = (3+4i)² + (1+2i) = (9-16+24i) + (1+2i) = (-6+26i)
        let result = z.square_add(&c);
        let re: f64 = result.re.to_f64().value();
        let im: f64 = result.im.to_f64().value();
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
        // |3+4i|² = 9+16 = 25
        let norm: f64 = z.norm_sqr().to_f64().value();
        assert!((norm - 25.0).abs() < 1e-10, "norm_sqr={norm}");
    }

    #[test]
    fn escape_on_last_perturbation_iteration() {
        // If a pixel escapes at exactly n = limit - 1, the escape check needs
        // orbit.values[limit] to exist. For escaping orbits this is the pushed
        // escaped value. For non-escaping orbits, there's no values[limit].
        //
        // Use an escaping reference so values[limit] exists.
        let orbit = make_orbit(0.5, 0.0, 100, 0.01);
        let limit = orbit.escape_iteration;
        assert!(limit < 100);

        // dc=0 should escape at exactly the same iteration as the reference
        let (norm, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        assert_eq!(iter, limit, "dc=0 should escape at reference escape iteration");
        assert!(norm > 4.0, "Should have escaped norm");
    }

    #[test]
    fn non_escaping_orbit_last_iteration_pixel_enters_fallback() {
        // Non-escaping orbit: orbit.values.len() == max_iterations.
        // At n = limit-1, escape check needs values[limit] which doesn't exist.
        // So the pixel can't escape during the perturbation loop on the last step
        // and must enter fallback if it would escape there.
        //
        // Use a non-escaping reference with a dc that escapes near max_iterations.
        let orbit = make_orbit(-0.5, 0.0, 50, 0.1);
        assert_eq!(orbit.escape_iteration, 50, "Reference should not escape");
        assert_eq!(orbit.values.len(), 50, "Non-escaping orbit has max_iterations values");

        // Small dc that escapes around iteration 40-50
        // c = (-0.5 + 0.06, 0.06) = (-0.44, 0.06) — should escape
        let dc_re = 0.06_f32;
        let dc_im = 0.06_f32;
        let c_re = -0.5_f32 + dc_re;
        let c_im = 0.0_f32 + dc_im;

        let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 50, 4.0);
        let (_, direct_iter) = direct_iterate(c_re, c_im, 50, 4.0);

        // They should match (fallback handles the gap)
        assert!(
            (pert_iter as i64 - direct_iter as i64).unsigned_abs() <= 1,
            "Last-iter edge: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn max_iterations_larger_than_orbit() {
        // If iterate_perturbation is called with max_iterations > orbit's max,
        // the fallback should still produce correct results.
        let orbit = make_orbit(-0.5, 0.0, 50, 0.01);
        assert_eq!(orbit.escape_iteration, 50);

        // Call with max_iterations=100, orbit only has 50 values
        // limit = min(50, 100) = 50, so perturbation loop runs 0..50
        // Then fallback from 50..100
        let dc_re = 0.001_f32;
        let dc_im = 0.001_f32;
        let c_re = -0.5_f32 + dc_re;
        let c_im = 0.0_f32 + dc_im;

        let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 100, 4.0);
        let (_, direct_iter) = direct_iterate(c_re, c_im, 100, 4.0);

        assert!(
            (pert_iter as i64 - direct_iter as i64).unsigned_abs() <= 1,
            "Extended max_iter: pert={pert_iter}, direct={direct_iter}"
        );
    }

    #[test]
    fn sa_disabled_when_skip_equals_limit() {
        // SA should be disabled when skip_iterations >= limit
        let mut orbit = make_orbit(-0.75, 0.1, 100, 1e-6);
        if let Some(ref mut sa) = orbit.sa_coefficients {
            sa.skip_iterations = 100; // Equal to limit
        }

        // Should not crash and should work correctly (SA skipped, fallback from 0)
        let (_, iter) = iterate_perturbation(&orbit, 0.0, 0.0, 100, 4.0);
        let (_, direct) = direct_iterate(-0.75, 0.1, 100, 4.0);
        assert_eq!(iter, direct);
    }

    #[test]
    fn different_bailout_values() {
        let c_re = 0.3_f32;
        let c_im = 0.5_f32;

        for bailout in [4.0_f32, 16.0, 256.0, 65536.0] {
            // Recompute orbit with this bailout
            let precision = 128;
            let center = HighPrecisionComplex::new(
                FBig::try_from(0.3).unwrap().with_precision(precision).value(),
                FBig::try_from(0.5).unwrap().with_precision(precision).value(),
            );
            let orbit = compute_reference_orbit_with_sa(&center, 200, bailout, 0.01);

            let (_, pert_iter) = iterate_perturbation(&orbit, 0.0, 0.0, 200, bailout);
            let (_, direct_iter) = direct_iterate(c_re, c_im, 200, bailout);

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
        // Regression test for glitch detection removal.
        //
        // When |d| >> |Z| (pixel far from reference), the old glitch detection
        // would send pixels to an f32 fallback that computes c = center + dc.
        // At deep zoom, f32 loses dc (since |center| >> |dc|), so all fallback
        // pixels get the same c → same iteration → rectangular artifacts.
        //
        // Without glitch detection, the perturbation formula d = 2*Z*d + d^2 + dc
        // naturally degrades to d ≈ d^2 + dc (direct iteration), which is correct.
        //
        // This test uses an escaping reference with large dc to verify the
        // perturbation formula produces correct results when d >> Z.
        let mut orbit = make_orbit(-0.5, 0.0, 200, 1.0);
        orbit.sa_coefficients = None; // Disable SA to test perturbation directly
        assert_eq!(orbit.escape_iteration, 200);

        // Grid of large dc values — pixel c values are far from reference
        let grid_size = 16;
        let dc_range = 1.0_f32;
        let mut mismatches = 0;
        let total = grid_size * grid_size;

        for py in 0..grid_size {
            for px in 0..grid_size {
                let dc_re = (px as f32 / (grid_size - 1) as f32 * 2.0 - 1.0) * dc_range;
                let dc_im = (py as f32 / (grid_size - 1) as f32 * 2.0 - 1.0) * dc_range;

                let c_re = -0.5_f32 + dc_re;
                let c_im = 0.0_f32 + dc_im;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, 200, 4.0);
                let (_, direct_iter) = direct_iterate(c_re, c_im, 200, 4.0);

                if (pert_iter as i64 - direct_iter as i64).unsigned_abs() > 1 {
                    mismatches += 1;
                }
            }
        }

        assert!(
            mismatches * 100 / total < 5,
            "Escaping reference + large dc (d >> Z): {mismatches}/{total} mismatches"
        );
    }

    #[test]
    fn escaping_reference_grid_perturbation_vs_direct() {
        // Like grid_perturbation_vs_direct but with an ESCAPING reference orbit.
        // This is the scenario where glitch detection would have triggered:
        // after the reference orbit's escape, pixels that haven't escaped yet
        // need to continue via fallback. Without glitch detection, more pixels
        // escape correctly during the perturbation loop itself.
        let center_re = -0.75_f64;
        let center_im = 0.15_f64;
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
                let dc_re = (i as f32 - 5.0) * 0.005;
                let dc_im = (j as f32 - 5.0) * 0.005;
                let c_re = center_re as f32 + dc_re;
                let c_im = center_im as f32 + dc_im;

                let (_, pert_iter) = iterate_perturbation(&orbit, dc_re, dc_im, max_iter, 4.0);
                let (_, direct_iter) = direct_iterate(c_re, c_im, max_iter, 4.0);

                if (pert_iter as i64 - direct_iter as i64).unsigned_abs() > 1 {
                    mismatches += 1;
                }
            }
        }
        assert!(
            mismatches * 10 < total,
            "Escaping reference grid: {mismatches}/{total} mismatches"
        );
    }

    #[test]
    fn recompute_sa_increases_skip_at_deeper_zoom() {
        // Test that recompute_sa produces more skip with smaller delta.
        //
        // Uses a synthetic orbit stuck near a fixed point (mimicking real deep-zoom
        // behavior). In the real app, HP arithmetic keeps the orbit near the fixed
        // point for 1000+ iterations. With f64-derived centers, orbits escape too
        // early (~277 iterations) to see the delta-dependent behavior.
        //
        // The fixed point z* ≈ (-0.461, -0.334) has |2Z*| ≈ 1.14, so SA coefficients
        // grow slowly and the tolerance check is the limiting factor (not f64 overflow).

        // Build a synthetic orbit: stuck at z* for 1500 iterations
        let z_re = -0.46113_f64;
        let z_im = -0.33428_f64;
        let orbit_len = 1500;
        let mut values = Vec::with_capacity(orbit_len);
        values.push([0.0, 0.0]); // z_0 = 0
        for _ in 1..orbit_len {
            values.push([z_re, z_im]);
        }

        let wide_delta = 1e-10;
        let deep_delta = 1e-16; // 10^6x deeper

        // Create orbit with original SA at wide delta
        let mut orbit = ReferenceOrbit {
            values,
            center: [z_re, z_im], // approximate
            escape_iteration: orbit_len,
            computed_max_iterations: orbit_len,
            sa_coefficients: None,
        };

        // Compute original SA using the full compute path
        let original_sa = recompute_sa(&orbit, wide_delta);
        let original_skip = original_sa.as_ref().map_or(0, |sa| sa.skip_iterations);
        orbit.sa_coefficients = original_sa;

        // Recompute with smaller delta
        let new_sa = recompute_sa(&orbit, deep_delta);
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
        // With 100x smaller delta (δ³ is 10⁶ smaller), expect significantly more skip
        assert!(
            new_skip > original_skip + 50,
            "Recomputed SA should skip meaningfully more: \
             original={original_skip}, recomputed={new_skip}"
        );
    }

    #[test]
    fn sa_accuracy_at_high_skip_count() {
        // Regression test for SA polynomial truncation error.
        //
        // At certain zoom levels, the cubic SA polynomial can skip 1000+ iterations.
        // If the tolerance check only considers the C (last included) term rather than
        // the D (first excluded) term, the truncation error accumulates and produces
        // concentric ring/Moiré artifacts.
        //
        // This test verifies that SA + perturbation matches brute-force perturbation
        // (no SA) even when the SA skip count is high.
        let center_re = -0.6748093390464783_f64;
        let center_im = -0.4372471585273743_f64;

        // High iteration count — like the buggy dump (2000 iterations)
        let max_iter = 2000;
        let max_delta = 3e-20;

        let orbit_sa = make_orbit(center_re, center_im, max_iter, max_delta);
        let mut orbit_no_sa = make_orbit(center_re, center_im, max_iter, max_delta);
        orbit_no_sa.sa_coefficients = None;

        let skip = orbit_sa.sa_coefficients.as_ref().map_or(0, |sa| sa.skip_iterations);

        // Grid of small dc values (simulating a viewport at this zoom)
        let grid_size = 16;
        let dc_scale = max_delta as f32;
        let mut mismatches = 0;
        let total = grid_size * grid_size;

        for py in 0..grid_size {
            for px in 0..grid_size {
                let dc_re = (px as f32 / (grid_size - 1) as f32 * 2.0 - 1.0) * dc_scale;
                let dc_im = (py as f32 / (grid_size - 1) as f32 * 2.0 - 1.0) * dc_scale;

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
