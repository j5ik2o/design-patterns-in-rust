use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct File {
  name: String,
  size: usize,
}

pub struct Directory {
  name: String,
  entries: Vec<Rc<RefCell<Entry>>>,
}

pub enum Entry {
  FileEntry(File),
  DirectoryEntry(Directory),
}

impl Display for Entry {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Entry::FileEntry(file) => write!(f, "{}", file),
      Entry::DirectoryEntry(directory) => write!(f, "{}", directory),
    }
  }
}

impl Display for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.size)
  }
}

impl File {
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_size(&self) -> usize {
    self.size
  }

  fn print_line_with_prefix(&self, prefix: &str) {
    println!("{}/{}", prefix, self);
  }
}

impl Display for Directory {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.get_size())
  }
}

impl Directory {
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn add(&mut self, entry: Rc<RefCell<Entry>>) {
    self.entries.push(entry);
  }

  pub fn get_size(&self) -> usize {
    self.entries.iter().fold(0, |r, e| r + e.borrow().get_size())
  }

  fn print_line_with_prefix(&self, prefix: &str) {
    println!("{}/{}", prefix, self);
    for entry in &self.entries {
      let entry_ref = (&**entry).borrow();
      entry_ref.print_line_with_prefix(&format!("{}/{}", prefix, self.name))
    }
  }
}

impl Entry {
  fn of_file(name: &str, size: usize) -> Self {
    Entry::FileEntry(File {
      name: name.to_owned(),
      size,
    })
  }

  fn of_directory(name: &str) -> Self {
    Entry::DirectoryEntry(Directory {
      name: name.to_owned(),
      entries: vec![],
    })
  }

  fn print_line_with_prefix(&self, prefix: &str) {
    match self {
      Entry::FileEntry(f) => f.print_line_with_prefix(prefix),
      Entry::DirectoryEntry(d) => d.print_line_with_prefix(prefix),
    }
  }

  pub fn get_name(&self) -> &str {
    match self {
      Entry::FileEntry(f) => f.get_name(),
      Entry::DirectoryEntry(d) => d.get_name(),
    }
  }

  pub fn get_size(&self) -> usize {
    match self {
      Entry::FileEntry(f) => f.get_size(),
      Entry::DirectoryEntry(d) => d.get_size(),
    }
  }

  pub fn print_line(&self) {
    self.print_line_with_prefix("");
  }

  pub fn as_file(&self) -> Option<&File> {
    match self {
      Entry::FileEntry(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_directory(&self) -> Option<&Directory> {
    match self {
      Entry::DirectoryEntry(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_directory_mut(&mut self) -> Option<&mut Directory> {
    match self {
      Entry::DirectoryEntry(d) => Some(d),
      _ => None,
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

    rootdir.as_directory_mut().unwrap().add(bindir.clone());
    rootdir.as_directory_mut().unwrap().add(tmpdir.clone());
    rootdir.as_directory_mut().unwrap().add(usrdir.clone());

    {
      let mut bindir_ref = (&*bindir).borrow_mut();
      bindir_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("vi", 10000))));
      bindir_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("latex", 20000))));
      // bindir_ref.print_line();
    }

    let mut yuki = Rc::new(RefCell::new(Entry::of_directory("yuki")));
    let mut hanako = Rc::new(RefCell::new(Entry::of_directory("hanako")));
    let mut tomura = Rc::new(RefCell::new(Entry::of_directory("tomura")));

    {
      let mut usrdir_ref = (&*usrdir).borrow_mut();
      usrdir_ref.as_directory_mut().unwrap().add(yuki.clone());
      usrdir_ref.as_directory_mut().unwrap().add(hanako.clone());
      usrdir_ref.as_directory_mut().unwrap().add(tomura.clone());
      // usrdir_ref.print_line();
    }

    {
      let mut yuki_ref = (&*yuki).borrow_mut();
      yuki_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("diary.html", 100))));
      yuki_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("Composite.java", 200))));
      // yuki_ref.print_line();
    }

    {
      let mut hanako_ref = (&*hanako).borrow_mut();
      hanako_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("memo.tex", 300))));
      // hanako_ref.print_line();
    }

    {
      let mut tomura_ref = (&*tomura).borrow_mut();
      tomura_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("game.doc", 400))));
      tomura_ref
        .as_directory_mut()
        .unwrap()
        .add(Rc::new(RefCell::new(Entry::of_file("junk.mail", 500))));
      // tomura_ref.print_line();
    }

    rootdir.print_line();
  }
}
