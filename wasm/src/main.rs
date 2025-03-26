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
use web_sys::ImageData;
use web_sys::MouseEvent;

#[derive(Serialize)]
struct ContextAttributes {
    alpha: bool,
    desynchronized: bool,
}

#[component]
fn App() -> impl IntoView {
    let (mandelbrot, set_mandelbrot) = signal(Mandelbrot::default());

    let action = Action::new(|mandelbrot: &Mandelbrot| {
        let mandelbrot = mandelbrot.clone();
        let (sender, receiver) = oneshot::channel::<Arc<Vec<u8>>>();
        async move {
            rayon::spawn(move || {
                let size = mandelbrot.width() * mandelbrot.height() * 4;
                let mut pixels = vec![0u8; size];
                mandelbrot.render(&mut pixels);
                let pixels = Arc::new(pixels);
                sender.send(pixels).unwrap();
            });
            receiver.await.unwrap()
        }
    });

    let canvas_ref = NodeRef::<Canvas>::new();

    let render = move || {
        action.dispatch(mandelbrot.get());
    };

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            if let Some(pixels) = action.value().get() {
                let width = canvas.width();
                let height = canvas.height();
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
            let element: Element = event.target().unwrap().dyn_into().unwrap();
            let rect = element.get_bounding_client_rect();
            let scale_x = canvas.width() as f64 / rect.width();
            let scale_y = canvas.height() as f64 / rect.height();
            let x = (event.client_x() as f64 - rect.left()) * scale_x;
            let y = (event.client_y() as f64 - rect.top()) * scale_y;
            set_mandelbrot.update(|mandelbrot| {
                let zoom_factor = if event.shift_key() { 1.0 / 0.25 } else { 0.25 };
                mandelbrot.zoom(x, y, zoom_factor);
            });
            render();
        }
    };

    view! {
        <main class="size-full">
            <canvas
                class="absolute w-full top-1/2 -translate-y-1/2"
                width=move || mandelbrot.read().width()
                height=move || mandelbrot.read().height()
                node_ref=canvas_ref
                on:click=on_click
            />
            <header class="absolute left-0 top-0 m-4 p-4 rounded-lg bg-slate-700 opacity-50 z-1">
                <h1 class="text-2xl">"rsfractal"</h1>
                <hr class="my-2" />
                <h2 class="text-base">"click to zoom in, shift-click to zoom out"</h2>
                <hr class="my-2" />
                <Button on:click=move |_| render() prop:disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "Rendering..." } else { "Render" }}
                </Button>
                <Button
                    on:click=move |_| {
                        *set_mandelbrot.write() = Mandelbrot::default();
                        render();
                    }
                    prop:disabled=move || action.pending().get()
                >
                    "Reset"
                </Button>
                <hr class="my-2" />
                <label class="text-base" for="resolution">
                    "Resolution:"
                </label>
                <Select
                    attr:id="resolution"
                    on:change=move |ev| {
                        let value = event_target_value(&ev).parse().unwrap();
                        set_mandelbrot.update(|mandelbrot| mandelbrot.selected_resolution = value);
                        render();
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || { mandelbrot.read().selected_resolution.to_string() }
                >
                    {mandelbrot
                        .read()
                        .resolutions()
                        .iter()
                        .enumerate()
                        .map(|(index, (width, height))| {
                            view! {
                                <option
                                    value=index.to_string()
                                    selected=index == mandelbrot.read().selected_resolution
                                >
                                    {*width}
                                    "x"
                                    {*height}
                                </option>
                            }
                        })
                        .collect_view()}
                </Select>
                <br />
                <label class="text-base" for="rendering">
                    "Rendering:"
                </label>
                <Select
                    attr:id="rendering"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_mandelbrot
                            .update(|mandelbrot| {
                                mandelbrot.rendering = Rendering::from_str(&value).unwrap();
                            });
                        render()
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || mandelbrot.read().rendering.to_string()
                >
                    {Rendering::iter()
                        .map(|rendering| {
                            view! {
                                <option
                                    value=rendering.to_string()
                                    selected=move || mandelbrot.read().rendering == rendering
                                >
                                    {rendering.to_string()}
                                </option>
                            }
                        })
                        .collect_view()}
                </Select>
                <br />
                <label class="text-base" for="bailout">
                    "Bailout:"
                </label>
                <Input
                    attr:id="bailout"
                    attr:r#type="number"
                    attr:min=2
                    on:change=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse() {
                            set_mandelbrot
                                .update(|mandelbrot| {
                                    mandelbrot.bailout = value;
                                });
                            render()
                        }
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || mandelbrot.read().bailout
                />
                <br />
                <label class="text-base" for="max_iterations">
                    "Max Iterations:"
                </label>
                <Input
                    attr:id="max_iterations"
                    attr:r#type="number"
                    attr:min=1
                    on:change=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse() {
                            set_mandelbrot
                                .update(|mandelbrot| {
                                    mandelbrot.max_iterations = value;
                                });
                            render()
                        }
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || mandelbrot.read().max_iterations
                />
                <hr class="my-2" />
                <label class="text-base" for="coloring">
                    "Coloring:"
                </label>
                <Select
                    attr:id="coloring"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_mandelbrot
                            .update(|mandelbrot| {
                                mandelbrot.coloring = Coloring::from_str(&value).unwrap();
                            });
                        render()
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || mandelbrot.read().coloring.to_string()
                >
                    {Coloring::iter()
                        .map(|coloring| {
                            view! {
                                <option
                                    value=coloring.to_string()
                                    selected=move || mandelbrot.read().coloring == coloring
                                >
                                    {coloring.to_string()}
                                </option>
                            }
                        })
                        .collect_view()}
                </Select>
                <Show when=move || mandelbrot.read().coloring == Coloring::Palette>
                    <Select
                        attr:id="palette"
                        on:change=move |ev| {
                            let value = event_target_value(&ev).parse().unwrap();
                            set_mandelbrot.update(|mandelbrot| mandelbrot.selected_palette = value);
                            render();
                        }
                        prop:disabled=move || action.pending().get()
                        prop:value=move || { mandelbrot.read().selected_palette.to_string() }
                    >
                        {mandelbrot
                            .read()
                            .palettes()
                            .iter()
                            .enumerate()
                            .map(|(index, (name, _))| {
                                view! {
                                    <option
                                        value=index.to_string()
                                        selected=move || mandelbrot.read().selected_palette == index
                                    >
                                        {name.clone()}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </Select>
                </Show>
                <br />
                <label class="text-base" for="exponent">
                    "Exponent:"
                </label>
                <Input
                    attr:id="exponent"
                    attr:r#type="number"
                    attr:step="0.01"
                    on:change=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse() {
                            set_mandelbrot
                                .update(|mandelbrot| {
                                    mandelbrot.exponent = value;
                                });
                            render();
                        }
                    }
                    prop:disabled=move || action.pending().get()
                    prop:value=move || mandelbrot.read().exponent
                />
            </header>
        </main>
    }
}

#[component]
fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="my-1 mr-2 px-2 h-8 hover:bg-white hover:text-black rounded-md border border-white">
            {children()}
        </button>
    }
}

#[component]
fn Input() -> impl IntoView {
    view! {
        <input class="my-1 ml-2 px-2 h-8 hover:bg-white hover:text-black rounded-md border border-white" />
    }
}

#[component]
fn Select(children: Children) -> impl IntoView {
    view! {
        <select class="my-1 ml-2 px-2 h-8 hover:bg-white hover:text-black rounded-md border border-white">
            {children()}
        </select>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
