fn main() {
  // 変数宣言
  let a_binding;

  {
    let x = 2;

    // 変数を初期化
    a_binding = x * x;
  }

  println!("a_binding: {}", a_binding);
  // -> a_binding: 4

  let another_binding: u8;
  // Error! 初期化していない変数の使用
  // println!("another_binding: {}", another_binding);
  // |
  // |   println!("another_binding: {}", another_binding);
  // |                                   ^^^^^^^^^^^^^^^ use of possibly-uninitialized `another_binding`

  another_binding = 1;
  println!("another_binding: {}", another_binding);
  // -> another_binding: 1
}
