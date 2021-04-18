#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  // &'static str はread-onlyメモリ上の文字列への参照
  author: &'static str,
  title: &'static str,
  year: u32,
}

// この関数はBook型への参照をとる
fn borrow_book(book: &Book) {
  println!(
    "I immutably borrowed {} - {} edition",
    book.title, book.year
  );
}

fn main() {}
