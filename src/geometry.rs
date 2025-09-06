use crate::{
    buffer_gpu::{BufferError, BufferUsage},
    index_buffer::IndexBuffer,
    obj_parser::OBJ,
    renderer::Renderer,
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

static QUAD_POSITIONS: [[f32; 2]; 4] = [
    [0.5, 0.5],   // Top right
    [0.5, -0.5],  // Bottom right
    [-0.5, -0.5], // Bottom left
    [-0.5, 0.5],  // Top left
];

static QUAD_COLORS: [[u8; 3]; 4] = [
    [255, 0, 0], // Top right
    [0, 255, 0], // Bottom right
    [0, 0, 255], // Bottom left
    [0, 255, 0], // Top left
];

static QUAD_UVS: [[f32; 2]; 4] = [
    [1.0, 1.0], // Top right
    [1.0, 0.0], // Bottom right
    [0.0, 0.0], // Bottom left
    [0.0, 1.0], // Top left
];

static QUAD_INDICES: [u8; 6] = [
    0, 1, 2, // Triangle #1
    2, 3, 0, // Triangle #2
];

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

    pub fn from_obj(renderer: &Renderer, obj: OBJ) -> Result<Geometry, BufferError> {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        for face in &obj.faces {
            let position_index = face[0] as usize - 1;
            let normal_index = face[2] as usize - 1;
            let uv_index = face[1] as usize - 1;

            let position = obj.positions[position_index];
            let normal = obj.normals[normal_index];
            let uv = obj.uvs[uv_index];

            positions.push(position.clone());
            normals.push(normal.clone());
            uvs.push(uv.clone());
        }

        let positions = VertexData {
            name:      String::from("position"),
            data:      Data::Vec3(positions),
            divisor:   0,
            normalize: false,
        };

        let normals = VertexData {
            name:      String::from("normal"),
            data:      Data::Vec3(normals),
            divisor:   0,
            normalize: false,
        };

        let uvs = VertexData {
            name:      String::from("uv"),
            data:      Data::Vec2(uvs),
            divisor:   0,
            normalize: false,
        };

        Ok(Geometry::from(InterleavedVertexBuffer::new(
            renderer.gl.clone(),
            BufferUsage::StaticDraw,
            vec![positions, normals, uvs],
        )?))
    }

    fn quad_data() -> (VertexData, VertexData, VertexData) {
        let position = VertexData {
            name:      String::from("position"),
            data:      Data::Vec2(Vec::from(QUAD_POSITIONS)),
            normalize: false,
            divisor:   0,
        };

        let color = VertexData {
            name:      String::from("color"),
            data:      Data::UnsignedByteVec3(Vec::from(QUAD_COLORS)),
            normalize: true,
            divisor:   0,
        };

        let uvs = VertexData {
            name:      String::from("uv"),
            data:      Data::Vec2(Vec::from(QUAD_UVS)),
            normalize: false,
            divisor:   0,
        };

        (position, color, uvs)
    }

    pub fn quad(renderer: &Renderer) -> Result<Geometry, BufferError> {
        let (position, color, uvs) = Geometry::quad_data();
        let indices = IndexBuffer::from_u8(renderer.gl.clone(), BufferUsage::StaticDraw, Vec::from(QUAD_INDICES))?;

        Ok(Geometry {
            vertex_count:               4,
            instance_count:             None,
            indices:                    Some(indices),
            vertex_buffers:             vec![
                VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, position)?,
                VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, color)?,
                VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, uvs)?,
            ],
            interleaved_vertex_buffers: vec![],
        })
    }

    pub fn quad_interleaved(renderer: &Renderer) -> Result<Geometry, BufferError> {
        let (position, color, uvs) = Geometry::quad_data();

        let indices = IndexBuffer::from_u8(
            renderer.gl.clone(),
            BufferUsage::StaticDraw,
            vec![
                0, 1, 2, // Triangle #1
                2, 3, 0, // Triangle #2
            ],
        )?;

        Ok(Geometry {
            vertex_count:               4,
            instance_count:             None,
            indices:                    Some(indices),
            vertex_buffers:             vec![],
            interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(
                renderer.gl.clone(),
                BufferUsage::StaticDraw,
                vec![position, color, uvs],
            )?],
        })
    }

    pub fn quad_instanced_and_interleaved(renderer: &Renderer, count: usize) -> Result<Geometry, BufferError> {
        // Instance buffer
        let mut trasnforms = Vec::with_capacity(count);

        for _ in 0..count {
            trasnforms.push(Transform2D::new().to_array());
        }

        let transform_buffer = VertexBuffer::new(
            renderer.gl.clone(),
            BufferUsage::StaticDraw,
            VertexData {
                name:      String::from("transform"),
                data:      Data::Mat3(trasnforms),
                normalize: false,
                divisor:   1,
            },
        )?;

        // Model buffer
        let (position, color, uvs) = Geometry::quad_data();

        let indices = IndexBuffer::from_u8(
            renderer.gl.clone(),
            BufferUsage::StaticDraw,
            vec![
                0, 1, 2, // Triangle #1
                2, 3, 0, // Triangle #2
            ],
        )?;

        Ok(Geometry {
            vertex_count:               4,
            indices:                    Some(indices),
            instance_count:             Some(count),
            interleaved_vertex_buffers: vec![InterleavedVertexBuffer::new(
                renderer.gl.clone(),
                BufferUsage::StaticDraw,
                vec![position, color, uvs],
            )?],
            vertex_buffers:             vec![transform_buffer],
        })
    }

    pub fn quad_instanced(renderer: &Renderer, count: usize) -> Result<Geometry, BufferError> {
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
        let (position, color, uvs) = Geometry::quad_data();

        let color = VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, color)?;
        let position = VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, position)?;
        let uvs = VertexBuffer::new(renderer.gl.clone(), BufferUsage::StaticDraw, uvs)?;
        let per_instance_transforms = VertexBuffer::new(renderer.gl.clone(), BufferUsage::DynamicDraw, per_instance_transforms)?;

        let indices = IndexBuffer::from_u8(
            renderer.gl.clone(),
            BufferUsage::StaticDraw,
            vec![
                0, 1, 2, // Triangle #1
                2, 3, 0, // Triangle #2
            ],
        )?;

        Ok(Geometry {
            vertex_count:               4,
            indices:                    Some(indices),
            instance_count:             Some(count),
            interleaved_vertex_buffers: vec![],
            vertex_buffers:             vec![color, position, uvs, per_instance_transforms],
        })
    }
}

impl From<InterleavedVertexBuffer> for Geometry {
    fn from(interleaved_vertex_buffer: InterleavedVertexBuffer) -> Geometry {
        Geometry {
            instance_count:             None,
            vertex_count:               interleaved_vertex_buffer.vertex_count(),
            indices:                    None,
            vertex_buffers:             vec![],
            interleaved_vertex_buffers: vec![interleaved_vertex_buffer],
        }
    }
}
