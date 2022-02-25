pub trait Display {
  fn open(&mut self);
  fn print(&self);
  fn close(&mut self);
  fn display(&mut self);
}

pub struct DisplayDefault {
  underlying: Box<dyn DisplayImpl>,
}

impl DisplayDefault {
  pub fn new(underlying: Box<dyn DisplayImpl>) -> Self {
    Self { underlying }
  }
}

impl Display for DisplayDefault {
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

pub struct CountDisplay {
  underlying: DisplayDefault,
}

impl CountDisplay {
  pub fn new(underlying: Box<dyn DisplayImpl>) -> Self {
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

impl Display for CountDisplay {
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

pub trait DisplayImpl {
  fn raw_open(&mut self);
  fn raw_print(&self);
  fn raw_close(&mut self);
}

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
    let mut d1: Box<dyn Display> = Box::new(DisplayDefault::new(Box::new(StringDisplayImpl::new("Hello, Japan."))));
    let mut d2: Box<dyn Display> = Box::new(DisplayDefault::new(Box::new(StringDisplayImpl::new("Hello, World"))));
    let mut d3: CountDisplay = CountDisplay::new(Box::new(StringDisplayImpl::new("Hello, Universe.")));
    d1.display();
    d2.display();
    d3.display();
    d3.multi_display(5);
  }
}
