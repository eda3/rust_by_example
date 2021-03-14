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

// 型エイリアス
// 型エイリアスを使用するとenumの型名を別名で参照することができる
enum VeriVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

// 型エイリアスの作成
type Operations = VeriVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // to_owned()は&strをStringに変換する
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 80 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);
  // -> pressed x
  // -> pasted my text
  // -> clicked at x=20, y=80
  // -> page loaded
  // -> page unloaded

  // 型エイリアス
  let x = Operations::Add;
}
