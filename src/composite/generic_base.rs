use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

trait EntryBase {
  fn print_line_with_prefix(&self, prefix: &str);
}

pub trait Entry: EntryBase + Display + Debug {
  fn get_name(&self) -> &str;
  fn get_size(&self) -> usize;
  fn print_line(&self) {
    self.print_line_with_prefix("");
  }
}

#[derive(Debug)]
pub struct File {
  name: String,
  size: usize,
}

impl File {
  pub fn new(name: &str, size: usize) -> Self {
    Self {
      name: name.to_owned(),
      size,
    }
  }
}

impl EntryBase for File {
  fn print_line_with_prefix(&self, prefix: &str) {
    println!("{}/{}", prefix, self)
  }
}

impl Display for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.size)
  }
}

impl Entry for File {
  fn get_name(&self) -> &str {
    &self.name
  }

  fn get_size(&self) -> usize {
    self.size
  }
}

#[derive(Debug)]
pub struct Directory<E: Entry> {
  name: String,
  entries: Vec<Rc<RefCell<E>>>,
}

impl<E: Entry> Directory<E> {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      entries: Vec::new(),
    }
  }

  pub fn add(&mut self, entry: Rc<RefCell<E>>) {
    self.entries.push(entry);
  }
}

impl<E: Entry> EntryBase for Directory<E> {
  fn print_line_with_prefix(&self, prefix: &str) {
    println!("{}/{}", prefix, self);
    for entry in &self.entries {
      let entry_ref = (&**entry).borrow();
      entry_ref.print_line_with_prefix(&format!("{}/{}", prefix, self.name))
    }
  }
}

impl<E: Entry> Display for Directory<E> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.get_size())
  }
}

impl<E: Entry> Entry for Directory<E> {
  fn get_name(&self) -> &str {
    &self.name
  }

  fn get_size(&self) -> usize {
    self.entries.iter().fold(0, |r, e| r + e.borrow().get_size())
  }
}

#[cfg(test)]
mod test {
  use std::borrow::BorrowMut;
  use std::cell::RefCell;

  use super::*;

  #[test]
  fn test() {
    let mut rootdir = Directory::new("root");
    let bindir = Rc::new(RefCell::new(Directory::new("bin")));
    let tmpdir = Rc::new(RefCell::new(Directory::new("tmp")));
    let usrdir = Rc::new(RefCell::new(Directory::new("usr")));

    rootdir.add(bindir.clone());
    rootdir.add(tmpdir.clone());
    rootdir.add(usrdir.clone());

    {
      let mut bindir_ref = (&*bindir).borrow_mut();
      bindir_ref.add(Rc::new(RefCell::new(File::new("vi", 10000))));
      bindir_ref.add(Rc::new(RefCell::new(File::new("latex", 20000))));
      // bindir_ref.print_line();
    }

    let mut yuki = Rc::new(RefCell::new(Directory::new("yuki")));
    let mut hanako = Rc::new(RefCell::new(Directory::new("hanako")));
    let mut tomura = Rc::new(RefCell::new(Directory::new("tomura")));

    {
      let mut usrdir_ref = (&*usrdir).borrow_mut();
      // userdirにはファイルしか追加できないが、ここでディレクトリを追加しようとしているので、このコードはコンパイルできない
      // usrdir_ref.add(yuki.clone());
      // usrdir_ref.add(hanako.clone());
      // usrdir_ref.add(tomura.clone());
      // usrdir_ref.print_line();
    }

    {
      let mut yuki_ref = (&*yuki).borrow_mut();
      yuki_ref.add(Rc::new(RefCell::new(File::new("diary.html", 100))));
      yuki_ref.add(Rc::new(RefCell::new(File::new("Composite.java", 200))));
      // yuki_ref.print_line();
    }

    {
      let mut hanako_ref = (&*hanako).borrow_mut();
      hanako_ref.add(Rc::new(RefCell::new(File::new("memo.tex", 300))));
      // hanako_ref.print_line();
    }

    {
      let mut tomura_ref = (&*tomura).borrow_mut();
      tomura_ref.add(Rc::new(RefCell::new(File::new("game.doc", 400))));
      tomura_ref.add(Rc::new(RefCell::new(File::new("junk.mail", 500))));
      // tomura_ref.print_line();
    }

    rootdir.print_line();
  }
}
