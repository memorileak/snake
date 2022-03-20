use super::Config;

pub struct Segment {
  x: u32,
  y: u32,
  width: u32,
  height: u32,
}

pub struct Position {
  pub x: u32,
  pub y: u32,
}

pub struct Size {
  pub width: u32,
  pub height: u32,
}

impl Segment {
  pub fn new(x: u32, y: u32) -> Segment {
    Segment {
      x: x,
      y: y,
      width: Config::SEG_W,
      height: Config::SEG_H,
    }
  }

  pub fn position(&self) -> Position {
    Position {
      x: self.x,
      y: self.y,
    }
  }

  pub fn size(&self) -> Size {
    Size {
      width: self.width,
      height: self.height,
    }
  }

  pub fn seg_above(&self) -> Segment {
    Segment {
      x: self.x,
      y: (self.y + Config::WIN_H - Config::STEP) % Config::WIN_H,
      width: self.width,
      height: self.height,
    }
  }

  pub fn seg_below(&self) -> Segment {
    Segment {
      x: self.x,
      y: (self.y + Config::STEP) % Config::WIN_H,
      width: self.width,
      height: self.height,
    }
  }

  pub fn seg_left(&self) -> Segment {
    Segment {
      x: (self.x + Config::WIN_W - Config::STEP) % Config::WIN_W,
      y: self.y,
      width: self.width,
      height: self.height,
    }
  }

  pub fn seg_right(&self) -> Segment {
    Segment {
      x: (self.x + Config::STEP) % Config::WIN_W,
      y: self.y,
      width: self.width,
      height: self.height,
    }
  }
}
