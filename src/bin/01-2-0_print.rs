fn main() {
  // 一般的に {}  は引数を指定する
  // 指定された引数は文字列化する
  println!("{}日後", 31);
  // -> 31日後

  // {} の中に数値を指定することで任意の位置引数を表示することができる
  println!(
    "もしもし。{0}と申します。{1}様はいらっしゃいますでしょうか",
    "田中", "山田"
  );
  // -> もしもし。田中と申します。山田様はいらっしゃいますでしょうか

  // 以下のとおり名前引数も使用できる
  println!(
    "{one} {two} {three}",
    one = "五月雨を",
    two = "集めてはやし",
    three = "最上川",
  );
  // -> 五月雨を 集めてはやし 最上川

  // {:b} とすることで二進数で表示することができる
  println!("{:b} {:b} {:b} {:b}", 1, 2, 100, 1000);
  // -> 1 10 1100100 1111101000

  // {:>n} とすることで右寄せ表示をすることができる
  println!("{number:>width$}", number = 1, width = 6);
  println!("{:>6}", 1);
  // ->      1
  // ->      1
  // ※どちらも同じ表示

  // {:>0n} とすることで右寄せ表示をすることができる
  println!("{number:>0width$}", number = 1, width = 6);
  println!("{:>06}", 1);
  // -> 000001
  // -> 000001
  // ※どちらも同じ表示
}