use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Visitor {
  fn visit_title(&mut self, title: &Title);
  fn visit_text(&mut self, text: &Text);
  fn visit_hyperlink(&mut self, hyperlink: &HyperLink);
}

pub trait Element {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>);
}

pub struct Title {
  text: String,
}

impl Title {
  pub fn new(text: &str) -> Self {
    Self { text: text.to_owned() }
  }
}

impl Element for Title {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) {
    (&*visitor).borrow_mut().visit_title(self)
  }
}

pub struct Text {
  text: String,
}

impl Text {
  pub fn new(text: &str) -> Self {
    Self { text: text.to_owned() }
  }
}

impl Element for Text {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) {
    (&*visitor).borrow_mut().visit_text(self)
  }
}

pub struct HyperLink {
  text: String,
  url: String,
}

impl HyperLink {
  pub fn new(text: &str, url: &str) -> Self {
    Self {
      text: text.to_owned(),
      url: url.to_owned(),
    }
  }
}

impl Element for HyperLink {
  fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) {
    (&*visitor).borrow_mut().visit_hyperlink(self)
  }
}

pub struct Document {
  parts: Vec<Rc<dyn Element>>,
}

impl Document {
  pub fn new(parts: impl IntoIterator<Item = Rc<dyn Element>>) -> Self {
    Self {
      parts: parts.into_iter().collect(),
    }
  }

  pub fn accept(&self, visitor: Rc<RefCell<dyn Visitor>>) {
    for e in &self.parts {
      e.accept(visitor.clone())
    }
  }
}

pub struct HtmlExporterVisitor {
  builder: String,
}

impl HtmlExporterVisitor {
  pub fn new() -> Self {
    Self { builder: "".to_owned() }
  }

  pub fn get_html(&self) -> &str {
    &self.builder
  }
}

impl Visitor for HtmlExporterVisitor {
  fn visit_title(&mut self, title: &Title) {
    self.builder.push_str(&format!("<h1>{}</h1>\n", title.text));
  }

  fn visit_text(&mut self, text: &Text) {
    self.builder.push_str(&format!("<p>{}</p>\n", text.text));
  }

  fn visit_hyperlink(&mut self, hyperlink: &HyperLink) {
    self
      .builder
      .push_str(&format!("<a href=\"{}\">{}</a>\n", hyperlink.url, hyperlink.text));
  }
}

pub struct PlainTextExporterVisitor {
  builder: String,
}

impl PlainTextExporterVisitor {
  pub fn new() -> Self {
    Self { builder: "".to_owned() }
  }

  pub fn get_text(&self) -> &str {
    &self.builder
  }
}

impl Visitor for PlainTextExporterVisitor {
  fn visit_title(&mut self, title: &Title) {
    self.builder.push_str(&format!("{}\n", title.text));
  }

  fn visit_text(&mut self, text: &Text) {
    self.builder.push_str(&format!("{}\n", text.text));
  }

  fn visit_hyperlink(&mut self, hyperlink: &HyperLink) {
    self
      .builder
      .push_str(&format!("{} {}\n", hyperlink.text, hyperlink.url));
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::borrow::Borrow;

  #[test]
  fn test() {
    let values: Vec<Rc<dyn Element>> = vec![
      Rc::new(Title::new("The Visitor Pattern Example")),
      Rc::new(Text::new(
        "The visitor pattern helps us add extra functionality without changing the classes.",
      )),
      Rc::new(HyperLink::new("Go check it online!", "https://www.google.com/")),
      Rc::new(Text::new("Thanks!")),
    ];
    let document = Document::new(values);

    let html_exporter = Rc::new(RefCell::new(HtmlExporterVisitor::new()));

    let plain_text_exporter = Rc::new(RefCell::new(PlainTextExporterVisitor::new()));

    println!("Export to html:");
    document.accept(html_exporter.clone());
    println!("{}", (&*html_exporter).borrow().get_html());

    println!("Export to plain:");
    document.accept(plain_text_exporter.clone());
    println!("{}", (&*plain_text_exporter).borrow().get_text());
  }
}
