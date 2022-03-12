use std::fmt::{Display, Formatter};

mod enum_base;
mod trait_base;

#[derive(Debug)]
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
