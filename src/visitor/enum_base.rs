pub enum Visitor {
  HtmlExporterVisitor { builder: String },
  PlainTextExporterVisitor { builder: String },
}

impl Visitor {
  pub fn get_result(&self) -> &str {
    match self {
      Visitor::HtmlExporterVisitor { builder } => builder,
      Visitor::PlainTextExporterVisitor { builder } => builder,
    }
  }

  pub fn of_html_exporter() -> Self {
    Visitor::HtmlExporterVisitor { builder: "".to_owned() }
  }

  pub fn of_plain_text_exporter() -> Self {
    Visitor::PlainTextExporterVisitor { builder: "".to_owned() }
  }

  pub fn visit_title(&mut self, title: &Title) {
    match self {
      Visitor::HtmlExporterVisitor { builder } => builder.push_str(&format!("<h1>{}</h1>\n", title.text)),
      Visitor::PlainTextExporterVisitor { builder } => builder.push_str(&format!("{}\n", title.text)),
    }
  }

  pub fn visit_text(&mut self, text: &Text) {
    match self {
      Visitor::HtmlExporterVisitor { builder } => builder.push_str(&format!("<p>{}</p>\n", text.text)),
      Visitor::PlainTextExporterVisitor { builder } => builder.push_str(&format!("{}\n", text.text)),
    }
  }

  pub fn visit_hyperlink(&mut self, hyperlink: &HyperLink) {
    match self {
      Visitor::HtmlExporterVisitor { builder } => {
        builder.push_str(&format!("<a href=\"{}\">{}</a>\n", hyperlink.url, hyperlink.text))
      }
      Visitor::PlainTextExporterVisitor { builder } => {
        builder.push_str(&format!("{} {}\n", hyperlink.text, hyperlink.url))
      }
    }
  }
}

#[derive(Debug)]
pub enum Element {
  Title(Title),
  Text(Text),
  HyperLink(HyperLink),
}

impl Element {
  pub fn of_title(s: &str) -> Self {
    Element::Title(Title::new(s))
  }

  pub fn of_text(s: &str) -> Self {
    Element::Text(Text::new(s))
  }

  pub fn of_hyper_link(text: &str, url: &str) -> Self {
    Element::HyperLink(HyperLink::new(text, url))
  }

  pub fn accept(&self, visitor: &mut Visitor) {
    match self {
      Element::Title(inner) => inner.accept(visitor),
      Element::Text(inner) => inner.accept(visitor),
      Element::HyperLink(inner) => inner.accept(visitor),
    }
  }
}

#[derive(Debug)]
pub struct Title {
  text: String,
}

impl Title {
  pub fn new(text: &str) -> Self {
    Self { text: text.to_owned() }
  }

  pub fn accept(&self, visitor: &mut Visitor) {
    visitor.visit_title(self)
  }
}

#[derive(Debug)]
pub struct Text {
  text: String,
}

impl Text {
  pub fn new(text: &str) -> Self {
    Self { text: text.to_owned() }
  }

  pub fn accept(&self, visitor: &mut Visitor) {
    visitor.visit_text(self)
  }
}

#[derive(Debug)]
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

  pub fn accept(&self, visitor: &mut Visitor) {
    visitor.visit_hyperlink(self)
  }
}

#[derive(Debug)]
pub struct Document {
  parts: Vec<Element>,
}

impl Document {
  pub fn new(parts: impl IntoIterator<Item = Element>) -> Self {
    Self {
      parts: parts.into_iter().collect(),
    }
  }

  pub fn accept(&self, visitor: &mut Visitor) {
    for e in &self.parts {
      e.accept(visitor)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let values: Vec<Element> = vec![
      Element::of_title("The Visitor Pattern Example"),
      Element::of_text("The visitor pattern helps us add extra functionality without changing the classes."),
      Element::of_hyper_link("Go check it online!", "https://www.google.com/"),
      Element::of_text("Thanks!"),
    ];

    let document = Document::new(values);
    let mut html_exporter = Visitor::of_html_exporter();
    let mut plain_text_exporter = Visitor::of_plain_text_exporter();

    println!("Export to html:");
    document.accept(&mut html_exporter);
    println!("{}", html_exporter.get_result());

    println!("Export to plain:");
    document.accept(&mut plain_text_exporter);
    println!("{}", plain_text_exporter.get_result());
  }
}
