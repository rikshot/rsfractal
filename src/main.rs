#![feature(test)]

use std::sync::mpsc;

extern crate serde_json;

extern crate num_cpus;
extern crate threadpool;

use threadpool::ThreadPool;

mod vector;
mod mandelbrot;
mod range;
mod rectangle;
mod color;

use mandelbrot::Config;
use mandelbrot::ChunkConfig;
use mandelbrot::ChunkResult;

fn render(config: &Config) -> image::RgbImage {
    let chunks = mandelbrot::chunkify(&config, config.chunk_size.unwrap_or(512));
    let pool = ThreadPool::new(num_cpus::get());

    let (result_sender, result_receiver) = mpsc::channel();
    for chunk in chunks {
        let sender = result_sender.clone();
        let config = config.clone();
        pool.execute(move || {
            let result = mandelbrot::iterate(&config, &chunk);
            sender.send((chunk, result)).unwrap();
        });
    }
    pool.join();
    drop(result_sender);

    let results: Vec<(ChunkConfig, ChunkResult)> = result_receiver.iter().collect();
    let mut histogram: Vec<u32> = vec![0; config.iterations as usize];
    let mut total: u32 = 0;
    for (_chunk, result) in &results {
        let mut index = 0;
        for iterations in &result.histogram {
            histogram[index] += iterations;
            index += 1;
        }
        total += result.total;
    }

    let (color_sender, color_receiver) = mpsc::channel();
    for (chunk, result) in results {
        let sender = color_sender.clone();
        let config = config.clone();
        let histogram = histogram.clone();
        pool.execute(move || {
            let pixels = mandelbrot::color(&config, &chunk, &result, &histogram, total);
            sender.send((chunk, pixels)).unwrap();
        });
    }
    pool.join();
    drop(color_sender);

    let mut image = image::RgbImage::new(config.width, config.height);
    for (chunk, pixels) in color_receiver.iter() {
        let mut index = 0;
        for y in chunk.screen.start.y..chunk.screen.end.y {
            for x in chunk.screen.start.x..chunk.screen.end.x {
                let pixel = &pixels[index];
                image.put_pixel(x, y, image::Rgb([pixel.r, pixel.g, pixel.b]));
                index += 1;
            }
        }
    }

    image
}

fn main() {
    let config: Config = serde_json::from_str(
        std::fs::read_to_string("config.json")
            .expect("config.json not found")
            .as_str(),
    )
    .expect("could not parse config.json");

    println!("{:?}", config);
    let image = render(&config);
    image.save("fractal.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    #[bench]
    fn render(bencher: &mut test::Bencher) {
        let config: Config = serde_json::from_str(
            std::fs::read_to_string("config.json")
                .expect("config.json not found")
                .as_str(),
        ).expect("could not parse config.json");
        bencher.iter(|| {
            super::render(&config);
        })
    }
}
