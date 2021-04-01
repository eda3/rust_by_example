use std::fmt::Display;

// `Display`トレイトを実装している`T`を引数として取る
// `printer`という関数を定義。
fn printer<T: Display>(t: T) {
  println!("{}", t);
}

fn main() {}
