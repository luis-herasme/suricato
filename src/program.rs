use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::{
    attributes::{AttributeBuffer, InterleavedAttributeBuffer},
    uniforms::Uniform,
};

pub struct Program {
    gl:                  WebGl2RenderingContext,
    pub webgl_program:   WebGlProgram,
    uniform_locations:   HashMap<String, WebGlUniformLocation>,
    attribute_locations: HashMap<String, u32>,
}

impl Program {
    pub fn new(gl: &WebGl2RenderingContext, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, String> {
        let webgl_program = gl.create_program().ok_or_else(|| String::from("Could not create program"))?;

        let vertex_shader = Program::compile_shader(&gl, vertex_shader_source, WebGl2RenderingContext::VERTEX_SHADER)?;
        let fragment_shader = Program::compile_shader(&gl, fragment_shader_source, WebGl2RenderingContext::FRAGMENT_SHADER)?;

        gl.attach_shader(&webgl_program, &vertex_shader);
        gl.attach_shader(&webgl_program, &fragment_shader);
        gl.link_program(&webgl_program);

        let program_link_status_is_ok = gl
            .get_program_parameter(&webgl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);

        if program_link_status_is_ok {
            let uniform_locations = Program::get_uniform_locations(&gl, &webgl_program);
            let attribute_locations = Program::get_attribute_locations(&gl, &webgl_program);

            Ok(Program {
                gl: gl.clone(),
                webgl_program,
                uniform_locations,
                attribute_locations,
            })
        } else {
            Err(gl
                .get_program_info_log(&webgl_program)
                .unwrap_or_else(|| "Error linking program".to_string()))
        }
    }

    fn compile_shader(gl: &WebGl2RenderingContext, shader_source: &str, shader_type: u32) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader"))?;
        gl.shader_source(&shader, &shader_source);
        gl.compile_shader(&shader);
        let shader_status_is_ok = gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);
        if shader_status_is_ok {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Error creating shader".to_string()))
        }
    }

    /// UNIFORMS
    pub fn set_uniform(&self, uniform_name: &str, unifrom: &Uniform) {
        let location = self.uniform_locations.get(uniform_name).unwrap();

        match unifrom {
            Uniform::Float(v) => self.gl.uniform1f(Some(location), *v),
            Uniform::Vec2(v1, v2) => self.gl.uniform2f(Some(location), *v1, *v2),
            Uniform::Vec3(v1, v2, v3) => self.gl.uniform3f(Some(location), *v1, *v2, *v3),
            Uniform::Vec4(v1, v2, v3, v4) => self.gl.uniform4f(Some(location), *v1, *v2, *v3, *v4),
        }
    }

    fn get_uniform_locations(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> HashMap<String, WebGlUniformLocation> {
        let mut uniform_locations = HashMap::new();

        let number_of_uniforms = gl
            .get_program_parameter(&program, WebGl2RenderingContext::ACTIVE_UNIFORMS)
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
    pub fn set_attribute(&self, name: &str, attribute: &AttributeBuffer) {
        let location = self.attribute_locations.get(name).unwrap();
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&attribute.buffer));
        self.gl.vertex_attrib_pointer_with_i32(
            *location,
            attribute.number_of_components,
            attribute.type_of_the_components as u32,
            attribute.normalize,
            attribute.stride,
            attribute.offset,
        )
    }

    pub fn set_attributes(&self, attributes: &InterleavedAttributeBuffer) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&attributes.buffer));

        for attribute in &attributes.description {
            let location = self.attribute_locations.get(&attribute.name).unwrap();
            self.gl.vertex_attrib_pointer_with_i32(
                *location,
                attribute.number_of_components,
                attribute.type_of_the_components as u32,
                attribute.normalize,
                attribute.stride,
                attribute.offset,
            )
        }
    }

    fn get_attribute_locations(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> HashMap<String, u32> {
        let mut attribute_locations = HashMap::new();

        let number_of_attributes = gl
            .get_program_parameter(&program, WebGl2RenderingContext::ACTIVE_ATTRIBUTES)
            .as_f64()
            .expect("Unable to get the number of attributes");

        for i in 0..number_of_attributes as u32 {
            let attribute = gl.get_active_attrib(program, i).unwrap();
            let attribute_name = attribute.name();

            let location = gl.get_attrib_location(program, &attribute_name) as u32;
            gl.enable_vertex_attrib_array(location);
            attribute_locations.insert(attribute_name, location);
        }

        attribute_locations
    }
}
