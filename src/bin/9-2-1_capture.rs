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
}
