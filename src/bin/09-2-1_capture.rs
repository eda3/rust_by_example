fn main() {
  use std::mem;
  let color = "green";

  // 変数colorをprintするためのクロージャ
  // これはcolorを借用(&)し、その借用とクロージャをprintという名の変数に保持する
  // 借用はprintがスコープから出るまで続く
  let print = || println!("color: {}", color);

  // 借用を行ったクロージャをコールする
  print();
  // -> color: green

  let _reborrow = &color;
  print();
  // -> color: green

  // `color` can be borrowed immutably again, because the closure only holds an immutable reference
  // to `color`.
  // color は再び不変的に借用することができます。なぜなら、クロージャは`colorへの不変的な参照のみを保持
  // するからです。
  let _color_moved = color;
  print();
  // -> color: green

  let mut count = 0;

  // countをインクリメントするためのクロージャ。countと&mut countの両方をとることができるが、後者のほうが制限が
  // 少ないため、そちらをとる。直後にcountを借用する
  let mut inc = || {
    count += 1;
    println!("count: {}", count);
  };

  // クロージャを実行
  inc();
  // -> count: 1

  // 再度借用を行おうとするとエラーになる
  // 理由はクロージャがあとで呼ばれるため
  // let _reborrow = &count;

  // コンパイル絵エラーメッセージは以下のとおり
  //   |   let mut inc = || {
  //
  //   |                 -- mutable borrow occurs here
  //   |     count += 1;
  //   |     ----- first borrow occurs due to use of `count` in closure
  //     ...
  //   |   let _reborrow = &count;
  //   |                   ^^^^^^ immutable borrow occurs here
  //   |
  //   |   inc();
  //   |   --- mutable borrow later used here

  inc();
  // -> count: 2

  let movable = Box::new(3);
  // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
  // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。

  let consume = || {
    println!("movable: {:?}", movable);
    mem::drop(movable);
  };

  // consumeは変数を消費(開放)するため、一度しか呼び出すことができない
  consume();
  // -> movable: 3

  // もう一度実行しようとするとエラーになる
  // consume();
  // |
  // |   consume();
  // |   --------- `consume` moved due to this call
  //   ...
  // |   consume();
  // |   ^^^^^^^ value used here after move
}
