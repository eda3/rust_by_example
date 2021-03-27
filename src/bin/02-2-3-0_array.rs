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

  // 配列はスタックの上に置かれる
  println!("配列が占めるサイズは{}byte", mem::size_of_val(&xs));
  // -> 配列が占めるサイズは20byte

  // 配列は自動的にスライスとして借用される
  println!("スライスとして配列全体を借用する");
  analyze_slice(&xs);
  // スライスとして配列全体を借用する
  // -> sliceの第一要素: 1
  // -> sliceは5個の要素を持つ

  println!("配列の一部分をスライスして借用する");
  analyze_slice(&ys[1..4]);
  // 配列の一部分をスライスして借用する
  // sliceの第一要素: 0
  // sliceは3個の要素を持つ

  // インデックスの範囲が配列のサイズを超えた場合パニックが発生する
  // println!("{}", xs[5]);
  // ビルド時に以下のエラーが発生
  // --> src\bin\2-2-3-0_array.rs:42:18
  //      |
  //   42 |   println!("{}", xs[5]);
  //      |                  ^^^^^ index out of bounds: the length is 5 but the index is 5
  //      |
  //      = note: `#[deny(unconditional_panic)]` on by default
  //
  // error: aborting due to previous error
}
