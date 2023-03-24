extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    println!("Hello, world!");
}
