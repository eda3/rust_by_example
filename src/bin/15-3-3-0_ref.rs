#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let c = 'Q';

  // 左辺に ref をつけることによる借用と & をつけることによる借用は等価
  let ref ref_c1 = c;
  let ref_c2 = &c;

  println!("*ref_c1 == *ref_c2: {}", *ref_c1 == *ref_c2);

  let point = Point { x: 0, y: 1 };
  // ref は構造体をデストラクトする際にも有用
  let _copy_of_x = {
    // ref_to_x は pointのxフィールドへの参照
    let Point {
      x: ref ref_to_x,
      y: _,
    } = point;

    // pointのxフィールドへのコピーを返す
    *ref_to_x
  };
  println!("_copy_of_x: {}", _copy_of_x);
  // -> _copy_of_x: 0

  // pointへのミュータブルなコピー
  let mut mutable_point = point;

  {
    // ref は　mutとともに使い、ミュータブルな参照を取ることもできる
    let Point {
      x: _,
      y: ref mut mut_ref_to_y,
    } = mutable_point;

    // mutable_pointのyというミュータブルなフィールドの値を変更する
    *mut_ref_to_y = 2;
  }

  println!("point:({}, {})", point.x, point.y);
  // -> point:(0, 1)

  println!("mutable_point:({}, {})", mutable_point.x, mutable_point.y);
  // -> mutable_point:(0, 2)

  // ポインタを含むミュータブルなタプル
  let mut mutable_tuple = (Box::new(5u32), 3u32);

  {
    // mutable_tupleをデストラクトして、lastの値を変更
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;
  }
  // -> mutable_tuple: (5, 2)

  println!("mutable_tuple: {:?}", mutable_tuple);
}
