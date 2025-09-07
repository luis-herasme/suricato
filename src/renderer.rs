use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

use crate::{
    buffer_gpu::BufferError,
    camera::PerspectiveCamera,
    material::MaterialError,
    mesh::{Mesh, MeshError},
};

#[derive(Debug)]
pub enum RenderError {
    MeshError(MeshError),
    BufferError(BufferError),
    MaterialError(MaterialError),
}

impl From<BufferError> for RenderError {
    fn from(value: BufferError) -> Self {
        RenderError::BufferError(value)
    }
}

impl From<MaterialError> for RenderError {
    fn from(value: MaterialError) -> Self {
        RenderError::MaterialError(value)
    }
}

impl From<MeshError> for RenderError {
    fn from(value: MeshError) -> Self {
        RenderError::MeshError(value)
    }
}

pub struct Renderer {
    pub gl:     GL,
    pub canvas: HtmlCanvasElement,
}

impl Renderer {
    pub fn new() -> Renderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        document.body().unwrap().append_child(&canvas).unwrap();
        let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
        let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();

        gl.enable(GL::DEPTH_TEST);

        Renderer { gl, canvas }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    pub fn handle_window_resize(&mut self, camera: &mut PerspectiveCamera) {
        let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap();

        if width as u32 != self.canvas.width() || height as u32 != self.canvas.height() {
            camera.aspect = width as f32 / height as f32;
            camera.update_projection_matrix();

            self.canvas.set_width(width as u32);
            self.canvas.set_height(height as u32);

            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn render(&mut self, mesh: &mut Mesh) {
        for vertex_buffer in &mut mesh.geometry.vertex_buffers {
            vertex_buffer.buffer.on_before_render(&self.gl);
        }

        for interleaved_vertex_buffer in &mut mesh.geometry.interleaved_vertex_buffers {
            interleaved_vertex_buffer.buffer.on_before_render(&self.gl);
        }

        mesh.material.on_before_render(&self.gl);

        self.gl.bind_vertex_array(mesh.get_or_create_vao(&self.gl));

        if let Some(indices) = &mut mesh.geometry.indices {
            indices.buffer.on_before_render(&self.gl);
            indices.buffer.bind(&self.gl);

            if let Some(instance_count) = mesh.geometry.instance_count {
                self.gl.draw_elements_instanced_with_i32(
                    mesh.render_primitive as u32,
                    indices.count as i32,
                    indices.kind,
                    indices.offset as i32,
                    instance_count as i32,
                );
            } else {
                self.gl.draw_elements_with_i32(
                    mesh.render_primitive as u32,
                    indices.count as i32,
                    indices.kind,
                    indices.offset as i32,
                );
            }
        } else {
            self.gl
                .draw_arrays(mesh.render_primitive as u32, 0, mesh.geometry.vertex_count as i32);
        }
    }
}
