use web_sys::WebGl2RenderingContext;

use crate::{
    attributes::{Attribute, AttributeBuffer, AttributeData, InterleavedAttributeBuffer, SingleAttributeBuffer},
    generate_id::generate_id,
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub id:         u64,
    pub attributes: Vec<Attribute>,
    pub indices:    Option<IndexData>,
}

impl Geometry {
    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        Geometry {
            id: generate_id(),
            attributes: vec![
                Attribute::Single(
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
                IndexData::UnsignedInt(vec![
                    0, 1, 2, // Triangle #1
                    2, 3, 0, // Triangle #2
                ])
            )
        }
    }
}

pub struct GeometryResource {
    pub vertex_count:      i32,
    pub index_buffer:      Option<IndexBuffer>,
    pub attribute_buffers: Vec<AttributeBuffer>,
}

impl GeometryResource {
    pub fn new(gl: &WebGl2RenderingContext, geometry: &Geometry) -> GeometryResource {
        let mut attribute_buffers = Vec::new();

        for attribute in &geometry.attributes {
            let buffer = match attribute {
                Attribute::Single(attribute) => SingleAttributeBuffer::new(gl, attribute),
                Attribute::Interleaved(attributes) => InterleavedAttributeBuffer::new(gl, attributes),
            };

            attribute_buffers.push(buffer);
        }

        let index_buffer = geometry.indices.as_ref().map(|indices| IndexBuffer::from_index_data(gl, indices));

        let vertex_count = if let Some(attribute) = geometry.attributes.get(0) {
            attribute.vertex_count() as i32
        } else {
            0
        };

        GeometryResource {
            vertex_count,
            index_buffer,
            attribute_buffers,
        }
    }
}
