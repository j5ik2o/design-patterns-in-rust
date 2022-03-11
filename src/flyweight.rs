use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct BigChar {
  char_name: char,
  font_data: String,
}

unsafe impl Send for BigChar {}

impl Display for BigChar {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.font_data)
  }
}

impl BigChar {
  fn read_font_data(char_name: char) -> Result<String> {
    println!("read_font_data: {}", char_name);
    let file_name = format!("flyweight/big{}.txt", char_name);
    let mut reader = BufReader::new(File::open(file_name)?);
    let mut line: String = String::new();
    let mut buf = String::new();
    while let Ok(size) = reader.read_line(&mut line) {
      if size == 0 {
        break;
      }
      buf.push_str(&*line);
      line.clear();
    }
    Ok(buf.to_string())
  }

  pub fn new(char_name: char) -> Self {
    let font_data = Self::read_font_data(char_name).unwrap();
    Self { char_name, font_data }
  }
}

#[derive(Debug)]
pub struct BigCharFactory {
  pool: HashMap<char, Rc<BigChar>>,
}

unsafe impl Send for BigCharFactory {}

impl BigCharFactory {
  pub fn new() -> Self {
    Self { pool: HashMap::new() }
  }

  pub fn get_big_char(&mut self, char_name: char) -> Rc<BigChar> {
    let result = self
      .pool
      .entry(char_name)
      .or_insert_with(|| Rc::new(BigChar::new(char_name)));
    result.clone()
  }
}

pub static BIG_CHAR_FACTORY_SINGLETON: OnceCell<Mutex<BigCharFactory>> = OnceCell::new();

#[derive(Debug)]
pub struct BigString {
  big_chars: Vec<Rc<BigChar>>,
}

impl Display for BigString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut buf = String::new();
    for bc in &self.big_chars {
      buf = format!("{}{}", buf, bc);
    }
    write!(f, "{}", buf)
  }
}

impl BigString {
  pub fn new(string: &str) -> Self {
    let mut big_chars = Vec::with_capacity(string.len());
    let factory = BIG_CHAR_FACTORY_SINGLETON.get_or_init(|| Mutex::new(BigCharFactory::new()));
    for (i, c) in string.chars().enumerate() {
      let mut f = factory.lock().unwrap();
      let bc = f.get_big_char(c);
      big_chars.insert(i, bc);
    }
    Self { big_chars }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let bs = BigString::new("1928374650564738291");
    print!("{}", bs);
  }
}
