use std::collections::HashMap;

use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    geometry::{Geometry, GeometryResource},
    material::{Material, MaterialResource},
};

pub struct Renderer {
    pub gl:     WebGl2RenderingContext,
    pub canvas: HtmlCanvasElement,

    // Resources
    materials:  HashMap<u64, MaterialResource>,
    geometries: HashMap<u64, GeometryResource>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        document.body().unwrap().append_child(&canvas).unwrap();

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
            geometries: HashMap::new(),
        }
    }

    #[rustfmt::skip]
    pub fn render(&mut self, material: &Material, geometry: &Geometry) {
        if !self.materials.contains_key(&material.id) {
            self.create_material_resource(material);
        }

        if !self.geometries.contains_key(&geometry.id) {
            self.create_geometry_resource(geometry);
        }

        let material_resource = self.materials.get(&material.id).unwrap();
        let geometry_resource = self.geometries.get(&geometry.id).unwrap();

        material_resource.use_program();

        // Set uniforms
        for (name, uniform) in &material.uniforms {
            material_resource.set_uniform(name, uniform);
        }

        // Set attributes
        for attribute in &geometry_resource.attributes {
            material_resource.set_attribute_buffer(attribute);
        }

        if let Some(indices) = &geometry_resource.indices {
            self.gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&indices.buffer)
            );
            self.gl.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                indices.count,
                indices.kind,
                indices.offset
            );
        } else {
            self.gl.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                0,
                0
            );
        }
    }

    fn create_material_resource(&mut self, material: &Material) {
        let resource = MaterialResource::new(&self.gl, material).unwrap();
        self.materials.insert(material.id, resource);
    }

    fn create_geometry_resource(&mut self, geometry: &Geometry) {
        let resource = GeometryResource::new(&self.gl, geometry);
        self.geometries.insert(geometry.id, resource);
    }
}
