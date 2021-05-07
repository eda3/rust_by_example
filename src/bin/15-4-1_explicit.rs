fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("xは{}、そしてyは{}", x, y);
}

// 引数を取らないがライフタイムパラメータ 'a を持つ関数
fn failed_borrow<'a>() {
  let _x = 12;
}

fn main() {}
