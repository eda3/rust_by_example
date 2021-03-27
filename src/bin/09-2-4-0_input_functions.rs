// 関数を引数として取り即座に実行する関数
fn call_me<F: Fn()>(f: F) {
  f();
}

fn function() {
  println!("私は関数！");
}

fn main() {
  let closure = || println!("私はクロージャ！");

  call_me(closure);
  // -> 私はクロージャ！

  call_me(function);
  // -> 私は関数！
}
