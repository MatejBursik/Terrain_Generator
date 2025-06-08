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
    player.x = 0.0;
    player.y = 0.0;
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

    // Load spaceship (3D object)
    /*
    let obj_loader = object_loader::ObjectLoader::new("resources/spaceship/placeholder_spaceship.gltf");
    let spaceship_vertices = obj_loader.get_vertices();
    let spaceship_indices = obj_loader.get_indices();
    let spaceship_triangle_count = obj_loader.get_triangle_count();

    println!("{:?}", spaceship_vertices);
    */

    // Initialize application
    let mut window = window::Window::new(1200, 720, "Terrain Generator");
    window.init_gl();
    window.set_fps(1);

    // Setup terrain rendering
    let terrain_vao = vao::ArrayObject::new();
    terrain_vao.bind();

    let terrain_vbo = vbo::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    terrain_vbo.bind();
    terrain_vbo.store_f32_data(&vertices);

    let terrain_ibo = vbo::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    terrain_ibo.bind();
    terrain_ibo.store_i32_data(&indices);

    let terrain_position_attribute = v_attribute::VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    terrain_position_attribute.enable();

    // Load shaders for terrain
    let mut terrain_shader = shader_reader::ShaderReader::new("resources/terrain/vertex_shader.glsl", "resources/terrain/fragment_shader.glsl");
    terrain_shader.bind();

    // Setup spaceship rendering
    /*
    let spaceship_vao = vao::ArrayObject::new();
    spaceship_vao.bind();

    let spaceship_vbo = vbo::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    spaceship_vbo.bind();
    spaceship_vbo.store_f32_data(spaceship_vertices);

    let spaceship_ibo = vbo::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    spaceship_ibo.bind();
    spaceship_ibo.store_i32_data(&spaceship_indices);

    let spaceship_position_attribute = v_attribute::VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    spaceship_position_attribute.enable();

    // Load shaders for spaceship
    let mut spaceship_shader = shader_reader::ShaderReader::new("resources/spaceship/vertex_shader.glsl", "resources/spaceship/fragment_shader.glsl");
    spaceship_shader.bind();
    */

    // Create a transformation matrix and apply it to the shader
    let mut terrain_transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_angle_x(Rad(PI/3.0)) * Matrix4::from_scale(0.75);
    terrain_shader.create_uniform("transform");
    terrain_shader.set_matrix4fv_uniform("transform", &terrain_transform);

    /*
    let mut spaceship_transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_scale(0.25);
    spaceship_shader.create_uniform("transform");
    spaceship_shader.set_matrix4fv_uniform("transform", &spaceship_transform);
    */

    // Setup Z-buffer (depth) testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !window.close() {
        player.has_moved = false;

        // QE for rotation
        if window.is_key_pressed(Key::E) {
            terrain_transform = terrain_transform * Matrix4::from_angle_z(Rad(rotate_value));
            //spaceship_transform = spaceship_transform * Matrix4::from_angle_z(Rad(rotate_value));
            player.direction += rotate_value;
            println!("{}", player.direction);
        }
        if window.is_key_pressed(Key::Q) {
            terrain_transform = terrain_transform * Matrix4::from_angle_z(-Rad(rotate_value));
            //spaceship_transform = spaceship_transform * Matrix4::from_angle_z(-Rad(rotate_value));
            player.direction -= rotate_value;
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
            terrain_vbo.bind();
            terrain_vbo.store_f32_data(&vertices);
            
            terrain_ibo.bind();
            terrain_ibo.store_i32_data(&indices);
        }

        unsafe {
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Render terrain
            terrain_vao.bind();
            terrain_shader.bind();
            terrain_shader.set_matrix4fv_uniform("transform", &terrain_transform);
            gl::DrawElements(gl::TRIANGLES, triangle_count*3, gl::UNSIGNED_INT, ptr::null());
            terrain_shader.unbind();

            // Render spaceship
            /*
            spaceship_vao.bind();
            spaceship_shader.bind();
            spaceship_shader.set_matrix4fv_uniform("transform", &spaceship_transform);
            gl::DrawElements(gl::TRIANGLES, spaceship_triangle_count, gl::UNSIGNED_INT, ptr::null());
            spaceship_shader.unbind();
            */
        }
        window.update();
    }
}