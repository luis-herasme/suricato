use glam::{Mat2, Mat3, Mat4};

use crate::{
    texture::Texture,
    transform::{Transform2D, Transform3D},
};

#[derive(Debug, Clone)]
pub enum Uniform {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),

    Int(i32),
    IntVec2([i32; 2]),
    IntVec3([i32; 3]),
    IntVec4([i32; 4]),

    UnsignedInt(u32),
    UnsignedIntVec2([u32; 2]),
    UnsignedIntVec3([u32; 3]),
    UnsignedIntVec4([u32; 4]),

    Mat2([f32; 4]),
    Mat3([f32; 9]),
    Mat4([f32; 16]),

    Texture(Texture),
}

// f32
impl From<f32> for Uniform {
    fn from(value: f32) -> Uniform {
        Uniform::Float(value)
    }
}

impl From<[f32; 2]> for Uniform {
    fn from(value: [f32; 2]) -> Uniform {
        Uniform::Vec2(value)
    }
}

impl From<[f32; 3]> for Uniform {
    fn from(value: [f32; 3]) -> Uniform {
        Uniform::Vec3(value)
    }
}

impl From<[f32; 4]> for Uniform {
    fn from(value: [f32; 4]) -> Uniform {
        Uniform::Vec4(value)
    }
}

// i32
impl From<i32> for Uniform {
    fn from(value: i32) -> Uniform {
        Uniform::Int(value)
    }
}

impl From<[i32; 2]> for Uniform {
    fn from(value: [i32; 2]) -> Uniform {
        Uniform::IntVec2(value)
    }
}

impl From<[i32; 3]> for Uniform {
    fn from(value: [i32; 3]) -> Uniform {
        Uniform::IntVec3(value)
    }
}

impl From<[i32; 4]> for Uniform {
    fn from(value: [i32; 4]) -> Uniform {
        Uniform::IntVec4(value)
    }
}

// u32
impl From<u32> for Uniform {
    fn from(value: u32) -> Uniform {
        Uniform::UnsignedInt(value)
    }
}

impl From<[u32; 2]> for Uniform {
    fn from(value: [u32; 2]) -> Uniform {
        Uniform::UnsignedIntVec2(value)
    }
}

impl From<[u32; 3]> for Uniform {
    fn from(value: [u32; 3]) -> Uniform {
        Uniform::UnsignedIntVec3(value)
    }
}

impl From<[u32; 4]> for Uniform {
    fn from(value: [u32; 4]) -> Uniform {
        Uniform::UnsignedIntVec4(value)
    }
}

// matrices
impl From<[[f32; 2]; 2]> for Uniform {
    #[rustfmt::skip]
    fn from(value: [[f32; 2]; 2]) -> Uniform {
        Uniform::Mat2([
            value[0][0], value[0][1],
            value[1][0], value[1][1]
        ])
    }
}

impl From<[[f32; 3]; 3]> for Uniform {
    #[rustfmt::skip]
    fn from(value: [[f32; 3]; 3]) -> Uniform {
        Uniform::Mat3([
            value[0][0], value[0][1], value[0][2],
            value[1][0], value[1][1], value[1][2],
            value[2][0], value[2][1], value[2][2],
        ])
    }
}

impl From<[[f32; 4]; 4]> for Uniform {
    #[rustfmt::skip]
    fn from(value: [[f32; 4]; 4]) -> Uniform {
        Uniform::Mat4([
            value[0][0], value[0][1], value[0][2], value[0][3],
            value[1][0], value[1][1], value[1][2], value[1][3],
            value[2][0], value[2][1], value[2][2], value[2][3],
            value[3][0], value[3][1], value[3][2], value[3][3],
        ])
    }
}

// texture
impl From<Texture> for Uniform {
    fn from(value: Texture) -> Uniform {
        Uniform::Texture(value)
    }
}

// Transform
impl From<&Transform3D> for Uniform {
    fn from(value: &Transform3D) -> Uniform {
        Uniform::Mat4(value.to_array())
    }
}

impl From<&Transform2D> for Uniform {
    fn from(value: &Transform2D) -> Uniform {
        Uniform::Mat3(value.to_array())
    }
}

// Matrix
impl From<&Mat2> for Uniform {
    fn from(value: &Mat2) -> Uniform {
        Uniform::Mat2(value.to_cols_array())
    }
}

impl From<&Mat3> for Uniform {
    fn from(value: &Mat3) -> Uniform {
        Uniform::Mat3(value.to_cols_array())
    }
}

impl From<&Mat4> for Uniform {
    fn from(value: &Mat4) -> Uniform {
        Uniform::Mat4(value.to_cols_array())
    }
}
