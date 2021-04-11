// この関数はヒープメモリ上の資源の所有権を取る。
fn destroy_box(c: Box<i32>) {
  println!("cの内容をデストロイします {}", c);

  // `c`は破棄されメモリは開放される。
}
fn main() {
  // スタックに上に置かれた整数
  let x = 5u32;

  // xをyに コピー する。元の値が移動するわけではない。
  let y = x;

  // 両方の値はそれぞれ独立に使うことができる。
  println!("xは{}、yは{}", x, y);
  // -> xは5、yは5

  // aはヒープ上の整数へのポインタ
  let a = Box::new(5i32);

  println!("aの内容:{}", a);
  // -> aの内容:5

  // aをbに ムーブ する。
  let b = a;

  // エラー発生！！！
  // println!("aの内容:{}", a);
  // |   let a = Box::new(5i32);
  // |       - move occurs because `a` has type `Box<i32>`, which does not implement the `Copy` trait
  // ...
  // |   let b = a;
  // |           - value moved here
  // |
  // |   println!("aの内容:{}", a);
  // |                          ^ value borrowed here after move
}
