use web_sys::{WebGl2RenderingContext, WebGlBuffer, js_sys};

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
    pub fn number_of_bytes(&self) -> u8 {
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
    pub divisor:         u32,
}

fn to_bytes<T>(slice: &[T]) -> &[u8] {
    let len = slice.len() * std::mem::size_of::<T>();
    unsafe {
        return std::slice::from_raw_parts(slice.as_ptr() as *const u8, len);
    }
}

pub enum VertexData {
    Float(Vec<f32>),
    Vec2(Vec<[f32; 2]>),
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),
    Mat4(Vec<[f32; 16]>),
}

impl VertexData {
    pub fn count(&self) -> usize {
        match &self {
            VertexData::Float(data) => data.len(),
            VertexData::Vec2(data) => data.len(),
            VertexData::Vec3(data) => data.len(),
            VertexData::Vec4(data) => data.len(),
            VertexData::Mat4(data) => data.len(),
        }
    }

    fn component_count(&self) -> u8 {
        match &self {
            VertexData::Float { .. } => 1,
            VertexData::Vec2 { .. } => 2,
            VertexData::Vec3 { .. } => 3,
            VertexData::Vec4 { .. } => 4,
            VertexData::Mat4 { .. } => 16,
        }
    }

    fn component_type(&self) -> AttributeComponentType {
        match &self {
            VertexData::Float { .. } => AttributeComponentType::Float,
            VertexData::Vec2 { .. } => AttributeComponentType::Float,
            VertexData::Vec3 { .. } => AttributeComponentType::Float,
            VertexData::Vec4 { .. } => AttributeComponentType::Float,
            VertexData::Mat4 { .. } => AttributeComponentType::Float,
        }
    }

    fn to_bytes(&self) -> &[u8] {
        match &self {
            VertexData::Float(data) => to_bytes(data),
            VertexData::Vec2(data) => to_bytes(data),
            VertexData::Vec3(data) => to_bytes(data),
            VertexData::Vec4(data) => to_bytes(data),
            VertexData::Mat4(data) => to_bytes(data),
        }
    }

    fn write_vertex_bytes(&self, vertex_index: usize, buffer: &mut Vec<u8>) {
        match self {
            VertexData::Float(data) => {
                buffer.extend_from_slice(&data[vertex_index].to_ne_bytes());
            }
            VertexData::Vec2(data) => {
                buffer.extend_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::Vec3(data) => {
                buffer.extend_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::Vec4(data) => {
                buffer.extend_from_slice(to_bytes(&data[vertex_index]));
            }
            VertexData::Mat4(data) => {
                buffer.extend_from_slice(to_bytes(&data[vertex_index]));
            }
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
            component_count: data.component_count(),
            component_type:  data.component_type(),
            normalize:       false,
            stride:          0,
            offset:          0,
            divisor:         0,
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            data:         vec![data],
            layout:       vec![layout],
        }
    }

    pub fn single_attribute_with_divisor(name: &str, data: VertexData, divisor: u32) -> VertexBuffer {
        let layout = AttributeLayout {
            name:            String::from(name),
            component_count: data.component_count(),
            component_type:  data.component_type(),
            normalize:       false,
            stride:          0,
            offset:          0,
            divisor:         divisor,
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
                _ => panic!("Invalid type"),
            };

            self.needs_update = true;
            return true;
        }

        false
    }

    pub fn set_vertex_at_mat4(&mut self, name: &str, vertex_index: usize, value: [f32; 16]) -> bool {
        for attribute_index in 0..self.data.len() {
            if self.layout[attribute_index].name != name {
                continue;
            }

            match &mut self.data[attribute_index] {
                VertexData::Mat4(data) => {
                    data[vertex_index] = value;
                }
                _ => panic!("Invalid type"),
            };

            self.needs_update = true;
            return true;
        }

        false
    }

    pub fn update_webgl_buffer(&self, gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer) {
        let buffer = if self.data.len() == 1 {
            self.data[0].to_bytes()
        } else {
            &self.build_bytes_buffer()
        };

        VertexBuffer::set_buffer(gl, webgl_buffer, buffer);
    }

    pub fn build_bytes_buffer(&self) -> Vec<u8> {
        let vertex_count = self.data.get(0).unwrap().count();
        let stride = self.layout.get(0).unwrap().stride as usize;

        let mut buffer = Vec::with_capacity(stride * vertex_count);

        for vertex_index in 0..vertex_count {
            for attribute_data in self.data.iter() {
                attribute_data.write_vertex_bytes(vertex_index, &mut buffer);
            }
        }

        buffer
    }

    fn set_buffer(gl: &WebGl2RenderingContext, webgl_buffer: &WebGlBuffer, bytes: &[u8]) {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));

        unsafe {
            let data = js_sys::Uint8Array::view(bytes);
            gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        }
    }

    fn attribute_data_array_to_attribute_layout_array(attributes: &Vec<(String, VertexData)>) -> Vec<AttributeLayout> {
        let mut attribute_layout_array = Vec::new();
        let mut offset = 0;

        for (name, attribute) in attributes {
            let layout = AttributeLayout {
                name:            String::from(name),
                component_count: attribute.component_count(),
                component_type:  attribute.component_type(),
                normalize:       false,
                offset:          offset,
                stride:          0, // Will be populated after the loop
                divisor:         0,
            };

            offset += attribute.component_count() * attribute.component_type().number_of_bytes();
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
