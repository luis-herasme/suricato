use suricato::{attributes::Attribute, program::Program, uniforms::Uniform};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");
    let element = document.create_element("canvas").expect("Unable to create canvas");
    let canvas = element.dyn_into::<HtmlCanvasElement>().expect("Invalid element");

    let gl = canvas
        .get_context("webgl2")
        .expect("Unable to get WebGL2 context")
        .expect("WebGL contet not found")
        .dyn_into::<WebGl2RenderingContext>()
        .expect("Failed to cast to WebGl2RenderingContext");
    let body = document.body().unwrap();
    body.append_child(&canvas).unwrap();

    let vertex_shader_source = r#"#version 300 es
in vec2 position;
in float size;

void main() {
    gl_PointSize = size;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;
    let fragment_shader_source = r#"#version 300 es
precision mediump float;

uniform vec4 color;
out vec4 fragment_color;

void main() {
    fragment_color = color;
}
"#;

    let program = Program::new(&gl, vertex_shader_source, fragment_shader_source).unwrap();
    gl.use_program(Some(&program.webgl_program));

    // Uniforms
    program.set_uniform("color", &Uniform::Vec4(0.0, 0.0, 1.0, 1.0));

    // Attributes
    program.set_attribute("position", &Attribute::vec2(&gl, vec![0.0, 0.0, 0.5, 0.0]));
    program.set_attribute("size", &Attribute::float(&gl, vec![10.0, 50.0]));

    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, 2);
}
