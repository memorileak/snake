use std::cmp::PartialEq;

pub struct Position {
  pub x: u32,
  pub y: u32,
}

impl PartialEq for Position {
  fn eq(&self, other: &Position) -> bool {
    self.x == other.x && self.y == other.y
  }

  fn ne(&self, other: &Position) -> bool {
    !(self.x == other.x && self.y == other.y)
  }
}
