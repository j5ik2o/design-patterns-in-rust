use std::fmt::Debug;

pub trait Display {
  fn open(&mut self);
  fn print(&self);
  fn close(&mut self);
  fn display(&mut self);
}

#[derive(Debug)]
pub struct DisplayDefault<DI: DisplayImpl> {
  underlying: DI,
}

impl<DI: DisplayImpl> DisplayDefault<DI> {
  pub fn new(underlying: DI) -> Self {
    Self { underlying }
  }
}

impl<DI: DisplayImpl> Display for DisplayDefault<DI> {
  fn open(&mut self) {
    self.underlying.raw_open();
  }

  fn print(&self) {
    self.underlying.raw_print();
  }

  fn close(&mut self) {
    self.underlying.raw_close();
  }

  fn display(&mut self) {
    self.open();
    self.print();
    self.close();
  }
}

#[derive(Debug)]
pub struct CountDisplay<DI: DisplayImpl> {
  underlying: DisplayDefault<DI>,
}

impl<DI: DisplayImpl> CountDisplay<DI> {
  pub fn new(underlying: DI) -> Self {
    Self {
      underlying: DisplayDefault::new(underlying),
    }
  }

  pub fn multi_display(&mut self, times: u32) {
    self.open();
    for i in 0..times {
      self.print();
    }
    self.close();
  }
}

impl<DI: DisplayImpl> Display for CountDisplay<DI> {
  fn open(&mut self) {
    self.underlying.open();
  }

  fn print(&self) {
    self.underlying.print();
  }

  fn close(&mut self) {
    self.underlying.close();
  }

  fn display(&mut self) {
    self.underlying.display();
  }
}

pub trait DisplayImpl: Debug {
  fn raw_open(&mut self);
  fn raw_print(&self);
  fn raw_close(&mut self);
}

#[derive(Debug)]
pub struct StringDisplayImpl {
  string: String,
  width: u32,
}

impl StringDisplayImpl {
  pub fn new(string: &str) -> Self {
    Self {
      string: string.to_owned(),
      width: 0,
    }
  }

  fn print_line(&self) {
    print!("+");
    for _ in 0..self.width {
      print!("-")
    }
    println!("+");
  }
}

impl DisplayImpl for StringDisplayImpl {
  fn raw_open(&mut self) {
    self.print_line();
  }

  fn raw_print(&self) {
    println!("|{}|", self.string);
  }

  fn raw_close(&mut self) {
    self.print_line();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut d1 = DisplayDefault::new(StringDisplayImpl::new("Hello, Japan."));
    let mut d2 = DisplayDefault::new(StringDisplayImpl::new("Hello, World"));
    let mut d3 = CountDisplay::new(StringDisplayImpl::new("Hello, Universe."));
    d1.display();
    d2.display();
    d3.display();
    d3.multi_display(5);
  }
}
