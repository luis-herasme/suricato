use std::collections::HashMap;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use web_sys::{WebGlBuffer, wasm_bindgen::JsCast};

use crate::{
    geometry::Geometry,
    material::{Material, MaterialResource},
};

pub struct Renderer {
    pub gl:     WebGl2RenderingContext,
    pub canvas: HtmlCanvasElement,

    // Resources
    materials:     HashMap<u64, MaterialResource>,
    webgl_buffers: HashMap<u64, WebGlBuffer>,
}

impl Renderer {
    pub fn new() -> Renderer {
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

        Renderer {
            gl,
            canvas,
            materials: HashMap::new(),
            webgl_buffers: HashMap::new(),
        }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    #[rustfmt::skip]
    pub fn render(&mut self, material: &mut Material, geometry: &mut Geometry) {
        self.create_geometry_resource(geometry);

        if !self.materials.contains_key(&material.id) {
            self.create_material_resource(material);
        }

        let material_resource = self.materials.get(&material.id).unwrap();

        material_resource.use_program();

        // Set uniforms
        for (name, uniform) in &material.uniforms {
            material_resource.set_uniform(name, uniform);
        }

        // Set attributes
        for vertex_data in &geometry.vertex_data {
            let buffer = self.webgl_buffers.get(&vertex_data.id()).unwrap(); // Created at create_geometry_resource
            material_resource.set_attribute_buffer(vertex_data, buffer);
        }

        if let Some(indices) = &geometry.indices {
            let index_webgl_buffer = self.webgl_buffers.get(&indices.id).unwrap();  // Created at create_geometry_resource

            let indices = geometry.indices.as_ref().unwrap();

            self.gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&index_webgl_buffer)
            );

            self.gl.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                indices.layout.count,
                indices.layout.kind,
                indices.layout.offset
            );
        } else {
            self.gl.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                0,
                geometry.vertex_count
            );
        }
    }

    fn create_material_resource(&mut self, material: &Material) {
        let resource = MaterialResource::new(&self.gl, material).unwrap();
        self.materials.insert(material.id, resource);
    }

    fn create_geometry_resource(&mut self, geometry: &mut Geometry) {
        for vertex_data in &mut geometry.vertex_data {
            let id = vertex_data.id();

            if !self.webgl_buffers.contains_key(&id) {
                let webgl_buffer = self.gl.create_buffer().unwrap();
                self.webgl_buffers.insert(id, webgl_buffer);
            }

            if vertex_data.needs_update() {
                let webgl_buffer = self.webgl_buffers.get(&vertex_data.id()).unwrap();
                vertex_data.update_webgl_buffer(&self.gl, &webgl_buffer);
                vertex_data.set_needs_update(false);
            }
        }

        if let Some(indices) = &geometry.indices {
            if !self.webgl_buffers.contains_key(&indices.id) {
                let buffer = indices.create_webgl_buffer(&self.gl);
                self.webgl_buffers.insert(indices.id, buffer);
            }
        }
    }
}
