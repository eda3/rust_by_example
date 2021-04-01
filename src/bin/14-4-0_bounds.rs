use std::fmt::Display;

// print時のマーカー{:?}を実装するtrait
use std::fmt::Debug;

// `Display`トレイトを実装している`T`を引数として取る
// `printer`という関数を定義。
fn printer<T: Display>(t: T) {
  println!("{}", t);
}

trait HashAreA {
  fn area(&self) -> f64;
}

struct Rectangle {
  length: f64,
  height: f64,
}

struct Triangle {
  length: f64,
  height: f64,
}

fn main() {}
