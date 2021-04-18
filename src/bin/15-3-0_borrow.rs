// この関数はBoxの所有権を奪い、破壊する
fn eat_box_i32(boxed_i32: &i32) {
  println!("Boxを破壊。Boxの内容は{}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("i32型の値: {}", borrowed_i32);
}

fn main() {}
