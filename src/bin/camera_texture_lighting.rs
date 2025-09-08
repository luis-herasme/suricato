use glam::Quat;
use suricato::{
    camera::PerspectiveCamera, geometry::Geometry, material::Material, mesh::Mesh, renderer::Renderer, texture::Texture, uniforms::Uniform,
    utils::request_animation_frame,
};
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(main_async());
}

const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
in vec3 position;
in vec3 normal;

uniform mat4 projection_matrix;
uniform mat4 camera_inverse_matrix;
uniform mat4 transform;

in vec2 uv;
out vec3 v_normal;
out vec2 v_texture_coordinate;

void main() {
    v_texture_coordinate = uv;
    v_normal = mat3(camera_inverse_matrix * transform) * normal;
    gl_Position = projection_matrix * camera_inverse_matrix * transform * vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

in vec3 v_normal;
in vec2 v_texture_coordinate;

out vec4 fragment_color;
uniform sampler2D texture_sampler;

void main() {
    vec3 normal = normalize(v_normal);
    float light = dot(normal, normalize(vec3(0.25, 25.0, 25.0)));
    fragment_color = texture(texture_sampler, v_texture_coordinate);
    fragment_color.rgb *= max(0.2, light);
}
"#;

async fn main_async() {
    let mut renderer = Renderer::new();

    let material = Material::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let geometry = Geometry::box_geometry();
    let mut mesh = Mesh::new(geometry, material);
    mesh.transform.translation.z = -5.0;

    let texture = Texture::from_image_url("./bob.png").await.unwrap();
    mesh.material.set_uniform("texture_sampler", Uniform::Texture(texture));

    let mut scene = vec![mesh];
    let mut camera = PerspectiveCamera::default();

    request_animation_frame(Box::new(move || {
        let mesh = &mut scene[0];
        mesh.transform.rotation *= Quat::from_rotation_y(0.01);
        mesh.transform.rotation *= Quat::from_rotation_z(0.005);
        renderer.render_scene(&mut scene, &mut camera);
    }));
}
