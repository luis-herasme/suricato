use suricato::{attributes::AttributeBuffer, index_buffer::IndexBuffer, material::Material, uniforms::Uniform};
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
// in float size;

void main() {
    // gl_PointSize = size;
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

    let material = Material::new(&gl, vertex_shader_source, fragment_shader_source).unwrap();
    gl.use_program(Some(&material.program));

    // Uniforms
    material.set_uniform("color", &Uniform::Vec4(0.0, 0.0, 1.0, 1.0));

    // Attributes
    #[rustfmt::skip]
    material.set_attribute(
        "position",
        &AttributeBuffer::vec2(
            &gl,
            vec![
                0.5,   0.5, // Top right
                0.5,  -0.5, // Bottom right
                -0.5, -0.5, // Bottom left
                -0.5,  0.5, // Top left
            ],
        ),
    );

    let index_buffer = IndexBuffer::u8(
        &gl,
        vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ],
    );

    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer.buffer));
    gl.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES,
        index_buffer.count as i32,
        index_buffer.kind,
        index_buffer.offset,
    );

    // program.set_attribute("size", &AttributeBuffer::float(&gl, vec![10.0, 50.0]));
    // program.set_attributes(&InterleavedAttributeBuffer::new(
    //     &gl,
    //     vec![
    //         AttributeData::Vec2 {
    //             name: String::from("position"),
    //             data: vec![(0.0, 0.0), (0.5, 0.0)],
    //         },
    //         AttributeData::Float {
    //             name: String::from("size"),
    //             data: vec![10.0, 50.0],
    //         },
    //     ],
    // ));

    // gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, 2);
}
