use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use crate::chain_of_responsibility::Trouble;

trait SupportBase: std::fmt::Display + Debug {
  fn done(&self, trouble: &Trouble) {
    println!("{} is resolved by {}.", trouble, self);
  }
  fn fail(&self, trouble: &Trouble) {
    println!("{} cannot be resolved.", trouble);
  }
  fn next(&self) -> Option<Rc<dyn Support>>;
}

pub trait Support: SupportBase {
  fn resolve(&self, trouble: &Trouble) -> bool;

  fn support(&self, trouble: &Trouble) {
    if self.resolve(trouble) {
      self.done(trouble);
    } else if self.next().is_some() {
      let next_rc = self.next().unwrap();
      let next_ref = &*next_rc;
      next_ref.support(trouble);
    } else {
      self.fail(trouble);
    }
  }
}

// ---

#[derive(Debug)]
pub struct NoSupport {
  name: String,
  next: Option<Rc<dyn Support>>,
}

impl NoSupport {
  pub fn new(name: &str, next: Option<Rc<dyn Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
    }
  }
}

impl Display for NoSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@NoSupport]", self.name)
  }
}

impl SupportBase for NoSupport {
  fn next(&self) -> Option<Rc<dyn Support>> {
    self.next.clone()
  }
}

impl Support for NoSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    // print!("NoSupport: false ");
    false
  }
}

// ---

#[derive(Debug)]
pub struct LimitSupport {
  name: String,
  next: Option<Rc<dyn Support>>,
  limit: u32,
}

impl LimitSupport {
  pub fn new(name: &str, limit: u32, next: Option<Rc<dyn Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
      limit,
    }
  }
}

impl SupportBase for LimitSupport {
  fn next(&self) -> Option<Rc<dyn Support>> {
    self.next.clone()
  }
}

impl Display for LimitSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@LimitSupport]", self.name)
  }
}

impl Support for LimitSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() < self.limit
  }
}

// ---

#[derive(Debug)]
pub struct OddSupport {
  name: String,
  next: Option<Rc<dyn Support>>,
}

impl OddSupport {
  pub fn new(name: &str, next: Option<Rc<dyn Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
    }
  }
}

impl SupportBase for OddSupport {
  fn next(&self) -> Option<Rc<dyn Support>> {
    self.next.clone()
  }
}

impl Display for OddSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@OddSupport]", self.name)
  }
}

impl Support for OddSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() % 2 == 1
  }
}

// ---

#[derive(Debug)]
pub struct SpecialSupport {
  name: String,
  next: Option<Rc<dyn Support>>,
  number: u32,
}

impl SpecialSupport {
  pub fn new(name: &str, number: u32, next: Option<Rc<dyn Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
      number,
    }
  }
}

impl SupportBase for SpecialSupport {
  fn next(&self) -> Option<Rc<dyn Support>> {
    self.next.clone()
  }
}

impl Display for SpecialSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@SpecialSupport]", self.name)
  }
}

impl Support for SpecialSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() == self.number
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test() {
    let fred = Rc::new(LimitSupport::new("Fred", 300, None));
    let elmo = Rc::new(OddSupport::new("Elmo", Some(fred)));
    let diana = Rc::new(LimitSupport::new("Diana", 200, Some(elmo)));
    let charlie = Rc::new(SpecialSupport::new("Charlie", 429, Some(diana)));
    let bob = Rc::new(LimitSupport::new("Bob", 100, Some(charlie)));
    let alice = NoSupport::new("Alice", Some(bob));

    for i in (0..500).step_by(33) {
      let t = Trouble::new(i);
      alice.support(&t);
    }
  }
}
