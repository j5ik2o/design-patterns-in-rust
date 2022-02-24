use std::{thread, time};

pub trait Printable {
  fn set_printer_name(&mut self, name: &str);
  fn get_printer_name(&self) -> &str;
  fn print(&mut self, text: &str);
}

pub struct Printer {
  name: String,
}

impl Printer {
  fn heavy_job(msg: String) {
    print!("{}", msg);
    for i in 0..5 {
      thread::sleep(time::Duration::from_millis(1000));
      println!(".");
    }
    println!("完了。");
  }

  pub fn new(name: &str) -> Self {
    Self::heavy_job(format!("Printerのインスタンス({})を生成中", name));
    Self { name: name.to_owned() }
  }
}

impl Printable for Printer {
  fn set_printer_name(&mut self, name: &str) {
    self.name = name.to_owned();
  }

  fn get_printer_name(&self) -> &str {
    &self.name
  }

  fn print(&mut self, text: &str) {
    println!("=== {} ===", self.name);
    println!("{}", text);
  }
}

pub struct PrinterProxy {
  name: String,
  underlying: Option<Printer>,
}

impl PrinterProxy {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      underlying: None,
    }
  }

  fn realize(&mut self) {
    if self.underlying.is_none() {
      self.underlying = Some(Printer::new(&self.name))
    }
  }
}

impl Printable for PrinterProxy {
  fn set_printer_name(&mut self, name: &str) {
    if self.underlying.is_some() {
      self.underlying.as_mut().unwrap().set_printer_name(name);
    }
    self.name = name.to_owned();
  }

  fn get_printer_name(&self) -> &str {
    &self.name
  }

  fn print(&mut self, text: &str) {
    self.realize();
    self.underlying.as_mut().unwrap().print(text);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut p = PrinterProxy::new("Alice");
    println!("名前は現在{}です。", p.get_printer_name());
    p.set_printer_name("Blob");
    println!("名前は現在{}です。", p.get_printer_name());
    p.print("Hello, world.");
  }
}
