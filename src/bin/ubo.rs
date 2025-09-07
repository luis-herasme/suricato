use suricato::{
    geometry::Geometry,
    material::Material,
    mesh::Mesh,
    renderer::Renderer,
    ubo::UniformBufferObject,
    uniforms::Uniform,
    utils::{request_animation_frame, to_bytes},
};
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(main_async());
}

const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
layout(std140) uniform Colors {
    vec4 colors[3];
};

uniform vec2 translation;
uniform uint color_index;
in vec2 position;
out vec4 v_color;

void main() {
    v_color = colors[color_index];
    gl_Position = vec4(position + translation, 0.0, 2.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 fragment_color;

void main() {
    fragment_color = v_color;
}
"#;

async fn main_async() {
    let mut renderer = Renderer::new();
    let material = Material::new(&renderer, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::quad(&renderer).unwrap();
    let mut mesh = Mesh::new(&renderer, geometry, material).unwrap();

    let ubo_binding_point = 1;

    // UBO #1
    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0, // colors[0]
        0.0, 1.0, 0.0, 1.0, // colors[1]
        0.0, 0.0, 1.0, 1.0, // colors[2]
    ];
    let mut ubo = UniformBufferObject::new(&renderer, to_bytes(&colors)).unwrap();

    // UBO #2
    let colors2: Vec<f32> = vec![
        1.0, 0.5, 0.5, 1.0, // colors[0]
        0.5, 1.0, 0.5, 1.0, // colors[1]
        0.5, 0.5, 1.0, 1.0, // colors[2]
    ];
    let mut ubo2 = UniformBufferObject::new(&renderer, to_bytes(&colors2)).unwrap();

    mesh.material.set_uniform_block("Colors", ubo_binding_point);

    request_animation_frame(Box::new(move || {
        renderer.clear();

        ubo.set_binding_point(ubo_binding_point);
        mesh.material.set_uniform("translation", Uniform::Vec2([-0.25, -0.25]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(0));
        renderer.render(&mut mesh);

        mesh.material.set_uniform("translation", Uniform::Vec2([0.0, 0.0]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(1));
        renderer.render(&mut mesh);

        mesh.material.set_uniform("translation", Uniform::Vec2([0.25, 0.25]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(2));
        renderer.render(&mut mesh);

        ubo2.set_binding_point(ubo_binding_point);
        mesh.material.set_uniform("translation", Uniform::Vec2([-0.25, 0.75]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(0));
        renderer.render(&mut mesh);

        mesh.material.set_uniform("translation", Uniform::Vec2([0.0, 1.0]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(1));
        renderer.render(&mut mesh);

        mesh.material.set_uniform("translation", Uniform::Vec2([0.25, 1.25]));
        mesh.material.set_uniform("color_index", Uniform::UnsignedInt(2));
        renderer.render(&mut mesh);
    }));
}
