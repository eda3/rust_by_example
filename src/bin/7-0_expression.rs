fn main() {
  // 変数束縛
  let x = 5;

  // 式
  x;
  x + 1;
  15;

  let x = 5u32;

  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // この式はyに代入されます
    x_cube + x_squared + x
  };

  let z = {
    // セミコロンがあるのでzには()が入ります
    2 * x;
  };

  println!("x is {:?}", x);
  // x is 5
  println!("y is {:?}", y);
  // y is 155
  println!("z is {:?}", z);
  // z is ()
}
