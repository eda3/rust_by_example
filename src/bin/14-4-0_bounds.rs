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

// ジェネリック型`T`は`Debug`トレイトを実装していなくてはならない。
// その限りにおいて、`T`がどのような具象型であろうとも次の関数は動作する。
fn print_debug<T: Debug>(t: T) {
  println!("{:?}", t);
}

// 「`T`は`HasArea`を実装していなくてはならない」という境界条件を
// 満たしていれば、`HasArea`の関数`area`にアクセスできる。
fn area<T: HasArea>(t: &T) -> f64 {
  t.area()
}

fn main() {}
