use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Trouble {
  number: u32,
}

impl Display for Trouble {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Trouble {}]", self.number)
  }
}

impl Trouble {
  pub fn new(number: u32) -> Self {
    Self { number }
  }

  pub fn number(&self) -> u32 {
    self.number
  }
}

trait SupportBase: std::fmt::Display {
  fn done(&self, trouble: &Trouble) {
    println!("{} is resolved by {}.", trouble, self);
  }
  fn fail(&self, trouble: &Trouble) {
    println!("{} cannot be resolved.", trouble);
  }
  fn next(&self) -> Option<Rc<RefCell<dyn Support>>>;
}

pub trait Support: SupportBase {
  fn set_next(&mut self, next: Rc<RefCell<dyn Support>>) -> Rc<RefCell<dyn Support>>;

  fn resolve(&self, trouble: &Trouble) -> bool;

  fn support(&self, trouble: &Trouble) {
    if self.resolve(trouble) {
      self.done(trouble);
    } else if self.next().is_some() {
      let n = self.next().unwrap();
      let n_ref = (&*n).borrow();
      n_ref.support(trouble);
    } else {
      self.fail(trouble);
    }
  }
}

// ---

pub struct NoSupport {
  name: String,
  next: Option<Rc<RefCell<dyn Support>>>,
}

impl NoSupport {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      next: None,
    }
  }
}

impl Display for NoSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@NoSupport]", self.name)
  }
}

impl SupportBase for NoSupport {
  fn next(&self) -> Option<Rc<RefCell<dyn Support>>> {
    self.next.clone()
  }
}

impl Support for NoSupport {
  fn set_next(&mut self, next: Rc<RefCell<dyn Support>>) -> Rc<RefCell<dyn Support>> {
    self.next = Some(next.clone());
    next
  }

  fn resolve(&self, trouble: &Trouble) -> bool {
    // print!("NoSupport: false ");
    false
  }
}

// ---

pub struct LimitSupport {
  name: String,
  next: Option<Rc<RefCell<dyn Support>>>,
  limit: u32,
}

impl LimitSupport {
  pub fn new(name: &str, limit: u32) -> Self {
    Self {
      name: name.to_owned(),
      next: None,
      limit,
    }
  }
}

impl SupportBase for LimitSupport {
  fn next(&self) -> Option<Rc<RefCell<dyn Support>>> {
    self.next.clone()
  }
}

impl Display for LimitSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@LimitSupport]", self.name)
  }
}

impl Support for LimitSupport {
  fn set_next(&mut self, next: Rc<RefCell<dyn Support>>) -> Rc<RefCell<dyn Support>> {
    self.next = Some(next.clone());
    next
  }

  fn resolve(&self, trouble: &Trouble) -> bool {
    let result = if trouble.number() < self.limit { true } else { false };
    // print!("LimitSupport: {} ", result);
    result
  }
}

// ---

pub struct OddSupport {
  name: String,
  next: Option<Rc<RefCell<dyn Support>>>,
}

impl OddSupport {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      next: None,
    }
  }
}

impl SupportBase for OddSupport {
  fn next(&self) -> Option<Rc<RefCell<dyn Support>>> {
    self.next.clone()
  }
}

impl Display for OddSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@OddSupport]", self.name)
  }
}

impl Support for OddSupport {
  fn set_next(&mut self, next: Rc<RefCell<dyn Support>>) -> Rc<RefCell<dyn Support>> {
    self.next = Some(next.clone());
    next
  }

  fn resolve(&self, trouble: &Trouble) -> bool {
    let result = if trouble.number() % 2 == 1 { true } else { false };
    // print!("OddSupport: {} ", result);
    result
  }
}

// ---

pub struct SpecialSupport {
  name: String,
  next: Option<Rc<RefCell<dyn Support>>>,
  number: u32,
}

impl SpecialSupport {
  pub fn new(name: &str, number: u32) -> Self {
    Self {
      name: name.to_owned(),
      next: None,
      number,
    }
  }
}

impl SupportBase for SpecialSupport {
  fn next(&self) -> Option<Rc<RefCell<dyn Support>>> {
    self.next.clone()
  }
}

impl Display for SpecialSupport {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}@SpecialSupport]", self.name)
  }
}

impl Support for SpecialSupport {
  fn set_next(&mut self, next: Rc<RefCell<dyn Support>>) -> Rc<RefCell<dyn Support>> {
    self.next = Some(next.clone());
    next
  }

  fn resolve(&self, trouble: &Trouble) -> bool {
    let result = if trouble.number() == self.number { true } else { false };
    // print!("SpecialSupport: {} ", result);
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::borrow::Borrow;

  #[test]
  fn test() {
    let mut alice = NoSupport::new("Alice");
    let bob = Rc::new(RefCell::new(LimitSupport::new("Bob", 100)));
    let charlie = Rc::new(RefCell::new(SpecialSupport::new("Charlie", 429)));
    let diana = Rc::new(RefCell::new(LimitSupport::new("Diana", 200)));
    let elmo = Rc::new(RefCell::new(OddSupport::new("Elmo")));
    let fred = Rc::new(RefCell::new(LimitSupport::new("Fred", 300)));

    // 連鎖の形成
    alice
      .set_next(bob)
      .borrow_mut()
      .set_next(charlie)
      .borrow_mut()
      .set_next(diana)
      .borrow_mut()
      .set_next(elmo)
      .borrow_mut()
      .set_next(fred);

    for i in (0..500).step_by(33) {
      let t = Trouble::new(i);
      alice.support(&t);
    }
  }
}
