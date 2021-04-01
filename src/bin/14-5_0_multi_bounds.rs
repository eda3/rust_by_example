use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: {:?}", t);
  println!("Display: {}", t);
}
fn main() {}
