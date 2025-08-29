use web_sys::WebGl2RenderingContext;

use crate::generate_id::generate_id;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum VertexComponentType {
    Byte          = WebGl2RenderingContext::BYTE,
    UnsignedByte  = WebGl2RenderingContext::UNSIGNED_BYTE,
    Short         = WebGl2RenderingContext::SHORT,
    UnsignedShort = WebGl2RenderingContext::UNSIGNED_SHORT,
    Int           = WebGl2RenderingContext::INT,
    UnsignedInt   = WebGl2RenderingContext::UNSIGNED_INT,
    Float         = WebGl2RenderingContext::FLOAT,
}

impl VertexComponentType {
    pub fn size_in_bytes(&self) -> u8 {
        match &self {
            VertexComponentType::Byte => 1,
            VertexComponentType::UnsignedByte => 1,
            VertexComponentType::Short => 2,
            VertexComponentType::UnsignedShort => 2,
            VertexComponentType::Int => 4,
            VertexComponentType::UnsignedInt => 4,
            VertexComponentType::Float => 4,
        }
    }
}

pub struct VertexLayout {
    pub name:            String,
    pub component_count: u8,
    pub component_type:  VertexComponentType,
    pub normalize:       bool,
    pub stride:          u8,
    pub offset:          u8,
    pub divisor:         u32,
}

impl VertexLayout {
    fn from_vertex_data_array(vertex_data: &Vec<(String, VertexData)>) -> Vec<VertexLayout> {
        let mut vertex_layouts = Vec::new();
        let mut offset = 0;

        for (name, vertex_data) in vertex_data {
            let layout = VertexLayout {
                name:            String::from(name),
                component_count: vertex_data.component_count(),
                component_type:  vertex_data.component_type(),
                normalize:       false,
                offset:          offset,
                stride:          0, // Will be populated after the loop
                divisor:         0,
            };

            offset += vertex_data.component_count() * vertex_data.component_type().size_in_bytes();
            vertex_layouts.push(layout);
        }

        // After the previous loop, offset will be equal to the stride
        for vertex_layout in &mut vertex_layouts {
            vertex_layout.stride = offset;
        }

        vertex_layouts
    }
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

    fn component_type(&self) -> VertexComponentType {
        match &self {
            VertexData::Float { .. } => VertexComponentType::Float,
            VertexData::Vec2 { .. } => VertexComponentType::Float,
            VertexData::Vec3 { .. } => VertexComponentType::Float,
            VertexData::Vec4 { .. } => VertexComponentType::Float,
            VertexData::Mat4 { .. } => VertexComponentType::Float,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match &self {
            VertexData::Float(data) => to_bytes(data).to_vec(),
            VertexData::Vec2(data) => to_bytes(data).to_vec(),
            VertexData::Vec3(data) => to_bytes(data).to_vec(),
            VertexData::Vec4(data) => to_bytes(data).to_vec(),
            VertexData::Mat4(data) => to_bytes(data).to_vec(),
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
    pub count:        usize,
    pub data:         Vec<u8>,
    pub layout:       Vec<VertexLayout>,
    pub needs_update: bool,
}

impl VertexBuffer {
    pub fn single_attribute(name: &str, data: VertexData) -> VertexBuffer {
        let layout = VertexLayout {
            name:            String::from(name),
            component_count: data.component_count(),
            component_type:  data.component_type(),
            normalize:       false,
            stride:          data.component_count() * data.component_type().size_in_bytes(),
            offset:          0,
            divisor:         0,
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            count:        data.count(),
            data:         data.to_bytes(),
            layout:       vec![layout],
        }
    }

    pub fn single_attribute_with_divisor(name: &str, data: VertexData, divisor: u32) -> VertexBuffer {
        let layout = VertexLayout {
            name:            String::from(name),
            component_count: data.component_count(),
            component_type:  data.component_type(),
            normalize:       false,
            stride:          data.component_count() * data.component_type().size_in_bytes(),
            offset:          0,
            divisor:         divisor,
        };

        VertexBuffer {
            id:           generate_id(),
            needs_update: true,
            count:        data.count(),
            data:         data.to_bytes(),
            layout:       vec![layout],
        }
    }

    pub fn interleaved_attributes(data: Vec<(String, VertexData)>) -> VertexBuffer {
        let layout = VertexLayout::from_vertex_data_array(&data);

        let data: Vec<VertexData> = data.into_iter().map(|x| x.1).collect();
        let count = data[0].count();
        let data = VertexBuffer::array_to_bytes(&data, &layout);

        VertexBuffer {
            id: generate_id(),
            needs_update: true,
            count,
            data,
            layout,
        }
    }

    pub fn set_vertex_at_f32(&mut self, name: &str, index: usize, value: f32) -> bool {
        for layout in &self.layout {
            if layout.name != name {
                continue;
            }

            let byte_index = index * layout.stride as usize + layout.offset as usize;
            self.data[byte_index..byte_index + 4].copy_from_slice(&value.to_ne_bytes());
            self.needs_update = true;

            return true;
        }

        false
    }

    pub fn set_vertex_at_mat4(&mut self, name: &str, index: usize, value: [f32; 16]) -> bool {
        for layout in &self.layout {
            if layout.name != name {
                continue;
            }

            let byte_index = index * layout.stride as usize + layout.offset as usize;
            self.data[byte_index..byte_index + 64].copy_from_slice(to_bytes(&value));
            self.needs_update = true;

            return true;
        }

        false
    }

    fn array_to_bytes(vertex_data_array: &Vec<VertexData>, layout: &Vec<VertexLayout>) -> Vec<u8> {
        let vertex_count = vertex_data_array[0].count();
        let stride = layout[0].stride as usize;

        let mut buffer = Vec::with_capacity(stride * vertex_count);

        for vertex_index in 0..vertex_count {
            for vertex_data in vertex_data_array.iter() {
                vertex_data.write_vertex_bytes(vertex_index, &mut buffer);
            }
        }

        buffer
    }
}
