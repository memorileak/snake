use std::convert::From;

pub enum KeyCode {
  Unknown,
  W,
  A,
  S,
  D,
}

impl From<i32> for KeyCode {
  fn from(value: i32) -> Self {
    if value == 17 {
      Self::W
    } else if value == 30 {
      Self::A
    } else if value == 31 {
      Self::S
    } else if value == 32 {
      Self::D
    } else {
      Self::Unknown
    }
  }
}
