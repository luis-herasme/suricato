use web_sys::{WebGl2RenderingContext, WebGlBuffer, js_sys};

use crate::generate_id::generate_id;

pub enum IndexData {
    UnsignedByte(Vec<u8>),
    UnsignedShort(Vec<u16>),
    UnsignedInt(Vec<u32>),
}

impl IndexData {
    fn count(&self) -> usize {
        match &self {
            IndexData::UnsignedByte(data) => data.len(),
            IndexData::UnsignedShort(data) => data.len(),
            IndexData::UnsignedInt(data) => data.len(),
        }
    }

    fn kind(&self) -> u32 {
        match &self {
            IndexData::UnsignedByte(_) => WebGl2RenderingContext::UNSIGNED_BYTE,
            IndexData::UnsignedShort(_) => WebGl2RenderingContext::UNSIGNED_SHORT,
            IndexData::UnsignedInt(_) => WebGl2RenderingContext::UNSIGNED_INT,
        }
    }

    pub fn to_index_buffer(self) -> IndexBuffer {
        IndexBuffer::new(self)
    }
}

pub struct IndexLayout {
    pub kind:               u32,
    pub count:              i32,
    pub offset:             i32,
    pub number_of_elements: u32,
}

pub struct IndexBuffer {
    pub id:     u64,
    pub data:   IndexData,
    pub layout: IndexLayout,
}

impl IndexBuffer {
    pub fn new(data: IndexData) -> IndexBuffer {
        IndexBuffer {
            id: generate_id(),
            layout: IndexLayout {
                offset:             0,
                kind:               data.kind(),
                count:              data.count() as i32,
                number_of_elements: data.count() as u32,
            },
            data,
        }
    }

    pub fn create_webgl_buffer(&self, gl: &WebGl2RenderingContext) -> WebGlBuffer {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            match &self.data {
                IndexData::UnsignedByte(data) => {
                    let data = js_sys::Uint8Array::view(&data);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                        &data,
                        WebGl2RenderingContext::STATIC_DRAW,
                    );
                }
                IndexData::UnsignedShort(data) => {
                    let data = js_sys::Uint16Array::view(&data);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                        &data,
                        WebGl2RenderingContext::STATIC_DRAW,
                    );
                }
                IndexData::UnsignedInt(data) => {
                    let data = js_sys::Uint32Array::view(&data);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                        &data,
                        WebGl2RenderingContext::STATIC_DRAW,
                    );
                }
            };
        }

        buffer
    }
}
