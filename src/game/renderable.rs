extern crate piston;
extern crate opengl_graphics;

use piston::input::RenderArgs;
use opengl_graphics::{
  GlGraphics,
  Texture,
};

pub trait Renderable {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, texture: &Texture);
}
