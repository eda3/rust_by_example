#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let c = 'Q';

  // 左辺に ref をつけることによる借用と & をつけることによる借用は等価
  let ref ref_c1 = c;
  let ref_c2 = &c;

  println!("*ref_c1 == *ref_c2: {}", *ref_c1 == *ref_c2);
}
