// To convert any type to a String is as simple as implementing the ToString trait for the type.
// 任意の型を String に変換するには、その型の ToString traitを実装するだけです。
// Rather than doing so directly, you should implement the fmt::Display trait which automagically provides ToString and also allows printing the type as discussed in the section on print!.
// 直接行うのではなく、自動的に ToString を提供する fmt::Display trait を実装するべきであり、また、 print のセクションで説明したように、型をprintすることもできます!

use std::fmt;

struct Circle {
  radius: i32,
}

fn main() {}
