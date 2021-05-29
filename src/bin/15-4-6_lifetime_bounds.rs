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

// Debugを実装しているTへの参照を取る。Tへの参照は
// 必ず'aよりも長生きでなくてはならない。さらに、'aは
// 関数自体よりも長生きでなくてはならない
fn print_ref<'a, T>(t: &'a T)
where
  T: Debug + 'a,
{
  println!("print_ref(): t is {:?}", t);
}

fn main() {
  let x = 7;
  let ref_x = Ref(&x);

  print_ref(&ref_x);
  // -> print_ref(): t is Ref(7)

  print(ref_x);
  // -> print_ref(): t is Ref(7)
}
