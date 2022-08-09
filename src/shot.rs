use std::time::Duration;
use rusty_time::timer::Timer;
use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploading: bool,
    timer: Timer,

}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploading: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploading {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploading = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.exploading && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] =  if self.exploading { "*" } else { "|" }
    }
}