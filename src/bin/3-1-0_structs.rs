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
  let point: Point = Point { x: 10.3, y: 0.4 };

  // pointのフィールドにアクセス
  println!("point.x:{}, point.y:{}", point.x, point.y);
  // -> point.x:10.3, point.y:0.4

  // `..point` といった構文を使うことで他インスタンスの設定値を流用できる
  let bottom_right = Point { x: 5.2, ..point };
  println!(
    "bottom_right.x:{}, bottom_right.y:{}",
    bottom_right.x, bottom_right.y
  );
  // -> bottom_right.x:10.3, bottom_right.y:0.4

  // letを使ってpointをデストラクトする
  let Point {
    x: top_edge,
    y: left_edge,
  } = point;

  // 構造体の定義とインスタンスの作成を同時に行う
  let _rectangle = Rectangle {
    top_left: Point {
      x: left_edge,
      y: top_edge,
    },
    bottom_right,
  };

  // ユニット構造体をインスタンス化
  let _nil = Nil;

  // タプル構造体をインスタンス化
  let pair = Pair(1, 0.1);

  // タプル構造体のフィールドにアクセス
  println!("pairは{}と{}を含む", pair.0, pair.1);
  // -> pairは1と0.1を含む

  // タプルをデストラクト
  let Pair(integer, decimal) = pair;
  println!("pairは{}と{}を含む", integer, decimal);
  // -> pairは1と0.1を含む
}
