use crate::{
    attributes::{SingleAttributeVertexBuffer, VertexBuffer, VertexData},
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub vertex_count: i32,
    pub indices:      Option<IndexBuffer>,
    pub vertex_data:  Vec<VertexBuffer>,
}

impl Geometry {
    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        let vertex_data = vec![
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
        ];

        let vertex_count = if let Some(attribute) = vertex_data.get(0) {
            attribute.vertex_count() as i32
        } else {
            0
        };

        Geometry {
            vertex_count,
            vertex_data,
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
