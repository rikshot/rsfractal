use rsfractal_mandelbrot::mandelbrot::Mandelbrot;

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 10, args = (10..30).map(|n| usize::pow(2, n)))]
fn chunk_size(chunk_size: usize) {
    let mut mandelbrot = Mandelbrot::default();
    mandelbrot.chunk_size = chunk_size;
    let mut buffer = vec![0u8; mandelbrot.width * mandelbrot.height * 4];
    mandelbrot.render(&mut buffer);
}

#[divan::bench(sample_count = 10, args = 1..40)]
fn period_length(period_length: usize) {
    let mut mandelbrot = Mandelbrot::default();
    mandelbrot.period_length = period_length;
    let mut buffer = vec![0u8; mandelbrot.width * mandelbrot.height * 4];
    mandelbrot.render(&mut buffer);
}
