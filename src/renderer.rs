use std::collections::HashMap;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture, WebGlVertexArrayObject};
use web_sys::{WebGlBuffer, wasm_bindgen::JsCast};

use crate::{
    geometry::Geometry, material::{Material, MaterialResource}, mesh::{Mesh, MeshId}, texture::TextureData, ubo::UniformBufferObject, uniforms::Uniform
};

pub struct App {
    pub gl:     WebGl2RenderingContext,
    pub canvas: HtmlCanvasElement,

    // Resources
    vaos:                   HashMap<MeshId, WebGlVertexArrayObject>,
    materials:              HashMap<u64, MaterialResource>,
    webgl_buffers:          HashMap<u64, WebGlBuffer>,
    webgl_textures:         HashMap<u64, WebGlTexture>,
    uniform_buffer_objects: Vec<UniformBufferObject>,
}

impl App {
    pub fn new() -> App {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        document.body().unwrap().append_child(&canvas).unwrap();
        canvas.set_width(800);
        canvas.set_height(800);

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        App {
            gl,
            canvas,
            vaos: HashMap::new(),
            materials: HashMap::new(),
            webgl_buffers: HashMap::new(),
            webgl_textures: HashMap::new(),
            uniform_buffer_objects: Vec::new()
        }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    #[rustfmt::skip]
    pub fn render(&mut self, mesh: &mut Mesh) {
        self.geometry_buffers_create(&mut mesh.geometry);

        if !self.materials.contains_key(&mesh.material.id) {
            self.compile_material(&mut mesh.material);
        }

        let material_resource = self.materials.get(&mesh.material.id).unwrap();

        material_resource.use_program();

        for uniform_buffer_object in &self.uniform_buffer_objects {
            uniform_buffer_object.update(&self.gl);
        }

        for (name, ubo_binding_point) in mesh.material.commands.drain(..) {
            material_resource.set_uniform_block(&name, ubo_binding_point);
        }

        // Set uniforms
        let mut current_texture_unit = 0;
        for (name, uniform) in &mesh.material.uniforms {
            material_resource.set_uniform(&name, &uniform, current_texture_unit);
            current_texture_unit = current_texture_unit + 1;

            if let Uniform::Texture(texture) = uniform {
                if !self.webgl_textures.contains_key(&texture.id) {
                    let webgl_texture = self.gl.create_texture().unwrap();
                    self.webgl_textures.insert(texture.id, webgl_texture);

                    // Setup texture
                    let webgl_texture = self.webgl_textures.get(&texture.id).unwrap();
                    self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(webgl_texture));

                    match &texture.texture_data {
                        TextureData::HtmlImageElement(source) => {
                            self.gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                                WebGl2RenderingContext::TEXTURE_2D,
                                0,
                                texture.internal_format as i32,
                                texture.format as u32,
                                texture.data_type as u32,
                                source
                            ).unwrap();
                        },
                        TextureData::ImagePixelData(data) => {
                            self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                                WebGl2RenderingContext::TEXTURE_2D,
                                0,
                                texture.internal_format as i32,
                                data.width as i32,
                                data.height as i32,
                                0,
                                texture.format as u32,
                                texture.data_type as u32,
                                Some(&data.bytes)
                            ).unwrap();
                        }
                    }
               

                    self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, texture.minification_filter as i32);
                    self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, texture.magnification_filter as i32);
                }

                let webgl_texture = self.webgl_textures.get(&texture.id).unwrap();
                self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(webgl_texture));
            }
        }

        if !self.vaos.contains_key(&mesh.id) {
            let vao = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(&vao));

            // Set attributes
            for vertex_buffer in &mesh.geometry.vertex_buffers {
                let buffer = self.webgl_buffers.get(&vertex_buffer.id).unwrap(); // Created at create_geometry_resource
                self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
                material_resource.set_attribute_buffer(&vertex_buffer.layout);
            }

            for vertex_buffer in &mesh.geometry.interleaved_vertex_buffers {
                let buffer = self.webgl_buffers.get(&vertex_buffer.id).unwrap(); // Created at create_geometry_resource
                self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
                for vertex_layout in &vertex_buffer.layouts {
                    material_resource.set_attribute_buffer(vertex_layout);
                }
            }

            self.gl.bind_vertex_array(None);
            self.vaos.insert(mesh.id, vao);
        }

        let vao = self.vaos.get(&mesh.id);
        self.gl.bind_vertex_array(vao);

        if let Some(indices) = &mesh.geometry.indices {
            let index_webgl_buffer = self.webgl_buffers.get(&indices.id).unwrap();  // Created at create_geometry_resource

            self.gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&index_webgl_buffer)
            );

            if let Some(instance_count) = mesh.geometry.instance_count {
                self.gl.draw_elements_instanced_with_i32(
                    mesh.render_primitive as u32,
                    indices.layout.count,
                    indices.layout.kind,
                    indices.layout.offset,
                    instance_count as i32
                );
            } else {
                self.gl.draw_elements_with_i32(
                    mesh.render_primitive as u32,
                    indices.layout.count,
                    indices.layout.kind,
                    indices.layout.offset
                );
            }
        } else {
            self.gl.draw_arrays(
                mesh.render_primitive as u32,
                0,
                mesh.geometry.vertex_count as i32
            );
        }

    }

    pub fn compile_material(&mut self, material: &Material) {
        let resource = MaterialResource::new(&self.gl, material).unwrap();
        self.materials.insert(material.id, resource);
    }

    fn geometry_buffers_create(&mut self, geometry: &mut Geometry) {
        for vertex_buffer in &mut geometry.vertex_buffers {
            if !self.webgl_buffers.contains_key(&vertex_buffer.id) {
                let webgl_buffer = self.create_webgl_buffer(&vertex_buffer.data);
                self.webgl_buffers.insert(vertex_buffer.id, webgl_buffer);
            }

            if vertex_buffer.needs_update {
                self.update_webgl_buffer(&vertex_buffer.id, &vertex_buffer.data);
                vertex_buffer.needs_update = false;
            }
        }

        for interleaved_vertex_buffer in &mut geometry.interleaved_vertex_buffers {
            if !self.webgl_buffers.contains_key(&interleaved_vertex_buffer.id) {
                let webgl_buffer = self.create_webgl_buffer(&interleaved_vertex_buffer.data);
                self.webgl_buffers.insert(interleaved_vertex_buffer.id, webgl_buffer);
            }

            if interleaved_vertex_buffer.needs_update {
                self.update_webgl_buffer(&interleaved_vertex_buffer.id, &interleaved_vertex_buffer.data);
                interleaved_vertex_buffer.needs_update = false;
            }
        }

        if let Some(indices) = &geometry.indices {
            if !self.webgl_buffers.contains_key(&indices.id) {
                let buffer = indices.create_webgl_buffer(&self.gl);
                self.webgl_buffers.insert(indices.id, buffer);
            }
        }
    }

    fn create_webgl_buffer(&self, data: &[u8]) -> WebGlBuffer {
        let webgl_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));
        self.gl.buffer_data_with_u8_array(WebGl2RenderingContext::ARRAY_BUFFER, data, WebGl2RenderingContext::STATIC_DRAW);
        webgl_buffer
    }

    fn update_webgl_buffer(&self, webgl_buffer_id: &u64, data: &[u8]) {
        let webgl_buffer = self.webgl_buffers.get(webgl_buffer_id).unwrap();
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));
        self.gl.buffer_sub_data_with_i32_and_u8_array(WebGl2RenderingContext::ARRAY_BUFFER, 0, data);
    }

    /// UBO
    pub fn create_ubo(&mut self) -> u32 {
        let ubo_binding_point = self.uniform_buffer_objects.len() as u32;
        let ubo = UniformBufferObject::new(&self.gl, ubo_binding_point);
        self.uniform_buffer_objects.push(ubo);
        ubo_binding_point
    }

    pub fn set_ubo(&mut self, ubo_binding_point: u32, value: Vec<u8>) {
        self.uniform_buffer_objects[ubo_binding_point as usize].set_buffer(value);
    }
}
