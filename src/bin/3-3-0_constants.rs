static LANGUAGE: &str = " Rust";
const THRESHOLD: i32 = 10; // しきい値

fn is_big(n: i32) -> bool {
  // 関数内から定数を参照
  n > THRESHOLD
}
fn main() {}
