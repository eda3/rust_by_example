fn main() {
  // この変数はメイン関数内がスコープ領域
  let long_lived_binding = 1;

  // ここから下がmain()`より小さいスコープを持つブロック
  {
    // この変数はこのブロック内にのみ存在する
    let short_lived_binding = 2;

    // この変数はスコープ外の同名の変数を **シャドーイング**します
    let long_lived_binding = 5_f32;

    println!("long_lived_binding: {}", long_lived_binding);
    // -> long_lived_binding: 5
  }

  // Error! short_lived_bindingはこのスコープ内に存在しないためエラーとなる
  // println!("short_lived_binding: {}", short_lived_binding);
  // |
  // |   println!("short_lived_binding: {}", short_lived_binding);
  // |                                       ^^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `long_lived_binding`

  println!("long_lived_binding: {}", long_lived_binding);
  // -> long_lived_binding: 1

  // 以前に定義した変数を **シャドーイング** する
  let long_lived_binding = 'a';
  println!("long_lived_binding: {}", long_lived_binding);
  // -> long_lived_binding: a
}
