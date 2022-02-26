use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Singleton {
  name: String,
}

impl Singleton {
  pub fn name(&self) -> &str {
    &self.name
  }
}

pub static SINGLETON: Lazy<Singleton> = Lazy::new(|| Singleton {
  name: "TEST".to_owned(),
});

pub static SINGLETON_MUT: Lazy<Mutex<Singleton>> = Lazy::new(|| {
  Mutex::new(Singleton {
    name: "TEST".to_owned(),
  })
});

#[cfg(test)]
mod test {
  use super::*;
  use std::borrow::Borrow;

  #[test]
  fn test() {
    let si = &SINGLETON.borrow();
    let s = si.name();
    println!("name = {}", s);
  }

  #[test]
  fn test_mut() {
    let si = &SINGLETON_MUT.lock().unwrap();
    let s = si.name();
    println!("name = {}", s);
  }
}
