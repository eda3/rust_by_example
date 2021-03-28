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
  }
}
fn main() {}
