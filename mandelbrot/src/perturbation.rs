use dashu::float::FBig;

#[derive(Clone, Debug)]
pub struct HighPrecisionComplex {
    pub re: FBig,
    pub im: FBig,
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

#[derive(Clone, Debug)]
pub struct ReferenceOrbit {
    pub values_f64: Vec<[f64; 2]>,
    pub values_f32: Vec<[f32; 2]>,
    pub escape_iteration: usize,
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

pub fn needs_perturbation(zoom_x: f32, width: usize) -> bool {
    let pixel_spacing = (2.0 * zoom_x) / width as f32;
    pixel_spacing < 1e-5
}

pub fn required_precision_bits(zoom_x: f32) -> usize {
    let log2_zoom = -(zoom_x.ln() / std::f32::consts::LN_2);
    let bits = (log2_zoom as usize).max(0) + 64;
    bits.max(64)
}

pub fn compute_reference_orbit(center: &HighPrecisionComplex, max_iterations: usize, bailout: f32) -> ReferenceOrbit {
    let precision = center.re.precision();
    let bailout_big: FBig = FBig::try_from(bailout as f64)
        .unwrap()
        .with_precision(precision)
        .value();
    let mut z = HighPrecisionComplex::new(
        FBig::ZERO.with_precision(precision).value(),
        FBig::ZERO.with_precision(precision).value(),
    );

    let mut values_f64 = Vec::with_capacity(max_iterations);
    let mut values_f32 = Vec::with_capacity(max_iterations);
    let mut escape_iteration = max_iterations;

    for i in 0..max_iterations {
        let re_f64: f64 = z.re.to_f64().value();
        let im_f64: f64 = z.im.to_f64().value();
        values_f64.push([re_f64, im_f64]);
        values_f32.push([re_f64 as f32, im_f64 as f32]);

        z = z.square_add(center);

        if z.norm_sqr() > bailout_big {
            escape_iteration = i + 1;
            break;
        }
    }

    ReferenceOrbit {
        values_f64,
        values_f32,
        escape_iteration,
    }
}

pub fn iterate_perturbation(
    orbit: &ReferenceOrbit,
    center_re: f64,
    center_im: f64,
    dc_re: f64,
    dc_im: f64,
    max_iterations: usize,
    bailout: f64,
) -> (f64, usize) {
    let c_re = center_re + dc_re;
    let c_im = center_im + dc_im;

    let mut d_re = 0.0_f64;
    let mut d_im = 0.0_f64;

    let limit = orbit.escape_iteration.min(max_iterations);

    for n in 0..limit {
        let z_re = orbit.values_f64[n][0];
        let z_im = orbit.values_f64[n][1];

        // Glitch detection: |δ|² > |Z|² × 1e-3
        let d_norm_sq = d_re * d_re + d_im * d_im;
        let z_norm_sq = z_re * z_re + z_im * z_im;
        if n > 0 && d_norm_sq > z_norm_sq * 1e-3 {
            return iterate_fallback_f64(z_re + d_re, z_im + d_im, c_re, c_im, n, max_iterations, bailout);
        }

        // δ(n+1) = 2·Z(n)·δ(n) + δ(n)² + δc
        let new_d_re = 2.0 * (z_re * d_re - z_im * d_im) + d_re * d_re - d_im * d_im + dc_re;
        let new_d_im = 2.0 * (z_re * d_im + z_im * d_re) + 2.0 * d_re * d_im + dc_im;
        d_re = new_d_re;
        d_im = new_d_im;

        // Check escape: |Z(n+1) + δ(n+1)|²
        if n + 1 < orbit.values_f64.len() {
            let full_re = orbit.values_f64[n + 1][0] + d_re;
            let full_im = orbit.values_f64[n + 1][1] + d_im;
            let full_norm_sq = full_re * full_re + full_im * full_im;
            if full_norm_sq > bailout {
                return (full_norm_sq, n + 1);
            }
        }
    }

    // Reference escaped before pixel — fall back to direct f64
    if orbit.escape_iteration < max_iterations {
        let z_re = orbit.values_f64[limit - 1][0] + d_re;
        let z_im = orbit.values_f64[limit - 1][1] + d_im;
        return iterate_fallback_f64(z_re, z_im, c_re, c_im, limit, max_iterations, bailout);
    }

    (0.0, max_iterations)
}

fn iterate_fallback_f64(
    start_re: f64,
    start_im: f64,
    c_re: f64,
    c_im: f64,
    start_iter: usize,
    max_iterations: usize,
    bailout: f64,
) -> (f64, usize) {
    let mut z_re = start_re;
    let mut z_im = start_im;

    for i in start_iter..max_iterations {
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

    (z_re * z_re + z_im * z_im, max_iterations)
}
