use std::fmt;

// Vec を含む Listという名前の構造体を定義
struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let vec = &self.0;

    // 開始用のカッコを表示
    write!(f, "[")?;

    for (count, v) in vec.iter().enumerate() {
      // 最初の要素以外はカンマを追加する
      // `count != 1`の書き方の場合以下のエラーが発生する
      // no implementation for `&i32 == {integer}`
      if count != 0 {
        write!(f, ",")?;
      }
      // 要素番号と要素内情報を記載
      write!(f, "{}: {}", count, v)?
    }
    // ここでfmt::Resultの値を返す
    write!(f, "]")
  }
}

fn main() {
  let v = List(vec![1, 2, 3]);
  println!("{}", v);
  // -> [0: 1,1: 2,2: 3]
}
