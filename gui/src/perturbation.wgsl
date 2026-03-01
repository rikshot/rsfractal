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

fn color_pixel(norm_sq: f32, iterations: u32) -> vec4f {
    let ln2 = log(2.0);
    let zn = log(norm_sq) / 2.0;
    let nu = log(zn / ln2) / ln2;
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

    // Perturbation iteration: d_{n+1} = 2*Z_n*d_n + d_n^2 + dc
    for (var n: u32 = start_n; n < limit; n++) {
        let z = reference_orbit[n];

        // 2*Z*d (complex multiply, Z is reference orbit value)
        let t1_re = 2.0 * (z.x * d_re - z.y * d_im);
        let t1_im = 2.0 * (z.x * d_im + z.y * d_re);

        // d*d
        let t2_re = d_re * d_re - d_im * d_im;
        let t2_im = 2.0 * d_re * d_im;

        // d_new = 2*Z*d + d*d + dc
        d_re = t1_re + t2_re + dc_re;
        d_im = t1_im + t2_im + dc_im;

        // Escape check using full z_{n+1} = Z_{n+1} + d_{n+1}
        if n + 1u < buffer_len {
            let full_re = reference_orbit[n + 1u].x + d_re;
            let full_im = reference_orbit[n + 1u].y + d_im;
            let norm_sq = full_re * full_re + full_im * full_im;
            if norm_sq > params.bailout {
                return color_pixel(norm_sq, n + 1u);
            }
        }

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
        }
    }

    // Interior — never escaped
    return vec4f(0.0, 0.0, 0.0, 1.0);
}
