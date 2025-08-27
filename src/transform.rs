use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub scale:       Vec3,
    pub rotation:    Quat,
    pub translation: Vec3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            scale:       Vec3::ONE,
            rotation:    Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }

    pub fn to_array(&self) -> [f32; 16] {
        self.to_mat4().to_cols_array()
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}
