#![allow(dead_code)]

// 値を明示しない場合、0から整数が入る
enum Number {
  Three,
  Two,
  One,
}

// 値を明示する場合
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn main() {
  // 列挙型の中身を整数としてキャスト
  println!("Three: {}", Number::Three as i32);
  println!("Two: {}", Number::Two as i32);
  // -> Three: 0
  // -> Two: 1

  println!("薔薇は#{:06x}", Color::Red as i32);
  println!("スミレは#{:06x}", Color::Blue as i32);
  // -> 薔薇は#ff0000
  // -> スミレは#0000ff
}
