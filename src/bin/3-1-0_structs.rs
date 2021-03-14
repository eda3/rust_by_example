#[derive(Debug)]
struct Person<'a> {
  // 'a はライフタイムの設定
  name: &'a str,
  age: u8,
}

// ユニット
struct Nil;

fn main() {}
