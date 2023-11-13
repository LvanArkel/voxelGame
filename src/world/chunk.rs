use nalgebra::{Vector3, Vector2};

use crate::{rendering::{Mesh, primitives}, asset::Texture};

use super::voxel::Voxel;

pub const CHUNK_SIZE_X: usize = 8;
pub const CHUNK_SIZE_Y: usize = 8;
pub const CHUNK_SIZE_Z: usize = 8;

pub struct Chunk {
    pub chunk_data: Vec<Option<Voxel>>,
}

impl Chunk {
    pub fn new(data: Vec<Option<Voxel>>) -> Self {
        Self { chunk_data: data }
    }

    pub fn get_voxel(&self, coordinates: Vector3<usize>) -> Result<Option<Voxel>, ()> {
        if coordinates.x >= CHUNK_SIZE_X || coordinates.y >= CHUNK_SIZE_Y || coordinates.z >= CHUNK_SIZE_Z {
            Err(())
        } else {
            Ok(self.chunk_data[coordinates.x + CHUNK_SIZE_X * coordinates.y + (CHUNK_SIZE_X * CHUNK_SIZE_Y) * coordinates.z])
        }
    }

    pub fn generate_mesh(&self, texture: Texture) -> Mesh {
        // let cube_vertices: [Vector3<f32>; 8] = [
        //     Vector3::new(0.0, 0.0, 0.0), // 0. Left bottom back
        //     Vector3::new(1.0, 0.0, 0.0), // 1. Right bottom back
        //     Vector3::new(0.0, 1.0, 0.0), // 2. Left top back
        //     Vector3::new(1.0, 1.0, 0.0), // 3. Right top back
        //     Vector3::new(0.0, 0.0, 1.0), // 4. Left bottom front
        //     Vector3::new(1.0, 0.0, 1.0), // 5. Right bottom front
        //     Vector3::new(0.0, 1.0, 1.0), // 6. Left top front
        //     Vector3::new(1.0, 1.0, 1.0), // 7. Right top front
        // ];
        // let cube_indices: HashMap<Direction, [u32; 6]> = HashMap::from([
        //     (Direction::Right, []),
        //     (Direction::Left, []),
        //     (),
        //     (),
        //     (),
        //     (),
        // ])


        let cube_data = primitives::cube();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertices: Vec<Vector3<f32>> = Vec::new();
        let mut uvs: Vec<Vector2<f32>> = Vec::new();
        let mut voxel_i = 0;
        for i in 0..(CHUNK_SIZE_X*CHUNK_SIZE_Y*CHUNK_SIZE_Z) {
            if self.chunk_data[i].is_none() {
                continue;
            }
            let x = (i % CHUNK_SIZE_X) as f32;
            let y = ((i % (CHUNK_SIZE_X*CHUNK_SIZE_Y)) / CHUNK_SIZE_X) as f32;
            let z = (i / (CHUNK_SIZE_X*CHUNK_SIZE_Y)) as f32;
            indices.extend(cube_data.indices.iter().map(|index| cube_data.vertices.len() as u32 * voxel_i + index));
            vertices.extend(cube_data.vertices.iter().map(|&vertex| Vector3::new(vertex.x + x, vertex.y + y, vertex.z + z)));
            uvs.extend(cube_data.uvs.iter());
            voxel_i += 1;
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
