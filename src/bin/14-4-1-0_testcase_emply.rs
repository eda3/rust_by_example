struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を設けているが、そのトレイトが空である
// か否かとは関係がない。
fn red<T: Red>(_: &T) -> &'static str {
  "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
  "blue"
}

fn main() {}
