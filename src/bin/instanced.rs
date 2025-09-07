use suricato::{geometry::Geometry, material::Material, mesh::Mesh, renderer::Renderer, transform::Transform2D, utils::*};
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(main_async());
}

const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
in vec2 position;
in mat3 transform;

void main() {
    gl_Position = vec4((transform * vec3(position, 1.0)).xy, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

out vec4 fragment_color;

void main() {
    fragment_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

async fn main_async() {
    let mut renderer = Renderer::new();

    let size = 10;

    let material = Material::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let geometry = Geometry::quad_instanced(size * size);
    let mut mesh = Mesh::new(geometry, material);

    let mut transforms = Vec::new();

    for x in 0..size {
        for y in 0..size {
            let mut transform = Transform2D::new();
            transform.scale *= 0.05;
            transform.translation.x = (x as f32 + 0.5 - size as f32 / 2.0) / size as f32;
            transform.translation.y = (y as f32 + 0.5 - size as f32 / 2.0) / size as f32;
            transforms.push(transform);
        }
    }

    request_animation_frame(Box::new(move || {
        renderer.clear();

        let transform_buffer = mesh.geometry.get_vertex_buffer("transform").unwrap();

        for (vertex_index, transform) in transforms.iter_mut().enumerate() {
            transform.rotation += vertex_index as f32 * 0.001;
            transform_buffer.set_vertex(vertex_index, &transform.to_array());
        }

        renderer.render(&mut mesh);
    }));
}
