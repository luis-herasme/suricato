use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::{uniforms::Uniform, vertex_buffer::VertexLayout};

#[derive(Debug)]
pub enum MaterialError {
    // Program
    ProgramCreationFailed,
    ProgramLinkingFailed(Option<String>),

    // Shader
    ShaderCreationFailed,
    ShaderCompilationFailed(Option<String>),
}

pub struct Material {
    pub uniforms: HashMap<String, Uniform>,

    // WebGL resouces
    gl:                      GL,
    program:                 WebGlProgram,
    uniform_locations:       HashMap<String, WebGlUniformLocation>,
    attribute_locations:     HashMap<String, u32>,
    uniform_block_locations: HashMap<String, u32>,
}

impl Material {
    pub fn new(gl: &GL, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Material, MaterialError> {
        let program = gl.create_program().ok_or_else(|| MaterialError::ProgramCreationFailed)?;

        let vertex_shader = Material::compile_shader(&gl, vertex_shader_source, GL::VERTEX_SHADER)?;
        let fragment_shader = Material::compile_shader(&gl, fragment_shader_source, GL::FRAGMENT_SHADER)?;

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        let program_link_status_is_ok = gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false);

        if !program_link_status_is_ok {
            return Err(MaterialError::ProgramLinkingFailed(gl.get_program_info_log(&program)));
        }

        let uniform_locations = Material::get_uniform_locations(&gl, &program);
        let attribute_locations = Material::get_attribute_locations(&gl, &program);
        let uniform_block_locations = Material::get_uniform_block_locations(&gl, &program);

        Ok(Material {
            gl: gl.clone(),
            program,
            uniform_locations,
            attribute_locations,
            uniform_block_locations,
            uniforms: HashMap::new(),
        })
    }

    pub fn set_uniform(&mut self, uniform_name: &str, uniform: Uniform) {
        self.uniforms.insert(String::from(uniform_name), uniform);
    }

    pub fn on_before_render(&mut self, gl: &GL) {
        self.gl.use_program(Some(&self.program));

        // Set uniforms
        let mut current_texture_unit = 0;
        for (name, uniform) in &self.uniforms {
            self.set_uniform_internal(&name, &uniform, current_texture_unit);

            if let Uniform::Texture(texture) = uniform {
                gl.bind_texture(GL::TEXTURE_2D, Some(&texture.webgl_texture));
                current_texture_unit = current_texture_unit + 1;
            }
        }
    }

    fn compile_shader(gl: &GL, shader_source: &str, shader_type: u32) -> Result<WebGlShader, MaterialError> {
        let shader = gl.create_shader(shader_type).ok_or_else(|| MaterialError::ShaderCreationFailed)?;
        gl.shader_source(&shader, &shader_source);
        gl.compile_shader(&shader);
        let shader_status_is_ok = gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false);

        if shader_status_is_ok {
            Ok(shader)
        } else {
            Err(MaterialError::ShaderCompilationFailed(gl.get_shader_info_log(&shader)))
        }
    }

    /// UNIFORMS
    fn set_uniform_internal(&self, uniform_name: &str, uniform: &Uniform, current_texture_unit: u32) {
        let location = self.uniform_locations.get(uniform_name).unwrap();

        match uniform {
            Uniform::Float(v) => self.gl.uniform1f(Some(location), *v),
            Uniform::Vec2(v) => self.gl.uniform2fv_with_f32_array(Some(location), v),
            Uniform::Vec3(v) => self.gl.uniform3fv_with_f32_array(Some(location), v),
            Uniform::Vec4(v) => self.gl.uniform4fv_with_f32_array(Some(location), v),

            Uniform::Int(v) => self.gl.uniform1i(Some(location), *v),
            Uniform::IntVec2(v) => self.gl.uniform2iv_with_i32_array(Some(location), v),
            Uniform::IntVec3(v) => self.gl.uniform3iv_with_i32_array(Some(location), v),
            Uniform::IntVec4(v) => self.gl.uniform4iv_with_i32_array(Some(location), v),

            Uniform::UnsignedInt(v) => self.gl.uniform1ui(Some(location), *v),
            Uniform::UnsignedIntVec2(v) => self.gl.uniform2uiv_with_u32_array(Some(location), v),
            Uniform::UnsignedIntVec3(v) => self.gl.uniform2uiv_with_u32_array(Some(location), v),
            Uniform::UnsignedIntVec4(v) => self.gl.uniform2uiv_with_u32_array(Some(location), v),

            Uniform::Mat2(v) => self.gl.uniform_matrix2fv_with_f32_array(Some(location), false, v),
            Uniform::Mat3(v) => self.gl.uniform_matrix3fv_with_f32_array(Some(location), false, v),
            Uniform::Mat4(v) => self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, v),

            Uniform::Texture(_) => {
                self.gl.uniform1i(Some(location), current_texture_unit as i32);
                self.gl.active_texture(GL::TEXTURE0 + current_texture_unit);
            }
        }
    }

    fn get_uniform_locations(gl: &GL, program: &WebGlProgram) -> HashMap<String, WebGlUniformLocation> {
        let mut uniform_locations = HashMap::new();

        let number_of_uniforms = gl
            .get_program_parameter(&program, GL::ACTIVE_UNIFORMS)
            .as_f64()
            .expect("Unable to get the number of uniforms");

        for i in 0..number_of_uniforms as u32 {
            let uniform = gl.get_active_uniform(&program, i).unwrap();
            let uniform_name = uniform.name();

            // Uniforms inside uniform blocks do not have locations
            if let Some(location) = gl.get_uniform_location(&program, &uniform_name) {
                uniform_locations.insert(uniform_name, location);
            }
        }

        uniform_locations
    }

    /// ATTRIBUTES
    pub fn set_attribute_buffer(&self, vertex_layout: &VertexLayout) {
        if self.attribute_locations.get(&vertex_layout.name).is_none() {
            return;
        }

        let location = self.attribute_locations.get(&vertex_layout.name).unwrap();

        if vertex_layout.number_of_columns == 1 {
            self.gl.enable_vertex_attrib_array(*location);
            self.gl.vertex_attrib_pointer_with_i32(
                *location,
                vertex_layout.component_count as i32,
                vertex_layout.component_type as u32,
                vertex_layout.normalize,
                vertex_layout.stride as i32,
                vertex_layout.offset as i32,
            );

            if vertex_layout.divisor != 0 {
                self.gl.vertex_attrib_divisor(*location, vertex_layout.divisor);
            }

            return;
        }

        // Only matrices have more than one column
        let components_per_column = vertex_layout.component_count / vertex_layout.number_of_columns;

        for i in 0..(vertex_layout.number_of_columns) {
            let column_location = location + i as u32;
            let offset = vertex_layout.offset + (i * components_per_column * vertex_layout.component_type.size_in_bytes()) as usize;

            self.gl.enable_vertex_attrib_array(column_location);
            self.gl.vertex_attrib_pointer_with_i32(
                column_location,
                components_per_column as i32,
                vertex_layout.component_type as u32,
                vertex_layout.normalize,
                vertex_layout.stride as i32,
                offset as i32,
            );

            if vertex_layout.divisor != 0 {
                self.gl.vertex_attrib_divisor(column_location, vertex_layout.divisor);
            }
        }
    }

    fn get_attribute_locations(gl: &GL, program: &WebGlProgram) -> HashMap<String, u32> {
        let mut attribute_locations = HashMap::new();

        let number_of_attributes = gl
            .get_program_parameter(&program, GL::ACTIVE_ATTRIBUTES)
            .as_f64()
            .expect("Unable to get the number of attributes");

        for i in 0..number_of_attributes as u32 {
            let attribute = gl.get_active_attrib(program, i).unwrap();
            let attribute_name = attribute.name();

            let location = gl.get_attrib_location(program, &attribute_name) as u32;
            attribute_locations.insert(attribute_name, location);
        }

        attribute_locations
    }

    /// UNIFORM BLOCKS
    pub fn get_uniform_block_locations(gl: &GL, program: &WebGlProgram) -> HashMap<String, u32> {
        let mut uniform_block_locations = HashMap::new();
        let number_of_uniform_blocks = gl.get_program_parameter(program, GL::ACTIVE_UNIFORM_BLOCKS).as_f64().unwrap() as u32;

        for uniform_block_location in 0..number_of_uniform_blocks {
            let name = gl.get_active_uniform_block_name(program, uniform_block_location).unwrap();
            uniform_block_locations.insert(name, uniform_block_location);
        }

        uniform_block_locations
    }

    pub fn set_uniform_block(&self, name: &str, ubo_binding_point: u32) {
        let Some(block_location) = self.uniform_block_locations.get(name) else {
            return;
        };

        self.gl.uniform_block_binding(&self.program, *block_location, ubo_binding_point);
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}
