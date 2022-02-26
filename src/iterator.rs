#[derive(Clone)]
pub struct Book {
  name: String,
}

impl Book {
  pub fn new(name: &str) -> Self {
    Self { name: name.to_owned() }
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}

pub struct BookShelf {
  values: Vec<Book>,
  last: usize,
}

impl BookShelf {
  pub fn new(capacity: usize) -> Self {
    Self {
      values: Vec::with_capacity(capacity),
      last: 0,
    }
  }

  pub fn with_elements(values: &[Book]) -> Self {
    Self {
      values: values.to_vec(),
      last: 0,
    }
  }

  pub fn get_book_at(&self, index: usize) -> &Book {
    &self.values[index]
  }

  pub fn append_book(&mut self, book: Book) {
    self.values.push(book);
    self.last += 1;
  }

  pub fn get_length(&self) -> usize {
    self.last
  }

  pub fn iter(&self) -> BookShelfIterator {
    BookShelfIterator::new(self)
  }
}

pub struct BookShelfIterator<'a> {
  book_shelf: &'a BookShelf,
  index: usize,
}

impl<'a> BookShelfIterator<'a> {
  pub fn new(book_shelf: &'a BookShelf) -> Self {
    Self { book_shelf, index: 0 }
  }
}

impl<'a> Iterator for BookShelfIterator<'a> {
  type Item = &'a Book;

  fn next(&mut self) -> Option<Self::Item> {
    match self.index < self.book_shelf.values.len() {
      true => {
        let t = Some(&self.book_shelf.values[self.index]);
        self.index += 1;
        t
      }
      false => None,
    }
  }
}

impl<'a> IntoIterator for &'a BookShelf {
  type IntoIter = BookShelfIterator<'a>;
  type Item = &'a Book;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut book_shelf = BookShelf::new(4);
    book_shelf.append_book(Book::new("Around the World in 80 Days"));
    book_shelf.append_book(Book::new("Bible"));
    book_shelf.append_book(Book::new("Cinderella"));
    book_shelf.append_book(Book::new("Daddy-Long-Legs"));

    // Iteratorの実装によって可能になる
    let mut it = book_shelf.iter();
    while let Some(book) = it.next() {
      println!("{}", book.name())
    }

    // IntoIteratorの実装によって可能になる
    for book in &book_shelf {
      println!("{}", book.name())
    }
  }
}
