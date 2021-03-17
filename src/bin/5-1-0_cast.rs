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


}
