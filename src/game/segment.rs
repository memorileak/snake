use super::{
  Config, 
  Neighbor,
  Sprite,
  SpriteCalculator,
  Position,
  Size,
};

pub struct Segment {
  x: u32,
  y: u32,
  width: u32,
  height: u32,
  head: bool,
  tail: bool,
  pub neighbor: Neighbor,
}

impl Segment {
  pub fn new(x: u32, y: u32) -> Segment {
    Segment {
      x: x,
      y: y,
      width: Config::SEG_W,
      height: Config::SEG_H,
      head: false,
      tail: false,
      neighbor: Neighbor::new(),
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

  pub fn is_head(&self) -> bool {
    self.head
  }

  pub fn is_tail(&self) -> bool {
    self.tail
  }

  pub fn be_head(&mut self) {
    self.head = true;
    self.tail = false;
  }

  pub fn be_tail(&mut self) {
    self.head = false;
    self.tail = true;
  }

  pub fn nomore_head(&mut self) {
    self.head = false;
  }

  pub fn get_sprite(&self) -> Sprite {
    SpriteCalculator::calculate(&self.neighbor, self.is_head(), self.is_tail())
  }

  pub fn seg_above(&self) -> Segment {
    Segment {
      x: self.x,
      y: (self.y + Config::WIN_H - self.height) % Config::WIN_H,
      width: self.width,
      height: self.height,
      head: false,
      tail: false,
      neighbor: Neighbor::new(),
    }
  }

  pub fn seg_below(&self) -> Segment {
    Segment {
      x: self.x,
      y: (self.y + self.height) % Config::WIN_H,
      width: self.width,
      height: self.height,
      head: false,
      tail: false,
      neighbor: Neighbor::new(),
    }
  }

  pub fn seg_left(&self) -> Segment {
    Segment {
      x: (self.x + Config::WIN_W - self.width) % Config::WIN_W,
      y: self.y,
      width: self.width,
      height: self.height,
      head: false,
      tail: false,
      neighbor: Neighbor::new(),
    }
  }

  pub fn seg_right(&self) -> Segment {
    Segment {
      x: (self.x + self.width) % Config::WIN_W,
      y: self.y,
      width: self.width,
      height: self.height,
      head: false,
      tail: false,
      neighbor: Neighbor::new(),
    }
  }
}
