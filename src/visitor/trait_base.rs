use std::fmt::Debug;

pub trait Visitor {
  fn visit_title(&mut self, title: &Title);
  fn visit_text(&mut self, text: &Text);
  fn visit_hyperlink(&mut self, hyperlink: &HyperLink);
}

pub trait Element: Debug {
  fn accept(&self, visitor: &mut dyn Visitor);
}

#[derive(Debug)]
pub struct Title<'a> {
  text: &'a str,
}

impl<'a> Title<'a> {
  pub fn new(text: &'a str) -> Self {
    Self { text }
  }
}

impl<'a> Element for Title<'a> {
  fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_title(self)
  }
}

#[derive(Debug)]
pub struct Text<'a> {
  text: &'a str,
}

impl<'a> Text<'a> {
  pub fn new(text: &'a str) -> Self {
    Self { text }
  }
}

impl<'a> Element for Text<'a> {
  fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_text(self)
  }
}

#[derive(Debug)]
pub struct HyperLink<'a> {
  text: &'a str,
  url: &'a str,
}

impl<'a> HyperLink<'a> {
  pub fn new(text: &'a str, url: &'a str) -> Self {
    Self { text, url }
  }
}

impl<'a> Element for HyperLink<'a> {
  fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_hyperlink(self)
  }
}

#[derive(Debug)]
pub struct Document<'a> {
  parts: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Document<'a> {
  pub fn new(parts: impl IntoIterator<Item = Box<dyn Element + 'a>>) -> Self {
    Self {
      parts: parts.into_iter().collect(),
    }
  }

  pub fn accept(&self, visitor: &mut dyn Visitor) {
    for e in &self.parts {
      e.accept(visitor)
    }
  }
}

#[derive(Debug, Default)]
pub struct HtmlExporterVisitor {
  builder: String,
}

impl HtmlExporterVisitor {
  pub fn new() -> Self {
    Self::default()
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

#[derive(Debug, Default)]
pub struct PlainTextExporterVisitor {
  builder: String,
}

impl PlainTextExporterVisitor {
  pub fn new() -> Self {
    Self::default()
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

  #[test]
  fn test() {
    let values: Vec<Box<dyn Element>> = vec![
      Box::new(Title::new("The Visitor Pattern Example")),
      Box::new(Text::new(
        "The visitor pattern helps us add extra functionality without changing the classes.",
      )),
      Box::new(HyperLink::new("Go check it online!", "https://www.google.com/")),
      Box::new(Text::new("Thanks!")),
    ];
    let document = Document::new(values);

    let mut html_exporter = HtmlExporterVisitor::new();
    let mut plain_text_exporter = PlainTextExporterVisitor::new();

    println!("Export to html:");
    document.accept(&mut html_exporter);
    println!("{}", html_exporter.get_html());

    println!("Export to plain:");
    document.accept(&mut plain_text_exporter);
    println!("{}", plain_text_exporter.get_text());
  }
}
