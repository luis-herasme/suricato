use crate::{
    index_buffer::IndexBuffer,
    transform::Transform2D,
    vertex_buffer::{Data, VertexBuffer, VertexData},
};

pub struct Geometry {
    pub instance_count: Option<usize>,
    pub vertex_count:   usize,
    pub indices:        Option<IndexBuffer>,
    pub vertex_buffers: Vec<VertexBuffer>,
}

impl Geometry {
    pub fn update_vertex<T>(&mut self, name: &str, vertex_index: usize, value: &[T]) {
        for vertex_buffer in &mut self.vertex_buffers {
            if vertex_buffer.update_vertex(name, vertex_index, value) {
                return;
            }
        }
    }

    pub fn quad() -> Geometry {
        let quad_data: Vec<[f32; 2]> = vec![
            [0.5, 0.5],   // Top right
            [0.5, -0.5],  // Bottom right
            [-0.5, -0.5], // Bottom left
            [-0.5, 0.5],  // Top left
        ];

        let position = VertexData {
            name:      String::from("position"),
            data:      Data::Vec2(quad_data),
            normalize: false,
            divisor:   0,
        };

        let color_data: Vec<[u8; 3]> = vec![
            [255, 0, 0], // Top right
            [0, 255, 0], // Bottom right
            [0, 0, 255], // Bottom left
            [0, 255, 0], // Top left
        ];

        let color = VertexData {
            name:      String::from("color"),
            data:      Data::UnsignedByteVec3(color_data),
            normalize: true,
            divisor:   0,
        };

        let indices = IndexBuffer::from_u8(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:   4,
            instance_count: None,
            indices:        Some(indices),
            vertex_buffers: vec![VertexBuffer::new(position), VertexBuffer::new(color)],
        }
    }

    pub fn quad_interleaved() -> Geometry {
        let quad_data: Vec<[f32; 2]> = vec![
            [0.5, 0.5],   // Top right
            [0.5, -0.5],  // Bottom right
            [-0.5, -0.5], // Bottom left
            [-0.5, 0.5],  // Top left
        ];

        let position = VertexData {
            name:      String::from("position"),
            data:      Data::Vec2(quad_data),
            normalize: false,
            divisor:   0,
        };

        let color_data: Vec<[u8; 3]> = vec![
            [255, 0, 0], // Top right
            [0, 255, 0], // Bottom right
            [0, 0, 255], // Bottom left
            [0, 255, 0], // Top left
        ];

        let color = VertexData {
            name:      String::from("color"),
            data:      Data::UnsignedByteVec3(color_data),
            normalize: true,
            divisor:   0,
        };

        let indices = IndexBuffer::from_u8(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:   4,
            instance_count: None,
            indices:        Some(indices),
            vertex_buffers: vec![VertexBuffer::interleaved_vertices(vec![position, color])],
        }
    }

    pub fn quad_instanced_and_interleaved(count: usize) -> Geometry {
        // Instance buffer
        let mut trasnforms = Vec::with_capacity(count);

        for _ in 0..count {
            trasnforms.push(Transform2D::new().to_array());
        }

        let instance_buffer = VertexBuffer::new(VertexData {
            name:      String::from("transform"),
            data:      Data::Mat3(trasnforms),
            normalize: false,
            divisor:   1,
        });

        // Model buffer
        let quad_data: Vec<[f32; 2]> = vec![
            [0.5, 0.5],   // Top right
            [0.5, -0.5],  // Bottom right
            [-0.5, -0.5], // Bottom left
            [-0.5, 0.5],  // Top left
        ];

        let position = VertexData {
            name:      String::from("position"),
            data:      Data::Vec2(quad_data),
            normalize: false,
            divisor:   0,
        };

        let color_data: Vec<[u8; 3]> = vec![
            [255, 0, 0], // Top right
            [0, 255, 0], // Bottom right
            [0, 0, 255], // Bottom left
            [0, 255, 0], // Top left
        ];

        let color = VertexData {
            name:      String::from("color"),
            data:      Data::UnsignedByteVec3(color_data),
            normalize: true,
            divisor:   0,
        };

        let model_buffer = VertexBuffer::interleaved_vertices(vec![position, color]);

        let indices = IndexBuffer::from_u8(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:   4,
            indices:        Some(indices),
            instance_count: Some(count),
            vertex_buffers: vec![model_buffer, instance_buffer],
        }
    }
}
