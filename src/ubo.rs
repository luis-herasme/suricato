use crate::buffer_gpu::BufferGPU;

pub struct UniformBufferObject {
    pub binding_point: usize,
    pub buffer:        BufferGPU,
}
