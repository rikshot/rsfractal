use rayon::prelude::*;

use rsfractal::mandelbrot;

fn render(config: &mandelbrot::Config) -> image::RgbImage {
    let chunks = mandelbrot::chunkify(&config);
    let results: Vec<_> = chunks
        .par_iter()
        .map(|chunk| mandelbrot::iterate(&config, &chunk))
        .collect();
    let (histogram, total) = results
        .iter()
        .fold((vec![0; config.iterations], 0), |(histogram, total), result| {
            (
                result
                    .histogram
                    .iter()
                    .enumerate()
                    .map(|(index, iterations)| histogram[index] + iterations)
                    .collect(),
                total + result.total,
            )
        });
    let colors: Vec<_> = chunks
        .par_iter()
        .zip(results)
        .map(|(chunk, result)| mandelbrot::color(&config, &chunk, &result, &histogram, total))
        .collect();
    chunks.iter().zip(colors).fold(
        image::RgbImage::new(config.width, config.height),
        |mut image, (chunk, colors)| {
            let mut index = 0;
            for y in chunk.screen.start.y..chunk.screen.end.y {
                for x in chunk.screen.start.x..chunk.screen.end.x {
                    let pixel = &colors[index];
                    image.put_pixel(x, y, image::Rgb([pixel.r, pixel.g, pixel.b]));
                    index += 1;
                }
            }
            image
        },
    )
}

fn main() {
    let config: mandelbrot::Config = serde_json::from_str(
        std::fs::read_to_string("config.json")
            .expect("config.json not found")
            .as_str(),
    )
    .expect("could not parse config.json");

    println!("{:?}", config);
    let image = render(&config);
    image.save("fractal.png").unwrap();
}
