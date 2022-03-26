extern crate piston;
extern crate opengl_graphics;
extern crate graphics;

use piston::input::RenderArgs;
use opengl_graphics::{
  GlGraphics,
};
use graphics::{
  Transformed,
  Image,
};
use super::{
  Renderable,
  Sprite,
  Config,
  Random,
  Position,
  Materials,
};

pub struct Prey {
  x: u32,
  y: u32,
  width: u32,
  height: u32,
}

impl Renderable for Prey {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, materials: &mut Materials) {
    let image = Image::new()
      .rect([0.0, 0.0, self.width as f64, self.height as f64])
      .src_rect(self.get_sprite().slot());
    gl.draw(args.viewport(), |context, gl| {
      image.draw(&materials.texture, &context.draw_state, context.transform.trans(self.x as f64, self.y as f64), gl);
    });
  }
}

impl Prey {
  pub fn new_avoid(positions: &[Position]) -> Prey {
    let position = Random::random_position_avoid(positions);
    Prey {
      x: position.x,
      y: position.y,
      width: Config::SEG_W,
      height: Config::SEG_H,
    }
  }

  pub fn get_sprite(&self) -> Sprite {
    return Sprite::Prey;
  }

  pub fn get_position(&self) -> Position {
    Position {
      x: self.x,
      y: self.y,
    }
  }

  pub fn move_randomly_avoid(&mut self, positions: &[Position]) {
    let position = Random::random_position_avoid(positions);
    self.x = position.x;
    self.y = position.y;
  }
}
