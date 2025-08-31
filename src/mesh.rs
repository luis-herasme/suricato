use web_sys::WebGl2RenderingContext;

use crate::{geometry::Geometry, material::Material, utils::generate_id};

pub type MeshId = u64;

/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArraysInstanced#mode
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum RenderPrimitive {
    Points        = WebGl2RenderingContext::POINTS,
    LineStrip     = WebGl2RenderingContext::LINE_STRIP,
    LineLoop      = WebGl2RenderingContext::LINE_LOOP,
    Lines         = WebGl2RenderingContext::LINES,
    TriangleStrip = WebGl2RenderingContext::TRIANGLE_STRIP,
    TriangleFan   = WebGl2RenderingContext::TRIANGLE_FAN,
    Triangles     = WebGl2RenderingContext::TRIANGLES,
}

pub struct Mesh {
    pub id:               MeshId,
    pub geometry:         Geometry,
    pub material:         Material,
    pub render_primitive: RenderPrimitive,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Mesh {
        Mesh {
            id: generate_id(),
            geometry,
            material,
            render_primitive: RenderPrimitive::Triangles,
        }
    }
}
