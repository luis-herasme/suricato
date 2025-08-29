use crate::{
    index_buffer::{IndexBuffer, IndexData},
    transform::Transform,
    vertex_buffer::{VertexBuffer, VertexData, VertexDescriptor},
};

pub struct Geometry {
    pub instance_count: Option<usize>,
    pub vertex_count:   usize,
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

    pub fn set_vertex_at_mat4(&mut self, name: &str, index: usize, value: [f32; 16]) {
        for vertex_buffer in &mut self.vertex_buffers {
            let value_was_set = vertex_buffer.set_vertex_at_mat4(name, index, value);

            if value_was_set {
                return;
            }
        }
    }

    pub fn quad() -> Geometry {
        let position = VertexDescriptor::new(
            "position",
            VertexData::Vec2(vec![
                [0.5, 0.5],   // Top right
                [0.5, -0.5],  // Bottom right
                [-0.5, -0.5], // Bottom left
                [-0.5, 0.5],  // Top left
            ]),
        );

        let color = VertexDescriptor::new(
            "color",
            VertexData::UByteVec3(vec![
                [255, 0, 0], // Top right
                [0, 255, 0], // Bottom right
                [0, 0, 255], // Bottom left
                [0, 255, 0], // Top left
            ]),
        )
        .normalize();

        let indices = IndexData::UnsignedByte(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:   4,
            instance_count: None,
            indices:        Some(indices.to_index_buffer()),
            vertex_buffers: vec![position.to_vertex_buffer(), color.to_vertex_buffer()],
        }
    }

    pub fn quad_interleaved() -> Geometry {
        let position = VertexDescriptor::new(
            "position",
            VertexData::Vec2(vec![
                [0.5, 0.5],   // Top right
                [0.5, -0.5],  // Bottom right
                [-0.5, -0.5], // Bottom left
                [-0.5, 0.5],  // Top left
            ]),
        );

        let color = VertexDescriptor::new(
            "color",
            VertexData::UByteVec3(vec![
                [255, 0, 0], // Top right
                [0, 255, 0], // Bottom right
                [0, 0, 255], // Bottom left
                [0, 255, 0], // Top left
            ]),
        )
        .normalize();

        let indices = IndexData::UnsignedByte(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:   4,
            instance_count: None,
            indices:        Some(indices.to_index_buffer()),
            vertex_buffers: vec![VertexBuffer::interleaved_vertices(vec![position, color])],
        }
    }

    #[rustfmt::skip]
    pub fn instance_quad(count: usize) -> Geometry {
        let mut trasnforms = Vec::with_capacity(count);

        for _ in 0..count {
            trasnforms.push(Transform::new().to_array());
        }

        Geometry {
            instance_count: Some(count),
            vertex_count: 4,
            vertex_buffers: vec![
                VertexBuffer::interleaved_vertices(vec![
                    VertexDescriptor::new(
                        "position",
                        VertexData::Vec2(vec![
                            [0.5, 0.5],   // Top right
                            [0.5, -0.5],  // Bottom right
                            [-0.5, -0.5], // Bottom left
                            [-0.5, 0.5],  // Top left
                        ]),
                    ),
                    VertexDescriptor::new(
                        "color",
                        VertexData::UByteVec3(vec![
                            [255, 0, 0], // Top right
                            [0, 255, 0], // Bottom right
                            [0, 0, 255], // Bottom left
                            [0, 255, 0], // Top left
                        ]),
                    )
                    .normalize(),
                ]),
                VertexBuffer::single_vertex(VertexDescriptor::new("transform", VertexData::Mat4(trasnforms)).divisor(1)),
            ],
            indices: Some(IndexBuffer::from_u8(vec![
                0, 1, 2, // Triangle #1
                2, 3, 0, // Triangle #2
            ])),
        }
    }
}
