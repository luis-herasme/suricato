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

// fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     web_sys::window()
//         .unwrap()
//         .request_animation_frame(f.as_ref().unchecked_ref())
//         .unwrap();
// }

// use glam::Quat;
// use suricato::{
//     obj_parser::OBJ,
//     renderer::Renderer,
//     transform::Transform3D,
//     uniforms::Uniform,
//     utils::{fetch_image, fetch_text, request_animation_frame},
// };
// use wasm_bindgen_futures::spawn_local;

// fn main() {
//     console_error_panic_hook::set_once();
//     spawn_local(main_async());
// }

// async fn main_async() {
//     console_error_panic_hook::set_once();

//     let vertex_shader_source = r#"#version 300 es
// in vec3 position;
// in vec3 normal;
// in vec2 uv;

// out vec3 v_normal;
// out vec2 v_texture_coordinate;

// uniform mat4 transform;

// void main() {
//     v_normal = (transform * vec4(normal, 0.0)).xyz;
//     v_texture_coordinate = uv;
//     gl_Position = transform * vec4(position, 1.0);
// }
// "#;
//     let fragment_shader_source = r#"#version 300 es
// precision mediump float;

// in vec3 v_normal;
// in vec2 v_texture_coordinate;

// out vec4 fragment_color;
// uniform sampler2D chair_texture;

// void main() {
//     vec3 normal = normalize(v_normal);
//     float light = dot(normal, normalize(vec3(0.25, 25.0, -25.0)));
//     fragment_color = texture(chair_texture, v_texture_coordinate);
//     fragment_color.rgb *= max(0.2, light);
// }
// "#;

//     let obj_data = fetch_text("./chair.obj").await.unwrap();
//     let obj = OBJ::try_from(obj_data).unwrap();

//     let mut renderer = Renderer::new();
//     let material = renderer.create_material(vertex_shader_source, fragment_shader_source).unwrap();
//     let geometry = renderer.create_geometry_from_ojb(obj).unwrap();
//     let mut mesh = renderer.create_mesh(geometry, material).unwrap();

//     let html_image = fetch_image("./chair.png").await.unwrap();
//     let texture = renderer.create_texture_from_html_image(html_image).unwrap();
//     mesh.material.set_uniform("chair_texture", Uniform::Texture(texture));

//     let mut transform = Transform3D::new();
//     transform.scale *= 0.005;

//     request_animation_frame(Box::new(move || {
//         renderer.clear();
//         transform.rotation *= Quat::from_rotation_x(0.01);
//         transform.rotation *= Quat::from_rotation_y(0.02);
//         mesh.material.set_uniform("transform", Uniform::Mat4(transform.to_array()));
//         renderer.render(&mut mesh);
//     }));
// }

// use suricato::{geometry::Geometry, material::Material, mesh::Mesh, renderer::App, uniforms::Uniform, utils::to_bytes};
// use wasm_bindgen::{JsCast, prelude::Closure};
// use wasm_bindgen_futures::spawn_local;

// fn main() {
//     console_error_panic_hook::set_once();
//     spawn_local(main_async());
// }

// async fn main_async() {
//     console_error_panic_hook::set_once();

//     let vertex_shader_source = r#"#version 300 es
// layout(std140) uniform Colors {
//     vec4 color_1;
//     vec4 color_2;
//     vec4 color_3;
// };

// uniform vec2 translation;
// uniform uint color_selector;
// in vec2 position;
// out vec4 v_color;

// void main() {
//     if (color_selector == 1u) {
//         v_color = color_1;
//     } else if (color_selector == 2u) {
//         v_color = color_2;
//     } else {
//         v_color = color_3;
//     }
//     gl_Position = vec4(position + translation, 0.0, 1.0);
// }
// "#;
//     let fragment_shader_source = r#"#version 300 es
// precision mediump float;

// in vec4 v_color;
// out vec4 fragment_color;

// void main() {
//     fragment_color = v_color;
// }
// "#;

//     let mut app = App::new();
//     let mut material = Material::new(vertex_shader_source, fragment_shader_source);
//     let geometry = Geometry::quad();

//     let ubo_binding_point = app.create_ubo();
//     material.set_uniform_block("Colors", ubo_binding_point);

//     let colors: Vec<f32> = vec![
//         1.0, 0.0, 0.0, 1.0, // Color 2
//         0.0, 1.0, 0.0, 1.0, // Color 3
//         0.0, 0.0, 1.0, 1.0, // Color 1
//         0.0, 0.0, 0.0, 0.0, // Padding
//     ];

//     app.set_ubo(ubo_binding_point, to_bytes(&colors).to_vec());
//     let mut mesh = Mesh::new(geometry, material);

//     let render_loop = Closure::wrap(Box::new(move || {
//         app.clear();

//         mesh.material.set_uniform("translation", Uniform::Vec2([-0.25, -0.25]));
//         mesh.material.set_uniform("color_selector", Uniform::UnsignedInt(1));
//         app.render(&mut mesh);

//         mesh.material.set_uniform("translation", Uniform::Vec2([0.0, 0.0]));
//         mesh.material.set_uniform("color_selector", Uniform::UnsignedInt(2));
//         app.render(&mut mesh);

//         mesh.material.set_uniform("translation", Uniform::Vec2([0.25, 0.25]));
//         mesh.material.set_uniform("color_selector", Uniform::UnsignedInt(3));
//         app.render(&mut mesh);
//     }) as Box<dyn FnMut()>);

//     web_sys::window()
//         .unwrap()
//         .set_interval_with_callback_and_timeout_and_arguments_0(render_loop.as_ref().unchecked_ref(), 1)
//         .unwrap();

//     render_loop.forget();
// }

// use std::{rc::Rc, sync::Mutex};

// use suricato::{
//     geometry::Geometry, material::Material, mesh::Mesh, renderer::App, texture::Texture, transform::Transform2D, uniforms::Uniform,
//     utils::fetch_image,
// };
// use wasm_bindgen::{JsCast, prelude::Closure};
// use wasm_bindgen_futures::spawn_local;
// use web_sys::console;

// fn main() {
//     console_error_panic_hook::set_once();
//     spawn_local(main_async());
// }

// async fn main_async() {
//     console_error_panic_hook::set_once();

//     let vertex_shader_source = r#"#version 300 es
// in vec2 position;
// in vec2 texture_coordinate;

// in vec3 color;
// in mat3 transform;
// out vec4 v_color;
// out vec2 v_texture_coordinate;

// void main() {
//     v_color = vec4(color, 1.0);
//     v_texture_coordinate = texture_coordinate;
//     gl_Position = vec4(transform * vec3(position, 1.0), 1.0);
// }
// "#;
//     let fragment_shader_source = r#"#version 300 es
// precision mediump float;

// in vec4 v_color;
// in vec2 v_texture_coordinate;

// out vec4 fragment_color;

// uniform sampler2D simple_sampler1;
// uniform sampler2D simple_sampler2;

// void main() {
//     fragment_color = 0.8 *texture(simple_sampler1, v_texture_coordinate) + 0.2 * texture(simple_sampler2, v_texture_coordinate);
// }
// "#;

//     let mut renderer = App::new();

//     let size = 4;

//     let mut material = Material::new(vertex_shader_source, fragment_shader_source);

//     renderer.uniform_blocks.set_block_property("Illumination", "light[0]", &[0, 1]);

//     let image = fetch_image("./cobble_stone.png").await.unwrap();
//     let texture = Texture::from(image);
//     material.set_uniform("simple_sampler1", Uniform::Texture(texture.clone()));

//     let geometry = Geometry::quad_instanced(size * size);
//     let mut mesh = Mesh::new(geometry, material);

//     let mut transforms = Vec::new();

//     for x in 0..size {
//         for y in 0..size {
//             let mut transform = Transform2D::new();
//             transform.scale *= 0.4;
//             transform.translation.x = 1.85 * (x as f32 + 0.5 - size as f32 / 2.0) / size as f32;
//             transform.translation.y = 1.85 * (y as f32 + 0.5 - size as f32 / 2.0) / size as f32;
//             transforms.push(transform);
//         }
//     }

//     // START CONFIG
//     let mut bob_trasnforms = transforms.clone();
//     let mut bob_material = Material::new(vertex_shader_source, fragment_shader_source);

//     let bob_image = fetch_image("./bob.png").await.unwrap();
//     let bob_texture = Texture::from(bob_image);

//     bob_material.set_uniform("simple_sampler1", Uniform::Texture(bob_texture.clone()));
//     bob_material.set_uniform("simple_sampler2", Uniform::Texture(texture));

//     let bob_geometry = Geometry::quad_instanced(size * size);
//     let mut bob_mesh = Mesh::new(bob_geometry, bob_material);
//     // END CONFIG

//     mesh.material.set_uniform("simple_sampler2", Uniform::Texture(bob_texture));

//     let frames_rendered = Rc::new(Mutex::new(1));
//     let frames_rendered_cloned = Rc::clone(&frames_rendered);

//     let render_loop = Closure::wrap(Box::new(move || {
//         let mut counter = frames_rendered_cloned.lock().unwrap();
//         *counter += 1;

//         renderer.clear();

//         let transform_buffer = mesh.geometry.get_vertex_buffer("transform").unwrap();

//         for (vertex_index, transform) in transforms.iter_mut().enumerate() {
//             transform.rotation += vertex_index as f32 * 0.0001;
//             transform_buffer.update_vertex(vertex_index, &transform.to_array());
//         }

//         let bob_transform_buffer = bob_mesh.geometry.get_vertex_buffer("transform").unwrap();
//         for (vertex_index, transform) in bob_trasnforms.iter_mut().enumerate() {
//             transform.scale.x = 0.2;
//             transform.scale.y = 0.2;
//             transform.rotation += vertex_index as f32 * 0.001;
//             bob_transform_buffer.update_vertex(vertex_index, &transform.to_array());
//         }
//         // bob_transform_buffer.update(0, &bob_trasnforms);

//         renderer.render(&mut mesh);
//         renderer.render(&mut bob_mesh);
//     }) as Box<dyn FnMut()>);
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

    let material = Material::new(&renderer.gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE).unwrap();
    let geometry = Geometry::quad_instanced(&renderer.gl, size * size).unwrap();
    let mut mesh = Mesh::new(&renderer.gl, geometry, material).unwrap();

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
