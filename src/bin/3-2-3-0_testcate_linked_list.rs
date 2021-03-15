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

  // リストを受け取り、その始端に新しい要素を付加したものを返す関数
  fn prepend(self, elem: u32) -> List {
    // この`Cons`自体も、その第二要素もどちらもList型
    Cons(elem, Box::new(self))
  }
}

fn main() {
  // 空の連結リストを作成
  let mut list = List::new();

  // 要素を追加
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);
}
