use crate::texture::Texture;

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
