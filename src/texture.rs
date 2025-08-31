use web_sys::{HtmlImageElement, WebGl2RenderingContext};

use crate::utils::generate_id;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum MinificationFilter {
    Linear               = WebGl2RenderingContext::LINEAR,
    Nearest              = WebGl2RenderingContext::NEAREST,
    NearestMipmapNearest = WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest  = WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear  = WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear   = WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum MagnificationFilter {
    Linear  = WebGl2RenderingContext::LINEAR,
    Nearest = WebGl2RenderingContext::NEAREST,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Wrap {
    Repeat         = WebGl2RenderingContext::REPEAT,
    ClampToEdge    = WebGl2RenderingContext::CLAMP_TO_EDGE,
    MirroredRepeat = WebGl2RenderingContext::MIRRORED_REPEAT,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
    RGB            = WebGl2RenderingContext::RGB,
    RGBA           = WebGl2RenderingContext::RGBA,
    LuminanceAlpha = WebGl2RenderingContext::LUMINANCE_ALPHA,
    Luminance      = WebGl2RenderingContext::LUMINANCE,
    Alpha          = WebGl2RenderingContext::ALPHA,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum TextureDataType {
    UnsignedByte      = WebGl2RenderingContext::UNSIGNED_BYTE,
    UnsignedShort565  = WebGl2RenderingContext::UNSIGNED_SHORT_5_6_5,
    UnsignedShort4444 = WebGl2RenderingContext::UNSIGNED_SHORT_4_4_4_4,
    UnsignedShort5551 = WebGl2RenderingContext::UNSIGNED_SHORT_5_5_5_1,
}

/// Extracted from:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameter#pname
#[derive(Clone, Debug)]
pub struct Texture {
    pub id:                   u64,
    pub minification_filter:  MinificationFilter,
    pub magnification_filter: MagnificationFilter,
    pub wrap_horizontal:      Wrap,
    pub wrap_vertical:        Wrap,
    pub data_type:            TextureDataType,
    pub format:               TextureFormat,
    pub internal_format:      TextureFormat,
    pub texture_data:         TextureData,
}

#[derive(Clone, Debug)]
pub enum TextureData {
    HtmlImageElement(HtmlImageElement),
    ImagePixelData(ImagePixelData),
}

#[derive(Clone, Debug)]
pub struct ImagePixelData {
    pub width:  u32,
    pub height: u32,
    pub bytes:  Vec<u8>,
}

impl Texture {
    pub fn new(texture_data: TextureData) -> Texture {
        Texture {
            id:                   generate_id(),
            minification_filter:  MinificationFilter::Nearest,
            magnification_filter: MagnificationFilter::Nearest,
            wrap_horizontal:      Wrap::Repeat,
            wrap_vertical:        Wrap::Repeat,
            data_type:            TextureDataType::UnsignedByte,
            format:               TextureFormat::RGBA,
            internal_format:      TextureFormat::RGBA,
            texture_data:         texture_data,
        }
    }
}

impl From<HtmlImageElement> for Texture {
    fn from(html_image: HtmlImageElement) -> Texture {
        Texture::new(TextureData::HtmlImageElement(html_image))
    }
}
