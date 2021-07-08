// ここではRustはライフタイムを出来る限り短く見積もり、
// 2つの参照をそのライフタイムに押し込める。
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
  first * second
}

fn main() {}
