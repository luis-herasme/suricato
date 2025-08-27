use web_sys::WebGl2RenderingContext;

use crate::{
    attributes::{AttributeData, InterleavedAttributesVertexBuffer, SingleAttributeVertexBuffer, VertexBuffer, VertexData},
    generate_id::generate_id,
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub id:          u64,
    pub indices:     Option<IndexData>,
    pub vertex_data: Vec<VertexData>,
}

impl Geometry {
    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        Geometry {
            id: generate_id(),
            vertex_data: vec![
                VertexData::SingleAttribute(
                    AttributeData::Vec2 {
                        name: String::from("position"),
                        data: vec![
                            (0.5, 0.5),   // Top right
                            (0.5, -0.5),  // Bottom right
                            (-0.5, -0.5), // Bottom left
                            (-0.5, 0.5),  // Top left
                        ],
                    }
                )
            ],
            indices: Some(
                IndexData::UnsignedByte(vec![
                    0, 1, 2, // Triangle #1
                    2, 3, 0, // Triangle #2
                ])
            )
        }
    }
}

pub struct GeometryResource {
    pub vertex_count:   i32,
    pub index_buffer:   Option<IndexBuffer>,
    pub vertex_buffers: Vec<VertexBuffer>,
}

impl GeometryResource {
    pub fn new(gl: &WebGl2RenderingContext, geometry: &Geometry) -> GeometryResource {
        let mut vertex_buffers = Vec::new();

        for vertex_data in &geometry.vertex_data {
            let buffer = match vertex_data {
                VertexData::SingleAttribute(vertex_data) => SingleAttributeVertexBuffer::new(gl, vertex_data),
                VertexData::InterleavedAttributes(vertex_data_array) => InterleavedAttributesVertexBuffer::new(gl, vertex_data_array),
            };

            vertex_buffers.push(buffer);
        }

        let index_buffer = geometry.indices.as_ref().map(|indices| IndexBuffer::from_index_data(gl, indices));

        let vertex_count = if let Some(attribute) = geometry.vertex_data.get(0) {
            attribute.vertex_count() as i32
        } else {
            0
        };

        GeometryResource {
            vertex_count,
            index_buffer,
            vertex_buffers,
        }
    }
}
