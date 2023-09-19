use std::path::Path;

// use glfw::Context;
use glutin::{event_loop::{EventLoop, ControlFlow}, ContextBuilder, GlRequest, Api, event::{Event, WindowEvent}, window::WindowBuilder, dpi::PhysicalSize};
use gl;
use nalgebra::{Perspective3, Matrix4, Translation3, Matrix, Vector4, Vector3, Rotation3, Isometry3, Point3, Projective3};

use voxel_game::{Shader, Texture};


fn main() {
    // let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    // glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    // glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw.window_hint(glfw::WindowHint::Resizable(false));

    let screen_width = 1280;
    let screen_height = 720;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize{width: screen_width, height: screen_height})
        .with_title("Test");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create window");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    // let (mut window, events) = glfw.create_window(screen_width, screen_height, "Test", glfw::WindowMode::Windowed)
    //     .expect("Failed to open GLFW window.");

    // window.make_current();
    // window.set_key_polling(true);
    // gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width as i32, screen_height as i32);
        gl::Enable(gl::TEXTURE_2D);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let shader = Shader::from_file("resources/shader/default.vert", "resources/shader/default.frag");  

    const VERTEX_COUNT: usize = 6*6;
    let vertices: [f32;VERTEX_COUNT*3] = [
        -1.0,-1.0,-1.0, // triangle 1 : begin
        -1.0,-1.0, 1.0,
        -1.0, 1.0, 1.0, // triangle 1 : end
        1.0, 1.0,-1.0, // triangle 2 : begin
        -1.0,-1.0,-1.0,
        -1.0, 1.0,-1.0, // triangle 2 : end
        1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0,-1.0,
        1.0,-1.0,-1.0,
        -1.0,-1.0,-1.0,
        -1.0,-1.0,-1.0,
        -1.0, 1.0, 1.0,
        -1.0, 1.0,-1.0,
        1.0,-1.0, 1.0,
        -1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0,
        -1.0, 1.0, 1.0,
        -1.0,-1.0, 1.0,
        1.0,-1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0,-1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0, 1.0,
        1.0,-1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0,-1.0,
        -1.0, 1.0,-1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0,-1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0,-1.0, 1.0
    ];

    let colors: [f32; VERTEX_COUNT*3] = [
        0.583,  0.771,  0.014,
        0.609,  0.115,  0.436,
        0.327,  0.483,  0.844,
        0.822,  0.569,  0.201,
        0.435,  0.602,  0.223,
        0.310,  0.747,  0.185,
        0.597,  0.770,  0.761,
        0.559,  0.436,  0.730,
        0.359,  0.583,  0.152,
        0.483,  0.596,  0.789,
        0.559,  0.861,  0.639,
        0.195,  0.548,  0.859,
        0.014,  0.184,  0.576,
        0.771,  0.328,  0.970,
        0.406,  0.615,  0.116,
        0.676,  0.977,  0.133,
        0.971,  0.572,  0.833,
        0.140,  0.616,  0.489,
        0.997,  0.513,  0.064,
        0.945,  0.719,  0.592,
        0.543,  0.021,  0.978,
        0.279,  0.317,  0.505,
        0.167,  0.620,  0.077,
        0.347,  0.857,  0.137,
        0.055,  0.953,  0.042,
        0.714,  0.505,  0.345,
        0.783,  0.290,  0.734,
        0.722,  0.645,  0.174,
        0.302,  0.455,  0.848,
        0.225,  0.587,  0.040,
        0.517,  0.713,  0.338,
        0.053,  0.959,  0.120,
        0.393,  0.621,  0.362,
        0.673,  0.211,  0.457,
        0.820,  0.883,  0.371,
        0.982,  0.099,  0.879
    ];

    // const VERTEX_COUNT: usize = 6;
    // let vertices: [f32; VERTEX_COUNT*3] = [
    //     -1.0, -1.0, 0.0,
    //      1.0, -1.0, 0.0,
    //     -1.0,  1.0, 0.0,
    //     -1.0,  1.0, 0.0,
    //      1.0,  1.0, 0.0,
    //      1.0, -1.0, 0.0,
    // ];

    // let colors: [f32; VERTEX_COUNT*3] = [
    //     1.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0,
    //     0.0, 0.0, 1.0,
    //     0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0,
    //     0.0, 1.0, 0.0,
    // ];

    // let uvs: [f32; VERTEX_COUNT*2] = [
    //     0.0, 0.0,
    //     1.0, 0.0,
    //     0.0, 1.0,
    //     0.0, 1.0,
    //     1.0, 1.0,
    //     1.0, 0.0,
    // ];

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    let mut vertex_vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vertex_vbo) };

    let mut color_vbo = 0;
    unsafe { gl::GenBuffers(1, &mut color_vbo) };

    // let mut uv_vbo = 0;
    // unsafe { gl::GenBuffers(1, &mut uv_vbo) };

    unsafe {
        gl::ClearColor(0.4, 0.4, 0.4, 1.0);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);

        gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&colors) as isize, colors.as_ptr().cast(), gl::STATIC_DRAW);

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);

        // gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        // gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&uvs) as isize, uvs.as_ptr().cast(), gl::STATIC_DRAW);

        // gl::EnableVertexAttribArray(1);
        // gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as i32, 0 as *const _);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // let texture = Texture::new(Path::new("resources/texture/cobblestone.png"));
    
    let model = Isometry3::new(nalgebra::zero(), nalgebra::zero());

    let camera_pos = Point3::new(2.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&camera_pos, &look_at, &Vector3::y());

    let projection = Perspective3::new(screen_width as f32 / screen_height as f32, 3.14 / 2.0, 0.1, 1000.0);

    let mvp = projection.as_matrix() * (view * model).to_homogeneous();
    // let mvp: Matrix4<f32> = model.to_homogeneous();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    
                    shader.bind();
                    shader.uniform_mat4("mvp", mvp);
                    // shader.uniform_float("scalar", 2.5);
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, VERTEX_COUNT as i32);
                    shader.unbind();
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });

    // while !window.should_close() {
    //     glfw.poll_events();
    //     for (_, event) in glfw::flush_messages(&events) {
    //         glfw_handle_event(&mut window, event);
    //     }

    //     unsafe {
    //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //     }

    //     // texture.bind();
    //     shader.bind();
    //     shader.uniform_mat4("mvp", mvp);
    //     unsafe {
    //         gl::BindVertexArray(vao);
    //         gl::DrawArrays(gl::TRIANGLES, 0, VERTEX_COUNT as i32);
    //     }
    //     shader.unbind();

        
    //     window.swap_buffers();
    // }
}

// pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
//     let v = unsafe { gl::GetString(name) };
//     let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
//     v.to_str().unwrap()
// }

// fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
//     use glfw::WindowEvent as Event;
//     use glfw::Key;
//     use glfw::Action;

//     match event {
//         Event::Key(Key::Escape, _, Action::Press, _) => {
//             window.set_should_close(true);
//         },
//         _ => {},
//     }
// }
