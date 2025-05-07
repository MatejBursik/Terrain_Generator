use std::{f32::consts::PI, ptr};
use gl::types::*;
use cgmath::{Matrix4, Rad, Vector3};
use glfw::Key;

mod graphics;
mod perlin_noise;
mod functions;
mod structs;

use graphics::*;
use perlin_noise::PerlinMap;
use functions::generate_mesh;
use structs::Player;

fn main() {
    // Setup Perlin noise map
    let mut perlin_map = PerlinMap::new();
    println!("{:?}", perlin_map);
    perlin_map.generate_vec_map(20, 20);
    
    //Initialize player
    let mut player = Player::new();
    player.x = 18.0;
    player.y = 16.0;
    player.speed = 0.01;
    let rotate_value = PI/500.0;

    // Initialize map
    let plain_h = 10;
    let plain_w = 10;
    let scale = 0.2;

    let mut r = generate_mesh(scale, plain_h, plain_w, player.x, player.y, &perlin_map);
    let mut triangle_count: i32 = r.2;
    let mut vertices: Vec<f32> = r.0;
    let mut indices: Vec<i32> = r.1;

    // Initialize application
    let mut window = window::Window::new(1200, 720, "Terrain Generator");
    window.init_gl();
    window.set_fps(1);

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
        player.has_moved = false;

        // QE for rotation
        if window.is_key_pressed(Key::Q) {
            transform = transform * Matrix4::from_angle_z(-Rad(rotate_value));
            player.direction -= rotate_value;
            println!("{}", player.direction);
        }
        if window.is_key_pressed(Key::E) {
            transform = transform * Matrix4::from_angle_z(Rad(rotate_value));
            player.direction += rotate_value;
            println!("{}", player.direction);
        }

        // Calculate movement direction based on player's direction
        let mut dx = 0.0;
        let mut dy = 0.0;

        // WASD movement relative to player direction
        if window.is_key_pressed(Key::W) {
            dx += player.speed * player.direction.sin();
            dy += player.speed * player.direction.cos();
        }
        if window.is_key_pressed(Key::S) {
            dx -= player.speed * player.direction.sin();
            dy -= player.speed * player.direction.cos();
        }
        if window.is_key_pressed(Key::A) {
            dx -= player.speed * player.direction.cos();
            dy += player.speed * player.direction.sin();
        }
        if window.is_key_pressed(Key::D) {
            dx += player.speed * player.direction.cos();
            dy -= player.speed * player.direction.sin();
        }
        
        // Apply movement if within bounds
        if dx != 0.0 || dy != 0.0 {
            let new_x = player.x + dx;
            let new_y = player.y + dy;
            
            if perlin_map.is_valid_coord(scale, plain_h, plain_w, new_x, new_y) {
                player.x = new_x;
                player.y = new_y;
                player.has_moved = true;
                println!("x = {}, y = {}", player.x, player.y);
            } else {
                println!("Edge");
            }
        }

        // Regenerate mesh if player moved
        if player.has_moved {
            r = generate_mesh(scale, plain_h, plain_w, player.x, player.y, &perlin_map);
            vertices = r.0;
            indices = r.1;
            triangle_count = r.2;
            
            // Update VBO and IBO with new data
            vbo.bind();
            vbo.store_f32_data(&vertices);
            
            ibo.bind();
            ibo.store_i32_data(&indices);
        }

        unsafe {
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.bind();
            shader.set_matrix4fv_uniform("transform", &transform);
            gl::DrawElements(gl::TRIANGLES, triangle_count*3, gl::UNSIGNED_INT, ptr::null());
            shader.unbind();
        }
        window.update();
    }
}