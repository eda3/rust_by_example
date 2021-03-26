// 奇数かどうかを判定する関数
fn is_odd(n: u32) -> bool {
  n % 2 == 1
}
fn main() {
  println!("1000以下の奇数を二乗した値の合計を求める");
  let upper = 1000;

  // 宣言型プログラミングによるアプローチ
  // 値を蓄積する変数を宣言
  let mut acc = 0;
  // 0から無限までイテレート
  for n in 0.. {
    let n_squared = n * n;
    if upper <= n_squared {
      // 上限に達した場合ループ終了
      break;
    } else if is_odd(n_squared) {
      // 奇数ならば値を値に立ち合わせていく
      acc += n_squared;
    }
  }
  println!("宣言型スタイル： {}", acc);
  // -> 宣言型スタイル： 5456
}
