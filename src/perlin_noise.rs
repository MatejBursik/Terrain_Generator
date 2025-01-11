use rand::{Rng, thread_rng};

pub struct PerlinMap {
    vec_map: Vec<i32>,
}

impl PerlinMap {
    pub fn new() -> Self {
        PerlinMap {
            vec_map: vec![]
        }
    }

    pub fn generate_vec_map(&mut self, width: i32, height: i32) {
        let mut rng = thread_rng();
        let mut map: Vec<i32> = Vec::new();
        for _ in 0 .. width * height {
            map.push(rng.gen_range(1 .. 360 + 1));
        }

        self.vec_map = map;
    }

    pub fn rotate_vec_map(&mut self, angle: i32) {
        for v in self.vec_map.iter_mut() {
            *v += angle;
        }
    }

    pub fn noise(&self, x: f32, y: f32) -> f32 {
        x + y
    }

    pub fn debug(&self) {
        println!("{:?}", self.vec_map)
    }
}