use js_sys::Uint8ClampedArray;
use leptos::html::Canvas;
use leptos::logging;
use leptos::prelude::*;
use rsfractal_mandelbrot::mandelbrot::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;

pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::MouseEvent;

#[component]
fn App() -> impl IntoView {
    let (config, set_config) = signal(Config::default());

    let canvas_ref = NodeRef::<Canvas>::new();

    let on_render = move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
            let width = canvas.width() as usize;
            let height = canvas.height() as usize;
            let size = width * height * 4;
            let mut pixels = vec![0u8; size];
            render(&config(), &mut pixels);
            let array = unsafe { Uint8ClampedArray::view(&pixels) };
            let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(
                &array.slice(0, size as u32),
                width as u32,
                height as u32,
            )
            .unwrap();
            context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    };

    let on_click = move |event: MouseEvent| {
        logging::log!("{:?}", event);
        let mut config = config();
        config.zoom(event.client_x() as f64, event.client_y() as f64, 0.25);
        set_config(config);
    };

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
            let width = canvas.width() as usize;
            let height = canvas.height() as usize;
            let size = width * height * 4;
            let mut pixels = vec![0u8; size];
            render(&config(), &mut pixels);
            let array = unsafe { Uint8ClampedArray::view(&pixels) };
            let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(
                &array.slice(0, size as u32),
                width as u32,
                height as u32,
            )
            .unwrap();
            context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    });

    view! {
        <main>
            <header>
                <h1>"rsfractal"</h1>
                <button on:click=on_render>"Render"</button>
            </header>
            <canvas width=1280 height=720 node_ref=canvas_ref on:click=on_click />
        </main>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
