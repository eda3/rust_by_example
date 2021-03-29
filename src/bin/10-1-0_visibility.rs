// my_mod という名称のモジュール
mod my_mod {
  // モジュール内の要素はデフォルトでプライベート
  fn private_function() {
    println!("my_mod::private_function()が呼ばれました！");
  }

  // pubを用いてパブリックに変更
  pub fn function() {
    println!("my_mod::function()が呼ばれました！");
  }

  // モジュール内からならば、プライベートな属性にアクセスすることに支障はなし
  pub fn indirect_access() {
    println!("my_mod::indirect_access()が呼ばれました！");
    println!("my_mod::private_function()を呼びます！");
  }

  // モジュール内でネストすることも可能です
  pub mod nested {
    pub fn function() {
      println!("my_mod::nested::function()が呼ばれました！");
    }

    #[allow(dead_code)]
    fn private_function() {
      println!("my_mod::nested::private_function()が呼ばれました！");
    }
    // Functions declared using `pub(in path)` syntax are only visible within the given path.
    // `path` must be a parent or ancestor module
    // pub(in path)構文を使って宣言された関数は、与えられたパスの中でのみ表示されます。

    // `path` must be a parent or ancestor module
    // path は親または先祖のモジュールでなければなりません。
    pub(in crate::my_mod) fn public_function_in_my_mod() {
      println!("my_mod::nested::public_function_in_my_modが呼ばれました！");
    }
  }
}
fn main() {}
