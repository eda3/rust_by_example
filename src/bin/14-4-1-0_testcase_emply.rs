struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を設けているが、そのトレイトが空である
// か否かとは関係がない。
fn red<T: Red>(_: &T) -> &'static str {
  "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
  "blue"
}

fn main() {
  let cardinal = Cardinal;
  let blue_jay = BlueJay;
  let _turkey = Turkey;

  // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
  // `blue`と`Cardinal`も同様、
  println!("cardinal: {}", red(&cardinal));
  // -> cardinal: red

  println!("blue_lay: {}", blue(&blue_jay));
  // -> blue_lay: blue

  // println!("_turkey: {}", red(&_turkey));
  // エラー発生する
  //| fn red<T: Red>(_: &T) -> &'static str {
  // |           --- required by this bound in `red`
  // ...
  // |   println!("_turkey: {}", red(&_turkey));
  // |                               ^^^^^^^^ the trait `Red` is not implemented for `Turkey`
}
