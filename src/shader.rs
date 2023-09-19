use gl;
use gl::types::{GLuint, GLint};
use nalgebra::Matrix4;
use std::ffi::CString;
use std::fs;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) };
    }

    pub fn uniform_int(&self, name: &str, value: i32) {
        unsafe {
            let name = CString::new(name).expect(format!("Invalid name {}", name).as_str());
            let location = gl::GetUniformLocation(self.id, name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    pub fn uniform_float(&self, name: &str, value: f32) {
        unsafe {
            let name = CString::new(name).expect(format!("Invalid name {}", name).as_str());
            let location = gl::GetUniformLocation(self.id, name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn uniform_mat4(&self, name: &str, value: Matrix4<f32>) {
        unsafe {
            let name = CString::new(name).expect(format!("Invalid name {}", name).as_str());
            let location = gl::GetUniformLocation(self.id, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr())
        }
    }

    pub fn from_file(vertex_filename: &str, fragment_filename: &str) -> Self {
        let vertex_code = fs::read_to_string(vertex_filename)
            .expect(format!("File {} does not exist", vertex_filename).as_str());
        let fragment_code = fs::read_to_string(fragment_filename)
            .expect(format!("File {} does not exist", fragment_filename).as_str());

        let id = load_shader(vertex_code, fragment_code);
        Shader{ id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn load_shader(vertex_code: String, fragment_code: String) -> GLuint {
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &vertex_code.as_bytes().as_ptr().cast(), &vertex_code.len().try_into().unwrap());
        gl::CompileShader(vertex_shader);
        
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            // gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_len);
            // let mut v: Vec<u8> = Vec::with_capacity(log_len as usize);
            // gl::GetShaderInfoLog(vertex_shader, log_len, &mut log_len, v.as_mut_ptr().cast());
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &fragment_code.as_bytes().as_ptr().cast(), &fragment_code.len().try_into().unwrap());
        gl::CompileShader(fragment_shader);
        
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    let shader_program: GLuint = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    shader_program
}
