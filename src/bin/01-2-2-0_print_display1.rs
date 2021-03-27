use std::fmt;

struct Structure(i32);

// {} でprintデバッグするために、 std::fmt::Displayを実装する
impl fmt::Display for Structure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

fn main() {
  // std::fmt::Displayを実装しているためエラー表示されない
  println!("{}", Structure(42));
}
