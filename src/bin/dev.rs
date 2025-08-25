use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");
    let element = document.create_element("canvas").expect("Unable to create canvas");
    let canvas = element.dyn_into::<HtmlCanvasElement>().expect("Invalid element");

    let mut context = canvas
        .get_context("webgl2")
        .expect("Unable to get WebGL2 context")
        .expect("WebGL contet not found")
        .dyn_into::<WebGl2RenderingContext>()
        .expect("Failed to cast to WebGl2RenderingContext");
}
