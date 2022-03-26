extern crate piston;
extern crate opengl_graphics;

use std::collections::VecDeque;
use piston::input::RenderArgs;
use opengl_graphics::{
  GlGraphics,
};
use super::{
  Renderable,
  Snake,
  Prey,
  Score,
  Direction,
  KeyCode,
  EvaluateResult,
  Materials,
};

pub struct Game {
  is_over: bool,
  snake: Snake,
  prey: Prey,
  score: Score,
  directions: VecDeque<Direction>,
}

impl Renderable for Game {
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, materials: &mut Materials) {
    self.score.render(gl, args, materials);
    self.prey.render(gl, args, materials);
    self.snake.render(gl, args, materials);
  }
}

impl Game {
  pub fn new() -> Game {
    let is_over = false;
    let snake = Snake::new();
    let prey = Prey::new_avoid(snake.get_positions().as_slice());
    let score = Score::new();
    let directions = VecDeque::new();
    Game {is_over, snake, prey, score, directions}
  }

  pub fn tick(&mut self) {
    if !self.is_gameover() {
      if let Some(direction) = self.directions.pop_front() {
        match direction {
          Direction::Up => {self.snake.turn_up()},
          Direction::Down => {self.snake.turn_down()},
          Direction::Left => {self.snake.turn_left()},
          Direction::Right => {self.snake.turn_right()},
        }
      }
      self.snake.shift();
      match self.evaluate() {
        EvaluateResult::SnakeDied => {
          self.gameover();
        },
        EvaluateResult::SnakeAte => {
          self.score.increase();
          self.snake.grow();
          self.prey.move_randomly_avoid(self.snake.get_positions().as_slice());
        },
        EvaluateResult::Normal => {},
      }
    }
  }

  pub fn evaluate(&self) -> EvaluateResult {
    if self.is_snake_dead() {
      EvaluateResult::SnakeDied
    } else if self.has_snake_eaten() {
      EvaluateResult::SnakeAte
    } else {
      EvaluateResult::Normal
    }
  }

  fn is_snake_dead(&self) -> bool {
    let head_position = self.snake.get_head_position();
    let body = self.snake.get_positions();
    let mut body_iter = body.iter();
    body_iter.next();
    while let Some(position) = body_iter.next() {
      if head_position == *position {
        return true;
      }
    }
    false
  }

  fn has_snake_eaten(&self) -> bool {
    self.snake.get_head_position() == self.prey.get_position()
  }

  pub fn receive_keystroke(&mut self, keycode: KeyCode) {
    match keycode {
      KeyCode::W => {self.directions.push_back(Direction::Up)},
      KeyCode::S => {self.directions.push_back(Direction::Down)},
      KeyCode::A => {self.directions.push_back(Direction::Left)},
      KeyCode::D => {self.directions.push_back(Direction::Right)},
      KeyCode::Unknown => {},
    }
  }

  fn is_gameover(&self) -> bool {
    self.is_over
  }

  fn gameover(&mut self) {
    self.is_over = true;
  }
}
