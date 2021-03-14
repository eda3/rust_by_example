enum WebEvent {
  // ユニット構造体ライク
  PageLoad,
  PageUnload,
  // タプル構造体ライク
  KeyPress(char),
  Paste(String),
  // C言語構造体ライク
  Click { x: i64, y: i64 },
}

// WebEvent列挙型を引数にとり、何も返却しない関数
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    WebEvent::KeyPress(c) => println!("pressed {}", c),
    WebEvent::Paste(s) => println!("pasted {}", s),
    WebEvent::Click { x, y } => {
      println!("clicked at x={}, y={}", x, y);
    }
  }
}

fn main() {}
