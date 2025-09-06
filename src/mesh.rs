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

#[derive(Debug)]
pub enum MeshError {
    VAOCreationFailed,
    UninitializedMaterial,
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

    pub fn get_or_create_vao(&mut self, gl: &GL) -> Result<&WebGlVertexArrayObject, MeshError> {
        if self.vao.is_none() {
            let vao = self.create_vao(gl)?;
            self.vao = Some(vao);
        }

        Ok(self.vao.as_ref().unwrap())
    }

    fn create_vao(&mut self, gl: &GL) -> Result<WebGlVertexArrayObject, MeshError> {
        let Some(vao) = gl.create_vertex_array() else {
            return Err(MeshError::VAOCreationFailed);
        };

        gl.bind_vertex_array(Some(&vao));

        // Set attributes
        let Some(material_resource) = &self.material.webgl_resources else {
            return Err(MeshError::UninitializedMaterial);
        };

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
        Ok(vao)
    }
}
