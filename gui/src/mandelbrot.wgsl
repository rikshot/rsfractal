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

struct Rectangle {
    start: vec2f,
    end: vec2f
}

struct Ranges {
    width_range: Range,
    @align(16) height_range: Range,
    @align(16) real_range: Range,
    @align(16) imaginary_range: Range
}

@group(0) @binding(0)
var<uniform> ranges: Ranges;

@fragment
fn fs_main(@builtin(position) input: vec4f) -> @location(0) vec4f {
    let viewport_width_range = Range(0.0, 2560.0);
    let viewport_height_range = Range(0.0, 1440.0);

    let c = Complex(scale(ranges.width_range, scale(viewport_width_range, input.x, ranges.width_range), ranges.real_range), scale(ranges.height_range, scale(viewport_height_range, input.y, ranges.height_range), ranges.imaginary_range));

    var temp: f32 = 0.0;
    var z = Complex(0.0, 0.0);
    var old = Complex(0.0, 0.0);
    var iterations: i32 = 0;
    var period: i32 = 0;

    var re2 = 0.0;
    var im2 = c.im * c.im;
    var q = c.re - 0.25;
    q *= q;
    q += im2;

    if q * (q + (c.re - 0.25)) < 0.25 * im2 {
        iterations = 1000;
    } else {
        while re2 + im2 <= 65536.0 && iterations < 1000 {
            re2 = z.re * z.re;
            im2 = z.im * z.im;
            temp = re2 - im2 + c.re;
            z.im = 2.0 * z.re * z.im + c.im;
            z.re = temp;
            if z.re == old.re && z.im == old.im {
                iterations = 1000;
				break;
            }
            iterations += 1;
            period += 1;
            if period > 10 {
                period = 0;
                old = z;
            }
        }
    }

    if iterations == 1000 {
        temp = 1000.0;
    } else {
        let ln2 = log(2.0);
        let zn = log(re2 + im2) / 2.0;
        let nu = log(zn / ln2) / ln2;
        temp = f32(iterations) + 1.0 - nu;
    }

    var v = temp / 1000.0;
    return vec4f(v, v, v, 1.0);
}