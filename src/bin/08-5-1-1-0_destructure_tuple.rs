fn main() {
  let pair = (0, -2);

  println!("pair: {:?}", pair);
  // -> pair: (0, -2)

  match pair {
    (0, y) => println!("1つ目が0で、yは{:?}", y),
    (x, 0) => println!("2つ目が0で、xは{:?}", x),
    _ => println!("タプルに0が含まれていないパターン"),
  }
  // -> 1つ目が0で、yは-2
}
