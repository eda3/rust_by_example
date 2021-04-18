#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  // &'static str はread-onlyメモリ上の文字列への参照
  author: &'static str,
  title: &'static str,
  year: u32,
}

fn main() {}
