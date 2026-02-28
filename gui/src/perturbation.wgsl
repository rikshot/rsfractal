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
}

@group(0) @binding(0)
var<uniform> params: PerturbationParams;

@group(0) @binding(1)
var color_texture: texture_2d<f32>;

@group(0) @binding(2)
var color_sampler: sampler;

@group(0) @binding(3)
var<storage, read> reference_orbit: array<vec2f>;

@fragment
fn fs_main(@builtin(position) input: vec4f) -> @location(0) vec4f {
    let frac = input.xy / params.viewport;

    // dc = pixel offset from reference point
    let dc_re = (frac.x * 2.0 - 1.0) * params.zoom.x - params.reference_offset.x;
    let dc_im = (frac.y * 2.0 - 1.0) * params.zoom.y - params.reference_offset.y;

    var d_re = 0.0;
    var d_im = 0.0;
    var iterations: u32 = 0u;
    var final_norm_sq: f32 = 0.0;

    let limit = params.orbit_length;

    // Perturbation iteration using reference orbit
    for (var n: u32 = 0u; n < limit && n < params.max_iterations; n++) {
        let z = reference_orbit[n];

        // d_new = 2*Z*d + d*d + dc
        let new_d_re = 2.0 * (z.x * d_re - z.y * d_im) + d_re * d_re - d_im * d_im + dc_re;
        let new_d_im = 2.0 * (z.x * d_im + z.y * d_re) + 2.0 * d_re * d_im + dc_im;
        d_re = new_d_re;
        d_im = new_d_im;

        // Escape check
        if n + 1u < limit {
            let full_re = reference_orbit[n + 1u].x + d_re;
            let full_im = reference_orbit[n + 1u].y + d_im;
            let norm_sq = full_re * full_re + full_im * full_im;
            if norm_sq > params.bailout {
                final_norm_sq = norm_sq;
                iterations = n + 1u;
                break;
            }
        }

        iterations = n + 1u;
    }

    // Fallback: reference orbit escaped before this pixel.
    // Continue with direct f32 iteration from current Z + d.
    if final_norm_sq == 0.0 && iterations < params.max_iterations && limit > 0u {
        var z_re = reference_orbit[limit - 1u].x + d_re;
        var z_im = reference_orbit[limit - 1u].y + d_im;
        // Pixel's c (f32 precision — best effort for fallback)
        let c_re = params.center.x + (frac.x * 2.0 - 1.0) * params.zoom.x;
        let c_im = params.center.y + (frac.y * 2.0 - 1.0) * params.zoom.y;

        for (var i = iterations; i < params.max_iterations; i++) {
            let re2 = z_re * z_re;
            let im2 = z_im * z_im;
            let norm_sq = re2 + im2;
            if norm_sq > params.bailout {
                final_norm_sq = norm_sq;
                iterations = i;
                break;
            }
            let new_re = re2 - im2 + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = new_re;
        }
    }

    // Interior — never escaped
    if final_norm_sq == 0.0 {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }

    // Smooth coloring
    let ln2 = log(2.0);
    let zn = log(final_norm_sq) / 2.0;
    let nu = log(zn / ln2) / ln2;
    let smoothed = max(f32(iterations) + 1.0 - nu, 0.0);

    let s = pow(smoothed / f32(params.max_iterations), params.exponent);

    return textureSampleLevel(color_texture, color_sampler, vec2f(s, 0.5), 0.0);
}
