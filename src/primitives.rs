use crate::{TexturedMesh, Texture};


pub fn cube_mesh(scale: f32, texture: Texture) -> TexturedMesh {
    const VERTEX_COUNT: usize = 6 * 6;
    let base_vertices: [f32; 8*3] = [
        -scale, -scale, -scale,
         scale, -scale, -scale,
        -scale,  scale, -scale,
         scale,  scale, -scale,
        -scale, -scale,  scale,
         scale, -scale,  scale,
        -scale,  scale,  scale,
         scale,  scale,  scale,
    ];
    let vertex_indices: [usize; VERTEX_COUNT] = [
        //Back
        1, 0, 3,
        3, 0, 2,
        //Bottom
        0, 1, 4,
        4, 1, 5,
        //Left
        0, 4, 2,
        2, 4, 6,
        //Right
        5, 1, 7,
        7, 1, 3,
        //Front
        4, 5, 6,
        6, 5, 7,
        //Top
        6, 7, 2,
        2, 7, 3,
    ];
    let mut vertices = [0.0; VERTEX_COUNT*3];
    for (i, &vertex_i) in vertex_indices.iter().enumerate() {
        vertices[3*i+0] = base_vertices[3*vertex_i+0];
        vertices[3*i+1] = base_vertices[3*vertex_i+1];
        vertices[3*i+2] = base_vertices[3*vertex_i+2];
    }
    let base_uvs: [f32; 6 * 2] = [
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0,
        0.0, 0.0,
        1.0, 1.0,
        1.0, 0.0,
    ];
    let mut uvs = [0.0; VERTEX_COUNT*2];
    for face in 0..6 {
        for base_uv_i in 0..6 {
            uvs[12*face+2*base_uv_i+0] = base_uvs[2*base_uv_i+0];
            uvs[12*face+2*base_uv_i+1] = base_uvs[2*base_uv_i+1];
        }
    }
    TexturedMesh::new(VERTEX_COUNT as i32, &vertices, &uvs, texture)
}