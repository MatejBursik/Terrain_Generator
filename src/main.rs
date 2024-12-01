use gl;
use glfw::{Action, Context, Key};
use std::time::Instant;
use std::ffi::c_void;

mod ray_caster;

use ray_caster::RayCaster;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Configure OpenGL context
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(
        800, 600, 
        "Rust Raycaster", 
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const c_void);

    // OpenGL setup
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut raycaster = RayCaster::new();
    let mut keys = [false, false, false, false]; // A, D, W, S

    while !window.should_close() {
        glfw.poll_events();
        
        // Process events
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => keys[0] = true,
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => keys[0] = false,
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => keys[1] = true,
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => keys[1] = false,
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => keys[2] = true,
                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => keys[2] = false,
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => keys[3] = true,
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => keys[3] = false,
                _ => {}
            }
        }

        // Update game state
        let now = Instant::now();
        let elapsed_time = now.elapsed().as_secs_f32();

        raycaster.handle_input(&keys, elapsed_time);
        let ray_results = raycaster.ray_cast();

        // Render raycasting results
        raycaster.render(&ray_results, &mut window);

        // Swap buffers to display rendered frame
        window.swap_buffers();
    }
}