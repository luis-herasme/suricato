use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

use crate::utils::to_bytes;

#[derive(Debug, Clone)]
pub struct BufferGPU {
    buffer_cpu:   Vec<u8>,
    buffer_gpu:   Option<WebGlBuffer>,
    needs_update: bool,
}

impl BufferGPU {
    pub fn new(data: Vec<u8>) -> BufferGPU {
        BufferGPU {
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
        gl.bind_buffer(GL::ARRAY_BUFFER, self.buffer_gpu.as_ref());
    }

    pub fn size(&self) -> usize {
        self.buffer_cpu.len()
    }

    fn create_buffer_gpu(&mut self, gl: &GL) {
        let webgl_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&webgl_buffer));
        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, &self.buffer_cpu, GL::STATIC_DRAW);
        self.buffer_gpu = Some(webgl_buffer);
    }

    fn update_buffer_gpu(&self, gl: &GL) {
        gl.bind_buffer(GL::ARRAY_BUFFER, self.buffer_gpu.as_ref());
        gl.buffer_sub_data_with_i32_and_u8_array(GL::ARRAY_BUFFER, 0, &self.buffer_cpu);
    }
}
