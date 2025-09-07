//     let vertex_shader_source = r#"#version 300 es
// in vec3 position;
// in vec3 normal;
// in vec2 uv;

// out vec3 v_normal;
// uniform mat4 transform;

// void main() {
//     v_normal = normal;
//     gl_Position = transform * vec4(position, 10.0);
// }
// "#;
//     let fragment_shader_source = r#"#version 300 es
// precision mediump float;

// in vec3 v_normal;
// out vec4 fragment_color;

// void main() {
//     vec3 normal = normalize(v_normal);
//     float light = dot(normal, vec3(0.25, 25.0, -25.0));
//     fragment_color = vec4(1.0, 0.0, 0.0, 1.0);
//     fragment_color.rgb *= max(0.1, light);
// }
// "#;

//     let data = fetch_bytes("./test.glb").await.unwrap();
//     let gltf = Gltf::from_slice(&data).unwrap();

//     let mut buffers: Vec<Vec<u8>> = Vec::new();

//     for buffer in gltf.buffers() {
//         match buffer.source() {
//             gltf::buffer::Source::Bin => {
//                 if let Some(blob) = gltf.blob.as_ref() {
//                     buffers.push(blob.clone());
//                 } else {
//                     panic!("Binary buffer expected but not found");
//                 }
//             }
//             gltf::buffer::Source::Uri(uri) => {
//                 panic!("External buffer not supported: {}", uri);
//             }
//         }
//     }

//     let gltf_mesh = &gltf.meshes().next().unwrap();
//     let primitive = gltf_mesh.primitives().next().unwrap();
//     let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

//     let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
//     let normals: Vec<[f32; 3]> = reader.read_normals().unwrap().collect();
//     let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

//     let positions = VertexData {
//         name:      String::from("position"),
//         data:      Data::Vec3(positions),
//         divisor:   0,
//         normalize: false,
//     };

//     let normals = VertexData {
//         name:      String::from("normal"),
//         data:      Data::Vec3(normals),
//         divisor:   0,
//         normalize: false,
//     };

//     let index_buffer = IndexBuffer::from(indices);

//     let mut app = App::new();
//     let material = Material::new(vertex_shader_source, fragment_shader_source);

//     let geometry = Geometry {
//         vertex_count:               0,
//         instance_count:             None,
//         indices:                    Some(index_buffer),
//         vertex_buffers:             vec![],
//         interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(vec![positions, normals])],
//     };

//     let mut mesh = Mesh::new(geometry, material);

//     let mut transform = Transform3D::new();
//     transform.scale *= 0.25;
//     transform.translation.y = -0.5;

//     let main_loop = Rc::new(RefCell::new(None));
//     let main_loop_clone = main_loop.clone();

//     *main_loop_clone.borrow_mut() = Some(Closure::new(move || {
//         app.clear();
//         transform.rotation *= Quat::from_rotation_x(0.0003);
//         transform.rotation *= Quat::from_rotation_y(0.002);
//         mesh.material.set_uniform("transform", Uniform::Mat4(transform.to_array()));
//         app.render(&mut mesh).unwrap();
//         request_animation_frame(main_loop.borrow().as_ref().unwrap());
//     }));

//     request_animation_frame(main_loop_clone.borrow().as_ref().unwrap());
// }
// }

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

    let material = Material::new(&renderer, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::quad_instanced(&renderer, size * size).unwrap();
    let mut mesh = Mesh::new(&renderer, geometry, material).unwrap();

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
