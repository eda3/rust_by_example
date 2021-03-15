static LANGUAGE: &str = " Rust";
const THRESHOLD: i32 = 10; // しきい値

fn is_big(n: i32) -> bool {
  // 関数内から定数を参照
  n > THRESHOLD
}
fn main() {
  let n = 16;
  // main 関数の中から定数を参照
  println!("これは{}", LANGUAGE);
  println!("しきい値は{}", THRESHOLD);
  println!("{}は{}", n, if is_big(n) { "大きい" } else { "小さい" });
  // -> これは Rust
  // -> しきい値は10
  // -> 16は大きい

  // コンパイルエラー
  // THRESHOLD = 5;
  //
  // 18 |   THRESHOLD = 5;
  //    |   --------- ^
  //    |   |
  //    |   cannot assign to this expression
}
