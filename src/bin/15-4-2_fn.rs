// 引数として`'a`のライフタイムで参照を一つ取る。最低でもこの関数
// と同じだけの長さでなくてはならない。
fn print_one<'a>(x: &'a i32) {
  println!("print_one: x is {}", x);
}

// ミュータブルな参照でも同様
fn add_one<'a>(x: &'a mut i32) {
  *x += 1;
}

// 異なるライフタイムを持つ複数の引数がある場合。
// ここでは1種類のライフタイムでも問題はないが、より複雑なケースでは
// 異なるライフタイムが必要になる場合がある。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("print_multi: x is {}, y is {}", x, y);
}

// 受け取った参照をそのまま返すことに問題はないが、適切なライフタイム
// でなくてはならない。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
  x
}

// fn invalid_output<'a>() -> &'a String {
//   &String::from("foo")
// }
// `'a`は関数より長くなくてはならないため上の関数は正しくない。

// ここでは、`&7`は`i32`のデータとそれへの参照を作り出す。
// その後データはスコープを抜けるとともに破棄される。そのため、
// 不適切なデータに対する参照を返すことになってしまう。

fn main() {
  let x = 7;
  let y = 9;

  print_one(&x);
  // -> print_one: x is 7

  print_multi(&x, &y);
  // -> print_multi: x is 7, y is 9

  let z = pass_x(&x, &y);
  print_one(z);
  // -> print_one: x is 7

  let mut t = 3;
  add_one(&mut t);
  print_one(&t);
  // -> print_one: x is 4
}
