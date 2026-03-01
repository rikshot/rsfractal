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
    sa_a: vec2f,
    sa_b: vec2f,
    sa_c: vec2f,
    skip_iterations: u32,
    sa_scale: f32,
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

fn color_pixel(norm_sq: f32, iterations: u32) -> vec4f {
    let ln2 = log(2.0);
    let zn = log(norm_sq) / 2.0;
    let nu = log(zn / ln2) / ln2;
    let smoothed = max(f32(iterations) + 1.0 - nu, 0.0);
    let s = pow(smoothed / f32(params.max_iterations), params.exponent);
    return textureSampleLevel(color_texture, color_sampler, vec2f(s, 0.5), 0.0);
}

@fragment
fn fs_main(@builtin(position) input: vec4f) -> @location(0) vec4f {
    let frac = input.xy / params.viewport;

    // Pixel offset in normalized coordinates [-1, 1]
    let pixel_offset = vec2f(frac.x * 2.0 - 1.0, frac.y * 2.0 - 1.0);

    // dc = pixel offset from reference point in fractal coordinates
    // At deep zoom, dc can be subnormal f32 — GPUs flush subnormals to zero (FTZ).
    // This is fine for the perturbation loop (dc is negligible there) but NOT for SA,
    // where u = dc/scale must be computed without subnormal intermediates.
    let dc_re = pixel_offset.x * params.zoom.x - params.reference_offset.x;
    let dc_im = pixel_offset.y * params.zoom.y - params.reference_offset.y;

    let limit = params.orbit_length;
    let buffer_len = arrayLength(&reference_orbit);

    // Initialize from SA polynomial or zero
    var d_re = 0.0;
    var d_im = 0.0;
    var start_n = 0u;
    if params.skip_iterations > 0u && params.skip_iterations < limit {
        // u = pixel_offset * (zoom/scale) - ref_offset/scale
        // Precomputed on CPU to ensure bit-identical evaluation with CPU path.
        // Also avoids subnormal intermediates at deep zoom (GPU FTZ).
        let u = pixel_offset * params.sa_zoom_norm - params.sa_ref_offset_norm;
        let u2 = vec2f(u.x * u.x - u.y * u.y, 2.0 * u.x * u.y);
        let u3 = vec2f(u2.x * u.x - u2.y * u.y, u2.x * u.y + u2.y * u.x);

        d_re = params.sa_a.x * u.x - params.sa_a.y * u.y
            + params.sa_b.x * u2.x - params.sa_b.y * u2.y
            + params.sa_c.x * u3.x - params.sa_c.y * u3.y;
        d_im = params.sa_a.x * u.y + params.sa_a.y * u.x
            + params.sa_b.x * u2.y + params.sa_b.y * u2.x
            + params.sa_c.x * u3.y + params.sa_c.y * u3.x;
        start_n = params.skip_iterations;
    }

    var iterations: u32 = start_n;

    // Perturbation iteration using reference orbit
    // Note: NO per-pixel rebasing on GPU — it causes severe artifacts because
    // GPU FMA non-determinism makes adjacent pixels rebase at different iterations,
    // destroying spatial coherence in the chaotic iteration that follows.
    for (var n: u32 = start_n; n < limit; n++) {
        let z = reference_orbit[n];

        // d_new = 2*Z*d + d*d + dc
        let new_d_re = 2.0 * (z.x * d_re - z.y * d_im) + d_re * d_re - d_im * d_im + dc_re;
        let new_d_im = 2.0 * (z.x * d_im + z.y * d_re) + 2.0 * d_re * d_im + dc_im;
        d_re = new_d_re;
        d_im = new_d_im;

        // Escape check: use buffer length so we can check on the last loop iteration
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
            // Orbit escaped: values[limit] = Z_escape. Full z = Z_limit + d_limit.
            z_re = reference_orbit[limit].x + d_re;
            z_im = reference_orbit[limit].y + d_im;
        } else {
            z_re = d_re;
            z_im = d_im;
        }

        // c = orbit_center + dc (correct for any reference_offset)
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
