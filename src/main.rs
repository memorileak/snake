extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;

use std::path::Path;
use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{
  GlGraphics,
  OpenGL,
  Texture,
  GlyphCache,
  TextureSettings,
};
use piston::event_loop::{
  EventSettings,
  Events,
  EventLoop,
};
use piston::input::{
  RenderEvent,
  UpdateEvent,
  ButtonEvent,
  ButtonState,
};
use piston::window::WindowSettings;
use game::{
  Renderable,
  Config,
  Game,
  KeyCode,
  Materials,
};

fn main() {
  let opengl = OpenGL::V4_5;

  let mut window: GlutinWindow = WindowSettings::new("Snake", [Config::WIN_W, Config::WIN_H])
    .graphics_api(opengl)
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut gl = GlGraphics::new(opengl);
  let mut materials = Materials {
    texture: Texture::from_path(Path::new("assets/sprites/pink-snake.png"), &TextureSettings::new()).unwrap(),
    glyphs: GlyphCache::new("assets/fonts/cute-aurora.ttf", (), TextureSettings::new()).unwrap(),
  };
  let mut game = Game::new();

  let mut events = Events::new(EventSettings::new());
  events.set_ups(Config::UPS);

  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.button_args() {
      if let ButtonState::Press = args.state {
        game.receive_keystroke(KeyCode::from(args.scancode.unwrap_or(0)));
      }
    }
    if let Some(_) = e.update_args() {
      game.tick();
    }
    if let Some(args) = e.render_args() {
      clear(Config::BG_COLOR, &mut gl);
      game.render(&mut gl, &args, &mut materials);
    }
  }
}
