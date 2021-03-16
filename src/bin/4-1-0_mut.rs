fn main() {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;

  println!("変更前:{}", mutable_binding);
  // -> 変更前:1

  // OK
  mutable_binding += 1;

  println!("変化後:{}", mutable_binding);
  // -> 変化後:2

  // Error
  // _immutable_binding += 1;
  //
  // 2  |   let _immutable_binding = 1;
  //    |       ------------------
  //    |       |
  //    |       first assignment to `_immutable_binding`
  //    |       help: make this binding mutable: `mut _immutable_binding`
  // ...
  // 15 |   _immutable_binding += 1;
  //    |   ^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
}
