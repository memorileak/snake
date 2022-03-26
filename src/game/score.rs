pub struct Score {
  score: u32,
}

impl Score {
  pub fn new() -> Score {
    Score {
      score: 0,
    }
  }

  pub fn increase(&mut self) {
    self.score += 1;
  }

  pub fn get_score(&self) -> u32 {
    self.score
  }
}
