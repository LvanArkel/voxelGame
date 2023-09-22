use gl::types::GLuint;

use crate::Texture;

pub struct ColoredMesh {
    vertex_count: i32,
    vao_id: GLuint,
    vertex_id: GLuint,
    color_id: GLuint,
}

impl ColoredMesh {
    pub fn new(vertex_count: i32, vertices: &[f32], colors: &[f32]) -> Self {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };

        let mut vertex_id = 0;
        unsafe { gl::GenBuffers(1, &mut vertex_id) };
        let mut color_id = 0;
        unsafe { gl::GenBuffers(1, &mut color_id) };

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);

            gl::BindBuffer(gl::ARRAY_BUFFER, color_id);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(colors) as isize, colors.as_ptr().cast(), gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);          
        }

        Self { vertex_count, vao_id: vao, vertex_id, color_id }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}

impl Drop for ColoredMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(2, [self.vertex_id, self.color_id].as_ptr());
            gl::DeleteVertexArrays(1, &self.vao_id);
        }
    }
}

pub struct TexturedMesh {
    vertex_count: i32,
    vao_id: GLuint,
    vertex_id: GLuint,
    uv_id: GLuint,
    texture: Texture,
}

impl TexturedMesh {
    pub fn new(vertex_count: i32, vertices: &[f32], uvs: &[f32], texture: Texture) -> Self {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };

        let mut vertex_id = 0;
        unsafe { gl::GenBuffers(1, &mut vertex_id) };
        let mut uv_id = 0;
        unsafe { gl::GenBuffers(1, &mut uv_id) };

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);

            gl::BindBuffer(gl::ARRAY_BUFFER, uv_id);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(uvs) as isize, uvs.as_ptr().cast(), gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as i32, 0 as *const _);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);          
        }

        Self { vertex_count, vao_id: vao, vertex_id, uv_id, texture }
    }

    pub fn draw(&self) {
        unsafe {
            self.texture.bind();
            gl::BindVertexArray(self.vao_id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            self.texture.unbind();
        }
    }
}

impl Drop for TexturedMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(2, [self.vertex_id, self.uv_id].as_ptr());
            gl::DeleteVertexArrays(1, &self.vao_id);
        }
    }
}