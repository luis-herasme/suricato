use suricato::{geometry::Geometry, material::Material, renderer::Renderer, uniforms::Uniform};

fn main() {
    console_error_panic_hook::set_once();

    let vertex_shader_source = r#"#version 300 es
in vec2 position;

void main() {
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

    let mut material = Material::new(vertex_shader_source, fragment_shader_source);
    material.set_uniform("color", Uniform::Vec4(0.0, 1.0, 0.0, 1.0));

    let geometry = Geometry::quad();
    let mut renderer = Renderer::new();

    renderer.render(&material, &geometry);
}
