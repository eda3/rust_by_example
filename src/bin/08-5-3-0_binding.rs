fn age() -> u32 {
  15
}

fn main() {
  println!("その人は何歳なのか？");

  match age() {
    0 => println!("まだ生まれていない"),
    n @ 1..=19 => println!("そいつは{}歳で少年", n),
    n @ 20..=100 => println!("そいつは{}歳で成年", n),
    n => println!("{}歳を超えているようだ", n),
  }
}
