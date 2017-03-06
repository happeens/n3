const CAMERA_SPEED: f64 = 0.03;

pub struct Camera {
    pub pos: [f64; 2],
    pub target: [f64; 2],
}

impl Camera {
    pub fn new(pos: [f64; 2]) -> Camera {
        Camera {
            pos: pos,
            target: pos,
        }
    }

    pub fn set_target(&mut self, target: [f64; 2]) {
        self.target = target;
    }

    pub fn update(&mut self, dt: f64) {
        let target_pos = [self.target[0] - 0.5 * super::SCREEN_WIDTH as f64,
                          self.target[1] - 0.5 * super::SCREEN_HEIGHT as f64];
        let distance = [target_pos[0] - self.pos[0], target_pos[1] - self.pos[1]];
        self.pos[0] += distance[0] * CAMERA_SPEED;
        self.pos[1] += distance[1] * CAMERA_SPEED;
    }
}
