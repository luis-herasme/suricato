use web_sys::{
    WebGl2RenderingContext, WebGlBuffer,
    js_sys::{self, ArrayBuffer, DataView},
};

use crate::generate_id::generate_id;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AttributeComponentType {
    Byte          = WebGl2RenderingContext::BYTE,
    UnsignedByte  = WebGl2RenderingContext::UNSIGNED_BYTE,
    Short         = WebGl2RenderingContext::SHORT,
    UnsignedShort = WebGl2RenderingContext::UNSIGNED_SHORT,
    Int           = WebGl2RenderingContext::INT,
    UnsignedInt   = WebGl2RenderingContext::UNSIGNED_INT,
    Float         = WebGl2RenderingContext::FLOAT,
}

impl AttributeComponentType {
    fn number_of_bytes(&self) -> u8 {
        match &self {
            AttributeComponentType::Byte => 1,
            AttributeComponentType::UnsignedByte => 1,
            AttributeComponentType::Short => 2,
            AttributeComponentType::UnsignedShort => 2,
            AttributeComponentType::Int => 4,
            AttributeComponentType::UnsignedInt => 4,
            AttributeComponentType::Float => 4,
        }
    }
}

pub struct AttributeLayout {
    pub name:            String,
    pub component_count: u8,
    pub component_type:  AttributeComponentType,
    pub normalize:       bool,
    pub stride:          u8,
    pub offset:          u8,
}

pub enum VertexData {
    Float(Vec<f32>),
    Vec2(Vec<f32>),
    Vec3(Vec<f32>),
    Vec4(Vec<f32>),
}

impl VertexData {
    pub fn count(&self) -> usize {
        match &self {
            VertexData::Float(data) => data.len(),
            VertexData::Vec2(data) => data.len() / 2,
            VertexData::Vec3(data) => data.len() / 3,
            VertexData::Vec4(data) => data.len() / 4,
        }
    }

    fn number_of_components(&self) -> u8 {
        match &self {
            VertexData::Float { .. } => 1,
            VertexData::Vec2 { .. } => 2,
            VertexData::Vec3 { .. } => 3,
            VertexData::Vec4 { .. } => 4,
        }
    }

    fn component_type(&self) -> AttributeComponentType {
        match &self {
            VertexData::Float { .. } => AttributeComponentType::Float,
            VertexData::Vec2 { .. } => AttributeComponentType::Float,
            VertexData::Vec3 { .. } => AttributeComponentType::Float,
            VertexData::Vec4 { .. } => AttributeComponentType::Float,
        }
    }
}

pub struct VertexBuffer {
    pub id:           u64,
    pub needs_update: bool,
    pub data:         Vec<VertexData>,
    pub layout:       Vec<AttributeLayout>,
}

impl VertexBuffer {
    pub fn single_attribute(name: &str, data: VertexData) -> VertexBuffer {
        let layout = AttributeLayout {
            name:            String::from(name),
            component_count: data.number_of_components(),
            component_type:  data.component_type(),
            normalize:       false,
            stride:          0,
            offset:          0,
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            data:         vec![data],
            layout:       vec![layout],
        }
    }

    pub fn interleaved_attributes(data: Vec<(String, VertexData)>) -> VertexBuffer {
        let layout = VertexBuffer::attribute_data_array_to_attribute_layout_array(&data);
        let data = data.into_iter().map(|x| x.1).collect();

        VertexBuffer {
            id: generate_id(),
            needs_update: true,
            data,
            layout,
        }
    }

    pub fn set_vertex_at_f32(&mut self, name: &str, vertex_index: usize, value: f32) -> bool {
        for attribute_index in 0..self.data.len() {
            if self.layout[attribute_index].name != name {
                continue;
            }

            match &mut self.data[attribute_index] {
                VertexData::Float(data) => data[vertex_index] = value,
                VertexData::Vec2(data) => data[vertex_index] = value,
                VertexData::Vec3(data) => data[vertex_index] = value,
                VertexData::Vec4(data) => data[vertex_index] = value,
            };

            self.needs_update = true;
            return true;
        }

        false
    }

    pub fn update_webgl_buffer(&self, gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer) {
        if self.data.len() == 1 {
            self.update_webgl_buffer_single(gl, webgl_buffer);
        } else {
            self.update_webgl_buffer_interleaved(gl, webgl_buffer);
        }
    }

    pub fn update_webgl_buffer_single(&self, gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer) {
        match &self.data[0] {
            VertexData::Float(data) => VertexBuffer::set_float_buffer(gl, webgl_buffer, data),
            VertexData::Vec2(data) => VertexBuffer::set_float_buffer(gl, webgl_buffer, data),
            VertexData::Vec3(data) => VertexBuffer::set_float_buffer(gl, webgl_buffer, data),
            VertexData::Vec4(data) => VertexBuffer::set_float_buffer(gl, webgl_buffer, data),
        };
    }

    pub fn update_webgl_buffer_interleaved(&self, gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer) {
        let attribute_data = self.data.get(0).unwrap();
        let attribute_layout = self.layout.get(0).unwrap();

        let stride = attribute_layout.stride;
        let buffer_size = stride as u32 * attribute_data.count() as u32;

        let array_buffer = ArrayBuffer::new(buffer_size);
        let data_view = DataView::new(&array_buffer, 0, buffer_size as usize);

        for vertex_index in 0..attribute_data.count() {
            for (attribute_index, attribute_data) in self.data.iter().enumerate() {
                let attribute_description = &self.layout[attribute_index];
                let offset = stride as usize * vertex_index + attribute_description.offset as usize;

                match attribute_data {
                    VertexData::Float(data) => {
                        let value = data[vertex_index];
                        data_view.set_float32_endian(offset, value, true);
                    }
                    VertexData::Vec2(data) => {
                        let vertex_index = vertex_index * 2;

                        let value_0 = data[vertex_index];
                        let value_1 = data[vertex_index + 1];

                        data_view.set_float32_endian(offset, value_0, true);
                        data_view.set_float32_endian(offset + 4, value_1, true);
                    }
                    VertexData::Vec3(data) => {
                        let vertex_index = vertex_index * 3;

                        let value_0 = data[vertex_index];
                        let value_1 = data[vertex_index + 1];
                        let value_2 = data[vertex_index + 2];

                        data_view.set_float32_endian(offset, value_0, true);
                        data_view.set_float32_endian(offset + 4, value_1, true);
                        data_view.set_float32_endian(offset + 8, value_2, true);
                    }
                    VertexData::Vec4(data) => {
                        let vertex_index = vertex_index * 4;

                        let value_0 = data[vertex_index];
                        let value_1 = data[vertex_index + 1];
                        let value_2 = data[vertex_index + 2];
                        let value_3 = data[vertex_index + 3];

                        data_view.set_float32_endian(offset, value_0, true);
                        data_view.set_float32_endian(offset + 4, value_1, true);
                        data_view.set_float32_endian(offset + 8, value_2, true);
                        data_view.set_float32_endian(offset + 12, value_3, true);
                    }
                }
            }
        }

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &js_sys::Uint8Array::new(&array_buffer),
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    fn set_float_buffer(gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer, data: &Vec<f32>) {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));

        unsafe {
            let data = js_sys::Float32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        }
    }

    fn attribute_data_array_to_attribute_layout_array(attributes: &Vec<(String, VertexData)>) -> Vec<AttributeLayout> {
        let mut attribute_layout_array = Vec::new();
        let mut offset = 0;

        for (name, attribute) in attributes {
            let layout = AttributeLayout {
                name:            name.to_string(),
                component_count: attribute.number_of_components(),
                component_type:  attribute.component_type(),
                normalize:       false,
                offset:          offset,
                stride:          0, // Will be populated after the loop
            };

            offset += attribute.number_of_components() * attribute.component_type().number_of_bytes();
            attribute_layout_array.push(layout);
        }

        // After the previous loop, offset will be equal to the stride
        for attribute_layout in &mut attribute_layout_array {
            attribute_layout.stride = offset;
        }

        attribute_layout_array
    }

    pub fn vertex_count(&self) -> usize {
        for attribute_data in &self.data {
            return attribute_data.count();
        }

        unreachable!("VertexBuffer data will never be empty");
    }
}
