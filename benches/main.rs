use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use rayon::prelude::*;

use rsfractal::mandelbrot::*;

fn render(config: &Config) -> image::RgbImage {
    let chunks = chunkify(&config);
    let results: Vec<_> = chunks
        .par_iter()
        .map(|chunk| iterate(&config, &chunk))
        .collect();
    let (histogram, total) = results.iter().fold(
        (vec![0; config.iterations], 0),
        |(histogram, total), result| {
            (
                result
                    .histogram
                    .iter()
                    .enumerate()
                    .map(|(index, iterations)| histogram[index] + iterations)
                    .collect(),
                total + result.total,
            )
        },
    );
    let colors: Vec<_> = chunks
        .par_iter()
        .zip(results)
        .map(|(chunk, result)| color(&config, &chunk, &result, &histogram, total))
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

fn criterion_benchmark(c: &mut Criterion) {
    static MIN_CHUNK_SIZE: usize = 32;

    let mut config: Config = serde_json::from_str(
        std::fs::read_to_string("config.json")
            .expect("config.json not found")
            .as_str(),
    )
    .expect("could not parse config.json");

    let mut group = c.benchmark_group("chunk_size");
    for size in 1..10 {
        let chunk_size = size * MIN_CHUNK_SIZE;
        group.throughput(Throughput::Bytes(chunk_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &chunk_size,
            |b, &size| {
                config.chunk_size = Some(size as u32);
                b.iter(|| render(&config));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
