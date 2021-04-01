use std::fmt::Display;

// print時のマーカー{:?}を実装するtrait
use std::fmt::Debug;

// `Display`トレイトを実装している`T`を引数として取る
// `printer`という関数を定義。
fn printer<T: Display>(t: T) {
  println!("{}", t);
}

trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Rectangle {
  fn area(&self) -> f64 {
    self.length * self.height
  }
}

#[derive(Debug)]
struct Rectangle {
  length: f64,
  height: f64,
}

#[derive(Debug)]
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

fn main() {
  let rectangle = Rectangle {
    length: 3.0,
    height: 4.0,
  };
  let _triangle = Triangle {
    length: 3.0,
    height: 4.0,
  };

  print_debug(&rectangle);
  // Rectangle { length: 3.0, height: 4.0 }

  println!("Area: {}", area(&rectangle));
  // -> Area: 12
}
