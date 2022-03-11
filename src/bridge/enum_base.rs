pub enum Display {
  Default(DisplayImpl),
  Count(CountDisplay),
}

pub struct CountDisplay(Box<Display>);

impl CountDisplay {
  pub fn multi_display(&mut self, times: u32) {
    self.0.open();
    for i in 0..times {
      self.0.print();
    }
    self.0.close();
  }
}

impl Display {
  pub fn of_default(imp: DisplayImpl) -> Self {
    Display::Default(imp)
  }

  pub fn of_count(imp: DisplayImpl) -> Self {
    Display::Count(CountDisplay(Box::new(Self::of_default(imp))))
  }

  pub fn open(&mut self) {
    match self {
      Display::Default(underlying) => underlying.raw_open(),
      Display::Count(underlying) => underlying.0.open(),
    }
  }

  pub fn print(&self) {
    match self {
      Display::Default(underlying) => underlying.raw_print(),
      Display::Count(underlying) => underlying.0.print(),
    }
  }

  pub fn close(&mut self) {
    match self {
      Display::Default(underlying) => underlying.raw_close(),
      Display::Count(underlying) => underlying.0.close(),
    }
  }

  pub fn display(&mut self) {
    match self {
      Display::Default(underlying) => {
        self.open();
        self.print();
        self.close();
      }
      Display::Count(underlying) => underlying.0.display(),
    }
  }

  pub fn as_count_display_mut(&mut self) -> Option<&mut CountDisplay> {
    match self {
      Display::Count(c) => Some(c),
      _ => None,
    }
  }
}

pub enum DisplayImpl {
  String(String, u32),
}

impl DisplayImpl {
  pub fn of_string(s: &str) -> Self {
    DisplayImpl::String(s.to_owned(), 0)
  }

  fn print_line(&self) {
    match self {
      DisplayImpl::String(.., width) => {
        print!("+");
        for _ in 0..*width {
          print!("-")
        }
        println!("+");
      }
    }
  }

  fn raw_open(&mut self) {
    match self {
      DisplayImpl::String(..) => self.print_line(),
    }
  }

  fn raw_print(&self) {
    match self {
      DisplayImpl::String(string, ..) => println!("|{}|", string),
    }
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
    let mut d1 = Display::of_default(DisplayImpl::of_string("Hello, Japan."));
    let mut d2 = Display::of_default(DisplayImpl::of_string("Hello, World"));
    let mut d3 = Display::of_count(DisplayImpl::of_string("Hello, Universe."));
    d1.display();
    d2.display();
    d3.display();
    d3.as_count_display_mut().unwrap().multi_display(5);
  }
}
