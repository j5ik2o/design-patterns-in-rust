use crate::adaptor::Banner;

pub enum Print {
  PrintBanner(Banner),
}

impl Print {
  pub fn of_print(banner: Banner) -> Self {
    Print::PrintBanner(banner)
  }

  pub fn print_weak(&self) {
    match self {
      Print::PrintBanner(b) => b.show_with_paren(),
    }
  }

  pub fn print_strong(&self) {
    match self {
      Print::PrintBanner(b) => b.show_with_aster(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    // BannerをPrintとして扱える
    let p: Print = Print::of_print(Banner::new("Hello"));
    fn print(p: &Print) {
      p.print_weak();
      p.print_strong();
    }
    print(&p)
  }
}
