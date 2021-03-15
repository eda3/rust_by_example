use crate::List::*;

#[derive(Debug)]
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

  // リストを受け取り、その始端に新しい要素を付加したものを返すメソッド
  fn prepend(self, elem: u32) -> List {
    // この`Cons`自体も、その第二要素もどちらもList型
    Cons(elem, Box::new(self))
  }

  // list の長さを返すメソッド
  fn len(&self) -> u32 {
    // このメソッドはselfの状態に寄って振る舞いが変化するためmatchする必要がある
    // selfの型は&Listであるので、*selfはListになる。マッチングは
    // 参照(&T)ではなく実体(T)に対して行うのが好ましい
    match *self {
      // selfをすでに借用しているので、tailの所有権を取ることができない
      // 代わりに参照を使用する
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }
}

fn main() {
  // 空の連結リストを作成
  let mut list = List::new();

  // 要素を追加
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);
  println!("{:?}", list);
  // -> Cons(3, Cons(2, Cons(1, Nil)))

  // 追加後の状態を表示
  println!("list.len: {}", list.len());
}
