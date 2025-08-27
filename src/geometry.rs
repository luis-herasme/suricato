use web_sys::WebGl2RenderingContext;

use crate::{
    attributes::{AttributeBuffer, AttributeData},
    generate_id::generate_id,
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub id:         u64,
    pub attributes: Vec<AttributeData>,
    pub indices:    Option<IndexData>,
}

impl Geometry {
    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        Geometry {
            id: generate_id(),
            attributes: vec![
                AttributeData::Vec2 {
                    name: String::from("position"),
                    data: vec![
                        (0.5, 0.5),   // Top right
                        (0.5, -0.5),  // Bottom right
                        (-0.5, -0.5), // Bottom left
                        (-0.5, 0.5),  // Top left
                    ],
                }
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
    pub vertex_count: u32,
    pub attributes:   Vec<AttributeBuffer>,
    pub indices:      Option<IndexBuffer>,
}

impl GeometryResource {
    pub fn new(gl: &WebGl2RenderingContext, geometry: &Geometry) -> GeometryResource {
        let mut attributes = Vec::new();

        for attribute in &geometry.attributes {
            attributes.push(AttributeBuffer::from_attribute_data(gl, attribute))
        }

        let indices = geometry.indices.as_ref().map(|indices| IndexBuffer::from_index_data(gl, indices));

        GeometryResource {
            vertex_count: geometry.attributes.get(0).unwrap().number_of_elements() as u32,
            indices,
            attributes,
        }
    }
}
