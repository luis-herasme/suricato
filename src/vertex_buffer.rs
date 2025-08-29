use web_sys::WebGl2RenderingContext;

use crate::utils::{generate_id, to_bytes};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum VertexComponentType {
    Byte          = WebGl2RenderingContext::BYTE,
    UnsignedByte  = WebGl2RenderingContext::UNSIGNED_BYTE,
    Short         = WebGl2RenderingContext::SHORT,
    UnsignedShort = WebGl2RenderingContext::UNSIGNED_SHORT,
    Int           = WebGl2RenderingContext::INT,
    UnsignedInt   = WebGl2RenderingContext::UNSIGNED_INT,
    Float         = WebGl2RenderingContext::FLOAT,
}

impl VertexComponentType {
    pub fn size_in_bytes(&self) -> u8 {
        match &self {
            VertexComponentType::Byte => 1,
            VertexComponentType::UnsignedByte => 1,
            VertexComponentType::Short => 2,
            VertexComponentType::UnsignedShort => 2,
            VertexComponentType::Int => 4,
            VertexComponentType::UnsignedInt => 4,
            VertexComponentType::Float => 4,
        }
    }
}

pub struct VertexLayout {
    pub name:            String,
    pub component_count: u8,
    pub component_type:  VertexComponentType,
    pub normalize:       bool,
    pub stride:          u8,
    pub offset:          u8,
    pub divisor:         u32,
}

impl VertexLayout {
    fn from_vertex_array(vertex_array: &Vec<VertexDescriptor>) -> Vec<VertexLayout> {
        let mut vertex_layouts = Vec::with_capacity(vertex_array.len());

        let mut max_alignment = 0;
        let mut current_offset = 0;

        for vertex in vertex_array {
            let alignment = vertex.data.component_type().size_in_bytes();

            max_alignment = max_alignment.max(alignment);
            current_offset = VertexLayout::align_to(current_offset, alignment);

            let layout = VertexLayout {
                name:            vertex.name.clone(),
                component_count: vertex.data.component_count(),
                component_type:  vertex.data.component_type(),
                normalize:       vertex.normalize,
                offset:          current_offset,
                stride:          0, // Will be populated after the loop
                divisor:         vertex.divisor,
            };

            current_offset += vertex.data.component_count() * vertex.data.component_type().size_in_bytes();
            vertex_layouts.push(layout);
        }

        // The stride must be aligned to a value that is valid for all attributes.
        // Since possible alignment values for attributes are powers of two,
        // aligning to the maximum alignment ensures it is a multiple of all smaller alignments,
        let stride = VertexLayout::align_to(current_offset, max_alignment);

        // After the previous loop, offset will be equal to the stride
        for vertex_layout in &mut vertex_layouts {
            vertex_layout.stride = stride;
        }

        vertex_layouts
    }

    /// Aligns a value to the specified alignment boundary.
    ///
    /// This ensures that data is placed at memory addresses that are multiples
    /// of the alignment requirement, which is necessary for optimal GPU access.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(VertexLayout::align_to(5, 4), 8);  // 5 aligned to 4-byte boundary = 8
    /// assert_eq!(VertexLayout::align_to(8, 4), 8);  // 8 is already aligned
    /// ```
    fn align_to(value: u8, alignment: u8) -> u8 {
        if alignment == 0 {
            return value;
        }

        let remainder = value % alignment;

        if remainder == 0 {
            return value;
        }

        return value + (alignment - remainder);
    }
}

pub struct VertexDescriptor {
    name:      String,
    data:      VertexData,
    divisor:   u32,
    normalize: bool,
}

impl VertexDescriptor {
    pub fn new(name: &str, data: VertexData) -> VertexDescriptor {
        VertexDescriptor {
            name: String::from(name),
            data,
            divisor: 0,
            normalize: false,
        }
    }

    pub fn normalize(mut self) -> VertexDescriptor {
        self.normalize = true;
        self
    }

    pub fn divisor(mut self, value: u32) -> VertexDescriptor {
        self.divisor = value;
        self
    }

    pub fn to_vertex_buffer(self) -> VertexBuffer {
        VertexBuffer::single_vertex(self)
    }
}

pub enum VertexData {
    Byte(Vec<i8>),

    UByte(Vec<u8>),
    UByteVec3(Vec<[u8; 3]>),

    Float(Vec<f32>),
    Vec2(Vec<[f32; 2]>),
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),

    Int(Vec<i32>),
    IntVec2(Vec<[i32; 2]>),
    IntVec3(Vec<[i32; 3]>),
    IntVec4(Vec<[i32; 4]>),

    Mat4(Vec<[f32; 16]>),
}

impl VertexData {
    pub fn count(&self) -> usize {
        match &self {
            VertexData::Byte(data) => data.len(),
            VertexData::UByte(data) => data.len(),
            VertexData::UByteVec3(data) => data.len(),

            VertexData::Float(data) => data.len(),
            VertexData::Vec2(data) => data.len(),
            VertexData::Vec3(data) => data.len(),
            VertexData::Vec4(data) => data.len(),

            VertexData::Int(data) => data.len(),
            VertexData::IntVec2(data) => data.len(),
            VertexData::IntVec3(data) => data.len(),
            VertexData::IntVec4(data) => data.len(),

            VertexData::Mat4(data) => data.len(),
        }
    }

    fn component_count(&self) -> u8 {
        match &self {
            VertexData::Byte { .. } => 1,
            VertexData::UByte { .. } => 1,
            VertexData::UByteVec3 { .. } => 3,

            VertexData::Float { .. } => 1,
            VertexData::Vec2 { .. } => 2,
            VertexData::Vec3 { .. } => 3,
            VertexData::Vec4 { .. } => 4,

            VertexData::Int { .. } => 1,
            VertexData::IntVec2 { .. } => 2,
            VertexData::IntVec3 { .. } => 3,
            VertexData::IntVec4 { .. } => 4,

            VertexData::Mat4 { .. } => 16,
        }
    }

    fn component_type(&self) -> VertexComponentType {
        match &self {
            VertexData::Byte { .. } => VertexComponentType::Byte,
            VertexData::UByte { .. } => VertexComponentType::UnsignedByte,
            VertexData::UByteVec3 { .. } => VertexComponentType::UnsignedByte,

            VertexData::Float { .. } => VertexComponentType::Float,
            VertexData::Vec2 { .. } => VertexComponentType::Float,
            VertexData::Vec3 { .. } => VertexComponentType::Float,
            VertexData::Vec4 { .. } => VertexComponentType::Float,

            VertexData::Int { .. } => VertexComponentType::Int,
            VertexData::IntVec2 { .. } => VertexComponentType::Int,
            VertexData::IntVec3 { .. } => VertexComponentType::Int,
            VertexData::IntVec4 { .. } => VertexComponentType::Int,

            VertexData::Mat4 { .. } => VertexComponentType::Float,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match &self {
            VertexData::Byte(data) => to_bytes(data).to_vec(),
            VertexData::UByte(data) => data.clone(),
            VertexData::UByteVec3(data) => to_bytes(data).to_vec(),

            VertexData::Float(data) => to_bytes(data).to_vec(),
            VertexData::Vec2(data) => to_bytes(data).to_vec(),
            VertexData::Vec3(data) => to_bytes(data).to_vec(),
            VertexData::Vec4(data) => to_bytes(data).to_vec(),

            VertexData::Int(data) => to_bytes(data).to_vec(),
            VertexData::IntVec2(data) => to_bytes(data).to_vec(),
            VertexData::IntVec3(data) => to_bytes(data).to_vec(),
            VertexData::IntVec4(data) => to_bytes(data).to_vec(),

            VertexData::Mat4(data) => to_bytes(data).to_vec(),
        }
    }

    fn write_vertex_bytes(&self, vertex_index: usize, vertex_byte_index: usize, buffer: &mut Vec<u8>) {
        match self {
            VertexData::Byte(data) => {
                buffer[vertex_byte_index] = data[vertex_byte_index] as u8;
            }
            VertexData::UByte(data) => {
                buffer[vertex_byte_index] = data[vertex_byte_index];
            }
            VertexData::UByteVec3(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 3].copy_from_slice(to_bytes(&data[vertex_index]));
            }

            VertexData::Float(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 4].copy_from_slice(&data[vertex_byte_index].to_ne_bytes());
            }
            VertexData::Vec2(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 2 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::Vec3(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 3 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::Vec4(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 4 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }

            VertexData::Int(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 4].copy_from_slice(&data[vertex_byte_index].to_ne_bytes());
            }
            VertexData::IntVec2(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 2 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::IntVec3(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 3 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::IntVec4(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 4 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }

            VertexData::Mat4(data) => {
                buffer[vertex_byte_index..vertex_byte_index + 16 * 4].copy_from_slice(to_bytes(&data[vertex_index]));
            }
        }
    }
}

pub struct VertexBuffer {
    pub id:           u64,
    pub count:        usize,
    pub data:         Vec<u8>,
    pub layout:       Vec<VertexLayout>,
    pub needs_update: bool,
}

impl VertexBuffer {
    pub fn single_vertex(vertex: VertexDescriptor) -> VertexBuffer {
        let layout = VertexLayout {
            name:            vertex.name.clone(),
            component_count: vertex.data.component_count(),
            component_type:  vertex.data.component_type(),
            normalize:       vertex.normalize,
            stride:          vertex.data.component_count() * vertex.data.component_type().size_in_bytes(),
            offset:          0,
            divisor:         vertex.divisor,
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            count:        vertex.data.count(),
            data:         vertex.data.to_bytes(),
            layout:       vec![layout],
        }
    }

    pub fn interleaved_vertices(data: Vec<VertexDescriptor>) -> VertexBuffer {
        let layout = VertexLayout::from_vertex_array(&data);

        let data: Vec<VertexData> = data.into_iter().map(|x| x.data).collect();
        let count = data[0].count();
        let data = VertexBuffer::array_to_bytes(&data, &layout);

        VertexBuffer {
            id: generate_id(),
            needs_update: true,
            count,
            data,
            layout,
        }
    }

    pub fn set_vertex_at_f32(&mut self, name: &str, index: usize, value: f32) -> bool {
        for layout in &self.layout {
            if layout.name != name {
                continue;
            }

            let byte_index = index * layout.stride as usize + layout.offset as usize;
            self.data[byte_index..byte_index + 4].copy_from_slice(&value.to_ne_bytes());
            self.needs_update = true;

            return true;
        }

        false
    }

    pub fn set_vertex_at_mat4(&mut self, name: &str, index: usize, value: [f32; 16]) -> bool {
        for layout in &self.layout {
            if layout.name != name {
                continue;
            }

            let byte_index = index * layout.stride as usize + layout.offset as usize;
            self.data[byte_index..byte_index + 64].copy_from_slice(to_bytes(&value));
            self.needs_update = true;

            return true;
        }

        false
    }

    fn array_to_bytes(vertex_data_array: &Vec<VertexData>, layout: &Vec<VertexLayout>) -> Vec<u8> {
        let vertex_count = vertex_data_array[0].count();
        let stride = layout[0].stride as usize;

        let mut buffer = vec![0; stride * vertex_count];

        for vertex_index in 0..vertex_count {
            for i in 0..vertex_data_array.len() {
                let vertex_data = &vertex_data_array[i];
                let vertex_layout = &layout[i];
                let vertex_byte_index = vertex_index * stride;
                vertex_data.write_vertex_bytes(vertex_index, vertex_byte_index + vertex_layout.offset as usize, &mut buffer);
            }
        }

        buffer
    }
}
