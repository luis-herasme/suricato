use web_sys::{WebGl2RenderingContext, WebGlBuffer, js_sys};

pub enum IndexData {
    UnsignedByte(Vec<u8>),
    UnsignedShort(Vec<u16>),
    UnsignedInt(Vec<u32>),
}

pub struct IndexBuffer {
    pub buffer:             WebGlBuffer,
    pub kind:               u32,
    pub count:              i32,
    pub offset:             i32,
    pub number_of_elements: u32,
}

impl IndexBuffer {
    pub fn from_index_data(gl: &WebGl2RenderingContext, index_data: &IndexData) -> IndexBuffer {
        match index_data {
            IndexData::UnsignedByte(data) => IndexBuffer::u8(gl, data),
            IndexData::UnsignedShort(data) => IndexBuffer::u16(gl, data),
            IndexData::UnsignedInt(data) => IndexBuffer::u32(gl, data),
        }
    }

    pub fn u8(gl: &WebGl2RenderingContext, data: &Vec<u8>) -> IndexBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Uint8Array::view(&data);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &data,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let number_of_elements = data.len() as u32;

        IndexBuffer {
            buffer,
            kind: WebGl2RenderingContext::UNSIGNED_BYTE,
            count: number_of_elements as i32,
            offset: 0,
            number_of_elements,
        }
    }

    pub fn u16(gl: &WebGl2RenderingContext, data: &Vec<u16>) -> IndexBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Uint16Array::view(&data);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &data,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let number_of_elements = data.len() as u32;

        IndexBuffer {
            buffer,
            kind: WebGl2RenderingContext::UNSIGNED_SHORT,
            count: number_of_elements as i32,
            offset: 0,
            number_of_elements,
        }
    }

    pub fn u32(gl: &WebGl2RenderingContext, data: &Vec<u32>) -> IndexBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let data = js_sys::Uint32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &data,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let number_of_elements = data.len() as u32;

        IndexBuffer {
            buffer,
            kind: WebGl2RenderingContext::UNSIGNED_INT,
            count: number_of_elements as i32,
            offset: 0,
            number_of_elements,
        }
    }
}
