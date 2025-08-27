use glam::Quat;
use suricato::{geometry::Geometry, material::Material, renderer::Renderer, transform::Transform, uniforms::Uniform};
use wasm_bindgen::{JsCast, prelude::Closure};

fn main() {
    console_error_panic_hook::set_once();

    let vertex_shader_source = r#"#version 300 es
in vec2 position;
in vec3 color;

uniform mat4 transform;
out vec3 v_color;

void main() {
    v_color = color;
    gl_Position = transform * vec4(position, 0.0, 1.0);
}
"#;
    let fragment_shader_source = r#"#version 300 es
precision mediump float;

// uniform vec4 color;
out vec4 fragment_color;
in vec3 v_color;

void main() {
    fragment_color = vec4(v_color, 1.0);
}
"#;

    let mut material = Material::new(vertex_shader_source, fragment_shader_source);

    let geometry = Geometry::quad();
    let mut renderer = Renderer::new();

    let mut transform1 = Transform::new();
    let rotation1 = Quat::from_rotation_z(0.01);

    let mut transform2 = Transform::new();
    let rotation2 = Quat::from_rotation_z(0.02);

    let callback = Closure::wrap(Box::new(move || {
        renderer.clear();

        transform1.rotation = transform1.rotation.mul_quat(rotation1);
        material.set_uniform("transform", Uniform::Mat4(transform1.to_array()));
        renderer.render(&material, &geometry);

        transform2.rotation = transform2.rotation.mul_quat(rotation2);
        material.set_uniform("transform", Uniform::Mat4(transform2.to_array()));
        renderer.render(&material, &geometry);
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1)
        .unwrap();

    callback.forget();
}
