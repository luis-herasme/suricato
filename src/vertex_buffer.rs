use web_sys::WebGl2RenderingContext as GL;

use crate::{
    buffer_gpu::{BufferGPU, BufferKind, BufferUsage},
    utils::to_bytes,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum VertexComponentType {
    Byte          = GL::BYTE,
    UnsignedByte  = GL::UNSIGNED_BYTE,
    Short         = GL::SHORT,
    UnsignedShort = GL::UNSIGNED_SHORT,
    Int           = GL::INT,
    UnsignedInt   = GL::UNSIGNED_INT,
    Float         = GL::FLOAT,
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
    pub name:              String,
    pub component_count:   u8,
    pub component_type:    VertexComponentType,
    pub normalize:         bool,
    pub stride:            usize,
    pub offset:            usize,
    pub divisor:           u32,
    pub number_of_columns: u8,
}

impl VertexLayout {
    fn from_vertex_array(vertex_array: &[VertexData]) -> Vec<VertexLayout> {
        let mut vertex_layouts = Vec::with_capacity(vertex_array.len());

        let mut max_alignment = 0;
        let mut current_offset = 0;

        for vertex in vertex_array {
            let alignment = vertex.data.component_type().size_in_bytes() as usize;

            max_alignment = max_alignment.max(alignment);
            current_offset = VertexLayout::align_to(current_offset, alignment);

            let layout = VertexLayout {
                name:              vertex.name.clone(),
                component_count:   vertex.data.component_count(),
                component_type:    vertex.data.component_type(),
                normalize:         vertex.normalize,
                offset:            current_offset,
                stride:            0, // Will be populated after the loop
                divisor:           vertex.divisor,
                number_of_columns: vertex.data.number_of_columns(),
            };

            current_offset += vertex.data.size_in_bytes();
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
    fn align_to(value: usize, alignment: usize) -> usize {
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

impl From<&VertexData> for VertexLayout {
    fn from(vertex: &VertexData) -> VertexLayout {
        VertexLayout {
            name:              vertex.name.clone(),
            component_count:   vertex.data.component_count(),
            component_type:    vertex.data.component_type(),
            normalize:         vertex.normalize,
            stride:            vertex.data.size_in_bytes(),
            offset:            0,
            divisor:           vertex.divisor,
            number_of_columns: vertex.data.number_of_columns(),
        }
    }
}

/// A temporary container for a single attribute's raw data (e.g., a Vec of positions)
/// before it's processed into a VertexBuffer.
pub struct VertexData {
    pub name:      String,
    pub data:      Data,
    pub divisor:   u32,
    pub normalize: bool,
}

impl VertexData {
    pub fn new<T>(name: &str, data: T) -> VertexData
    where
        Data: From<T>,
    {
        VertexData {
            name:      String::from(name),
            data:      Data::from(data),
            divisor:   0,
            normalize: false,
        }
    }
}

pub enum Data {
    Byte(Vec<i8>),
    ByteVec2(Vec<[i8; 2]>),
    ByteVec3(Vec<[i8; 3]>),
    ByteVec4(Vec<[i8; 4]>),

    UnsignedByte(Vec<u8>),
    UnsignedByteVec2(Vec<[u8; 2]>),
    UnsignedByteVec3(Vec<[u8; 3]>),
    UnsignedByteVec4(Vec<[u8; 4]>),

    Float(Vec<f32>),
    Vec2(Vec<[f32; 2]>),
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),

    Int(Vec<i32>),
    IntVec2(Vec<[i32; 2]>),
    IntVec3(Vec<[i32; 3]>),
    IntVec4(Vec<[i32; 4]>),

    UnsignedInt(Vec<u32>),
    UnsignedIntVec2(Vec<[u32; 2]>),
    UnsignedIntVec3(Vec<[u32; 3]>),
    UnsignedIntVec4(Vec<[u32; 4]>),

    Short(Vec<i16>),
    ShortVec2(Vec<[i16; 2]>),
    ShortVec3(Vec<[i16; 3]>),
    ShortVec4(Vec<[i16; 4]>),

    UnsignedShort(Vec<u16>),
    UnsignedShortVec2(Vec<[u16; 2]>),
    UnsignedShortVec3(Vec<[u16; 3]>),
    UnsignedShortVec4(Vec<[u16; 4]>),

    Mat2(Vec<[[f32; 2]; 2]>),
    Mat3(Vec<[[f32; 3]; 3]>),
    Mat4(Vec<[[f32; 4]; 4]>),
}

impl From<Vec<i8>> for Data {
    fn from(value: Vec<i8>) -> Self {
        Data::Byte(value)
    }
}

impl From<Vec<[i8; 2]>> for Data {
    fn from(value: Vec<[i8; 2]>) -> Self {
        Data::ByteVec2(value)
    }
}

impl From<Vec<[i8; 3]>> for Data {
    fn from(value: Vec<[i8; 3]>) -> Self {
        Data::ByteVec3(value)
    }
}

impl From<Vec<[i8; 4]>> for Data {
    fn from(value: Vec<[i8; 4]>) -> Self {
        Data::ByteVec4(value)
    }
}

impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::UnsignedByte(value)
    }
}

impl From<Vec<[u8; 2]>> for Data {
    fn from(value: Vec<[u8; 2]>) -> Self {
        Data::UnsignedByteVec2(value)
    }
}

impl From<Vec<[u8; 3]>> for Data {
    fn from(value: Vec<[u8; 3]>) -> Self {
        Data::UnsignedByteVec3(value)
    }
}

impl From<Vec<[u8; 4]>> for Data {
    fn from(value: Vec<[u8; 4]>) -> Self {
        Data::UnsignedByteVec4(value)
    }
}

impl From<Vec<f32>> for Data {
    fn from(value: Vec<f32>) -> Self {
        Data::Float(value)
    }
}

impl From<Vec<[f32; 2]>> for Data {
    fn from(value: Vec<[f32; 2]>) -> Self {
        Data::Vec2(value)
    }
}

impl From<Vec<[f32; 3]>> for Data {
    fn from(value: Vec<[f32; 3]>) -> Self {
        Data::Vec3(value)
    }
}

impl From<Vec<[f32; 4]>> for Data {
    fn from(value: Vec<[f32; 4]>) -> Self {
        Data::Vec4(value)
    }
}

impl From<Vec<i32>> for Data {
    fn from(value: Vec<i32>) -> Self {
        Data::Int(value)
    }
}

impl From<Vec<[i32; 2]>> for Data {
    fn from(value: Vec<[i32; 2]>) -> Self {
        Data::IntVec2(value)
    }
}

impl From<Vec<[i32; 3]>> for Data {
    fn from(value: Vec<[i32; 3]>) -> Self {
        Data::IntVec3(value)
    }
}

impl From<Vec<[i32; 4]>> for Data {
    fn from(value: Vec<[i32; 4]>) -> Self {
        Data::IntVec4(value)
    }
}

impl From<Vec<u32>> for Data {
    fn from(value: Vec<u32>) -> Self {
        Data::UnsignedInt(value)
    }
}

impl From<Vec<[u32; 2]>> for Data {
    fn from(value: Vec<[u32; 2]>) -> Self {
        Data::UnsignedIntVec2(value)
    }
}

impl From<Vec<[u32; 3]>> for Data {
    fn from(value: Vec<[u32; 3]>) -> Self {
        Data::UnsignedIntVec3(value)
    }
}

impl From<Vec<[u32; 4]>> for Data {
    fn from(value: Vec<[u32; 4]>) -> Self {
        Data::UnsignedIntVec4(value)
    }
}

impl From<Vec<i16>> for Data {
    fn from(value: Vec<i16>) -> Self {
        Data::Short(value)
    }
}

impl From<Vec<[i16; 2]>> for Data {
    fn from(value: Vec<[i16; 2]>) -> Self {
        Data::ShortVec2(value)
    }
}

impl From<Vec<[i16; 3]>> for Data {
    fn from(value: Vec<[i16; 3]>) -> Self {
        Data::ShortVec3(value)
    }
}

impl From<Vec<[i16; 4]>> for Data {
    fn from(value: Vec<[i16; 4]>) -> Self {
        Data::ShortVec4(value)
    }
}

impl From<Vec<u16>> for Data {
    fn from(value: Vec<u16>) -> Self {
        Data::UnsignedShort(value)
    }
}

impl From<Vec<[u16; 2]>> for Data {
    fn from(value: Vec<[u16; 2]>) -> Self {
        Data::UnsignedShortVec2(value)
    }
}

impl From<Vec<[u16; 3]>> for Data {
    fn from(value: Vec<[u16; 3]>) -> Self {
        Data::UnsignedShortVec3(value)
    }
}

impl From<Vec<[u16; 4]>> for Data {
    fn from(value: Vec<[u16; 4]>) -> Self {
        Data::UnsignedShortVec4(value)
    }
}

impl From<Vec<[[f32; 2]; 2]>> for Data {
    fn from(value: Vec<[[f32; 2]; 2]>) -> Self {
        Data::Mat2(value)
    }
}

impl From<Vec<[[f32; 3]; 3]>> for Data {
    fn from(value: Vec<[[f32; 3]; 3]>) -> Self {
        Data::Mat3(value)
    }
}

impl From<Vec<[[f32; 4]; 4]>> for Data {
    fn from(value: Vec<[[f32; 4]; 4]>) -> Self {
        Data::Mat4(value)
    }
}

impl Data {
    pub fn count(&self) -> usize {
        match &self {
            Data::Byte(data) => data.len(),
            Data::ByteVec2(data) => data.len(),
            Data::ByteVec3(data) => data.len(),
            Data::ByteVec4(data) => data.len(),

            Data::UnsignedByte(data) => data.len(),
            Data::UnsignedByteVec2(data) => data.len(),
            Data::UnsignedByteVec3(data) => data.len(),
            Data::UnsignedByteVec4(data) => data.len(),

            Data::Float(data) => data.len(),
            Data::Vec2(data) => data.len(),
            Data::Vec3(data) => data.len(),
            Data::Vec4(data) => data.len(),

            Data::Int(data) => data.len(),
            Data::IntVec2(data) => data.len(),
            Data::IntVec3(data) => data.len(),
            Data::IntVec4(data) => data.len(),

            Data::UnsignedInt(data) => data.len(),
            Data::UnsignedIntVec2(data) => data.len(),
            Data::UnsignedIntVec3(data) => data.len(),
            Data::UnsignedIntVec4(data) => data.len(),

            Data::Short(data) => data.len(),
            Data::ShortVec2(data) => data.len(),
            Data::ShortVec3(data) => data.len(),
            Data::ShortVec4(data) => data.len(),

            Data::UnsignedShort(data) => data.len(),
            Data::UnsignedShortVec2(data) => data.len(),
            Data::UnsignedShortVec3(data) => data.len(),
            Data::UnsignedShortVec4(data) => data.len(),

            Data::Mat2(data) => data.len(),
            Data::Mat3(data) => data.len(),
            Data::Mat4(data) => data.len(),
        }
    }

    fn size_in_bytes(&self) -> usize {
        (self.component_count() * self.component_type().size_in_bytes()) as usize
    }

    fn component_count(&self) -> u8 {
        match &self {
            Data::Byte { .. } => 1,
            Data::ByteVec2 { .. } => 2,
            Data::ByteVec3 { .. } => 3,
            Data::ByteVec4 { .. } => 4,

            Data::UnsignedByte { .. } => 1,
            Data::UnsignedByteVec2 { .. } => 2,
            Data::UnsignedByteVec3 { .. } => 3,
            Data::UnsignedByteVec4 { .. } => 4,

            Data::Float { .. } => 1,
            Data::Vec2 { .. } => 2,
            Data::Vec3 { .. } => 3,
            Data::Vec4 { .. } => 4,

            Data::Int { .. } => 1,
            Data::IntVec2 { .. } => 2,
            Data::IntVec3 { .. } => 3,
            Data::IntVec4 { .. } => 4,

            Data::UnsignedInt { .. } => 1,
            Data::UnsignedIntVec2 { .. } => 2,
            Data::UnsignedIntVec3 { .. } => 3,
            Data::UnsignedIntVec4 { .. } => 4,

            Data::Short { .. } => 1,
            Data::ShortVec2 { .. } => 2,
            Data::ShortVec3 { .. } => 3,
            Data::ShortVec4 { .. } => 4,

            Data::UnsignedShort { .. } => 1,
            Data::UnsignedShortVec2 { .. } => 2,
            Data::UnsignedShortVec3 { .. } => 3,
            Data::UnsignedShortVec4 { .. } => 4,

            Data::Mat2 { .. } => 4,
            Data::Mat3 { .. } => 9,
            Data::Mat4 { .. } => 16,
        }
    }

    fn component_type(&self) -> VertexComponentType {
        match &self {
            Data::Byte { .. } => VertexComponentType::Byte,
            Data::ByteVec2 { .. } => VertexComponentType::Byte,
            Data::ByteVec3 { .. } => VertexComponentType::Byte,
            Data::ByteVec4 { .. } => VertexComponentType::Byte,

            Data::UnsignedByte { .. } => VertexComponentType::UnsignedByte,
            Data::UnsignedByteVec2 { .. } => VertexComponentType::UnsignedByte,
            Data::UnsignedByteVec3 { .. } => VertexComponentType::UnsignedByte,
            Data::UnsignedByteVec4 { .. } => VertexComponentType::UnsignedByte,

            Data::Float { .. } => VertexComponentType::Float,
            Data::Vec2 { .. } => VertexComponentType::Float,
            Data::Vec3 { .. } => VertexComponentType::Float,
            Data::Vec4 { .. } => VertexComponentType::Float,

            Data::Int { .. } => VertexComponentType::Int,
            Data::IntVec2 { .. } => VertexComponentType::Int,
            Data::IntVec3 { .. } => VertexComponentType::Int,
            Data::IntVec4 { .. } => VertexComponentType::Int,

            Data::UnsignedInt { .. } => VertexComponentType::UnsignedInt,
            Data::UnsignedIntVec2 { .. } => VertexComponentType::UnsignedInt,
            Data::UnsignedIntVec3 { .. } => VertexComponentType::UnsignedInt,
            Data::UnsignedIntVec4 { .. } => VertexComponentType::UnsignedInt,

            Data::Short { .. } => VertexComponentType::Short,
            Data::ShortVec2 { .. } => VertexComponentType::Short,
            Data::ShortVec3 { .. } => VertexComponentType::Short,
            Data::ShortVec4 { .. } => VertexComponentType::Short,

            Data::UnsignedShort { .. } => VertexComponentType::UnsignedShort,
            Data::UnsignedShortVec2 { .. } => VertexComponentType::UnsignedShort,
            Data::UnsignedShortVec3 { .. } => VertexComponentType::UnsignedShort,
            Data::UnsignedShortVec4 { .. } => VertexComponentType::UnsignedShort,

            Data::Mat2 { .. } => VertexComponentType::Float,
            Data::Mat3 { .. } => VertexComponentType::Float,
            Data::Mat4 { .. } => VertexComponentType::Float,
        }
    }

    fn number_of_columns(&self) -> u8 {
        match &self {
            Data::Mat2 { .. } => 2,
            Data::Mat3 { .. } => 3,
            Data::Mat4 { .. } => 4,
            _ => 1,
        }
    }

    fn to_bytes(&self) -> &[u8] {
        match &self {
            Data::Byte(data) => to_bytes(data),
            Data::ByteVec2(data) => to_bytes(data),
            Data::ByteVec3(data) => to_bytes(data),
            Data::ByteVec4(data) => to_bytes(data),

            Data::UnsignedByte(data) => to_bytes(data),
            Data::UnsignedByteVec2(data) => to_bytes(data),
            Data::UnsignedByteVec3(data) => to_bytes(data),
            Data::UnsignedByteVec4(data) => to_bytes(data),

            Data::Float(data) => to_bytes(data),
            Data::Vec2(data) => to_bytes(data),
            Data::Vec3(data) => to_bytes(data),
            Data::Vec4(data) => to_bytes(data),

            Data::Int(data) => to_bytes(data),
            Data::IntVec2(data) => to_bytes(data),
            Data::IntVec3(data) => to_bytes(data),
            Data::IntVec4(data) => to_bytes(data),

            Data::UnsignedInt(data) => to_bytes(data),
            Data::UnsignedIntVec2(data) => to_bytes(data),
            Data::UnsignedIntVec3(data) => to_bytes(data),
            Data::UnsignedIntVec4(data) => to_bytes(data),

            Data::Short(data) => to_bytes(data),
            Data::ShortVec2(data) => to_bytes(data),
            Data::ShortVec3(data) => to_bytes(data),
            Data::ShortVec4(data) => to_bytes(data),

            Data::UnsignedShort(data) => to_bytes(data),
            Data::UnsignedShortVec2(data) => to_bytes(data),
            Data::UnsignedShortVec3(data) => to_bytes(data),
            Data::UnsignedShortVec4(data) => to_bytes(data),

            Data::Mat2(data) => to_bytes(data),
            Data::Mat3(data) => to_bytes(data),
            Data::Mat4(data) => to_bytes(data),
        }
    }
}

/// Represents a buffer of vertex data stored in the CPC and the
/// GPU, with metadata about how the data should be uploaded to and
/// interpreted by the GPU.
pub struct VertexBuffer {
    pub layout: VertexLayout,
    pub buffer: BufferGPU,
}

impl VertexBuffer {
    pub fn new<T>(name: &str, data: T) -> VertexBuffer
    where
        Data: From<T>,
    {
        let vertex_data = VertexData::new(name, data);
        let layout = VertexLayout::from(&vertex_data);

        VertexBuffer {
            buffer: BufferGPU::new(
                BufferKind::ArrayBuffer,
                BufferUsage::StaticDraw,
                vertex_data.data.to_bytes().to_vec(),
            ),
            layout,
        }
    }

    pub fn with_config(usage: BufferUsage, vertex_data: VertexData) -> VertexBuffer {
        let layout = VertexLayout::from(&vertex_data);

        VertexBuffer {
            buffer: BufferGPU::new(BufferKind::ArrayBuffer, usage, vertex_data.data.to_bytes().to_vec()),
            layout,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.buffer.size() / self.layout.stride
    }

    #[inline]
    pub fn set_vertex<T>(&mut self, vertex_index: usize, value: &[T]) {
        self.buffer.set_bytes(vertex_index * self.layout.stride, value);
    }
}

pub struct InterleavedVertexBuffer {
    pub buffer:  BufferGPU,
    pub layouts: Vec<VertexLayout>,
}

impl InterleavedVertexBuffer {
    pub fn new(usage: BufferUsage, data: Vec<VertexData>) -> InterleavedVertexBuffer {
        let layouts = VertexLayout::from_vertex_array(&data);

        let data: Vec<Data> = data.into_iter().map(|x| x.data).collect();
        let data = InterleavedVertexBuffer::vertex_data_array_to_bytes(&data, &layouts);

        InterleavedVertexBuffer {
            buffer: BufferGPU::new(BufferKind::ArrayBuffer, usage, data),
            layouts,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.buffer.size() / self.stride()
    }

    pub fn stride(&self) -> usize {
        for layout in &self.layouts {
            return layout.stride as usize;
        }

        unreachable!("Vertex buffer cannot be empty");
    }

    fn vertex_data_array_to_bytes(vertex_data_array: &[Data], layout_array: &[VertexLayout]) -> Vec<u8> {
        let vertex_count = vertex_data_array[0].count();
        let stride = layout_array[0].stride;

        let mut interleaved_buffer = vec![0; stride * vertex_count];

        for i in 0..vertex_data_array.len() {
            let vertex = &vertex_data_array[i];
            let offset = &layout_array[i].offset;

            for vertex_index in 0..vertex_count {
                let vertex_size_in_bytes = vertex.size_in_bytes();

                let source_start = vertex_index * vertex_size_in_bytes;
                let source_final = source_start + vertex_size_in_bytes;

                let vertex_byte_index = vertex_index * stride + offset;

                let destination_start = vertex_byte_index;
                let destination_final = vertex_byte_index + vertex_size_in_bytes;

                interleaved_buffer[destination_start..destination_final].copy_from_slice(&vertex.to_bytes()[source_start..source_final]);
            }
        }

        interleaved_buffer
    }

    /// Updates a specific vertex attribute for a given vertex index.
    ///
    /// # Returns
    /// * `true` if the update was successful.
    /// * `false` if the attribute name was not found in the layout.
    #[inline]
    pub fn update_vertex<T>(&mut self, name: &str, vertex_index: usize, value: &[T]) -> bool {
        if let Some(byte_offset) = self.get_vertex_byte_offset(name, vertex_index) {
            self.buffer.set_bytes(byte_offset, value);
            return true;
        }

        false
    }

    /// Calculates the byte offset inside the buffer for a specific vertex attribute.
    /// This is useful for interlaved VertexBuffers.
    #[inline]
    pub fn get_vertex_byte_offset(&self, attribute_name: &str, vertex_index: usize) -> Option<usize> {
        for layout in &self.layouts {
            if layout.name != attribute_name {
                continue;
            }

            return Some(vertex_index * layout.stride + layout.offset);
        }

        None
    }
}
