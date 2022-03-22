extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use std::collections::LinkedList;
use graphics::{
  DrawState,
  Transformed,
  Image,
};
use opengl_graphics::{
  GlGraphics,
  Texture,
};
use piston::input::RenderArgs;
use super::{
  Renderable, 
  Direction,
  Segment,
};

pub struct Snake {
  direction: Direction,
  body: LinkedList<Segment>,
}

impl Renderable for Snake {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, texture: &Texture) {
    let mut iter = self.body.iter();
    while let Some(segment) = iter.next() {
      let position = segment.position();
      let size = segment.size();
      let image = Image::new()
        .rect([0.0, 0.0, size.width as f64, size.height as f64])
        .src_rect(segment.get_sprite().slot());
      gl.draw(args.viewport(), |context, gl| {
				image.draw(texture, &DrawState::default(), context.transform.trans(position.x as f64, position.y as f64), gl);
      });
    }
  }
}

impl Snake {
  pub fn new() -> Snake {
    let mut segment6 = Segment::new(0, 0);   segment6.be_tail(); segment6.neighbor.aware_right();
    let mut segment5 = segment6.seg_right(); segment5.neighbor.aware_left().aware_right();
    let mut segment4 = segment5.seg_right(); segment4.neighbor.aware_left().aware_right();
    let mut segment3 = segment4.seg_right(); segment3.neighbor.aware_left().aware_right();
    let mut segment2 = segment3.seg_right(); segment2.neighbor.aware_left().aware_right();
    let mut segment1 = segment2.seg_right(); segment1.be_head(); segment1.neighbor.aware_left();
    Snake {
      direction: Direction::Up,
      body: LinkedList::from([segment1, segment2, segment3, segment4, segment5, segment6]),
    }
  }

  pub fn shift(&mut self) {
    let old_head = self.body.front_mut().unwrap();

    match self.direction {
      Direction::Up => {
        let mut new_head = old_head.seg_above();
        old_head.nomore_head();
        old_head.neighbor.aware_above();
        new_head.be_head();
        new_head.neighbor.aware_below();
        self.body.push_front(new_head);
      },
      Direction::Down => {
        let mut new_head = old_head.seg_below();
        old_head.nomore_head();
        old_head.neighbor.aware_below();
        new_head.be_head();
        new_head.neighbor.aware_above();
        self.body.push_front(new_head);
      },
      Direction::Left => {
        let mut new_head = old_head.seg_left();
        old_head.nomore_head();
        old_head.neighbor.aware_left();
        new_head.be_head();
        new_head.neighbor.aware_right();
        self.body.push_front(new_head);
      },
      Direction::Right => {
        let mut new_head = old_head.seg_right();
        old_head.nomore_head();
        old_head.neighbor.aware_right();
        new_head.be_head();
        new_head.neighbor.aware_left();
        self.body.push_front(new_head);
      },
    };

    let old_tail = self.body.pop_back().unwrap();
    let new_tail = self.body.back_mut().unwrap();

    new_tail.be_tail();

    if old_tail.neighbor.has_above() {
      new_tail.neighbor.nomore_below();
    } else if old_tail.neighbor.has_below() {
      new_tail.neighbor.nomore_above();
    } else if old_tail.neighbor.has_left() {
      new_tail.neighbor.nomore_right();
    } else {
      new_tail.neighbor.nomore_left();
    }
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
