use crate::List::*;

enum List {
  // 要素をラップして、次の要素へのポインタを保持するタプル
  Cons(u32, Box<List>),
  Nil,
}

fn main() {}
