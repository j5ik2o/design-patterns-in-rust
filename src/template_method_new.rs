pub enum DisplayType {
  Char(char),
  String(String),
}

impl DisplayType {
  pub fn display(&self) {
    self.open();
    for _ in 0..5 {
      self.print();
    }
    self.close();
  }

  fn print_line(&self) {
    match self {
      DisplayType::Char(..) => {}
      DisplayType::String(s) => {
        print!("+");
        for _ in 0..s.len() {
          print!("-");
        }
        println!("+");
      }
    }
  }

  fn open(&self) {
    match self {
      DisplayType::Char(..) => print!("<<"),
      DisplayType::String(..) => self.print_line(),
    }
  }

  fn print(&self) {
    match self {
      DisplayType::Char(c) => print!("{}", c),
      DisplayType::String(s) => println!("|{}|", s),
    }
  }

  fn close(&self) {
    match self {
      DisplayType::Char(..) => println!(">>"),
      DisplayType::String(..) => self.print_line(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let d1 = DisplayType::Char('H');
    let d2 = DisplayType::String("Hello,world.".to_owned());

    d1.display();
    d2.display();
  }
}
