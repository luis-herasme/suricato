use suricato::{
    geometry::Geometry,
    material::Material,
    mesh::Mesh,
    renderer::Renderer,
    texture::{Texture, TextureData},
    uniforms::Uniform,
    utils::*,
};
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(main_async());
}

const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
in vec3 position;
in vec2 uv;

out vec2 v_texture_coordinate;

void main() {
    v_texture_coordinate = uv;
    gl_Position = vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

in vec2 v_texture_coordinate;

out vec4 fragment_color;
uniform sampler2D t1;

void main() {
    fragment_color = texture(t1, v_texture_coordinate);
}
"#;

async fn main_async() {
    let mut renderer = Renderer::new();

    let material = Material::new(&renderer.gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::quad(&renderer.gl).unwrap();
    let mut mesh = Mesh::new(&renderer.gl, geometry, material).unwrap();

    let html_image = fetch_image("./bob.png").await.unwrap();
    let texture = Texture::new(&renderer.gl, TextureData::HtmlImageElement(html_image)).unwrap();
    mesh.material.set_uniform("t1", Uniform::Texture(texture));

    request_animation_frame(Box::new(move || {
        renderer.clear();
        renderer.render(&mut mesh);
    }));
}
