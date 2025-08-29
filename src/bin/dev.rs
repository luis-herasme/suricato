use std::{rc::Rc, sync::Mutex};

use glam::Quat;
use suricato::{geometry::Geometry, material::Material, renderer::Renderer, transform::Transform};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::console;

fn main() {
    console_error_panic_hook::set_once();

    let vertex_shader_source = r#"#version 300 es
in vec2 position;
in vec3 color;

in mat4 transform;
out vec4 v_color;

void main() {
    v_color = vec4(color, 1.0);
    gl_Position = transform * vec4(position, 0.0, 1.0);
}
"#;
    let fragment_shader_source = r#"#version 300 es
precision mediump float;

out vec4 fragment_color;
in vec4 v_color;

void main() {
    fragment_color = v_color;
}
"#;

    let mut renderer = Renderer::new();

    let size = 512;

    let mut material = Material::new(vertex_shader_source, fragment_shader_source);
    let mut geometry = Geometry::instance_quad(size * size);
    let mut transforms = Vec::new();

    for x in 0..size {
        for y in 0..size {
            let mut transform = Transform::new();
            transform.scale *= 0.005;
            transform.translation.x = 1.85 * (x as f32 - size as f32 / 2.0) / size as f32;
            transform.translation.y = 1.85 * (y as f32 - size as f32 / 2.0) / size as f32;
            transforms.push(transform);
        }
    }

    let frames_rendered = Rc::new(Mutex::new(1));
    let frames_rendered_cloned = Rc::clone(&frames_rendered);

    let render_loop = Closure::wrap(Box::new(move || {
        let mut counter = frames_rendered_cloned.lock().unwrap();
        *counter += 1;

        renderer.clear();

        for (i, transform) in transforms.iter_mut().enumerate() {
            transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_z(i as f32 * 0.0001));
            geometry.set_vertex_at_mat4("transform", i, transform.to_array());
        }

        renderer.render(&mut material, &mut geometry);
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(render_loop.as_ref().unchecked_ref(), 1)
        .unwrap();

    render_loop.forget();

    let frames_rendered_cloned = Rc::clone(&frames_rendered);

    let fps_loop = Closure::wrap(Box::new(move || {
        let mut counter = frames_rendered_cloned.lock().unwrap();
        console::log_1(&format!("Frames rendered the last second: {}", counter).into());
        *counter = 0;
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(fps_loop.as_ref().unchecked_ref(), 1000)
        .unwrap();

    fps_loop.forget();
}

// use glam::Quat;
// use suricato::{geometry::Geometry, material::Material, renderer::Renderer, transform::Transform, uniforms::Uniform};
// use wasm_bindgen::{JsCast, prelude::Closure};

// fn main() {
//     console_error_panic_hook::set_once();

//     let vertex_shader_source = r#"#version 300 es
// in vec2 position;
// in vec3 color;

// uniform mat4 transform;
// out vec3 v_color;

// void main() {
//     v_color = color;
//     gl_Position = transform * vec4(position, 0.0, 1.0);
// }
// "#;
//     let fragment_shader_source = r#"#version 300 es
// precision mediump float;

// // uniform vec4 color;
// out vec4 fragment_color;
// in vec3 v_color;

// void main() {
//     fragment_color = vec4(v_color, 1.0);
// }
// "#;

//     let mut material = Material::new(vertex_shader_source, fragment_shader_source);

//     let mut renderer = Renderer::new();

//     let mut transform1 = Transform::new();
//     transform1.scale *= 0.2;
//     let rotation1 = Quat::from_rotation_z(0.01);

//     let mut transform2 = Transform::new();
//     transform2.scale *= 0.2;
//     let rotation2 = Quat::from_rotation_z(0.02);

//     let mut geometry = Geometry::quad();
//     let mut t = 0.1;

//     let callback = Closure::wrap(Box::new(move || {
//         t += 0.01;
//         geometry.set_vertex_at_f32("position", 0, f32::sin(t));

//         renderer.clear();
//         transform1.rotation = transform1.rotation.mul_quat(rotation1);
//         material.set_uniform("transform", Uniform::Mat4(transform1.to_array()));
//         renderer.render(&mut material, &mut geometry);
//         transform2.rotation = transform2.rotation.mul_quat(rotation2);
//         material.set_uniform("transform", Uniform::Mat4(transform2.to_array()));
//         renderer.render(&mut material, &mut geometry);
//     }) as Box<dyn FnMut()>);

//     web_sys::window()
//         .unwrap()
//         .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1)
//         .unwrap();

//     callback.forget();
// }
