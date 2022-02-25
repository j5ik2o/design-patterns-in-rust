use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub enum Entry {
  File {
    name: String,
    size: usize,
  },
  Directory {
    name: String,
    entries: Vec<Rc<RefCell<Entry>>>,
  },
}

impl Display for Entry {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Entry::File { name, size } => write!(f, "{} ({})", name, size),
      Entry::Directory { name, .. } => write!(f, "{} ({})", name, self.get_size()),
    }
  }
}

impl Entry {
  fn of_file(name: &str, size: usize) -> Self {
    Entry::File {
      name: name.to_owned(),
      size,
    }
  }

  fn of_directory(name: &str) -> Self {
    Entry::Directory {
      name: name.to_owned(),
      entries: vec![],
    }
  }

  fn print_line_with_prefix(&self, prefix: &str) {
    match self {
      Entry::File { .. } => println!("{}/{}", prefix, self),
      Entry::Directory { name, entries } => {
        println!("{}/{}", prefix, self);
        for entry in entries {
          let entry_ref = (&**entry).borrow();
          entry_ref.print_line_with_prefix(&format!("{}/{}", prefix, name))
        }
      }
    }
  }

  pub fn get_name(&self) -> &str {
    match self {
      Entry::File { name, .. } => name,
      Entry::Directory { name, .. } => name,
    }
  }

  pub fn get_size(&self) -> usize {
    match self {
      Entry::File { size, .. } => *size,
      Entry::Directory { entries, .. } => entries.iter().fold(0, |r, e| r + e.borrow().get_size()),
    }
  }

  pub fn print_line(&self) {
    self.print_line_with_prefix("");
  }

  // FIXME: enumベースではEntry::Fileのときにaddが不要なのにメソッドを定義するハメになる…
  pub fn add(&mut self, entry: Rc<RefCell<Entry>>) {
    match self {
      Entry::Directory { entries, .. } => entries.push(entry),
      _ => panic!("unsupported operation!!!"),
    }
  }
}

#[cfg(test)]
mod test {
  use std::borrow::BorrowMut;
  use std::cell::RefCell;

  use super::*;

  #[test]
  fn test() {
    let mut rootdir = Entry::of_directory("root");
    let bindir = Rc::new(RefCell::new(Entry::of_directory("bin")));
    let tmpdir = Rc::new(RefCell::new(Entry::of_directory("tmp")));
    let usrdir = Rc::new(RefCell::new(Entry::of_directory("usr")));

    rootdir.add(bindir.clone());
    rootdir.add(tmpdir.clone());
    rootdir.add(usrdir.clone());

    {
      let mut bindir_ref = (&*bindir).borrow_mut();
      bindir_ref.add(Rc::new(RefCell::new(Entry::of_file("vi", 10000))));
      bindir_ref.add(Rc::new(RefCell::new(Entry::of_file("latex", 20000))));
      // bindir_ref.print_line();
    }

    let mut yuki = Rc::new(RefCell::new(Entry::of_directory("yuki")));
    let mut hanako = Rc::new(RefCell::new(Entry::of_directory("hanako")));
    let mut tomura = Rc::new(RefCell::new(Entry::of_directory("tomura")));

    {
      let mut usrdir_ref = (&*usrdir).borrow_mut();
      usrdir_ref.add(yuki.clone());
      usrdir_ref.add(hanako.clone());
      usrdir_ref.add(tomura.clone());
      // usrdir_ref.print_line();
    }

    {
      let mut yuki_ref = (&*yuki).borrow_mut();
      yuki_ref.add(Rc::new(RefCell::new(Entry::of_file("diary.html", 100))));
      yuki_ref.add(Rc::new(RefCell::new(Entry::of_file("Composite.java", 200))));
      // yuki_ref.print_line();
    }

    {
      let mut hanako_ref = (&*hanako).borrow_mut();
      hanako_ref.add(Rc::new(RefCell::new(Entry::of_file("memo.tex", 300))));
      // hanako_ref.print_line();
    }

    {
      let mut tomura_ref = (&*tomura).borrow_mut();
      tomura_ref.add(Rc::new(RefCell::new(Entry::of_file("game.doc", 400))));
      tomura_ref.add(Rc::new(RefCell::new(Entry::of_file("junk.mail", 500))));
      // tomura_ref.print_line();
    }

    rootdir.print_line();
  }
}
