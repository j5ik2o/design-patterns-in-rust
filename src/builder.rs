use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

pub trait Builder: Debug {
  fn make_title(&mut self, title: &str);
  fn make_string(&mut self, str: &str);
  fn make_items(&mut self, items: &[&str]);
  fn close(&mut self);
}

#[derive(Debug)]
pub struct Director {
  builder: Rc<RefCell<dyn Builder>>,
}

impl Director {
  pub fn new(builder: Rc<RefCell<dyn Builder>>) -> Self {
    Self { builder }
  }

  pub fn build(&self) {
    let mut builder = self.builder.borrow_mut();
    builder.make_title("Greeting");
    builder.make_string("一般的なあいさつ");
    builder.make_items(&["How are you?", "Hello.", "Hi."]);
    builder.make_title("時間帯に応じたあいさつ");
    builder.make_string("一般的なあいさつ");
    builder.make_items(&["Good morning.", "Good afternoon.", "Good evening."]);
    builder.close();
  }
}

#[derive(Debug)]
pub struct TextBuilder {
  string: String,
}

impl TextBuilder {
  pub fn new() -> Self {
    Self { string: "".to_owned() }
  }

  pub fn get_text_result(&self) -> &str {
    &self.string
  }
}

impl Builder for TextBuilder {
  fn make_title(&mut self, title: &str) {
    self.string.push_str("==============================\n");
    self.string.push('『');
    self.string.push_str(title);
    self.string.push_str("』\n\n");
  }

  fn make_string(&mut self, str: &str) {
    self.string.push('■');
    self.string.push_str(str);
    self.string.push_str("\n\n");
  }

  fn make_items(&mut self, items: &[&str]) {
    for s in items {
      self.string.push_str("　・");
      self.string.push_str(s);
      self.string.push('\n');
    }
  }

  fn close(&mut self) {
    self.string.push_str("==============================\n");
  }
}

#[derive(Debug)]
pub struct HtmlBuilder {
  file_name: Option<String>,
  string: String,
}

impl HtmlBuilder {
  pub fn new() -> Self {
    Self {
      file_name: None,
      string: "".to_owned(),
    }
  }

  pub fn get_html_result(&self) -> &str {
    self.file_name.as_ref().unwrap()
  }
}

impl Builder for HtmlBuilder {
  fn make_title(&mut self, title: &str) {
    self.file_name = Some(format!("{}.html", title));
    self.string.push_str("<!DOCTYPE html>\n");
    self.string.push_str("<html>\n");
    self.string.push_str("<head><title>");
    self.string.push_str(title);
    self.string.push_str("</title></head>\n");
    self.string.push_str("<body>\n");
    self.string.push_str("<h1>");
    self.string.push_str(title);
    self.string.push_str("</h1>\n\n");
  }

  fn make_string(&mut self, str: &str) {
    self.string.push_str("<p>");
    self.string.push_str(str);
    self.string.push_str("</p>\n\n")
  }

  fn make_items(&mut self, items: &[&str]) {
    self.string.push_str("<ul>\n");
    for s in items {
      self.string.push_str("<li>");
      self.string.push_str(s);
      self.string.push_str("</li>\n");
    }
    self.string.push_str("</ul>\n\n")
  }

  fn close(&mut self) {
    self.string.push_str("</body>");
    self.string.push_str("</html>\n");
    let mut writer = BufWriter::new(File::create(self.file_name.as_ref().unwrap().clone()).unwrap());
    writer.write(self.string.as_bytes()).unwrap();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let builder = Rc::new(RefCell::new(TextBuilder::new()));
    let director = Director::new(builder.clone());
    director.build();
    let builder_ref = builder.borrow();
    let result = builder_ref.get_text_result();
    println!("{}", result);
  }
}
