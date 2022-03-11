use std::rc::Rc;

#[derive(Debug)]
pub enum Display {
  String(String),
  SideBorder(Rc<Display>, char),
  FullBorder(Rc<Display>),
}

impl Display {
  fn make_line(ch: char, count: usize) -> String {
    let mut line = String::new();
    for _ in 0..count {
      line.push(ch);
    }
    line
  }

  pub fn of_string(value: &str) -> Self {
    Display::String(value.to_owned())
  }

  pub fn of_side_border(underlying: Rc<Display>, border_char: char) -> Self {
    Display::SideBorder(underlying, border_char)
  }

  pub fn of_full_border(underlying: Rc<Display>) -> Self {
    Display::FullBorder(underlying)
  }

  pub fn get_columns(&self) -> usize {
    match self {
      Display::String(value) => value.len(),
      Display::SideBorder(underlying, ..) => 1 + underlying.get_columns() + 1,
      Display::FullBorder(underlying) => 1 + underlying.get_columns() + 1,
    }
  }

  pub fn get_rows(&self) -> u32 {
    match self {
      Display::String(value) => 1,
      Display::SideBorder(underlying, ..) => underlying.get_rows(),
      Display::FullBorder(underlying) => 1 + underlying.get_rows() + 1,
    }
  }

  pub fn get_row_text(&self, row: u32) -> String {
    match self {
      Display::String(value) => {
        if row != 0 {
          panic!("index of bounds");
        }
        value.clone()
      }
      Display::SideBorder(underlying, border_char) => {
        format!("{}{}{}", border_char, underlying.get_row_text(row), border_char)
      }
      Display::FullBorder(underlying) => {
        if row == 0 {
          format!("+{}+", Self::make_line('-', underlying.get_columns()))
        } else if row == underlying.get_rows() + 1 {
          format!("+{}+", Self::make_line('-', underlying.get_columns()))
        } else {
          format!("|{}|", underlying.get_row_text(row - 1))
        }
      }
    }
  }

  pub fn show(&self) {
    for i in 0..self.get_rows() {
      let s = self.get_row_text(i);
      println!("{}", s)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let b1 = Rc::new(Display::of_string("Hello, world."));
    let b2 = Rc::new(Display::of_side_border(b1.clone(), '#'));
    let b3 = Display::of_full_border(b2.clone());
    b1.show();
    b2.show();
    b3.show();
    let b4 = Display::of_side_border(
      Rc::new(Display::of_full_border(Rc::new(Display::of_full_border(Rc::new(
        Display::of_side_border(
          Rc::new(Display::of_full_border(Rc::new(Display::of_string("Hello, world.")))),
          '*',
        ),
      ))))),
      '/',
    );
    b4.show();
  }
}
