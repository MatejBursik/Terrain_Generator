use gl::types::*;
use std::ptr;

const SCREEN_WIDTH: usize = 200;
//const SCREEN_HEIGHT: usize = 60;
const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

pub struct RayCaster {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    fov: f32,
    depth: f32,
    speed: f32,
    map: Vec<char>
}

impl RayCaster {
    pub fn new() -> Self {
        RayCaster {
            player_x: 10.5,
            player_y: 5.5,
            player_angle: 0.0,
            fov: std::f32::consts::PI / 4.0,
            depth: 16.0,
            speed: 5000.0,
            map: vec![
                '#','#','#','#','#','#','#','#','#','#','#','#','#','#','#','#',
                '#','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','.','.','.','#','#','#','#','#','#','#','#',
                '#','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','#','#','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','#','#','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '#','#','#','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','#','#','#','#','.','.','#','#','#','#','#',
                '#','.','.','.','.','#','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','#','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '#','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '.','.','.','.','.','#','#','#','#','#','#','#','#','#','#','#',
                '.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','#',
                '.','.','.','#','#','#','#','#','#','#','#','#','#','#','#','#'
            ]
        }
    }

    pub fn handle_input(&mut self, keys: &[bool], elapsed_time: f32) {
        // Rotation
        if keys[0] { // 'A'
            self.player_angle -= (self.speed * 0.75) * elapsed_time;
        }
        if keys[1] { // 'D'
            self.player_angle += (self.speed * 0.75) * elapsed_time;
        }

        // Forward movement
        if keys[2] { // 'W'
            let new_x = self.player_x + self.player_angle.sin() * self.speed * elapsed_time;
            let new_y = self.player_y + self.player_angle.cos() * self.speed * elapsed_time;
            
            if self.is_valid_move(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }

        // Backward movement
        if keys[3] { // 'S'
            let new_x = self.player_x - self.player_angle.sin() * self.speed * elapsed_time;
            let new_y = self.player_y - self.player_angle.cos() * self.speed * elapsed_time;
            
            if self.is_valid_move(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
    }

    fn is_valid_move(&self, x: f32, y: f32) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;
        if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT {
            return false;
        }
        self.map[map_y * MAP_WIDTH + map_x] != '#'
    }

    pub fn ray_cast(&self) -> Vec<(f32, bool)> {
        let mut ray_results = Vec::new();

        for x in 0..SCREEN_WIDTH {
            let ray_angle = self.player_angle - self.fov/2.0 + 
                ((x as f32) / (SCREEN_WIDTH as f32)) * self.fov;

            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;

            while !hit_wall && distance_to_wall < self.depth {
                distance_to_wall += 0.1;
                
                let test_x = (self.player_x + eye_x * distance_to_wall) as usize;
                let test_y = (self.player_y + eye_y * distance_to_wall) as usize;

                if test_x >= MAP_WIDTH || test_y >= MAP_HEIGHT {
                    hit_wall = true;
                    distance_to_wall = self.depth;
                } else if self.map[test_y * MAP_WIDTH + test_x] == '#' {
                    hit_wall = true;
                }
            }

            ray_results.push((distance_to_wall, hit_wall));
        }

        ray_results
    }

    pub fn render(&self, ray_results: &Vec<(f32, bool)>, window: &mut glfw::Window) {
        // Generate and bind a Vertex Array Object (VAO)
        let mut vao: GLuint = 0;
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
        
            for (x, (distance, hit_wall)) in ray_results.iter().enumerate() {
                // Normalize distance for rendering
                let normalized_height = 1.0 - (*distance / self.depth);
                let normalized_x = (x as f32 / SCREEN_WIDTH as f32) * 2.0 - 1.0;
                
                // Define vertices for the vertical line
                let vertices: [f32; 4] = [
                    normalized_x, -normalized_height,
                    normalized_x, normalized_height, 
                ];

                // Generate and bind a Vertex Buffer Object (VBO) 
                let mut vbo: GLuint = 0;
                gl::GenBuffers(1, &mut vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &vertices[0] as *const f32 as *const _, gl::STATIC_DRAW);
                    
                // Set the vertex attributes pointers
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
                gl::EnableVertexAttribArray(0);
                
                // Draw the line
                gl::DrawArrays(gl::LINES, 0, 2);
                
                // Unbind the VBO and delete it
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::DeleteBuffers(1, &vbo);
            }
        }

        // Unbind the VAO and delete it
        unsafe {
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &vao);
        }

        println!("{}, {}, {}", self.player_x, self.player_y, self.player_angle);
    }
}

// main.rs

use gl;
use glfw::{Action, Context, Key};
use std::time::Instant;
use std::ffi::c_void;

//mod ray_caster;
//mod perlin_noise;

//use ray_caster::RayCaster;
//use perlin_noise::PerlinMap;

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

    // Setup Perlin noise map
    /*
    let mut map = PerlinMap::new();
    println!("{:?}", map);
    println!("noise: {}", map.noise(0.0, 0.0));
    map.generate_vec_map(5, 5);*/
    
    let mut raycaster = RayCaster::new();
    let mut keys = [false; 4]; // W, A, S, D
    let mut modes = false; // true = First person view, false = Landscape view

    while !window.should_close() {
        glfw.poll_events();
        
        // Process events
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => modes = !modes,
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

        // true = First person view
        // false = Landscape view
        if modes {
            // First person view
            let now = Instant::now();
            let elapsed_time = now.elapsed().as_secs_f32();

            raycaster.handle_input(&keys, elapsed_time);
            let ray_results = raycaster.ray_cast();
            raycaster.render(&ray_results, &mut window);

            // Swap buffers to display rendered frame
            window.swap_buffers();
        } else {
            // Landscape view
        }
    }
}