use glam::Quat;
use suricato::{geometry::Geometry, material::Material, renderer::Renderer, transform::Transform, uniforms::Uniform};
use wasm_bindgen::{JsCast, prelude::Closure};

fn main() {
    console_error_panic_hook::set_once();

    let vertex_shader_source = r#"#version 300 es
in vec2 position;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 0.0, 1.0);
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
    material.set_uniform("color", Uniform::Vec4([0.0, 1.0, 0.0, 1.0]));

    let mut transform = Transform::new();
    let geometry = Geometry::quad();
    let mut renderer = Renderer::new();

    let rotation = Quat::from_rotation_z(0.01);

    let callback = Closure::wrap(Box::new(move || {
        transform.rotation = transform.rotation.mul_quat(rotation);
        material.set_uniform("transform", Uniform::Mat4(transform.to_array()));

        renderer.render(&material, &geometry);
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1)
        .unwrap();

    callback.forget();
}
