use crate::{
    attributes::{VertexBuffer, VertexData},
    index_buffer::{IndexBuffer, IndexData},
};

pub struct Geometry {
    pub vertex_count:   i32,
    pub indices:        Option<IndexBuffer>,
    pub vertex_buffers: Vec<VertexBuffer>,
}

impl Geometry {
    pub fn set_vertex_at_f32(&mut self, name: &str, index: usize, value: f32) {
        for vertex_buffer in &mut self.vertex_buffers {
            let value_was_set = vertex_buffer.set_vertex_at_f32(name, index, value);

            if value_was_set {
                return;
            }
        }
    }

    #[rustfmt::skip]
    pub fn quad() -> Geometry {
        let vertex_data = vec![
            VertexBuffer::interleaved_attributes(
                vec![
                    (
                        "position".to_string(),
                        VertexData::Vec2(vec![
                            0.5, 0.5,   // Top right
                            0.5, -0.5,  // Bottom right
                            -0.5, -0.5, // Bottom left
                            -0.5, 0.5,  // Top left
                        ])
                    ),
                    (
                        "color".to_string(),
                        VertexData::Vec3(vec![
                            1.0, 0.0, 0.0, // Top right
                            0.0, 1.0, 0.0, // Bottom right
                            0.0, 0.0, 1.0, // Bottom left
                            0.0, 1.0, 0.0, // Top left
                        ])
                    )
                ]
            )
            // VertexBuffer::single_attribute("position",
            //     VertexData::Vec2(vec![
            //         0.5, 0.5,   // Top right
            //         0.5, -0.5,  // Bottom right
            //         -0.5, -0.5, // Bottom left
            //         -0.5, 0.5,  // Top left
            //     ])
            // ),
            // VertexBuffer::single_attribute("color",
            //     VertexData::Vec3(vec![
            //         1.0, 0.0, 0.0, // Top right
            //         0.0, 1.0, 0.0, // Bottom right
            //         0.0, 0.0, 1.0, // Bottom left
            //         0.0, 1.0, 0.0, // Top left
            //     ])
            // ),
        ];

        let vertex_count = match vertex_data.get(0) {
            Some(attribute) => attribute.vertex_count() as i32,
            None => 0
        };

        Geometry {
            vertex_count,
            vertex_buffers: vertex_data,
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
