@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    var positions = array(vec2f(- 1.0, - 1.0), vec2f(3.0, - 1.0), vec2f(- 1.0, 3.0));
    return vec4f(positions[vertex_index], 0.0, 1.0);
}

struct Range {
    min: f32,
    max: f32
}

fn scale(input: Range, value: f32, output: Range) -> f32 {
    let input_size = abs(input.max - input.min);
    let output_size = abs(output.max - output.min);
    return (input.max * output.min - input.min * output.max + value * output_size) / input_size;
}

struct Complex {
    re: f32,
    im: f32
}

struct Params {
    real_range: Range,
    @align(16) imaginary_range: Range,
    @align(16) viewport: vec2f,
    bailout: f32,
    max_iterations: u32,
    exponent: f32,
}

@group(0) @binding(0)
var<uniform> params: Params;

@group(0) @binding(1)
var color_texture: texture_2d<f32>;

@group(0) @binding(2)
var color_sampler: sampler;

@fragment
fn fs_main(@builtin(position) input: vec4f) -> @location(0) vec4f {
    let c = Complex(
        scale(Range(0.0, params.viewport.x), input.x, params.real_range),
        scale(Range(0.0, params.viewport.y), input.y, params.imaginary_range)
    );

    var temp: f32 = 0.0;
    var z = Complex(0.0, 0.0);
    var iterations: u32 = 0u;

    var re2 = 0.0;
    var im2 = c.im * c.im;
    var q = c.re - 0.25;
    q *= q;
    q += im2;

    let p2 = c.re + 1.0;
    if q * (q + (c.re - 0.25)) < 0.25 * im2 || p2 * p2 + im2 < 0.0625 {
        iterations = params.max_iterations;
    } else {
        while re2 + im2 <= params.bailout && iterations < params.max_iterations {
            re2 = z.re * z.re;
            im2 = z.im * z.im;
            temp = re2 - im2 + c.re;
            z.im = 2.0 * z.re * z.im + c.im;
            z.re = temp;
            iterations += 1u;
        }
    }

    if iterations == params.max_iterations {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }

    let ln2 = log(2.0);
    let zn = log(re2 + im2) / 2.0;
    let nu = log(zn / ln2) / ln2;
    temp = f32(iterations) + 1.0 - nu;

    let s = pow(temp / f32(params.max_iterations), params.exponent);

    return textureSampleLevel(color_texture, color_sampler, vec2f(s, 0.5), 0.0);
}
