use std::rc::Rc;

pub trait Display {
  fn get_columns(&self) -> usize;
  fn get_rows(&self) -> u32;
  fn get_row_text(&self, row: u32) -> String;

  fn show(&self) {
    for i in 0..self.get_rows() {
      let s = self.get_row_text(i);
      println!("{}", s)
    }
  }
}

pub struct StringDisplay(String);

impl StringDisplay {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl Display for StringDisplay {
  fn get_columns(&self) -> usize {
    self.0.len()
  }

  fn get_rows(&self) -> u32 {
    1
  }

  fn get_row_text(&self, row: u32) -> String {
    if row != 0 {
      panic!("index of bounds");
    }
    self.0.clone()
  }
}

pub trait Border: Display {}

pub struct FullBorder {
  underlying: Rc<dyn Display>,
}

impl FullBorder {
  pub fn new(underlying: Rc<dyn Display>) -> Self {
    Self { underlying }
  }

  fn make_line(ch: char, count: usize) -> String {
    let mut line = String::new();
    for _ in 0..count {
      line.push(ch);
    }
    line
  }
}

impl Display for FullBorder {
  fn get_columns(&self) -> usize {
    1 + self.underlying.get_columns() + 1
  }

  fn get_rows(&self) -> u32 {
    1 + self.underlying.get_rows() + 1
  }

  fn get_row_text(&self, row: u32) -> String {
    if row == 0 {
      format!("+{}+", FullBorder::make_line('-', self.underlying.get_columns()))
    } else if row == self.underlying.get_rows() + 1 {
      format!("+{}+", FullBorder::make_line('-', self.underlying.get_columns()))
    } else {
      format!("|{}|", self.underlying.get_row_text(row - 1))
    }
  }
}

pub struct SideBorder {
  underlying: Rc<dyn Display>,
  border_char: char,
}

impl SideBorder {
  pub fn new(underlying: Rc<dyn Display>, ch: char) -> Self {
    Self {
      underlying,
      border_char: ch,
    }
  }
}

impl Display for SideBorder {
  fn get_columns(&self) -> usize {
    1 + self.underlying.get_columns() + 1
  }

  fn get_rows(&self) -> u32 {
    self.underlying.get_rows()
  }

  fn get_row_text(&self, row: u32) -> String {
    format!(
      "{}{}{}",
      self.border_char,
      self.underlying.get_row_text(row),
      self.border_char
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let b1 = Rc::new(StringDisplay::new("Hello, world."));
    let b2 = Rc::new(SideBorder::new(b1.clone(), '#'));
    let b3 = FullBorder::new(b2.clone());
    b1.show();
    b2.show();
    b3.show();
    let b4 = SideBorder::new(
      Rc::new(FullBorder::new(Rc::new(FullBorder::new(Rc::new(SideBorder::new(
        Rc::new(FullBorder::new(Rc::new(StringDisplay::new("Hello, world.")))),
        '*',
      )))))),
      '/',
    );
    b4.show();
  }
}
