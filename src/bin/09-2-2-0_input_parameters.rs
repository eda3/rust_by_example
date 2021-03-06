// While Rust chooses how to capture variables on the fly mostly without type annotation, this
// ambiguity is not allowed when writing functions.
// Rustでは、型の注釈を付けずにその場で変数を取得する方法を選択しますが、関数を書くときにはこのような曖昧さは許されません。

// When taking a closure as an input parameter, the closure's complete type must be annotated using
// one of a few traits.
// クロージャを入力パラメータとして受け取る場合、クロージャの完全な型は、いくつかのtraitのうちの1つを使ってアノテーション
// されなければなりません。

// In order of decreasing restriction, they are:
// 制限の少ない順に並べると、以下のようになります。
//   - Fn: the closure captures by reference (&T)
//   - Fn: クロージャは参照によってキャプチャされます (&T)

//   - FnMut: the closure captures by mutable reference (&mut T)
//   - FnMut: クロージャはミュータブルリファレンス(&mut T)によってキャプチャされます。

//   - FnOnce: the closure captures by value (T)
//   - FnOnce: クロージャは値(T)でキャプチャします。

// On a variable-by-variable basis, the compiler will capture variables in the least restrictive
// manner possible.
// 変数ごとに、コンパイラは可能な限り制限の少ない方法で変数を捕捉します。

// For instance, consider a parameter annotated as FnOnce.
// 例えば、FnOnceとアノテーションされたパラメータを考えてみましょう。

// This specifies that the closure may capture by &T, &mut T, or T, but the compiler will ultimately
// choose based on how the captured variables are used in the closure.
// これは、クロージャが&T、&mut T、またはTでキャプチャできることを指定していますが、コンパイラは最終的に、キャプチャされた
// 変数がクロージャ内でどのように使用されるかに基づいて選択します。

// This is because if a move is possible, then any type of borrow should also be possible.
// これは、ムーブが可能であれば、あらゆるタイプの借用も可能なはずだからです。

// Note that the reverse is not true.
// 逆は真ではないことに注意してください。

// If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.
// パラメータがFnとしてアノテーションされている場合、&mut TやTによる変数のキャプチャは許されません。

// In the following example, try swapping the usage of Fn, FnMut, and FnOnce to see what happens:
// 次の例では、Fn、FnMut、FnOnceの使い方を入れ替えてみて、何が起こるかを見てみましょう。

// A function which takes a closure as an argument and calls it.
// クロージャを引数に取り、それを呼び出す関数です。
// <F> denotes that F is a "Generic type parameter"
// <F>はFが "Generic type parameter "であることを示す。
fn apply<F>(f: F)
where
  F: FnOnce(),
{
  f();
}

// クロージャを引数に取り、`i32`を返す関数
fn apply_to_3<F>(f: F) -> i32
where
  // このクロージャは引数、返り値ともにi32
  F: Fn(i32) -> i32,
{
  f(3)
}
fn main() {
  use std::mem;

  let greeting = "hello";
  // コピーではなくmoveが起きる型
  let mut farewell = "goodby".to_owned();

  // 変数を2つ捕捉。greetingは参照を、farewellは値をそれぞれ捕捉する
  let diary = || {
    // greetingは参照なのでFnが必要
    println!("I said {}.", greeting);

    // farewellの値を変更するのでこの時点でFnMutが必要になる
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);

    // mem::dropを明示的に呼ぶとfarewellが値で捕捉される必要性が発生する。よってFnOnceが必要になる。
    mem::drop(farewell);
  };

  // クロージャを適用する関数を実行
  apply(diary);

  let double = |x| 2 * x;
  println!("3 doubled: {}", apply_to_3(double));
}
