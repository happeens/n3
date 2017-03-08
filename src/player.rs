use piston_window::{rectangle, Context, G2d};

use super::camera::Camera;

const PLAYER_WIDTH: f64 = 36.0;
const PLAYER_HEIGHT: f64 = 50.0;

const PLAYER_SPEED: f64 = 200.0;

pub struct Player {
    pos: [f64; 2],
    color: [f32; 4],
    vel: [f64; 2],
}

impl Player {
    pub fn new(pos: [f64; 2]) -> Player {
        Player {
            pos: pos,
            color: [1.0, 1.0, 1.0, 0.7],
            vel: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.pos[0] += self.vel[0] * PLAYER_SPEED * dt;
        self.pos[1] += self.vel[1] * PLAYER_SPEED * dt;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d, cam: &Camera) {
        rectangle(self.color,
                  [self.pos[0] - cam.pos[0] - (0.5 * PLAYER_WIDTH),
                   self.pos[1] - cam.pos[1] - (0.5 * PLAYER_HEIGHT),
                   PLAYER_WIDTH, PLAYER_HEIGHT],
                  c.transform, g);
    }

    pub fn get_pos(&self) -> [f64; 2] {
        self.pos
    }

    pub fn set_vel(&mut self, vel: [f64; 2]) {
        self.vel = vel;
    }
}
