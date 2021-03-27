// 関数内でクロージャを使う場合、ジェネリック型でなくてはならない
fn apply<F>(f: F)
where
  F: FnOnce(),
{
  f();
}
fn main() {
  let x = 7;

  // xを無名の構造体に入れ、それにたいしてFnを実装する
  // ※ここではFnは fn Fn(&self) ->  println!("{}", &self)
  // その構造体をprintにアサインする
  let print = || println!("{}", x);

  apply(print);
  // -> 7
}
