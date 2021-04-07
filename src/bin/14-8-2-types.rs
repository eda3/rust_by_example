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

// A、Bを関連型にしているため、記載する必要がない
fn difference<C: Contains>(container: &C) -> i32 {
  container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!(
    "containerは{}と{}を保持しています:{}",
    &number_1,
    &number_2,
    container.contains(&number_1, &number_2)
  );
  // -> containerは3と10を保持しています:true

  println!("最初の数字：{}", container.first());
  // -> 最初の数字：3

  println!("最後の数字:{}", container.last());
  // -> 最後の数字:10

  println!("数字の差:{}", difference(&container));
  // -> 数字の差:7
}
