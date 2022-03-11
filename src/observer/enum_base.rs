use rand::prelude::ThreadRng;
use rand::Rng;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::{thread, time};

pub enum NumberGenerator {
  RandomNumber(RandomNumberNumberGenerator),
}

impl NumberGenerator {
  pub fn get_number(&self) -> u32 {
    match self {
      NumberGenerator::RandomNumber(g) => g.get_number(),
      _ => panic!(),
    }
  }
}

#[derive(Clone)]
pub struct RandomNumberNumberGenerator {
  inner: Rc<RefCell<RandomNumberNumberGeneratorInner>>,
}

struct RandomNumberNumberGeneratorInner {
  observers: Vec<Observer>,
  rng: ThreadRng,
  number: u32,
}

impl RandomNumberNumberGenerator {
  fn add_observer(&mut self, observer: Observer) {
    let mut g = (&*self.inner).borrow_mut();
    g.observers.push(observer);
  }

  fn delete_observer(&mut self, observer: &Observer) {
    let mut g = (&*self.inner).borrow_mut();
    let index = g
      .observers
      .iter()
      .position(|e| {
        let p1: *const Observer = e;
        let p2: *const Observer = observer;
        p1 == p2
      })
      .unwrap();
    g.observers.remove(index);
  }

  fn notify_observers(&self) {
    let g = (&*self.inner).borrow();
    for o in &g.observers {
      let p = NumberGenerator::RandomNumber(self.clone());
      o.update(&p);
    }
  }

  fn get_number(&self) -> u32 {
    let g = (&*self.inner).borrow();
    g.number
  }

  fn execute(&mut self) {
    for _ in 0..20 {
      let mut g = (&*self.inner).borrow_mut();
      g.number = g.rng.gen_range(0, 49);
      self.notify_observers();
    }
  }
}

pub trait AnyObserver: Debug {
  fn update(&self, generator: &NumberGenerator);
}

#[derive(Clone)]
pub enum Observer {
  Digit,
  Graph,
  Any(Rc<dyn AnyObserver>),
}

impl Observer {
  pub fn update(&self, generator: &NumberGenerator) {
    match self {
      Observer::Digit => {
        println!("DigitObserver:{}", generator.get_number());
        thread::sleep(time::Duration::from_millis(100));
      }
      Observer::Graph => {
        print!("GraphObserver:");
        let count = generator.get_number();
        for _ in 0..count {
          print!("*");
        }
        println!("");
        thread::sleep(time::Duration::from_millis(100));
      }
      Observer::Any(rc) => rc.update(generator),
    }
  }
}
