use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{
    attributes::{VertexData, SingleAttributeVertexBuffer, VertexBuffer},
    generate_id::generate_id,
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub id:          u64,
    pub indices:     Option<IndexBuffer>,
    pub vertex_data: Vec<VertexBuffer>,
}

impl Geometry {
    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        Geometry {
            id: generate_id(),
            vertex_data: vec![
                VertexBuffer::SingleAttribute(
                    SingleAttributeVertexBuffer::new(
                        "position",
                        VertexData::Vec2(vec![
                            (0.5, 0.5),   // Top right
                            (0.5, -0.5),  // Bottom right
                            (-0.5, -0.5), // Bottom left
                            (-0.5, 0.5),  // Top left
                        ])
                    )
                )
            ],
            indices: Some(
                IndexBuffer::new(
                    IndexData::UnsignedByte(vec![
                        0, 1, 2, // Triangle #1
                        2, 3, 0, // Triangle #2
                    ])
                )
            )
        }
    }
}

pub struct GeometryResource {
    pub vertex_count:         i32,
    pub index_webgl_buffer:   Option<WebGlBuffer>,
    pub vertex_webgl_buffers: Vec<WebGlBuffer>,
}

impl GeometryResource {
    pub fn new(gl: &WebGl2RenderingContext, geometry: &Geometry) -> GeometryResource {
        let mut vertex_webgl_buffers = Vec::new();

        for vertex_data in &geometry.vertex_data {
            let buffer = match vertex_data {
                VertexBuffer::SingleAttribute(attribute) => attribute.create_webgl_buffer(gl),
                VertexBuffer::InterleavedAttributes(attribute) => attribute.create_webgl_buffer(gl),
            };

            vertex_webgl_buffers.push(buffer);
        }

        let index_buffer = geometry.indices.as_ref().map(|indices| indices.create_webgl_buffer(gl));

        let vertex_count = if let Some(attribute) = geometry.vertex_data.get(0) {
            attribute.vertex_count() as i32
        } else {
            0
        };

        GeometryResource {
            vertex_count,
            index_webgl_buffer: index_buffer,
            vertex_webgl_buffers,
        }
    }
}
