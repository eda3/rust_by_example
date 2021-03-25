// 関数を引数として取り即座に実行する関数
fn call_me<F: Fn()>(f: F) {
  f();
}

fn function() {
  println!("私は関数！");
}

fn main() {}
