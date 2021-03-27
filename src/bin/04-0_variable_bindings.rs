fn main() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  // an_integerくぉcopied_integerへとコピー
  let copied_integer = an_integer;
  println!("an_integer: {:?}", copied_integer);
  println!("a_boolean: {:?}", a_boolean);
  println!("unit: {:?}", unit);
  // -> an_integer: 1
  // -> a_boolean: true
  // -> unit: ()

  // 使用されていない変数があるとコンパイラは警告を出します
  // 変数名の頭に`_`(アンダーバー)をつけると警告を消すことができます
  let _unused_variable = 3u32;

  let noisy_unused_variable = 2u32;
  // 以下の警告が出力される
  // --> src\bin\4-0_variable_bindings.rs:19:7
  //      |
  //   19 |   let noisy_unused_variable = 2u32;
  //      |       ^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_noisy_unused_variable`
  //      |
  // = note: `#[warn(unused_variables)]` on by default
}
