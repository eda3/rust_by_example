#[derive(Debug)]
struct Person<'a> {
  // 'a はライフタイムの設定
  name: &'a str,
  age: u8,
}

// ユニット
struct Nil;

// タプル
struct Pair(i32, f32);

// 2つのフィールドを持つクラシックな構造体
struct Point {
  x: f32,
  y: f32,
}

// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  // struct Personをサクッと初期化
  let name = "Peter";
  let age = 27;
  let perter = Person { name, age };
  println!("{:?}", perter);
  // -> Person { name: "Peter", age: 27 }

  // struct Pointをインスタンス化
  let point: Point = Point { x: 10.3, y: 0.4};

  // pointのフィールドにアクセス
  println!("point.x:{}, point.y:{}", point.x, point.y);
  // -> point.x:10.3, point.y:0.4
}