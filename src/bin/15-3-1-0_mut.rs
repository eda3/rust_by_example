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

fn main() {
  // immutabook という名のイミュータブルなBookを作成
  let immutabook = Book {
    author: "夏目漱石",
    title: "坊っちゃん",
    year: 1906,
  };

  // immutabook のミュータブルなコピーを作成し、mutabookと名付ける
  let mut mutabook = immutabook;

  // イミュータブルなオブジェクトをイミュータブルに借用する
  borrow_book(&immutabook);
  // -> I immutably borrowed 坊っちゃん - 1906 edition

  // ミュータブルなオブジェクトをイミュータブルに借用する
  borrow_book(&mutabook);
  // -> I immutably borrowed 坊っちゃん - 1906 edition

  // ミュータブルなオブジェクトをミュータブルに借用する
  new_edition(&mut mutabook);
  // -> I mutably borrowed 坊っちゃん - 2014 edition

  // new_edition(&mut immutabook);
  //
  // エラー発生！！
  // error[E0596]: cannot borrow `immutabook` as mutable, as it is not declared as mutable
  //   --> src\bin\15-3-1-0_mut.rs:47:15
  //   |
  //   |   let immutabook = Book {
  //   |       ---------- help: consider changing this to be mutable: `mut immutabook`
  //   ...
  //   |   new_edition(&mut immutabook);
  //   |               ^^^^^^^^^^^^^^^ cannot borrow as mutable
}
