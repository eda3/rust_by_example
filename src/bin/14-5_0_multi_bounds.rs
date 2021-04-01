use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: {:?}", t);
  println!("Display: {}", t);
}

fn compare_type<T: Debug, U:Debug>(t: &T, u: &U){
  println!("t: {:?}", t);
  println!("u: {:?}", u);
}
fn main() {}
