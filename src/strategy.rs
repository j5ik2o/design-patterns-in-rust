use std::fmt::{Display, Formatter};

mod enum_base;
mod trait_base;

#[derive(Clone, Debug, PartialEq)]
pub enum Hand {
  GUU,
  CHO,
  PAA,
}

impl Hand {
  pub fn name(&self) -> &str {
    match self {
      Hand::GUU => "グー",
      Hand::CHO => "チョキ",
      Hand::PAA => "パー",
    }
  }

  pub fn get_hand(value: u32) -> Self {
    match value {
      0 => Hand::GUU,
      1 => Hand::CHO,
      2 => Hand::PAA,
      _ => panic!("not found"),
    }
  }

  fn hand_value(&self) -> i32 {
    match self {
      Hand::GUU => 0,
      Hand::CHO => 1,
      Hand::PAA => 2,
    }
  }

  fn fight(&self, h: Hand) -> i32 {
    if *self == h {
      0
    } else if (self.hand_value() + 1) % 3 == h.hand_value() {
      1
    } else {
      -1
    }
  }

  pub fn is_stronger_than(&self, h: Hand) -> bool {
    self.fight(h) == 1
  }

  pub fn is_weaker_than(&self, h: Hand) -> bool {
    self.fight(h) == -1
  }
}

impl Display for Hand {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}
