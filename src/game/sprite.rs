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
    match self {
      Self::SnakeHeadUp => [3.0 * Config::SEG_W as f64, 0.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeHeadDown => [4.0 * Config::SEG_W as f64, 1.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeHeadLeft => [3.0 * Config::SEG_W as f64, 1.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeHeadRight => [4.0 * Config::SEG_W as f64, 0.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],

      Self::SnakeTailUp => [3.0 * Config::SEG_W as f64, 2.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeTailDown => [4.0 * Config::SEG_W as f64, 3.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeTailLeft => [3.0 * Config::SEG_W as f64, 3.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeTailRight => [4.0 * Config::SEG_W as f64, 2.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],

      Self::SnakeBendTopLeft => [0.0 * Config::SEG_W as f64, 0.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeBendTopRight => [2.0 * Config::SEG_W as f64, 0.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeBendBottomLeft => [0.0 * Config::SEG_W as f64, 1.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeBendBottomRight => [2.0 * Config::SEG_W as f64, 2.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],

      Self::SnakeVertical => [2.0 * Config::SEG_W as f64, 1.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
      Self::SnakeHorizontal => [1.0 * Config::SEG_W as f64, 0.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],

      Self::Prey => [0.0 * Config::SEG_W as f64, 3.0 * Config::SEG_H as f64, Config::SEG_W as f64, Config::SEG_H as f64],
    }
  }
}
