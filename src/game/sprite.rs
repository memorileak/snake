use super::Config;

pub enum Sprite {
  SnakeHeadUp,
  SnakeHeadDown,
  SnakeHeadLeft,
  SnakeHeadRight,

  SnakeTailUp,
  SnakeTailDown,
  SnakeTailLeft,
  SnakeTailRight,

  SnakeBendTopLeft,
  SnakeBendTopRight,
  SnakeBendBottomLeft,
  SnakeBendBottomRight,

  SnakeVertical,
  SnakeHorizontal,

  Prey,
}

impl Sprite {
  pub fn slot(&self) -> [f64; 4] {
    let seg_w = Config::SEG_W as f64;
    let seg_h = Config::SEG_H as f64;
    match self {
      Self::SnakeHeadUp => [3.0 * seg_w, 0.0 * seg_h, seg_w, seg_h],
      Self::SnakeHeadDown => [4.0 * seg_w, 1.0 * seg_h, seg_w, seg_h],
      Self::SnakeHeadLeft => [3.0 * seg_w, 1.0 * seg_h, seg_w, seg_h],
      Self::SnakeHeadRight => [4.0 * seg_w, 0.0 * seg_h, seg_w, seg_h],

      Self::SnakeTailUp => [3.0 * seg_w, 2.0 * seg_h, seg_w, seg_h],
      Self::SnakeTailDown => [4.0 * seg_w, 3.0 * seg_h, seg_w, seg_h],
      Self::SnakeTailLeft => [3.0 * seg_w, 3.0 * seg_h, seg_w, seg_h],
      Self::SnakeTailRight => [4.0 * seg_w, 2.0 * seg_h, seg_w, seg_h],

      Self::SnakeBendTopLeft => [0.0 * seg_w, 0.0 * seg_h, seg_w, seg_h],
      Self::SnakeBendTopRight => [2.0 * seg_w, 0.0 * seg_h, seg_w, seg_h],
      Self::SnakeBendBottomLeft => [0.0 * seg_w, 1.0 * seg_h, seg_w, seg_h],
      Self::SnakeBendBottomRight => [2.0 * seg_w, 2.0 * seg_h, seg_w, seg_h],

      Self::SnakeVertical => [2.0 * seg_w, 1.0 * seg_h, seg_w, seg_h],
      Self::SnakeHorizontal => [1.0 * seg_w, 0.0 * seg_h, seg_w, seg_h],

      Self::Prey => [0.0 * seg_w, 3.0 * seg_h, seg_w, seg_h],
    }
  }
}
