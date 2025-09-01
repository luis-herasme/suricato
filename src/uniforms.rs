use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram};

use crate::{
    texture::Texture,
    utils::{js_array_to_vec_u32, js_value_to_vec_u32, to_bytes},
};

pub enum Uniform {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),

    Int(i32),
    IntVec2([i32; 2]),
    IntVec3([i32; 3]),
    IntVec4([i32; 4]),

    UnsignedInt(u32),
    UnsignedIntVec2([u32; 2]),
    UnsignedIntVec3([u32; 3]),
    UnsignedIntVec4([u32; 4]),

    Mat2([f32; 4]),
    Mat3([f32; 9]),
    Mat4([f32; 16]),

    Texture(Texture),
}

pub struct UnifromBlockElementLayout {
    pub name:            String,
    /// WebGl type of the uniform
    pub kind:            u32,
    /// Offset in bytes of the element on the Uniform Buffer
    pub byte_offset:     u32,
    /// Number of elements on the array. If this uniform is not of type array
    /// then this value will be equal to 1
    pub array_dimension: u32,
}

pub struct UniformBlock {
    pub name:          String,
    pub binding_point: u32,
    pub buffer_cpu:    Vec<u8>,
    pub buffer_gpu:    WebGlBuffer,
    pub needs_update:  bool,
    pub layout:        Vec<UnifromBlockElementLayout>,
}

impl UniformBlock {
    fn get_layout(gl: &GL, program: &WebGlProgram, uniform_block_index: u32) -> Vec<UnifromBlockElementLayout> {
        let indices = gl
            .get_active_uniform_block_parameter(program, uniform_block_index, GL::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES)
            .unwrap();

        let uniform_types = gl.get_active_uniforms(program, &indices, GL::UNIFORM_TYPE);
        let uniform_offsets = gl.get_active_uniforms(program, &indices, GL::UNIFORM_OFFSET);
        let uniform_array_dimension = gl.get_active_uniforms(program, &indices, GL::UNIFORM_SIZE);

        let uniform_types = js_array_to_vec_u32(uniform_types);
        let uniform_offsets = js_array_to_vec_u32(uniform_offsets);
        let uniform_array_dimension = js_array_to_vec_u32(uniform_array_dimension);

        let indices = js_value_to_vec_u32(indices);

        let mut layout = Vec::new();

        for (i, uniform_index) in indices.into_iter().enumerate() {
            let uniform = gl.get_active_uniform(program, uniform_index).unwrap();

            layout.push(UnifromBlockElementLayout {
                name:            uniform.name(),
                kind:            uniform_types[i],
                byte_offset:     uniform_offsets[i],
                array_dimension: uniform_array_dimension[i],
            });
        }

        layout
    }

    #[inline(always)]
    pub fn set_buffer<T>(&mut self, byte_offset: usize, value: &[T]) {
        let bytes = to_bytes(&value);
        self.buffer_cpu[byte_offset..byte_offset + bytes.len()].copy_from_slice(bytes);
        self.needs_update = true;
    }

    #[inline(always)]
    pub fn set_property<T>(&mut self, property_name: &str, value: &[T]) {
        let layout = self.get_element_layout(property_name).unwrap();
        let byte_offset = layout.byte_offset as usize;
        self.set_buffer(byte_offset, value);
    }

    #[inline(always)]
    fn get_element_layout(&self, name: &str) -> Option<&UnifromBlockElementLayout> {
        for element_layout in &self.layout {
            if element_layout.name == name {
                return Some(&element_layout);
            }
        }

        None
    }

    pub fn update_gpu(&self, gl: &GL) {
        gl.bind_buffer_base(GL::UNIFORM_BUFFER, self.binding_point, Some(&self.buffer_gpu));
        gl.buffer_data_with_u8_array(GL::UNIFORM_BUFFER, &self.buffer_cpu, GL::DYNAMIC_DRAW);
    }
}

pub struct UniformBlockManager {
    gl:                        GL,
    uniform_blocks:            Vec<UniformBlock>,
    current_ubo_binding_point: u32,
}

impl UniformBlockManager {
    pub fn new(gl: &GL) -> UniformBlockManager {
        UniformBlockManager {
            gl:                        gl.clone(),
            uniform_blocks:            Vec::new(),
            current_ubo_binding_point: 0,
        }
    }

    pub fn update(&self) {
        for uniform_block in &self.uniform_blocks {
            if uniform_block.needs_update {
                uniform_block.update_gpu(&self.gl);
            }
        }
    }

    pub fn set_block_property<T>(&mut self, name: &str, property_name: &str, value: &[T]) {
        if let Some(uniform_block) = self.get_uniform_block(name) {
            uniform_block.set_property(property_name, value);
        }
    }

    pub fn set_block<T>(&mut self, name: &str, byte_offset: usize, value: &[T]) {
        if let Some(uniform_block) = self.get_uniform_block(name) {
            uniform_block.set_buffer(byte_offset, value);
        }
    }

    pub fn create_uniform_blocks_from_program(&mut self, program: &WebGlProgram) {
        let number_of_uniform_blocks = self.gl.get_program_parameter(program, GL::ACTIVE_UNIFORM_BLOCKS).as_f64().unwrap() as u32;

        for uniform_block_index in 0..number_of_uniform_blocks {
            let name = self.gl.get_active_uniform_block_name(program, uniform_block_index).unwrap();

            let gl = self.gl.clone();
            match self.get_uniform_block(&name) {
                Some(uniform_block) => {
                    gl.uniform_block_binding(program, uniform_block_index, uniform_block.binding_point);
                }
                None => {
                    self.create_uniform_block(program, uniform_block_index);
                }
            }
        }
    }

    fn create_uniform_block(&mut self, program: &WebGlProgram, uniform_block_index: u32) {
        let binding_point = self.current_ubo_binding_point;
        self.current_ubo_binding_point += 1;

        self.gl.uniform_block_binding(program, uniform_block_index, binding_point);

        let buffer_gpu = self.gl.create_buffer().unwrap();

        self.gl.bind_buffer_base(GL::UNIFORM_BUFFER, binding_point, Some(&buffer_gpu));

        let uniform_block_buffer_size = self
            .gl
            .get_active_uniform_block_parameter(program, uniform_block_index, GL::UNIFORM_BLOCK_DATA_SIZE)
            .unwrap()
            .as_f64()
            .unwrap() as usize;

        let buffer_cpu = vec![0; uniform_block_buffer_size];

        self.gl.buffer_data_with_u8_array(GL::UNIFORM_BUFFER, &buffer_cpu, GL::DYNAMIC_DRAW);

        let name = self.gl.get_active_uniform_block_name(program, uniform_block_index).unwrap();
        let layout = UniformBlock::get_layout(&self.gl, program, uniform_block_index);

        let uniform_block = UniformBlock {
            name,
            binding_point: 0,
            buffer_cpu,
            buffer_gpu,
            layout,
            needs_update: false,
        };

        self.uniform_blocks.push(uniform_block);
    }

    #[inline]
    fn get_uniform_block(&mut self, name: &str) -> Option<&mut UniformBlock> {
        for uniform_block in &mut self.uniform_blocks {
            if uniform_block.name == name {
                return Some(uniform_block);
            }
        }

        return None;
    }
}
