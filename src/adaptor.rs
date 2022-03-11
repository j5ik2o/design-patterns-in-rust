mod enum_base;
mod trait_base;

#[derive(Debug)]
pub struct Banner(String);

impl Banner {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }

  pub fn show_with_paren(&self) {
    println!("({})", self.0);
  }

  pub fn show_with_aster(&self) {
    println!("*{}*", self.0);
  }
}
