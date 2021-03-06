trait Operation {
  fn open(&self);
  fn print(&self);
  fn close(&self);
}

pub trait AbstractDisplay: Operation {
  fn display(&self) {
    self.open();
    for _ in 0..5 {
      self.print();
    }
    self.close();
  }
}

#[derive(Debug)]
pub struct CharDisplay(char);

impl CharDisplay {
  pub fn new(c: char) -> Self {
    Self(c)
  }
}

impl Operation for CharDisplay {
  fn open(&self) {
    print!("<<");
  }

  fn print(&self) {
    print!("{}", self.0);
  }

  fn close(&self) {
    println!(">>");
  }
}

impl AbstractDisplay for CharDisplay {}

#[derive(Debug)]
pub struct StringDisplay(String, usize);

impl StringDisplay {
  pub fn new(s: String) -> Self {
    let w = s.len();
    Self(s, w)
  }

  fn print_line(&self) {
    print!("+");
    for _ in 0..self.1 {
      print!("-");
    }
    println!("+");
  }
}

impl Operation for StringDisplay {
  fn open(&self) {
    self.print_line();
  }

  fn print(&self) {
    println!("|{}|", self.0);
  }

  fn close(&self) {
    self.print_line();
  }
}

impl AbstractDisplay for StringDisplay {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn _1_usage_static_dispatch_generic() {
    let d1: CharDisplay = CharDisplay::new('H');
    let d2: StringDisplay = StringDisplay::new("Hello,world.".to_owned());

    fn display<T: AbstractDisplay>(ad: T) {
      ad.display();
    }

    display(d1);
    display(d2);
  }

  #[test]
  fn _2_usage_static_dispatch_impl_trait() {
    let d1: CharDisplay = CharDisplay::new('H');
    let d2: StringDisplay = StringDisplay::new("Hello,world.".to_owned());

    // impl traitはgenericのシンタックスシュガー
    fn display(ad: impl AbstractDisplay) {
      ad.display();
    }

    display(d1);
    display(d2);
  }

  #[test]
  fn _3_usage_dynamic_dispatch() {
    let d1: Box<dyn AbstractDisplay> = Box::new(CharDisplay::new('H'));
    let d2: Box<dyn AbstractDisplay> = Box::new(StringDisplay::new("Hello,world.".to_owned()));

    fn display(ad: Box<dyn AbstractDisplay>) {
      ad.display();
    }

    display(d1);
    display(d2);
  }
}
