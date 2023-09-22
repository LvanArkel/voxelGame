use std::path::Path;

use glfw::Context;
// use glutin::{event_loop::{EventLoop, ControlFlow}, ContextBuilder, GlRequest, Api, event::{Event, WindowEvent}, window::WindowBuilder, dpi::PhysicalSize};
use gl;
use nalgebra::{Perspective3, Vector3, Isometry3, Point3, UnitQuaternion};

use voxel_game::{Shader, Texture, cube_mesh};


fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let screen_width = 1280;
    let screen_height = 720;

    let (mut window, events) = glfw.create_window(screen_width, screen_height, "Test", glfw::WindowMode::Windowed)
        .expect("Failed to open GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width as i32, screen_height as i32);
        gl::Enable(gl::TEXTURE_2D);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::DepthFunc(gl::LESS);
    }

    const VERTEX_COUNT: usize = 6;
    let vertices: [f32;VERTEX_COUNT*3] = [
        -1.0, -1.0, 0.0,
         1.0, -1.0, 0.0,
        -1.0,  1.0, 0.0,
        -1.0,  1.0, 0.0,
         1.0, -1.0, 0.0,
         1.0,  1.0, 0.0,
    ];

    let colors: [f32; VERTEX_COUNT*3] = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 1.0, 0.0,
        1.0, 1.0, 1.0,
    ];

    let uvs: [f32; VERTEX_COUNT*2] = [
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0,
        0.0, 0.0,
        1.0, 1.0,
        1.0, 0.0,
    ];

    let texture = Texture::new(&Path::new("resources/texture/cobblestone.png"));

    // let shader = Shader::from_file("resources/shader/default.vert", "resources/shader/default.frag");  
    // let mesh = ColoredMesh::new(VERTEX_COUNT as i32, &vertices, &colors);
    
    let shader = Shader::from_file("resources/shader/textured.vert", "resources/shader/textured.frag");
    // let mesh = TexturedMesh::new(VERTEX_COUNT as i32, &vertices, &uvs, texture);
    let mesh = cube_mesh(1.0, texture);
    
    let mut model = Isometry3::new(nalgebra::zero(), nalgebra::zero());

    let camera_pos = Point3::new(2.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&camera_pos, &look_at, &Vector3::y());

    let projection = Perspective3::new(screen_width as f32 / screen_height as f32, 3.14 / 2.0, 0.1, 1000.0);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        model.append_rotation_mut(&UnitQuaternion::from_euler_angles(0.0, 0.001, 0.0));

        let mvp = projection.as_matrix() * (view * model).to_homogeneous();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // texture.bind();
        shader.bind();
        shader.uniform_mat4("mvp", mvp);
        shader.uniform_int("texture0", 0);
        mesh.draw();
        shader.unbind();

        
        window.swap_buffers();
    }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::WindowEvent as Event;
    use glfw::Key;
    use glfw::Action;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        },
        _ => {},
    }
}
