use glam::{Mat3, Mat4, Quat, Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Transform3D {
    pub scale:       Vec3,
    pub rotation:    Quat,
    pub translation: Vec3,
}

impl Transform3D {
    pub fn new() -> Transform3D {
        Transform3D {
            scale:       Vec3::ONE,
            rotation:    Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }

    pub fn to_array(&self) -> [f32; 16] {
        self.to_mat4().to_cols_array()
    }

    pub fn to_cols_array_2d(&self) -> [[f32; 4]; 4] {
        self.to_mat4().to_cols_array_2d()
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

impl From<Mat4> for Transform3D {
    fn from(value: Mat4) -> Transform3D {
        let (scale, rotation, translation) = value.to_scale_rotation_translation();

        Transform3D {
            scale,
            rotation,
            translation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transform2D {
    pub scale:       Vec2,
    pub rotation:    f32, // radians
    pub translation: Vec2,
}

impl Transform2D {
    pub fn new() -> Self {
        Self {
            scale:       Vec2::ONE,
            rotation:    0.0,
            translation: Vec2::ZERO,
        }
    }

    pub fn to_array(&self) -> [f32; 9] {
        self.to_mat3().to_cols_array()
    }

    pub fn to_cols_array_2d(&self) -> [[f32; 3]; 3] {
        self.to_mat3().to_cols_array_2d()
    }

    pub fn to_mat3(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }
}
