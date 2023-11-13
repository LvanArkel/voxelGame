use nalgebra::{Vector3, Vector2};

use crate::{rendering::Mesh, asset::Texture};

const VERTEX_COUNT: usize = 4 * 6;
const INDEX_COUNT: usize = 6 * 6;

pub struct MeshData {
    pub indices: Vec<u32>,
    pub vertices: Vec<Vector3<f32>>,
    //normals
    pub uvs: Vec<Vector2<f32>>,
    pub vertex_amount: u32,
}

pub fn cube() -> MeshData {
    let base_vertices: [Vector3<f32>; 8] = [
        Vector3::new(0.0, 0.0, 0.0), // 0. Left bottom back
        Vector3::new(1.0, 0.0, 0.0), // 1. Right bottom back
        Vector3::new(0.0, 1.0, 0.0), // 2. Left top back
        Vector3::new(1.0, 1.0, 0.0), // 3. Right top back
        Vector3::new(0.0, 0.0, 1.0), // 4. Left bottom front
        Vector3::new(1.0, 0.0, 1.0), // 5. Right bottom front
        Vector3::new(0.0, 1.0, 1.0), // 6. Left top front
        Vector3::new(1.0, 1.0, 1.0), // 7. Right top front
    ];
    let vertex_indices: [usize; VERTEX_COUNT] = [
        1, 0, 3, 2, //Back
        0, 1, 4, 5, //Bottom
        0, 4, 2, 6, //Left
        5, 1, 7, 3, //Right
        4, 5, 6, 7, //Front
        6, 7, 2, 3, //Top
    ];
    let index_pattern: [u32; 6] = [0, 1, 2, 2, 1, 3];
    let mut vertices = Vec::with_capacity(VERTEX_COUNT);
    for &vertex_i in vertex_indices.iter() {
        vertices.push(base_vertices[vertex_i]);
    }
    let base_uvs: [Vector2<f32>; 4] = [
        Vector2::new(0.0, 1.0), // Bottom left
        Vector2::new(1.0, 1.0), // Bottom right
        Vector2::new(0.0, 0.0), // Top left
        Vector2::new(1.0, 0.0), // Top right
    ];
    let mut uvs = Vec::with_capacity(VERTEX_COUNT);
    for _ in 0..6 {
        uvs.extend(base_uvs);
    }
    let mut indices = vec![0; INDEX_COUNT];
    for i in 0..6 {
        for j in 0..6 {
            indices[6*i + j] = 4*(i as u32) + index_pattern[j];
        }
    }
    MeshData {
        indices,
        vertices,
        uvs,
        vertex_amount: VERTEX_COUNT as u32,
    }
}

pub fn cube_mesh(texture: Texture) -> Mesh {
    let mesh_data = cube();
    Mesh::new(INDEX_COUNT as i32, &mesh_data.indices, &mesh_data.vertices, None, Some(&mesh_data.uvs), Some(texture))
}
