// この状態でprintデバッグするためには、fmt::Displayもしくはfmt::Debugを実装する必要がある
struct UnPrintable(i32); // -> printデバッグできない

// #[derive(Debug)]
// と記述することで、簡単にfmt::Debugを実装することができる
#[derive(Debug)]
struct DebugPrintable(i32); // -> printデバッグできる

// {:#?} を使用することで構造体の内容を全て表示することができる
#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

fn main() {
  // println!("{}", UnPrintable(42));
  // -> 以下の警告が表示される
  //    `UnPrintable` doesn't implement `Display` (required by {})
  //    fmt::Displayを実装していないため

  // println!("{:?}", UnPrintable(42));
  //    `UnPrintable` doesn't implement `Debug` (required by {:?})
  //     fmt::Debugを実装していないため

  // println!("{}", DebugPrintable(42));
  // -> 以下の警告が表示される
  //    `UnPrintable` doesn't implement `Display` (required by {})
  //    fmt::Displayを実装していないため

  println!("{:?}", DebugPrintable(42));
  // -> DebugPrintable(42)
  // #[derive(Debug)]を記述しているため正常にprintデバッグできる

  // {:#?} を使用することで構造体の内容を全て表示することができる
  let name = "太郎";
  let age = 27;
  let taro = Person { name, age };
  println!("{:#?}", taro);
  // -> 以下のとおり表示される
  //    Person {
  //      name: "太郎",
  //      age: 27,
  //    }
}
