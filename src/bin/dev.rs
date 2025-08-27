use glam::{Mat4, Quat, Vec3};
use suricato::{geometry::Geometry, material::Material, renderer::Renderer, uniforms::Uniform};
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

    let geometry = Geometry::quad();
    let mut renderer = Renderer::new();
    let mut t = 0.0;

    let callback = Closure::wrap(Box::new(move || {
        let scale = Vec3::new(1.0, 0.5, 1.0);
        let rotation = Quat::from_rotation_z(t);
        let translation = Vec3::new(0.25, 0.0, 0.0);
        let transform_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let data = transform_matrix.to_cols_array();
        material.set_uniform("transform", Uniform::Mat4(data));
        t += 0.01;

        renderer.render(&material, &geometry);
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1)
        .unwrap();

    callback.forget();
}
