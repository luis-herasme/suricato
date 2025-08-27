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

pub enum VertexData {
    Float(Vec<f32>),
    Vec2(Vec<(f32, f32)>),
    Vec3(Vec<(f32, f32, f32)>),
    Vec4(Vec<(f32, f32, f32, f32)>),
}

impl VertexData {
    pub fn count(&self) -> usize {
        match &self {
            VertexData::Float(data) => data.len(),
            VertexData::Vec2(data) => data.len(),
            VertexData::Vec3(data) => data.len(),
            VertexData::Vec4(data) => data.len(),
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

    fn element_size_in_bytes(&self) -> u8 {
        self.number_of_components() * self.component_type().number_of_bytes()
    }
}

pub enum VertexBuffer {
    SingleAttribute(SingleAttributeVertexBuffer),
    InterleavedAttributes(InterleavedAttributesVertexBuffer),
}

impl VertexBuffer {
    pub fn single_attribute(name: &str, data: VertexData) -> VertexBuffer {
        VertexBuffer::SingleAttribute(SingleAttributeVertexBuffer::new(name, data))
    }

    pub fn interleaved_attributes(data: Vec<(String, VertexData)>) -> VertexBuffer {
        VertexBuffer::InterleavedAttributes(InterleavedAttributesVertexBuffer::new(data))
    }

    pub fn id(&self) -> u64 {
        match &self {
            VertexBuffer::SingleAttribute(data) => data.id,
            VertexBuffer::InterleavedAttributes(data) => data.id,
        }
    }

    pub fn create_webgl_buffer(&self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        match &self {
            VertexBuffer::SingleAttribute(buffer) => buffer.create_webgl_buffer(gl),
            VertexBuffer::InterleavedAttributes(buffer) => buffer.create_webgl_buffer(gl),
        }
    }

    pub fn vertex_count(&self) -> usize {
        match &self {
            VertexBuffer::SingleAttribute(attribute) => attribute.data.count(),
            VertexBuffer::InterleavedAttributes(attribute_vector) => {
                for attribute_data in &attribute_vector.data {
                    return attribute_data.count();
                }

                unreachable!("Interleaved attribute data will never be empty");
            }
        }
    }
}

pub struct SingleAttributeVertexBuffer {
    pub id:     u64,
    pub data:   VertexData,
    pub layout: AttributeLayout,
}

impl SingleAttributeVertexBuffer {
    pub fn new(name: &str, data: VertexData) -> Self {
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
            VertexData::Float(data) => SingleAttributeVertexBuffer::float(gl, data),
            VertexData::Vec2(data) => SingleAttributeVertexBuffer::vec2(gl, data),
            VertexData::Vec3(data) => SingleAttributeVertexBuffer::vec3(gl, data),
            VertexData::Vec4(data) => SingleAttributeVertexBuffer::vec4(gl, data),
        }
    }

    fn float(gl: &WebGl2RenderingContext, data: &Vec<f32>) -> WebGlBuffer {
        SingleAttributeVertexBuffer::create_float_buffer(gl, data)
    }

    fn vec2(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 2);

        for (a, b) in data {
            values.push(*a);
            values.push(*b);
        }

        SingleAttributeVertexBuffer::create_float_buffer(gl, &values)
    }

    fn vec3(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 3);

        for (a, b, c) in data {
            values.push(*a);
            values.push(*b);
            values.push(*c);
        }

        SingleAttributeVertexBuffer::create_float_buffer(gl, &values)
    }

    fn vec4(gl: &WebGl2RenderingContext, data: &Vec<(f32, f32, f32, f32)>) -> WebGlBuffer {
        let mut values = Vec::with_capacity(data.len() * 4);

        for (a, b, c, d) in data {
            values.push(*a);
            values.push(*b);
            values.push(*c);
            values.push(*d);
        }

        SingleAttributeVertexBuffer::create_float_buffer(gl, &values)
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

pub struct InterleavedAttributesVertexBuffer {
    pub id:     u64,
    pub data:   Vec<VertexData>,
    pub layout: Vec<AttributeLayout>,
}

impl InterleavedAttributesVertexBuffer {
    pub fn new(data: Vec<(String, VertexData)>) -> Self {
        let layout = InterleavedAttributesVertexBuffer::convert_attribute_data_array_to_attribute_layout_array(&data);

        Self {
            id: generate_id(),
            data: data.into_iter().map(|x| x.1).collect(),
            layout,
        }
    }

    fn convert_attribute_data_array_to_attribute_layout_array(attributes: &[(String, VertexData)]) -> Vec<AttributeLayout> {
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
        let buffer_size = stride as u32 * attribute_data.count() as u32;

        let array_buffer = ArrayBuffer::new(buffer_size);
        let data_view = DataView::new(&array_buffer, 0, buffer_size as usize);

        for vertex_index in 0..attribute_data.count() {
            for (attribute_index, attribute_data) in self.data.iter().enumerate() {
                let attribute_description = &self.layout[attribute_index];
                let offset = stride as usize * vertex_index + attribute_description.offset as usize;

                match attribute_data {
                    VertexData::Float(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, *value, true);
                    }
                    VertexData::Vec2(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                    }
                    VertexData::Vec3(data) => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                        data_view.set_float32_endian(offset + 8, value.2, true);
                    }
                    VertexData::Vec4(data) => {
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
