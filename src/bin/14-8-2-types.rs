struct Container(i32, i32);

// 2つの要素がコンテナ型の中に保持されていることを確認するトレイト。
// また、最初あるいは最後の要素を取り出すこともできる。
trait Contains {
  // メソッドが使用できるジェネリック型を定義
  type A;
  type B;

  fn contains(&self, _: Self::A, _: &Self::B) -> bool;
  fn first(&self) -> i32;
  fn last(&self) -> i32;

}

fn main() {}
