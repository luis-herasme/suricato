use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

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

#[derive(Debug)]
pub enum TextureError {
    CreationFailed,
    DataUploadFailed,
}

/// Extracted from:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameter#pname
#[derive(Clone, Debug)]
pub struct Texture {
    pub minification_filter:  MinificationFilter,
    pub magnification_filter: MagnificationFilter,
    pub wrap_horizontal:      Wrap,
    pub wrap_vertical:        Wrap,
    pub data_type:            TextureDataType,
    pub format:               TextureFormat,
    pub internal_format:      TextureFormat,
    pub texture_data:         TextureData,
    pub webgl_texture:        WebGlTexture,
}

impl Texture {
    pub fn new(gl: &WebGl2RenderingContext, texture_data: TextureData) -> Result<Texture, TextureError> {
        let Some(webgl_texture) = gl.create_texture() else {
            return Err(TextureError::CreationFailed);
        };

        let texture = Texture {
            minification_filter:  MinificationFilter::Nearest,
            magnification_filter: MagnificationFilter::Nearest,
            wrap_horizontal:      Wrap::Repeat,
            wrap_vertical:        Wrap::Repeat,
            data_type:            TextureDataType::UnsignedByte,
            format:               TextureFormat::RGBA,
            internal_format:      TextureFormat::RGBA,
            texture_data:         texture_data,
            webgl_texture:        webgl_texture,
        };

        texture.upload_webgl_texture(gl)?;

        Ok(texture)
    }

    fn upload_webgl_texture(&self, gl: &WebGl2RenderingContext) -> Result<(), TextureError> {
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.webgl_texture));

        match &self.texture_data {
            TextureData::HtmlImageElement(source) => {
                gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    self.internal_format as i32,
                    self.format as u32,
                    self.data_type as u32,
                    source,
                )
                .map_err(|_| TextureError::DataUploadFailed)?;
            }
            TextureData::ImagePixelData(data) => {
                gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    self.internal_format as i32,
                    data.width as i32,
                    data.height as i32,
                    0,
                    self.format as u32,
                    self.data_type as u32,
                    Some(&data.bytes),
                )
                .map_err(|_| TextureError::DataUploadFailed)?;
            }
        }

        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            self.minification_filter as i32,
        );

        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            self.magnification_filter as i32,
        );

        Ok(())
    }
}
