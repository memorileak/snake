pub struct Config {}

impl Config {
  pub const WIN_W: u32 = 720;
  pub const WIN_H: u32 = 480;
  pub const UPS: u64 = 30;
  pub const SEG_W: u32 = 10;
  pub const SEG_H: u32 = 10;
  pub const STEP: u32 = 10;
  pub const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
  pub const SNAKE_COLOR: [f32; 4] = [0.433, 0.304, 0.950, 1.0];
}
