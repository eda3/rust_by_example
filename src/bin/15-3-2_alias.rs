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
}
