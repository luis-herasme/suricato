use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};
use web_sys::{HtmlImageElement, wasm_bindgen::JsCast};

use crate::{
    buffer_gpu::BufferError,
    geometry::Geometry,
    material::{Material, MaterialError},
    mesh::{Mesh, MeshError},
    obj_parser::OBJ,
    texture::{Texture, TextureData, TextureError},
    ubo::UniformBufferObject,
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
    pub gl:                 GL,
    pub canvas:             HtmlCanvasElement,
    uniform_buffer_objects: Vec<UniformBufferObject>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        document.body().unwrap().append_child(&canvas).unwrap();
        canvas.set_width(800);
        canvas.set_height(800);

        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();

        gl.enable(GL::DEPTH_TEST);

        Renderer {
            gl,
            canvas,
            uniform_buffer_objects: Vec::new(),
        }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    pub fn render(&mut self, mesh: &mut Mesh) {
        for vertex_buffer in &mut mesh.geometry.vertex_buffers {
            vertex_buffer.buffer.on_before_render();
        }

        for interleaved_vertex_buffer in &mut mesh.geometry.interleaved_vertex_buffers {
            interleaved_vertex_buffer.buffer.on_before_render();
        }

        for uniform_buffer_object in &mut self.uniform_buffer_objects {
            uniform_buffer_object.buffer.on_before_render();
        }

        mesh.material.on_before_render(&self.gl);

        self.gl.bind_vertex_array(Some(&mesh.vao));

        if let Some(indices) = &mut mesh.geometry.indices {
            indices.buffer.on_before_render();
            indices.buffer.bind();

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

    /// UBO
    pub fn add_ubo(&mut self, ubo: UniformBufferObject) {
        self.uniform_buffer_objects.push(ubo);
    }

    pub fn get_ubo(&mut self, ubo_binding_point: usize) -> &UniformBufferObject {
        &self.uniform_buffer_objects[ubo_binding_point]
    }

    /// MATERIAL
    pub fn create_material(&self, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Material, MaterialError> {
        Material::new(self.gl.clone(), vertex_shader_source, fragment_shader_source)
    }

    /// TEXTURES
    pub fn create_texture_from_html_image(&self, html_image: HtmlImageElement) -> Result<Texture, TextureError> {
        Texture::new(&self.gl, TextureData::HtmlImageElement(html_image))
    }

    /// MESH
    pub fn create_mesh(&self, geometry: Geometry, material: Material) -> Result<Mesh, MeshError> {
        Mesh::new(&self.gl, geometry, material)
    }

    /// GEOMETRY
    pub fn create_geometry_from_ojb(&self, obj: OBJ) -> Result<Geometry, BufferError> {
        Geometry::from_obj(self.gl.clone(), obj)
    }
}
