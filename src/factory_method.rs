use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub trait Product: Display {
  fn r#use(&self);
}

trait FactoryBase {
  fn create_product(&self, owner: &str) -> Rc<dyn Product>;
  fn register_product(&mut self, product: Rc<dyn Product>);
}

pub trait Factory: FactoryBase {
  fn create(&mut self, owner: &str) -> Rc<dyn Product> {
    let p = self.create_product(owner);
    self.register_product(p.clone());
    p
  }
}

#[derive(Debug)]
pub struct IdCard {
  owner: String,
}

impl IdCard {
  pub fn new(owner: &str) -> Self {
    println!("{}のカードを作ります", owner);
    Self {
      owner: owner.to_owned(),
    }
  }

  pub fn owner(&self) -> &str {
    &self.owner
  }
}

impl Display for IdCard {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[IdCard:{}]", self.owner)
  }
}

impl Product for IdCard {
  fn r#use(&self) {
    println!("{}を使います", self.owner)
  }
}

#[derive(Debug)]
pub struct IdCardFactory;

impl IdCardFactory {
  pub fn new() -> Self {
    Self
  }
}

impl FactoryBase for IdCardFactory {
  fn create_product(&self, owner: &str) -> Rc<dyn Product> {
    Rc::new(IdCard::new(owner))
  }

  fn register_product(&mut self, product: Rc<dyn Product>) {
    println!("{}を登録しました", product)
  }
}

impl Factory for IdCardFactory {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut factory = IdCardFactory::new();
    let card1 = factory.create("Hiroshi Yuki");
    let card2 = factory.create("Tomura");
    let card3 = factory.create("Hanako Sato");
    card1.r#use();
    card2.r#use();
    card3.r#use();
  }
}
