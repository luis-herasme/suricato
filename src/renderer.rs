use std::collections::HashMap;

use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlVertexArrayObject};

use crate::{
    mesh::{Mesh, MeshId},
    ubo::UniformBufferObject,
};

pub struct App {
    pub gl:     GL,
    pub canvas: HtmlCanvasElement,

    // Resources
    vaos:                   HashMap<MeshId, WebGlVertexArrayObject>,
    uniform_buffer_objects: Vec<UniformBufferObject>,
}

impl App {
    pub fn new() -> App {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        document.body().unwrap().append_child(&canvas).unwrap();
        canvas.set_width(800);
        canvas.set_height(800);

        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();

        gl.enable(GL::DEPTH_TEST);

        App {
            gl,
            canvas,
            vaos: HashMap::new(),
            uniform_buffer_objects: Vec::new(),
        }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    pub fn render(&mut self, mesh: &mut Mesh) {
        for vertex_buffer in &mut mesh.geometry.vertex_buffers {
            vertex_buffer.on_render(&self.gl);
        }

        for interleaved_vertex_buffer in &mut mesh.geometry.interleaved_vertex_buffers {
            interleaved_vertex_buffer.on_render(&self.gl);
        }

        mesh.material.on_render(&self.gl);

        for uniform_buffer_object in &self.uniform_buffer_objects {
            uniform_buffer_object.update(&self.gl);
        }

        if !self.vaos.contains_key(&mesh.id) {
            let vao = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(&vao));

            // Set attributes
            let material_resource = mesh.material.webgl_resources.as_ref().unwrap();

            for vertex_buffer in &mesh.geometry.vertex_buffers {
                self.gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.buffer_gpu.as_ref());
                material_resource.set_attribute_buffer(&vertex_buffer.layout);
            }

            for vertex_buffer in &mesh.geometry.interleaved_vertex_buffers {
                self.gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.buffer_gpu.as_ref());
                for vertex_layout in &vertex_buffer.layouts {
                    material_resource.set_attribute_buffer(vertex_layout);
                }
            }

            self.gl.bind_vertex_array(None);
            self.vaos.insert(mesh.id, vao);
        }

        let vao = self.vaos.get(&mesh.id);
        self.gl.bind_vertex_array(vao);

        if let Some(indices) = &mut mesh.geometry.indices {
            let indices_webgl_buffer = indices.get_or_create_gpu_buffer(&self.gl);
            self.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(indices_webgl_buffer));

            if let Some(instance_count) = mesh.geometry.instance_count {
                self.gl.draw_elements_instanced_with_i32(
                    mesh.render_primitive as u32,
                    indices.count as i32,
                    indices.kind,
                    indices.offset,
                    instance_count as i32,
                );
            } else {
                self.gl
                    .draw_elements_with_i32(mesh.render_primitive as u32, indices.count as i32, indices.kind, indices.offset);
            }
        } else {
            self.gl
                .draw_arrays(mesh.render_primitive as u32, 0, mesh.geometry.vertex_count as i32);
        }
    }

    /// UBO
    pub fn create_ubo(&mut self) -> u32 {
        let ubo_binding_point = self.uniform_buffer_objects.len() as u32;
        let ubo = UniformBufferObject::new(&self.gl, ubo_binding_point);
        self.uniform_buffer_objects.push(ubo);
        ubo_binding_point
    }

    pub fn set_ubo(&mut self, ubo_binding_point: u32, value: Vec<u8>) {
        self.uniform_buffer_objects[ubo_binding_point as usize].set_buffer(value);
    }
}
