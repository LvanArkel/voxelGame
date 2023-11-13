use nalgebra::{Vector3, Vector2};

use crate::{rendering::{Mesh, primitives}, asset::Texture, math::Direction};

use super::voxel::Voxel;

pub const CHUNK_SIZE_X: i32 = 8;
pub const CHUNK_SIZE_Y: i32 = 8;
pub const CHUNK_SIZE_Z: i32 = 8;

pub struct Chunk {
    pub chunk_data: Vec<Option<Voxel>>,
}

impl Chunk {
    pub fn new(data: Vec<Option<Voxel>>) -> Self {
        Self { chunk_data: data }
    }

    pub fn get_voxel(&self, coordinates: Vector3<i32>) -> Option<&Option<Voxel>> {
        if coordinates.x < 0 || coordinates.x >= CHUNK_SIZE_X ||
            coordinates.y < 0 || coordinates.y >= CHUNK_SIZE_Y ||
            coordinates.z < 0 || coordinates.z >= CHUNK_SIZE_Z {
            None
        } else {
            let i = coordinates.x + CHUNK_SIZE_X * coordinates.y + (CHUNK_SIZE_X * CHUNK_SIZE_Y) * coordinates.z;
            self.chunk_data.get(i as usize)
        }
    }

    pub fn get_neighbour(&self, coordinates: Vector3<i32>, direction: &Direction) -> Option<&Option<Voxel>> {
        let neighbour_coordinates: Vector3<i32> = coordinates + direction.facing();
        self.get_voxel(neighbour_coordinates)
    }

    pub fn generate_mesh(&self, texture: Texture) -> Mesh {
        let cube_vertices: [Vector3<f32>; 8] = [
            Vector3::new(1.0, 0.0, 1.0), // 0. Left bottom back
        Vector3::new(0.0, 0.0, 1.0), // 1. Right bottom back
        Vector3::new(1.0, 1.0, 1.0), // 2. Left top back
        Vector3::new(0.0, 1.0, 1.0), // 3. Right top back
        Vector3::new(1.0, 0.0, 0.0), // 4. Left bottom front
        Vector3::new(0.0, 0.0, 0.0), // 5. Right bottom front
        Vector3::new(1.0, 1.0, 0.0), // 6. Left top front
        Vector3::new(0.0, 1.0, 0.0), // 7. Right top front
        ];
        let cube_indices: [(Direction, [usize; 4]); 6] = [
            (Direction::Left, [0, 4, 2, 6]),
            (Direction::Right, [5, 1, 7, 3]),
            (Direction::Up, [6, 7, 2, 3]),
            (Direction::Down, [0, 1, 4, 5]),
            (Direction::Back, [1, 0, 3, 2]),
            (Direction::Front, [4, 5, 6, 7]),
        ];
        let index_pattern: [u32; 6] = [0, 1, 2, 2, 1, 3];
        let base_uvs: [Vector2<f32>; 4] = [
            Vector2::new(0.0, 1.0), // Bottom left
            Vector2::new(1.0, 1.0), // Bottom right
            Vector2::new(0.0, 0.0), // Top left
            Vector2::new(1.0, 0.0), // Top right
        ];
        let mut indices: Vec<u32> = Vec::new();
        let mut vertices: Vec<Vector3<f32>> = Vec::new();
        let mut uvs: Vec<Vector2<f32>> = Vec::new();
        let mut face_i = 0;

        for i in 0..(CHUNK_SIZE_X*CHUNK_SIZE_Y*CHUNK_SIZE_Z) {
            if self.chunk_data[i as usize].is_none() {
                continue;
            }
            let x = i % CHUNK_SIZE_X;
            let y = (i % (CHUNK_SIZE_X*CHUNK_SIZE_Y)) / CHUNK_SIZE_X;
            let z = i / (CHUNK_SIZE_X*CHUNK_SIZE_Y);
            let displacement = Vector3::new(x as f32, y as f32, z as f32);

            for (direction, vertex_pattern) in &cube_indices {
                let neighbour = self.get_neighbour(Vector3::new(x, y, z), direction);
                if neighbour.is_some_and(|neighbour| neighbour.is_some()) {
                    continue;
                }
                vertices.extend(vertex_pattern.map(|vertex_i| {
                    cube_vertices[vertex_i] + displacement
                }));
                indices.extend(index_pattern.map(|index_i| index_i + 4*face_i));
                uvs.extend(base_uvs);
                face_i += 1;
            }
        }
        
        Mesh::new(
            indices.len() as i32,
            &indices,
            &vertices,
            None, 
            Some(&uvs), 
            Some(texture),
        )
    }
}
