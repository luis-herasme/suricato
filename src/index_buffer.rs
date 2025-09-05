use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

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
            IndexData::UnsignedByte(_) => GL::UNSIGNED_BYTE,
            IndexData::UnsignedShort(_) => GL::UNSIGNED_SHORT,
            IndexData::UnsignedInt(_) => GL::UNSIGNED_INT,
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

pub struct IndexBuffer {
    pub id:         u64,
    pub buffer_cpu: IndexData,
    pub buffer_gpu: Option<WebGlBuffer>,
    pub kind:       u32,
    pub count:      usize,
    pub offset:     i32,
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
    pub fn new(buffer_cpu: IndexData) -> IndexBuffer {
        IndexBuffer {
            id: generate_id(),
            offset: 0,
            kind: buffer_cpu.kind(),
            count: buffer_cpu.count(),
            buffer_cpu,
            buffer_gpu: None,
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

    pub fn create_webgl_buffer(&mut self, gl: &GL) {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, self.buffer_cpu.bytes(), GL::STATIC_DRAW);
        self.buffer_gpu = Some(buffer);
    }
}
