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
    pub vao:              WebGlVertexArrayObject,
}

impl Mesh {
    pub fn new(gl: &GL, geometry: Geometry, material: Material) -> Result<Mesh, MeshError> {
        Ok(Mesh {
            vao: Mesh::create_vao(gl, &geometry, &material)?,
            geometry,
            material,
            render_primitive: RenderPrimitive::Triangles,
        })
    }

    fn create_vao(gl: &GL, geometry: &Geometry, material: &Material) -> Result<WebGlVertexArrayObject, MeshError> {
        let Some(vao) = gl.create_vertex_array() else {
            return Err(MeshError::VAOCreationFailed);
        };

        gl.bind_vertex_array(Some(&vao));

        for vertex_buffer in &geometry.vertex_buffers {
            vertex_buffer.buffer.bind();
            material.set_attribute_buffer(&vertex_buffer.layout);
        }

        for vertex_buffer in &geometry.interleaved_vertex_buffers {
            vertex_buffer.buffer.bind();
            for vertex_layout in &vertex_buffer.layouts {
                material.set_attribute_buffer(vertex_layout);
            }
        }

        gl.bind_vertex_array(None);
        Ok(vao)
    }
}
