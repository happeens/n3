use piston_window::{Context, G2d};

use super::world::World;
use super::player::Player;
use super::camera::Camera;

pub struct Game {
    world: World,
    pub player: Player,
    camera: Camera,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new_default(),
            player: Player::new(World::from_tile_pos([3, 3])),
            camera: Camera::new([0.0, 0.0]),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.camera.set_target(self.player.get_pos());
        self.camera.update(dt);
        self.player.update(dt);
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d) {
        self.world.draw(&c, g, &self.camera);
        self.player.draw(&c, g, &self.camera);
    }

    pub fn try_move_player(&mut self, v: [f64; 2]) {
        let mut target = self.player.get_pos();
        target[0] += v[0];
        target[1] += v[1];

        if !self.world.check_pos_collides(target) {
            self.player.set_vel(v);
        } else {
            self.player.set_vel([0.0, 0.0]);
        }
    }
}
