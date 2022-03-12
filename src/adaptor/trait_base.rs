use crate::adaptor::Banner;

pub trait Print {
  fn print_weak(&self);
  fn print_strong(&self);
}

#[derive(Debug)]
pub struct PrintBanner(Banner);

impl PrintBanner {
  pub fn new(banner: Banner) -> Self {
    Self(banner)
  }
}

impl Print for PrintBanner {
  fn print_weak(&self) {
    self.0.show_with_paren();
  }

  fn print_strong(&self) {
    self.0.show_with_aster();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    // BannerをPrintとして扱える
    let print_banner = PrintBanner::new(Banner::new("Hello"));
    fn print(p: &dyn Print) {
      p.print_weak();
      p.print_strong();
    }
    print(&print_banner)
  }
}
