use rsfractal_mandelbrot::mandelbrot::{Config, render};

fn main() {
    let config = Config::default();
    let mut buffer = vec![0u8; config.width as usize * config.height as usize * 4];
    render(&config, &mut buffer);
    image::save_buffer(
        "fractal.png",
        &buffer,
        config.width,
        config.height,
        image::ExtendedColorType::Rgba8,
    )
    .unwrap();
}
