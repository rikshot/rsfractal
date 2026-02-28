use divan::Bencher;
use rsfractal_mandelbrot::mandelbrot::{Mandelbrot, Rendering};

fn main() {
    divan::main();
}

#[divan::bench]
fn approximate(bencher: Bencher) {
    let mandelbrot = Mandelbrot::default();
    let mut buffer = vec![0; mandelbrot.width * mandelbrot.height * 4];

    bencher.bench_local(|| {
        mandelbrot.render(&mut buffer);
    });
}

#[divan::bench]
fn smooth(bencher: Bencher) {
    let mut mandelbrot = Mandelbrot::default();
    mandelbrot.rendering = Rendering::Smooth;
    let mut buffer = vec![0; mandelbrot.width * mandelbrot.height * 4];

    bencher.bench_local(|| {
        mandelbrot.render(&mut buffer);
    });
}
