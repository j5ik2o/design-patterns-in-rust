use std::{thread, time};

pub trait PrintableBehavior {
  fn set_printer_name(&mut self, name: &str);
  fn get_printer_name(&self) -> &str;
  fn print(&mut self, text: &str);
}

pub enum Printable {
  Printer(Printer),
  PrinterProxy(PrinterProxy),
}

impl Printable {
  pub fn of_printer(name: &str) -> Self {
    Printable::Printer(Printer::new(name))
  }

  pub fn of_printer_proxy(name: &str) -> Self {
    Printable::PrinterProxy(PrinterProxy::new(name))
  }
}

impl PrintableBehavior for Printable {
  fn set_printer_name(&mut self, name: &str) {
    match self {
      Printable::Printer(u) => u.set_printer_name(name),
      Printable::PrinterProxy(u) => u.set_printer_name(name),
    }
  }

  fn get_printer_name(&self) -> &str {
    match self {
      Printable::Printer(u) => u.get_printer_name(),
      Printable::PrinterProxy(u) => u.get_printer_name(),
    }
  }

  fn print(&mut self, text: &str) {
    match self {
      Printable::Printer(u) => u.print(text),
      Printable::PrinterProxy(u) => u.print(text),
    }
  }
}

#[derive(Debug)]
pub struct Printer {
  name: String,
}

impl Printer {
  fn heavy_job(msg: &str) {
    print!("{}", msg);
    for i in 0..5 {
      thread::sleep(time::Duration::from_millis(1000));
      println!(".");
    }
    println!("完了。");
  }

  pub fn new(name: &str) -> Self {
    Self::heavy_job(&format!("Printerのインスタンス({})を生成中", name));
    Self { name: name.to_owned() }
  }
}

impl PrintableBehavior for Printer {
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

#[derive(Debug)]
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

impl PrintableBehavior for PrinterProxy {
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
    let mut proxy = Printable::of_printer_proxy("Alice");
    fn process(p: &mut Printable) {
      println!("名前は現在{}です。", p.get_printer_name());
      p.set_printer_name("Blob");
      println!("名前は現在{}です。", p.get_printer_name());
      p.print("Hello, world.");
    }
    process(&mut proxy);
  }
}
