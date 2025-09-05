use web_sys::WebGl2RenderingContext as GL;

use crate::{buffer_gpu::BufferGPU, utils::to_bytes};

pub struct IndexBuffer {
    pub kind:   u32,
    pub count:  usize,
    pub offset: usize,
    pub buffer: BufferGPU,
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
    pub fn from_u8(data: Vec<u8>) -> Self {
        IndexBuffer {
            kind:   GL::UNSIGNED_BYTE,
            count:  data.len(),
            offset: 0,
            buffer: BufferGPU::index_buffer(data),
        }
    }

    pub fn from_u16(data: Vec<u16>) -> Self {
        IndexBuffer {
            kind:   GL::UNSIGNED_SHORT,
            count:  data.len(),
            offset: 0,
            buffer: BufferGPU::index_buffer(to_bytes(&data).to_vec()),
        }
    }

    pub fn from_u32(data: Vec<u32>) -> Self {
        IndexBuffer {
            kind:   GL::UNSIGNED_INT,
            count:  data.len(),
            offset: 0,
            buffer: BufferGPU::index_buffer(to_bytes(&data).to_vec()),
        }
    }
}
