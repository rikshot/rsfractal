use std::str::FromStr;
use std::sync::Arc;

use futures::channel::oneshot;
use js_sys::Uint8ClampedArray;
use leptos::html::Canvas;
use leptos::prelude::*;
use rsfractal_mandelbrot::mandelbrot::*;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::Element;

use strum::IntoEnumIterator;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::HtmlSelectElement;
use web_sys::ImageData;
use web_sys::MouseEvent;

#[derive(Serialize)]
struct ContextAttributes {
    alpha: bool,
    desynchronized: bool,
}

#[component]
fn App() -> impl IntoView {
    let (config, set_config) = signal(Config::default());

    let action = Action::new(|config: &Config| {
        let config = config.clone();
        let (sender, receiver) = oneshot::channel::<Arc<Vec<u8>>>();
        async move {
            rayon::spawn(move || {
                let size = config.width * config.height * 4;
                let mut pixels = vec![0u8; size as usize];
                render(&config, &mut pixels);
                let pixels = Arc::new(pixels);
                sender.send(pixels).unwrap();
            });
            receiver.await.unwrap()
        }
    });

    let canvas_ref = NodeRef::<Canvas>::new();

    let render = move || {
        action.dispatch(config());
    };

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            if let Some(pixels) = action.value().get() {
                let width = canvas.width() as u32;
                let height = canvas.height() as u32;
                let size = width * height * 4;
                let data = Uint8ClampedArray::new_with_length(size);
                data.copy_from(&pixels);
                let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(&data, width, height).unwrap();
                let context: CanvasRenderingContext2d = canvas
                    .get_context_with_context_options(
                        "2d",
                        &serde_wasm_bindgen::to_value(&ContextAttributes {
                            alpha: false,
                            desynchronized: true,
                        })
                        .unwrap(),
                    )
                    .unwrap()
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();
                context.set_image_smoothing_enabled(false);
                context.put_image_data(&image_data, 0.0, 0.0).unwrap();
            }
        }
    });

    let on_click = move |event: MouseEvent| {
        if let Some(canvas) = canvas_ref.get() {
            event.prevent_default();
            let element: Element = event.target().unwrap().dyn_into().unwrap();
            let rect = element.get_bounding_client_rect();
            let scale_x = canvas.width() as f64 / rect.width();
            let scale_y = canvas.height() as f64 / rect.height();
            let x = (event.client_x() as f64 - rect.left()) * scale_x;
            let y = (event.client_y() as f64 - rect.top()) * scale_y;
            let mut config = config();
            let zoom_factor = if event.shift_key() { 1.0 / 0.25 } else { 0.25 };
            config.zoom(x, y, zoom_factor);
            set_config(config);
            render();
        }
    };

    view! {
        <main>
            <header>
                <h1>"rsfractal"</h1>
                <button on:click=move |_| render() disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "Rendering..." } else { "Render" }}
                </button>
                <select on:change=move |ev| {
                    let value = ev.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
                    let mut config = config();
                    config.coloring(Coloring::from_str(&value).unwrap());
                    set_config(config);
                }>
                    {Coloring::iter()
                        .map(|coloring| {
                            view! { <option>{coloring.to_string()}</option> }
                        })
                        .collect_view()}
                </select>
                <button on:click=move |_| {
                    set_config(Config::default());
                    render();
                }>"Reset"</button>
            </header>
            <canvas
                width=config().width
                height=config().height
                node_ref=canvas_ref
                on:click=on_click
            />
        </main>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
