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
    // Functions declared using `pub(self)` syntax are only visible within the current module, which
    // is the same as leaving them private
    // pub(self)`構文を使って宣言された関数は，現在のモジュール内でしか見ることができませんが，これはprivateに
    // しているのと同じです。
    pub(self) fn public_function_in_nested() {
      println!("my_mod::nested::public_function_in_nested()が呼ばれました！");
    }

    // Functions declared using `pub(super)` syntax are only visible within the parent module
    // pub(super)`構文で宣言された関数は、親モジュール内でのみ表示されます。
    pub(super) fn public_function_in_super_mod() {
      println!("my_mod::nested::public_function_in_super_mod()が呼ばれました！");
    }
  }

  pub fn call_public_function_in_my_mod() {
    println!("my_mod::call_public_function_in_my_mod()が呼ばれました！");
    nested::public_function_in_my_mod();
    nested::public_function_in_super_mod();
  }

  // pub(crate) makes functions visible only within the current crate
  // pub(crate)は、現在のcrate内でのみ関数を表示します。
  pub(crate) fn public_function_in_crate() {
    println!("my_mod::public_function_in_crateが呼ばれました！");
  }

  // ネストしたモジュールも同様の性質を示します。
  mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
      println!("my_mod::private_nested::function()");
    }
    // Private parent items will still restrict the visibility of a child item, even if it is
    // declared as visible within a bigger scope.
    // 非公開の親アイテムは、子アイテムがより大きなスコープ内で可視であると宣言されていても、子アイテムの可視性を
    // 制限します。
    #[allow(de)]
    pub(crate) fn restricted_function() {
      println!("my_mod::private_nested::restricted_function()が呼ばれました！");
    }
  }
}

fn function() {
  println!("function()が呼ばれました！");
}

fn main() {
  // モジュールによって、同名の関数を区別することができる。
  function();
  // -> function()が呼ばれました！

  my_mod::function();
  // -> my_mod::function()が呼ばれました！

  // pub(in path)アイテムは、指定されたモジュール内からのみ呼び出すことができます。
  // Error! function `public_function_in_my_mod` is private
  // my_mod::nested::public_function_in_my_mod();

  // プライベートな要素は、たとえパブリックなモジュール内に存在していても
  // 直接アクセスすることはできません。

  // Error! `private_function` is private
  // my_mod::private_function();

  // Error! `private_function` is private
  // my_mod::nested::private_function();

  // Error! `private_nested` is a private module
  // my_mod::private_nested::function();

  // Error! `private_nested` is a private module
  // my_mod::private_nested::restricted_function();
}
