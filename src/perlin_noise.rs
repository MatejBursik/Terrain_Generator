use rand::{Rng, thread_rng};

#[derive(Debug)]
pub struct PerlinMap {
    vec_map: Vec<i32>,
    width: i32,
    height: i32
}

impl PerlinMap {
    pub fn new() -> Self {
        PerlinMap {
            vec_map: vec![60, 45, 150, 240],
            width: 2,
            height: 2
        }
    }

    pub fn generate_vec_map(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        let mut rng = thread_rng();
        let mut map: Vec<i32> = Vec::new();
        for _ in 0 .. width * height {
            map.push(rng.gen_range(0 .. 360));
        }

        self.vec_map = map;
    }

    pub fn rotate_vec_map(&mut self, angle: i32) {
        for v in self.vec_map.iter_mut() {
            *v = (*v + angle) % 360;
        }
    }

    fn gradient_angle(&self, x: i32, y: i32) -> i32 {
        let index = y * self.width + x;
        self.vec_map[index as usize]
    }

    fn dot_product(&self, x: f32, y: f32, ix: i32, iy: i32) -> f32 {
        // Get the gradient angle at grid point
        let theta = self.gradient_angle(ix, iy);

        // Convert angle to radians and calculate sin and cos
        let theta_rad = (theta as f32).to_radians();
        let gx = theta_rad.cos();
        let gy = theta_rad.sin();

        // Calculate the vector from grid point to input point
        let dx = ix as f32 - x;
        let dy = iy as f32 - y;

        // Dot product
        (gx * dx) + (gy * dy)
    }

    fn fade(&self, t: f32) -> f32 {
        // 6t^5 - 15t^4 + 10t^3
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(&self, a: f32, b: f32, t: f32) -> f32 {
        // Linear interpolation
        (1.0 - t) * a + t * b
    }

    pub fn noise(&self, x: f32, y: f32) -> f32 {
        // Identify the four cells of the vec_map enclosing the input point
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;
        
        // Calculate dot products for each vertex
        let n00 = self.dot_product(x, y, x0, y0);
        let n10 = self.dot_product(x, y, x1, y0);
        let n01 = self.dot_product(x, y, x0, y1);
        let n11 = self.dot_product(x, y, x1, y1);

        // Linearly interpolate the four dot products of the vertices
        // Relative positions within the cell
        let sx = x - x0 as f32;
        let sy = y - y0 as f32;

        // Fade curves for x and y
        // sx/sy: no fade, u/v: fade
        let u = self.fade(sx);
        let v = self.fade(sy);
        
        // Interpolate along x for top and bottom edges
        let nx0 = self.lerp(n00, n10, sx);
        let nx1 = self.lerp(n01, n11, sx);

        /* DEBUG
        println!("0,0: {}", n00);
        println!("1,0: {}", n10);
        println!("0,1: {}", n01);
        println!("1,1: {}", n11);
        println!("sx: {}", sx);
        println!("lerp x0: {}", nx0);
        println!("lerp x1: {}", nx1);
        println!("sy: {}", sy);
        */

        // Interpolate along y for final value
        self.lerp(nx0, nx1, sy)
    }

    pub fn is_valid_coord(&self, scale: f32, plain_h: i32, plain_w: i32, x: f32, y: f32) -> bool {
        let end_x = x + (plain_w as f32 * scale);
        let end_y = y + (plain_h as f32 * scale);
        
        if end_x > self.width as f32 ||
            end_y > self.height as f32 ||
            x < 0.0 ||
            y < 0.0 {
            return false;
        }
        
        return true;
    }
}