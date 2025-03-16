use std::{f32::consts::PI, ptr};
use gl::types::*;
use cgmath::{Matrix4, Rad, Vector3};
use std::time::{Duration, Instant};

mod graphics;
mod perlin_noise;
mod generate_mesh;

use graphics::*;
use perlin_noise::PerlinMap;
use generate_mesh::generate_mesh;

fn main() {
    // FPS limiting variables
    let target_fps = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);

    // Setup Perlin noise map
    let mut perlin_map = PerlinMap::new();
    println!("{:?}", perlin_map);
    println!("noise: {}", perlin_map.noise(0.3, 0.4));
    perlin_map.generate_vec_map(10, 10);

    // Initialize map
    let map_h = 10;
    let map_w = 10;
    let scale = 0.3;

    let r = generate_mesh(scale, map_h, map_w, 0.0, &perlin_map);
    let triangle_count: i32 = r.2;
    let vertices: Vec<f32> = r.0;
    let indices: Vec<i32> = r.1;

    // Initialize application
    let mut window = window::Window::new(1200, 720, "Terrain Generator");
    window.init_gl();

    let vao = vao::ArrayObject::new();
    vao.bind();

    let vbo = vbo::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let ibo = vbo::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();
    ibo.store_i32_data(&indices);

    let position_attribute = v_attribute::VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    position_attribute.enable();

    // Load shaders
    let mut shader = shader_reader::ShaderReader::new("resources/vertex_shader.glsl", "resources/fragment_shader.glsl");
    shader.bind();

    // Create a transformation matrix and apply it to the shader
    let mut transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_angle_x(Rad(PI/3.0)) * Matrix4::from_scale(0.75);
    shader.create_uniform("transform");
    shader.set_matrix4fv_uniform("transform", &transform);

    // Setup Z-buffer (depth) testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !window.close() {
        let frame_start = Instant::now();

        transform = transform * Matrix4::from_angle_z(Rad(PI/1000.0));

        unsafe {
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.bind();
            shader.set_matrix4fv_uniform("transform", &transform);
            gl::DrawElements(gl::TRIANGLES, triangle_count*3, gl::UNSIGNED_INT, ptr::null());
            shader.unbind();
        }
        window.update();

        // Calculate how long to sleep to maintain target FPS
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}