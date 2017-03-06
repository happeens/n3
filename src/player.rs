use piston_window::{rectangle, Context, G2d};

use super::camera::Camera;

const PLAYER_WIDTH: u32 = 36;
const PLAYER_HEIGHT: u32 = 50;

const PLAYER_SPEED: f64 = 20.0;

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
        self.pos[0] += self.vel[0] * PLAYER_SPEED * dt * 10.0;
        self.pos[1] += self.vel[1] * PLAYER_SPEED * dt * 10.0;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d, cam: &Camera) {
        rectangle(self.color,
                  [self.pos[0] + cam.pos[0] - 0.5 * PLAYER_WIDTH as f64,
                   self.pos[1] + cam.pos[1] - 0.5 * PLAYER_HEIGHT as f64,
                   PLAYER_WIDTH as f64, PLAYER_HEIGHT as f64],
                  c.transform, g);
    }

    pub fn get_pos(&self) -> [f64; 2] {
        self.pos
    }

    pub fn set_vel(&mut self, vel: [f64; 2]) {
        self.vel = vel;
    }
}
