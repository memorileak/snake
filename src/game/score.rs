extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use graphics::{
  Transformed,
  Text,
  CharacterCache,
};
use graphics::types::FontSize;
use opengl_graphics::{
  GlGraphics,
};
use piston::input::RenderArgs;
use super::{
  Renderable, 
  Config,
  Materials,
};

pub struct Score {
  score: u32,
}

impl Renderable for Score {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, materials: &mut Materials) {
    let score_str = &format!("{}", self.get_score());
    let padding: f64 = 20.0;
    let fontsize: FontSize = 24;
    let text = Text::new_color(Config::SCORE_COLOR, fontsize);
    let text_width = materials.glyphs.width(fontsize, score_str).unwrap_or(0.0);

    gl.draw(args.viewport(), |context, gl| {
      text.draw(
        score_str,
        &mut materials.glyphs, 
        &context.draw_state, 
        context.transform.trans(Config::WIN_W as f64 - text_width - padding, fontsize as f64 + padding), 
        gl
      ).unwrap();
    });
  }
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
