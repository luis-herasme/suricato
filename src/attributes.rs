use web_sys::{
    WebGl2RenderingContext, WebGlBuffer,
    js_sys::{self, ArrayBuffer, DataView},
};

/// Extracted from:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants#data_types
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    // Byte          = 0x1400,
    // UnsignedByte  = 0x1401,
    // Short         = 0x1402,
    // UnsignedShort = 0x1403,
    // Int           = 0x1404,
    // UnsignedInt   = 0x1405,
    Float = 0x1406,
}

impl ComponentType {
    // https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Types
    fn number_of_bytes(&self) -> u8 {
        match &self {
            // ComponentType::Byte => 1,
            // ComponentType::UnsignedByte => 1,
            // ComponentType::Short => 2,
            // ComponentType::UnsignedShort => 2,
            // ComponentType::Int => 4,
            // ComponentType::UnsignedInt => 4,
            ComponentType::Float => 4,
        }
    }
}

pub struct AttributeBuffer {
    pub buffer:                 WebGlBuffer,
    pub number_of_components:   i32,
    pub type_of_the_components: ComponentType,
    pub normalize:              bool,
    pub stride:                 i32,
    pub offset:                 i32,
}

impl AttributeBuffer {
    pub fn float(gl: &WebGl2RenderingContext, data: Vec<f32>) -> AttributeBuffer {
        AttributeBuffer::float_attribute_generator(gl, data, 1)
    }

    pub fn vec2(gl: &WebGl2RenderingContext, data: Vec<f32>) -> AttributeBuffer {
        AttributeBuffer::float_attribute_generator(gl, data, 2)
    }

    pub fn vec3(gl: &WebGl2RenderingContext, data: Vec<f32>) -> AttributeBuffer {
        AttributeBuffer::float_attribute_generator(gl, data, 3)
    }

    pub fn vec4(gl: &WebGl2RenderingContext, data: Vec<f32>) -> AttributeBuffer {
        AttributeBuffer::float_attribute_generator(gl, data, 4)
    }

    #[inline]
    fn float_attribute_generator(gl: &WebGl2RenderingContext, data: Vec<f32>, number_of_components: i32) -> AttributeBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Float32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        }

        AttributeBuffer {
            buffer,
            number_of_components,
            type_of_the_components: ComponentType::Float,
            normalize: false,
            stride: 0,
            offset: 0,
        }
    }
}

pub enum AttributeData {
    Float { name: String, data: Vec<f32> },
    Vec2 { name: String, data: Vec<(f32, f32)> },
    Vec3 { name: String, data: Vec<(f32, f32, f32)> },
    Vec4 { name: String, data: Vec<(f32, f32, f32, f32)> },
}

impl AttributeData {
    fn number_of_components(&self) -> u8 {
        match &self {
            AttributeData::Float { .. } => 1,
            AttributeData::Vec2 { .. } => 2,
            AttributeData::Vec3 { .. } => 3,
            AttributeData::Vec4 { .. } => 4,
        }
    }

    fn component_type(&self) -> ComponentType {
        match &self {
            AttributeData::Float { .. } => ComponentType::Float,
            AttributeData::Vec2 { .. } => ComponentType::Float,
            AttributeData::Vec3 { .. } => ComponentType::Float,
            AttributeData::Vec4 { .. } => ComponentType::Float,
        }
    }

    fn element_size_in_bytes(&self) -> u8 {
        self.number_of_components() * self.component_type().number_of_bytes()
    }

    fn number_of_elements(&self) -> usize {
        match &self {
            AttributeData::Float { data, .. } => data.len(),
            AttributeData::Vec2 { data, .. } => data.len(),
            AttributeData::Vec3 { data, .. } => data.len(),
            AttributeData::Vec4 { data, .. } => data.len(),
        }
    }

    fn name(&self) -> String {
        match &self {
            AttributeData::Float { name, .. } => name.clone(),
            AttributeData::Vec2 { name, .. } => name.clone(),
            AttributeData::Vec3 { name, .. } => name.clone(),
            AttributeData::Vec4 { name, .. } => name.clone(),
        }
    }
}

pub struct InterleavedAttributeBuffer {
    pub buffer:      WebGlBuffer,
    pub description: Vec<AttributeDescription>,
}

/// Parameters for vertexAttribPointer:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer
#[derive(Debug)]
pub struct AttributeDescription {
    pub name:                   String,
    pub number_of_components:   i32,
    pub type_of_the_components: ComponentType,
    pub normalize:              bool,
    pub stride:                 i32,
    pub offset:                 i32,
}

impl InterleavedAttributeBuffer {
    pub fn new(gl: &WebGl2RenderingContext, attributes_data: Vec<AttributeData>) -> InterleavedAttributeBuffer {
        let attribute_descriptions = InterleavedAttributeBuffer::convert_attribute_data_to_description(&attributes_data);

        let attribute_data = attributes_data.get(0).unwrap();
        let attribute_description = attribute_descriptions.get(0).unwrap();

        let stride = attribute_description.stride;
        let buffer_size = stride as u32 * attribute_data.number_of_elements() as u32;

        let array_buffer = ArrayBuffer::new(buffer_size);
        let data_view = DataView::new(&array_buffer, 0, buffer_size as usize);

        for vertex_index in 0..attribute_data.number_of_elements() {
            for (attribute_index, attribute_data) in attributes_data.iter().enumerate() {
                let attribute_description = &attribute_descriptions[attribute_index];
                let offset = stride as usize * vertex_index + attribute_description.offset as usize;

                match attribute_data {
                    AttributeData::Float { data, .. } => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, *value, true);
                    }
                    AttributeData::Vec2 { data, .. } => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                    }
                    AttributeData::Vec3 { data, .. } => {
                        let value = data.get(vertex_index).unwrap();
                        data_view.set_float32_endian(offset, value.0, true);
                        data_view.set_float32_endian(offset + 4, value.1, true);
                        data_view.set_float32_endian(offset + 8, value.2, true);
                    }
                    AttributeData::Vec4 { data, .. } => {
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

        InterleavedAttributeBuffer {
            buffer,
            description: attribute_descriptions,
        }
    }

    fn convert_attribute_data_to_description(attributes: &Vec<AttributeData>) -> Vec<AttributeDescription> {
        let mut attribute_descriptions = Vec::new();
        let mut offset = 0;

        for attribute in attributes {
            let attribute_description = AttributeDescription {
                name:                   attribute.name(),
                number_of_components:   attribute.number_of_components() as i32,
                type_of_the_components: attribute.component_type(),
                normalize:              false,
                offset:                 offset,
                stride:                 0, // Will be populated after the loop
            };

            offset += attribute.element_size_in_bytes() as i32;
            attribute_descriptions.push(attribute_description);
        }

        // After the previous loop, offset will be equal to the stride
        for attribute in &mut attribute_descriptions {
            attribute.stride = offset;
        }

        attribute_descriptions
    }
}
