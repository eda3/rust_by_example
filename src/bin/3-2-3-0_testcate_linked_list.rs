use crate::List::*;

enum List {
  // 要素をラップして、次の要素へのポインタを保持するタプル
  Cons(u32, Box<List>),
  Nil,
}

// 列挙型にはメソッドを付与することができる
impl List {
  // 空リストの作成
  fn new() -> List {
    // `Nil` は `List`型を持つ
    Nil
  }
}

fn main() {}
