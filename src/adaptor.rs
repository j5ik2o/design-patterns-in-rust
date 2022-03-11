pub trait Print {
  fn print_weak(&self);
  fn print_strong(&self);
}

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

#[derive(Debug)]
pub struct PrintBanner(Banner);

impl PrintBanner {
  pub fn new(banner: Banner) -> Self {
    Self(banner)
  }
}

impl Print for PrintBanner {
  fn print_weak(&self) {
    self.0.show_with_paren();
  }

  fn print_strong(&self) {
    self.0.show_with_aster();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let p = PrintBanner::new(Banner::new("Hello"));
    p.print_weak();
    p.print_strong();
  }
}
