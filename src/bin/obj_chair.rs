use glam::Quat;
use suricato::{
    geometry::Geometry,
    material::Material,
    mesh::Mesh,
    obj_parser::OBJ,
    renderer::Renderer,
    texture::{Texture, TextureData},
    transform::Transform3D,
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
in vec3 normal;
in vec2 uv;

out vec3 v_normal;
out vec2 v_texture_coordinate;

uniform mat4 transform;

void main() {
    v_normal = (transform * vec4(normal, 0.0)).xyz;
    v_texture_coordinate = uv;
    gl_Position = transform * vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

in vec3 v_normal;
in vec2 v_texture_coordinate;

out vec4 fragment_color;
uniform sampler2D chair_texture;

void main() {
    vec3 normal = normalize(v_normal);
    float light = dot(normal, normalize(vec3(0.25, 25.0, -25.0)));
    fragment_color = texture(chair_texture, v_texture_coordinate);
    fragment_color.rgb *= max(0.2, light);
}
"#;

async fn main_async() {
    let obj_data = fetch_text("./chair.obj").await.unwrap();
    let obj = OBJ::try_from(obj_data).unwrap();

    let mut renderer = Renderer::new();
    let material = Material::new(&renderer, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::from_obj(&renderer, obj).unwrap();
    let mut mesh = Mesh::new(&renderer, geometry, material).unwrap();

    let html_image = fetch_image("./chair.png").await.unwrap();
    let texture = Texture::new(&renderer, TextureData::HtmlImageElement(html_image)).unwrap();
    mesh.material.set_uniform("chair_texture", Uniform::Texture(texture));

    let mut transform = Transform3D::new();
    transform.scale *= 0.005;

    request_animation_frame(Box::new(move || {
        renderer.clear();
        transform.rotation *= Quat::from_rotation_x(0.01);
        transform.rotation *= Quat::from_rotation_y(0.02);
        mesh.material.set_uniform("transform", Uniform::Mat4(transform.to_array()));
        renderer.render(&mut mesh);
    }));
}
