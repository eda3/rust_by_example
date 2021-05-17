fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("xは{}、そしてyは{}", x, y);
}

// 引数を取らないがライフタイムパラメータ 'a を持つ関数
fn failed_borrow<'a>() {
  let _x = 12;
}

fn main() {
  // 下で借用するための変数を作成
  let (four, nine) = (4, 9);

  // 2つの変数の借用(&)が関数に渡される
  print_refs(&four, &nine);
  // -> xは4、そしてyは9
  // 借用された変数の寿命は借り手のそれよりも長くなくてはならない
  // つまり、 four、nineのライフタイムはprint_refsのそれよりも
  // 長くなくてはならない

  failed_borrow();
  // `failed_borrow`は関数のライフタイムよりも`'a`を長くさせるような
  // 参照を持たないが、それでも`'a`のほうが長くなる。なぜならそのような
  // 場合`'a`はデフォルトで`'static`になるからである。
}
