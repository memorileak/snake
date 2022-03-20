extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use std::collections::LinkedList;
use graphics::rectangle;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use super::{
  Renderable, 
  Direction,
  Segment,
  Config,
};

pub struct Snake {
  direction: Direction,
  body: LinkedList<Segment>,
}

impl Renderable for Snake {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
    let mut iter = self.body.iter();
    while let Some(segment) = iter.next() {
      let position = segment.position();
      let size = segment.size();
      let rect = [
        position.x as f64, 
        position.y as f64, 
        size.width as f64,
        size.height as f64,
      ];
      gl.draw(args.viewport(), |context, gl| {
        rectangle(Config::SNAKE_COLOR, rect, context.transform, gl);
      });
    }
  }
}

impl Snake {
  pub fn new() -> Snake {
    let segment = Segment::new(0, 0);
    Snake {
      direction: Direction::Up,
      body: LinkedList::from([
        segment.seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right().seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right().seg_right(),
        segment.seg_right().seg_right(),
        segment.seg_right(),
        segment,
      ]),
    }
  }

  pub fn shift(&mut self) {
    let head = self.body.front().unwrap();
    let new_head = match self.direction {
      Direction::Up => head.seg_above(),
      Direction::Down => head.seg_below(),
      Direction::Left => head.seg_left(),
      Direction::Right => head.seg_right(),
    };
    self.body.push_front(new_head);
    self.body.pop_back();
  }

  pub fn turn_up(&mut self) {
    match self.direction {
      Direction::Down => {},
      _ => {self.direction = Direction::Up},
    }
  }

  pub fn turn_down(&mut self) {
    match self.direction {
      Direction::Up => {},
      _ => {self.direction = Direction::Down},
    }
  }

  pub fn turn_left(&mut self) {
    match self.direction {
      Direction::Right => {},
      _ => {self.direction = Direction::Left},
    }
  }

  pub fn turn_right(&mut self) {
    match self.direction {
      Direction::Left => {},
      _ => {self.direction = Direction::Right},
    }
  }
}
