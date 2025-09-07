use glam::Quat;
use gltf::Gltf;
use suricato::{
    buffer_gpu::BufferUsage,
    geometry::Geometry,
    index_buffer::IndexBuffer,
    material::Material,
    mesh::Mesh,
    renderer::Renderer,
    transform::Transform3D,
    uniforms::Uniform,
    utils::*,
    vertex_buffer::{Data, InterleavedVertexBuffer, VertexData},
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
uniform mat4 transform;

void main() {
    v_normal = normal;
    gl_Position = transform * vec4(position, 10.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;

in vec3 v_normal;
out vec4 fragment_color;

void main() {
    vec3 normal = normalize(v_normal);
    float light = dot(normal, vec3(0.25, 25.0, -25.0));
    fragment_color = vec4(1.0, 0.0, 0.0, 1.0);
    fragment_color.rgb *= max(0.1, light);
}
"#;

async fn main_async() {
    let data = fetch_bytes("./test.glb").await.unwrap();
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
    let normals: Vec<[f32; 3]> = reader.read_normals().unwrap().collect();
    let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

    let positions = VertexData {
        name:      String::from("position"),
        data:      Data::Vec3(positions),
        divisor:   0,
        normalize: false,
    };

    let normals = VertexData {
        name:      String::from("normal"),
        data:      Data::Vec3(normals),
        divisor:   0,
        normalize: false,
    };

    let mut renderer = Renderer::new();
    let material = Material::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let index_buffer = IndexBuffer::from_u32(BufferUsage::StaticDraw, indices);

    let geometry = Geometry {
        vertex_count:               0,
        instance_count:             None,
        indices:                    Some(index_buffer),
        vertex_buffers:             vec![],
        interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(BufferUsage::StaticDraw, vec![positions, normals])],
    };

    let mut mesh = Mesh::new(geometry, material);

    let mut transform = Transform3D::new();
    transform.scale *= 0.25;
    transform.translation.y = -0.5;

    request_animation_frame(Box::new(move || {
        renderer.clear();
        transform.rotation *= Quat::from_rotation_x(0.003);
        transform.rotation *= Quat::from_rotation_y(0.002);
        mesh.material.set_uniform("transform", Uniform::Mat4(transform.to_array()));
        renderer.render(&mut mesh)
    }));
}
