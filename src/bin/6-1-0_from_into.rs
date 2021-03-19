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

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

// ### Into
// The Into trait is simply the reciprocal of the From trait.
// Into traitは、単純にFrom traitの逆数です。

// That is, if you have implemented the From trait for your type, Into will call it when necessary.
// つまり、自分の型にFrom形質を実装していれば、Intoは必要に応じてそれを呼び出します。

// Using the Into trait will typically require specification of the type to convert into as the compiler is unable to determine this most of the time.
// Into traitを使用する際には、変換先の型を指定する必要がありますが、これはコンパイラがほとんどの場合判断できないためです。

// However this is a small trade-off considering we get the functionality for free.
// しかし、この機能を無料で手に入れられることを考えると、これは小さなトレードオフです。

fn main() {
  // ### From
  let num = Number::from(30);
  println!("My number is {:?}", num);
  // -> My number is Number { value: 30 }

  // ### Into
  let int = 5;
  // Try removing the type declaration
  let num: Number = int.into();
  println!("My number is {:?}", num);
  // -> My number is Number { value: 5 }
}
