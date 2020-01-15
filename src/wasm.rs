use super::mandelbrot::*;
use super::range::*;
use super::vector::*;

use seed::{prelude::*, *};

pub struct Model {
    pub config: Config,
    pub zoom_factor: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            config: Config::default(),
            zoom_factor: 0.25,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Render,
    Reset,
    Click(web_sys::MouseEvent),
    ChangeIterations(String),
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

fn zoom(config: &mut Config, x: f64, y: f64, width: f64, height: f64, zoom_factor: f64) {
    let width_range = Range::new(0.0, width);
    let height_range = Range::new(0.0, height);
    let selection = rect_from_position(&config.position, &config.zoom);
    let real_range = Range::new(selection.start.x, selection.end.x);
    let imaginary_range = Range::new(selection.start.y, selection.end.y);
    config.position = Vector {
        x: Range::scale(&width_range, x, &real_range),
        y: Range::scale(&height_range, y, &imaginary_range),
    };
    config.zoom = Vector {
        x: config.zoom.x * zoom_factor,
        y: config.zoom.y * zoom_factor,
    };
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Render => render(&model.config).unwrap(),
        Msg::Reset => {
            model.config = Config::default();
            render(&model.config).unwrap();
        }
        Msg::Click(ev) => {
            let target = &ev.target().unwrap();
            let element = to_html_el(target);
            if element.id() == "canvas" {
                let rect = element.get_bounding_client_rect();
                let x = ev.client_x() as f64 - rect.left();
                let y = ev.client_y() as f64 - rect.top();
                zoom(&mut model.config, x, y, rect.width(), rect.height(), model.zoom_factor);
                render(&model.config).unwrap();
            }
        }
        Msg::ChangeIterations(input) => {
            model.config.iterations = input.parse::<usize>().unwrap_or(Config::default().iterations)
        }
    }
}

fn window_events(_model: &Model) -> Vec<seed::virtual_dom::Listener<Msg>> {
    let mut listeners = Vec::new();
    listeners.push(mouse_ev("click", |ev| Msg::Click(ev)));
    listeners
}

#[wasm_bindgen(start)]
pub fn main() {
    App::builder(update, super::ui::view)
        .window_events(window_events)
        .build_and_start();
}
