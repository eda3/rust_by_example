// オーバーフローを起こすようなキャスティングによる警告を無視
#![allow(overflowing_literals)]

fn main() {
  let decimal = 64.4321_f32;
  println!("decimal: {}", decimal);
  // -> decimal: 64.4321

  // let integer: u8 = decimal;
  // Error! 暗黙的な型変換はできない
  //
  // error[E0308]: mismatched types
  //   --> src\bin\5-1-0_cast.rs:9:21
  // |
  // |   let integer: u8 = decimal;
  // |                --   ^^^^^^^ expected `u8`, found `f32`
  // |                |
  // |                expected due to this
  //
  // error: aborting due to previous error

  // 明示的な型変換
  let integer = decimal as u8;
  let character = integer as char;
  println!(
    "キャスティング: {} -> {} -> {}",
    decimal, integer, character
  );
  // -> キャスティング: 64.4321 -> 64 -> @

  // ### キャスティングによる値の変化
  // 何らかの値を符号なしの型(T)へキャスティングすると、
  // 値がTに収まるまでstd::T::MAX + 1 が加算あるいは減算される

  // 1000はすでにu16の範囲に収まっているため変化しない
  println!("1000 as u16: {}", 1000 as u16);
  // -> 1000 as u16: 1000

  // 1000はu8の範囲に収まっていないため変化する
  // 1000 - 256 - 256 - 256 = 232
  println!("1000 as u8: {}", 1000 as u8);
  // -> 1000 as u8: 232

  // -1 + 256 = 255
  println!("-1i8 as u8: {}", (-1i8) as u8);
  // -> -1i8 as u8: 255

  // 符号付きの型にキャストする場合結果は以下の2つを行った場合に等しい
  // 1. 対応する符号なしの型にキャストする
  // 2. 2の補数をとる

  // 128はi16の範囲に収まっているためそのまま
  println!("128 as i16: {}", 128 as i16);
  // -> 128 as i16: 128

  // 128 - 256(std::u8::MAX) = -128
  println!("128 as i8: {}", 128 as i8);
  // -> 128 as i8: -128

  // 1000 as u8 -> 232
  println!("1000 as u8: {}", 1000 as u8);
  // -> 1000 as u8: 232

  println!("232 as i8: {}", 232 as i8);
  // 232 as i8: -24
}
