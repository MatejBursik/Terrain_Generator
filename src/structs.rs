pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
    pub speed: f32,
    pub has_moved: bool
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 0.0,
            y: 0.0,
            direction: 0.0,
            speed: 0.1,
            has_moved: false
        }
    }
}
