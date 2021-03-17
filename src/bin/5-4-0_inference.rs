fn type_of<T>(_: T) -> String {
  let a = std::any::type_name::<T>();
  a.to_string()
}
fn main() {
  // アノテーションのおかげで、コンパイラはelemがu8型であることがわかる
  let elem = 5u8;
  println!("{}", type_of(elem));
  // -> u8
}
