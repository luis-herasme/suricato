use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

use crate::utils::to_bytes;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
    ArrayBuffer        = GL::ARRAY_BUFFER,
    UniformBuffer      = GL::UNIFORM_BUFFER,
    ElementArrayBuffer = GL::ELEMENT_ARRAY_BUFFER,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    StaticDraw  = GL::STATIC_DRAW,
    DynamicDraw = GL::DYNAMIC_DRAW,
}

#[derive(Debug)]
pub enum BufferError {
    UsageChangeAfterCreation,
    CreationFailed,
}

#[derive(Debug, Clone)]
pub struct BufferGPU {
    gl:           GL,
    kind:         BufferKind,
    buffer_cpu:   Vec<u8>,
    buffer_gpu:   WebGlBuffer,
    needs_update: bool,
}

impl BufferGPU {
    pub fn new(gl: GL, kind: BufferKind, usage: BufferUsage, buffer_cpu: Vec<u8>) -> Result<BufferGPU, BufferError> {
        let buffer_gpu = BufferGPU::create_buffer_gpu(&gl, kind, usage, &buffer_cpu)?;

        Ok(BufferGPU {
            gl,
            kind,
            buffer_cpu,
            buffer_gpu,
            needs_update: false,
        })
    }

    fn create_buffer_gpu(gl: &GL, kind: BufferKind, usage: BufferUsage, buffer_cpu: &[u8]) -> Result<WebGlBuffer, BufferError> {
        let Some(webgl_buffer) = gl.create_buffer() else {
            return Err(BufferError::CreationFailed);
        };

        gl.bind_buffer(kind as u32, Some(&webgl_buffer));
        gl.buffer_data_with_u8_array(kind as u32, buffer_cpu, usage as u32);
        Ok(webgl_buffer)
    }

    #[inline]
    pub fn set_bytes<T>(&mut self, byte_offset: usize, value: &[T]) {
        let bytes = to_bytes(&value);
        self.buffer_cpu[byte_offset..byte_offset + bytes.len()].copy_from_slice(bytes);
        self.needs_update = true;
    }

    #[inline(always)]
    pub fn on_before_render(&mut self) {
        if !self.needs_update {
            return;
        }

        self.update_buffer_gpu();
        self.needs_update = false;
    }

    fn update_buffer_gpu(&self) {
        self.gl.bind_buffer(self.kind as u32, Some(&self.buffer_gpu));
        self.gl.buffer_sub_data_with_i32_and_u8_array(self.kind as u32, 0, &self.buffer_cpu);
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(self.kind as u32, Some(&self.buffer_gpu));
    }

    pub fn size(&self) -> usize {
        self.buffer_cpu.len()
    }
}
