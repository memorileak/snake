extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game_objects;

use glutin_window::GlutinWindow;
use graphics::{
  clear,
};
use opengl_graphics::{
  GlGraphics,
  OpenGL,
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
};
use piston::window::WindowSettings;
use game_objects::{
  Config,
  Snake,
  Renderable,
  KeyCode,
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
  let mut snake = Snake::new();

  let mut events = Events::new(EventSettings::new());
  events.set_ups(Config::UPS);

  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      clear(Config::BG_COLOR, &mut gl);
      snake.render(&mut gl, &args);
    }
    if let Some(_) = e.update_args() {
      snake.shift();
    }
    if let Some(args) = e.button_args() {
      match KeyCode::from(args.scancode.unwrap()) {
        KeyCode::W => {snake.turn_up()},
        KeyCode::S => {snake.turn_down()},
        KeyCode::A => {snake.turn_left()},
        KeyCode::D => {snake.turn_right()},
        KeyCode::Unknown => {},
      }
    }
  }
}
