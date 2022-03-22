use super::{
  Sprite,
  Neighbor,
};

pub struct SpriteCalculator {}

impl SpriteCalculator {
  pub fn calculate(neighbor: &Neighbor, is_head: bool, is_tail: bool) -> Sprite {
    if is_head {
      SpriteCalculator::get_head_sprite(neighbor)
    } else if is_tail {
      SpriteCalculator::get_tail_sprite(neighbor)
    } else {
      SpriteCalculator::get_mid_sprite(neighbor)
    }
  }

  fn get_head_sprite(neighbor: &Neighbor) -> Sprite {
    if neighbor.has_above() {
      Sprite::SnakeHeadDown
    } else if neighbor.has_below() {
      Sprite::SnakeHeadUp
    } else if neighbor.has_left() {
      Sprite::SnakeHeadRight
    } else {
      Sprite::SnakeHeadLeft
    }
  }

  fn get_tail_sprite(neighbor: &Neighbor) -> Sprite {
    if neighbor.has_above() {
      Sprite::SnakeTailUp
    } else if neighbor.has_below() {
      Sprite::SnakeTailDown
    } else if neighbor.has_left() {
      Sprite::SnakeTailLeft
    } else {
      Sprite::SnakeTailRight
    }
  }

  fn get_mid_sprite(neighbor: &Neighbor) -> Sprite {
    if neighbor.has_above() && neighbor.has_left() {
      Sprite::SnakeBendBottomRight
    } else if neighbor.has_above() && neighbor.has_below() {
      Sprite::SnakeVertical
    } else if neighbor.has_above() && neighbor.has_right() {
      Sprite::SnakeBendBottomLeft
    } else if neighbor.has_left() && neighbor.has_below() {
      Sprite::SnakeBendTopRight
    } else if neighbor.has_left() && neighbor.has_right() {
      Sprite::SnakeHorizontal
    } else {
      Sprite::SnakeBendTopLeft
    }
  }
}
