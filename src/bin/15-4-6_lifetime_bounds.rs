use std::fmt::Debug; // ライフタイムを紐付けるトレイト

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Refは'aというライフタイムを持つジェネリック型Tに対する参照を持ち、
// Tの値 *に対する参照* は必ず'aよりも長生きでなくてはならない。
// さらに、Refのライフタイムは'aを超えてはならない。

//  Debugトレイトを利用してprintを行うジェネリック関数
fn print<T>(t: T)
where
  T: Debug,
{
  println!("print(): t is {:?}", t);
}
fn main() {}
