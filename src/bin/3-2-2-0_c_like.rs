#![allow(dead_code)]

// 値を明示しない場合、0から整数が入る
enum Number {
  Zero,
  One,
  Two,
}

// 値を明示する場合
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn main() {}
