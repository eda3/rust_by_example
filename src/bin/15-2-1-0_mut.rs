fn main() {
  let immutable_box = Box::new(5u32);

  println!("immutable_boxの内容: {}", immutable_box);
  // -> immutable_boxの内容: 5

  // boxをmoveする、同時に所有権とミュー旅リティを変更
  let mut mutable_box = immutable_box;
  println!("mutable_boxの内容: {}", mutable_box);
  // -> mutable_boxの内容: 5

  // boxの内容を変更
  *mutable_box = 4;
  println!("mutable_boxの内容: {}", mutable_box);
  // -> mutable_boxの内容: 4
}
