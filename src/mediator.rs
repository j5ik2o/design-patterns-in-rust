use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Colleague {
  fn name(&self) -> &str;
  fn on_changed(&mut self, msg: &str);
  fn run(&self);
}

pub trait Mediator {
  fn add_colleague(&mut self, colleague: Rc<RefCell<dyn Colleague>>);
  fn colleague_changed(&mut self, colleague_updated: &dyn Colleague, msg: &str);
}

pub struct ConcreteMediator {
  colleagues: HashMap<String, Rc<RefCell<dyn Colleague>>>,
}

impl ConcreteMediator {
  pub fn new() -> ConcreteMediator {
    ConcreteMediator {
      colleagues: HashMap::new(),
    }
  }
}

impl Mediator for ConcreteMediator {
  fn add_colleague(&mut self, colleague: Rc<RefCell<dyn Colleague>>) {
    let colleague_cloned = colleague.clone();
    let cr = colleague.borrow();
    self.colleagues.insert(cr.name().to_owned(), colleague_cloned);
  }

  fn colleague_changed(&mut self, colleague: &dyn Colleague, msg: &str) {
    self.colleagues.iter().for_each(|(k, v)| {
      if k != colleague.name() {
        (&**v).borrow_mut().on_changed(msg);
      }
    });
  }
}

pub struct ConcreteColleagueA {
  mediator: Rc<RefCell<dyn Mediator>>,
  name: String,
}

impl ConcreteColleagueA {
  pub fn new(mediator: Rc<RefCell<dyn Mediator>>, name: &str) -> ConcreteColleagueA {
    Self {
      mediator,
      name: name.to_owned(),
    }
  }
}


impl Colleague for ConcreteColleagueA {
  fn name(&self) -> &str {
    &self.name
  }

  fn on_changed(&mut self, msg: &str) {
    println!("{} received: {}", self.name, msg);
  }

  fn run(&self) {
    let mut mr = (&*self.mediator).borrow_mut();
    mr.colleague_changed(self, "Hello");
  }
}

pub struct ConcreteColleagueB {
  mediator: Rc<RefCell<dyn Mediator>>,
  name: String,
}

impl ConcreteColleagueB {
  pub fn new(mediator: Rc<RefCell<dyn Mediator>>, name: &str) -> Self {
    Self {
      mediator,
      name: name.to_owned(),
    }
  }
}

impl Colleague for ConcreteColleagueB {
  fn name(&self) -> &str {
    &self.name
  }

  fn on_changed(&mut self, msg: &str) {
    println!("{} received: {}", self.name, msg);
  }

  fn run(&self) {
    let mut mr = (&*self.mediator).borrow_mut();
    mr.colleague_changed(self, "Hi");
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mediator = Rc::new(RefCell::new(ConcreteMediator::new()));
    let colleague_a = Rc::new(RefCell::new(ConcreteColleagueA::new(mediator.clone(), "A")));
    let colleague_b = Rc::new(RefCell::new(ConcreteColleagueB::new(mediator.clone(), "B")));

    (&*mediator).borrow_mut().add_colleague(colleague_a.clone());
    (&*mediator).borrow_mut().add_colleague(colleague_b.clone());

    (&*colleague_a).borrow().run();
    (&*colleague_b).borrow().run();
  }
}

