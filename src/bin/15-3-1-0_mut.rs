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

// この関数はミュータブルなBook型へのミュータブルなリファレンスを取り、
// `year`を2014へ変更する。
fn new_edition(book: &mut Book) {
  book.year = 2014;
  println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {}
