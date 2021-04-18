// この関数はBoxの所有権を奪い、破棄する
fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("Boxを破棄。Boxの内容は{}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("i32型の値: {}", borrowed_i32);
}

fn main() {
  // Box化された整数を作成
  let boxed_i32 = Box::new(5_i32);
  let stacked_i32 = 6_i32;

  // Boxの中身を借用。所有権を奪うわけではないため、
  // 直後にもう一度借用できる
  borrow_i32(&boxed_i32);
  // -> i32型の値: 5

  borrow_i32(&stacked_i32);
  // -> i32型の値: 6

  {
    // Box内の要素に対する参照を取得
    let _ref_to_i32: &i32 = &boxed_i32;

    // eat_box_i32(boxed_i32);

    // エラー！
    // Box内の要素が借用されるため、boxed_i32を破棄することはできない
    // エラーメッセージ
    // |
    // |     let _ref_to_i32: &i32 = &boxed_i32;
    // |                             ---------- borrow of `boxed_i32` occurs here
    // |
    // |     eat_box_i32(boxed_i32);
    // |                 ^^^^^^^^^ move out of `boxed_i32` occurs here
    // |
    // |     borrow_i32(_ref_to_i32);
    // |                ----------- borrow later used here

    borrow_i32(_ref_to_i32);
    // -> i32型の値: 5
  }
  // ここでeat_boxは所有権を移譲し、破棄することができる
  eat_box_i32(boxed_i32);
  // -> Boxを破棄。Boxの内容は5
}
