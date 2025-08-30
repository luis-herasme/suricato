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
    fn from_vertex_array(vertex_array: &Vec<VertexData>) -> Vec<VertexLayout> {
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

/// A temporary container for a single attribute's raw data (e.g., a Vec of positions)
/// before it's processed into a VertexBuffer.
pub struct VertexData {
    pub name:      String,
    pub data:      Data,
    pub divisor:   u32,
    pub normalize: bool,
}

pub enum Data {
    Byte(Vec<i8>),
    ByteVec2(Vec<[u8; 2]>),
    ByteVec3(Vec<[u8; 3]>),
    ByteVec4(Vec<[u8; 4]>),

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

    Mat2(Vec<[f32; 4]>),
    Mat3(Vec<[f32; 9]>),
    Mat4(Vec<[f32; 16]>),
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

/// Represents a buffer of vertex data stored in system memory,
/// with metadata about how the data should be uploaded to and
/// interpreted by the GPU.
///
/// A `VertexBuffer` contains raw vertex data (`data`) along with
/// a description of how that data is structured (`layout`).
/// The GPU-side buffer is not automatically updated when `data` is
/// modified. To notify the renderer that the GPU copy must be updated,
/// set [`needs_update`] to `true`.
pub struct VertexBuffer {
    /// Unique identifier for this buffer, used internally to distinguish
    /// between multiple vertex buffers.
    pub id: u64,

    /// Raw byte representation of the vertex data.
    ///
    /// Modifying this field directly does not affect the GPU copy of
    /// the buffer. After editing, set [`needs_update`] to `true` to
    /// signal that the buffer should be re-uploaded to the GPU.
    pub data: Vec<u8>,

    /// Describes how the data inside [`data`] is laid out.
    ///
    /// Each entry in this vector corresponds to an attribute (e.g.,
    /// position, normal, color). If the vector contains more than one
    /// layout, the data is interleaved.
    pub layout: Vec<VertexLayout>,

    /// Marks whether the GPU buffer needs to be updated.
    ///
    /// If `true`, the renderer should re-upload [`data`] to the GPU.
    pub needs_update: bool,
}

impl VertexBuffer {
    pub fn new(vertex: VertexData) -> VertexBuffer {
        let layout = VertexLayout {
            name:              vertex.name.clone(),
            component_count:   vertex.data.component_count(),
            component_type:    vertex.data.component_type(),
            normalize:         vertex.normalize,
            stride:            vertex.data.size_in_bytes(),
            offset:            0,
            divisor:           vertex.divisor,
            number_of_columns: vertex.data.number_of_columns(),
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            data:         vertex.data.to_bytes().to_vec(),
            layout:       vec![layout],
        }
    }

    pub fn vertex_count(&self) -> usize {
        for layout in &self.layout {
            return self.data.len() / layout.stride;
        }

        unreachable!("All vertex buffers should have at least 1 layout");
    }

    pub fn interleaved_vertices(data: Vec<VertexData>) -> VertexBuffer {
        let layout = VertexLayout::from_vertex_array(&data);

        let data: Vec<Data> = data.into_iter().map(|x| x.data).collect();
        let data = VertexBuffer::interleaved_buffer_from_vertex_data_array(&data, &layout);

        VertexBuffer {
            id: generate_id(),
            needs_update: true,
            data,
            layout,
        }
    }

    /// Writes raw bytes into the buffer at a specific byte offset.
    #[inline]
    pub fn update<T>(&mut self, byte_offset: usize, value: &[T]) {
        let bytes = to_bytes(&value);
        self.data[byte_offset..byte_offset + bytes.len()].copy_from_slice(bytes);
        self.needs_update = true;
    }

    /// Updates a specific vertex attribute for a given vertex index.
    ///
    /// # Returns
    /// * `true` if the update was successful.
    /// * `false` if the attribute name was not found in the layout.
    #[inline]
    pub fn update_vertex<T>(&mut self, name: &str, vertex_index: usize, value: &[T]) -> bool {
        if let Some(byte_index) = self.get_vertex_byte_offset(name, vertex_index) {
            self.update(byte_index, value);
            return true;
        }

        false
    }

    /// Calculates the byte offset inside the buffer for a specific vertex attribute.
    /// This is useful for interlaved VertexBuffers.
    #[inline]
    pub fn get_vertex_byte_offset(&self, attribute_name: &str, vertex_index: usize) -> Option<usize> {
        for layout in &self.layout {
            if layout.name != attribute_name {
                continue;
            }

            return Some(vertex_index * layout.stride + layout.offset);
        }

        None
    }

    fn interleaved_buffer_from_vertex_data_array(vertex_data_array: &Vec<Data>, layout_array: &Vec<VertexLayout>) -> Vec<u8> {
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
}
