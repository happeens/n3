use piston_window::{rectangle, Context, G2d};

use super::camera::Camera;

const PLAYER_WIDTH: u32 = 36;
const PLAYER_HEIGHT: u32 = 50;

const PLAYER_SPEED: f64 = 0.5;

pub struct Player {
    pos: [f64; 2],
    color: [f32; 4],
    state: MoveState,
}

#[derive(PartialEq)]
pub enum MoveState {
    Idle,
    MovingTo(f64, f64),
}

impl Player {
    pub fn new(pos: [f64; 2]) -> Player {
        Player {
            pos: pos,
            color: [1.0, 1.0, 1.0, 0.7],
            state: MoveState::Idle,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            MoveState::MovingTo(x, y) => {
                if self.pos[1].floor() > y {
                    self.pos[1] = y.max(self.pos[1] - PLAYER_SPEED);

                } else if self.pos[1].ceil() < y {
                    self.pos[1] = y.min(self.pos[1] + PLAYER_SPEED);

                } else if self.pos[0].floor() > x {
                    self.pos[0] = x.max(self.pos[0] - PLAYER_SPEED);

                } else if self.pos[0].ceil() < x {
                    self.pos[0] = x.min(self.pos[0] + PLAYER_SPEED);

                } else {
                    self.state = MoveState::Idle;
                }
            },
            MoveState::Idle => ()
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d, cam: &Camera) {
        rectangle(self.color,
                  [self.pos[0] + cam.pos[0] - PLAYER_WIDTH as f64 / 2.0,
                   self.pos[1] + cam.pos[1] - PLAYER_HEIGHT as f64 / 2.0,
                   PLAYER_WIDTH as f64, PLAYER_HEIGHT as f64],
                  c.transform, g);
    }

    pub fn get_pos(&self) -> [f64; 2] {
        self.pos
    }

    pub fn try_move(&mut self, target: [f64; 2]) {
        if self.state != MoveState::Idle {
            return
        }

        self.state = MoveState::MovingTo(target[0], target[1]);
    }
}
