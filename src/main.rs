extern crate piston_window;

mod game;
use game::Game;

mod world;
mod player;
mod camera;

use piston_window::*;

pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("test", [1000, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    let mut held_keys = Vec::new();

    while let Some(e) = window.next() {
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

        if let Some(update) = e.update_args() {
            let mut move_intention = [0.0, 0.0];
            for key in held_keys.iter() {
                match key {
                    &Key::W => {
                        move_intention[1] -= 1.0;
                    }
                    &Key::S => {
                        move_intention[1] += 1.0;
                    }
                    &Key::A => {
                        move_intention[0] -= 1.0;
                    }
                    &Key::D => {
                        move_intention[0] += 1.0;
                    }
                    _ => ()
                }
            }

            if move_intention[0] != 0.0 && move_intention[1] != 0.0 {
                let diag_mult = 0.707106781;
                move_intention[0] *= diag_mult;
                move_intention[1] *= diag_mult;
            }

            game.try_move_player(move_intention);
            game.update(update.dt);
        }

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                game.draw(&c, g);
            });
        }

    }
}
