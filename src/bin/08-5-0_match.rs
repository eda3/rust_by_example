fn main() {
  let number = 13;
  println!("{:?}", number);
  // ->13

  match number {
    // 単一の値とのマッチをチェック
    1 => println!("One!"),
    // いくつかの値とのマッチをチェック
    2 | 3 | 5 | 7 | 11 => println!("素数!"),
    // 特定の範囲との値とのマッチをチェック
    13..=19 => println!("13から19の間"),
    // その他の場合
    _ => println!("特になし"),
  }
  // -> 13から19の間

  let boolean = true;
  // matchは式でもある
  let binary = match boolean {
    // マッチは全ての可能な値をカバーしなくてはならない
    false => 0,
    true => 1,
  };
  println!("{} -> {}", boolean, binary);
  // -> true -> 1
}
