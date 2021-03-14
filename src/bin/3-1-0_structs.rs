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

fn main() {}
