use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::uniforms::Uniform;

pub struct Program {
    gl: WebGl2RenderingContext,
    webgl_program: WebGlProgram,
    uniform_locations: HashMap<String, WebGlUniformLocation>,
}

impl Program {
    pub fn new(gl: WebGl2RenderingContext, vertex_shader_source: String, fragment_shader_source: String) -> Result<Program, String> {
        let webgl_program = gl.create_program().ok_or_else(|| String::from("Could not create program"))?;

        let vertex_shader = Program::compile_shader(&gl, &vertex_shader_source, WebGl2RenderingContext::VERTEX_SHADER)?;
        let fragment_shader = Program::compile_shader(&gl, &fragment_shader_source, WebGl2RenderingContext::FRAGMENT_SHADER)?;

        gl.attach_shader(&webgl_program, &vertex_shader);
        gl.attach_shader(&webgl_program, &fragment_shader);
        gl.link_program(&webgl_program);

        let program_link_status_is_ok = gl
            .get_program_parameter(&webgl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);

        if program_link_status_is_ok {
            let uniform_locations = Program::get_uniform_locations(&gl, &webgl_program);

            Ok(Program {
                gl,
                webgl_program,
                uniform_locations,
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

    pub fn set_uniform(&self, uniform_name: &str, unifrom: Uniform) {
        let location = self.uniform_locations.get(uniform_name).unwrap();

        match unifrom {
            Uniform::Float(v) => self.gl.uniform1f(Some(location), v),
            Uniform::FloatVec2(v1, v2) => self.gl.uniform2f(Some(location), v1, v2),
        }
    }
}
