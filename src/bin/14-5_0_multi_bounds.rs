use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: {:?}", t);
  println!("Display: {}", t);
}

fn compare_type<T: Debug, U: Debug>(t: &T, u: &U) {
  println!("t: {:?}", t);
  println!("u: {:?}", u);
}
fn main() {
  let string = "word";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  // -> Debug: "word"
  // -> Display: word

  // compare_prints(&array);
  // 上をコメントアウトするとエラー発生
  //
  // error[E0277]: `[{integer}; 3]` doesn't implement `std::fmt::Display`
  // --> src\bin\14-5_0_multi_bounds.rs:21:18
  //   |
  //   | fn compare_prints<T: Debug + Display>(t: &T) {
  //   |                              ------- required by this bound in `compare_prints`
  //   ...
  //   |   compare_prints(&array);
  //   |                  ^^^^^^ `[{integer}; 3]` cannot be formatted with the default formatter

  compare_type(&array, &vec);
  // -> t: [1, 2, 3]
  // -> u: [1, 2, 3]
}
