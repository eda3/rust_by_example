// Closures in Rust, also called lambda expressions or lambdas, are functions that can capture the
// enclosing environment.
// Rustにおけるクロージャは、ラムダ式やラムダとも呼ばれ、囲い込まれた環境を捉えることができる関数です。

// For example, a closure that captures the x variable:
// 例えば、xという変数を捕捉するクロージャ。
// |val| val + x

// The syntax and capabilities of closures make them very convenient for on the fly usage.
// クロージャの構文と機能は、その場での使用にとても便利です。

// Calling a closure is exactly like calling a function.
// クロージャの呼び出しは、関数の呼び出しとまったく同じです。

// However, both input and return types can be inferred and input variable names must be specified.
// ただし、入力型と戻り型の両方を推測することができ、入力変数名を指定する必要があります。

// Other characteristics of closures include:
// その他、クロージャの特徴として

//   - using || instead of () around input variables.
//   ・入力変数を () で囲む代わりに || を使用する。
//   - optional body delimination ({}) for a single expression (mandatory otherwise).
//   ・単一の式の場合、オプションでボディの削除（{}）が可能（それ以外は必須）
//   - the ability to capture the outer environment variables.
//   ・外部の環境変数をキャプチャする機能
fn main() {
  // 関数とクロージャのそれぞれの数値をインクリメントする
  fn function(i: i32) -> i32 {
    i + 1
  }

  // 型アノテーションは通常の関数と同様の方法で行えるが必須ではない
  // {} も必須ではない
  let closure_annotated = |i: i32| -> i32 { i + 1 };

  let closure_inferred = |i| i + 1;

  let i = 1;
  println!("function: {}", function(i));
  // -> function: 2

  println!("closure 型アノテーション {}", closure_annotated(i));
  // -> closure 型アノテーション 2

  println!("closure 型推論 {}", closure_inferred(i));
  // closure 型推論 2

  // i32を返す引数を取らないクロージャ
  // 戻り値の型は型推論される
  let one = || 1;
  println!("one(): {}", one());
  // -> one(): 1
}
