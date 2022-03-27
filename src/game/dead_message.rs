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

pub struct DeadMessage {
  message: String,
}

impl Renderable for DeadMessage {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, materials: &mut Materials) {
    let message = &self.message;
    let fontsize: FontSize = 36;
    let text = Text::new_color(Config::TEXT_COLOR, fontsize);
    let text_width = materials.glyphs.width(fontsize, message).unwrap_or(0.0);

    gl.draw(args.viewport(), |context, gl| {
      text.draw(
        message,
        &mut materials.glyphs, 
        &context.draw_state, 
        context.transform.trans(
          (Config::WIN_W as f64 / 2.0) - (text_width / 2.0), 
          (Config::WIN_H as f64 / 2.0) + 0.0,
        ), 
        gl
      ).unwrap();
    });
  }
}

impl DeadMessage {
  pub fn new() -> DeadMessage {
    DeadMessage {
      message: String::from("You noob!"),
    }
  }

  pub fn aware_score(&mut self, score: u32) {
    self.message = if score < 20 {
      String::from("You noob!")
    } else if score < 50 {
      String::from("Nice try!")
    } else if score < 100 {
      String::from("Very good!")
    } else {
      String::from("What a Pro!")
    }
  }
}
