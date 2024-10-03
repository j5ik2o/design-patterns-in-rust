use crate::chain_of_responsibility::Trouble;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub enum Support {
  No(NoSupport),
  Limit(LimitSupport),
  Odd(OddSupport),
  Special(SpecialSupport),
}

impl Support {
  pub fn of_no(name: &str, next: Option<Rc<Support>>) -> Self {
    Support::No(NoSupport::new(name, next))
  }

  pub fn of_limit(name: &str, limit: u32, next: Option<Rc<Support>>) -> Self {
    Support::Limit(LimitSupport::new(name, limit, next))
  }

  pub fn of_odd(name: &str, next: Option<Rc<Support>>) -> Self {
    Support::Odd(OddSupport::new(name, next))
  }

  pub fn of_special(name: &str, number: u32, next: Option<Rc<Support>>) -> Self {
    Support::Special(SpecialSupport::new(name, number, next))
  }

  pub fn support(&self, trouble: &Trouble) {
    match self {
      Support::No(u) => u.support(trouble),
      Support::Limit(u) => u.support(trouble),
      Support::Odd(u) => u.support(trouble),
      Support::Special(u) => u.support(trouble),
    }
  }
}

trait SupportBehaviorBase: std::fmt::Display + Debug {
  fn done(&self, trouble: &Trouble) {
    println!("{} is resolved by {}.", trouble, self);
  }
  fn fail(&self, trouble: &Trouble) {
    println!("{} cannot be resolved.", trouble);
  }
  fn next(&self) -> Option<Rc<Support>>;
}

pub trait SupportBehavior: SupportBehaviorBase {
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

#[derive(Debug)]
pub struct NoSupport {
  name: String,
  next: Option<Rc<Support>>,
}

impl NoSupport {
  pub fn new(name: &str, next: Option<Rc<Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
    }
  }
}

impl SupportBehaviorBase for NoSupport {
  fn next(&self) -> Option<Rc<Support>> {
    self.next.clone()
  }
}

impl Display for NoSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@NoSupport]", self.name)
  }
}

impl SupportBehavior for NoSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    // print!("NoSupport: false ");
    false
  }
}

#[derive(Debug)]
pub struct LimitSupport {
  name: String,
  next: Option<Rc<Support>>,
  limit: u32,
}

impl LimitSupport {
  pub fn new(name: &str, limit: u32, next: Option<Rc<Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
      limit,
    }
  }
}

impl SupportBehaviorBase for LimitSupport {
  fn next(&self) -> Option<Rc<Support>> {
    self.next.clone()
  }
}

impl Display for LimitSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@LimitSupport]", self.name)
  }
}

impl SupportBehavior for LimitSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() < self.limit
  }
}

#[derive(Debug)]
pub struct OddSupport {
  name: String,
  next: Option<Rc<Support>>,
}

impl OddSupport {
  pub fn new(name: &str, next: Option<Rc<Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
    }
  }
}

impl SupportBehaviorBase for OddSupport {
  fn next(&self) -> Option<Rc<Support>> {
    self.next.clone()
  }
}

impl Display for OddSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@OddSupport]", self.name)
  }
}

impl SupportBehavior for OddSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() % 2 == 1
  }
}

#[derive(Debug)]
pub struct SpecialSupport {
  name: String,
  next: Option<Rc<Support>>,
  number: u32,
}

impl SpecialSupport {
  pub fn new(name: &str, number: u32, next: Option<Rc<Support>>) -> Self {
    Self {
      name: name.to_owned(),
      next,
      number,
    }
  }
}

impl SupportBehaviorBase for SpecialSupport {
  fn next(&self) -> Option<Rc<Support>> {
    self.next.clone()
  }
}

impl Display for SpecialSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@SpecialSupport]", self.name)
  }
}

impl SupportBehavior for SpecialSupport {
  fn resolve(&self, trouble: &Trouble) -> bool {
    trouble.number() == self.number
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test() {
    let fred = Rc::new(Support::of_limit("Fred", 300, None));
    let elmo = Rc::new(Support::of_odd("Elmo", Some(fred)));
    let diana = Rc::new(Support::of_limit("Diana", 200, Some(elmo)));
    let charlie = Rc::new(Support::of_special("Charlie", 429, Some(diana)));
    let bob = Rc::new(Support::of_limit("Bob", 100, Some(charlie)));
    let alice = Support::of_no("Alice", Some(bob));

    for i in (0..500).step_by(33) {
      let t = Trouble::new(i);
      alice.support(&t);
    }
  }
}
