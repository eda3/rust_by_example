fn main() {
  // 整数の足し算
  println!("1 + 2 = {}", 1u32 + 2);
  // -> 1 + 2 = 3

  // 整数の引き算
  // この場合エラーとなる
  // println!("1 - 2 = {}", 1u32 - 2);
  // エラーメッセージ:attempt to compute `1_u32 - 2_u32`, which would overflow

  // u32をi32に変更すればエラーにならない
  println!("1 - 2 = {}", 1i32 - 2);
  // 1 - 2 = -1

  // 単純な論理演算子
  println!("true AND false は {}", true && false) ;
  // -> true AND false は false
  println!("true OR false は {}", true || false) ;
  // -> true OR  false は true
  println!("NOT true は {}", !true) ;
  // -> NOT true は false

  // ビットワイズ演算
  println!("0011 AND 0101 は {:04b}", 0b0011u32 & 0b0101);
  // -> 0011 AND 0101 は 0001
  println!("0011 OR 0101 は {:04b}", 0b0011u32 | 0b0101);
  // -> 0011 OR 0101 は 0111
  println!("0011 XOR 0101 は {:04b}", 0b0011u32 ^ 0b0101);
  // -> 0011 XOR 0101 は 0110
  println!("1 << 5  は {:04b}", 1u32 << 5);
  // -> 1 << 5  は 100000
  println!("0x80 >> 2 は 0x{:x}", 0x80u32 >> 2);
  // -> 0x80 >> 2 は 0x20

  // 可読性のため`_`(アンダースコア)を使用
  println!("5000兆円は{}円", 5_00_000_000_000_000u64);
  // -> 5000兆円は500000000000000円
}
