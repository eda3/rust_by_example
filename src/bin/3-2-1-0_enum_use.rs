#![allow(dead_code)]

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn main() {
  // `use` することで絶対名でなくとも使用可能になる。
  use crate::Status::{Poor, Rich};
  // `Work` の中の名前を全て`use`する
  use crate::Work::*;

  // `use` しているため、`Status::Poor`と同じ
  let status = Poor;
  // `Work::Civilian`と等しい
  let work = Civilian;

  match status {
    // `use`しているのでスコープを明示していない
    Rich => println!("お金持ちはたくさんお金を持っている"),
    Poor => println!("貧乏人はお金を持っていない"),
  }
  // -> 貧乏人はお金を持っていない

  match work {
    Civilian => println!("市民は働く！"),
    Soldier => println!("兵士は戦う！"),
  }
  // -> 市民は働く！
}
