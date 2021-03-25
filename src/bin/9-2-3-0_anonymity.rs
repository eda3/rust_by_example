// 関数内でクロージャを使う場合、ジェネリック型でなくてはならない
fn apply<F>(f: F)
where
  F: FnOnce(),
{
  f();
}
fn main() {}
