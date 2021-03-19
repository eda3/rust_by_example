// ### From and Into
// The From and Into traits are inherently linked,
// FromとIntoの特性は本質的につながっている

// and this is actually part of its implementation.
// とありますが、これは実際にその実装の一部です。

// If you are able to convert type A from type B,
// タイプAをタイプBに変換することができれば
// then it should be easy to believe that we should be able to convert type B to type A.
// であれば、タイプBをタイプAに変換することができるはずだと、簡単に信じられるはずです。

// #### From
// The From trait allows for a type to define how to create itself from another type,
// From traitは、ある型が他の型から自分自身を生成する方法を定義することができます。

// There are numerous implementations of this trait within the standard library for conversion of primitive and common types.
// 標準ライブラリには、プリミティブ型と共通型の変換のために、このtraitの多くの実装があります。

use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32,
}

fn main() {}
