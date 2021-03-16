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
}
