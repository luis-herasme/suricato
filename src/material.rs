use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::{uniforms::Uniform, utils::generate_id, vertex_buffer::VertexBuffer};

pub struct Material {
    pub id:                 u64,
    pub uniforms:           HashMap<String, Uniform>,
    vertex_shader_source:   String,
    fragment_shader_source: String,
}

impl Material {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Material {
        Material {
            id:                     generate_id(),
            vertex_shader_source:   String::from(vertex_shader_source),
            fragment_shader_source: String::from(fragment_shader_source),
            uniforms:               HashMap::new(),
        }
    }

    pub fn set_uniform(&mut self, uniform_name: &str, uniform: Uniform) {
        self.uniforms.insert(String::from(uniform_name), uniform);
    }
}

pub struct MaterialResource {
    gl:                  WebGl2RenderingContext,
    program:             WebGlProgram,
    uniform_locations:   HashMap<String, WebGlUniformLocation>,
    attribute_locations: HashMap<String, u32>,
}

impl MaterialResource {
    pub fn new(gl: &WebGl2RenderingContext, material: &Material) -> Result<MaterialResource, String> {
        let webgl_program = gl.create_program().ok_or_else(|| String::from("Could not create program"))?;

        let vertex_shader = MaterialResource::compile_shader(&gl, &material.vertex_shader_source, WebGl2RenderingContext::VERTEX_SHADER)?;
        let fragment_shader =
            MaterialResource::compile_shader(&gl, &material.fragment_shader_source, WebGl2RenderingContext::FRAGMENT_SHADER)?;

        gl.attach_shader(&webgl_program, &vertex_shader);
        gl.attach_shader(&webgl_program, &fragment_shader);
        gl.link_program(&webgl_program);

        let program_link_status_is_ok = gl
            .get_program_parameter(&webgl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);

        if program_link_status_is_ok {
            let uniform_locations = MaterialResource::get_uniform_locations(&gl, &webgl_program);
            let attribute_locations = MaterialResource::get_attribute_locations(&gl, &webgl_program);

            Ok(MaterialResource {
                gl: gl.clone(),
                program: webgl_program,
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

    pub fn use_program(&self) {
        self.gl.use_program(Some(&self.program));
    }

    /// UNIFORMS
    pub fn set_uniform(&self, uniform_name: &str, uniform: &Uniform) {
        let location = self.uniform_locations.get(uniform_name).unwrap();

        match uniform {
            Uniform::Float(v) => self.gl.uniform1f(Some(location), *v),
            Uniform::Vec2(v) => self.gl.uniform2fv_with_f32_array(Some(location), v),
            Uniform::Vec3(v) => self.gl.uniform3fv_with_f32_array(Some(location), v),
            Uniform::Vec4(v) => self.gl.uniform4fv_with_f32_array(Some(location), v),
            Uniform::Mat4(v) => self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, v),
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
    pub fn set_attribute_buffer(&self, attributes: &VertexBuffer, buffer: &WebGlBuffer) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        for attribute in &attributes.layout {
            let location = self.attribute_locations.get(&attribute.name).unwrap();

            if attribute.component_count <= 4 {
                self.gl.enable_vertex_attrib_array(*location);
                self.gl.vertex_attrib_pointer_with_i32(
                    *location,
                    attribute.component_count as i32,
                    attribute.component_type as u32,
                    attribute.normalize,
                    attribute.stride as i32,
                    attribute.offset as i32,
                );

                if attribute.divisor != 0 {
                    self.gl.vertex_attrib_divisor(*location, attribute.divisor);
                }
            } else {
                for i in 0..(attribute.component_count as u32 / 4) {
                    self.gl.enable_vertex_attrib_array(*location + i);
                    self.gl.vertex_attrib_pointer_with_i32(
                        *location + i,
                        4,
                        attribute.component_type as u32,
                        attribute.normalize,
                        (attribute.component_count * attribute.component_type.size_in_bytes()) as i32,
                        (i as u8 * 4 * attribute.component_type.size_in_bytes()) as i32,
                    );

                    if attribute.divisor != 0 {
                        self.gl.vertex_attrib_divisor(*location + i, attribute.divisor);
                    }
                }
            }
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
            attribute_locations.insert(attribute_name, location);
        }

        attribute_locations
    }
}
