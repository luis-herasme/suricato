use suricato::{geometry::Geometry, material::Material, mesh::Mesh, renderer::Renderer, utils::request_animation_frame};

const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

out vec4 fragment_color;

void main() {
    fragment_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

fn main() {
    let mut renderer = Renderer::new();

    let material = Material::new(&renderer, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::quad(&renderer).unwrap();
    let mut mesh = Mesh::new(&renderer, geometry, material).unwrap();

    request_animation_frame(Box::new(move || {
        renderer.clear();
        renderer.render(&mut mesh);
    }));
}
