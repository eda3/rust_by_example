// この関数はBoxの所有権を奪い、破壊する
fn eat_box_i32(boxed_i32: &i32) {
  println!("Boxを破壊。Boxの内容は{}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("i32型の値: {}", borrowed_i32);
}

fn main() {
  // Box化された整数を作成
  let boxed_i32 = Box::new(5i32);
  let stacked_i32 = 6i32;

  // Boxの中身を借用。所有権を奪うわけではないため、
  // 直後にもう一度借用できる
  borrow_i32(&boxed_i32);
  // -> i32型の値: 5

  borrow_i32(&stacked_i32);
  // -> i32型の値: 6
}
