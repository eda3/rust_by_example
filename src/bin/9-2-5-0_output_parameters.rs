// Closures as input parameters are possible, so returning closures as output parameters should also
// be possible.
// 入力パラメータとしてのクロージャが可能なので、出力パラメータとしてクロージャを返すことも可能なはずです。

// However, anonymous closure types are, by definition, unknown, so we have to use impl Trait to
// return them.
// しかし、無名のクロージャ型は定義上、未知のものなので、それを返すためには impl Trait を使わなければなりません。

// The valid traits for returning a closure are:
// クロージャを返すのに有効なTraitsは以下の通りです。
// - Fn
// - FnMut
// - FnOnce

// Beyond this, the move keyword must be used, which signals that all captures occur by value
// その先、moveキーワードを使用する必要があります。これは、すべてのキャプチャーが値によって行われることを示すものです。

// This is required because any captures by reference would be dropped as soon as the function
// exited, leaving invalid references in the closure.
// これは、参照によるキャプチャが、関数が終了すると同時に削除され、クロージャに無効な参照が残ってしまうためです。

fn create_fn() -> impl Fn() {
  let text = "Fn".to_owned();
  move || println!("これは{}()", text)
}

fn create_fnmut() -> impl FnMut() {
  let text = "FnMut".to_owned();

  move || println!("これは{}()", text)
}

fn create_fnonce() -> impl FnOnce() {
  let text = "FnOnce".to_owned();

  move || println!("これは{}()", text)
}

fn main() {}
