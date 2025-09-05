use web_sys::{WebGl2RenderingContext as GL, WebGlVertexArrayObject};

use crate::{geometry::Geometry, material::Material};

/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArraysInstanced#mode
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum RenderPrimitive {
    Points        = GL::POINTS,
    LineStrip     = GL::LINE_STRIP,
    LineLoop      = GL::LINE_LOOP,
    Lines         = GL::LINES,
    TriangleStrip = GL::TRIANGLE_STRIP,
    TriangleFan   = GL::TRIANGLE_FAN,
    Triangles     = GL::TRIANGLES,
}

pub struct Mesh {
    pub geometry:         Geometry,
    pub material:         Material,
    pub render_primitive: RenderPrimitive,
    pub vao:              Option<WebGlVertexArrayObject>,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Mesh {
        Mesh {
            geometry,
            material,
            render_primitive: RenderPrimitive::Triangles,
            vao: None,
        }
    }

    pub fn get_or_create_vao(&mut self, gl: &GL) -> &WebGlVertexArrayObject {
        if self.vao.is_none() {
            self.vao = Some(self.create_vao(gl));
        }

        self.vao.as_ref().unwrap()
    }

    fn create_vao(&mut self, gl: &GL) -> WebGlVertexArrayObject {
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(&vao));

        // Set attributes
        let material_resource = self.material.webgl_resources.as_ref().unwrap();

        for vertex_buffer in &self.geometry.vertex_buffers {
            vertex_buffer.buffer.bind(gl);
            material_resource.set_attribute_buffer(&vertex_buffer.layout);
        }

        for vertex_buffer in &self.geometry.interleaved_vertex_buffers {
            vertex_buffer.buffer.bind(gl);
            for vertex_layout in &vertex_buffer.layouts {
                material_resource.set_attribute_buffer(vertex_layout);
            }
        }

        gl.bind_vertex_array(None);
        vao
    }
}
