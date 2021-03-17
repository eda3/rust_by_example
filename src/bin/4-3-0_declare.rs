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
}
