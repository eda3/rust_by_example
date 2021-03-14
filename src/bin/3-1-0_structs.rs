use std::fmt::Pointer;

#[derive(Debug)]
struct Person<'a> {
  // 'a はライフタイムの設定
  name: &'a str,
  age: u8,
}

// ユニット
struct Nil;

// タプル
struct Pair(i32, f32);

// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {}
