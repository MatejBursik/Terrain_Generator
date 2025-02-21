use std::ptr;
use gl::types::*;

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

    let mut window = window::Window::new(1280, 720, "Terrain Generator");
    window.init_gl();

    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0
    ];
    let indices: [i32; 6]= [0, 1, 3, 1, 2, 3];

    let vao = vao::ArrayObject::new();
    vao.bind();

    let vbo = vbo::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let ibo = vbo::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();
    ibo.store_i32_data(&indices);

    let position_attribute = v_attribute::VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    let index_attribute = v_attribute::VertexAttribute::new(1, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    position_attribute.enable();
    index_attribute.enable();

    while !window.close() {
        unsafe {
            gl::ClearColor(0.25, 0.25, 0.25, 1.0); // Gray background color
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}