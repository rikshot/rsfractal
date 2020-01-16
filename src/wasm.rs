use super::mandelbrot::*;
use super::range::*;
use super::vector::*;

use rayon::prelude::*;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

use futures::channel::mpsc;
use futures_util::stream::StreamExt;

pub struct Model {
    pub config: Config,
    pub zoom_factor: f64,
    pub concurrency: usize,
    worker_pool: super::pool::WorkerPool,
}

impl Default for Model {
    fn default() -> Self {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let concurrency = navigator.hardware_concurrency() as usize;
        let worker_pool = super::pool::WorkerPool::new(concurrency).unwrap();

        Self {
            config: Config::default(),
            zoom_factor: 0.25,
            concurrency,
            worker_pool,
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

fn render(model: &Model) -> Option<()> {
    let canvas = canvas("canvas")?;
    let context = canvas_context_2d(&canvas);

    let config = model.config.clone();
    let width = config.width;
    let height = config.height;
    let size = width * height * 4;
    let mut pixels = vec![255u8; size as usize];
    let base = pixels.as_ptr() as usize;

    let worker_pool = &model.worker_pool;
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(model.concurrency)
        .spawn_handler(|thread| Ok(worker_pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap();

    let (tx, rx) = mpsc::unbounded();
    worker_pool
        .run(move || {
            thread_pool.install(move || {
                let chunks = chunkify(&config);
                let results: Vec<_> = chunks.par_iter().map(|chunk| iterate(&config, &chunk)).collect();
                let (histogram, total) =
                    results
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
                    .map(|(chunk, result)| color(&config, &chunk, &result, &histogram, total))
                    .collect();
                chunks.iter().zip(colors).for_each(|(chunk, colors)| {
                    let mut index = 0;
                    for y in chunk.screen.start.y..chunk.screen.end.y {
                        for x in chunk.screen.start.x..chunk.screen.end.x {
                            let color = &colors[index];
                            let pixel_index = (y as usize * config.width as usize * 4) + x as usize * 4;
                            pixels[pixel_index] = color.r;
                            pixels[pixel_index + 1] = color.g;
                            pixels[pixel_index + 2] = color.b;
                            index += 1;
                        }
                    }
                    tx.unbounded_send(chunk.clone()).unwrap();
                });
            });
        })
        .unwrap();

    wasm_bindgen_futures::spawn_local(rx.for_each(move |chunk| {
        let image_data = image_data(base, size as usize, width as u32, height as u32);
        context
            .put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
                &image_data.unchecked_into::<web_sys::ImageData>(),
                0.0,
                0.0,
                chunk.screen.start.x as f64,
                chunk.screen.start.y as f64,
                chunk.screen.width() as f64,
                chunk.screen.height() as f64,
            )
            .unwrap();
        futures::future::ready(())
    }));

    Some(())
}

#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor, catch)]
    fn new(data: &js_sys::Uint8ClampedArray, width: f64, height: f64) -> Result<ImageData, JsValue>;
}

fn image_data(base: usize, len: usize, width: u32, height: u32) -> ImageData {
    let mem = wasm_bindgen::memory().unchecked_into::<js_sys::WebAssembly::Memory>();
    let mem = js_sys::Uint8ClampedArray::new(&mem.buffer()).slice(base as u32, (base + len) as u32);
    ImageData::new(&mem, width as f64, height as f64).unwrap()
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
        Msg::Render => {
            render(model);
        }
        Msg::Reset => {
            model.config = Config::default();
            render(model);
        }
        Msg::Click(ev) => {
            let target = &ev.target().unwrap();
            let element = to_html_el(target);
            if element.id() == "canvas" {
                let rect = element.get_bounding_client_rect();
                let x = ev.client_x() as f64 - rect.left();
                let y = ev.client_y() as f64 - rect.top();
                zoom(&mut model.config, x, y, rect.width(), rect.height(), model.zoom_factor);
                render(model);
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

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let model = Model::default();
    render(&model);
    AfterMount::new(model)
}

#[wasm_bindgen(start)]
pub fn main() {
    if js_sys::global().has_type::<web_sys::Window>() {
        App::builder(update, super::ui::view)
            .after_mount(after_mount)
            .window_events(window_events)
            .build_and_start();
    }
}
