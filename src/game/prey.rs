extern crate piston;
extern crate opengl_graphics;
extern crate graphics;

use piston::input::RenderArgs;
use opengl_graphics::{
  GlGraphics,
  Texture,
};
use graphics::{
  Transformed,
  DrawState,
  Image,
};
use super::{
  Renderable,
  Sprite,
  Config,
  Random,
  Position,
};

pub struct Prey {
  x: u32,
  y: u32,
  width: u32,
  height: u32,
}

impl Renderable for Prey {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, texture: &Texture) {
    let image = Image::new()
      .rect([0.0, 0.0, self.width as f64, self.height as f64])
      .src_rect(self.get_sprite().slot());
    gl.draw(args.viewport(), |context, gl| {
      image.draw(texture, &DrawState::default(), context.transform.trans(self.x as f64, self.y as f64), gl);
    });
  }
}

impl Prey {
  pub fn new() -> Prey {
    let position = Random::random_position();
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

  pub fn move_randomly(&mut self) {
    let position = Random::random_position();
    self.x = position.x;
    self.y = position.y;
  }
}
