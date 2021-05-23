// 引数として`'a`のライフタイムで参照を一つ取る。最低でもこの関数
// と同じだけの長さでなくてはならない。
fn print_one<'a>(x: &'a i32) {
  println!("print_one: x is {}", x);
}

// ミュータブルな参照でも同様
fn add_one<'a>(x: &'a mut i32) {
  *x += 1;
}

fn main() {}
