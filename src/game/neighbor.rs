pub struct Neighbor {
  above: bool,
  below: bool,
  left: bool,
  right: bool,
}

impl Neighbor {
  pub fn new() -> Neighbor {
    Neighbor {
      above: false,
      below: false,
      left: false,
      right: false,
    }
  }

  pub fn has_above(&self) -> bool {
    self.above
  }

  pub fn has_below(&self) -> bool {
    self.below
  }

  pub fn has_left(&self) -> bool {
    self.left
  }

  pub fn has_right(&self) -> bool {
    self.right
  }

  pub fn aware_above(&mut self) -> &mut Neighbor {
    self.above = true;
    self
  }

  pub fn aware_below(&mut self) -> &mut Neighbor {
    self.below = true;
    self
  }

  pub fn aware_left(&mut self) -> &mut Neighbor {
    self.left = true;
    self
  }

  pub fn aware_right(&mut self) -> &mut Neighbor {
    self.right = true;
    self
  }

  pub fn nomore_above(&mut self) -> &mut Neighbor {
    self.above = false;
    self
  }

  pub fn nomore_below(&mut self) -> &mut Neighbor {
    self.below = false;
    self
  }

  pub fn nomore_left(&mut self) -> &mut Neighbor {
    self.left = false;
    self
  }

  pub fn nomore_right(&mut self) -> &mut Neighbor {
    self.right = false;
    self
  }
}
