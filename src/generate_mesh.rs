use crate::perlin_noise::PerlinMap;

pub fn generate_mesh(scale: f32, map_h: i32, map_w: i32, pos_x: f32, pos_y: f32, p_map: &PerlinMap) -> (Vec<f32>, Vec<i32>, i32) {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<i32> = Vec::new();
    let mut triangle_count: i32 = 0;
    
    // Populate map
    for i in 0 .. map_h * map_w {
        let x = (i % map_w) as f32; // Column
        let y = (i / map_w) as f32; // Row
        let z = p_map.noise((x * scale) + pos_x, (y * scale) + pos_y);
        
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
    
    (vertices, indices, triangle_count)
}