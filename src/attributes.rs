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
    pub component_count: i32,
    pub component_type:  AttributeComponentType,
    pub normalize:       bool,
    pub stride:          i32,
    pub offset:          i32,
}

pub enum AttributeData {
    Float(Vec<f32>),
    Vec2(Vec<(f32, f32)>),
    Vec3(Vec<(f32, f32, f32)>),
    Vec4(Vec<(f32, f32, f32, f32)>),
}

impl AttributeData {
    pub fn vertex_count(&self) -> usize {
        match &self {
            AttributeData::Float(data) => data.len(),
            AttributeData::Vec2(data) => data.len(),
            AttributeData::Vec3(data) => data.len(),
            AttributeData::Vec4(data) => data.len(),
        }
    }

    fn number_of_components(&self) -> u8 {
        match &self {
            AttributeData::Float { .. } => 1,
            AttributeData::Vec2 { .. } => 2,
            AttributeData::Vec3 { .. } => 3,
            AttributeData::Vec4 { .. } => 4,
        }
    }

    fn component_type(&self) -> AttributeComponentType {
        match &self {
            AttributeData::Float { .. } => AttributeComponentType::Float,
            AttributeData::Vec2 { .. } => AttributeComponentType::Float,
            AttributeData::Vec3 { .. } => AttributeComponentType::Float,
            AttributeData::Vec4 { .. } => AttributeComponentType::Float,
        }
    }

    fn element_size_in_bytes(&self) -> u8 {
        self.number_of_components() * self.component_type().number_of_bytes()
    }
}

pub enum VertexData {
    SingleAttribute(SingleAttributeVertexData),
    InterleavedAttributes(InterleavedAttributesVertexData),
}

impl VertexData {
    pub fn vertex_count(&self) -> usize {
        match &self {
            VertexData::SingleAttribute(attribute) => attribute.data.vertex_count(),
            VertexData::InterleavedAttributes(attribute_vector) => {
                for attribute_data in &attribute_vector.data {
                    return attribute_data.vertex_count();
                }

                unreachable!("Interleaved attribute data will never be empty");
            }
        }
    }
}

pub struct SingleAttributeVertexData {
    pub id:     u64,
    pub data:   AttributeData,
    pub layout: AttributeLayout,
}

impl SingleAttributeVertexData {
    pub fn new(name: &str, data: AttributeData) -> Self {
        let component_count = data.number_of_components() as i32;

        Self {
            id: generate_id(),
            data,
            layout: AttributeLayout {
                name: String::from(name),
                component_count,
                component_type: AttributeComponentType::Float,
                normalize: false,
                stride: 0,
                offset: 0,
            },
        }
    }

    pub fn create_webgl_buffer(&self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        match &self.data {
            AttributeData::Float(data) => SingleAttributeVertexData::float(gl, data),
            AttributeData::Vec2(data) => SingleAttributeVertexData::vec2(gl, data),
            AttributeData::Vec3(data) => SingleAttributeVertexData::vec3(gl, data),
            AttributeData::Vec4(data) => SingleAttributeVertexData::vec4(gl, data),
        }
    }

    fn float(gl: &WebGl2RenderingContext, data: &Vec<f32>) -> WebGlBuffer {
        SingleAttributeVertexData::create_float_buffer(gl, data)
    }

    fn vec2(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 2);

        for (a, b) in data {
            values.push(*a);
            values.push(*b);
        }

        SingleAttributeVertexData::create_float_buffer(gl, &values)
    }

    fn vec3(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 3);

        for (a, b, c) in data {
            values.push(*a);
            values.push(*b);
            values.push(*c);
        }

        SingleAttributeVertexData::create_float_buffer(gl, &values)
    }

    fn vec4(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32, f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 4);

        for (a, b, c, d) in data {
            values.push(*a);
            values.push(*b);
            values.push(*c);
            values.push(*d);
        }

        SingleAttributeVertexData::create_float_buffer(gl, &values)
    }

    #[inline]
    fn create_float_buffer(gl: &WebGl2RenderingContext, data: &Vec<f32>) -> WebGlBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Float32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        }

        buffer
    }
}

pub struct InterleavedAttributesVertexData {
    pub id:     u64,
    pub data:   Vec<AttributeData>,
    pub layout: Vec<AttributeLayout>,
}

impl InterleavedAttributesVertexData {
    pub fn new(data: Vec<(String, AttributeData)>) -> Self {
        let layout = InterleavedAttributesVertexData::convert_attribute_data_array_to_attribute_layout_array(&data);

        Self {
            id: generate_id(),
            data: data.into_iter().map(|x| x.1).collect(),
            layout,
        }
    }

    fn convert_attribute_data_array_to_attribute_layout_array(attributes: &[(String, AttributeData)]) -> Vec<AttributeLayout> {
        let mut attribute_layout = Vec::new();
        let mut offset = 0;

        for attribute in attributes {
            let attribute_description = AttributeLayout {
                name:            attribute.0.to_string(),
                component_count: attribute.1.number_of_components() as i32,
                component_type:  attribute.1.component_type(),
                normalize:       false,
                offset:          offset,
                stride:          0, // Will be populated after the loop
            };

            offset += attribute.1.element_size_in_bytes() as i32;
            attribute_layout.push(attribute_description);
        }

        // After the previous loop, offset will be equal to the stride
        for attribute in &mut attribute_layout {
            attribute.stride = offset;
        }

        attribute_layout
    }

    pub fn create_webgl_buffer(&self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        let attribute_data = self.data.get(0).unwrap();
        let attribute_layout = self.layout.get(0).unwrap();

        let stride = attribute_layout.stride;
        let buffer_size = stride as u32 * attribute_data.vertex_count() as u32;

        let array_buffer = ArrayBuffer::new(buffer_size);
        let data_view = DataView::new(&array_buffer, 0, buffer_size as usize);

        for vertex_index in 0..attribute_data.vertex_count() {
            for (attribute_index, attribute_data) in self.data.iter().enumerate() {
                let attribute_description = &self.layout[attribute_index];
                let offset = stride as usize * vertex_index + attribute_description.offset as usize;

                match attribute_data {
                    AttributeData::Float(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, *value, true);
                    }
                    AttributeData::Vec2(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                    }
                    AttributeData::Vec3(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                        data_view.set_float32_endian(offset + 8, value.2, true);
                    }
                    AttributeData::Vec4(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                        data_view.set_float32_endian(offset + 8, value.2, true);
                        data_view.set_float32_endian(offset + 12, value.3, true);
                    }
                }
            }
        }

        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &js_sys::Uint8Array::new(&array_buffer),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        buffer
    }
}
