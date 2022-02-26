use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Singleton {
  name: String,
}

pub static SINGLETON: Lazy<Singleton> = Lazy::new(|| Singleton {
  name: "TEST".to_owned(),
});

#[cfg(test)]
mod test {
  use super::*;
  use std::borrow::Borrow;

  #[test]
  fn test() {
    let s = &SINGLETON.borrow().name;
    println!("name = {}", s);
  }
}
