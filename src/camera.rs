use nalgebra::{Isometry3, Perspective3, Matrix4, Point3, Vector3};

use crate::transform::Transform;

pub struct Camera {
    transform: Transform,
    projection: Perspective3<f32>,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, transform: Transform) -> Self {
        let projection = Perspective3::new(screen_width as f32 / screen_height as f32, 3.14 / 2.0, 0.1, 1000.0);
        Self { transform, projection }
    }

    pub fn new_look_at(screen_width: u32, screen_height: u32, position: &Point3<f32>, target: &Point3<f32>) -> Self {
        let transform = Transform::from_isometry(Isometry3::look_at_rh(position, target, &Vector3::y()));
        Self::new(screen_width, screen_height, transform)
    }

    pub fn mvp(&self, model: &Isometry3<f32>) -> Matrix4<f32> {
        self.projection.as_matrix() * (self.transform.view() * model).to_homogeneous()
    }
}