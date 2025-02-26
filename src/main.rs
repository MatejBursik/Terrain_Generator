use std::{f32::consts::PI, ffi::CString, ptr};
use gl::types::*;
use cgmath::{Matrix4, Rad, Vector3};

mod graphics;
mod perlin_noise;

use graphics::*;
use perlin_noise::PerlinMap;

fn main() {
    // Setup Perlin noise map
    let mut p_map = PerlinMap::new();
    println!("{:?}", p_map);
    println!("noise: {}", p_map.noise(0.3, 0.4));
    p_map.generate_vec_map(5, 5);

    // Initialize map
    let map_h = 10;
    let map_w = 10;
    let mut triangle_count = 0;
    let mut vertices:Vec<f32> = Vec::new();
    let mut indices:Vec<i32> = Vec::new();

    // Populate map
    for i in 0 .. map_h * map_w{
        let x = (i % map_w) as f32; // Column
        let y = (i / map_w) as f32; // Row
        let z = p_map.noise(x / 10.0, y / 10.0);

        // Normalize to UV coordinates (0.0 to 1.0)
        let u = (x as f32 / (map_w - 1) as f32) * 2.0 - 1.0; // X
        let v = (y as f32 / (map_h - 1) as f32) * 2.0 - 1.0; // Y

        vertices.push(u);
        vertices.push(v);
        vertices.push(z);

        // Generate indices (except for the last row and column)
        if x < (map_w - 1) as f32 && y < (map_h - 1) as f32 {
            let top_left = i as i32;
            let top_right = (i + 1) as i32;
            let bottom_left = (i + map_w) as i32;
            let bottom_right = (i + map_w + 1) as i32;

            // First triangle (Top Left, Bottom Left, Bottom Right)
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(bottom_right);
            triangle_count += 1;

            // Second triangle (Top Left, Bottom Right, Top Right)
            indices.push(top_left);
            indices.push(bottom_right);
            indices.push(top_right);
            triangle_count += 1;
        }
    }

    println!("{:?}", vertices);
    println!("{:?}", indices);
    println!("{triangle_count}");

    // Initialize application
    let mut window = window::Window::new(720, 720, "Terrain Generator");
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

    // Create shader program
    let mut shader = shader_reader::ShaderReader::new("resources/vertex_shader.glsl", "resources/fragment_shader.glsl");
    
    // Create a transformation matrix
    let transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_angle_x(Rad(PI/4.0));

    // Set the uniform in the shader
    shader.bind();
    shader.create_uniform("transform");
    shader.set_matrix4fv_uniform("transform", &transform);

    while !window.close() {
        unsafe {
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader.bind();
            gl::DrawElements(gl::TRIANGLES, triangle_count*3, gl::UNSIGNED_INT, ptr::null());
            shader.unbind();
        }
        window.update();
    }
}