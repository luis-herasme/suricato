use web_sys::{WebGl2RenderingContext as GL, WebGlVertexArrayObject};

use crate::{geometry::Geometry, material::Material, transform::Transform3D};

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
    pub transform:        Transform3D,
    pub geometry:         Geometry,
    pub material:         Material,
    pub render_primitive: RenderPrimitive,
    pub vao:              Option<WebGlVertexArrayObject>,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Mesh {
        Mesh {
            transform: Transform3D::new(),
            vao: None,
            geometry,
            material,
            render_primitive: RenderPrimitive::Triangles,
        }
    }

    pub fn get_or_create_vao(&mut self, gl: &GL) -> Option<&WebGlVertexArrayObject> {
        if self.vao.is_none() {
            let vao = self.create_vao(gl).unwrap();
            self.vao = Some(vao);
        }

        self.vao.as_ref()
    }

    fn create_vao(&mut self, gl: &GL) -> Result<WebGlVertexArrayObject, MeshError> {
        let Some(vao) = gl.create_vertex_array() else {
            return Err(MeshError::VAOCreationFailed);
        };

        gl.bind_vertex_array(Some(&vao));

        for vertex_buffer in &self.geometry.vertex_buffers {
            vertex_buffer.buffer.bind(gl);
            self.material
                .resources
                .as_ref()
                .unwrap()
                .set_attribute_buffer(&vertex_buffer.layout);
        }

        for vertex_buffer in &self.geometry.interleaved_vertex_buffers {
            vertex_buffer.buffer.bind(gl);
            for vertex_layout in &vertex_buffer.layouts {
                self.material.resources.as_ref().unwrap().set_attribute_buffer(vertex_layout);
            }
        }

        gl.bind_vertex_array(None);
        Ok(vao)
    }
}
