pub struct Camera {
    pub pos: [f64; 2],
}

impl Camera {
    pub fn new(pos: [f64; 2]) -> Camera {
        Camera {
            pos: pos,
        }
    }

    pub fn follow(&mut self, pos: [f64; 2]) {
        self.pos = pos;
    }
}
