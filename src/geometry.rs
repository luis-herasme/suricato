use crate::{
    index_buffer::IndexBuffer,
    transform::Transform2D,
    vertex_buffer::{Data, InterleavedVertexBuffer, VertexBuffer, VertexData},
};

pub struct Geometry {
    pub instance_count:             Option<usize>,
    pub vertex_count:               usize,
    pub indices:                    Option<IndexBuffer>,
    pub vertex_buffers:             Vec<VertexBuffer>,
    pub interleaved_vertex_buffers: Vec<InterleavedVertexBuffer>,
}

impl Geometry {
    pub fn get_vertex_buffer(&mut self, name: &str) -> Option<&mut VertexBuffer> {
        for vertex_buffer in &mut self.vertex_buffers {
            if vertex_buffer.layout.name == name {
                return Some(vertex_buffer);
            }
        }

        None
    }

    /// Returns a mutable reference to the [`InterleavedVertexBuffer`] that contains the
    /// specified vertex attribute, if it exists.
    ///
    /// # Note:
    /// - A single `InterleavedVertexBuffer` can store multiple attributes when interleaved.
    ///   Mutating it directly may unintentionally affect other attributes.
    pub fn get_interleaved_vertex_buffer(&mut self, name: &str) -> Option<&mut InterleavedVertexBuffer> {
        for interleaved_vertex in &mut self.interleaved_vertex_buffers {
            for layout in &interleaved_vertex.layouts {
                if layout.name == name {
                    return Some(interleaved_vertex);
                }
            }
        }

        None
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
            vertex_count:               4,
            instance_count:             None,
            indices:                    Some(indices),
            vertex_buffers:             vec![VertexBuffer::new(position), VertexBuffer::new(color)],
            interleaved_vertex_buffers: vec![],
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
            vertex_count:               4,
            instance_count:             None,
            indices:                    Some(indices),
            vertex_buffers:             vec![],
            interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(vec![position, color])],
        }
    }

    pub fn quad_instanced_and_interleaved(count: usize) -> Geometry {
        // Instance buffer
        let mut trasnforms = Vec::with_capacity(count);

        for _ in 0..count {
            trasnforms.push(Transform2D::new().to_array());
        }

        let transform_buffer = VertexBuffer::new(VertexData {
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

        let indices = IndexBuffer::from_u8(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count:               4,
            indices:                    Some(indices),
            instance_count:             Some(count),
            interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(vec![position, color])],
            vertex_buffers:             vec![transform_buffer],
        }
    }

    pub fn quad_instanced(count: usize) -> Geometry {
        // Instance buffer
        let mut trasnforms = Vec::with_capacity(count);

        for _ in 0..count {
            trasnforms.push(Transform2D::new().to_array());
        }

        let per_instance_transforms = VertexData {
            name:      String::from("transform"),
            data:      Data::Mat3(trasnforms),
            normalize: false,
            divisor:   1,
        };

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

        let texture_coordinates_data: Vec<[f32; 2]> = vec![
            [1.0, 1.0], // Top right
            [1.0, 0.0], // Bottom right
            [0.0, 0.0], // Bottom left
            [0.0, 1.0], // Top left
        ];

        let texture_coordinate = VertexData {
            name:      String::from("texture_coordinate"),
            data:      Data::Vec2(texture_coordinates_data),
            normalize: true,
            divisor:   0,
        };

        let vertex_buffers = vec![
            VertexBuffer::new(color),
            VertexBuffer::new(position),
            VertexBuffer::new(texture_coordinate),
            VertexBuffer::new(per_instance_transforms),
        ];

        let indices = IndexBuffer::from_u8(vec![
            0, 1, 2, // Triangle #1
            2, 3, 0, // Triangle #2
        ]);

        Geometry {
            vertex_count: 4,
            indices: Some(indices),
            instance_count: Some(count),
            interleaved_vertex_buffers: vec![],
            vertex_buffers,
        }
    }
}
