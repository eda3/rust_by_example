fn main() {
  // 型指定して変数束縛
  let logical: bool = true;

  // 通常の型指定
  let a_float: f64 = 1.0;

  //サフィックスによる型指定
  let an_integer = 5i32;

  // サフィックスを指定しない場合、型推論よりデフォルトが選択される
  let default_float = 3.0; // f64
  let default = integer = 7; // i32

  // mutをつけることで変数をミュータブルにすることができる
  let mut mutable = 12;

  // エラー！ミュータブルな変数でも型はかえられない
  // mutable = true;

  // シャドーイングにより変数を使えるようにする
  let mutable = true;
}
