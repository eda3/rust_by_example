use std::fmt;

// 2つの数字を扱うための構造体。
#[derive(Debug)]
struct MinMax(i64, i64);

// {} でprintデバッグするために、 std::fmt::Displayを実装する
impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

// 比較のために、フィールドに名前をつけられるような構造体を定義
#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

// 構造体Point2Dについて std::fmt::Displayを実装する
impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

fn main() {
  let minmax = MinMax(0, 14);

  // std::fmt::Display とstd::fmt::Debug を比較
  println!("Display:{}", minmax);
  println!("Debug  :{:?}", minmax);
  println!();
  // -> 表示は以下のとおり
  // Display:(0, 14)
  // Debug  :MinMax(0, 14)

  // 名前引数を使った表示を確認
  let small_range = MinMax(-3, 3);
  let big_range = MinMax(-300, 300);
  println!(
    "small={small} big={big}",
    small = small_range,
    big = big_range
  );
  println!();
  // -> small=(-3, 3) big=(-300, 300)

  let point = Point2D { x: 3.3, y: 7.2 };
  println!("point変数の表示を比較");
  println!("Display:{}", point);
  println!("Debug  :{:?}", point);
  // -> 表示は以下のとおり
  // Display:x: 3.3, y: 7.2
  // Debug  :Point2D { x: 3.3, y: 7.2 }
}
