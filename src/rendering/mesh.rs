use gl::types::GLuint;
use nalgebra::{Vector3, Vector2};

use crate::asset::Texture;

pub struct Mesh {
    element_count: i32,
    vao_id: GLuint,
    ebo_id: GLuint,
    buffers: Vec<GLuint>,
    texture: Option<Texture>
}

impl Mesh {
    pub fn new(element_count: i32, indices: &Vec<u32>, vertices: &Vec<Vector3<f32>>, colors: Option<&[f32]>, uvs: Option<&Vec<Vector2<f32>>>, texture: Option<Texture>) -> Self {
        let mut vao_id = 0;
        unsafe { 
            gl::GenVertexArrays(1, &mut vao_id); 
            gl::BindVertexArray(vao_id) 
        };

        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }

        let mut buffers = Vec::new();

        let mut vertex_id = 0;
        unsafe { 
            gl::GenBuffers(1, &mut vertex_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * 3 * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(buffers.len() as u32);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                0 as *const _,
            );
        };
        buffers.push(vertex_id);

        if let Some(color_buffer) = colors {
            let mut color_id = 0;
            unsafe { 
                gl::GenBuffers(1, &mut color_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, color_id);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    std::mem::size_of_val(color_buffer) as isize,
                    color_buffer.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );

                gl::EnableVertexAttribArray(buffers.len() as u32);
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    3 * std::mem::size_of::<f32>() as i32,
                    0 as *const _,
                );
            };
            buffers.push(color_id);
        }

        if let Some(uv_buffer) = uvs {
            let mut uv_id = 0;
            unsafe {
                gl::GenBuffers(1, &mut uv_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, uv_id);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (uv_buffer.len() * 2 * std::mem::size_of::<f32>()) as isize,
                    uv_buffer.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );

                gl::EnableVertexAttribArray(buffers.len() as u32);
                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    2 * std::mem::size_of::<f32>() as i32,
                    0 as *const _,
                );
            };
            buffers.push(uv_id);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            element_count,
            vao_id,
            ebo_id,
            buffers,
            texture
        }
    }

    pub fn draw(&self) {
        unsafe {
            if let Some(texture) = &self.texture {
                texture.bind();
            }
        
            gl::BindVertexArray(self.vao_id);
            for i in 0..self.buffers.len() {
                gl::EnableVertexAttribArray(i as u32);
            }
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            gl::DrawElements(gl::TRIANGLES, self.element_count, gl::UNSIGNED_INT, std::ptr::null());
            for i in 0..self.buffers.len() {
                gl::DisableVertexAttribArray(i as u32);
            }
            gl::BindVertexArray(0);

            if let Some(texture) = &self.texture {
                texture.unbind();
            }
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.buffers.len() as i32, self.buffers.as_ptr());
            gl::DeleteVertexArrays(1, &self.vao_id);
        }
    }
}
