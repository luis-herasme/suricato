use glam::Mat4;

use crate::transform::Transform3D;

pub struct PerspectiveCamera {
    pub fov:    f32,
    pub aspect: f32,
    pub near:   f32,
    pub far:    f32,

    pub transform:         Transform3D,
    pub projection_matrix: Mat4,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> PerspectiveCamera {
        PerspectiveCamera {
            fov,
            aspect,
            near,
            far,
            transform: Transform3D::new(),
            projection_matrix: Mat4::ZERO,
        }
    }

    pub fn update_projection_matrix(&mut self) {
        self.projection_matrix = Mat4::perspective_rh_gl(self.fov, self.aspect, self.near, self.far);
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as f32;
        let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as f32;

        let mut camera = PerspectiveCamera {
            fov:               (45.0 * 3.14) / 180.0,
            aspect:            width / height,
            near:              0.1,
            far:               100.0,
            transform:         Transform3D::new(),
            projection_matrix: Mat4::ZERO,
        };

        camera.update_projection_matrix();
        camera
    }
}
