use super::mandelbrot::*;

use seed::{prelude::*, *};

#[derive(Clone)]
enum Msg {
    Render,
    Reset,
}

fn render(config: &Config) -> Option<()> {
    let canvas = canvas("canvas")?;
    let context = canvas_context_2d(&canvas);

    let chunks = chunkify(&config);
    let results: Vec<_> = chunks.iter().map(|chunk| iterate(&config, &chunk)).collect();
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
        .iter()
        .zip(results)
        .map(|(chunk, result)| color(&config, &chunk, &result, &histogram, total))
        .collect();
    chunks.iter().zip(colors).for_each(|(chunk, colors)| {
        let mut index = 0;
        for y in chunk.screen.start.y..chunk.screen.end.y {
            for x in chunk.screen.start.x..chunk.screen.end.x {
                let pixel = &colors[index];
                context.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", pixel.r, pixel.g, pixel.b)));
                context.fill_rect(x.into(), y.into(), 1.0, 1.0);
                index += 1;
            }
        }
    });
    Some(())
}

fn update(msg: Msg, config: &mut Config, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Render => render(config).unwrap(),
        Msg::Reset => {
            *config = Config::default();
            render(config).unwrap();
        }
    }
}

fn view(_model: &Config) -> impl View<Msg> {
    vec![
        button![simple_ev(Ev::Click, Msg::Render), "Render"],
        button![simple_ev(Ev::Click, Msg::Reset), "Reset"],
    ]
}

#[wasm_bindgen(start)]
pub fn main() {
    App::builder(update, view).build_and_start();
}
