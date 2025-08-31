use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::utils::{generate_id, to_bytes};

pub enum IndexData {
    UnsignedByte(Vec<u8>),
    UnsignedShort(Vec<u16>),
    UnsignedInt(Vec<u32>),
}

impl IndexData {
    fn count(&self) -> usize {
        match &self {
            IndexData::UnsignedByte(data) => data.len(),
            IndexData::UnsignedShort(data) => data.len(),
            IndexData::UnsignedInt(data) => data.len(),
        }
    }

    fn kind(&self) -> u32 {
        match &self {
            IndexData::UnsignedByte(_) => WebGl2RenderingContext::UNSIGNED_BYTE,
            IndexData::UnsignedShort(_) => WebGl2RenderingContext::UNSIGNED_SHORT,
            IndexData::UnsignedInt(_) => WebGl2RenderingContext::UNSIGNED_INT,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match &self {
            IndexData::UnsignedByte(data) => to_bytes(data),
            IndexData::UnsignedShort(data) => to_bytes(data),
            IndexData::UnsignedInt(data) => to_bytes(data),
        }
    }

    pub fn to_index_buffer(self) -> IndexBuffer {
        IndexBuffer::new(self)
    }
}

pub struct IndexLayout {
    pub kind:               u32,
    pub count:              i32,
    pub offset:             i32,
    pub number_of_elements: u32,
}

pub struct IndexBuffer {
    pub id:     u64,
    pub data:   IndexData,
    pub layout: IndexLayout,
}

impl From<Vec<u8>> for IndexBuffer {
    fn from(data: Vec<u8>) -> Self {
        IndexBuffer::from_u8(data)
    }
}

impl From<Vec<u16>> for IndexBuffer {
    fn from(data: Vec<u16>) -> Self {
        IndexBuffer::from_u16(data)
    }
}

impl From<Vec<u32>> for IndexBuffer {
    fn from(data: Vec<u32>) -> Self {
        IndexBuffer::from_u32(data)
    }
}

impl IndexBuffer {
    pub fn new(data: IndexData) -> IndexBuffer {
        IndexBuffer {
            id: generate_id(),
            layout: IndexLayout {
                offset:             0,
                kind:               data.kind(),
                count:              data.count() as i32,
                number_of_elements: data.count() as u32,
            },
            data,
        }
    }

    pub fn from_u8(data: Vec<u8>) -> Self {
        IndexBuffer::new(IndexData::UnsignedByte(data))
    }

    pub fn from_u16(data: Vec<u16>) -> Self {
        IndexBuffer::new(IndexData::UnsignedShort(data))
    }

    pub fn from_u32(data: Vec<u32>) -> Self {
        IndexBuffer::new(IndexData::UnsignedInt(data))
    }

    pub fn create_webgl_buffer(&self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_u8_array(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            self.data.bytes(),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        buffer
    }
}
