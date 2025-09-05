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

#[derive(Debug, Clone)]
pub struct BufferGPU {
    kind:         BufferKind,
    usage:        BufferUsage,
    buffer_cpu:   Vec<u8>,
    buffer_gpu:   Option<WebGlBuffer>,
    needs_update: bool,
}

impl BufferGPU {
    pub fn array_buffer(data: Vec<u8>) -> BufferGPU {
        BufferGPU {
            kind:         BufferKind::ArrayBuffer,
            usage:        BufferUsage::StaticDraw,
            buffer_cpu:   data,
            buffer_gpu:   None,
            needs_update: false,
        }
    }

    pub fn uniform_buffer(data: Vec<u8>) -> BufferGPU {
        BufferGPU {
            kind:         BufferKind::UniformBuffer,
            usage:        BufferUsage::DynamicDraw,
            buffer_cpu:   data,
            buffer_gpu:   None,
            needs_update: false,
        }
    }

    pub fn index_buffer(data: Vec<u8>) -> BufferGPU {
        BufferGPU {
            kind:         BufferKind::ElementArrayBuffer,
            usage:        BufferUsage::StaticDraw,
            buffer_cpu:   data,
            buffer_gpu:   None,
            needs_update: false,
        }
    }

    #[inline]
    pub fn set<T>(&mut self, byte_offset: usize, value: &[T]) {
        let bytes = to_bytes(&value);
        self.buffer_cpu[byte_offset..byte_offset + bytes.len()].copy_from_slice(bytes);
        self.needs_update = true;
    }

    #[inline(always)]
    pub fn on_before_render(&mut self, gl: &GL) {
        if self.buffer_gpu.is_none() {
            self.create_buffer_gpu(gl);
        }

        if self.needs_update {
            self.update_buffer_gpu(gl);
            self.needs_update = false;
        }
    }

    pub fn bind(&self, gl: &GL) {
        gl.bind_buffer(self.kind as u32, self.buffer_gpu.as_ref());
    }

    pub fn size(&self) -> usize {
        self.buffer_cpu.len()
    }

    fn create_buffer_gpu(&mut self, gl: &GL) {
        let webgl_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(self.kind as u32, Some(&webgl_buffer));
        gl.buffer_data_with_u8_array(self.kind as u32, &self.buffer_cpu, self.usage as u32);
        self.buffer_gpu = Some(webgl_buffer);
    }

    fn update_buffer_gpu(&self, gl: &GL) {
        gl.bind_buffer(self.kind as u32, self.buffer_gpu.as_ref());
        gl.buffer_sub_data_with_i32_and_u8_array(self.kind as u32, 0, &self.buffer_cpu);
    }
}
