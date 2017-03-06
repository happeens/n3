use piston_window::{rectangle, Context, G2d};

use super::camera::Camera;

pub const TILE_HEIGHT: u16 = 64;
pub const TILE_WIDTH: u16 = 64;

const WORLD_HEIGHT: u32 = 15;
const WORLD_WIDTH: u32 = 15;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub color: [f32; 4],
    pub meta: u8,
}

pub struct World {
    pub fields: Vec<Vec<Tile>>,
}

impl World {
    pub fn new_default() -> World {
        let ground_field = Tile {
            color: [0.0, 0.8, 0.0, 1.0],
            meta: 0,
        };

        let solid_field = Tile {
            color: [0.5, 0.0, 0.0, 1.0],
            meta: 1,
        };

        let mut fields = Vec::new();
        for y in 0..WORLD_HEIGHT {
            let mut row = Vec::new();
            for x in 0..WORLD_WIDTH {
                if x == 0 || y == 0 || x == WORLD_WIDTH - 1 || y == WORLD_HEIGHT - 1 {
                    row.push(solid_field.clone())
                } else {
                    row.push(ground_field.clone())
                }
            }
            fields.push(row);
        }

        World {
            fields: fields,
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d, cam: &Camera) {
        for (y, row) in self.fields.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let field_x = x as f64 * TILE_WIDTH as f64 + cam.pos[0];
                let field_y = y as f64 * TILE_HEIGHT as f64 + cam.pos[1];

                // draw bounds
                rectangle([0.0, 0.0, 0.0, 1.0],
                          [field_x, field_y,
                           TILE_WIDTH as f64, TILE_HEIGHT as f64],
                          c.transform, g);
                rectangle(field.color,
                          [field_x + 1.0, field_y + 1.0,
                           TILE_WIDTH as f64 - 2.0, TILE_HEIGHT as f64 - 2.0],
                          c.transform, g);
            }
        }
    }

    pub fn to_tile_pos(pos: [f64; 2]) -> [u32; 2] {
        let tile_x = (pos[0] / TILE_WIDTH as f64).floor() as u32;
        let tile_y = (pos[1] / TILE_HEIGHT as f64).floor() as u32;

        [tile_x, tile_y]
    }

    pub fn from_tile_pos(tile_pos: [u32; 2]) -> [f64; 2] {
        let pos_x = (tile_pos[0] * TILE_WIDTH as u32) + (TILE_WIDTH / 2) as u32;
        let pos_y = (tile_pos[1] * TILE_HEIGHT as u32) + (TILE_HEIGHT / 2) as u32;

        [pos_x as f64, pos_y as f64]
    }

    pub fn add_tile_offset(pos: [f64; 2], offset: [i32; 2]) -> [f64; 2] {
        [pos[0] + (offset[0] as f64 * TILE_WIDTH as f64),
         pos[1] + (offset[1] as f64 * TILE_HEIGHT as f64)]
    }

    pub fn check_pos_collides(&self, pos: [f64; 2]) -> bool {
        let pos = World::to_tile_pos(pos);
        self.check_tile_collides(pos)
    }

    pub fn check_tile_collides(&self, pos: [u32; 2]) -> bool {
        let pos = [pos[0] as usize, pos[1] as usize];
        if pos[1] >= self.fields.len() {
            println!("invalid pos y for collision check");
            return true;
        } else if pos[0] >= self.fields[pos[1]].len() {
            println!("invalid pos x for collision check");
            return true;
        }

        self.fields[pos[1]][pos[0]].meta == 1
    }
}
