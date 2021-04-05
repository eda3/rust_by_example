struct Container(i32, i32);

// ２つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains<A, B> {
  fn contains(&self, _: &A, _: &B) -> bool; // `A`と`B`の両方を明示的に要求する

  fn first(&self) -> i32; // `A`、`B`いずれも要求しない

  fn last(&self) -> i32; // `A`、`B`いずれも要求しない
}

fn main() {}
