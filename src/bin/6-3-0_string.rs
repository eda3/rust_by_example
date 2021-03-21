// To convert any type to a String is as simple as implementing the ToString trait for the type.
// 任意の型を String に変換するには、その型の ToString traitを実装するだけです。

// Rather than doing so directly, you should implement the fmt::Display trait which automagically
// provides ToString and also allows printing the type as discussed in the section on print!.
// 直接行うのではなく、自動的に ToString を提供する fmt::Display trait を実装するべきであり、また、 printの
// セクションで説明したように、型をprintすることもできます!

use std::fmt;

#[derive(Debug)]
struct Circle {
  radius: i32,
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circleの半径は{}", self.radius)
  }
}

fn main() {
  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());
  // -> Circleの半径は6
  println!("{:?}", circle);
  // -> Circle { radius: 6 }
  println!("{:?}", circle.to_string());
  // -> "Circleの半径は6"

  // One of the more common types to convert a string into is a number.
  // 文字列を数値に変換する際によく使われる型のひとつが「数値型」です。

  // The idiomatic approach to this is to use the parse function and either to arrange for type
  // inference or to specify the type to parse using the 'turbofish' syntax.
  // この場合の慣用的な方法は、parse関数を使用して型の推論を行うか、「turbofish」構文を使用してparseする型を指定
  // することです。

  // Both alternatives are shown in the following example.
  // 次の例では、どちらの方法も使用できます。

  // This will convert the string into the type specified so long as the FromStr trait is
  // implemented for that type.
  // これにより、FromStr traitがその型に対して実装されている限り、文字列は指定された型に変換されます。

  // This is implemented for numerous types within the standard library.
  // これは、標準ライブラリの多くの型に実装されています。

  // To obtain this functionality on a user defined type simply implement the FromStr trait for
  // that type.
  // ユーザー定義の型でこの機能を利用するには、その型のFromStr traitを実装するだけです。

  let parsed: i32 = "5".parse().unwrap();
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println!("Sum: {:?}", sum);
  // Sum: 15
}
