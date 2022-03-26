extern crate rand;

use rand::{
  Rng,
  thread_rng,
};
use super::{
  Position,
  Config,
};

pub struct Random {}

impl Random {
  pub fn random_position() -> Position {
    let mut gen = thread_rng();
    let upperbound_x = Config::WIN_W / Config::SEG_W;
    let upperbound_y = Config::WIN_H / Config::SEG_H;
    let x: u32 = gen.gen_range(0..upperbound_x) * Config::SEG_W;
    let y: u32 = gen.gen_range(0..upperbound_y) * Config::SEG_H;
    Position {x, y}
  }
}
