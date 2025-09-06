use crate::buffer_gpu::BufferGPU;

pub struct UniformBufferObject {
    pub binding_point: Option<usize>,
    pub buffer:        BufferGPU,
}
