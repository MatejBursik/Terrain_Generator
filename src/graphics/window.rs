use glfw::{Action, Context, Key, WindowEvent};
use std::collections::HashSet;
use std::sync::mpsc::Receiver;

// Application window
/* Example:
let mut window = Window::new(1280, 720, "Window");
window.init_gl();

while !window.close() {
    unsafe {
        gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.update();
}
*/

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    keys_pressed: HashSet<Key>
}

impl Window {
    // Create new window with settings
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {glfw, window_handle: window, events, keys_pressed: HashSet::new()}
    }

    // Load gl functions.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the new window dimensions.
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                glfw::WindowEvent::Key(key, _, action, _) => match action {
                    Action::Press => {
                        self.keys_pressed.insert(key);
                        if key == Key::Escape {
                            self.window_handle.set_should_close(true);
                        }
                    },
                    Action::Release => {
                        self.keys_pressed.remove(&key);
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn set_fps(&mut self, interval: i32) {
        self.window_handle.make_current();
        match interval {
            -1 => self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive),
            0 => self.glfw.set_swap_interval(glfw::SwapInterval::None),
            1 => self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1)),
            n if n > 1 => {
                // Many GLFW implementations only support intervals of 0, 1, or adaptive vsync
                // But try to set the specific interval if possible
                // The swap interval mechanism (glfw.set_swap_interval()) only allows you to synchronize
                //  with fractions or multiples of the monitor's refresh rate.
                // Input of 2 or 3 means Sync to every 2nd or 3rd frame (30 FPS, 20 FPS on 60Hz)
                self.glfw.set_swap_interval(glfw::SwapInterval::Sync(n as u32))
            },
            _ => {
                self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1))
            }
        }
    }
}