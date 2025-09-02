use crate::utils::to_bytes;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer};

pub struct UniformBufferObject {
    binding_point: u32,
    buffer_cpu:    Vec<u8>,
    buffer_gpu:    WebGlBuffer,
    needs_update:  bool,
}

impl UniformBufferObject {
    pub fn new(gl: &GL, binding_point: u32) -> UniformBufferObject {
        let buffer_gpu = gl.create_buffer().unwrap();
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, binding_point, Some(&buffer_gpu));

        UniformBufferObject {
            binding_point,
            buffer_gpu,
            buffer_cpu: Vec::new(),
            needs_update: true,
        }
    }

    pub fn set_buffer(&mut self, value: Vec<u8>) {
        self.buffer_cpu = value;
        self.needs_update = true;
    }

    #[inline(always)]
    pub fn update_buffer<T>(&mut self, byte_offset: usize, value: &[T]) {
        let bytes = to_bytes(&value);
        self.buffer_cpu[byte_offset..byte_offset + bytes.len()].copy_from_slice(bytes);
        self.needs_update = true;
    }

    pub fn update(&self, gl: &GL) {
        if self.needs_update {
            gl.bind_buffer_base(GL::UNIFORM_BUFFER, self.binding_point, Some(&self.buffer_gpu));
            gl.buffer_data_with_u8_array(GL::UNIFORM_BUFFER, &self.buffer_cpu, GL::DYNAMIC_DRAW);
        }
    }
}
