#![allow(dead_code)]
#![allow(unused)]

use glam::Quat;
use gltf::Gltf;
use suricato::{
    animation::Animation,
    buffer_gpu::{BufferGPU, BufferKind, BufferUsage},
    camera::PerspectiveCamera,
    geometry::{self, Geometry},
    index_buffer::IndexBuffer,
    material::Material,
    mesh::{Mesh, RenderPrimitive},
    renderer::Renderer,
    uniforms::Uniform,
    utils::{fetch_bytes, request_animation_frame, to_bytes},
    vertex_buffer::*,
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

void main() {
    gl_Position = projection_matrix * camera_inverse_matrix * transform * vec4(position, 1.0);
    gl_PointSize = 2.0;
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

out vec4 fragment_color;
uniform vec4 color;

void main() {
    fragment_color = color;
}
"#;

async fn main_async() {
    let mut t: f32 = 0.0;
    let mut renderer = Renderer::new();
    let material = Material::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

    let data = fetch_bytes("./fox.glb").await.unwrap();
    let gltf = Gltf::from_slice(&data).unwrap();

    let mut blob: Vec<u8> = Vec::new();

    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Bin => {
                if let Some(b) = gltf.blob.as_ref() {
                    blob = b.clone();
                } else {
                    panic!("Binary buffer expected but not found");
                }
            }
            gltf::buffer::Source::Uri(uri) => {
                panic!("External buffer not supported: {}", uri);
            }
        }
    }

    let gltf_mesh = &gltf.meshes().next().unwrap();
    let primitive = gltf_mesh.primitives().next().unwrap();
    let reader = primitive.reader(|_| Some(&blob));

    let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();

    let mut indices: Vec<u32> = Vec::new();
    for i in (0..positions.len()).step_by(3) {
        // A -> B
        indices.push(i as u32);
        indices.push(i as u32 + 1);

        // B -> C
        indices.push(i as u32 + 1);
        indices.push(i as u32 + 2);

        // C -> A
        indices.push(i as u32 + 2);
        indices.push(i as u32);
    }

    let index_buffer = IndexBuffer::from_u32(BufferUsage::StaticDraw, indices);

    let geometry = Geometry {
        vertex_count:               positions.len(),
        instance_count:             None,
        indices:                    Some(index_buffer),
        vertex_buffers:             vec![VertexBuffer::new("position", positions)],
        interleaved_vertex_buffers: vec![],
    };
    let mut mesh = Mesh::new(geometry, material);
    mesh.transform.scale *= 0.075;
    mesh.transform.translation.z = -20.0;
    mesh.transform.translation.y = -3.5;
    mesh.render_primitive = RenderPrimitive::Lines;

    // Skeleton
    let mut animation = Animation::from(gltf);
    animation.update_global_transform();
    let lines = animation.get_lines();
    let vertex_buffer = VertexBuffer::new("position", lines);
    let geometry = Geometry::from(vertex_buffer);
    let material = Material::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let mut skeleton_mesh = Mesh::new(geometry, material);
    skeleton_mesh.transform = mesh.transform.clone();
    skeleton_mesh.render_primitive = RenderPrimitive::Lines;
    // end skeleton

    let mut scene = vec![mesh, skeleton_mesh];
    let mut camera = PerspectiveCamera::default();

    request_animation_frame(Box::new(move || {
        let mesh = &mut scene[0];
        mesh.transform.rotation *= Quat::from_rotation_y(0.01);
        mesh.material.set_uniform("transform", Uniform::from(&mesh.transform));
        mesh.material.set_uniform("color", Uniform::Vec4([0.5, 0.5, 0.5, 1.0]));

        let skeleton = &mut scene[1];
        skeleton.transform.rotation *= Quat::from_rotation_y(0.01);
        skeleton.material.set_uniform("transform", Uniform::from(&skeleton.transform));
        skeleton.material.set_uniform("color", Uniform::Vec4([0.25, 1.0, 0.25, 1.0]));

        renderer.render_scene(&mut scene, &mut camera);
    }));
}
