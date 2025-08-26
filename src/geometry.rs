use crate::{attributes::AttributeData, index_buffer::IndexData};

pub struct Geometry {
    pub attributes: Vec<AttributeData>,
    pub indices:    Option<IndexData>,
}
