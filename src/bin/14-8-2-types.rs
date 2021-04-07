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

impl Contains for Container {
  // `A`と`B`がどの型であるかを明示。インプットの型（訳注: つまり`Self`の型）
  // が`Container(i32, i32)`である場合、出力型は`i32`と`i32`となる。
  type A = i32;
  type B = i32;

  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }
  // 1つ目の数を取得
  fn first(&self) -> i32 {
    self.0
  }

  // 最後の数を取得
  fn last(&self) -> i32 {
    self.1
  }
}

fn main() {}
