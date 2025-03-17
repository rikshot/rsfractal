use rsfractal_mandelbrot::mandelbrot::Mandelbrot;

fn main() {
    let mandelbrot = Mandelbrot::default();
    let mut buffer = vec![0u8; mandelbrot.width as usize * mandelbrot.height as usize * 4];
    mandelbrot.render(&mut buffer);
    image::save_buffer(
        "fractal.png",
        &buffer,
        mandelbrot.width,
        mandelbrot.height,
        image::ExtendedColorType::Rgba8,
    )
    .unwrap();
}
