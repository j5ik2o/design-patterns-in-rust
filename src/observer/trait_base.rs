use std::fmt::Debug;
use std::thread::Thread;
use std::{thread, time};

use rand::prelude::ThreadRng;
use rand::Rng;

pub trait NumberGenerator {
  fn add_observer(&mut self, observer: Box<dyn Observer>);
  fn delete_observer(&mut self, observer: Box<dyn Observer>);
  fn notify_observers(&self);
  fn get_number(&self) -> u32;
  fn execute(&mut self);
}

#[derive(Debug)]
pub struct RandomNumberGenerator {
  observers: Vec<Box<dyn Observer>>,
  rng: ThreadRng,
  number: u32,
}

impl RandomNumberGenerator {
  pub fn new() -> Self {
    Self {
      observers: Vec::new(),
      rng: rand::thread_rng(),
      number: 0,
    }
  }
}

impl NumberGenerator for RandomNumberGenerator {
  fn add_observer(&mut self, observer: Box<dyn Observer>) {
    self.observers.push(observer);
  }

  fn delete_observer(&mut self, observer: Box<dyn Observer>) {
    let index = self
      .observers
      .iter()
      .position(|e| {
        let p1: *const dyn Observer = &**e;
        let p2: *const dyn Observer = &*observer;
        std::ptr::addr_eq(p1, p2)
      })
      .unwrap();
    self.observers.remove(index);
  }

  fn notify_observers(&self) {
    for o in &self.observers {
      o.update(self)
    }
  }

  fn get_number(&self) -> u32 {
    self.number
  }

  fn execute(&mut self) {
    for _ in 0..20 {
      self.number = self.rng.gen_range(0..=49);
      self.notify_observers();
    }
  }
}

pub trait Observer: Debug {
  fn update(&self, generator: &dyn NumberGenerator);
}

#[derive(Debug)]
pub struct DigitObserver;

impl DigitObserver {
  pub fn new() -> Self {
    Self
  }
}

impl Observer for DigitObserver {
  fn update(&self, generator: &dyn NumberGenerator) {
    println!("DigitObserver:{}", generator.get_number());
    thread::sleep(time::Duration::from_millis(100));
  }
}

#[derive(Debug)]
pub struct GraphObserver;

impl GraphObserver {
  pub fn new() -> Self {
    Self
  }
}

impl Observer for GraphObserver {
  fn update(&self, generator: &dyn NumberGenerator) {
    print!("GraphObserver:");
    let count = generator.get_number();
    for _ in 0..count {
      print!("*");
    }
    println!();
    thread::sleep(time::Duration::from_millis(100));
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut generator = RandomNumberGenerator::new();
    let observer1 = DigitObserver::new();
    let observer2 = GraphObserver::new();
    generator.add_observer(Box::new(observer1));
    generator.add_observer(Box::new(observer2));
    generator.execute();
  }
}
