struct Point {
  x: i32,
  y: i32,
  z: i32,
}

fn main() {
  let mut point = Point { x: 0, y: 0, z: 0 };

  let borrowed_point = &point;
  let another_borrow = &point;

  // データはもともとの持ち主と参照の両方からアクセスすることが出来ます。
  println!(
    "Pointの各値:({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z
  );
  // -> Pointの各値:(0, 0, 0)

  // let mutable_borrow = &mut point;
  //
  // 上記はエラーが発生する！
  // error[E0502]: cannot borrow `point` as mutable
  //               because it is also borrowed as immutable
  // |
  // |   let borrowed_point = &point;
  // |                        ------ immutable borrow occurs here
  // ...
  // |   let mutable_borrow = &mut point;
  // |                        ^^^^^^^^^^ mutable borrow occurs here
  // ...
  // |     borrowed_point.x, another_borrow.y, point.z
  // |     ---------------- immutable borrow later used here

  println!(
    "Pointの各値:({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z
  );
  // -> Pointの各値:(0, 0, 0)

  // イミュータブルな参照は、残りのコードでは使用されなくなるので、ミュータブルな参照で再利用することが可能です。
  let mutable_borrow = &mut point;

  // ミュータブルなリファレンスを介してデータを変更
  mutable_borrow.x = 5;
  mutable_borrow.y = 2;
  mutable_borrow.z = 1;

  // let y = &point.y;
  //
  // 上記はエラーが発生する！
  // error[E0502]: cannot borrow `point.y` as immutable because it is also borrowed as mutable
  // |
  // |   let mutable_borrow = &mut point;
  // |                        ---------- mutable borrow occurs here
  // ...
  // |   let y = &point.y;
  // |           ^^^^^^^^ immutable borrow occurs here
  // ...
  // |     mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
  // |     ---------------- mutable borrow later used here

  // println!("Point Zの内容：{}", point.z);
  //
  // 上記はエラーが発生する！
  // println!はイミュータブルなリファレンスを取るためprintできない
  // error[E0502]: cannot borrow `point.z` as immutable because it is also borrowed as mutable
  // |
  // |   let mutable_borrow = &mut point;
  // |                        ---------- mutable borrow occurs here
  // ...
  // |   println!("Point Zの内容：{}", point.z);
  // |                                 ^^^^^^^ immutable borrow occurs here
  // ...
  // |     mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
  // |     ---------------- mutable borrow later used here

  // ミュータブル参照をイミュータブルとしてprintln!にわたすことはできる
  println!(
    "Pointの各値:({}, {}, {})",
    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
  );
  // -> Pointの各値:(5, 2, 1)

  // ミュータブルリファレンスは残りのコードでは使用されないので再利用することができます。
  let new_borrowed_point = &point;
  println!(
    "Pointの各値:({}, {}, {})",
    new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
  );
  // Pointの各値:(5, 2, 1)
}
