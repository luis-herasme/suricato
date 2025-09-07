use web_sys::WebGl2RenderingContext;

use crate::{
    buffer_gpu::{BufferError, BufferGPU, BufferKind, BufferUsage},
    renderer::Renderer,
};

pub struct UniformBufferObject {
    pub gl:            WebGl2RenderingContext,
    pub binding_point: Option<u32>,
    pub buffer:        BufferGPU,
}

impl UniformBufferObject {
    pub fn new(renderer: &Renderer, buffer_cpu: &[u8]) -> Result<UniformBufferObject, BufferError> {
        Ok(UniformBufferObject {
            gl:            renderer.gl.clone(),
            binding_point: None,
            buffer:        BufferGPU::new(
                renderer.gl.clone(),
                BufferKind::UniformBuffer,
                BufferUsage::DynamicDraw,
                buffer_cpu.to_vec(),
            )?,
        })
    }

    pub fn set_binding_point(&mut self, binding_point: u32) {
        self.binding_point = Some(binding_point);
        self.gl
            .bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, binding_point, Some(&self.buffer.buffer_gpu));
    }

    pub fn set_bytes(&mut self, byte_offset: usize, data: &[u8]) {
        self.buffer.set_bytes(byte_offset, data);
        self.buffer.on_before_render();
    }
}
