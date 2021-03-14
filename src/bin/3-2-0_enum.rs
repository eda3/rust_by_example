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
fn main() {}
