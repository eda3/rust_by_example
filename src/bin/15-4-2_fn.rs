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

fn main() {}
