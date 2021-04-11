fn create_box() {
  // 整数をヒープ上に確保
  let _box1 = Box::new(3i32);

  // `_box1`はここで破棄され、メモリは解放される。
}

fn main() {}
