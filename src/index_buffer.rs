use web_sys::WebGl2RenderingContext as GL;

use crate::{buffer_gpu::*, utils::to_bytes};

pub struct IndexBuffer {
    pub kind:   u32,
    pub count:  usize,
    pub offset: usize,
    pub buffer: BufferGPU,
}

impl IndexBuffer {
    pub fn from_u8(usage: BufferUsage, buffer_cpu: Vec<u8>) -> IndexBuffer {
        IndexBuffer {
            kind:   GL::UNSIGNED_BYTE,
            count:  buffer_cpu.len(),
            offset: 0,
            buffer: BufferGPU::new(BufferKind::ElementArrayBuffer, usage, buffer_cpu),
        }
    }

    pub fn from_u16(usage: BufferUsage, buffer_cpu: Vec<u16>) -> IndexBuffer {
        IndexBuffer {
            kind:   GL::UNSIGNED_SHORT,
            count:  buffer_cpu.len(),
            offset: 0,
            buffer: BufferGPU::new(BufferKind::ElementArrayBuffer, usage, to_bytes(&buffer_cpu).to_vec()),
        }
    }

    pub fn from_u32(usage: BufferUsage, buffer_cpu: Vec<u32>) -> IndexBuffer {
        IndexBuffer {
            kind:   GL::UNSIGNED_INT,
            count:  buffer_cpu.len(),
            offset: 0,
            buffer: BufferGPU::new(BufferKind::ElementArrayBuffer, usage, to_bytes(&buffer_cpu).to_vec()),
        }
    }
}
