@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    var positions = array(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
    return vec4f(positions[vertex_index], 0.0, 1.0);
}

struct PerturbationParams {
    viewport: vec2f,
    zoom: vec2f,
    reference_offset: vec2f,
    center: vec2f,
    bailout: f32,
    max_iterations: u32,
    orbit_length: u32,
    exponent: f32,
    skip_iterations: u32,
    num_sa_terms: u32,
    sa_zoom_norm: vec2f,
    sa_ref_offset_norm: vec2f,
}

@group(0) @binding(0)
var<uniform> params: PerturbationParams;

@group(0) @binding(1)
var color_texture: texture_2d<f32>;

@group(0) @binding(2)
var color_sampler: sampler;

@group(0) @binding(3)
var<storage, read> reference_orbit: array<vec2f>;

@group(0) @binding(4)
var<storage, read> sa_coefficients: array<vec2f>;

const LN2: f32 = 0.6931471805599453;

fn color_pixel(norm_sq: f32, iterations: u32) -> vec4f {
    let zn = log(norm_sq) / 2.0;
    let nu = log(zn / LN2) / LN2;
    let smoothed = max(f32(iterations) + 1.0 - nu, 0.0);
    let s = pow(smoothed / f32(params.max_iterations), params.exponent);
    return textureSampleLevel(color_texture, color_sampler, vec2f(s, 0.5), 0.0);
}

// Complex multiply: (a.x + i*a.y) * (b.x + i*b.y)
fn cmul(a: vec2f, b: vec2f) -> vec2f {
    return vec2f(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

@fragment
fn fs_main(@builtin(position) input: vec4f) -> @location(0) vec4f {
    let frac = input.xy / params.viewport;

    let pixel_offset = frac * 2.0 - 1.0;

    // dc = pixel offset from reference point in fractal coordinates
    let dc_re = pixel_offset.x * params.zoom.x - params.reference_offset.x;
    let dc_im = pixel_offset.y * params.zoom.y - params.reference_offset.y;

    // Cardioid and period-2 bulb check (O(1) interior detection).
    // Compute actual pixel c from screen position, NOT from dc
    // (dc has reference_offset baked in, giving wrong c).
    let pixel_c_re = params.center.x + pixel_offset.x * params.zoom.x;
    let pixel_c_im = params.center.y + pixel_offset.y * params.zoom.y;
    let pixel_c_im2 = pixel_c_im * pixel_c_im;
    var q = pixel_c_re - 0.25;
    q = q * q + pixel_c_im2;
    let p2 = pixel_c_re + 1.0;
    if q * (q + (pixel_c_re - 0.25)) < 0.25 * pixel_c_im2 || p2 * p2 + pixel_c_im2 < 0.0625 {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }

    let limit = params.orbit_length;
    let buffer_len = arrayLength(&reference_orbit);

    var d_re = 0.0;
    var d_im = 0.0;
    var start_n = 0u;
    if params.skip_iterations > 0u && params.skip_iterations < limit && params.num_sa_terms > 0u {
        // Compute u = dc/scale using precomputed zoom_norm and ref_offset_norm.
        // These are computed in f64 on CPU to avoid FTZ and precision loss.
        let u = vec2f(
            pixel_offset.x * params.sa_zoom_norm.x - params.sa_ref_offset_norm.x,
            pixel_offset.y * params.sa_zoom_norm.y - params.sa_ref_offset_norm.y
        );

        // Horner's method: eps = u*(c[0] + u*(c[1] + u*(c[2] + ... + u*c[N-1])))
        var t = sa_coefficients[params.num_sa_terms - 1u];
        for (var k = i32(params.num_sa_terms) - 2; k >= 0; k--) {
            t = sa_coefficients[u32(k)] + cmul(u, t);
        }
        let eps = cmul(u, t);

        d_re = eps.x;
        d_im = eps.y;
        start_n = params.skip_iterations;
    }

    var iterations: u32 = start_n;

    // Brent's cycle detection state
    var old_re = 0.0;
    var old_im = 0.0;
    var brent_power = 1u;
    var brent_lambda = 1u;

    // Perturbation iteration with escape check at the START of each iteration.
    // This avoids a second buffer read for reference_orbit[n+1] — we only read Z[n]
    // and use it for both the escape check (full_z = Z[n] + d[n]) and the
    // perturbation step (d_{n+1} = 2*Z[n]*d[n] + d[n]^2 + dc).
    for (var n: u32 = start_n; n < limit; n++) {
        let z = reference_orbit[n];

        // Escape + cycle check using full z_n = Z[n] + d[n]
        let full_re = z.x + d_re;
        let full_im = z.y + d_im;
        let norm_sq = full_re * full_re + full_im * full_im;
        if norm_sq > params.bailout {
            return color_pixel(norm_sq, n);
        }

        // Brent's cycle detection (epsilon-based for perturbation path)
        let diff_re = full_re - old_re;
        let diff_im = full_im - old_im;
        let diff_sq = diff_re * diff_re + diff_im * diff_im;
        if diff_sq < 1e-6 * norm_sq {
            return vec4f(0.0, 0.0, 0.0, 1.0);
        }
        brent_lambda -= 1u;
        if brent_lambda == 0u {
            old_re = full_re;
            old_im = full_im;
            brent_power *= 2u;
            brent_lambda = brent_power;
        }

        // d_{n+1} = 2*Z[n]*d[n] + d[n]^2 + dc
        let t1_re = 2.0 * (z.x * d_re - z.y * d_im);
        let t1_im = 2.0 * (z.x * d_im + z.y * d_re);
        let t2_re = d_re * d_re - d_im * d_im;
        let t2_im = 2.0 * d_re * d_im;
        d_re = t1_re + t2_re + dc_re;
        d_im = t1_im + t2_im + dc_im;

        iterations = n + 1u;
    }

    // Fallback: reference orbit escaped before max_iterations — direct f32 iteration
    if iterations < params.max_iterations {
        var z_re: f32;
        var z_im: f32;

        if limit < buffer_len {
            z_re = reference_orbit[limit].x + d_re;
            z_im = reference_orbit[limit].y + d_im;
        } else {
            z_re = d_re;
            z_im = d_im;
        }

        let c_re = params.center.x + dc_re;
        let c_im = params.center.y + dc_im;

        // Reset Brent's for direct iteration (exact equality works here —
        // no Z+d decomposition, so identical orbits produce identical f32 bits)
        var fb_old_re = 0.0;
        var fb_old_im = 0.0;
        var fb_power = 1u;
        var fb_lambda = 1u;

        for (var i = iterations; i < params.max_iterations; i++) {
            let re2 = z_re * z_re;
            let im2 = z_im * z_im;
            let norm_sq = re2 + im2;
            if norm_sq > params.bailout {
                return color_pixel(norm_sq, i);
            }
            let new_re = re2 - im2 + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_re;

            // Brent's cycle detection (exact equality for direct f32 iteration)
            if z_re == fb_old_re && z_im == fb_old_im {
                return vec4f(0.0, 0.0, 0.0, 1.0);
            }
            fb_lambda -= 1u;
            if fb_lambda == 0u {
                fb_old_re = z_re;
                fb_old_im = z_im;
                fb_power *= 2u;
                fb_lambda = fb_power;
            }
        }
    }

    // Interior — never escaped
    return vec4f(0.0, 0.0, 0.0, 1.0);
}
