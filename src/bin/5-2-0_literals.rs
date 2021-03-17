fn main() {
  // サフィックスを指定したリテラル。型は初期値とともに確定
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  // サフィックスを指定しないリテラル。型は使用方法に依存
  let i = 1;
  let f = 1.0;

  // std::mem::size_of_val : 変数のサイズをバイトで返す
  println!("x(1u8)のバイト数: {}", std::mem::size_of_val(&x));
  println!("y(2u32)のバイト数: {}", std::mem::size_of_val(&y));
  println!("z(3f32)のバイト数: {}", std::mem::size_of_val(&z));
  println!("i(i32)のバイト数: {}", std::mem::size_of_val(&i));
  println!("f(f64)のバイト数: {}", std::mem::size_of_val(&f));
  // -> x(1u8)のバイト数: 1
  // -> y(2u32)のバイト数: 4
  // -> z(3f32)のバイト数: 4
  // -> i(i32)のバイト数: 4
  // -> f(f64)のバイト数: 8
}
