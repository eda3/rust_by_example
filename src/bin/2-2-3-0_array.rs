use std::mem;

fn analyze_slice(slice: &[i32]) {
  println!("sliceの第一要素: {}", slice[0]);
  println!("sliceは{}個の要素を持つ", slice.len());
}

fn main() {
  // 固定長の配列
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  // すべての要素を0にする場合
  let ys: [i32; 500] = [0; 500];

  // インデックスは0からスタート
  println!("配列の1つ目の要素: {}", xs[0]);
  println!("配列の2つ目の要素: {}", xs[1]);
  // -> 配列の1つ目の要素: 1
  // -> 配列の2つ目の要素: 2

  // `len()` は配列のサイズを返却する
  println!("array.len(): {}", xs.len());
  // -> array.len(): 5
}
