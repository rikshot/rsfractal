use futures::future::{join_all, lazy};
use futures::executor::ThreadPool;
use futures::task::SpawnExt;

mod vector;
mod mandelbrot;
mod range;
mod rectangle;
mod color;

use mandelbrot::Config;

async fn render(mut pool: ThreadPool, config: &Config) -> image::RgbImage {
    let chunks = mandelbrot::chunkify(&config, config.chunk_size.unwrap_or(512));

    let length = chunks.len();
    let mut chunk_futures = Vec::with_capacity(length);
    for chunk in chunks {
        let config = config.clone();
        let future = pool.spawn_with_handle(lazy(move |_| {
            let result = mandelbrot::iterate(&config, &chunk);
            (chunk, result)
        })).unwrap();
        chunk_futures.push(future);
    }
    let chunk_results = join_all(chunk_futures).await;

    let mut histogram: Vec<u32> = vec![0; config.iterations as usize];
    let mut total: u32 = 0;
    for (_chunk, result) in &chunk_results {
        let mut index = 0;
        for iterations in &result.histogram {
            histogram[index] += iterations;
            index += 1;
        }
        total += result.total;
    }

    let mut color_futures = Vec::with_capacity(length);
    for (chunk, result) in chunk_results {
        let config = config.clone();
        let histogram = histogram.clone();
        let future = pool.spawn_with_handle(lazy(move |_| {
            let pixels = mandelbrot::color(&config, &chunk, &result, &histogram, total);
            (chunk, pixels)
        })).unwrap();
        color_futures.push(future);
    }

    let mut image = image::RgbImage::new(config.width, config.height);
    for (chunk, pixels) in join_all(color_futures).await {
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
    let mut pool = ThreadPool::new().expect("unable to create a threadpool");
    let image = pool.run(render(pool.clone(), &config));
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
        let mut pool = ThreadPool::new().expect("unable to create a threadpool");
        bencher.iter(|| {
            pool.run(super::render(pool.clone(), &config));
        })
    }
}
