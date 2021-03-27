// タブルを関数の引数及び返り値として利用している
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // let を使ってタプルの中の値を別の変数に束縛することができる
  let (integer, boolean) = pair;

  (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  let long_tuple = (
    1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, 4i64, -0.1f32, 0.2f64, "a", true,
  );

  // インデックスを用いてタプル内の要素を参照できる
  println!("long_tupleの1番目の要素: {}", long_tuple.0);
  println!("long_tupleの2番目の要素: {}", long_tuple.1);
  // -> long_tupleの1番目の要素: 1
  // -> long_tupleの2番目の要素: 2

  // タプルはタプルのメンバになる
  let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

  // タプルはプリント可能
  println!("タプルの中のタプル: {:?}", tuple_of_tuples);
  // -> タプルの中のタプル: ((1, 2, 3), (4, -1), -2)

  // しかし、以下のtoo_long_tupleはプリント不可
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too_long_tuple: {:?}", too_long_tuple);
  //
  // -> 以下のエラーメッセージが表示される
  // 27 |   println!("too_long_tuple: {:?}", too_long_tuple);
  //    |                                    ^^^^^^^^^^^^^^ `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})` cannot be form
  // atted using `{:?}` because it doesn't implement `Debug`
  //    |
  //    = help: the trait `Debug` is not implemented for `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})`
  //    = note: required by `std::fmt::Debug::fmt`
  //    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

  let pair = (1, true);
  println!("pair: {:?}", pair);
  println!("reverse(pair): {:?}", reverse(pair));
  // -> pair: (1, true)
  // -> reverse(pair): (true, 1)

  // 要素を1つしか持たないタプルを作成する場合、()で囲まれたただのリテラルと区別するためにカンマが必要になる
  println!("要素が1つのみのタプル: {:?}", (5u32,));
  println!("ただのInteger: {:?}", (5u32));
  // -> 要素が1つのみのタプル: (5,)
  // -> ただのInteger: 5

  // タプルを分解して別の変数にそれぞれの値を代入
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?},{:?},{:?},{:?}", a, b, c, d);
  // -> 1,"hello",4.5,true

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);
  // -> Matrix(1.1, 1.2, 2.1, 2.2)
}
