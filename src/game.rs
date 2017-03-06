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
        let player_pos = self.player.get_pos();
        let target = [player_pos[0] + v[0], player_pos[1] + v[1]];
        let mut vel = v;

        if self.world.check_pos_collides([target[0], player_pos[1]]) {
            vel[0] = 0.0;
            vel[1] *= 0.707106781;
        } 
        if self.world.check_pos_collides([player_pos[0], target[1]]) {
            vel[1] = 0.0;
            vel[0] *= 0.707106781;
        }

        self.player.set_vel(vel);
    }
}
