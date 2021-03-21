fn main() {
  use std::mem;
  let color = "green";

  // 変数colorをprintするためのクロージャ
  // これはcolorを借用(&)し、その借用とクロージャをprintという名の変数に保持する
  // 借用はprintがスコープから出るまで続く
  let print = || println!("color: {}", color);

  // 借用を行ったクロージャをコールする
  print();
  // -> color: green

  let _reborrow = &color;
  print();
  // -> color: green

  // `color` can be borrowed immutably again, because the closure only holds an immutable reference
  // to `color`.
  // color は再び不変的に借用することができます。なぜなら、クロージャは`colorへの不変的な参照のみを保持
  // するからです。
  let _color_moved = color;
  print();
  // -> color: green
}
