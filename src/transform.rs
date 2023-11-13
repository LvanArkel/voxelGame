use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};

pub struct Transform {
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
}

impl Transform {
    pub fn new(position: Vector3<f32>, rotation: UnitQuaternion<f32>) -> Self { Self { position, rotation } }

    pub fn from_isometry(isometry: Isometry3<f32>) -> Self {
        Self {
            position: isometry.translation.vector,
            rotation: isometry.rotation,
        }
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }

    pub fn view(&self) -> Isometry3<f32> {
        Isometry3::from_parts(Translation3{ vector: self.position }, self.rotation)
    }
}

