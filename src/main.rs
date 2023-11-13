use std::{path::Path, time::Instant};

use glfw::Context;
use gl;
use nalgebra::{Isometry3, Point3};

use voxel_game::{asset::{Shader, Texture}, camera::Camera, rendering::{MeshRenderer, Mesh}, world::{chunk::{Chunk, CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z}, voxel::Voxel}};

struct WindowSettings {
    wireframe: bool,
}

impl WindowSettings {
    fn new() -> Self { Self { wireframe: false } }

    fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        if self.wireframe {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        } else {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

fn print_usage() {
    println!("Controls:");
    println!("Y - toggle wireframe mode");
}

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

    let mut window_settings = WindowSettings::new();

    // let texture = Texture::new(&Path::new("resources/texture/cobblestone.png"));
    let shader = Shader::from_file("resources/shader/textured.vert", "resources/shader/textured.frag");
    
    let mut chunks: Vec<Vec<Chunk>> = Vec::new();
    for _ in 0..4 {
        let mut chunk_row = Vec::new();
        for _ in 0..4 {
            // let mut voxels = vec![None; (CHUNK_SIZE_X*CHUNK_SIZE_Y*CHUNK_SIZE_Z) as usize];
            // voxels[0] = Some(Voxel{});
            // voxels[1] = Some(Voxel{});
            // voxels[CHUNK_SIZE_X as usize] = Some(Voxel{});
            // voxels[(CHUNK_SIZE_X*CHUNK_SIZE_Y) as usize] = Some(Voxel{});
            // let chunk = Chunk::new(voxels);
            let chunk = Chunk::new(vec![Some(Voxel{}); (CHUNK_SIZE_X*CHUNK_SIZE_Y*CHUNK_SIZE_Z) as usize]);
            chunk_row.push(chunk);
        }
        chunks.push(chunk_row);
    }

    let meshes: Vec<Vec<Mesh>> = chunks
        .iter()
        .map(|chunk_row| 
            chunk_row
                .iter()
                .map(|chunk| {
                    let texture = Texture::new(&Path::new("resources/texture/cobblestone.png")).unwrap();
                    chunk.generate_mesh(texture)
                }).collect()).collect();

    // let chunk = Chunk::new(voxels);
    // let chunk_mesh = chunk.generate_mesh(texture);
    let renderer = MeshRenderer::new(shader);

    let camera = Camera::new_look_at(
        screen_width, screen_height,
        &Point3::new(4.0, 12.0, -8.0),
        // &Point3::new(-2.0, 4.0, -3.0),
        &Point3::new(16.0, 4.0, 16.0),   
        // &Point3::new(0.5,0.5,0.5),
    );

    print_usage();

    let mut instant = Instant::now();
    let mut fps = 0.0;
    while !window.should_close() {
        let elapsed = instant.elapsed();
        let delta = elapsed.as_secs_f32();
        fps = 0.95 * fps + 0.05 * (1.0/delta);
        window.set_title(format!("{}", fps).as_str());
        instant = Instant::now();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event, &mut window_settings);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (z, mesh_row) in meshes.iter().enumerate() {
            for (x, mesh) in mesh_row.iter().enumerate() {
                renderer.render(&Isometry3::translation((x*CHUNK_SIZE_X as usize) as f32, 0.0, (z*CHUNK_SIZE_Z as usize) as f32), mesh, &camera);
            }
        }

        window.swap_buffers();
    }
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent, window_settings: &mut WindowSettings) {
    use glfw::WindowEvent as Event;
    use glfw::Key;
    use glfw::Action;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        },
        Event::Key(Key::Y, _, Action::Press, _) => {
            window_settings.toggle_wireframe();
        }
        _ => {},
    }
}
