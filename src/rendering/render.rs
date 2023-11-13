use nalgebra::Isometry3;

use crate::{camera::Camera, asset::Shader};

use super::Mesh;

pub struct MeshRenderer {
    shader: Shader,
}

impl MeshRenderer {
    pub fn new(shader: Shader) -> Self { Self { shader } }

    pub fn render(
        &self, 
        transform: &Isometry3<f32>, 
        mesh: &Mesh, 
        camera: &Camera
    ) {
        let mvp = camera.mvp(transform);

        self.shader.bind();
        self.shader.uniform_mat4("mvp", mvp);
        self.shader.uniform_int("texture0", 0);
        mesh.draw();
        self.shader.unbind();
    }
}
