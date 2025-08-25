use web_sys::{WebGl2RenderingContext, WebGlBuffer, js_sys};

/// Extracted from:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants#data_types
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum AttributeKind {
    Byte          = 0x1400,
    UnsignedByte  = 0x1401,
    Short         = 0x1402,
    UnsignedShort = 0x1403,
    Int           = 0x1404,
    UnsignedInt   = 0x1405,
    Float         = 0x1406,
}

/// Parameters of vertexAttribPointer:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer
pub struct Attribute {
    pub buffer:    WebGlBuffer,
    pub size:      i32,
    pub kind:      AttributeKind,
    pub normalize: bool,
}

impl Attribute {
    pub fn float(gl: &WebGl2RenderingContext, data: Vec<f32>) -> Attribute {
        Attribute::float_attribute_generator(gl, data, 1)
    }

    pub fn vec2(gl: &WebGl2RenderingContext, data: Vec<f32>) -> Attribute {
        Attribute::float_attribute_generator(gl, data, 2)
    }

    pub fn vec3(gl: &WebGl2RenderingContext, data: Vec<f32>) -> Attribute {
        Attribute::float_attribute_generator(gl, data, 3)
    }

    pub fn vec4(gl: &WebGl2RenderingContext, data: Vec<f32>) -> Attribute {
        Attribute::float_attribute_generator(gl, data, 4)
    }

    #[inline]
    fn float_attribute_generator(gl: &WebGl2RenderingContext, data: Vec<f32>, size: i32) -> Attribute {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Float32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        }

        Attribute {
            buffer,
            size,
            kind: AttributeKind::Float,
            normalize: false,
        }
    }
}
