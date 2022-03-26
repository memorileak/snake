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
  pub fn random_position_avoid(positions: &[Position]) -> Position {
    let mut gen = thread_rng();
    let upperbound_x = Config::WIN_W / Config::SEG_W;
    let upperbound_y = Config::WIN_H / Config::SEG_H;

    let mut accepted = false;
    let mut position = Position {x: 0, y: 0};

    while !accepted {
      let x: u32 = gen.gen_range(0..upperbound_x) * Config::SEG_W;
      let y: u32 = gen.gen_range(0..upperbound_y) * Config::SEG_H;
      position = Position {x, y};
      let mut should_accept = true;
      for avoid_position_item in positions.iter().enumerate() {
        let (_, avoid_position) = avoid_position_item;
        if position == *avoid_position {
          should_accept = false;
          break;
        }
      }
      accepted = should_accept;
    }

    position
  }
}
