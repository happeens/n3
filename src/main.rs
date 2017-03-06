extern crate piston_window;

mod world;
use world::World;

mod player;
use player::Player;

mod camera;
use camera::Camera;

use piston_window::*;

const CAMERA_SPEED: f64 = 0.3;

pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("test", [1000, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = World::new_default();
    let mut player = Player::new(World::from_tile_pos([3, 3]));
    let mut camera = Camera::new([0.0, 0.0]);

    let mut held_keys = Vec::new();


    while let Some(e) = window.next() {
        // input
        if let Some(pressed) = e.press_args() {
            match pressed {
                Button::Keyboard(key) => {
                    held_keys.push(key.clone());
                }
                _ => {
                    println!("pressed: {:?}", pressed);
                }
            }
        }

        if let Some(released) = e.release_args() {
            match released {
                Button::Keyboard(key) => {
                    held_keys.retain(|&k| k != key);
                }
                _ => {
                    println!("released: {:?}", released);
                }
            }
        }

        if let Some(key) = held_keys.last() {
            match key {
                &Key::W => {
                    let target = World::add_tile_offset(player.get_pos(), [0, -1]);
                    player.try_move(target);
                }
                &Key::S => {
                    let target = World::add_tile_offset(player.get_pos(), [0, 1]);
                    player.try_move(target);
                }
                &Key::A => {
                    let target = World::add_tile_offset(player.get_pos(), [-1, 0]);
                    player.try_move(target);
                }
                &Key::D => {
                    let target = World::add_tile_offset(player.get_pos(), [1, 0]);
                    player.try_move(target);
                }
                _ => ()
            }
        }

        // camera.follow(player.get_pos());
        player.update();

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            world.draw(&c, g, &camera);
            player.draw(&c, g, &camera);
        });
    }
}
