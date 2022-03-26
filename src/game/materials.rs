extern crate opengl_graphics;

use opengl_graphics::{
  Texture,
  GlyphCache,
};

pub struct Materials<'a> {
  pub texture: Texture,
  pub glyphs: GlyphCache<'a>,
}
