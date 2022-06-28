use rsfractal_mandelbrot::color::*;
use rsfractal_mandelbrot::mandelbrot::*;

use rayon::prelude::*;
use seed::prelude::*;
use wasm_bindgen::JsCast;

use futures::channel::oneshot;

#[derive(Clone)]
pub struct Model {
    pub config: Config,
    zoom_factor: f64,
    concurrency: usize,
    worker_pool: super::pool::WorkerPool,
    pub rendering: bool,
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
            rendering: false,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Render,
    ParseConfig(String),
    RenderDone,
    Reset,
    Click(web_sys::MouseEvent),
    ChangeIterations(String),
    ChangeColors(String),
    Export,
    UrlChanged(seed::app::subs::UrlChanged),
}

async fn progressive_render(model: &Model) {
    let mut i = 32;
    while i < model.config.iterations {
        let mut model = model.clone();
        model.config.iterations = i;
        wasm_bindgen_futures::JsFuture::from(render(&model).unwrap())
            .await
            .unwrap();
        i *= 2;
    }
}

fn render(model: &Model) -> Option<js_sys::Promise> {
    let canvas = seed::canvas("canvas")?;
    let context = seed::canvas_context_2d(&canvas);

    let config = model.config.clone();
    let width = config.width;
    let height = config.height;
    let size = width * height * 4;
    let mut pixels = vec![255u8; size as usize];
    let base = pixels.as_ptr() as usize;

    let worker_pool = &model.worker_pool;
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(model.concurrency)
        .spawn_handler(|thread| {
            worker_pool.run(|| thread.run()).unwrap();
            Ok(())
        })
        .build()
        .unwrap();

    let (tx, rx) = oneshot::channel();
    worker_pool
        .run(move || {
            thread_pool.install(move || {
                let global = js_sys::global().unchecked_into::<web_sys::Window>();
                let performance = global.performance().unwrap();
                let start = performance.now();
                let chunks = chunkify(&config);
                let results: Vec<_> = chunks.par_iter().map(|chunk| iterate(&config, chunk)).collect();
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
                    .map(|(chunk, result)| color(&config, chunk, &result, &histogram, total))
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
                });
                let end = performance.now();
                tx.send(end - start).unwrap();
            });
        })
        .unwrap();

    Some(wasm_bindgen_futures::future_to_promise(async move {
        let duration = rx.await.unwrap();
        let image_data = image_data(base, size as usize, width as u32, height as u32);
        context
            .put_image_data(&image_data.unchecked_into::<web_sys::ImageData>(), 0.0, 0.0)
            .unwrap();
        seed::log![format!("Rendering took: {}ms", duration)];
        Ok(JsValue::UNDEFINED)
    }))
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

fn parse_config(raw_config: String) -> Option<Config> {
    let decoded = base64::decode(&raw_config).ok()?;
    rmp_serde::from_slice(&decoded).ok()?
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Render => {
            if !model.rendering {
                model.rendering = true;
                let model = model.clone();
                orders.perform_cmd(async move {
                    progressive_render(&model).await;
                    Msg::RenderDone
                });
            }
        }
        Msg::ParseConfig(raw_config) => {
            let config = parse_config(raw_config);
            model.config = config.unwrap_or_default();
            orders.send_msg(Msg::Render);
        }
        Msg::RenderDone => {
            model.rendering = false;
        }
        Msg::Reset => {
            model.config = Config::default();
            let mut url = seed::browser::url::Url::current();
            url = url.set_hash("");
            orders.request_url(url);
        }
        Msg::Click(ev) => {
            let target = &ev.target().unwrap();
            let element = seed::to_html_el(target);
            if element.id() == "canvas" && !model.rendering {
                ev.prevent_default();
                let rect = element.get_bounding_client_rect();
                let x = ev.client_x() as f64 - rect.left();
                let y = ev.client_y() as f64 - rect.top();
                let zoom_factor = if ev.shift_key() {
                    1.0 / model.zoom_factor
                } else {
                    model.zoom_factor
                };
                zoom(&mut model.config, x, y, rect.width(), rect.height(), zoom_factor);
                orders.send_msg(Msg::Render);
            }
            orders.skip();
        }
        Msg::ChangeIterations(input) => {
            model.config.iterations = input.parse::<usize>().unwrap_or(Config::default().iterations);
        }
        Msg::ChangeColors(input) => {
            let colors: Vec<&str> = input.split(',').collect();
            model.config.palette = colors.iter().filter_map(|hex| Color::from_hex(hex)).collect();
            if model.config.palette.is_empty() {
                model.config.palette = Config::default().palette;
            }
        }
        Msg::Export => {
            let buffer = rmp_serde::to_vec(&model.config).unwrap();
            let encoded = base64::encode(&buffer);
            let mut url = seed::browser::url::Url::current();
            url = url.set_hash_path(&[&encoded]);
            url.go_and_replace();
            orders.skip();
        }
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            if url.hash().is_none() {
                orders.send_msg(Msg::Render);
            } else {
                orders.send_msg(Msg::ParseConfig(url.hash().unwrap().to_string()));
            }
        }
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    orders.stream(seed::app::streams::window_event(Ev::Click, |ev| {
        Msg::Click(ev.unchecked_into())
    }));
    orders.send_msg(Msg::UrlChanged(subs::UrlChanged(url)));
    Model::default()
}

#[wasm_bindgen(start)]
pub fn main() {
    if js_sys::global().has_type::<web_sys::Window>() {
        App::start("app", init, update, super::ui::view);
    }
}
